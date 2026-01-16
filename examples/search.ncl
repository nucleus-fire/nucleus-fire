<n:script>
    use nucleus_std::sonar::{Sonar, Document};

    // Example: Indexing and Searching
    async fn index_content(id: String, text: String) {
        let doc = Document {
            id,
            content: text,
        };
        // In real app, Sonar would be a global singleton or accessible via Context
        Sonar::index_document(doc); 
    }

    async fn search_api(query: String) -> Vec<String> {
        let results = Sonar::search(&query);
        // Returns [(id, score)], map to just IDs
        results.into_iter().map(|(id, _)| id).collect()
    }
</n:script>
