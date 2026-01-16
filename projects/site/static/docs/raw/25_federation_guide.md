# Content Federation Guide

Complete guide to the Nucleus Content Federation module for aggregating content from multiple headless CMS platforms.

---

## Overview

The Federation module provides a unified interface for querying content from:

| Platform | Protocol | Query Language |
|----------|----------|----------------|
| **Directus** | REST/GraphQL | SQL-like filters |
| **Sanity** | HTTP | GROQ |
| **Strapi** | REST/GraphQL | Strapi v4 filters |

---

## Configuration

### nucleus.config

```toml
[[federation.sources]]
name = "directus"
type = "directus"
url = "${DIRECTUS_URL}"
token = "${DIRECTUS_TOKEN}"

[[federation.sources]]
name = "sanity"
type = "sanity"
project_id = "${SANITY_PROJECT_ID}"
dataset = "production"
token = "${SANITY_TOKEN}"

[[federation.sources]]
name = "strapi"
type = "strapi"
url = "${STRAPI_URL}"
token = "${STRAPI_TOKEN}"
```

### Environment Variables

```bash
# Directus
export DIRECTUS_URL="https://cms.example.com"
export DIRECTUS_TOKEN="your-token"

# Sanity
export SANITY_PROJECT_ID="your-project-id"
export SANITY_DATASET="production"
export SANITY_TOKEN="your-token"

# Strapi
export STRAPI_URL="https://strapi.example.com"
export STRAPI_TOKEN="your-token"
```

---

## API Reference

### Federation

Main entry point for querying content.

```rust
use nucleus_std::federation::*;
```

#### Federation::query

Start a query for a collection:

```rust
pub fn query(collection: &str) -> FederationQuery;
```

Example:
```rust
let articles = Federation::query("articles")
    .filter("status", "published")
    .order_by("created_at", Direction::Desc)
    .limit(10)
    .fetch()
    .await?;
```

#### Federation::source

Query a specific source:

```rust
pub fn source(name: &str) -> SourceQuery;
```

Example:
```rust
let items = Federation::source("directus")
    .collection("blog_posts")
    .fetch()
    .await?;
```

### FederationQuery

Query builder for federated content.

```rust
impl FederationQuery {
    /// Filter by a specific source
    pub fn from_source(self, source: &str) -> Self;
    
    /// Add a filter condition
    pub fn filter(self, field: &str, value: impl Into<Value>) -> Self;
    
    /// Order results
    pub fn order_by(self, field: &str, direction: Direction) -> Self;
    
    /// Limit results
    pub fn limit(self, n: usize) -> Self;
    
    /// Offset for pagination
    pub fn offset(self, n: usize) -> Self;
    
    /// Execute the query
    pub async fn fetch(self) -> Result<Vec<ContentItem>>;
}
```

### ContentItem

Unified content item from any CMS.

```rust
pub struct ContentItem {
    pub id: String,           // Unique ID from source
    pub source: String,       // Source name (e.g., "directus")
    pub collection: String,   // Collection/content type
    pub data: Value,          // Raw content data
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
```

### Direction

Sort direction for queries.

```rust
pub enum Direction {
    Asc,
    Desc,
}
```

---

## Adapter APIs

### DirectusAdapter

```rust
use nucleus_std::federation::DirectusAdapter;

// Query with builder
let items = DirectusAdapter::query(&FederationQuery::new("articles")).await?;

// Fetch collection directly
let posts = DirectusAdapter::fetch_collection(Some("posts")).await?;
```

### SanityAdapter

```rust
use nucleus_std::federation::SanityAdapter;

// Query with builder (auto-converts to GROQ)
let items = SanityAdapter::query(&FederationQuery::new("article")).await?;

// Execute raw GROQ
let items = SanityAdapter::fetch_groq(Some("*[_type == 'post'][0...5]")).await?;
```

### StrapiAdapter

```rust
use nucleus_std::federation::StrapiAdapter;

// Query with builder (uses Strapi v4 format)
let items = StrapiAdapter::query(&FederationQuery::new("articles")).await?;

// Fetch collection directly
let posts = StrapiAdapter::fetch_collection(Some("posts")).await?;
```

---

## Caching

The module includes an in-memory cache for performance.

```rust
use nucleus_std::federation::FederationCache;

let cache = FederationCache::new(60); // 60 second TTL

// Cache content
cache.set("articles", articles);

// Get cached content
if let Some(cached) = cache.get("articles") {
    return Ok(cached);
}

// Invalidate on update
cache.invalidate("articles");

// Clear all
cache.clear();
```

---

## Examples

### Fetch Articles from Multiple Sources

```rust
use nucleus_std::federation::*;

async fn get_all_articles() -> Result<Vec<ContentItem>> {
    let mut all = Vec::new();
    
    // Fetch from Directus
    let directus = Federation::source("directus")
        .collection("articles")
        .fetch()
        .await?;
    all.extend(directus);
    
    // Fetch from Sanity
    let sanity = Federation::source("sanity")
        .raw("*[_type == 'article'] | order(publishedAt desc)[0...10]")
        .fetch()
        .await?;
    all.extend(sanity);
    
    Ok(all)
}
```

### Paginated Query

```rust
async fn paginate(page: usize, per_page: usize) -> Result<Vec<ContentItem>> {
    Federation::query("posts")
        .filter("status", "published")
        .order_by("created_at", Direction::Desc)
        .limit(per_page)
        .offset((page - 1) * per_page)
        .fetch()
        .await
}
```

### Webhook Handler

```rust
use axum::{Json, extract::State};

async fn webhook(Json(payload): Json<WebhookPayload>) -> impl IntoResponse {
    let cache: FederationCache = get_cache();
    
    match payload.event {
        "item.create" | "item.update" | "item.delete" => {
            cache.invalidate(&payload.collection);
        }
        _ => {}
    }
    
    StatusCode::OK
}
```

---

## GROQ Query Reference

For Sanity, queries are converted to GROQ:

| Nucleus | GROQ |
|---------|------|
| `.filter("status", "published")` | `*[_type == 'article' && status == "published"]` |
| `.order_by("date", Desc)` | `\| order(date desc)` |
| `.limit(10)` | `[0...10]` |

Manual GROQ:
```rust
SanityAdapter::fetch_groq(Some(
    "*[_type == 'post' && defined(slug)] | order(publishedAt desc)[0...5]{
        title,
        slug,
        excerpt
    }"
)).await?;
```

---

## Best Practices

### 1. Use Environment Variables

```toml
url = "${DIRECTUS_URL}"
token = "${DIRECTUS_TOKEN}"
```

### 2. Cache Aggressively

```rust
let cache = FederationCache::new(300); // 5 minute TTL for content
```

### 3. Handle Errors Gracefully

```rust
match Federation::source("directus").collection("posts").fetch().await {
    Ok(posts) => render(posts),
    Err(e) => {
        log::error!("CMS fetch failed: {}", e);
        render(Vec::new()) // Fallback to empty
    }
}
```

---

## See Also

- [Getting Started](#01_getting_started)
- [API Development](#22_api_development)
- [Configuration](#configuration)
