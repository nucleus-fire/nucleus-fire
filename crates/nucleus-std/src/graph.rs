//! Nucleus Graph - GraphQL Support
//!
//! Full-featured GraphQL server with async-graphql integration.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::graph::{GraphQL, Schema, Object, Context};
//!
//! #[derive(SimpleObject)]
//! struct User {
//!     id: i32,
//!     name: String,
//! }
//!
//! struct Query;
//!
//! #[Object]
//! impl Query {
//!     async fn user(&self, id: i32) -> Option<User> {
//!         Some(User { id, name: "Test".into() })
//!     }
//! }
//!
//! let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
//! router.route("/graphql", GraphQL::handler(schema));
//! router.route("/graphql/playground", GraphQL::playground());
//! ```

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-export async-graphql types for convenience
pub use async_graphql::{
    Context, EmptyMutation, EmptySubscription, InputObject, Interface, MergedObject,
    Object, Schema, SimpleObject, Subscription, Union, ID,
};

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// GraphQL error types
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Query execution failed: {0}")]
    QueryError(String),

    #[error("Schema build failed: {0}")]
    SchemaBuildError(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Authentication required")]
    Unauthenticated,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<async_graphql::Error> for GraphError {
    fn from(err: async_graphql::Error) -> Self {
        GraphError::QueryError(err.message)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GRAPHQL REQUEST/RESPONSE
// ═══════════════════════════════════════════════════════════════════════════

/// GraphQL request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    #[serde(default)]
    pub operation_name: Option<String>,
    #[serde(default)]
    pub variables: Option<serde_json::Value>,
}

impl GraphQLRequest {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            operation_name: None,
            variables: None,
        }
    }

    pub fn operation(mut self, name: &str) -> Self {
        self.operation_name = Some(name.to_string());
        self
    }

    pub fn variables(mut self, vars: serde_json::Value) -> Self {
        self.variables = Some(vars);
        self
    }
}

/// GraphQL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<GraphQLError>,
}

impl GraphQLResponse {
    pub fn data(data: serde_json::Value) -> Self {
        Self {
            data: Some(data),
            errors: vec![],
        }
    }

    pub fn error(err: GraphQLError) -> Self {
        Self {
            data: None,
            errors: vec![err],
        }
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}

/// GraphQL error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ErrorLocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
}

impl GraphQLError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            locations: None,
            path: None,
            extensions: None,
        }
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.extensions = Some(serde_json::json!({ "code": code }));
        self
    }
}

/// Error location in query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
}

// ═══════════════════════════════════════════════════════════════════════════
// GRAPHQL HANDLER
// ═══════════════════════════════════════════════════════════════════════════

/// GraphQL handler utilities
pub struct GraphQL;

impl GraphQL {
    /// Create an axum handler for GraphQL queries
    #[allow(clippy::type_complexity)]
    pub fn handler<Q, M, S>(
        schema: Schema<Q, M, S>,
    ) -> impl Fn(
        axum::extract::State<Schema<Q, M, S>>,
        axum::Json<GraphQLRequest>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = axum::Json<GraphQLResponse>> + Send>>
           + Clone
           + Send
    where
        Q: async_graphql::ObjectType + 'static,
        M: async_graphql::ObjectType + 'static,
        S: async_graphql::SubscriptionType + 'static,
    {
        let _ = schema; // Just to use the param
        |state: axum::extract::State<Schema<Q, M, S>>,
         axum::Json(req): axum::Json<GraphQLRequest>| {
            Box::pin(async move {
                let mut request = async_graphql::Request::new(&req.query);
                if let Some(op) = req.operation_name {
                    request = request.operation_name(op);
                }
                if let Some(vars) = req.variables {
                    let variables = async_graphql::Variables::from_json(vars);
                    request = request.variables(variables);
                }

                let response = state.execute(request).await;
                let data = response.data.into_json().ok();
                let errors: Vec<GraphQLError> = response
                    .errors
                    .into_iter()
                    .map(|e| GraphQLError::new(&e.message))
                    .collect();

                axum::Json(GraphQLResponse { data, errors })
            })
        }
    }

    /// Create a GraphQL playground HTML page
    pub fn playground_html(endpoint: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>GraphQL Playground</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
    <link rel="shortcut icon" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
    <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
</head>
<body>
    <div id="root"></div>
    <script>
        window.addEventListener('load', function (event) {{
            GraphQLPlayground.init(document.getElementById('root'), {{
                endpoint: '{}',
                settings: {{
                    'request.credentials': 'same-origin'
                }}
            }})
        }})
    </script>
</body>
</html>"#,
            endpoint
        )
    }

    /// Create an axum handler for GraphQL playground
    pub async fn playground() -> axum::response::Html<String> {
        axum::response::Html(Self::playground_html("/graphql"))
    }

    /// Create GraphiQL HTML page (alternative to playground)
    pub fn graphiql_html(endpoint: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>GraphiQL</title>
    <style>
        body {{ margin: 0; height: 100vh; }}
        #graphiql {{ height: 100%; }}
    </style>
    <script crossorigin src="https://unpkg.com/react@18/umd/react.production.min.js"></script>
    <script crossorigin src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js"></script>
    <link rel="stylesheet" href="https://unpkg.com/graphiql/graphiql.min.css" />
    <script crossorigin src="https://unpkg.com/graphiql/graphiql.min.js"></script>
</head>
<body>
    <div id="graphiql"></div>
    <script>
        const fetcher = GraphiQL.createFetcher({{ url: '{}' }});
        ReactDOM.render(
            React.createElement(GraphiQL, {{ fetcher: fetcher }}),
            document.getElementById('graphiql')
        );
    </script>
</body>
</html>"#,
            endpoint
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DATALOADER
// ═══════════════════════════════════════════════════════════════════════════

/// Simple DataLoader for N+1 prevention
pub struct DataLoader<K, V> {
    cache: Arc<RwLock<std::collections::HashMap<K, V>>>,
}

impl<K, V> DataLoader<K, V>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Load a single item by key
    pub async fn load<F, Fut>(&self, key: K, loader: F) -> Option<V>
    where
        F: FnOnce(K) -> Fut,
        Fut: std::future::Future<Output = Option<V>>,
    {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(value) = cache.get(&key) {
                return Some(value.clone());
            }
        }

        // Load and cache
        if let Some(value) = loader(key.clone()).await {
            let mut cache = self.cache.write().await;
            cache.insert(key, value.clone());
            Some(value)
        } else {
            None
        }
    }

    /// Load multiple items by keys
    pub async fn load_many<F, Fut>(&self, keys: Vec<K>, loader: F) -> Vec<Option<V>>
    where
        F: FnOnce(Vec<K>) -> Fut,
        Fut: std::future::Future<Output = std::collections::HashMap<K, V>>,
    {
        let mut results = Vec::with_capacity(keys.len());
        let mut missing_keys = Vec::new();
        let mut missing_indices = Vec::new();

        // Check cache
        {
            let cache = self.cache.read().await;
            for (i, key) in keys.iter().enumerate() {
                if let Some(value) = cache.get(key) {
                    results.push(Some(value.clone()));
                } else {
                    results.push(None);
                    missing_keys.push(key.clone());
                    missing_indices.push(i);
                }
            }
        }

        // Load missing
        if !missing_keys.is_empty() {
            let loaded = loader(missing_keys).await;
            let mut cache = self.cache.write().await;

            for idx in missing_indices.into_iter() {
                let key = keys[idx].clone();
                if let Some(value) = loaded.get(&key) {
                    results[idx] = Some(value.clone());
                    cache.insert(key, value.clone());
                }
            }
        }

        results
    }

    /// Clear the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache size
    pub async fn len(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    /// Check if cache is empty
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}

impl<K, V> Default for DataLoader<K, V>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for DataLoader<K, V> {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AUTH GUARD
// ═══════════════════════════════════════════════════════════════════════════

/// Guard for authentication/authorization in resolvers
pub struct AuthGuard;

impl AuthGuard {
    /// Check if user is authenticated
    pub fn require_auth<T>(user: &Option<T>) -> Result<&T, GraphError> {
        user.as_ref().ok_or(GraphError::Unauthenticated)
    }

    /// Check if user has a specific role
    pub fn require_role<'a, T, F>(user: &'a Option<T>, role: &str, get_role: F) -> Result<&'a T, GraphError>
    where
        F: Fn(&T) -> &str,
    {
        let u = user.as_ref().ok_or(GraphError::Unauthenticated)?;
        if get_role(u) == role {
            Ok(u)
        } else {
            Err(GraphError::PermissionDenied(format!(
                "Required role: {}",
                role
            )))
        }
    }

    /// Check if user has one of the specified roles
    pub fn require_any_role<'a, T, F>(
        user: &'a Option<T>,
        roles: &[&str],
        get_role: F,
    ) -> Result<&'a T, GraphError>
    where
        F: Fn(&T) -> &str,
    {
        let u = user.as_ref().ok_or(GraphError::Unauthenticated)?;
        let user_role = get_role(u);
        if roles.contains(&user_role) {
            Ok(u)
        } else {
            Err(GraphError::PermissionDenied(format!(
                "Required roles: {:?}",
                roles
            )))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PAGINATION
// ═══════════════════════════════════════════════════════════════════════════

/// Cursor-based pagination input
#[derive(Debug, Clone, Serialize, Deserialize, Default, InputObject)]
pub struct PaginationInput {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub last: Option<i32>,
    pub before: Option<String>,
}

impl PaginationInput {
    pub fn limit(&self) -> i32 {
        self.first.or(self.last).unwrap_or(10).min(100)
    }
}

/// Connection for cursor-based pagination
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Connection<T: async_graphql::OutputType> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
    pub total_count: i32,
}

/// Edge in a connection
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Edge<T: async_graphql::OutputType> {
    pub node: T,
    pub cursor: String,
}

/// Page info for cursor-based pagination
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

impl<T: async_graphql::OutputType> Connection<T> {
    pub fn new(nodes: Vec<T>, page_info: PageInfo, total_count: i32) -> Self
    where
        T: Clone,
    {
        let edges = nodes
            .into_iter()
            .enumerate()
            .map(|(i, node)| Edge {
                cursor: base64_encode(&format!("cursor:{}", i)),
                node,
            })
            .collect();

        Self {
            edges,
            page_info,
            total_count,
        }
    }
}

fn base64_encode(s: &str) -> String {
    // Simple base64-like encoding for cursors
    s.as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
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
    fn test_error_query() {
        let err = GraphError::QueryError("syntax error".to_string());
        assert!(err.to_string().contains("Query"));
    }

    #[test]
    fn test_error_schema() {
        let err = GraphError::SchemaBuildError("invalid type".to_string());
        assert!(err.to_string().contains("Schema"));
    }

    #[test]
    fn test_error_validation() {
        let err = GraphError::ValidationError("required field".to_string());
        assert!(err.to_string().contains("Validation"));
    }

    #[test]
    fn test_error_unauthenticated() {
        let err = GraphError::Unauthenticated;
        assert!(err.to_string().contains("Authentication"));
    }

    #[test]
    fn test_error_permission() {
        let err = GraphError::PermissionDenied("admin only".to_string());
        assert!(err.to_string().contains("Permission"));
    }

    #[test]
    fn test_error_not_found() {
        let err = GraphError::NotFound("user".to_string());
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_error_rate_limited() {
        let err = GraphError::RateLimited;
        assert!(err.to_string().contains("Rate limited"));
    }

    #[test]
    fn test_error_internal() {
        let err = GraphError::InternalError("db error".to_string());
        assert!(err.to_string().contains("Internal"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // REQUEST/RESPONSE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_request_new() {
        let req = GraphQLRequest::new("{ users { id } }");
        assert_eq!(req.query, "{ users { id } }");
        assert!(req.operation_name.is_none());
        assert!(req.variables.is_none());
    }

    #[test]
    fn test_request_builder() {
        let req = GraphQLRequest::new("query GetUser($id: ID!) { user(id: $id) { name } }")
            .operation("GetUser")
            .variables(serde_json::json!({"id": "1"}));

        assert_eq!(req.operation_name, Some("GetUser".to_string()));
        assert!(req.variables.is_some());
    }

    #[test]
    fn test_response_data() {
        let response = GraphQLResponse::data(serde_json::json!({"user": {"id": 1}}));
        assert!(response.is_success());
        assert!(response.data.is_some());
    }

    #[test]
    fn test_response_error() {
        let response = GraphQLResponse::error(GraphQLError::new("Not found"));
        assert!(!response.is_success());
        assert_eq!(response.errors.len(), 1);
    }

    #[test]
    fn test_graphql_error() {
        let err = GraphQLError::new("Something went wrong");
        assert_eq!(err.message, "Something went wrong");
    }

    #[test]
    fn test_graphql_error_with_code() {
        let err = GraphQLError::new("Unauthorized").with_code("UNAUTHENTICATED");
        assert!(err.extensions.is_some());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PLAYGROUND TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_playground_html() {
        let html = GraphQL::playground_html("/graphql");
        assert!(html.contains("GraphQL Playground"));
        assert!(html.contains("/graphql"));
    }

    #[test]
    fn test_graphiql_html() {
        let html = GraphQL::graphiql_html("/api/graphql");
        assert!(html.contains("GraphiQL"));
        assert!(html.contains("/api/graphql"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DATALOADER TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_dataloader_new() {
        let loader: DataLoader<i32, String> = DataLoader::new();
        assert!(loader.is_empty().await);
    }

    #[tokio::test]
    async fn test_dataloader_load() {
        let loader: DataLoader<i32, String> = DataLoader::new();

        let result = loader.load(1, |id| async move { Some(format!("user_{}", id)) }).await;
        assert_eq!(result, Some("user_1".to_string()));
        assert_eq!(loader.len().await, 1);
    }

    #[tokio::test]
    async fn test_dataloader_cache() {
        let loader: DataLoader<i32, String> = DataLoader::new();

        // First load
        let _ = loader.load(1, |id| async move { Some(format!("user_{}", id)) }).await;

        // Second load should use cache (loader not called)
        let result = loader.load(1, |_| async move { None }).await;
        assert_eq!(result, Some("user_1".to_string()));
    }

    #[tokio::test]
    async fn test_dataloader_clear() {
        let loader: DataLoader<i32, String> = DataLoader::new();
        let _ = loader.load(1, |id| async move { Some(format!("user_{}", id)) }).await;

        assert!(!loader.is_empty().await);
        loader.clear().await;
        assert!(loader.is_empty().await);
    }

    #[tokio::test]
    async fn test_dataloader_clone() {
        let loader1: DataLoader<i32, String> = DataLoader::new();
        let loader2 = loader1.clone();

        let _ = loader1.load(1, |id| async move { Some(format!("user_{}", id)) }).await;

        // Cloned loader should see the same cache
        assert_eq!(loader2.len().await, 1);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // AUTH GUARD TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_require_auth_success() {
        let user = Some("user_id".to_string());
        let result = AuthGuard::require_auth(&user);
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_auth_failure() {
        let user: Option<String> = None;
        let result = AuthGuard::require_auth(&user);
        assert!(matches!(result, Err(GraphError::Unauthenticated)));
    }

    #[derive(Debug)]
    struct TestUser {
        role: String,
    }

    #[test]
    fn test_require_role_success() {
        let user = Some(TestUser {
            role: "admin".to_string(),
        });
        let result = AuthGuard::require_role(&user, "admin", |u| &u.role);
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_role_wrong_role() {
        let user = Some(TestUser {
            role: "user".to_string(),
        });
        let result = AuthGuard::require_role(&user, "admin", |u| &u.role);
        assert!(matches!(result, Err(GraphError::PermissionDenied(_))));
    }

    #[test]
    fn test_require_any_role_success() {
        let user = Some(TestUser {
            role: "editor".to_string(),
        });
        let result = AuthGuard::require_any_role(&user, &["admin", "editor"], |u| &u.role);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PAGINATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_pagination_default() {
        let input = PaginationInput::default();
        assert_eq!(input.limit(), 10);
    }

    #[test]
    fn test_pagination_first() {
        let input = PaginationInput {
            first: Some(25),
            ..Default::default()
        };
        assert_eq!(input.limit(), 25);
    }

    #[test]
    fn test_pagination_max_limit() {
        let input = PaginationInput {
            first: Some(999),
            ..Default::default()
        };
        assert_eq!(input.limit(), 100);
    }

    #[test]
    fn test_page_info() {
        let info = PageInfo {
            has_next_page: true,
            has_previous_page: false,
            start_cursor: Some("abc".to_string()),
            end_cursor: Some("xyz".to_string()),
        };
        assert!(info.has_next_page);
        assert!(!info.has_previous_page);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SERIALIZATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_request_serialization() {
        let req = GraphQLRequest::new("{ users }");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"query\":\"{ users }\""));
    }

    #[test]
    fn test_response_serialization() {
        let resp = GraphQLResponse::data(serde_json::json!({"id": 1}));
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"data\""));
        assert!(!json.contains("\"errors\"")); // Empty arrays should be skipped
    }

    #[test]
    fn test_error_serialization() {
        let err = GraphQLError::new("Test error");
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("\"message\":\"Test error\""));
    }
}
