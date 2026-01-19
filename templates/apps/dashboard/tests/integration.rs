#[tokio::test]
async fn test_dashboard_startup() {
    // Basic "Smoke Test"
    // In lieu of full E2E, we verify the HTML file exists and has expected content.
    let content = tokio::fs::read_to_string("src/views/index.ncl")
        .await
        .expect("View missing");
    assert!(content.contains("<title>Nucleus Analytics</title>"));
    assert!(content.contains("Requests / Sec"));
}
