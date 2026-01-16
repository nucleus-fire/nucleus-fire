//! Nucleus Scout - Full-Text Search
//!
//! Multi-backend search integration with Meilisearch and Typesense support.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::scout::{Scout, SearchQuery};
//!
//! // Initialize with Meilisearch
//! let scout = Scout::meilisearch("http://localhost:7700", "api_key").await?;
//!
//! // Index documents
//! scout.index("products", &products).await?;
//!
//! // Search with filters
//! let results = scout.search("products")
//!     .query("laptop")
//!     .filter("price < 1000")
//!     .sort("created_at:desc")
//!     .limit(20)
//!     .execute()
//!     .await?;
//!
//! for hit in results.hits {
//!     println!("{}: {}", hit.id, hit.score);
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Scout error types
#[derive(Debug, thiserror::Error)]
pub enum ScoutError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Index not found: {0}")]
    IndexNotFound(String),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Timeout")]
    Timeout,
}

impl From<reqwest::Error> for ScoutError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ScoutError::Timeout
        } else if err.is_connect() {
            ScoutError::ConnectionFailed(err.to_string())
        } else {
            ScoutError::BackendError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for ScoutError {
    fn from(err: serde_json::Error) -> Self {
        ScoutError::SerializationError(err.to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Index information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub primary_key: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Index task (async operation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexTask {
    pub task_id: u64,
    pub index_name: String,
    pub status: TaskStatus,
}

/// Task status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Enqueued,
    Processing,
    Succeeded,
    Failed,
}

/// Search parameters
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub query: String,
    pub filter: Option<String>,
    pub sort: Vec<String>,
    pub limit: usize,
    pub offset: usize,
    pub attributes_to_retrieve: Vec<String>,
    pub attributes_to_highlight: Vec<String>,
    pub facets: Vec<String>,
}

/// Search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub hits: Vec<SearchHit>,
    pub query: String,
    pub processing_time_ms: u64,
    pub total_hits: usize,
    pub facet_distribution: Option<HashMap<String, HashMap<String, usize>>>,
}

/// Individual search hit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub id: String,
    pub score: Option<f64>,
    pub document: serde_json::Value,
    pub highlights: Option<HashMap<String, String>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND ENUM (for dyn compatibility)
// ═══════════════════════════════════════════════════════════════════════════

/// Search backend implementations
enum Backend {
    Meilisearch(MeilisearchBackend),
    Typesense(TypesenseBackend),
}

impl Backend {
    async fn index(
        &self,
        index_name: &str,
        documents: serde_json::Value,
        primary_key: Option<&str>,
    ) -> Result<IndexTask, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.index(index_name, documents, primary_key).await,
            Backend::Typesense(b) => b.index(index_name, documents, primary_key).await,
        }
    }

    async fn delete(&self, index_name: &str, ids: &[String]) -> Result<IndexTask, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.delete(index_name, ids).await,
            Backend::Typesense(b) => b.delete(index_name, ids).await,
        }
    }

    async fn clear(&self, index_name: &str) -> Result<IndexTask, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.clear(index_name).await,
            Backend::Typesense(b) => b.clear(index_name).await,
        }
    }

    async fn search(&self, index_name: &str, params: &SearchParams) -> Result<SearchResults, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.search(index_name, params).await,
            Backend::Typesense(b) => b.search(index_name, params).await,
        }
    }

    async fn get(&self, index_name: &str, id: &str) -> Result<Option<serde_json::Value>, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.get(index_name, id).await,
            Backend::Typesense(b) => b.get(index_name, id).await,
        }
    }

    async fn create_index(&self, index_name: &str, primary_key: Option<&str>) -> Result<(), ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.create_index(index_name, primary_key).await,
            Backend::Typesense(b) => b.create_index(index_name, primary_key).await,
        }
    }

    async fn delete_index(&self, index_name: &str) -> Result<(), ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.delete_index(index_name).await,
            Backend::Typesense(b) => b.delete_index(index_name).await,
        }
    }

    async fn list_indexes(&self) -> Result<Vec<IndexInfo>, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.list_indexes().await,
            Backend::Typesense(b) => b.list_indexes().await,
        }
    }

    async fn task_status(&self, task_id: u64) -> Result<TaskStatus, ScoutError> {
        match self {
            Backend::Meilisearch(b) => b.task_status(task_id).await,
            Backend::Typesense(b) => b.task_status(task_id).await,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MEILISEARCH BACKEND
// ═══════════════════════════════════════════════════════════════════════════

/// Meilisearch backend implementation
struct MeilisearchBackend {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl MeilisearchBackend {
    fn new(url: &str, api_key: Option<&str>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url.trim_end_matches('/').to_string(),
            api_key: api_key.map(|s| s.to_string()),
        }
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.request(method, &url);
        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }
        req.header("Content-Type", "application/json")
    }

    async fn index(
        &self,
        index_name: &str,
        documents: serde_json::Value,
        primary_key: Option<&str>,
    ) -> Result<IndexTask, ScoutError> {
        let mut path = format!("/indexes/{}/documents", index_name);
        if let Some(pk) = primary_key {
            path.push_str(&format!("?primaryKey={}", pk));
        }

        let response = self
            .request(reqwest::Method::POST, &path)
            .json(&documents)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }
        if response.status() == 429 {
            return Err(ScoutError::RateLimited);
        }

        let task: MeilisearchTask = response.json().await?;
        Ok(IndexTask {
            task_id: task.task_uid,
            index_name: index_name.to_string(),
            status: TaskStatus::Enqueued,
        })
    }

    async fn delete(&self, index_name: &str, ids: &[String]) -> Result<IndexTask, ScoutError> {
        let path = format!("/indexes/{}/documents/delete-batch", index_name);
        let response = self
            .request(reqwest::Method::POST, &path)
            .json(ids)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        let task: MeilisearchTask = response.json().await?;
        Ok(IndexTask {
            task_id: task.task_uid,
            index_name: index_name.to_string(),
            status: TaskStatus::Enqueued,
        })
    }

    async fn clear(&self, index_name: &str) -> Result<IndexTask, ScoutError> {
        let path = format!("/indexes/{}/documents", index_name);
        let response = self
            .request(reqwest::Method::DELETE, &path)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        let task: MeilisearchTask = response.json().await?;
        Ok(IndexTask {
            task_id: task.task_uid,
            index_name: index_name.to_string(),
            status: TaskStatus::Enqueued,
        })
    }

    async fn search(&self, index_name: &str, params: &SearchParams) -> Result<SearchResults, ScoutError> {
        let path = format!("/indexes/{}/search", index_name);

        let mut body = serde_json::json!({
            "q": params.query,
            "limit": params.limit,
            "offset": params.offset,
        });

        if let Some(filter) = &params.filter {
            body["filter"] = serde_json::json!(filter);
        }
        if !params.sort.is_empty() {
            body["sort"] = serde_json::json!(params.sort);
        }
        if !params.attributes_to_retrieve.is_empty() {
            body["attributesToRetrieve"] = serde_json::json!(params.attributes_to_retrieve);
        }
        if !params.attributes_to_highlight.is_empty() {
            body["attributesToHighlight"] = serde_json::json!(params.attributes_to_highlight);
        }
        if !params.facets.is_empty() {
            body["facets"] = serde_json::json!(params.facets);
        }

        let response = self
            .request(reqwest::Method::POST, &path)
            .json(&body)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        let meili_results: MeilisearchSearchResults = response.json().await?;

        let hits = meili_results
            .hits
            .into_iter()
            .map(|hit| {
                let id = hit
                    .get("id")
                    .or_else(|| hit.get("_id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                SearchHit {
                    id,
                    score: None,
                    document: hit,
                    highlights: None,
                }
            })
            .collect();

        Ok(SearchResults {
            hits,
            query: params.query.clone(),
            processing_time_ms: meili_results.processing_time_ms,
            total_hits: meili_results.estimated_total_hits.unwrap_or(0),
            facet_distribution: meili_results.facet_distribution,
        })
    }

    async fn get(&self, index_name: &str, id: &str) -> Result<Option<serde_json::Value>, ScoutError> {
        let path = format!("/indexes/{}/documents/{}", index_name, id);
        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let doc = response.json().await?;
        Ok(Some(doc))
    }

    async fn create_index(&self, index_name: &str, primary_key: Option<&str>) -> Result<(), ScoutError> {
        let mut body = serde_json::json!({ "uid": index_name });
        if let Some(pk) = primary_key {
            body["primaryKey"] = serde_json::json!(pk);
        }

        let response = self
            .request(reqwest::Method::POST, "/indexes")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: serde_json::Value = response.json().await?;
            return Err(ScoutError::BackendError(
                error["message"].as_str().unwrap_or("Unknown error").to_string(),
            ));
        }

        Ok(())
    }

    async fn delete_index(&self, index_name: &str) -> Result<(), ScoutError> {
        let path = format!("/indexes/{}", index_name);
        let response = self.request(reqwest::Method::DELETE, &path).send().await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        Ok(())
    }

    async fn list_indexes(&self) -> Result<Vec<IndexInfo>, ScoutError> {
        let response = self.request(reqwest::Method::GET, "/indexes").send().await?;

        let result: MeilisearchIndexList = response.json().await?;
        Ok(result
            .results
            .into_iter()
            .map(|idx| IndexInfo {
                name: idx.uid,
                primary_key: idx.primary_key,
                created_at: idx.created_at,
                updated_at: idx.updated_at,
            })
            .collect())
    }

    async fn task_status(&self, task_id: u64) -> Result<TaskStatus, ScoutError> {
        let path = format!("/tasks/{}", task_id);
        let response = self.request(reqwest::Method::GET, &path).send().await?;

        let task: MeilisearchTaskStatus = response.json().await?;
        Ok(match task.status.as_str() {
            "enqueued" => TaskStatus::Enqueued,
            "processing" => TaskStatus::Processing,
            "succeeded" => TaskStatus::Succeeded,
            "failed" => TaskStatus::Failed,
            _ => TaskStatus::Enqueued,
        })
    }
}

// Meilisearch response types
#[derive(Debug, Deserialize)]
struct MeilisearchTask {
    #[serde(rename = "taskUid")]
    task_uid: u64,
}

#[derive(Debug, Deserialize)]
struct MeilisearchTaskStatus {
    status: String,
}

#[derive(Debug, Deserialize)]
struct MeilisearchSearchResults {
    hits: Vec<serde_json::Value>,
    #[serde(rename = "processingTimeMs")]
    processing_time_ms: u64,
    #[serde(rename = "estimatedTotalHits")]
    estimated_total_hits: Option<usize>,
    #[serde(rename = "facetDistribution")]
    facet_distribution: Option<HashMap<String, HashMap<String, usize>>>,
}

#[derive(Debug, Deserialize)]
struct MeilisearchIndexList {
    results: Vec<MeilisearchIndex>,
}

#[derive(Debug, Deserialize)]
struct MeilisearchIndex {
    uid: String,
    #[serde(rename = "primaryKey")]
    primary_key: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPESENSE BACKEND
// ═══════════════════════════════════════════════════════════════════════════

/// Typesense backend implementation
struct TypesenseBackend {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl TypesenseBackend {
    fn new(url: &str, api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        self.client
            .request(method, &url)
            .header("X-TYPESENSE-API-KEY", &self.api_key)
            .header("Content-Type", "application/json")
    }

    async fn index(
        &self,
        index_name: &str,
        documents: serde_json::Value,
        _primary_key: Option<&str>,
    ) -> Result<IndexTask, ScoutError> {
        let path = format!("/collections/{}/documents/import?action=upsert", index_name);

        // Convert to JSONL for Typesense
        let docs = documents.as_array().cloned().unwrap_or_default();
        let body: String = docs
            .iter()
            .filter_map(|doc| serde_json::to_string(doc).ok())
            .collect::<Vec<_>>()
            .join("\n");

        let response = self
            .request(reqwest::Method::POST, &path)
            .body(body)
            .send()
            .await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        Ok(IndexTask {
            task_id: 0,
            index_name: index_name.to_string(),
            status: TaskStatus::Succeeded,
        })
    }

    async fn delete(&self, index_name: &str, ids: &[String]) -> Result<IndexTask, ScoutError> {
        for id in ids {
            let path = format!("/collections/{}/documents/{}", index_name, id);
            self.request(reqwest::Method::DELETE, &path).send().await?;
        }

        Ok(IndexTask {
            task_id: 0,
            index_name: index_name.to_string(),
            status: TaskStatus::Succeeded,
        })
    }

    async fn clear(&self, index_name: &str) -> Result<IndexTask, ScoutError> {
        let path = format!("/collections/{}/documents?filter_by=id:>=0", index_name);
        self.request(reqwest::Method::DELETE, &path).send().await?;

        Ok(IndexTask {
            task_id: 0,
            index_name: index_name.to_string(),
            status: TaskStatus::Succeeded,
        })
    }

    async fn search(&self, index_name: &str, params: &SearchParams) -> Result<SearchResults, ScoutError> {
        let mut query_params = vec![
            format!("q={}", urlencoding::encode(&params.query)),
            format!("per_page={}", params.limit),
            format!("page={}", (params.offset / params.limit.max(1)) + 1),
            "query_by=*".to_string(),
        ];

        if let Some(filter) = &params.filter {
            query_params.push(format!("filter_by={}", urlencoding::encode(filter)));
        }
        if !params.sort.is_empty() {
            query_params.push(format!("sort_by={}", params.sort.join(",")));
        }

        let path = format!(
            "/collections/{}/documents/search?{}",
            index_name,
            query_params.join("&")
        );

        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        let ts_results: TypesenseSearchResults = response.json().await?;

        let hits = ts_results
            .hits
            .unwrap_or_default()
            .into_iter()
            .map(|hit| {
                let id = hit
                    .document
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                SearchHit {
                    id,
                    score: Some(hit.text_match as f64),
                    document: hit.document,
                    highlights: None,
                }
            })
            .collect();

        Ok(SearchResults {
            hits,
            query: params.query.clone(),
            processing_time_ms: ts_results.search_time_ms.unwrap_or(0),
            total_hits: ts_results.found.unwrap_or(0),
            facet_distribution: None,
        })
    }

    async fn get(&self, index_name: &str, id: &str) -> Result<Option<serde_json::Value>, ScoutError> {
        let path = format!("/collections/{}/documents/{}", index_name, id);
        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let doc = response.json().await?;
        Ok(Some(doc))
    }

    async fn create_index(&self, index_name: &str, _primary_key: Option<&str>) -> Result<(), ScoutError> {
        let schema = serde_json::json!({
            "name": index_name,
            "fields": [{"name": ".*", "type": "auto"}]
        });

        let response = self
            .request(reqwest::Method::POST, "/collections")
            .json(&schema)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: serde_json::Value = response.json().await?;
            return Err(ScoutError::BackendError(
                error["message"].as_str().unwrap_or("Unknown error").to_string(),
            ));
        }

        Ok(())
    }

    async fn delete_index(&self, index_name: &str) -> Result<(), ScoutError> {
        let path = format!("/collections/{}", index_name);
        let response = self.request(reqwest::Method::DELETE, &path).send().await?;

        if response.status() == 404 {
            return Err(ScoutError::IndexNotFound(index_name.to_string()));
        }

        Ok(())
    }

    async fn list_indexes(&self) -> Result<Vec<IndexInfo>, ScoutError> {
        let response = self.request(reqwest::Method::GET, "/collections").send().await?;

        let collections: Vec<TypesenseCollection> = response.json().await?;
        Ok(collections
            .into_iter()
            .map(|c| IndexInfo {
                name: c.name,
                primary_key: None,
                created_at: Some(c.created_at.to_string()),
                updated_at: None,
            })
            .collect())
    }

    async fn task_status(&self, _task_id: u64) -> Result<TaskStatus, ScoutError> {
        Ok(TaskStatus::Succeeded)
    }
}

// Typesense response types
#[derive(Debug, Deserialize)]
struct TypesenseSearchResults {
    hits: Option<Vec<TypesenseHit>>,
    found: Option<usize>,
    search_time_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct TypesenseHit {
    document: serde_json::Value,
    text_match: i64,
}

#[derive(Debug, Deserialize)]
struct TypesenseCollection {
    name: String,
    created_at: i64,
}

// ═══════════════════════════════════════════════════════════════════════════
// SCOUT (MAIN API)
// ═══════════════════════════════════════════════════════════════════════════

/// Main Scout search interface
pub struct Scout {
    backend: Backend,
}

impl Scout {
    /// Create Scout with Meilisearch backend
    pub fn meilisearch(url: &str, api_key: Option<&str>) -> Self {
        Self {
            backend: Backend::Meilisearch(MeilisearchBackend::new(url, api_key)),
        }
    }

    /// Create Scout with Typesense backend
    pub fn typesense(url: &str, api_key: &str) -> Self {
        Self {
            backend: Backend::Typesense(TypesenseBackend::new(url, api_key)),
        }
    }

    /// Index documents
    pub async fn index<T: Serialize>(&self, index_name: &str, documents: &[T]) -> Result<IndexTask, ScoutError> {
        let json = serde_json::to_value(documents)?;
        self.backend.index(index_name, json, None).await
    }

    /// Index documents with primary key
    pub async fn index_with_key<T: Serialize>(
        &self,
        index_name: &str,
        documents: &[T],
        primary_key: &str,
    ) -> Result<IndexTask, ScoutError> {
        let json = serde_json::to_value(documents)?;
        self.backend.index(index_name, json, Some(primary_key)).await
    }

    /// Start a search query
    pub fn search(&self, index_name: &str) -> SearchQuery<'_> {
        SearchQuery::new(self, index_name)
    }

    /// Get a document by ID
    pub async fn get(&self, index_name: &str, id: &str) -> Result<Option<serde_json::Value>, ScoutError> {
        self.backend.get(index_name, id).await
    }

    /// Delete documents by IDs
    pub async fn delete(&self, index_name: &str, ids: &[String]) -> Result<IndexTask, ScoutError> {
        self.backend.delete(index_name, ids).await
    }

    /// Clear all documents from an index
    pub async fn clear(&self, index_name: &str) -> Result<IndexTask, ScoutError> {
        self.backend.clear(index_name).await
    }

    /// Create an index
    pub async fn create_index(&self, index_name: &str) -> Result<(), ScoutError> {
        self.backend.create_index(index_name, None).await
    }

    /// Create an index with primary key
    pub async fn create_index_with_key(&self, index_name: &str, primary_key: &str) -> Result<(), ScoutError> {
        self.backend.create_index(index_name, Some(primary_key)).await
    }

    /// Delete an index
    pub async fn delete_index(&self, index_name: &str) -> Result<(), ScoutError> {
        self.backend.delete_index(index_name).await
    }

    /// List all indexes
    pub async fn list_indexes(&self) -> Result<Vec<IndexInfo>, ScoutError> {
        self.backend.list_indexes().await
    }

    /// Check task status
    pub async fn task_status(&self, task_id: u64) -> Result<TaskStatus, ScoutError> {
        self.backend.task_status(task_id).await
    }

    /// Wait for a task to complete
    pub async fn wait_for_task(&self, task: &IndexTask, timeout_ms: u64) -> Result<TaskStatus, ScoutError> {
        let start = std::time::Instant::now();
        loop {
            let status = self.backend.task_status(task.task_id).await?;
            if status == TaskStatus::Succeeded || status == TaskStatus::Failed {
                return Ok(status);
            }
            if start.elapsed().as_millis() as u64 > timeout_ms {
                return Err(ScoutError::Timeout);
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SEARCH QUERY BUILDER
// ═══════════════════════════════════════════════════════════════════════════

/// Fluent search query builder
pub struct SearchQuery<'a> {
    scout: &'a Scout,
    index_name: String,
    params: SearchParams,
}

impl<'a> SearchQuery<'a> {
    fn new(scout: &'a Scout, index_name: &str) -> Self {
        Self {
            scout,
            index_name: index_name.to_string(),
            params: SearchParams {
                limit: 20,
                ..Default::default()
            },
        }
    }

    /// Set the search query string
    pub fn query(mut self, query: &str) -> Self {
        self.params.query = query.to_string();
        self
    }

    /// Add a filter
    pub fn filter(mut self, filter: &str) -> Self {
        self.params.filter = Some(filter.to_string());
        self
    }

    /// Add sorting
    pub fn sort(mut self, sort: &str) -> Self {
        self.params.sort.push(sort.to_string());
        self
    }

    /// Set result limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.params.limit = limit;
        self
    }

    /// Set result offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.params.offset = offset;
        self
    }

    /// Specify attributes to retrieve
    pub fn attributes(mut self, attrs: &[&str]) -> Self {
        self.params.attributes_to_retrieve = attrs.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Specify attributes to highlight
    pub fn highlight(mut self, attrs: &[&str]) -> Self {
        self.params.attributes_to_highlight = attrs.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add facets for aggregation
    pub fn facets(mut self, facets: &[&str]) -> Self {
        self.params.facets = facets.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Execute the search
    pub async fn execute(self) -> Result<SearchResults, ScoutError> {
        self.scout.backend.search(&self.index_name, &self.params).await
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // ERROR TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_error_connection_failed() {
        let err = ScoutError::ConnectionFailed("localhost:7700".to_string());
        assert!(err.to_string().contains("Connection failed"));
    }

    #[test]
    fn test_error_index_not_found() {
        let err = ScoutError::IndexNotFound("products".to_string());
        assert!(err.to_string().contains("Index not found"));
    }

    #[test]
    fn test_error_document_not_found() {
        let err = ScoutError::DocumentNotFound("doc_123".to_string());
        assert!(err.to_string().contains("Document not found"));
    }

    #[test]
    fn test_error_invalid_query() {
        let err = ScoutError::InvalidQuery("bad syntax".to_string());
        assert!(err.to_string().contains("Invalid query"));
    }

    #[test]
    fn test_error_serialization() {
        let err = ScoutError::SerializationError("invalid JSON".to_string());
        assert!(err.to_string().contains("Serialization"));
    }

    #[test]
    fn test_error_backend() {
        let err = ScoutError::BackendError("internal error".to_string());
        assert!(err.to_string().contains("Backend"));
    }

    #[test]
    fn test_error_rate_limited() {
        let err = ScoutError::RateLimited;
        assert!(err.to_string().contains("Rate limited"));
    }

    #[test]
    fn test_error_timeout() {
        let err = ScoutError::Timeout;
        assert!(err.to_string().contains("Timeout"));
    }

    #[test]
    fn test_error_config() {
        let err = ScoutError::ConfigError("missing key".to_string());
        assert!(err.to_string().contains("Configuration"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TYPE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_search_params_default() {
        let params = SearchParams::default();
        assert!(params.query.is_empty());
        assert!(params.filter.is_none());
        assert!(params.sort.is_empty());
        assert_eq!(params.limit, 0);
    }

    #[test]
    fn test_index_info() {
        let info = IndexInfo {
            name: "products".to_string(),
            primary_key: Some("id".to_string()),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(info.name, "products");
    }

    #[test]
    fn test_index_task() {
        let task = IndexTask {
            task_id: 42,
            index_name: "products".to_string(),
            status: TaskStatus::Enqueued,
        };
        assert_eq!(task.task_id, 42);
    }

    #[test]
    fn test_task_status_variants() {
        assert_eq!(TaskStatus::Enqueued, TaskStatus::Enqueued);
        assert_ne!(TaskStatus::Enqueued, TaskStatus::Failed);
    }

    #[test]
    fn test_search_results() {
        let results = SearchResults {
            hits: vec![],
            query: "test".to_string(),
            processing_time_ms: 5,
            total_hits: 0,
            facet_distribution: None,
        };
        assert_eq!(results.query, "test");
    }

    #[test]
    fn test_search_hit() {
        let hit = SearchHit {
            id: "doc_1".to_string(),
            score: Some(0.95),
            document: serde_json::json!({}),
            highlights: None,
        };
        assert_eq!(hit.id, "doc_1");
    }

    #[test]
    fn test_search_hit_with_highlights() {
        let mut highlights = HashMap::new();
        highlights.insert("title".to_string(), "<em>Hello</em>".to_string());
        let hit = SearchHit {
            id: "doc_1".to_string(),
            score: None,
            document: serde_json::json!({}),
            highlights: Some(highlights),
        };
        assert!(hit.highlights.is_some());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SCOUT API TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_scout_meilisearch() {
        let scout = Scout::meilisearch("http://localhost:7700", Some("key"));
        drop(scout);
    }

    #[test]
    fn test_scout_typesense() {
        let scout = Scout::typesense("http://localhost:8108", "key");
        drop(scout);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // QUERY BUILDER TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_query_builder_chain() {
        let scout = Scout::meilisearch("http://localhost:7700", None);
        let query = scout
            .search("products")
            .query("laptop")
            .filter("price < 1000")
            .sort("created_at:desc")
            .limit(20)
            .offset(10);

        assert_eq!(query.params.query, "laptop");
        assert_eq!(query.params.filter, Some("price < 1000".to_string()));
        assert_eq!(query.params.limit, 20);
    }

    #[test]
    fn test_query_builder_attributes() {
        let scout = Scout::meilisearch("http://localhost:7700", None);
        let query = scout.search("products").attributes(&["id", "title"]);
        assert_eq!(query.params.attributes_to_retrieve.len(), 2);
    }

    #[test]
    fn test_query_builder_highlight() {
        let scout = Scout::meilisearch("http://localhost:7700", None);
        let query = scout.search("products").highlight(&["title"]);
        assert_eq!(query.params.attributes_to_highlight.len(), 1);
    }

    #[test]
    fn test_query_builder_facets() {
        let scout = Scout::meilisearch("http://localhost:7700", None);
        let query = scout.search("products").facets(&["category"]);
        assert_eq!(query.params.facets.len(), 1);
    }

    #[test]
    fn test_query_builder_multiple_sorts() {
        let scout = Scout::meilisearch("http://localhost:7700", None);
        let query = scout.search("products").sort("price:asc").sort("date:desc");
        assert_eq!(query.params.sort.len(), 2);
    }

    #[test]
    fn test_query_default_limit() {
        let scout = Scout::meilisearch("http://localhost:7700", None);
        let query = scout.search("products");
        assert_eq!(query.params.limit, 20);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SERIALIZATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_task_status_serialization() {
        let status = TaskStatus::Succeeded;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"succeeded\"");
    }

    #[test]
    fn test_task_status_deserialization() {
        let status: TaskStatus = serde_json::from_str("\"processing\"").unwrap();
        assert_eq!(status, TaskStatus::Processing);
    }

    #[test]
    fn test_index_info_serialization() {
        let info = IndexInfo {
            name: "test".to_string(),
            primary_key: None,
            created_at: None,
            updated_at: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"name\":\"test\""));
    }
}
