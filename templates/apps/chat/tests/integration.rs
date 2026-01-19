#[tokio::test]
async fn test_chat_startup() {
    // Verify View Content
    let content = tokio::fs::read_to_string("src/views/index.ncl")
        .await
        .expect("View missing");
    assert!(content.contains("Nucleus Chat"));
    assert!(content.contains("Messages"));

    // Verify Logic (if any)
    // Chat is currently client-side JS + static serving.
}
