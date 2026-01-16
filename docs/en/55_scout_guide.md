# Full-Text Search (Scout) Guide

Scout is Nucleus's full-text search module with multi-backend support for Meilisearch and Typesense.

## Quick Start

```rust
use nucleus_std::scout::{Scout, SearchQuery};

// Initialize with Meilisearch
let scout = Scout::meilisearch("http://localhost:7700", Some("your_api_key"));

// Index documents
let products = vec![
    json!({"id": "1", "name": "Gaming Laptop", "price": 999}),
    json!({"id": "2", "name": "Wireless Mouse", "price": 49}),
];
scout.index("products", &products).await?;

// Search with query builder
let results = scout.search("products")
    .query("gaming")
    .filter("price < 1000")
    .sort("price:asc")
    .limit(20)
    .execute()
    .await?;

for hit in results.hits {
    println!("Found: {} (score: {:?})", hit.id, hit.score);
}
```

## Backends

### Meilisearch

```rust
// Without authentication
let scout = Scout::meilisearch("http://localhost:7700", None);

// With API key
let scout = Scout::meilisearch("http://localhost:7700", Some("master_key"));
```

### Typesense

```rust
let scout = Scout::typesense("http://localhost:8108", "your_api_key");
```

## Indexing Documents

### Basic Indexing

```rust
#[derive(Serialize)]
struct Product {
    id: String,
    name: String,
    price: f64,
    category: String,
}

let products = vec![
    Product { id: "1".into(), name: "Laptop".into(), price: 999.0, category: "electronics".into() },
];

scout.index("products", &products).await?;
```

### With Primary Key

```rust
scout.index_with_key("products", &products, "product_id").await?;
```

## Search Query Builder

### Basic Query

```rust
let results = scout.search("products")
    .query("laptop")
    .execute()
    .await?;
```

### With Filters

```rust
let results = scout.search("products")
    .query("laptop")
    .filter("price < 1000 AND category = 'electronics'")
    .execute()
    .await?;
```

### Sorting

```rust
let results = scout.search("products")
    .query("laptop")
    .sort("price:asc")
    .sort("created_at:desc")  // Multiple sorts
    .execute()
    .await?;
```

### Pagination

```rust
let results = scout.search("products")
    .query("laptop")
    .limit(20)   // Results per page
    .offset(40)  // Skip first 40 results (page 3)
    .execute()
    .await?;
```

### Faceted Search

```rust
let results = scout.search("products")
    .query("laptop")
    .facets(&["category", "brand"])
    .execute()
    .await?;

// Access facet counts
if let Some(facets) = results.facet_distribution {
    for (field, values) in facets {
        println!("{}: {:?}", field, values);
    }
}
```

### Highlighted Results

```rust
let results = scout.search("products")
    .query("laptop")
    .highlight(&["name", "description"])
    .execute()
    .await?;
```

## Index Management

### Create Index

```rust
scout.create_index("products").await?;
scout.create_index_with_key("products", "id").await?;
```

### Delete Index

```rust
scout.delete_index("products").await?;
```

### List Indexes

```rust
let indexes = scout.list_indexes().await?;
for index in indexes {
    println!("{}: primary_key={:?}", index.name, index.primary_key);
}
```

### Clear Documents

```rust
scout.clear("products").await?;
```

## Document Operations

### Get by ID

```rust
let product: Option<Product> = scout.get("products", "1").await?;
```

### Delete Documents

```rust
scout.delete("products", &["1".into(), "2".into()]).await?;
```

## Async Task Management

```rust
// Index returns a task
let task = scout.index("products", &products).await?;

// Wait for task to complete
let status = scout.wait_for_task(&task, 30000).await?;
match status {
    TaskStatus::Succeeded => println!("Indexing complete!"),
    TaskStatus::Failed => println!("Indexing failed"),
    _ => {}
}
```

## Error Handling

```rust
use nucleus_std::scout::ScoutError;

match scout.search("products").query("test").execute().await {
    Ok(results) => handle_results(results),
    Err(ScoutError::IndexNotFound(name)) => create_index(&name),
    Err(ScoutError::RateLimited) => retry_later(),
    Err(ScoutError::Timeout) => increase_timeout(),
    Err(e) => log_error(e),
}
```

## Complete Example

```rust
use nucleus_std::scout::{Scout, ScoutError, TaskStatus};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Article {
    id: String,
    title: String,
    content: String,
    author: String,
    published_at: String,
}

async fn search_articles() -> Result<(), ScoutError> {
    let scout = Scout::meilisearch("http://localhost:7700", Some("key"));
    
    // Ensure index exists
    let _ = scout.create_index_with_key("articles", "id").await;
    
    // Index articles
    let articles = load_articles();
    let task = scout.index("articles", &articles).await?;
    scout.wait_for_task(&task, 10000).await?;
    
    // Search
    let results = scout.search("articles")
        .query("rust programming")
        .filter("published_at > '2024-01-01'")
        .sort("published_at:desc")
        .limit(10)
        .execute()
        .await?;
    
    println!("Found {} articles", results.total_hits);
    for hit in results.hits {
        println!("- {}", hit.document["title"]);
    }
    
    Ok(())
}
```
