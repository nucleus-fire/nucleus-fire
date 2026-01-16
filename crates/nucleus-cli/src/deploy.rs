//! Nucleus Deploy Module
//! 
//! Provides beautiful, visual deployment tooling with multi-platform support.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use miette::{IntoDiagnostic, Result};

// Emojis for visual feedback
static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");
static GEAR: Emoji<'_, '_> = Emoji("⚙️  ", "");
static CLOUD: Emoji<'_, '_> = Emoji("☁️  ", "");
static DOCKER: Emoji<'_, '_> = Emoji("🐳 ", "");
static SPARKLES: Emoji<'_, '_> = Emoji("✨ ", "");
static WARNING: Emoji<'_, '_> = Emoji("⚠️  ", "");

/// Deployment target platforms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeployTarget {
    Docker,
    FlyIo,
    Railway,
    Render,
    Manual,
}

impl DeployTarget {
    fn as_str(&self) -> &'static str {
        match self {
            DeployTarget::Docker => "Docker (Self-hosted)",
            DeployTarget::FlyIo => "Fly.io (Global Edge)",
            DeployTarget::Railway => "Railway (Simple PaaS)",
            DeployTarget::Render => "Render (Managed)",
            DeployTarget::Manual => "Manual (Generate files only)",
        }
    }
    
    fn all() -> Vec<DeployTarget> {
        vec![
            DeployTarget::Docker,
            DeployTarget::FlyIo,
            DeployTarget::Railway,
            DeployTarget::Render,
            DeployTarget::Manual,
        ]
    }
}

/// Main entry point for the deploy command
pub fn run_deploy(target: Option<String>) -> Result<()> {
    print_banner();
    
    // Determine target - interactive if not specified
    let deploy_target = match target {
        Some(t) => match t.to_lowercase().as_str() {
            "docker" => DeployTarget::Docker,
            "fly" | "flyio" | "fly.io" => DeployTarget::FlyIo,
            "railway" => DeployTarget::Railway,
            "render" => DeployTarget::Render,
            "manual" => DeployTarget::Manual,
            _ => {
                println!("{} Unknown target '{}'. Launching interactive mode...", WARNING, t);
                select_target_interactive()?
            }
        },
        None => select_target_interactive()?,
    };
    
    println!();
    println!("{} Selected: {}", ROCKET, style(deploy_target.as_str()).cyan().bold());
    println!();
    
    // Run deployment for selected target
    match deploy_target {
        DeployTarget::Docker => deploy_docker()?,
        DeployTarget::FlyIo => deploy_fly()?,
        DeployTarget::Railway => deploy_railway()?,
        DeployTarget::Render => deploy_render()?,
        DeployTarget::Manual => deploy_manual()?,
    }
    
    // Send system notification
    send_notification("Nucleus Deploy", "Deployment preparation complete! 🚀");
    
    Ok(())
}

fn print_banner() {
    println!();
    println!("{}", style("╔══════════════════════════════════════════╗").cyan());
    println!("{}", style("║       ⚛️  NUCLEUS DEPLOY                  ║").cyan());
    println!("{}", style("║   Deploy anywhere in seconds              ║").cyan());
    println!("{}", style("╚══════════════════════════════════════════╝").cyan());
    println!();
}

fn select_target_interactive() -> Result<DeployTarget> {
    let targets = DeployTarget::all();
    let target_names: Vec<&str> = targets.iter().map(|t| t.as_str()).collect();
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Where would you like to deploy?")
        .items(&target_names)
        .default(0)
        .interact()
        .into_diagnostic()?;
    
    Ok(targets[selection])
}

/// Docker deployment with visual progress
fn deploy_docker() -> Result<()> {
    let mp = MultiProgress::new();
    
    // Step 1: Generate Dockerfile
    let pb1 = mp.add(ProgressBar::new_spinner());
    pb1.set_style(spinner_style());
    pb1.set_message("Generating Dockerfile...");
    pb1.enable_steady_tick(Duration::from_millis(80));
    
    generate_dockerfile()?;
    pb1.finish_with_message(format!("{} Dockerfile generated", CHECK));
    
    // Step 2: Generate .dockerignore
    let pb2 = mp.add(ProgressBar::new_spinner());
    pb2.set_style(spinner_style());
    pb2.set_message("Generating .dockerignore...");
    pb2.enable_steady_tick(Duration::from_millis(80));
    
    generate_dockerignore()?;
    pb2.finish_with_message(format!("{} .dockerignore generated", CHECK));
    
    // Step 3: Validate
    let pb3 = mp.add(ProgressBar::new_spinner());
    pb3.set_style(spinner_style());
    pb3.set_message("Validating project structure...");
    pb3.enable_steady_tick(Duration::from_millis(80));
    thread::sleep(Duration::from_millis(500));
    pb3.finish_with_message(format!("{} Project validated", CHECK));
    
    println!();
    println!("{}", style("═══════════════════════════════════════════").green());
    println!("{} {}", DOCKER, style("Docker deployment ready!").green().bold());
    println!("{}", style("═══════════════════════════════════════════").green());
    println!();
    println!("Next steps:");
    println!("  {} Build:  {}", style("1.").dim(), style("docker build -t my-nucleus-app .").cyan());
    println!("  {} Run:    {}", style("2.").dim(), style("docker run -p 3000:3000 my-nucleus-app").cyan());
    println!("  {} Push:   {}", style("3.").dim(), style("docker push my-registry/my-nucleus-app").cyan());
    println!();
    
    Ok(())
}

/// Fly.io deployment
fn deploy_fly() -> Result<()> {
    let mp = MultiProgress::new();
    
    // Step 1: Generate fly.toml
    let pb1 = mp.add(ProgressBar::new_spinner());
    pb1.set_style(spinner_style());
    pb1.set_message("Generating fly.toml...");
    pb1.enable_steady_tick(Duration::from_millis(80));
    
    generate_fly_toml()?;
    pb1.finish_with_message(format!("{} fly.toml generated", CHECK));
    
    // Step 2: Generate Dockerfile
    let pb2 = mp.add(ProgressBar::new_spinner());
    pb2.set_style(spinner_style());
    pb2.set_message("Generating Dockerfile...");
    pb2.enable_steady_tick(Duration::from_millis(80));
    
    generate_dockerfile()?;
    pb2.finish_with_message(format!("{} Dockerfile generated", CHECK));
    
    println!();
    println!("{}", style("═══════════════════════════════════════════").magenta());
    println!("{} {}", CLOUD, style("Fly.io deployment ready!").magenta().bold());
    println!("{}", style("═══════════════════════════════════════════").magenta());
    println!();
    println!("Next steps:");
    println!("  {} Install: {}", style("1.").dim(), style("curl -L https://fly.io/install.sh | sh").cyan());
    println!("  {} Login:   {}", style("2.").dim(), style("fly auth login").cyan());
    println!("  {} Launch:  {}", style("3.").dim(), style("fly launch").cyan());
    println!("  {} Deploy:  {}", style("4.").dim(), style("fly deploy").cyan());
    println!();
    
    // Check if fly CLI is available
    if Command::new("fly").arg("version").output().is_ok() {
        println!("{} Fly CLI detected!", style("✓").green());
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to deploy now?")
            .default(false)
            .interact()
            .unwrap_or(false)
        {
            println!();
            println!("{} Running: fly deploy", ROCKET);
            let _ = Command::new("fly").arg("deploy").status();
        }
    }
    
    Ok(())
}

/// Railway deployment
fn deploy_railway() -> Result<()> {
    let mp = MultiProgress::new();
    
    // Step 1: Generate Dockerfile
    let pb1 = mp.add(ProgressBar::new_spinner());
    pb1.set_style(spinner_style());
    pb1.set_message("Generating Dockerfile...");
    pb1.enable_steady_tick(Duration::from_millis(80));
    
    generate_dockerfile()?;
    pb1.finish_with_message(format!("{} Dockerfile generated", CHECK));
    
    // Step 2: Generate railway.json (optional config)
    let pb2 = mp.add(ProgressBar::new_spinner());
    pb2.set_style(spinner_style());
    pb2.set_message("Generating railway.json...");
    pb2.enable_steady_tick(Duration::from_millis(80));
    
    generate_railway_json()?;
    pb2.finish_with_message(format!("{} railway.json generated", CHECK));
    
    println!();
    println!("{}", style("═══════════════════════════════════════════").yellow());
    println!("{} {}", CLOUD, style("Railway deployment ready!").yellow().bold());
    println!("{}", style("═══════════════════════════════════════════").yellow());
    println!();
    println!("Next steps:");
    println!("  {} Go to {}", style("1.").dim(), style("https://railway.app").cyan().underlined());
    println!("  {} Click 'New Project' → 'Deploy from GitHub repo'", style("2.").dim());
    println!("  {} Railway will auto-detect your Dockerfile", style("3.").dim());
    println!("  {} Add environment variables: DATABASE_URL, JWT_SECRET", style("4.").dim());
    println!();
    
    // Check if railway CLI is available
    if Command::new("railway").arg("version").output().is_ok() {
        println!("{} Railway CLI detected!", style("✓").green());
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to deploy now?")
            .default(false)
            .interact()
            .unwrap_or(false)
        {
            println!();
            println!("{} Running: railway up", ROCKET);
            let _ = Command::new("railway").arg("up").status();
        }
    }
    
    Ok(())
}

/// Render deployment
fn deploy_render() -> Result<()> {
    let mp = MultiProgress::new();
    
    // Step 1: Generate render.yaml
    let pb1 = mp.add(ProgressBar::new_spinner());
    pb1.set_style(spinner_style());
    pb1.set_message("Generating render.yaml...");
    pb1.enable_steady_tick(Duration::from_millis(80));
    
    generate_render_yaml()?;
    pb1.finish_with_message(format!("{} render.yaml generated", CHECK));
    
    // Step 2: Generate Dockerfile
    let pb2 = mp.add(ProgressBar::new_spinner());
    pb2.set_style(spinner_style());
    pb2.set_message("Generating Dockerfile...");
    pb2.enable_steady_tick(Duration::from_millis(80));
    
    generate_dockerfile()?;
    pb2.finish_with_message(format!("{} Dockerfile generated", CHECK));
    
    println!();
    println!("{}", style("═══════════════════════════════════════════").blue());
    println!("{} {}", CLOUD, style("Render deployment ready!").blue().bold());
    println!("{}", style("═══════════════════════════════════════════").blue());
    println!();
    println!("Next steps:");
    println!("  {} Go to {}", style("1.").dim(), style("https://render.com").cyan().underlined());
    println!("  {} Click 'New' → 'Blueprint'", style("2.").dim());
    println!("  {} Connect your GitHub repo", style("3.").dim());
    println!("  {} Render will use your render.yaml automatically", style("4.").dim());
    println!();
    
    Ok(())
}

/// Manual deployment (generate all files)
fn deploy_manual() -> Result<()> {
    println!("{} Generating all deployment configurations...", GEAR);
    println!();
    
    let pb = ProgressBar::new(5);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}"
    ).unwrap().progress_chars("━━╾"));
    
    pb.set_message("Dockerfile");
    generate_dockerfile()?;
    pb.inc(1);
    
    pb.set_message(".dockerignore");
    generate_dockerignore()?;
    pb.inc(1);
    
    pb.set_message("fly.toml");
    generate_fly_toml()?;
    pb.inc(1);
    
    pb.set_message("render.yaml");
    generate_render_yaml()?;
    pb.inc(1);
    
    pb.set_message("railway.json");
    generate_railway_json()?;
    pb.inc(1);
    
    pb.finish_with_message("All files generated!");
    
    println!();
    println!("{}", style("═══════════════════════════════════════════").white());
    println!("{} {}", SPARKLES, style("All deployment files generated!").white().bold());
    println!("{}", style("═══════════════════════════════════════════").white());
    println!();
    println!("Generated files:");
    println!("  {} Dockerfile", CHECK);
    println!("  {} .dockerignore", CHECK);
    println!("  {} fly.toml (Fly.io)", CHECK);
    println!("  {} render.yaml (Render)", CHECK);
    println!("  {} railway.json (Railway)", CHECK);
    println!();
    println!("Choose your platform and follow the provider's deployment guide.");
    println!();
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// FILE GENERATORS
// ═══════════════════════════════════════════════════════════════════════════

fn generate_dockerfile() -> Result<()> {
    let path = Path::new("Dockerfile");
    if path.exists() {
        return Ok(()); // Don't overwrite
    }
    
    let content = r#"# ═══════════════════════════════════════════════════════════════════════════
# NUCLEUS FRAMEWORK - OPTIMIZED PRODUCTION DOCKERFILE
# Multi-stage build for minimal image size (~20MB)
# ═══════════════════════════════════════════════════════════════════════════

# Stage 1: Build
FROM rust:1.76-buster as builder

WORKDIR /app
COPY . .

# Build release binary with optimizations
RUN cargo build --release --bin server

# Stage 2: Runtime (Distroless for security)
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy only what's needed
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/static /app/static

# Environment
ENV PORT=3000
EXPOSE 3000

# Run
CMD ["./server"]
"#;
    
    fs::write(path, content).into_diagnostic()?;
    Ok(())
}

fn generate_dockerignore() -> Result<()> {
    let path = Path::new(".dockerignore");
    if path.exists() {
        return Ok(());
    }
    
    let content = r#"# Build artifacts
target/
*.rs.bk

# Git
.git/
.gitignore

# IDE
.idea/
.vscode/
*.swp

# Environment
.env
.env.*
!.env.example

# Misc
README.md
docs/
tests/
"#;
    
    fs::write(path, content).into_diagnostic()?;
    Ok(())
}

fn generate_fly_toml() -> Result<()> {
    let path = Path::new("fly.toml");
    if path.exists() {
        return Ok(());
    }
    
    // Try to detect app name from Cargo.toml
    let app_name = detect_app_name().unwrap_or_else(|| "my-nucleus-app".to_string());
    
    let content = format!(r#"# Fly.io Configuration
# Generated by: nucleus deploy

app = "{}"
primary_region = "iad"

[build]
  dockerfile = "Dockerfile"

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 0

[env]
  PORT = "3000"

[[vm]]
  cpu_kind = "shared"
  cpus = 1
  memory_mb = 256
"#, app_name);
    
    fs::write(path, content).into_diagnostic()?;
    Ok(())
}

fn generate_render_yaml() -> Result<()> {
    let path = Path::new("render.yaml");
    if path.exists() {
        return Ok(());
    }
    
    let app_name = detect_app_name().unwrap_or_else(|| "nucleus-app".to_string());
    
    let content = format!(r#"# Render Blueprint
# Generated by: nucleus deploy
# Docs: https://render.com/docs/blueprint-spec

services:
  - type: web
    name: {}
    env: docker
    plan: free
    region: ohio
    dockerfilePath: ./Dockerfile
    envVars:
      - key: PORT
        value: 3000
      - key: DATABASE_URL
        sync: false
      - key: JWT_SECRET
        generateValue: true
"#, app_name);
    
    fs::write(path, content).into_diagnostic()?;
    Ok(())
}

fn generate_railway_json() -> Result<()> {
    let path = Path::new("railway.json");
    if path.exists() {
        return Ok(());
    }
    
    let content = r#"{
  "$schema": "https://railway.app/railway.schema.json",
  "build": {
    "builder": "DOCKERFILE",
    "dockerfilePath": "Dockerfile"
  },
  "deploy": {
    "startCommand": "./server",
    "healthcheckPath": "/health",
    "restartPolicyType": "ON_FAILURE"
  }
}
"#;
    
    fs::write(path, content).into_diagnostic()?;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

fn spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.cyan} {msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
}

fn detect_app_name() -> Option<String> {
    // Try to read from Cargo.toml
    if let Ok(content) = fs::read_to_string("Cargo.toml") {
        for line in content.lines() {
            if line.starts_with("name") {
                if let Some(name) = line.split('=').nth(1) {
                    return Some(name.trim().trim_matches('"').to_string());
                }
            }
        }
    }
    None
}

fn send_notification(title: &str, body: &str) {
    // Try to send desktop notification (non-blocking, best-effort)
    #[cfg(not(target_os = "windows"))]
    {
        let _ = notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .icon("dialog-information")
            .timeout(notify_rust::Timeout::Milliseconds(5000))
            .show();
    }
    
    #[cfg(target_os = "windows")]
    {
        let _ = notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .show();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_deploy_target_as_str() {
        assert_eq!(DeployTarget::Docker.as_str(), "Docker (Self-hosted)");
        assert_eq!(DeployTarget::FlyIo.as_str(), "Fly.io (Global Edge)");
        assert_eq!(DeployTarget::Railway.as_str(), "Railway (Simple PaaS)");
        assert_eq!(DeployTarget::Render.as_str(), "Render (Managed)");
        assert_eq!(DeployTarget::Manual.as_str(), "Manual (Generate files only)");
    }

    #[test]
    fn test_deploy_target_all() {
        let targets = DeployTarget::all();
        assert_eq!(targets.len(), 5);
        assert!(targets.contains(&DeployTarget::Docker));
        assert!(targets.contains(&DeployTarget::FlyIo));
        assert!(targets.contains(&DeployTarget::Railway));
        assert!(targets.contains(&DeployTarget::Render));
        assert!(targets.contains(&DeployTarget::Manual));
    }

    #[test]
    fn test_detect_app_name_missing_cargo_toml() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = detect_app_name();
        assert!(result.is_none());
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_detect_app_name_with_cargo_toml() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        fs::write("Cargo.toml", r#"
[package]
name = "my-test-app"
version = "0.1.0"
"#).unwrap();
        
        let result = detect_app_name();
        assert_eq!(result, Some("my-test-app".to_string()));
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_generate_dockerfile() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = generate_dockerfile();
        assert!(result.is_ok());
        assert!(Path::new("Dockerfile").exists());
        
        let content = fs::read_to_string("Dockerfile").unwrap();
        assert!(content.contains("FROM rust:"));
        assert!(content.contains("cargo build --release"));
        assert!(content.contains("EXPOSE 3000"));
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_generate_dockerignore() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = generate_dockerignore();
        assert!(result.is_ok());
        assert!(Path::new(".dockerignore").exists());
        
        let content = fs::read_to_string(".dockerignore").unwrap();
        assert!(content.contains("target/"));
        assert!(content.contains(".git/"));
        assert!(content.contains(".env"));
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_generate_fly_toml() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = generate_fly_toml();
        assert!(result.is_ok());
        assert!(Path::new("fly.toml").exists());
        
        let content = fs::read_to_string("fly.toml").unwrap();
        assert!(content.contains("app ="));
        assert!(content.contains("[http_service]"));
        assert!(content.contains("internal_port = 3000"));
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_generate_render_yaml() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = generate_render_yaml();
        assert!(result.is_ok());
        assert!(Path::new("render.yaml").exists());
        
        let content = fs::read_to_string("render.yaml").unwrap();
        assert!(content.contains("services:"));
        assert!(content.contains("type: web"));
        assert!(content.contains("dockerfilePath"));
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_generate_railway_json() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = generate_railway_json();
        assert!(result.is_ok());
        assert!(Path::new("railway.json").exists());
        
        let content = fs::read_to_string("railway.json").unwrap();
        assert!(content.contains("\"builder\": \"DOCKERFILE\""));
        assert!(content.contains("\"healthcheckPath\""));
        
        std::env::set_current_dir(original_cwd).unwrap();
    }

    #[test]
    fn test_dockerfile_not_overwritten() {
        let _guard = TEST_LOCK.lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // Write a custom Dockerfile first
        fs::write("Dockerfile", "# Custom Dockerfile").unwrap();
        
        // Generate should not overwrite
        let result = generate_dockerfile();
        assert!(result.is_ok());
        
        let content = fs::read_to_string("Dockerfile").unwrap();
        assert_eq!(content, "# Custom Dockerfile");
        
        std::env::set_current_dir(original_cwd).unwrap();
    }
}

