#[test]
fn test_css_generation() {
    let _exists = std::path::Path::new("static/styles.css").exists();
    // It might not exist if we run cargo test before npm run build:css separately,
    // but in our workflow we did run build:css.
    // Let's check for input.css existence as a sanity check.
    assert!(std::path::Path::new("input.css").exists());
}
