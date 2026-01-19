//! Static Export Module for Nucleus CLI
//!
//! Provides static site generation with:
//! - Route discovery from `.ncl` views
//! - HTML pre-rendering
//! - Incremental builds (hash-based change detection)
//! - WebP image conversion
//! - Asset optimization

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use miette::{IntoDiagnostic, Result};
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};


// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Export configuration
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub output_dir: PathBuf,
    pub base_url: Option<String>,
    pub incremental: bool,
    pub minify: bool,
    pub convert_webp: bool,
    pub platform: Option<Platform>,
    /// Generate PWA assets (manifest, service worker, offline page)
    pub pwa: bool,
    /// PWA app name (defaults to project name)
    pub pwa_name: Option<String>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("dist"),
            base_url: None,
            incremental: false,
            minify: true,
            convert_webp: true,
            platform: None,
            pwa: false,
            pwa_name: None,
        }
    }
}

/// Target deployment platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Netlify,
    Vercel,
    Cloudflare,
    GithubPages,
    Generic,
}

impl Platform {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "netlify" => Some(Self::Netlify),
            "vercel" => Some(Self::Vercel),
            "cloudflare" => Some(Self::Cloudflare),
            "github" | "github-pages" | "gh-pages" => Some(Self::GithubPages),
            "generic" | "static" => Some(Self::Generic),
            _ => None,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::Netlify => "Netlify",
            Self::Vercel => "Vercel",
            Self::Cloudflare => "Cloudflare Pages",
            Self::GithubPages => "GitHub Pages",
            Self::Generic => "Generic Static",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTE DISCOVERY
// ═══════════════════════════════════════════════════════════════════════════

/// Discovered route from views
#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub file: PathBuf,
    pub is_dynamic: bool,
}

/// Discover all routes from the views directory
pub fn discover_routes(views_dir: &Path) -> Result<Vec<Route>> {
    let mut routes = Vec::new();
    
    if !views_dir.exists() {
        return Ok(routes);
    }
    
    for entry in WalkDir::new(views_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "ncl").unwrap_or(false))
    {
        let file_path = entry.path();
        let relative = file_path.strip_prefix(views_dir).unwrap_or(file_path);
        
        // Convert file path to URL path
        let mut url_path = String::from("/");
        for component in relative.components() {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_string_lossy();
                if name_str != "index.ncl" {
                    let segment = name_str.trim_end_matches(".ncl");
                    url_path.push_str(segment);
                    url_path.push('/');
                }
            }
        }
        
        // Handle index files
        if url_path.len() > 1 && url_path.ends_with('/') {
            url_path.pop();
        }
        if url_path.is_empty() {
            url_path = "/".to_string();
        }
        
        // Check for dynamic routes
        let is_dynamic = url_path.contains('[') && url_path.contains(']');
        
        routes.push(Route {
            path: url_path,
            file: file_path.to_path_buf(),
            is_dynamic,
        });
    }
    
    // Sort routes for consistent ordering
    routes.sort_by(|a, b| a.path.cmp(&b.path));
    
    println!("  📍 Discovered {} routes", routes.len());
    for route in &routes {
        let marker = if route.is_dynamic { "🔸" } else { "  " };
        println!("     {} {}", marker, route.path);
    }
    
    Ok(routes)
}

// ═══════════════════════════════════════════════════════════════════════════
// INCREMENTAL BUILD CACHE
// ═══════════════════════════════════════════════════════════════════════════

/// Cache manifest for incremental builds with dependency tracking
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportCache {
    pub file_hashes: HashMap<String, String>,
    pub last_export: Option<String>,
    /// Tracks which files depend on other files (e.g., view → layout)
    pub dependencies: HashMap<String, Vec<String>>,
    /// Build metrics for performance tracking
    pub metrics: BuildMetrics,
}

/// Build performance metrics
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BuildMetrics {
    pub total_builds: u64,
    pub last_build_duration_ms: u64,
    pub files_processed: u64,
    pub files_skipped: u64,
    pub cache_hit_rate: f64,
}

impl ExportCache {
    pub fn load(project_dir: &Path) -> Self {
        let cache_path = project_dir.join(".nucleus").join("export-cache.json");
        if cache_path.exists() {
            if let Ok(content) = fs::read_to_string(&cache_path) {
                if let Ok(cache) = serde_json::from_str(&content) {
                    return cache;
                }
            }
        }
        Self::default()
    }
    
    pub fn save(&self, project_dir: &Path) -> Result<()> {
        let nucleus_dir = project_dir.join(".nucleus");
        fs::create_dir_all(&nucleus_dir).into_diagnostic()?;
        
        let cache_path = nucleus_dir.join("export-cache.json");
        let content = serde_json::to_string_pretty(self).into_diagnostic()?;
        fs::write(&cache_path, content).into_diagnostic()?;
        
        Ok(())
    }
    
    pub fn compute_hash(path: &Path) -> Option<String> {
        if let Ok(content) = fs::read(path) {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            Some(format!("{:x}", hasher.finish()))
        } else {
            None
        }
    }
    
    /// Compute a content-aware hash that includes dependencies
    pub fn compute_hash_with_deps(&self, path: &Path) -> Option<String> {
        let mut combined = String::new();
        
        // Include the file's own hash
        if let Some(hash) = Self::compute_hash(path) {
            combined.push_str(&hash);
        }
        
        // Include hashes of all dependencies
        let path_str = path.to_string_lossy().to_string();
        if let Some(deps) = self.dependencies.get(&path_str) {
            for dep in deps {
                if let Some(dep_hash) = self.file_hashes.get(dep) {
                    combined.push_str(dep_hash);
                }
            }
        }
        
        if combined.is_empty() {
            return None;
        }
        
        // Hash the combined string
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        combined.hash(&mut hasher);
        Some(format!("{:x}", hasher.finish()))
    }
    
    pub fn needs_rebuild(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_string();
        if let Some(old_hash) = self.file_hashes.get(&path_str) {
            if let Some(new_hash) = Self::compute_hash(path) {
                if old_hash != &new_hash {
                    return true;
                }
            }
        } else {
            return true; // No cache entry, needs build
        }
        
        // Check if any dependencies changed
        if let Some(deps) = self.dependencies.get(&path_str) {
            for dep_path in deps {
                let dep = Path::new(dep_path);
                if let Some(old_dep_hash) = self.file_hashes.get(dep_path) {
                    if let Some(new_dep_hash) = Self::compute_hash(dep) {
                        if old_dep_hash != &new_dep_hash {
                            return true; // Dependency changed
                        }
                    }
                }
            }
        }
        
        false // No changes detected
    }
    
    /// Check if a shared file (layout, component) changed, triggering cascade rebuilds
    pub fn check_cascade_rebuild(&self, shared_files: &[PathBuf]) -> Vec<String> {
        let mut affected = Vec::new();
        
        for shared in shared_files {
            let shared_str = shared.to_string_lossy().to_string();
            
            // Check if shared file changed
            if let Some(old_hash) = self.file_hashes.get(&shared_str) {
                if let Some(new_hash) = Self::compute_hash(shared) {
                    if old_hash != &new_hash {
                        // Find all files that depend on this shared file
                        for (file, deps) in &self.dependencies {
                            if deps.contains(&shared_str) {
                                affected.push(file.clone());
                            }
                        }
                    }
                }
            }
        }
        
        affected
    }
    
    pub fn update_hash(&mut self, path: &Path) {
        let path_str = path.to_string_lossy().to_string();
        if let Some(hash) = Self::compute_hash(path) {
            self.file_hashes.insert(path_str, hash);
        }
    }
    
    /// Register a dependency relationship (e.g., view.ncl depends on layout.ncl)
    pub fn add_dependency(&mut self, file: &Path, depends_on: &Path) {
        let file_str = file.to_string_lossy().to_string();
        let dep_str = depends_on.to_string_lossy().to_string();
        
        self.dependencies
            .entry(file_str)
            .or_default()
            .push(dep_str);
    }
    
    /// Parse NCL file to extract layout dependencies
    pub fn extract_dependencies(&mut self, ncl_path: &Path, views_dir: &Path) -> Result<()> {
        if let Ok(content) = fs::read_to_string(ncl_path) {
            // Find n:layout declarations
            let layout_pattern = regex::Regex::new(r#"<n:layout\s+name="([^"]+)""#)
                .into_diagnostic()?;
            
            for cap in layout_pattern.captures_iter(&content) {
                if let Some(layout_match) = cap.get(1) {
                    let layout_name = layout_match.as_str();
                    let layout_file = views_dir.join(format!("{}.ncl", layout_name));
                    if layout_file.exists() {
                        self.add_dependency(ncl_path, &layout_file);
                    }
                }
            }
            
            // Find component imports/includes
            let component_pattern = regex::Regex::new(r#"<n:include\s+src="([^"]+)""#)
                .into_diagnostic()?;
            
            for cap in component_pattern.captures_iter(&content) {
                if let Some(comp_match) = cap.get(1) {
                    let comp_path = comp_match.as_str();
                    let comp_file = views_dir.join(comp_path);
                    if comp_file.exists() {
                        self.add_dependency(ncl_path, &comp_file);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Update build metrics
    pub fn record_build(&mut self, duration_ms: u64, processed: u64, skipped: u64) {
        self.metrics.total_builds += 1;
        self.metrics.last_build_duration_ms = duration_ms;
        self.metrics.files_processed = processed;
        self.metrics.files_skipped = skipped;
        
        let total = processed + skipped;
        if total > 0 {
            self.metrics.cache_hit_rate = (skipped as f64 / total as f64) * 100.0;
        }
    }
    
    /// Print build metrics summary
    pub fn print_metrics(&self) {
        if self.metrics.total_builds > 0 {
            println!("\n  📊 \x1b[1mBuild Metrics\x1b[0m");
            println!("     Duration: {}ms", self.metrics.last_build_duration_ms);
            println!("     Processed: {} files", self.metrics.files_processed);
            println!("     Skipped: {} files (cached)", self.metrics.files_skipped);
            println!("     Cache hit rate: {:.1}%", self.metrics.cache_hit_rate);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STATIC EXPORT ENGINE
// ═══════════════════════════════════════════════════════════════════════════

/// Run static export
pub fn run_export(config: ExportConfig) -> Result<()> {
    use std::time::Instant;
    let start_time = Instant::now();
    
    println!("\n⚡ \x1b[1;36mNucleus Static Export\x1b[0m\n");
    
    let project_dir = std::env::current_dir().into_diagnostic()?;
    let views_dir = project_dir.join("src").join("views");
    let static_dir = project_dir.join("static");
    let output_dir = &config.output_dir;
    
    // Load cache for incremental builds
    let mut cache = if config.incremental {
        println!("  📦 Incremental mode enabled");
        ExportCache::load(&project_dir)
    } else {
        ExportCache::default()
    };
    
    // Step 1: Discover routes
    println!("\n  \x1b[1m1. Discovering routes...\x1b[0m");
    let routes = discover_routes(&views_dir)?;
    
    if routes.is_empty() {
        println!("  ⚠️  No routes found in src/views/");
        return Ok(());
    }
    
    // Step 1.5: Build dependency graph (incremental mode)
    let mut files_skipped = 0u64;
    let mut files_processed = 0u64;
    
    if config.incremental {
        println!("\n  \x1b[1m1.5. Building dependency graph...\x1b[0m");
        for route in &routes {
            if let Err(e) = cache.extract_dependencies(&route.file, &views_dir) {
                println!("     ⚠️  Could not extract deps for {}: {}", route.path, e);
            }
        }
        
        // Check for cascade rebuilds (layout changes affecting multiple views)
        let layouts: Vec<PathBuf> = std::fs::read_dir(&views_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| {
                        p.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.starts_with("layout"))
                            .unwrap_or(false)
                    })
                    .collect()
            })
            .unwrap_or_default();
        
        let affected = cache.check_cascade_rebuild(&layouts);
        if !affected.is_empty() {
            println!("     ⚡ Cascade rebuild: {} files affected by layout changes", affected.len());
        }
        
        println!("     ✓ Tracked {} dependency relationships", 
            cache.dependencies.values().map(|v| v.len()).sum::<usize>());
    }
    
    // Step 2: Create output directory
    println!("\n  \x1b[1m2. Preparing output directory...\x1b[0m");
    if output_dir.exists() && !config.incremental {
        fs::remove_dir_all(output_dir).into_diagnostic()?;
    }
    fs::create_dir_all(output_dir).into_diagnostic()?;
    println!("     ✓ Output: {}", output_dir.display());
    
    // Step 3: Copy static assets
    println!("\n  \x1b[1m3. Copying static assets...\x1b[0m");
    let (assets_copied, assets_skipped) = copy_static_assets_with_metrics(&static_dir, output_dir, &config, &mut cache)?;
    files_processed += assets_copied as u64;
    files_skipped += assets_skipped as u64;
    println!("     ✓ Copied {} assets ({} cached)", assets_copied, assets_skipped);
    
    // Step 4: Pre-render routes
    println!("\n  \x1b[1m4. Pre-rendering pages...\x1b[0m");
    let (pages_rendered, pages_skipped) = prerender_routes_with_cache(&routes, output_dir, &config, &mut cache)?;
    files_processed += pages_rendered as u64;
    files_skipped += pages_skipped as u64;
    println!("     ✓ Rendered {} pages ({} cached)", pages_rendered, pages_skipped);
    
    // Step 5: Generate metadata files
    println!("\n  \x1b[1m5. Generating metadata...\x1b[0m");
    generate_sitemap(&routes, output_dir, &config)?;
    generate_robots_txt(output_dir, &config)?;
    generate_404_page(output_dir)?;
    
    // Step 6: Generate PWA assets (if enabled)
    if config.pwa {
        println!("\n  \x1b[1m6. Generating PWA assets...\x1b[0m");
        generate_pwa_assets(output_dir, &routes, &config)?;
    }
    
    // Step 7: Generate platform-specific files
    if let Some(platform) = &config.platform {
        println!("\n  \x1b[1m7. Generating {} config...\x1b[0m", platform.name());
        generate_platform_config(output_dir, *platform)?;
    }
    
    // Save cache and record metrics
    let duration_ms = start_time.elapsed().as_millis() as u64;
    if config.incremental {
        cache.last_export = Some(chrono::Utc::now().to_rfc3339());
        cache.record_build(duration_ms, files_processed, files_skipped);
        cache.save(&project_dir)?;
        cache.print_metrics();
    }
    
    // Summary
    println!("\n  \x1b[1;32m✓ Export complete!\x1b[0m");
    println!("    Output: {}", output_dir.display());
    println!("    Routes: {}", routes.len());
    println!("    Assets: {}", assets_copied);
    println!("    Duration: {}ms", duration_ms);
    
    if let Some(base_url) = &config.base_url {
        println!("    URL: {}", base_url);
    }
    
    println!("\n  To preview locally:");
    println!("    \x1b[90mcd {} && python3 -m http.server 8080\x1b[0m\n", output_dir.display());
    
    Ok(())
}

/// Copy static assets with metrics tracking
fn copy_static_assets_with_metrics(
    static_dir: &Path,
    output_dir: &Path,
    config: &ExportConfig,
    cache: &mut ExportCache,
) -> Result<(usize, usize)> {
    let mut copied = 0;
    let mut skipped = 0;
    
    if !static_dir.exists() {
        return Ok((0, 0));
    }
    
    for entry in WalkDir::new(static_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let src = entry.path();
        let relative = src.strip_prefix(static_dir).unwrap_or(src);
        let dest = output_dir.join(relative);
        
        // Check if rebuild needed (incremental mode)
        if config.incremental && !cache.needs_rebuild(src) {
            skipped += 1;
            continue;
        }
        
        // Create parent directories
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).into_diagnostic()?;
        }
        
        let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        // Check if image needs WebP conversion
        if config.convert_webp && (ext == "png" || ext == "jpg" || ext == "jpeg") {
            if let Ok(()) = convert_to_webp(src, &dest.with_extension("webp")) {
                cache.update_hash(src);
                copied += 1;
                continue;
            }
        }
        
        // Minify CSS files
        if config.minify && ext == "css" {
            if let Ok(content) = fs::read_to_string(src) {
                let minified = minify_css(&content);
                fs::write(&dest, minified).into_diagnostic()?;
                cache.update_hash(src);
                copied += 1;
                println!("     ⚡ Minified: {}", relative.display());
                continue;
            }
        }
        
        // Minify JS files
        if config.minify && ext == "js" {
            if let Ok(content) = fs::read_to_string(src) {
                let minified = minify_js(&content);
                fs::write(&dest, minified).into_diagnostic()?;
                cache.update_hash(src);
                copied += 1;
                println!("     ⚡ Minified: {}", relative.display());
                continue;
            }
        }
        
        // Copy file as-is
        fs::copy(src, &dest).into_diagnostic()?;
        cache.update_hash(src);
        copied += 1;
    }
    
    Ok((copied, skipped))
}

/// Pre-render routes with cache support
fn prerender_routes_with_cache(
    routes: &[Route],
    output_dir: &Path,
    config: &ExportConfig,
    cache: &mut ExportCache,
) -> Result<(usize, usize)> {
    let mut rendered = 0;
    let mut skipped = 0;
    
    for route in routes {
        if route.is_dynamic {
            println!("     ⚠️  Skipping dynamic route: {}", route.path);
            continue;
        }
        
        // Check if rebuild needed (incremental mode)
        if config.incremental && !cache.needs_rebuild(&route.file) {
            skipped += 1;
            continue;
        }
        
        // Create directory structure
        let html_path = if route.path == "/" {
            output_dir.join("index.html")
        } else {
            let clean_path = route.path.trim_start_matches('/');
            output_dir.join(clean_path).join("index.html")
        };
        
        if let Some(parent) = html_path.parent() {
            fs::create_dir_all(parent).into_diagnostic()?;
        }
        
        // Generate placeholder HTML (future: headless pre-rendering)
        let html = generate_placeholder_html(&route.path);
        fs::write(&html_path, html).into_diagnostic()?;
        cache.update_hash(&route.file);
        
        rendered += 1;
    }
    
    Ok((rendered, skipped))
}

// ═══════════════════════════════════════════════════════════════════════════
// ASSET PROCESSING
// ═══════════════════════════════════════════════════════════════════════════

// function copy_static_assets removed


/// Simple CSS minification (removes comments and unnecessary whitespace)
fn minify_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut in_comment = false;
    let mut chars = css.chars().peekable();
    
    while let Some(c) = chars.next() {
        if in_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_comment = false;
            }
            continue;
        }
        
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
            continue;
        }
        
        // Collapse whitespace
        if c.is_whitespace() {
            // Only add single space, skip if next/prev is special char
            if !result.is_empty() {
                let last = result.chars().last().unwrap_or(' ');
                if !matches!(last, '{' | '}' | ';' | ':' | ',' | ' ') {
                    if let Some(&next) = chars.peek() {
                        if !matches!(next, '{' | '}' | ';' | ':' | ',') {
                            result.push(' ');
                        }
                    }
                }
            }
            // Skip remaining whitespace
            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }
            continue;
        }
        
        result.push(c);
    }
    
    result
}

/// Simple JS minification (removes comments and unnecessary whitespace)
fn minify_js(js: &str) -> String {
    let mut result = String::with_capacity(js.len());
    let mut in_string = None;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut chars = js.chars().peekable();
    
    while let Some(c) = chars.next() {
        // Handle line comments
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                result.push('\n');
            }
            continue;
        }
        
        // Handle block comments
        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }
        
        // Handle strings
        if let Some(quote) = in_string {
            result.push(c);
            if c == quote && result.chars().rev().nth(1) != Some('\\') {
                in_string = None;
            }
            continue;
        }
        
        // Start string
        if c == '"' || c == '\'' || c == '`' {
            in_string = Some(c);
            result.push(c);
            continue;
        }
        
        // Start comments
        if c == '/' {
            if chars.peek() == Some(&'/') {
                chars.next();
                in_line_comment = true;
                continue;
            }
            if chars.peek() == Some(&'*') {
                chars.next();
                in_block_comment = true;
                continue;
            }
        }
        
        // Collapse whitespace
        if c.is_whitespace() {
            if !result.is_empty() {
                let last = result.chars().last().unwrap_or(' ');
                // Keep space if needed for syntax
                if last.is_alphanumeric() || last == '_' || last == '$' {
                    if let Some(&next) = chars.peek() {
                        if next.is_alphanumeric() || next == '_' || next == '$' {
                            result.push(' ');
                        }
                    }
                }
            }
            // Skip remaining whitespace
            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }
            continue;
        }
        
        result.push(c);
    }
    
    result
}

fn convert_to_webp(src: &Path, dest: &Path) -> Result<()> {
    use image::ImageFormat;
    
    let img = image::open(src).into_diagnostic()?;
    
    // Also save original for fallback
    let original_dest = dest.with_extension(src.extension().unwrap_or_default());
    img.save(&original_dest).into_diagnostic()?;
    
    // Save WebP version
    let mut webp_file = fs::File::create(dest).into_diagnostic()?;
    img.write_to(&mut webp_file, ImageFormat::WebP).into_diagnostic()?;
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// PRE-RENDERING
// ═══════════════════════════════════════════════════════════════════════════

// function prerender_routes removed

fn generate_placeholder_html(route: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus - {}</title>
</head>
<body>
    <h1>Route: {}</h1>
    <p>This page will be pre-rendered when the server is running.</p>
</body>
</html>
"#, route, route)
}

// ═══════════════════════════════════════════════════════════════════════════
// METADATA GENERATION
// ═══════════════════════════════════════════════════════════════════════════

fn generate_sitemap(routes: &[Route], output_dir: &Path, config: &ExportConfig) -> Result<()> {
    let base_url = config.base_url.as_deref().unwrap_or("https://example.com");
    
    let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);
    
    for route in routes {
        if !route.is_dynamic {
            let url = format!("{}{}", base_url.trim_end_matches('/'), route.path);
            sitemap.push_str(&format!(r#"  <url>
    <loc>{}</loc>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
"#, url));
        }
    }
    
    sitemap.push_str("</urlset>\n");
    
    fs::write(output_dir.join("sitemap.xml"), sitemap).into_diagnostic()?;
    println!("     ✓ Generated sitemap.xml");
    
    Ok(())
}

fn generate_robots_txt(output_dir: &Path, config: &ExportConfig) -> Result<()> {
    let base_url = config.base_url.as_deref().unwrap_or("https://example.com");
    
    let content = format!(r#"User-agent: *
Allow: /

Sitemap: {}/sitemap.xml
"#, base_url.trim_end_matches('/'));
    
    fs::write(output_dir.join("robots.txt"), content).into_diagnostic()?;
    println!("     ✓ Generated robots.txt");
    
    Ok(())
}

fn generate_404_page(output_dir: &Path) -> Result<()> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Page Not Found</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: #0a0a0a;
            color: #ededed;
        }
        .container {
            text-align: center;
        }
        h1 {
            font-size: 6rem;
            margin: 0;
            background: linear-gradient(135deg, #00dc82, #0ea5e9);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        p {
            color: #888;
            margin: 1rem 0 2rem;
        }
        a {
            color: #00dc82;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <p>Page not found</p>
        <a href="/">← Back to home</a>
    </div>
</body>
</html>
"#;
    
    fs::write(output_dir.join("404.html"), html).into_diagnostic()?;
    println!("     ✓ Generated 404.html");
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// PWA ASSETS GENERATION
// ═══════════════════════════════════════════════════════════════════════════

/// Generate PWA assets (manifest.json, sw.js, offline.html)
fn generate_pwa_assets(output_dir: &Path, routes: &[Route], config: &ExportConfig) -> Result<()> {
    use crate::pwa::{PwaConfig, generate_manifest, generate_service_worker, generate_offline_page, generate_sw_registration};
    
    // Build PWA config from export config
    let pwa_config = PwaConfig {
        name: config.pwa_name.clone().unwrap_or_else(|| "Nucleus App".to_string()),
        short_name: config.pwa_name.clone().unwrap_or_else(|| "App".to_string()),
        ..PwaConfig::default()
    };
    
    // Generate manifest.json
    let manifest = generate_manifest(&pwa_config);
    fs::write(output_dir.join("manifest.json"), &manifest).into_diagnostic()?;
    println!("     ✓ Generated manifest.json");
    
    // Generate service worker
    let route_paths: Vec<String> = routes.iter().map(|r| r.path.clone()).collect();
    let sw = generate_service_worker(&route_paths, &pwa_config);
    fs::write(output_dir.join("sw.js"), &sw).into_diagnostic()?;
    println!("     ✓ Generated sw.js (cache-first strategy)");
    
    // Generate offline page
    let offline = generate_offline_page(&pwa_config);
    fs::write(output_dir.join("offline.html"), &offline).into_diagnostic()?;
    println!("     ✓ Generated offline.html");
    
    // Copy neutron-store.js if available
    let neutron_store_src = std::env::current_dir()
        .unwrap_or_default()
        .join("static")
        .join("assets")
        .join("neutron-store.js");
    
    if neutron_store_src.exists() {
        let dest_dir = output_dir.join("assets");
        fs::create_dir_all(&dest_dir).into_diagnostic()?;
        fs::copy(&neutron_store_src, dest_dir.join("neutron-store.js")).into_diagnostic()?;
        println!("     ✓ Copied neutron-store.js");
    }
    
    // Print SW registration snippet for users
    println!("\n  \x1b[90mAdd this to your HTML <head>:\x1b[0m");
    let registration = generate_sw_registration();
    for line in registration.lines().take(5) {
        println!("  \x1b[90m{}\x1b[0m", line);
    }
    println!("  \x1b[90m  ...\x1b[0m");
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// PLATFORM-SPECIFIC CONFIGS
// ═══════════════════════════════════════════════════════════════════════════

fn generate_platform_config(output_dir: &Path, platform: Platform) -> Result<()> {
    match platform {
        Platform::Netlify => {
            // _redirects file
            let redirects = "/*    /index.html   200\n";
            fs::write(output_dir.join("_redirects"), redirects).into_diagnostic()?;
            
            // _headers file
            let headers = r#"/*
  X-Frame-Options: DENY
  X-XSS-Protection: 1; mode=block
  X-Content-Type-Options: nosniff
  Referrer-Policy: strict-origin-when-cross-origin

/assets/*
  Cache-Control: public, max-age=31536000, immutable
"#;
            fs::write(output_dir.join("_headers"), headers).into_diagnostic()?;
            println!("     ✓ Generated _redirects, _headers");
        }
        
        Platform::Vercel => {
            let config = r#"{
  "cleanUrls": true,
  "trailingSlash": false,
  "headers": [
    {
      "source": "/assets/(.*)",
      "headers": [
        { "key": "Cache-Control", "value": "public, max-age=31536000, immutable" }
      ]
    }
  ]
}
"#;
            fs::write(output_dir.join("vercel.json"), config).into_diagnostic()?;
            println!("     ✓ Generated vercel.json");
        }
        
        Platform::Cloudflare => {
            let config = r#"{
  "version": 1,
  "include": ["/*"],
  "exclude": []
}
"#;
            fs::write(output_dir.join("_routes.json"), config).into_diagnostic()?;
            println!("     ✓ Generated _routes.json");
        }
        
        Platform::GithubPages => {
            // .nojekyll to disable Jekyll processing
            fs::write(output_dir.join(".nojekyll"), "").into_diagnostic()?;
            println!("     ✓ Generated .nojekyll");
        }
        
        Platform::Generic => {
            // No special files needed
        }
    }
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// INTERACTIVE WIZARD
// ═══════════════════════════════════════════════════════════════════════════

pub fn run_export_wizard() -> Result<ExportConfig> {
    println!("\n┌─────────────────────────────────────────┐");
    println!("│  ⚡ \x1b[1;36mNucleus Static Export Wizard\x1b[0m        │");
    println!("└─────────────────────────────────────────┘\n");
    
    let mut config = ExportConfig::default();
    
    // Output directory
    print!("  Output directory [\x1b[90mdist\x1b[0m]: ");
    std::io::stdout().flush().into_diagnostic()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).into_diagnostic()?;
    let input = input.trim();
    if !input.is_empty() {
        config.output_dir = PathBuf::from(input);
    }
    
    // Base URL
    print!("  Base URL [\x1b[90mhttps://example.com\x1b[0m]: ");
    std::io::stdout().flush().into_diagnostic()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).into_diagnostic()?;
    let input = input.trim();
    if !input.is_empty() {
        config.base_url = Some(input.to_string());
    }
    
    // Incremental builds
    print!("  Use incremental builds? [\x1b[90mY/n\x1b[0m]: ");
    std::io::stdout().flush().into_diagnostic()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).into_diagnostic()?;
    config.incremental = !input.trim().eq_ignore_ascii_case("n");
    
    // WebP conversion
    print!("  Convert images to WebP? [\x1b[90mY/n\x1b[0m]: ");
    std::io::stdout().flush().into_diagnostic()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).into_diagnostic()?;
    config.convert_webp = !input.trim().eq_ignore_ascii_case("n");
    
    // Platform selection
    println!("\n  Target platform:");
    println!("    1) Generic (plain HTML)");
    println!("    2) Netlify");
    println!("    3) Vercel");
    println!("    4) Cloudflare Pages");
    println!("    5) GitHub Pages");
    print!("  Select [\x1b[90m1\x1b[0m]: ");
    std::io::stdout().flush().into_diagnostic()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).into_diagnostic()?;
    config.platform = match input.trim() {
        "2" => Some(Platform::Netlify),
        "3" => Some(Platform::Vercel),
        "4" => Some(Platform::Cloudflare),
        "5" => Some(Platform::GithubPages),
        _ => Some(Platform::Generic),
    };
    
    println!();
    
    Ok(config)
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLISH COMMAND
// ═══════════════════════════════════════════════════════════════════════════

pub fn run_publish(platform: Option<String>) -> Result<()> {
    println!("\n⚡ \x1b[1;36mNucleus Publish\x1b[0m\n");
    
    let platform = if let Some(p) = platform {
        Platform::parse(&p).ok_or_else(|| miette::miette!("Unknown platform: {}", p))?
    } else {
        // Interactive selection
        select_platform()?
    };
    
    // Check if dist exists
    let dist_dir = PathBuf::from("dist");
    if !dist_dir.exists() {
        println!("  ⚠️  No 'dist' directory found. Run 'nucleus export' first.\n");
        return Ok(());
    }
    
    println!("  📦 Publishing to {}...\n", platform.name());
    
    match platform {
        Platform::Netlify => publish_netlify(&dist_dir)?,
        Platform::Vercel => publish_vercel(&dist_dir)?,
        Platform::Cloudflare => publish_cloudflare(&dist_dir)?,
        Platform::GithubPages => publish_github_pages(&dist_dir)?,
        Platform::Generic => {
            println!("  ℹ️  Generic platform - no auto-publish available.");
            println!("     Upload the 'dist' folder to your hosting provider.\n");
        }
    }
    
    Ok(())
}

fn select_platform() -> Result<Platform> {
    println!("  Select platform:\n");
    
    // Detect existing configs
    let has_netlify = Path::new("netlify.toml").exists() || Path::new("dist/_redirects").exists();
    let has_vercel = Path::new("vercel.json").exists();
    
    let netlify_marker = if has_netlify { " \x1b[90m(detected)\x1b[0m" } else { "" };
    let vercel_marker = if has_vercel { " \x1b[90m(detected)\x1b[0m" } else { "" };
    
    println!("    1) Netlify{}", netlify_marker);
    println!("    2) Vercel{}", vercel_marker);
    println!("    3) Cloudflare Pages");
    println!("    4) GitHub Pages");
    
    print!("\n  Select [1-4]: ");
    std::io::stdout().flush().into_diagnostic()?;
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).into_diagnostic()?;
    
    match input.trim() {
        "1" => Ok(Platform::Netlify),
        "2" => Ok(Platform::Vercel),
        "3" => Ok(Platform::Cloudflare),
        "4" => Ok(Platform::GithubPages),
        _ => Ok(Platform::Netlify), // Default
    }
}

fn publish_netlify(dist_dir: &Path) -> Result<()> {
    // Check if netlify CLI is installed
    let status = std::process::Command::new("netlify")
        .arg("--version")
        .output();
    
    if status.is_err() {
        println!("  ⚠️  Netlify CLI not found. Install with:");
        println!("     \x1b[90mnpm install -g netlify-cli\x1b[0m\n");
        return Ok(());
    }
    
    println!("  Running: netlify deploy --prod --dir {}\n", dist_dir.display());
    
    let status = std::process::Command::new("netlify")
        .args(["deploy", "--prod", "--dir", &dist_dir.to_string_lossy()])
        .status()
        .into_diagnostic()?;
    
    if status.success() {
        println!("\n  \x1b[1;32m✓ Deployed to Netlify!\x1b[0m\n");
    }
    
    Ok(())
}

fn publish_vercel(dist_dir: &Path) -> Result<()> {
    let status = std::process::Command::new("vercel")
        .arg("--version")
        .output();
    
    if status.is_err() {
        println!("  ⚠️  Vercel CLI not found. Install with:");
        println!("     \x1b[90mnpm install -g vercel\x1b[0m\n");
        return Ok(());
    }
    
    println!("  Running: vercel --prod {}\n", dist_dir.display());
    
    let status = std::process::Command::new("vercel")
        .args(["--prod", &dist_dir.to_string_lossy()])
        .status()
        .into_diagnostic()?;
    
    if status.success() {
        println!("\n  \x1b[1;32m✓ Deployed to Vercel!\x1b[0m\n");
    }
    
    Ok(())
}

fn publish_cloudflare(dist_dir: &Path) -> Result<()> {
    let status = std::process::Command::new("wrangler")
        .arg("--version")
        .output();
    
    if status.is_err() {
        println!("  ⚠️  Wrangler CLI not found. Install with:");
        println!("     \x1b[90mnpm install -g wrangler\x1b[0m\n");
        return Ok(());
    }
    
    println!("  Running: wrangler pages deploy {}\n", dist_dir.display());
    
    let status = std::process::Command::new("wrangler")
        .args(["pages", "deploy", &dist_dir.to_string_lossy()])
        .status()
        .into_diagnostic()?;
    
    if status.success() {
        println!("\n  \x1b[1;32m✓ Deployed to Cloudflare Pages!\x1b[0m\n");
    }
    
    Ok(())
}

fn publish_github_pages(dist_dir: &Path) -> Result<()> {
    // Check if gh CLI is installed
    let status = std::process::Command::new("gh")
        .arg("--version")
        .output();
    
    if status.is_err() {
        println!("  ⚠️  GitHub CLI not found. Install from:");
        println!("     \x1b[90mhttps://cli.github.com\x1b[0m\n");
        println!("  Alternative: Push the 'dist' folder to a gh-pages branch.\n");
        return Ok(());
    }
    
    println!("  To deploy to GitHub Pages:");
    println!("  1. Create a gh-pages branch");
    println!("  2. Push the {} folder to that branch", dist_dir.display());
    println!("  3. Enable GitHub Pages in repo settings\n");
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_platform_from_str() {
        assert_eq!(Platform::parse("netlify"), Some(Platform::Netlify));
        assert_eq!(Platform::parse("VERCEL"), Some(Platform::Vercel));
        assert_eq!(Platform::parse("github"), Some(Platform::GithubPages));
        assert_eq!(Platform::parse("unknown"), None);
    }
    
    #[test]
    fn test_export_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.output_dir, PathBuf::from("dist"));
        assert!(config.minify);
        assert!(config.convert_webp);
    }
    
    #[test]
    fn test_export_cache() {
        let temp = TempDir::new().unwrap();
        let cache = ExportCache::default();
        cache.save(temp.path()).unwrap();
        
        let loaded = ExportCache::load(temp.path());
        assert!(loaded.file_hashes.is_empty());
    }
    
    #[test]
    fn test_discover_routes_empty() {
        let temp = TempDir::new().unwrap();
        let views_dir = temp.path().join("views");
        fs::create_dir_all(&views_dir).unwrap();
        
        let routes = discover_routes(&views_dir).unwrap();
        assert!(routes.is_empty());
    }
    
    #[test]
    fn test_discover_routes() {
        let temp = TempDir::new().unwrap();
        let views_dir = temp.path().join("views");
        fs::create_dir_all(&views_dir).unwrap();
        
        // Create test files
        fs::write(views_dir.join("index.ncl"), "<h1>Home</h1>").unwrap();
        fs::write(views_dir.join("about.ncl"), "<h1>About</h1>").unwrap();
        
        let blog_dir = views_dir.join("blog");
        fs::create_dir_all(&blog_dir).unwrap();
        fs::write(blog_dir.join("index.ncl"), "<h1>Blog</h1>").unwrap();
        
        let routes = discover_routes(&views_dir).unwrap();
        assert_eq!(routes.len(), 3);
    }
    
    #[test]
    fn test_generate_placeholder_html() {
        let html = generate_placeholder_html("/about");
        assert!(html.contains("Route: /about"));
        assert!(html.contains("<!DOCTYPE html>"));
    }
    #[test]
    fn test_compute_hash_with_deps() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");
        let dep_path = temp.path().join("dep.txt");
        
        fs::write(&file_path, "content").unwrap();
        fs::write(&dep_path, "dep content").unwrap();
        
        let mut cache = ExportCache::default();
        cache.update_hash(&dep_path);
        cache.add_dependency(&file_path, &dep_path);
        
        let hash1 = cache.compute_hash_with_deps(&file_path).unwrap();
        
        // Change dependency
        fs::write(&dep_path, "new dep content").unwrap();
        cache.update_hash(&dep_path); // Simulate re-hashing dependency first
        
        let hash2 = cache.compute_hash_with_deps(&file_path).unwrap();
        assert_ne!(hash1, hash2, "Hash should change when dependency changes");
    }

    #[test]
    fn test_extract_dependencies() {
        let temp = TempDir::new().unwrap();
        let views_dir = temp.path().join("views");
        fs::create_dir_all(&views_dir).unwrap();
        
        let view_path = views_dir.join("page.ncl");
        let layout_path = views_dir.join("main_layout.ncl");
        
        fs::write(&view_path, r#"<n:layout name="main_layout">Content</n:layout>"#).unwrap();
        fs::write(&layout_path, "Layout").unwrap();
        
        let mut cache = ExportCache::default();
        cache.extract_dependencies(&view_path, &views_dir).unwrap();
        
        let view_str = view_path.to_string_lossy().to_string();
        let deps = cache.dependencies.get(&view_str).unwrap();
        
        assert!(deps.iter().any(|d| d.contains("main_layout.ncl")));
    }

    #[test]
    fn test_check_cascade_rebuild() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("page.ncl");
        let layout_path = temp.path().join("layout.ncl");
        
        fs::write(&file_path, "content").unwrap();
        fs::write(&layout_path, "layout v1").unwrap();
        
        let mut cache = ExportCache::default();
        cache.update_hash(&file_path);
        cache.update_hash(&layout_path);
        cache.add_dependency(&file_path, &layout_path);
        
        // Change layout
        fs::write(&layout_path, "layout v2").unwrap();
        
        let affected = cache.check_cascade_rebuild(&[layout_path]);
        assert_eq!(affected.len(), 1);
        assert!(affected[0].contains("page.ncl"));
    }
}
