//! Content Federation Module
//!
//! Provides a unified interface for querying content from multiple headless CMS platforms:
//! - Directus (REST/GraphQL)
//! - Sanity (GROQ)
//! - Strapi (REST/GraphQL)
//!
//! # Example
//! ```rust,ignore
//! use nucleus_std::federation::*;
//!
//! let articles = Federation::query("articles")
//!     .filter("status", "published")
//!     .limit(10)
//!     .fetch()
//!     .await?;
//! ```

use crate::errors::{NucleusError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported CMS types
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CmsType {
    Directus,
    Sanity,
    Strapi,
    Custom,
}

/// Configuration for a content source
#[derive(Debug, Clone, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub cms_type: CmsType,
    pub url: Option<String>,
    pub token: Option<String>,
    pub project_id: Option<String>,
    pub dataset: Option<String>,
}

/// Unified content item from any CMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: String,
    pub source: String,
    pub collection: String,
    pub data: serde_json::Value,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Query builder for federated content
#[derive(Debug, Clone)]
pub struct FederationQuery {
    collection: String,
    source: Option<String>,
    filters: HashMap<String, serde_json::Value>,
    order_by: Option<(String, Direction)>,
    limit: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Asc,
    Desc,
}

impl FederationQuery {
    pub fn new(collection: &str) -> Self {
        Self {
            collection: collection.to_string(),
            source: None,
            filters: HashMap::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    /// Filter by a specific source
    pub fn from_source(mut self, source: &str) -> Self {
        self.source = Some(source.to_string());
        self
    }

    /// Add a filter condition
    pub fn filter(mut self, field: &str, value: impl Into<serde_json::Value>) -> Self {
        self.filters.insert(field.to_string(), value.into());
        self
    }

    /// Order results
    pub fn order_by(mut self, field: &str, direction: Direction) -> Self {
        self.order_by = Some((field.to_string(), direction));
        self
    }

    /// Limit results
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Offset for pagination
    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
        self
    }

    /// Execute the query
    pub async fn fetch(self) -> Result<Vec<ContentItem>> {
        Federation::execute_query(self).await
    }
}

/// Main Federation interface
pub struct Federation;

impl Federation {
    /// Start a query for a collection
    pub fn query(collection: &str) -> FederationQuery {
        FederationQuery::new(collection)
    }

    /// Query a specific source
    pub fn source(name: &str) -> SourceQuery {
        SourceQuery::new(name)
    }

    /// Execute a federated query
    pub async fn execute_query(query: FederationQuery) -> Result<Vec<ContentItem>> {
        let _config = crate::config::Config::load();

        // If no source specified, query all configured sources
        let sources = if let Some(ref source_name) = query.source {
            vec![source_name.clone()]
        } else {
            // Would normally come from config
            vec![]
        };

        let mut results = Vec::new();

        for source_name in sources {
            let items = match source_name.as_str() {
                "directus" => DirectusAdapter::query(&query).await?,
                "sanity" => SanityAdapter::query(&query).await?,
                "strapi" => StrapiAdapter::query(&query).await?,
                _ => vec![],
            };
            results.extend(items);
        }

        // Apply ordering if specified
        if let Some((field, direction)) = query.order_by {
            results.sort_by(|a, b| {
                let a_val = a.data.get(&field);
                let b_val = b.data.get(&field);
                let cmp = match (a_val, b_val) {
                    (Some(a), Some(b)) => a.to_string().cmp(&b.to_string()),
                    _ => std::cmp::Ordering::Equal,
                };
                match direction {
                    Direction::Asc => cmp,
                    Direction::Desc => cmp.reverse(),
                }
            });
        }

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }
}

/// Source-specific query builder
pub struct SourceQuery {
    source: String,
    collection: Option<String>,
    raw_query: Option<String>,
}

impl SourceQuery {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            collection: None,
            raw_query: None,
        }
    }

    /// Query a specific collection
    pub fn collection(mut self, name: &str) -> Self {
        self.collection = Some(name.to_string());
        self
    }

    /// Execute a raw query (GROQ for Sanity, GraphQL for others)
    pub fn raw(mut self, query: &str) -> Self {
        self.raw_query = Some(query.to_string());
        self
    }

    /// Execute the query
    pub async fn fetch(self) -> Result<Vec<ContentItem>> {
        match self.source.as_str() {
            "directus" => DirectusAdapter::fetch_collection(self.collection.as_deref()).await,
            "sanity" => SanityAdapter::fetch_groq(self.raw_query.as_deref()).await,
            "strapi" => StrapiAdapter::fetch_collection(self.collection.as_deref()).await,
            _ => Err(NucleusError::InternalError(format!(
                "Unknown source: {}",
                self.source
            ))),
        }
    }
}

// ============================================================================
// CMS Adapters
// ============================================================================

/// Directus CMS Adapter
/// Supports REST and GraphQL APIs over SQL databases
pub struct DirectusAdapter;

impl DirectusAdapter {
    /// Query Directus using the unified query format
    pub async fn query(query: &FederationQuery) -> Result<Vec<ContentItem>> {
        // Build Directus REST API URL
        let base_url =
            std::env::var("DIRECTUS_URL").unwrap_or_else(|_| "http://localhost:8055".to_string());
        let _token = std::env::var("DIRECTUS_TOKEN").ok();

        let url = format!("{}/items/{}", base_url, query.collection);

        // Build query parameters
        let mut params = Vec::new();

        if let Some(limit) = query.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = query.offset {
            params.push(format!("offset={}", offset));
        }

        // Build filter
        if !query.filters.is_empty() {
            let filter_json = serde_json::to_string(&query.filters)
                .map_err(|e| NucleusError::InternalError(e.to_string()))?;
            params.push(format!("filter={}", urlencoding::encode(&filter_json)));
        }

        // Build sort
        if let Some((field, direction)) = &query.order_by {
            let prefix = match direction {
                Direction::Asc => "",
                Direction::Desc => "-",
            };
            params.push(format!("sort={}{}", prefix, field));
        }

        let _full_url = if params.is_empty() {
            url
        } else {
            format!("{}?{}", url, params.join("&"))
        };

        // Make request (mocked for testing)
        #[cfg(test)]
        {
            Ok(Self::mock_response(&query.collection))
        }

        #[cfg(not(test))]
        {
            Self::http_get(&_full_url, _token.as_deref()).await
        }
    }

    pub async fn fetch_collection(collection: Option<&str>) -> Result<Vec<ContentItem>> {
        let collection = collection
            .ok_or_else(|| NucleusError::ValidationError("Collection name required".to_string()))?;

        let query = FederationQuery::new(collection);
        Self::query(&query).await
    }

    #[cfg(not(test))]
    async fn http_get(url: &str, token: Option<&str>) -> Result<Vec<ContentItem>> {
        let client = reqwest::Client::new();
        let mut req = client.get(url);

        if let Some(t) = token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }

        let res = req
            .send()
            .await
            .map_err(|e| NucleusError::InternalError(format!("HTTP request failed: {}", e)))?;

        if !res.status().is_success() {
            return Err(NucleusError::InternalError(format!(
                "HTTP error: {}",
                res.status()
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| NucleusError::InternalError(format!("JSON parse failed: {}", e)))?;

        // Parse Directus response format: { "data": [...] }
        let items = body
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| NucleusError::InternalError("Invalid Directus response".to_string()))?;

        Ok(items
            .iter()
            .enumerate()
            .map(|(i, item)| ContentItem {
                id: item
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&format!("{}", i))
                    .to_string(),
                source: "directus".to_string(),
                collection: "items".to_string(),
                data: item.clone(),
                created_at: item
                    .get("date_created")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                updated_at: item
                    .get("date_updated")
                    .and_then(|v| v.as_str())
                    .map(String::from),
            })
            .collect())
    }

    #[cfg(test)]
    fn mock_response(collection: &str) -> Vec<ContentItem> {
        vec![ContentItem {
            id: "1".to_string(),
            source: "directus".to_string(),
            collection: collection.to_string(),
            data: serde_json::json!({
                "title": "Test Article from Directus",
                "status": "published"
            }),
            created_at: Some("2024-01-01T00:00:00Z".to_string()),
            updated_at: Some("2024-01-01T00:00:00Z".to_string()),
        }]
    }
}

/// Sanity CMS Adapter
/// Supports GROQ queries and real-time APIs
pub struct SanityAdapter;

impl SanityAdapter {
    /// Query Sanity using the unified query format
    pub async fn query(query: &FederationQuery) -> Result<Vec<ContentItem>> {
        // Convert to GROQ
        let groq = Self::to_groq(query);
        Self::fetch_groq(Some(&groq)).await
    }

    pub async fn fetch_groq(groq: Option<&str>) -> Result<Vec<ContentItem>> {
        let project_id = std::env::var("SANITY_PROJECT_ID").unwrap_or_else(|_| "demo".to_string());
        let dataset = std::env::var("SANITY_DATASET").unwrap_or_else(|_| "production".to_string());
        let _token = std::env::var("SANITY_TOKEN").ok();

        let groq = groq.unwrap_or("*[_type == 'article']");

        let _url = format!(
            "https://{}.api.sanity.io/v2021-06-07/data/query/{}?query={}",
            project_id,
            dataset,
            urlencoding::encode(groq)
        );

        #[cfg(test)]
        {
            Ok(Self::mock_response())
        }

        #[cfg(not(test))]
        {
            Self::http_get(&_url, _token.as_deref()).await
        }
    }

    /// Convert unified query to GROQ
    fn to_groq(query: &FederationQuery) -> String {
        let mut groq = format!("*[_type == '{}']", query.collection);

        // Add filters
        if !query.filters.is_empty() {
            let filters: Vec<String> = query
                .filters
                .iter()
                .map(|(k, v)| format!("{} == {}", k, v))
                .collect();
            groq = format!(
                "*[_type == '{}' && {}]",
                query.collection,
                filters.join(" && ")
            );
        }

        // Add ordering
        if let Some((field, direction)) = &query.order_by {
            let dir = match direction {
                Direction::Asc => "asc",
                Direction::Desc => "desc",
            };
            groq = format!("{} | order({} {})", groq, field, dir);
        }

        // Add limit
        if let Some(limit) = query.limit {
            groq = format!("{}[0...{}]", groq, limit);
        }

        groq
    }

    #[cfg(not(test))]
    async fn http_get(url: &str, _token: Option<&str>) -> Result<Vec<ContentItem>> {
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .send()
            .await
            .map_err(|e| NucleusError::InternalError(format!("HTTP request failed: {}", e)))?;

        if !res.status().is_success() {
            return Err(NucleusError::InternalError(format!(
                "HTTP error: {}",
                res.status()
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| NucleusError::InternalError(format!("JSON parse failed: {}", e)))?;

        // Parse Sanity response format: { "result": [...] }
        let items = body
            .get("result")
            .and_then(|d| d.as_array())
            .ok_or_else(|| NucleusError::InternalError("Invalid Sanity response".to_string()))?;

        Ok(items
            .iter()
            .enumerate()
            .map(|(i, item)| ContentItem {
                id: item
                    .get("_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&format!("{}", i))
                    .to_string(),
                source: "sanity".to_string(),
                collection: item
                    .get("_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("document")
                    .to_string(),
                data: item.clone(),
                created_at: item
                    .get("_createdAt")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                updated_at: item
                    .get("_updatedAt")
                    .and_then(|v| v.as_str())
                    .map(String::from),
            })
            .collect())
    }

    #[cfg(test)]
    fn mock_response() -> Vec<ContentItem> {
        vec![ContentItem {
            id: "sanity-1".to_string(),
            source: "sanity".to_string(),
            collection: "article".to_string(),
            data: serde_json::json!({
                "title": "Test Article from Sanity",
                "status": "published"
            }),
            created_at: Some("2024-01-01T00:00:00Z".to_string()),
            updated_at: Some("2024-01-01T00:00:00Z".to_string()),
        }]
    }
}

/// Strapi CMS Adapter
/// Supports REST and GraphQL APIs with webhooks
pub struct StrapiAdapter;

impl StrapiAdapter {
    /// Query Strapi using the unified query format
    pub async fn query(query: &FederationQuery) -> Result<Vec<ContentItem>> {
        let base_url =
            std::env::var("STRAPI_URL").unwrap_or_else(|_| "http://localhost:1337".to_string());
        let _token = std::env::var("STRAPI_TOKEN").ok();

        // Strapi v4 uses /api prefix
        let url = format!("{}/api/{}", base_url, query.collection);

        // Build query parameters (Strapi v4 format)
        let mut params = Vec::new();

        if let Some(limit) = query.limit {
            params.push(format!("pagination[pageSize]={}", limit));
        }
        if let Some(offset) = query.offset {
            let page = (offset / query.limit.unwrap_or(25)) + 1;
            params.push(format!("pagination[page]={}", page));
        }

        // Filters (Strapi v4 format)
        for (key, value) in &query.filters {
            params.push(format!("filters[{}][$eq]={}", key, value));
        }

        // Sort
        if let Some((field, direction)) = &query.order_by {
            let dir = match direction {
                Direction::Asc => "asc",
                Direction::Desc => "desc",
            };
            params.push(format!("sort={}:{}", field, dir));
        }

        let _full_url = if params.is_empty() {
            url
        } else {
            format!("{}?{}", url, params.join("&"))
        };

        #[cfg(test)]
        {
            Ok(Self::mock_response(&query.collection))
        }

        #[cfg(not(test))]
        {
            Self::http_get(&_full_url, _token.as_deref()).await
        }
    }

    pub async fn fetch_collection(collection: Option<&str>) -> Result<Vec<ContentItem>> {
        let collection = collection
            .ok_or_else(|| NucleusError::ValidationError("Collection name required".to_string()))?;

        let query = FederationQuery::new(collection);
        Self::query(&query).await
    }

    #[cfg(not(test))]
    async fn http_get(url: &str, token: Option<&str>) -> Result<Vec<ContentItem>> {
        let client = reqwest::Client::new();
        let mut req = client.get(url);

        if let Some(t) = token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }

        let res = req
            .send()
            .await
            .map_err(|e| NucleusError::InternalError(format!("HTTP request failed: {}", e)))?;

        if !res.status().is_success() {
            return Err(NucleusError::InternalError(format!(
                "HTTP error: {}",
                res.status()
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| NucleusError::InternalError(format!("JSON parse failed: {}", e)))?;

        // Parse Strapi v4 response format: { "data": [...] }
        let items = body
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| NucleusError::InternalError("Invalid Strapi response".to_string()))?;

        Ok(items
            .iter()
            .enumerate()
            .map(|(i, item)| ContentItem {
                id: item
                    .get("id")
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| format!("{}", i)),
                source: "strapi".to_string(),
                collection: "items".to_string(),
                data: item.get("attributes").cloned().unwrap_or(item.clone()),
                created_at: item
                    .get("attributes")
                    .and_then(|a| a.get("createdAt"))
                    .and_then(|v| v.as_str())
                    .map(String::from),
                updated_at: item
                    .get("attributes")
                    .and_then(|a| a.get("updatedAt"))
                    .and_then(|v| v.as_str())
                    .map(String::from),
            })
            .collect())
    }

    #[cfg(test)]
    fn mock_response(collection: &str) -> Vec<ContentItem> {
        vec![ContentItem {
            id: "strapi-1".to_string(),
            source: "strapi".to_string(),
            collection: collection.to_string(),
            data: serde_json::json!({
                "title": "Test Article from Strapi",
                "status": "published"
            }),
            created_at: Some("2024-01-01T00:00:00Z".to_string()),
            updated_at: Some("2024-01-01T00:00:00Z".to_string()),
        }]
    }
}

// ============================================================================
// Caching Layer
// ============================================================================

/// Simple in-memory cache for federation queries
pub struct FederationCache {
    entries: std::sync::RwLock<HashMap<String, CacheEntry>>,
    ttl_seconds: u64,
}

struct CacheEntry {
    data: Vec<ContentItem>,
    expires_at: std::time::Instant,
}

impl FederationCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            entries: std::sync::RwLock::new(HashMap::new()),
            ttl_seconds,
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<ContentItem>> {
        let entries = self.entries.read().ok()?;
        let entry = entries.get(key)?;

        if entry.expires_at > std::time::Instant::now() {
            Some(entry.data.clone())
        } else {
            None
        }
    }

    pub fn set(&self, key: &str, data: Vec<ContentItem>) {
        if let Ok(mut entries) = self.entries.write() {
            entries.insert(
                key.to_string(),
                CacheEntry {
                    data,
                    expires_at: std::time::Instant::now()
                        + std::time::Duration::from_secs(self.ttl_seconds),
                },
            );
        }
    }

    pub fn invalidate(&self, key: &str) {
        if let Ok(mut entries) = self.entries.write() {
            entries.remove(key);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directus_query_building() {
        let query = FederationQuery::new("articles")
            .filter("status", "published")
            .order_by("created_at", Direction::Desc)
            .limit(10);

        assert_eq!(query.collection, "articles");
        assert_eq!(query.limit, Some(10));
        assert!(query.filters.contains_key("status"));
    }

    #[test]
    fn test_sanity_groq_conversion() {
        let query = FederationQuery::new("article")
            .filter("status", "published")
            .order_by("publishedAt", Direction::Desc)
            .limit(5);

        let groq = SanityAdapter::to_groq(&query);

        assert!(groq.contains("_type == 'article'"));
        assert!(groq.contains("order(publishedAt desc)"));
        assert!(groq.contains("[0...5]"));
    }

    #[tokio::test]
    async fn test_directus_mock_query() {
        let items = DirectusAdapter::query(&FederationQuery::new("articles"))
            .await
            .unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "directus");
        assert_eq!(items[0].collection, "articles");
    }

    #[tokio::test]
    async fn test_sanity_mock_query() {
        let items = SanityAdapter::query(&FederationQuery::new("article"))
            .await
            .unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "sanity");
    }

    #[tokio::test]
    async fn test_strapi_mock_query() {
        let items = StrapiAdapter::query(&FederationQuery::new("articles"))
            .await
            .unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "strapi");
    }

    #[test]
    fn test_cache_operations() {
        let cache = FederationCache::new(60);

        let items = vec![ContentItem {
            id: "1".to_string(),
            source: "test".to_string(),
            collection: "test".to_string(),
            data: serde_json::json!({}),
            created_at: None,
            updated_at: None,
        }];

        cache.set("test_key", items.clone());

        let cached = cache.get("test_key");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);

        cache.invalidate("test_key");
        assert!(cache.get("test_key").is_none());
    }

    #[test]
    fn test_content_item_serialization() {
        let item = ContentItem {
            id: "123".to_string(),
            source: "directus".to_string(),
            collection: "posts".to_string(),
            data: serde_json::json!({"title": "Hello"}),
            created_at: Some("2024-01-01".to_string()),
            updated_at: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("directus"));
        assert!(json.contains("Hello"));

        let parsed: ContentItem = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "123");
    }

    #[test]
    fn test_cms_type_deserialization() {
        let config: SourceConfig = serde_json::from_str(
            r#"{
            "name": "my-cms",
            "type": "directus",
            "url": "https://example.com"
        }"#,
        )
        .unwrap();

        assert_eq!(config.cms_type, CmsType::Directus);
        assert_eq!(config.name, "my-cms");
    }
}
