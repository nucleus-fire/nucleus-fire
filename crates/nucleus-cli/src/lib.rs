use clap::{Parser, Subcommand};
pub mod generate; // Register module
pub mod deploy;   // Deploy module with multi-platform support
pub mod export;   // Static export and publish module
pub mod console;  // Interactive REPL
pub mod studio;   // Database Studio web UI
pub mod animations; // CLI animations
use std::fs;
use std::path::Path;
use miette::IntoDiagnostic;
use rayon::prelude::*;


#[derive(Parser, Debug)]
#[command(name = "nucleus")]
#[command(version)]
#[command(about = "Nucleus Framework CLI", long_about = None)]
#[command(disable_version_flag = true)]
pub struct Cli {
    /// Print version information
    #[arg(short = 'v', short_alias = 'V', long = "version", action = clap::ArgAction::Version)]
    pub version: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scaffolds a new project
    New {
        name: String,
    },
    /// Starts the Reactor
    Run,
    /// Runs tests (Guardian)
    Test,
    /// Deploys the application (interactive multi-platform)
    Deploy {
        #[command(subcommand)]
        command: Option<DeployCommands>,

        /// Target platform: docker, fly, railway, render, manual
        #[arg(short, long)]
        target: Option<String>,
    },
    /// Installs a dependency (Crate or Nucleus Module)
    Install {
        package: String,
    },
    /// Compiles the application to a binary (AOT)
    Build,
    /// Development server with file watching and auto-rebuild
    Dev,
    /// Database management
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    /// Generates code (scaffold, model, etc.)
    Generate {
        #[command(subcommand)]
        command: generate::GenerateCommands,
    },
    /// Export static site (SSG)
    Export {
        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: String,
        /// Run interactive wizard
        #[arg(long)]
        wizard: bool,
        /// Incremental build (only rebuild changed files)
        #[arg(long)]
        incremental: bool,
        /// Base URL for generated links
        #[arg(long)]
        base_url: Option<String>,
        /// Target platform: netlify, vercel, cloudflare, github
        #[arg(long)]
        platform: Option<String>,
    },
    /// Publish static site to platform
    Publish {
        /// Target platform: netlify, vercel, cloudflare, github
        #[arg(short, long)]
        platform: Option<String>,
    },
    /// Browser automation tools (Chrome/Chromium)
    Browser {
        #[command(subcommand)]
        command: BrowserCommands,
    },
    /// Interactive database console (REPL)
    Console {
        /// Database URL (overrides config/environment)
        #[arg(short, long)]
        database: Option<String>,
    },
    /// Database browser web UI
    Studio {
        /// Database URL (overrides config/environment)
        #[arg(short, long)]
        database: Option<String>,
        /// Port to run on (default: 4000)
        #[arg(short, long, default_value = "4000")]
        port: u16,
    },
}

#[derive(Subcommand, Debug)]
pub enum DeployCommands {
    /// Initialize deployment configuration (Dockerfile, fly.toml, etc.)
    Init,
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Initialize migrations directory
    Init,
    /// Create a new migration file
    New { name: String },
    /// Run pending migrations
    Up {
        /// Number of migrations to apply (default: all)
        #[arg(short, long)]
        step: Option<usize>,
    },
    /// Rollback migrations
    Down {
        /// Number of migrations to rollback (default: 1)
        #[arg(short, long, default_value = "1")]
        step: usize,
    },
    /// Show migration status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum BrowserCommands {
    /// Install Chrome/Chromium for browser automation
    Install {
        /// Force reinstall even if Chrome is detected
        #[arg(long)]
        force: bool,
    },
    /// Check if Chrome is installed and working
    Check,
}

// Logic implementations
pub async fn run_cli() -> miette::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::New { name }) => {
            use std::io::Write;
            println!("⚛️  Creating Nucleus project: {}", name);
            std::io::stdout().flush().unwrap();
            
            if let Err(e) = create_project(name) {
                eprintln!("❌ Error creating project: {}", e);
            } else {
                println!("✅ Project {} created successfully!", name);
                println!("ℹ️  Tip: Put images in src/assets for automatic optimization.");
            }
        }
        Some(Commands::Generate { command }) => {
            if let Err(e) = generate::handle_generate(command) {
                eprintln!("❌ Error generating: {}", e);
            }
        }
        Some(Commands::Run) => {
            println!("⚛️  Starting Nucleus Reactor...");
            
            // 0. Optimize Assets (Dev Mode)
            let _ = optimize_css(); // Ignore errors in dev (e.g. if no assets)
            
            // interpreter mode: scan source files and render them
            let mut routes = std::collections::HashMap::new();
            
            if Path::new("nucleus.config").exists() {
                println!("📂 Found nucleus.config, loading views...");
                
                let views_path = Path::new("src/views");
                // Collect all paths to scan
                let mut paths_to_scan = Vec::new();
                
                if views_path.exists() {
                     if let Ok(entries) = fs::read_dir(views_path) {
                         for entry in entries.flatten() {
                             paths_to_scan.push(entry.path());
                         }
                     }
                }
                
                // Scan src/vendor
                let vendor_path = Path::new("src/vendor");
                if vendor_path.exists() {
                    if let Ok(entries) = fs::read_dir(vendor_path) {
                        for entry in entries.flatten() {
                            if entry.path().is_dir() {
                                if let Ok(subs) = fs::read_dir(entry.path()) {
                                    for sub in subs.flatten() {
                                        paths_to_scan.push(sub.path());
                                    }
                                }
                            }
                        }
                    }
                }

                for path in paths_to_scan {
                    if path.extension().is_some_and(|e| e == "ncl") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            println!("   - Compiling {}", path.display());
                                    // Parse and Render
                                    // INJECT LAYOUT LOGIC (JIT)
                                    // 1. Check for layout
                                    // Hack: Simple check for <n:layout> tag isn't enough, we need to merge nodes.
                                    // But we CAN'T merge nodes before we parse.
                                    
                                    match ncc::parse_code(&content) {
                                         Ok(mut nodes) => {
                                             // Check for parent layout
                                             if let Some(parent) = path.parent() {
                                                 let layout_path = parent.join("layout.ncl");
                                                 if layout_path.exists() {
                                                     if let Ok(mut layout_content) = fs::read_to_string(&layout_path) {
                                                         
                                                         // INJECT PRECONNECTS FROM CONFIG (JIT)
                                                         if let Ok(config) = nucleus_std::config::Config::try_load() {
                                                             if !config.performance.preconnect_origins.is_empty() {
                                                                 let mut tags = String::new();
                                                                 for origin in config.performance.preconnect_origins {
                                                                     tags.push_str(&format!(r#"<link rel="preconnect" href="{}">"#, origin));
                                                                 }
                                                                 layout_content = layout_content.replace("</head>", &format!("{}\n</head>", tags));
                                                             }
                                                         }
                                                         
                                                         if let Ok(layout_nodes) = ncc::parse_code(&layout_content) {
                                                              nodes = merge_layouts(layout_nodes, nodes);
                                                         }
                                                     }
                                                 }
                                             }

                                             let html = ncc::render_html(&nodes);
                                             
                                            // Route: home.ncl -> "home", blog/post.ncl -> "blog/post"
                                            // Calculate relative path from "src/views"
                                            let rel_path = path.strip_prefix("src/views").unwrap_or(&path);
                                            let route_key = rel_path.with_extension("").to_string_lossy().to_string();
                                            // Windows fix: replace \ with /
                                            let route_key = route_key.replace("\\", "/");
                                            
                                            // Handle index files: "blog/index" -> "blog/" (or just "blog")
                                            let route_key = if route_key.ends_with("/index") {
                                                route_key.strip_suffix("/index").unwrap().to_string()
                                            } else if route_key == "index" {
                                                "".to_string() // root
                                            } else {
                                                route_key
                                            };
                                            
                                            // Note: Atom runtime needs to handle empty string as "/" 
                                            // or we map "" -> "home" convention.
                                            let final_key = if route_key.is_empty() { "home".to_string() } else { route_key };
                                            
                                            routes.insert(final_key, html);
                                         },
                                         Err(e) => {
                                             return Err(e.into());
                                         }
                                    }
                                }
                            }
                        }
                        
                // Load static files (assets, js, css, etc.)
                let static_path = Path::new("static");
                if static_path.exists() {
                    fn load_static_recursive(base: &Path, dir: &Path, routes: &mut std::collections::HashMap<String, String>) {
                        if let Ok(entries) = fs::read_dir(dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_dir() {
                                    load_static_recursive(base, &path, routes);
                                } else if let Ok(content) = fs::read(&path) {
                                    // Get relative path from static/ folder
                                    if let Ok(rel) = path.strip_prefix(base) {
                                        // Route key: assets/home.js -> "assets/home.js"
                                        let key = rel.to_string_lossy().to_string().replace("\\", "/");
                                        // Store as string (for binary files this will be lossy but OK for js/css)
                                        routes.insert(key, String::from_utf8_lossy(&content).to_string());
                                    }
                                }
                            }
                        }
                    }
                    load_static_recursive(static_path, static_path, &mut routes);
                }
            } else {
                 println!("ℹ️  No nucleus.config found, running in standalone mode.");
                 routes.insert("home".to_string(), "<h1>Hello from Nucleus Standalone</h1>".to_string());
            }
            
            atom::start_reactor(Some(routes), None).await;
        }
        Some(Commands::Db { command }) => {
            handle_db_command(command).await?;
        },
        Some(Commands::Build) => {
            println!("⚛️  Compiling Nucleus App (AOT)...");
            build_project()?;
            println!("✅ Build complete! Run ./target/release/app to start.");
        }
        Some(Commands::Dev) => {
            use notify::{Watcher, RecursiveMode, recommended_watcher};
            use std::sync::mpsc::channel;
            use std::time::Duration;
            use std::process::{Command, Child, Stdio};
            
            // Show animated startup
            animations::show_startup_banner();
            
            // Initial build of Nucleus assets/code
            animations::with_spinner("Building Nucleus assets...", || {
                if let Err(e) = build_project() {
                    eprintln!("\n❌ Build failed: {}", e);
                }
            });

            // Explicitly compile the site binary so 'cargo run' is instant
            animations::with_spinner("Compiling application...", || {
                let status = Command::new("cargo")
                    .args(["build", "--bin", "site", "--quiet"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::inherit()) // Show errors if any
                    .status();
                    
                if let Ok(s) = status {
                    if !s.success() {
                        eprintln!("\n❌ Compilation failed");
                    }
                }
            });
            
            // Start cargo run in background
            fn start_server() -> Option<Child> {
                Command::new("cargo")
                    .args(["run", "--bin", "site", "--quiet"])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .ok()
            }
            
            let mut server_process = start_server();
            
            // Show animated dev server info
            animations::show_dev_server_start(3000);
            
            // Setup file watcher
            let (tx, rx) = channel();
            let mut watcher = recommended_watcher(move |res| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            }).expect("Failed to create file watcher");
            
            // Watch only NCL source directories
            let watch_paths = ["src/views", "src/components"];
            for path in &watch_paths {
                if Path::new(path).exists() {
                    let _ = watcher.watch(Path::new(path), RecursiveMode::Recursive);
                }
            }
            
            // Debounce: track last rebuild time  
            let mut last_rebuild = std::time::Instant::now();
            let debounce_duration = Duration::from_millis(1000);
            
            loop {
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(event) => {
                        use notify::EventKind;
                        
                        // Only trigger on modify/create events
                        let relevant_kind = matches!(
                            event.kind, 
                            EventKind::Modify(_) | EventKind::Create(_)
                        );
                        
                        // Only trigger on .ncl files
                        let is_ncl = event.paths.iter().any(|p| {
                            p.extension().and_then(|e| e.to_str()) == Some("ncl")
                        });
                        
                        if relevant_kind && is_ncl && last_rebuild.elapsed() > debounce_duration {
                            last_rebuild = std::time::Instant::now();
                            
                            // Get changed file name for display
                            let changed_file = event.paths.first()
                                .and_then(|p| p.file_name())
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "file".to_string());
                            
                            animations::show_file_change(&changed_file);
                            animations::show_rebuild_start();
                            
                            // Kill old server
                            if let Some(ref mut child) = server_process {
                                let _ = child.kill();
                                let _ = child.wait();
                            }
                            
                            // Rebuild assets
                            if let Err(e) = build_project() {
                                eprintln!("❌ Build failed: {}", e);
                            } else {
                                // Recompile binary
                                let _ = Command::new("cargo")
                                    .args(["build", "--bin", "site", "--quiet"])
                                    .stdout(Stdio::null())
                                    .stderr(Stdio::inherit())
                                    .status();

                                animations::show_rebuild_complete();
                                
                                // Restart server
                                server_process = start_server();
                                println!("  {}Restarted{}\n", animations::colors::DIM, animations::colors::RESET);
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // No events, continue watching
                    }
                    Err(_) => break,
                }
            }
            
            // Cleanup on exit
            if let Some(ref mut child) = server_process {
                let _ = child.kill();
            }
        }
        Some(Commands::Test) => {
            println!("⚛️  Running Guardian...");
            println!("✅ All tests passed (Stub).");
        }
        Some(Commands::Install { package }) => {
            handle_install(package)?;
        }
        Some(Commands::Deploy { command, target }) => {
            if let Some(DeployCommands::Init) = command {
                 deploy::run_init()?;
            } else {
                 deploy::run_deploy(target.clone())?;
            }
        }
        Some(Commands::Export { output, wizard, incremental, base_url, platform }) => {
            if *wizard {
                let config = export::run_export_wizard()?;
                export::run_export(config)?;
            } else {
                let config = export::ExportConfig {
                    output_dir: std::path::PathBuf::from(output),
                    base_url: base_url.clone(),
                    incremental: *incremental,
                    minify: true,
                    convert_webp: true,
                    platform: platform.as_ref().and_then(|p| export::Platform::parse(p)),
                };
                export::run_export(config)?;
            }
        }
        Some(Commands::Publish { platform }) => {
            export::run_publish(platform.clone())?;
        }
        Some(Commands::Browser { command }) => {
            handle_browser_command(command)?;
        }
        Some(Commands::Console { database }) => {
            console::run_console(database.clone()).await?;
        }
        Some(Commands::Studio { database, port }) => {
            studio::run_studio(database.clone(), *port).await?;
        }

        None => {
            println!("Welcome to Nucleus. Use --help to see commands.");
        }
    }
    Ok(())
}

pub fn build_project() -> miette::Result<()> {
    // 1. Image Optimization Pipeline
    optimize_images()?;
    optimize_css()?;
    


    // 1b. NPM Setup (Phase 5: Adoption)
    let package_json = Path::new("package.json");
    if !package_json.exists() {
        println!("📦 Initializing NPM Project...");
        std::process::Command::new("npm")
            .args(["init", "-y"])
            .stdout(std::process::Stdio::null())
            .status()
            .into_diagnostic()?;
    }

    // 2. Scan views & Vendor Modules
    let mut files_to_scan = Vec::new();
    let mut sitemap_routes = Vec::new(); // Track routes for Sitemap

    // src/views (Recursive)
    let views_path = Path::new("src/views");
    if views_path.exists() {
        for entry in walkdir::WalkDir::new(views_path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().is_some_and(|e| e == "ncl") {
                 files_to_scan.push(entry.path().to_path_buf());
            }
        }
    }
    
    // src/vendor (Modules)
    let vendor_path = Path::new("src/vendor");
    if vendor_path.exists() {
        // Read one level deep (src/vendor/MODULE/VIEW.ncl)
        for entry in fs::read_dir(vendor_path).into_diagnostic()?.flatten() {
            if entry.path().is_dir() {
                for sub in fs::read_dir(entry.path()).into_diagnostic()?.flatten() {
                     files_to_scan.push(sub.path());
                }
            }
        }
    }
    
    

    let mut handlers = String::new();
    let mut router_match = String::new();

    // PARALLEL PROCESSING (Unrivalled Speed)
    // Map: File -> (Handler Code, Router Code, Sitemap entries, WASM Fragments, TS Files)
    #[allow(clippy::type_complexity)]
    let results: Vec<_> = files_to_scan.par_iter().map(|path| -> miette::Result<(String, String, Vec<String>, Vec<String>, Vec<String>)> {
            if path.extension().is_some_and(|e| e == "ncl") {
                let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                
                // SKIP layout.ncl (it is not a route itself)
                if stem == "layout" {
                     return Ok((String::new(), String::new(), Vec::new(), Vec::new(), Vec::new()));
                }

                let content = fs::read_to_string(path).map_err(|e| miette::miette!(e))?;
                let mut raw_nodes = ncc::parse_code(&content)?;
                
                // CHECK FOR LAYOUT WRAPPING
                if let Some(parent) = path.parent() {
                    let layout_path = parent.join("layout.ncl");
                    if layout_path.exists() {
                         let mut layout_content = fs::read_to_string(&layout_path).map_err(|e| miette::miette!("Failed to read layout: {}", e))?;
                         
                         // INJECT PRECONNECTS FROM CONFIG (Build-Time)
                         if let Ok(config) = nucleus_std::config::Config::try_load() {
                             if !config.performance.preconnect_origins.is_empty() {
                                 let mut tags = String::new();
                                 for origin in config.performance.preconnect_origins {
                                     tags.push_str(&format!(r#"<link rel="preconnect" href="{}">"#, origin));
                                 }
                                 // Inject before </head>
                                 layout_content = layout_content.replace("</head>", &format!("{}\n</head>", tags));
                             }
                         }

                         let layout_nodes = ncc::parse_code(&layout_content)?;
                         
                         // Merge: Layout wraps View
                         raw_nodes = merge_layouts(layout_nodes, raw_nodes);
                    }
                }

                // Guardian Validation (A11y & Spec)
                if let Err(e) = ncc::guardian::Guardian::new().validate(&raw_nodes) {
                      return Err(miette::miette!("Validation Error in {}: {}", path.display(), e));
                }
                
                // --- OPTIMIZATION: Asset Extraction ---
                let mut nodes = Vec::new();
                let mut local_ts_files = Vec::new();
                let mut local_wasm_fragments = Vec::new();
                
                for node in raw_nodes {
                     nodes.push(optimize_node(node, &stem, &mut local_ts_files, &mut local_wasm_fragments));
                }
                
                let mut local_handler = String::new();
                let mut local_router = String::new();
                let mut local_sitemap = Vec::new();
                
                let fn_name = format!("handle_{}", stem);
                let mut view_found = false;
                let mut has_action = false;
                
                for node in &nodes {
                        match node {
                            ncc::ast::Node::Element(el) if el.tag_name == "n:view" => {
                                local_handler.push_str(&ncc::generate_view_handler_fn(el, &fn_name));
                                
                                // NEW: Action Handler
                                let action_fn_name = format!("handle_action_{}", stem);
                                let action_code = ncc::generate_action_handler_fn(el, &action_fn_name);
                                if !action_code.is_empty() {
                                    local_handler.push_str(&action_code);
                                    has_action = true;
                                }
                                
                                view_found = true;
                            },
                            ncc::ast::Node::Model(model) => {
                                local_handler.push_str(&ncc::generate_model(model));
                            },
                            _ => {}
                        }
                }
                if !view_found {
                        // Fallback: Raw Layout Mode (No n:view wrapper)
                        local_handler.push_str(&ncc::generate_nodes_handler_body(&nodes, &fn_name));
                        
                        // Check for Action recursively
                        if let Some(action_code) = ncc::find_action_recursive(&nodes) {
                            let action_fn_name = format!("handle_action_{}", stem);
                            local_handler.push_str(&format!(
                                "#[allow(non_snake_case, unreachable_code, unused_variables)]\nasync fn {}(headers: axum::http::HeaderMap, Form(params): Form<std::collections::HashMap<String, String>>) -> impl axum::response::IntoResponse {{\n    {}\n    axum::response::Html(\"Action Completed\").into_response()\n}}\n\n",
                                action_fn_name, action_code
                            ));
                            has_action = true;
                        }
                }

                if stem == "home" || stem == "index" {

                        local_sitemap.push("/".to_string());
                        if has_action {
                            local_router.push_str(&format!(r#".route("/", get(handle_{}).post(handle_action_{}))"#, stem, stem));
                            local_router.push_str(&format!(r#".route("/index", get(handle_{}).post(handle_action_{}))"#, stem, stem));
                        } else {
                            local_router.push_str(&format!(r#".route("/", get(handle_{}))"#, stem));
                            local_router.push_str(&format!(r#".route("/index", get(handle_{}))"#, stem));
                        }
                } else if stem.starts_with('[') && stem.ends_with(']') {
                        let param = &stem[1..stem.len()-1];
                        let route_path = format!("/:{}", param);
                        if has_action {
                             local_router.push_str(&format!(r#".route("{}", get(handle_{}).post(handle_action_{}))"#, route_path, stem, stem));
                        } else {
                             local_router.push_str(&format!(r#".route("{}", get(handle_{}))"#, route_path, stem));
                        }
                } else {
                        local_sitemap.push(format!("/{}", stem));
                        if has_action {
                            local_router.push_str(&format!(r#".route("/{0}", get(handle_{0}).post(handle_action_{0}))"#, stem));
                        } else {
                            local_router.push_str(&format!(r#".route("/{0}", get(handle_{0}))"#, stem));
                        }
                }
                
                Ok((local_handler, local_router, local_sitemap, local_wasm_fragments, local_ts_files))
            } else {
                Ok((String::new(), String::new(), Vec::new(), Vec::new(), Vec::new()))
            }
    }).collect::<Result<Vec<_>, _>>()?;

    // Reduce
    let mut combined_wasm_fragments = Vec::new();
    let mut all_ts_files = Vec::new();
    for (h, r, s, w, t) in results {
        handlers.push_str(&h);
        router_match.push_str(&r);
        sitemap_routes.extend(s);
        combined_wasm_fragments.extend(w);
        all_ts_files.extend(t);
    }

    // TypeScript Compilation
    if !all_ts_files.is_empty() {
        println!("Type-Checking & Bundling TypeScript...");
        let mut args = vec!["esbuild"];
        for f in &all_ts_files {
            args.push(f);
        }
        args.push("--bundle");
        args.push("--outdir=static/js");
        args.push("--format=esm"); 
        args.push("--platform=browser");
        args.push("--minify"); 

        let status = std::process::Command::new("npx")
            .args(&args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();
             
        if let Ok(s) = status {
             if !s.success() { eprintln!("⚠️  TypeScript build failed."); }
        } else {
             eprintln!("⚠️  npx/esbuild not found.");
        }
    }

    // Generalized WASM Build
    if !combined_wasm_fragments.is_empty() {
            println!("⚛️  Compiling Client WASM Bundle...");
            
            let mut wasm_source = ncc::generate_wasm_header();
            for fragment in combined_wasm_fragments {
                wasm_source.push_str("    {\n");
                wasm_source.push_str(&fragment);
                wasm_source.push_str("\n    }\n");
            }
            wasm_source.push_str(&ncc::generate_wasm_footer());
            
            fs::write("src/lib.rs", &wasm_source).into_diagnostic()?;
            
            let cargo_path = Path::new("Cargo.toml");
            let mut cargo_toml = fs::read_to_string(cargo_path).into_diagnostic()?;
            if !cargo_toml.contains("crate-type") {
                cargo_toml.push_str("\n[lib]\ncrate-type = [\"cdylib\", \"rlib\"]\n");
                fs::write(cargo_path, cargo_toml).into_diagnostic()?;
            }
            
            let status = std::process::Command::new("wasm-pack")
                .args(["build", "--target", "web", "--out-dir", "static/pkg"])
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status();
            
            if let Ok(s) = status {
                if !s.success() { eprintln!("⚠️  wasm-pack failed."); }
            } else {
                eprintln!("⚠️  wasm-pack not found.");
            }
    }

    // 2. Generate optimized main.rs
    let has_middleware = Path::new("src/middleware.rs").exists();
    let middleware_mod = if has_middleware {
        r#"#[path = "../middleware.rs"] mod middleware;"#
    } else {
        ""
    };
    let middleware_layer = if has_middleware {
        r#"let app = app.layer(axum::middleware::from_fn(crate::middleware::global_middleware));"#
    } else {
        ""
    };
    let has_logic = Path::new("src/logic").exists();
    let logic_mod = if has_logic {
        r#"#[path = "../logic/mod.rs"] pub mod logic;"#
    } else {
        ""
    };

    let has_logic_api = Path::new("src/logic/api.rs").exists();
    let logic_routes = if has_logic_api {
        ".merge(logic::api::routes())"
    } else {
        ""
    };

    let has_models = Path::new("src/models").exists();
    let models_mod = if has_models {
        r#"#[path = "../models/mod.rs"] pub mod models;"#
    } else {
        ""
    };

    let main_code = format!(
        r#"#![allow(unused_imports, clippy::single_char_add_str)]
        use axum::{{response::{{Html, IntoResponse}}, routing::get, extract::{{Query, Form}}, Router}};
        use tower_http::services::ServeDir;
        use tower_http::compression::CompressionLayer;
        use tokio::net::TcpListener;
        use serde::{{Serialize, Deserialize}};
        
        // --- Static Assets (Zero-Allocation) ---
        {}

        // Middleware Module Support
        {}

        // Logic Module Support
        {}

        // Models Module Support
        {}

        use mimalloc::MiMalloc;
        #[global_allocator]
        static GLOBAL: MiMalloc = MiMalloc;

        pub async fn app() -> Router {{
            // Load Configuration
            let config = nucleus_std::config::Config::load();
            
            // Initialize Database
            // Only init if URL is set (simple check)
            if !config.database.url.is_empty() {{
                 match nucleus_std::photon::init_db(&config.database.url).await {{
                     Ok(_) => {{
                         println!("✅ Database initialized on {{}}", config.database.url);
                         // Auto-run migrations
                         match nucleus_std::photon::run_migrations("migrations").await {{
                             Ok(applied) => {{
                                 if !applied.is_empty() {{
                                     println!("✅ Applied {{}} migration(s)", applied.len());
                                 }}
                             }}
                             Err(e) => eprintln!("⚠️ Migration error: {{}}", e),
                         }}
                     }}
                     Err(e) => eprintln!("❌ Database initialization failed: {{}}", e),
                 }}
            }}

            // Static Router with Zero-Allocation Assets
            #[allow(clippy::let_and_return)]
            {{
                let app = Router::new()
                    {}
                    .nest_service("/pkg", ServeDir::new("static/pkg"))
                    .nest_service("/assets", ServeDir::new("static/assets"))
                    .nest_service("/static", ServeDir::new("static"))
                    .nest_service("/docs/raw", ServeDir::new("../../docs/en"))
                    .route_service("/docs/manifest.json", tower_http::services::ServeFile::new("static/docs/manifest.json"))
                    .layer(tower_http::set_header::SetResponseHeaderLayer::if_not_present(
                        axum::http::header::CACHE_CONTROL,
                        axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
                    ))
                    .layer(CompressionLayer::new().br(true).gzip(true))
                    {};
                    
                // Auto-Inject Middleware if `src/middleware.rs` exists
                {}

                app
            }}
        }}
        "#,
        handlers, // Statics
        middleware_mod, // Generated mod
        logic_mod, // Generated mod
        models_mod, // Generated mod
        router_match, // .route() calls
        logic_routes, // .merge() calls
        middleware_layer // Layer application
    );

    // 3. Write to src/app_generated.rs 

    fs::write("src/app_generated.rs", &main_code).into_diagnostic()?;
    
    // 4. Update Sitemap
    // if !sitemap_routes.is_empty() {
    //     let sitemap_xml = ncc::generate_sitemap(&sitemap_routes, "https://example.com");
    //     fs::write("static/sitemap.xml", sitemap_xml).into_diagnostic()?;
    // }
    
    println!("✅ Build Complete.");
    Ok(())
}


pub fn create_project(name: &str) -> std::io::Result<()> {
    let path = Path::new(name);
    if path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Directory already exists",
        ));
    }

    // 1. Directory Structure
    fs::create_dir(path)?;
    fs::create_dir(path.join("src"))?;
    fs::create_dir(path.join("src/models"))?;
    fs::create_dir(path.join("src/views"))?;
    fs::create_dir(path.join("src/logic"))?;
    fs::create_dir(path.join("src/assets"))?; // New: Asset Pipeline source
    fs::create_dir(path.join("src/vendor"))?;
    fs::create_dir(path.join("migrations"))?;
    fs::create_dir(path.join("static"))?;

    // 2. Configs
    fs::write(path.join("nucleus.config"), r#"version = "1.0.0"

[server]
port = 3000
host = "0.0.0.0"
# environment = "development"

[database]
url = "sqlite:nucleus.db"
# url = "${DATABASE_URL}"  # Use env var in production

[app]
name = "__NAME__"
# secret_key = "${SECRET_KEY}"  # Required for auth

# Performance Configuration (optimized defaults)
[performance]
compression = true
inline_critical_css = true

[performance.cache]
css_max_age = 31536000      # 1 year
js_max_age = 31536000       # 1 year
font_max_age = 31536000     # 1 year
image_max_age = 604800      # 1 week
html_no_cache = true
immutable = true

[performance.fonts]
display_swap = true         # font-display: swap (prevents FOIT)
preconnect = true           # Add preconnect hints
async_load = true           # Non-render-blocking loading
# google_fonts_url = "https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap"
"#.replace("__NAME__", name))?;

    fs::write(path.join("content.deck"), r#"
welcome_title:en = Welcome to Nucleus
welcome_message:en = The high-performance, unified web framework.
"#)?;

    fs::write(path.join(".gitignore"), r#"
/target
/.env
/nucleus.db
**/*.DS_Store
"#)?;

    // README.md (Safe Replace)
    let readme = r#"# __NAME__

Generated by Nucleus Framework.

## Getting Started

1. **Install Dependencies**
   ```bash
   nucleus install serde
   ```

2. **Run the Server**
   ```bash
   nucleus run
   ```
   Server will start at http://localhost:3000

## Project Structure
- `src/views`: Your UI components (`.ncl` files)
- `src/logic`: Backend Rust logic
- `src/models`: Database Schemas
- `migrations`: SQL migration files
"#.replace("__NAME__", name);
    fs::write(path.join("README.md"), readme)?;

    // 3. Default Migration
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    // let timestamp = "20250101000000";
    let filename = format!("migrations/{}_init.sql", timestamp);
    fs::write(path.join(&filename), r#"-- Initial Migration
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
"#)?;

    // 4. Default Content (Index View)
    fs::write(path.join("src/views/index.ncl"), r#"<n:view title="Welcome to Nucleus">
<n:layout name="layout">
    <div class="container">
        <header>
            <h1><n:text key="welcome_title" /></h1>
            <p><n:text key="welcome_message" /></p>
        </header>

        <main>
            <div class="card">
                <h2>🚀 You are running on Nucleus Framework</h2>
                <p>Edit <code>src/views/index.ncl</code> to see changes instantly (HMR).</p>
            </div>
            
            <div class="features">
                <div class="feature">
                    <h3>⚡️ Fast</h3>
                    <p>Compiled to binary. No GC pauses.</p>
                </div>
                <div class="feature">
                    <h3>🔋 Battery Included</h3>
                    <p>Database, Auth, and WASM in one box.</p>
                </div>
            </div>
        </main>
    </div>
</n:layout>
</n:view>
"#)?;

    // 4b. Optimized Layout with font preloading and cache hints
    fs::write(path.join("src/views/layout.ncl"), r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <meta name="description" content="Built with Nucleus Framework">
    
    <!-- DNS Prefetch & Preconnect (Performance) -->
    <link rel="dns-prefetch" href="//fonts.googleapis.com">
    <link rel="dns-prefetch" href="//fonts.gstatic.com">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    
    <!-- Critical CSS (inlined for render performance) -->
    <style>
        body { margin: 0; font-family: 'Inter', system-ui, sans-serif; background: #111; color: #fff; line-height: 1.5; }
        .container { max-width: 800px; margin: 0 auto; padding: 2rem; }
        header { text-align: center; margin-bottom: 3rem; }
        h1 { font-size: 2.5rem; background: linear-gradient(to right, #4facfe 0%, #00f2fe 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }
        .card { background: #222; padding: 1.5rem; border-radius: 8px; border: 1px solid #333; margin-bottom: 2rem; }
        .features { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
        .feature { background: #1a1a1a; padding: 1rem; border-radius: 6px; }
        code { background: #333; padding: 0.2rem 0.4rem; border-radius: 4px; font-family: monospace; }
    </style>
    
    <!-- Stylesheets -->
    <link href="/assets/style.css" rel="stylesheet">
    
    <!-- Google Fonts (non-render-blocking with media swap) -->
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet" media="print" onload="this.media='all'">
    <noscript><link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet"></noscript>
</head>
<body>
    <n:slot name="content" />
</body>
</html>
"#)?;

    // 5. Cargo.toml
    let cargo_toml = String::from(r#"[package]
name = "__NAME__"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
chrono = "0.4"
simd-json = "0.13"
mimalloc = "0.1"
tower-http = { version = "0.5", features = ["fs", "trace", "compression-full"] }
nucleus-std = { path = "../crates/nucleus-std" } # dev path
atom = { path = "../crates/atom" } # dev path
"#).replace("__NAME__", name);

    fs::write(path.join("Cargo.toml"), cargo_toml)?;

    Ok(())
}

async fn handle_db_command(cmd: &DbCommands) -> miette::Result<()> {
    match cmd {
        DbCommands::Init => {
            fs::create_dir_all("migrations").into_diagnostic()?;
            println!("✅ Created migrations directory");
        },
        DbCommands::New { name } => {
            let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
            let filename = format!("migrations/{}_{}.sql", timestamp, name);
            fs::create_dir_all("migrations").into_diagnostic()?;
            fs::write(&filename, format!(
                "-- Migration: {}\n-- Created: {}\n\n-- UP\n\n\n-- DOWN\n\n",
                name,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            )).into_diagnostic()?;
            println!("✅ Created migration: {}", filename);
        },
        DbCommands::Up { step } => {
            println!("⚛️  Applying migrations...");
            
            // Connect to DB using Config
            let config = nucleus_std::config::Config::load();
            let db_url = config.database.url;

            // Ensure SQLite file exists if local
            if db_url.starts_with("sqlite:") {
                 let path = db_url.trim_start_matches("sqlite:");
                 if !Path::new(path).exists() {
                     fs::File::create(path).into_diagnostic()?;
                 }
            }
            
            let pool = sqlx::SqlitePool::connect(&db_url).await.into_diagnostic()?;
            
            // Create migrations table
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS _migrations (
                    id INTEGER PRIMARY KEY,
                    key TEXT UNIQUE NOT NULL,
                    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )"
            ).execute(&pool).await.into_diagnostic()?;

            // Read migrations dir
            let mut entries = fs::read_dir("migrations").into_diagnostic()?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "sql"))
                .collect::<Vec<_>>();
            entries.sort();

            let mut applied_count = 0;
            for path in entries {
                if let Some(max) = step {
                    if applied_count >= *max {
                        break;
                    }
                }
                
                let key = path.file_name().unwrap().to_string_lossy().to_string();
                
                // Check if applied
                let applied: bool = sqlx::query("SELECT 1 FROM _migrations WHERE key = ?")
                    .bind(&key)
                    .fetch_optional(&pool).await.into_diagnostic()?
                    .is_some();

                if !applied {
                    let sql = fs::read_to_string(&path).into_diagnostic()?;
                    // Extract UP migration (everything before -- DOWN)
                    let up_sql = if let Some(pos) = sql.find("-- DOWN") {
                        sql[..pos].trim()
                    } else {
                        sql.trim()
                    };
                    
                    // Execute
                    sqlx::query(up_sql).execute(&pool).await.into_diagnostic()?;
                    // Record
                    sqlx::query("INSERT INTO _migrations (key) VALUES (?)")
                        .bind(&key)
                        .execute(&pool).await.into_diagnostic()?;
                    
                    println!("🚀 Applied: {}", key);
                    applied_count += 1;
                }
            }
            
            if applied_count == 0 {
                println!("✨ No pending migrations.");
            } else {
                println!("✨ Applied {} migration(s).", applied_count);
            }
        },
        DbCommands::Down { step } => {
            println!("⏪ Rolling back {} migration(s)...", step);
            
            let config = nucleus_std::config::Config::load();
            let db_url = config.database.url;
            let pool = sqlx::SqlitePool::connect(&db_url).await.into_diagnostic()?;
            
            // Get last N applied migrations
            let rows: Vec<(String,)> = sqlx::query_as(
                "SELECT key FROM _migrations ORDER BY applied_at DESC LIMIT ?"
            )
            .bind(*step as i64)
            .fetch_all(&pool).await.into_diagnostic()?;
            
            for (key,) in rows {
                // Try to find migration file
                let path = std::path::PathBuf::from(format!("migrations/{}", key));
                
                if path.exists() {
                    let sql = fs::read_to_string(&path).into_diagnostic()?;
                    // Extract DOWN migration (everything after -- DOWN)
                    if let Some(pos) = sql.find("-- DOWN") {
                        let down_sql = sql[pos + 7..].trim();
                        if !down_sql.is_empty() {
                            sqlx::query(down_sql).execute(&pool).await.into_diagnostic()?;
                        }
                    }
                }
                
                // Remove from tracking
                sqlx::query("DELETE FROM _migrations WHERE key = ?")
                    .bind(&key)
                    .execute(&pool).await.into_diagnostic()?;
                
                println!("⏪ Rolled back: {}", key);
            }
            
            println!("✨ Rollback complete.");
        },
        DbCommands::Status => {
            let config = nucleus_std::config::Config::load();
            let db_url = config.database.url;
            
            // Ensure SQLite file exists if local
            if db_url.starts_with("sqlite:") {
                let path = db_url.trim_start_matches("sqlite:");
                if !Path::new(path).exists() {
                    println!("📄 No database file found. Run `nucleus db up` first.");
                    return Ok(());
                }
            }
            
            let pool = sqlx::SqlitePool::connect(&db_url).await.into_diagnostic()?;
            
            // Create migrations table if not exists
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS _migrations (
                    id INTEGER PRIMARY KEY,
                    key TEXT UNIQUE NOT NULL,
                    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )"
            ).execute(&pool).await.into_diagnostic()?;
            
            // Get applied migrations
            let applied: Vec<(String,)> = sqlx::query_as("SELECT key FROM _migrations")
                .fetch_all(&pool).await.into_diagnostic()?;
            let applied_set: std::collections::HashSet<_> = applied.into_iter().map(|(k,)| k).collect();
            
            // Read migrations dir
            let mut entries: Vec<_> = fs::read_dir("migrations")
                .into_diagnostic()?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "sql"))
                .collect();
            entries.sort();
            
            println!("\n📊 Migration Status:");
            println!("{}", "-".repeat(50));
            
            for path in entries {
                let key = path.file_name().unwrap().to_string_lossy().to_string();
                let status = if applied_set.contains(&key) { "✅" } else { "⏳" };
                let label = if applied_set.contains(&key) { "applied" } else { "pending" };
                println!("{} {} ({})", status, key, label);
            }
            println!("{}", "-".repeat(50));
        }
    }
    Ok(())
}

fn handle_browser_command(cmd: &BrowserCommands) -> miette::Result<()> {
    match cmd {
        BrowserCommands::Check => {
            println!("🔍 Checking for Chrome/Chromium installation...\n");
            
            match detect_chrome() {
                Some(path) => {
                    println!("✅ Chrome found at: {}", path);
                    println!("\n   Browser automation is ready to use!");
                    println!("   Use `nucleus_std::browser::Browser::launch()` in your code.");
                }
                None => {
                    println!("❌ Chrome/Chromium not found.");
                    println!("\n   Run `nucleus browser install` to install it,");
                    println!("   or install Chrome manually from https://www.google.com/chrome/");
                }
            }
        }
        BrowserCommands::Install { force } => {
            if !force {
                if let Some(path) = detect_chrome() {
                    println!("✅ Chrome already installed at: {}", path);
                    println!("   Use --force to reinstall.");
                    return Ok(());
                }
            }
            
            install_chrome()?;
        }
    }
    Ok(())
}

fn detect_chrome() -> Option<String> {
    // Check common Chrome/Chromium paths
    let paths = if cfg!(target_os = "macos") {
        vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
        ]
    } else if cfg!(target_os = "windows") {
        vec![
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ]
    } else {
        // Linux
        vec![
            "/usr/bin/google-chrome",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "/snap/bin/chromium",
        ]
    };
    
    for path in paths {
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    
    // Try which/where command
    let cmd = if cfg!(target_os = "windows") { "where" } else { "which" };
    let browsers = ["google-chrome", "chromium", "chromium-browser"];
    
    for browser in browsers {
        if let Ok(output) = std::process::Command::new(cmd).arg(browser).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
    }
    
    None
}

fn install_chrome() -> miette::Result<()> {
    println!("📦 Installing Chrome/Chromium for browser automation...\n");
    
    if cfg!(target_os = "macos") {
        println!("🍎 macOS detected - Installing via Homebrew...\n");
        
        // Check if brew is available
        let brew_check = std::process::Command::new("which")
            .arg("brew")
            .output();
        
        if brew_check.is_err() || !brew_check.unwrap().status.success() {
            println!("⚠️  Homebrew not found. Please install Chrome manually:");
            println!("   https://www.google.com/chrome/");
            println!("\n   Or install Homebrew first:");
            println!("   /bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"");
            return Ok(());
        }
        
        let status = std::process::Command::new("brew")
            .args(["install", "--cask", "chromium"])
            .status()
            .into_diagnostic()?;
        
        if status.success() {
            println!("\n✅ Chromium installed successfully!");
        } else {
            println!("\n⚠️  Installation failed. Try installing Chrome manually.");
        }
    } else if cfg!(target_os = "linux") {
        println!("🐧 Linux detected\n");
        
        // Try apt first (Debian/Ubuntu)
        if Path::new("/usr/bin/apt").exists() {
            println!("   Using apt to install chromium-browser...\n");
            let status = std::process::Command::new("sudo")
                .args(["apt", "install", "-y", "chromium-browser"])
                .status()
                .into_diagnostic()?;
            
            if status.success() {
                println!("\n✅ Chromium installed successfully!");
                return Ok(());
            }
        }
        
        // Try dnf (Fedora/RHEL)
        if Path::new("/usr/bin/dnf").exists() {
            println!("   Using dnf to install chromium...\n");
            let status = std::process::Command::new("sudo")
                .args(["dnf", "install", "-y", "chromium"])
                .status()
                .into_diagnostic()?;
            
            if status.success() {
                println!("\n✅ Chromium installed successfully!");
                return Ok(());
            }
        }
        
        // Fallback instructions
        println!("⚠️  Automatic installation not available for your distro.");
        println!("   Please install Chromium manually using your package manager:");
        println!("   - Debian/Ubuntu: sudo apt install chromium-browser");
        println!("   - Fedora/RHEL: sudo dnf install chromium");
        println!("   - Arch: sudo pacman -S chromium");
    } else if cfg!(target_os = "windows") {
        println!("🪟 Windows detected\n");
        println!("   Please install Chrome manually from:");
        println!("   https://www.google.com/chrome/");
        println!("\n   Or use winget:");
        println!("   winget install Google.Chrome");
    }
    
    Ok(())
}


pub fn optimize_images() -> miette::Result<()> {
    let src_dir = Path::new("src/assets");
    let out_dir = Path::new("static/assets");
    
    if !src_dir.exists() { return Ok(()); }
    fs::create_dir_all(out_dir).into_diagnostic()?;
    
    println!("🎨 Optimizing Assets...");
    
    for entry in fs::read_dir(src_dir).into_diagnostic()?.flatten() {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
             if ["jpg", "jpeg", "png"].contains(&ext.to_lowercase().as_str()) {
                 println!("   - Processing {}", path.display());
                 
                 // Open
                 if let Ok(img) = image::open(&path) {
                     // Resize Logic (Default: Max 1920 width)
                     let processed = if img.width() > 1920 {
                         img.resize(1920, 1080, image::imageops::FilterType::Lanczos3)
                     } else {
                         img
                     };
                     
                     // Helper to save format
                     let stem = path.file_stem().unwrap().to_str().unwrap();
                     
                     // 1. WebP (Modern)
                     let webp_path = out_dir.join(format!("{}.webp", stem));
                     // Note: image crate WebP encoder is basic, usually specialized crates used, 
                     // but we save as is for now or just copy if encoding fails context.
                     // Actually, image crate supports saving as webp if feature enabled.
                     // We enabled it in Cargo.toml.
                     processed.save(&webp_path).ok(); // Ignore errors for prototype
                     
                     // 2. Original Fallback (Optimized/Copied)
                     let out_path = out_dir.join(path.file_name().unwrap());
                     processed.save(&out_path).into_diagnostic()?;
                 }
             }
        }
    }
    Ok(())
}

pub fn optimize_css() -> miette::Result<()> {
    let src_dir = Path::new("src/assets");
    let out_dir = Path::new("static/assets");

    if !src_dir.exists() { return Ok(()); }
    fs::create_dir_all(out_dir).into_diagnostic()?;

    println!("🎨 Optimizing CSS...");
    
    // Collect CSS files
    let mut css_files = Vec::new();
    for entry in fs::read_dir(src_dir).into_diagnostic()?.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "css") {
            css_files.push(path);
        }
    }

    if css_files.is_empty() { return Ok(()); }

    // 1. Optimize src/assets -> static/assets
    if !css_files.is_empty() {
        let mut args = vec!["esbuild"];
        for f in &css_files {
            args.push(f.to_str().unwrap());
        }
        args.push("--outdir=static/assets");
        args.push("--minify");
        
        // Execute
        let _ = std::process::Command::new("npx")
            .args(&args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();
    }

    // 2. Minify any OTHER css in static/assets (like output.css from Tailwind)
    // We run this separately to catch files that don't satisfy the src->dest mapping above.
    // Ideally we'd just use one pass, but preserving the existing src->dest logic is safer.
    if out_dir.exists() {
        let mut static_css = Vec::new();
        if let Ok(entries) = fs::read_dir(out_dir) {
            for entry in entries.flatten() {
                 let path = entry.path();
                 if path.extension().is_some_and(|e| e == "css") {
                     static_css.push(path);
                 }
            }
        }
        
        if !static_css.is_empty() {
            let mut args = vec!["esbuild"];
             for f in &static_css {
                args.push(f.to_str().unwrap());
            }
            args.push("--outdir=static/assets");
            args.push("--minify");
            args.push("--allow-overwrite");
            
            let _ = std::process::Command::new("npx")
                .args(&args)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status();
        }
    }

    Ok(())
}

pub fn generate_sitemap(routes: &[String]) -> miette::Result<()> {
    println!("🗺️  Generating sitemap.xml...");
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    
    // Domain should be configurable, default to placeholders
    let domain = "http://localhost:3000"; 
    
    for route in routes {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}{}</loc>\n", domain, route));
        xml.push_str("    <lastmod>2025-01-01</lastmod>\n"); // Should use file mtime
        xml.push_str("    <changefreq>daily</changefreq>\n");
        xml.push_str("  </url>\n");
    }
    
    xml.push_str("</urlset>");
    fs::create_dir_all("static").into_diagnostic()?;
    fs::write("static/sitemap.xml", xml).into_diagnostic()?;
    Ok(())
}

pub fn generate_pwa() -> miette::Result<()> {
    println!("📱 Generating PWA Manifest & Service Worker...");
    
    // Manifest
    // Manifest
    let manifest = r##"{
    "name": "Nucleus App",
    "short_name": "Nucleus",
    "start_url": "/",
    "display": "standalone",
    "background_color": "#ffffff",
    "theme_color": "#000000",
    "icons": [
        { "src": "/assets/icon-192.png", "sizes": "192x192", "type": "image/png" },
        { "src": "/assets/icon-512.png", "sizes": "512x512", "type": "image/png" }
    ]
}"##;
    fs::create_dir_all("static").into_diagnostic()?;
    fs::write("static/manifest.json", manifest).into_diagnostic()?;
    
    // Service Worker (Offline Cache)
    let sw = r#"
const CACHE_NAME = 'nucleus-v1';
const ASSETS = ['/', '/static/main.css', '/manifest.json'];

self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS)));
});

self.addEventListener('fetch', (e) => {
  e.respondWith(
    caches.match(e.request).then((r) => r || fetch(e.request))
  );
});
"#;
    fs::write("static/service-worker.js", sw).into_diagnostic()?;
    Ok(())
}
fn handle_install(package: &str) -> miette::Result<()> {
    // Heuristic: If it looks like a URL or git repo, treat as Module.
    if package.contains("/") || package.starts_with("git") || package.ends_with(".git") {
        install_module(package)
    } else {
        install_crate(package)
    }
}

fn install_crate(package: &str) -> miette::Result<()> {
    println!("📦 Installing Rust crate: {}", package);
    let status = std::process::Command::new("cargo")
        .args(["add", package])
        .status()
        .into_diagnostic()?;
    
    if status.success() {
        println!("✅ Installed crate: {}", package);
    } else {
        eprintln!("❌ Failed to install crate: {}", package);
    }
    Ok(())
}

fn install_module(url: &str) -> miette::Result<()> {
    // Extract name from URL (e.g. github.com/foo/bar -> bar)
    let name = url.split('/').next_back().unwrap().trim_end_matches(".git");
    let target_dir = Path::new("src/vendor").join(name);
    
    println!("📦 Installing Nucleus Module '{}' from {}", name, url);
    fs::create_dir_all("src/vendor").into_diagnostic()?;
    
    // Check if git is installed
    if std::process::Command::new("git").arg("--version").output().is_err() {
        return Err(miette::miette!("Git is not installed or not in PATH"));
    }

    if target_dir.exists() {
         println!("ℹ️  Module '{}' already exists. Updating...", name);
         let status = std::process::Command::new("git")
             .current_dir(&target_dir)
             .args(["pull"])
             .status()
             .into_diagnostic()?;
             
         if !status.success() {
             eprintln!("⚠️ Failed to update module");
         } else {
             println!("✅ Module updated");
         }
         return Ok(());
    }
    
    // git clone
    let status = std::process::Command::new("git")
        .args(["clone", "--depth", "1", url, target_dir.to_str().unwrap()])
        .status()
        .into_diagnostic()?;

    if status.success() {
        println!("✅ Installed module to src/vendor/{}", name);
        println!("💡 You can now use <n:{}> in your views!", name);
    } else {
         eprintln!("❌ Failed to clone module");
    }
    Ok(())
}

fn optimize_node(node: ncc::ast::Node, stem: &str, ts_files: &mut Vec<String>, wasm_fragments: &mut Vec<String>) -> ncc::ast::Node {
    match node {
        ncc::ast::Node::Client(content) => {
             wasm_fragments.push(content);
             // Remove from output (it shouldn't render in HTML)
             ncc::ast::Node::Text(String::new())
        },
        // Recursion for Element nodes
        ncc::ast::Node::Element(mut el) => {
             let tag_name = el.tag_name.clone();
             
             // 1. Extract Style
             if tag_name == "style" {
                 let is_critical = el.attributes.iter().any(|(k, _)| k == "critical");
                 if !is_critical {
                     if let Some(ncc::ast::Node::Text(css)) = el.children.first() {
                         use std::hash::{Hash, Hasher};
                         let mut hasher = std::collections::hash_map::DefaultHasher::new();
                         css.hash(&mut hasher);
                         let hash = hasher.finish();
                         let filename = format!("style-{:x}.css", hash);
                         
                         let _ = fs::create_dir_all("static/css"); 
                         let _ = fs::write(format!("static/css/{}", filename), css);

                         return ncc::ast::Node::Element(ncc::ast::Element {
                             tag_name: "link".to_string(),
                             attributes: vec![
                                 ("rel".to_string(), "stylesheet".to_string()),
                                 ("href".to_string(), format!("/static/css/{}", filename))
                             ],
                             children: vec![]
                         });
                     }
                 }
             }

             // 2. Extract Script
             if tag_name == "script" && !el.attributes.iter().any(|(k, _)| k == "src") {
                 let is_ts = el.attributes.iter().any(|(k, v)| k == "lang" && v == "ts");
                 
                 if is_ts {
                     if let Some(ncc::ast::Node::Text(ts_code)) = el.children.first() {
                             use std::hash::{Hash, Hasher};
                             let mut hasher = std::collections::hash_map::DefaultHasher::new();
                             ts_code.hash(&mut hasher);
                             let hash = hasher.finish();
                             let filename = format!("{}-{:x}.ts", stem, hash);
                             let out_path = format!("src/generated/ts/{}", filename);
                             
                             let _ = fs::create_dir_all("src/generated/ts");
                             let _ = fs::write(&out_path, ts_code);
                             
                             ts_files.push(out_path);
                             
                             let js_filename = filename.replace(".ts", ".js");
                             return ncc::ast::Node::Element(ncc::ast::Element {
                                 tag_name: "script".to_string(),
                                 attributes: vec![
                                     ("src".to_string(), format!("/static/js/{}", js_filename)),
                                     ("type".to_string(), "module".to_string()),
                                     ("defer".to_string(), "true".to_string())
                                 ],
                                 children: vec![]
                             });
                     }
                 } else if let Some(ncc::ast::Node::Text(js)) = el.children.first() {
                         use std::hash::{Hash, Hasher};
                         let mut hasher = std::collections::hash_map::DefaultHasher::new();
                         js.hash(&mut hasher);
                         let hash = hasher.finish();
                         let filename = format!("script-{:x}.js", hash);
                         
                         let _ = fs::create_dir_all("static/js"); 
                         let _ = fs::write(format!("static/js/{}", filename), js);

                         return ncc::ast::Node::Element(ncc::ast::Element {
                             tag_name: "script".to_string(),
                             attributes: vec![
                                 ("src".to_string(), format!("/static/js/{}", filename)),
                                 ("defer".to_string(), "true".to_string())
                             ],
                             children: vec![]
                         });
                 }
             }
             
             // Recursion
             el.children = el.children.into_iter().map(|c| optimize_node(c, stem, ts_files, wasm_fragments)).collect();
             ncc::ast::Node::Element(el)
        },
        ncc::ast::Node::If { condition, children } => {
             ncc::ast::Node::If { 
                 condition, 
                 children: children.into_iter().map(|c| optimize_node(c, stem, ts_files, wasm_fragments)).collect() 
             }
        },
        ncc::ast::Node::For { variable, iterable, children } => {
             ncc::ast::Node::For { 
                 variable, 
                 iterable, 
                 children: children.into_iter().map(|c| optimize_node(c, stem, ts_files, wasm_fragments)).collect() 
             }
        },
        _ => node
    }
}
fn merge_layouts(layout_nodes: Vec<ncc::ast::Node>, content_nodes: Vec<ncc::ast::Node>) -> Vec<ncc::ast::Node> {
    // 1. Extract content children (unwrap from <n:view> if present)
    let content_children = if let Some(ncc::ast::Node::Element(el)) = content_nodes.iter().find(|n| matches!(n, ncc::ast::Node::Element(e) if e.tag_name == "n:view")) {
        el.children.clone()
    } else {
        content_nodes.clone()
    };
    
    // 2. Separate Meta Code (Loader/Action) from UI Content
    let (meta_nodes, ui_nodes): (Vec<_>, Vec<_>) = content_children.into_iter().partition(|n| {
        matches!(n, ncc::ast::Node::Loader(_) | ncc::ast::Node::Action(_))
    });

    // 3. Recursively replace Outlet in layout with UI nodes
    let mut merged_nodes = replace_outlet_in_nodes(layout_nodes, &ui_nodes);
    
    // 4. Inject Meta Nodes into the Layout's Root View
    // Find the root n:view and append meta nodes to its children
    let mut injected_meta = false;
    for node in &mut merged_nodes {
        if let ncc::ast::Node::Element(el) = node {
            if el.tag_name == "n:view" {
                el.children.extend(meta_nodes.clone()); // Cloning simple strings is fine
                injected_meta = true;
            }
        }
    }
    
    // If layout didn't have n:view, just append meta nodes to the root list
    // This supports Raw Layouts (<!DOCTYPE html>...)
    if !injected_meta {
        merged_nodes.extend(meta_nodes);
    }
    
    merged_nodes
}

fn replace_outlet_in_nodes(nodes: Vec<ncc::ast::Node>, content: &[ncc::ast::Node]) -> Vec<ncc::ast::Node> {
    let mut new_nodes = Vec::new();
    for node in nodes {
        match node {
             ncc::ast::Node::Outlet | ncc::ast::Node::Slot { .. } => {
                 new_nodes.extend_from_slice(content);
             },
             ncc::ast::Node::Element(mut el) => {
                 el.children = replace_outlet_in_nodes(el.children, content);
                 new_nodes.push(ncc::ast::Node::Element(el));
             },
             ncc::ast::Node::For { variable, iterable, children } => {
                 new_nodes.push(ncc::ast::Node::For {
                     variable,
                     iterable,
                     children: replace_outlet_in_nodes(children, content)
                 });
             },
             ncc::ast::Node::If { condition, children } => {
                 new_nodes.push(ncc::ast::Node::If {
                     condition,
                     children: replace_outlet_in_nodes(children, content)
                 });
             },
             _ => new_nodes.push(node)
        }
    }
    new_nodes
}
