# Full-Text Search Guide

Nucleus Sonar provides embedded full-text search using the BM25 algorithm.

## Quick Start

```rust
use nucleus_std::sonar::Sonar;

// Create search index
let mut sonar = Sonar::new();

// Index documents
sonar.index("doc1", "Rust is a systems programming language");
sonar.index("doc2", "Python is great for machine learning");
sonar.index("doc3", "Rust has zero-cost abstractions");

// Search
let results = sonar.search("rust programming");
for result in results {
    println!("{}: {:.2}", result.id, result.score);
}
```

## Indexing Documents

### Basic Indexing

```rust
let mut sonar = Sonar::new();

// Index with ID and content
sonar.index("article:1", "Introduction to Rust programming");
sonar.index("article:2", "Advanced Rust patterns");
sonar.index("article:3", "Web development with Rust");
```

### Bulk Indexing

```rust
let documents = vec![
    ("1", "First document content"),
    ("2", "Second document content"),
    ("3", "Third document content"),
];

for (id, content) in documents {
    sonar.index(id, content);
}
```

### Indexing from Database

```rust
// Index all articles from database
let articles = db.query("SELECT id, title, content FROM articles").await?;

for article in articles {
    let text = format!("{} {}", article.title, article.content);
    sonar.index(&article.id.to_string(), &text);
}
```

## Searching

### Basic Search

```rust
let results = sonar.search("rust programming");

for result in results {
    println!("ID: {}, Score: {:.4}", result.id, result.score);
}
```

### Search with Limit

```rust
let results = sonar.search_with_limit("web development", 10);
```

### Phrase Search

```rust
// Search for exact phrase
let results = sonar.search("\"zero-cost abstractions\"");
```

### Boolean Search

```rust
// Must contain both terms
let results = sonar.search("rust AND programming");

// Must contain either term
let results = sonar.search("rust OR python");

// Must not contain term
let results = sonar.search("rust NOT python");
```

## Search Results

```rust
pub struct SearchResult {
    pub id: String,      // Document ID
    pub score: f64,      // BM25 relevance score
}

// Results are sorted by score (highest first)
let results = sonar.search("query");

if let Some(top_result) = results.first() {
    println!("Best match: {} (score: {})", top_result.id, top_result.score);
}
```

## Updating Documents

```rust
// Re-index updates the document
sonar.index("doc1", "Updated content for document one");
```

## Removing Documents

```rust
sonar.remove("doc1");
```

## Practical Examples

### Product Search

```rust
struct ProductSearch {
    sonar: Sonar,
}

impl ProductSearch {
    fn new() -> Self {
        Self { sonar: Sonar::new() }
    }
    
    fn index_product(&mut self, product: &Product) {
        let text = format!(
            "{} {} {} {}",
            product.name,
            product.description,
            product.category,
            product.brand
        );
        self.sonar.index(&product.id.to_string(), &text);
    }
    
    fn search(&self, query: &str) -> Vec<i32> {
        self.sonar.search(query)
            .iter()
            .filter_map(|r| r.id.parse().ok())
            .collect()
    }
}

// Usage
let mut search = ProductSearch::new();

for product in products {
    search.index_product(&product);
}

let matching_ids = search.search("wireless headphones");
let products = db.get_products_by_ids(&matching_ids).await?;
```

### Article Search API

```rust
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    limit: Option<usize>,
}

async fn search_articles(
    Query(params): Query<SearchParams>,
    Extension(sonar): Extension<Arc<Sonar>>,
) -> Json<Vec<Article>> {
    let limit = params.limit.unwrap_or(20);
    let results = sonar.search_with_limit(&params.q, limit);
    
    let ids: Vec<i32> = results
        .iter()
        .filter_map(|r| r.id.parse().ok())
        .collect();
    
    let articles = db.get_articles_by_ids(&ids).await.unwrap();
    Json(articles)
}
```

### Search with Highlighting

```rust
fn highlight_matches(text: &str, query: &str) -> String {
    let terms: Vec<&str> = query.split_whitespace().collect();
    let mut result = text.to_string();
    
    for term in terms {
        let pattern = format!(r"(?i)\b{}\b", regex::escape(term));
        let re = regex::Regex::new(&pattern).unwrap();
        result = re.replace_all(&result, "<mark>$0</mark>").to_string();
    }
    
    result
}

// Usage
let results = sonar.search("rust programming");
for result in results {
    let article = db.get_article(&result.id).await?;
    let highlighted = highlight_matches(&article.content, "rust programming");
    println!("{}", highlighted);
}
```

### Autocomplete/Suggestions

```rust
struct Autocomplete {
    sonar: Sonar,
}

impl Autocomplete {
    fn new() -> Self {
        Self { sonar: Sonar::new() }
    }
    
    fn add_term(&mut self, term: &str) {
        // Index all prefixes for prefix matching
        for i in 1..=term.len() {
            let prefix = &term[..i];
            self.sonar.index(&format!("{}:{}", prefix, term), term);
        }
    }
    
    fn suggest(&self, prefix: &str, limit: usize) -> Vec<String> {
        self.sonar.search_with_limit(prefix, limit)
            .iter()
            .filter_map(|r| r.id.split(':').last().map(String::from))
            .collect()
    }
}

// Usage
let mut ac = Autocomplete::new();
ac.add_term("rust");
ac.add_term("ruby");
ac.add_term("python");

let suggestions = ac.suggest("ru", 5);
// Returns: ["rust", "ruby"]
```

## Index Persistence

```rust
// Save index to disk
sonar.save("search_index.bin")?;

// Load index from disk
let sonar = Sonar::load("search_index.bin")?;
```

## Performance Tips

1. **Batch indexing** - Index many documents, then search
2. **Use IDs wisely** - Keep IDs short for memory efficiency
3. **Limit results** - Use `search_with_limit` for pagination
4. **Background updates** - Re-index in background tasks
5. **Persist index** - Save/load to avoid re-indexing on restart

## Best Practices

1. **Combine fields** - Index title + content + tags together
2. **Normalize text** - Lowercase, remove punctuation before indexing
3. **Update on change** - Re-index when documents change
4. **Handle empty results** - Show suggestions or "no results found"
5. **Consider relevance** - BM25 balances term frequency and document length
