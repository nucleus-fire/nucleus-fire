use std::fs;
use std::path::Path;
use nucleus_cli::{generate_sitemap, generate_pwa};

use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_sitemap_generation() {
    let _guard = LOCK.lock().unwrap();
    // Setup
    let test_dir = "test_sitemap_out";
    let static_dir = format!("{}/static", test_dir);
    fs::create_dir_all(&static_dir).unwrap();
    
    // Switch CWD (Critical for CLI relative path logic)
    // Note: This is hacky for tests, normally we'd pass paths to functions.
    // But since we are testing the actual CLI logic which assumes CWD...
    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(test_dir).unwrap();

    // Execute
    let routes = vec!["/".to_string(), "/about".to_string()];
    let result = generate_sitemap(&routes);

    // Verify
    assert!(result.is_ok());
    let sitemap_path = Path::new("static/sitemap.xml");
    assert!(sitemap_path.exists());
    let content = fs::read_to_string(sitemap_path).unwrap();
    assert!(content.contains("<loc>http://localhost:3000/</loc>"));
    assert!(content.contains("<loc>http://localhost:3000/about</loc>"));

    // Cleanup
    std::env::set_current_dir(original_cwd).unwrap();
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_pwa_generation() {
    let _guard = LOCK.lock().unwrap();
    // Setup
    let test_dir = "test_pwa_out";
    let static_dir = format!("{}/static", test_dir);
    fs::create_dir_all(&static_dir).unwrap();
    
    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(test_dir).unwrap();

    // Execute
    let result = generate_pwa();

    // Verify
    assert!(result.is_ok());
    
    let manifest_path = Path::new("static/manifest.json");
    assert!(manifest_path.exists());
    let manifest = fs::read_to_string(manifest_path).unwrap();
    assert!(manifest.contains("\"name\": \"Nucleus App\""));

    let sw_path = Path::new("static/service-worker.js");
    assert!(sw_path.exists());
    let sw = fs::read_to_string(sw_path).unwrap();
    assert!(sw.contains("CACHE_NAME"));

    // Cleanup
    std::env::set_current_dir(original_cwd).unwrap();
    fs::remove_dir_all(test_dir).unwrap();
}

// FUTURE: Add test_image_optimization when synthetic image generation is available
