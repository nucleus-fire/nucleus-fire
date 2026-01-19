use nucleus_cli::create_project;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_create_project_structure() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "test_app";
    let project_path = temp_dir.path().join(project_name);

    // Run create_project (simulating 'nucleus new')
    // Note: We need to change directory or just pass absolute path?
    // create_project takes a name and creates it in current dir.
    // We need to change cwd or modify create_project to accept path.
    // Looking at lib.rs: `let path = Path::new(name);` and `fs::create_dir(path)?`
    // So if we pass a full path, it might work?
    // `Path::new("/tmp/foo")` works.

    let result = create_project(project_path.to_str().unwrap());
    assert!(
        result.is_ok(),
        "Project creation failed: {:?}",
        result.err()
    );

    // 1. Verify Directory Structure
    assert!(project_path.exists());
    assert!(project_path.join("src/views").exists());
    assert!(
        project_path.join("src/assets").exists(),
        "src/assets should exist"
    );
    assert!(project_path.join("static").exists());

    // 2. Verify Config Best Practices
    let config_path = project_path.join("nucleus.config");
    assert!(config_path.exists());
    let config_content = fs::read_to_string(&config_path).unwrap();

    // Check for cache settings
    assert!(
        config_content.contains("css_max_age"),
        "Config should contain css_max_age"
    );
    assert!(
        config_content.contains("image_max_age"),
        "Config should contain image_max_age"
    );
    assert!(
        config_content.contains("[performance.fonts]"),
        "Config should contain font settings"
    );

    // 3. Verify Layout Optimization
    let layout_path = project_path.join("src/views/layout.ncl");
    assert!(layout_path.exists());
    let layout_content = fs::read_to_string(&layout_path).unwrap();
    // Check for font display swap or other optimizations if we added them
    // (We did add preconnect hints in ncc but layout.ncl has simple structure)
    // Checking standard structure
    assert!(layout_content.contains("<!DOCTYPE html>"));
}
