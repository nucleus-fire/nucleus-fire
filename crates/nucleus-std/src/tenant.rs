//! Nucleus Tenant - Multi-tenancy Support
//!
//! Row-level security pattern for SaaS applications with:
//! - Multiple tenant resolution strategies (subdomain, header, path)
//! - Thread-local tenant context
//! - Automatic query scoping helpers
//! - Middleware integration
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::tenant::{Tenant, TenantExtractor, TenantStrategy};
//!
//! // Configure tenant extraction
//! let extractor = TenantExtractor::new(TenantStrategy::Subdomain);
//!
//! // In middleware or handler
//! let tenant_id = extractor.extract(&request)?;
//! Tenant::set(&tenant_id);
//!
//! // All subsequent queries are scoped to this tenant
//! let users = User::find_all().await?; // Automatically filtered by tenant_id
//!
//! // Clear when request ends
//! Tenant::clear();
//! ```

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════════
// TENANT CONTEXT
// ═══════════════════════════════════════════════════════════════════════════

thread_local! {
    static CURRENT_TENANT: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Thread-local tenant context for row-level security
#[derive(Debug, Clone)]
pub struct Tenant;

impl Tenant {
    /// Set the current tenant for this thread/task
    pub fn set(tenant_id: &str) {
        CURRENT_TENANT.with(|t| {
            *t.borrow_mut() = Some(tenant_id.to_string());
        });
    }

    /// Get the current tenant ID
    pub fn get() -> Option<String> {
        CURRENT_TENANT.with(|t| t.borrow().clone())
    }

    /// Get the current tenant ID, returning error if not set
    pub fn require() -> Result<String, TenantError> {
        Self::get().ok_or(TenantError::NotSet)
    }

    /// Clear the current tenant (call at end of request)
    pub fn clear() {
        CURRENT_TENANT.with(|t| {
            *t.borrow_mut() = None;
        });
    }

    /// Execute a closure with a specific tenant context
    pub fn with<F, R>(tenant_id: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let previous = Self::get();
        Self::set(tenant_id);
        let result = f();
        match previous {
            Some(id) => Self::set(&id),
            None => Self::clear(),
        }
        result
    }

    /// Check if a tenant is currently set
    pub fn is_set() -> bool {
        Self::get().is_some()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TENANT STRATEGIES
// ═══════════════════════════════════════════════════════════════════════════

/// Strategy for extracting tenant ID from requests
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenantStrategy {
    /// Extract from subdomain: `tenant1.example.com` → `tenant1`
    Subdomain,
    
    /// Extract from header: `X-Tenant-ID: tenant1`
    Header(String),
    
    /// Extract from path prefix: `/tenant1/api/users` → `tenant1`
    PathPrefix,
    
    /// Extract from query param: `?tenant=tenant1`
    QueryParam(String),
    
    /// Fixed tenant (for single-tenant mode or testing)
    Fixed(String),
    
    /// Try multiple strategies in order
    Chain(Vec<TenantStrategy>),
}

impl Default for TenantStrategy {
    fn default() -> Self {
        TenantStrategy::Header("X-Tenant-ID".to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TENANT EXTRACTOR
// ═══════════════════════════════════════════════════════════════════════════

/// Extracts tenant ID from HTTP requests
#[derive(Debug, Clone)]
pub struct TenantExtractor {
    strategy: TenantStrategy,
    /// Base domain for subdomain extraction (e.g., "example.com")
    base_domain: Option<String>,
    /// Whether tenant is required (return error if not found)
    required: bool,
}

impl Default for TenantExtractor {
    fn default() -> Self {
        Self {
            strategy: TenantStrategy::default(),
            base_domain: None,
            required: true,
        }
    }
}

impl TenantExtractor {
    /// Create a new extractor with the given strategy
    pub fn new(strategy: TenantStrategy) -> Self {
        Self {
            strategy,
            base_domain: None,
            required: true,
        }
    }

    /// Create subdomain-based extractor
    pub fn subdomain(base_domain: &str) -> Self {
        Self {
            strategy: TenantStrategy::Subdomain,
            base_domain: Some(base_domain.to_string()),
            required: true,
        }
    }

    /// Create header-based extractor
    pub fn header(header_name: &str) -> Self {
        Self {
            strategy: TenantStrategy::Header(header_name.to_string()),
            base_domain: None,
            required: true,
        }
    }

    /// Create path-prefix extractor
    pub fn path_prefix() -> Self {
        Self {
            strategy: TenantStrategy::PathPrefix,
            base_domain: None,
            required: true,
        }
    }

    /// Set whether tenant is required
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Extract tenant from request
    pub fn extract(&self, headers: &HeaderMap, uri: &str, host: Option<&str>) -> Result<Option<String>, TenantError> {
        let result = self.extract_strategy(&self.strategy, headers, uri, host);
        
        match (result, self.required) {
            (Ok(Some(tenant)), _) => Ok(Some(tenant)),
            (Ok(None), true) => Err(TenantError::NotFound),
            (Ok(None), false) => Ok(None),
            (Err(e), _) => Err(e),
        }
    }

    fn extract_strategy(
        &self,
        strategy: &TenantStrategy,
        headers: &HeaderMap,
        uri: &str,
        host: Option<&str>,
    ) -> Result<Option<String>, TenantError> {
        match strategy {
            TenantStrategy::Subdomain => {
                self.extract_subdomain(host)
            }
            TenantStrategy::Header(name) => {
                self.extract_header(headers, name)
            }
            TenantStrategy::PathPrefix => {
                self.extract_path_prefix(uri)
            }
            TenantStrategy::QueryParam(param) => {
                self.extract_query_param(uri, param)
            }
            TenantStrategy::Fixed(tenant) => {
                Ok(Some(tenant.clone()))
            }
            TenantStrategy::Chain(strategies) => {
                for s in strategies {
                    if let Ok(Some(tenant)) = self.extract_strategy(s, headers, uri, host) {
                        return Ok(Some(tenant));
                    }
                }
                Ok(None)
            }
        }
    }

    fn extract_subdomain(&self, host: Option<&str>) -> Result<Option<String>, TenantError> {
        let host = match host {
            Some(h) => h,
            None => return Ok(None),
        };

        // Remove port if present
        let host = host.split(':').next().unwrap_or(host);

        let base = match &self.base_domain {
            Some(b) => b.as_str(),
            None => return Err(TenantError::InvalidConfig("base_domain required for subdomain strategy".into())),
        };

        // Check if host ends with base domain
        if !host.ends_with(base) {
            return Ok(None);
        }

        // Extract subdomain
        let subdomain = host.strip_suffix(base)
            .and_then(|s| s.strip_suffix('.'))
            .filter(|s| !s.is_empty() && !s.contains('.'));

        Ok(subdomain.map(|s| s.to_string()))
    }

    fn extract_header(&self, headers: &HeaderMap, name: &str) -> Result<Option<String>, TenantError> {
        Ok(headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()))
    }

    fn extract_path_prefix(&self, uri: &str) -> Result<Option<String>, TenantError> {
        // Parse path: /tenant-id/rest/of/path
        let path = uri.split('?').next().unwrap_or(uri);
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        
        Ok(segments.first().map(|s| s.to_string()))
    }

    fn extract_query_param(&self, uri: &str, param: &str) -> Result<Option<String>, TenantError> {
        let query = uri.split('?').nth(1).unwrap_or("");
        
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                if key == param && !value.is_empty() {
                    return Ok(Some(value.to_string()));
                }
            }
        }
        
        Ok(None)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TENANT INFO
// ═══════════════════════════════════════════════════════════════════════════

/// Tenant information (for admin/management features)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: i64,
    pub settings: serde_json::Value,
    pub active: bool,
}

impl TenantInfo {
    /// Create a new tenant info
    pub fn new(id: &str, name: &str) -> Self {
        let slug = name.to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        
        Self {
            id: id.to_string(),
            name: name.to_string(),
            slug,
            created_at: chrono::Utc::now().timestamp(),
            settings: serde_json::json!({}),
            active: true,
        }
    }

    /// Check if tenant is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Tenant-related errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenantError {
    /// Tenant not set in context
    NotSet,
    /// Tenant not found in request
    NotFound,
    /// Invalid tenant ID
    Invalid(String),
    /// Tenant is inactive/suspended
    Inactive,
    /// Configuration error
    InvalidConfig(String),
    /// Access denied to this tenant
    AccessDenied,
}

impl std::fmt::Display for TenantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantError::NotSet => write!(f, "Tenant not set in context"),
            TenantError::NotFound => write!(f, "Tenant not found in request"),
            TenantError::Invalid(id) => write!(f, "Invalid tenant ID: {}", id),
            TenantError::Inactive => write!(f, "Tenant is inactive or suspended"),
            TenantError::InvalidConfig(msg) => write!(f, "Invalid tenant configuration: {}", msg),
            TenantError::AccessDenied => write!(f, "Access denied to this tenant"),
        }
    }
}

impl std::error::Error for TenantError {}

impl IntoResponse for TenantError {
    fn into_response(self) -> Response {
        let status = match &self {
            TenantError::NotSet | TenantError::NotFound => StatusCode::BAD_REQUEST,
            TenantError::Invalid(_) => StatusCode::BAD_REQUEST,
            TenantError::Inactive => StatusCode::FORBIDDEN,
            TenantError::InvalidConfig(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TenantError::AccessDenied => StatusCode::FORBIDDEN,
        };
        
        (status, self.to_string()).into_response()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SQL HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Helpers for building tenant-scoped SQL queries
pub struct TenantQuery;

impl TenantQuery {
    /// Get WHERE clause for current tenant
    /// Returns `"tenant_id = ?"` if tenant is set, empty string otherwise
    pub fn where_clause() -> String {
        if Tenant::is_set() {
            "tenant_id = ?".to_string()
        } else {
            String::new()
        }
    }

    /// Get the current tenant ID for binding
    pub fn bind_value() -> Option<String> {
        Tenant::get()
    }

    /// Build a scoped SELECT query
    pub fn select(table: &str, columns: &str) -> String {
        match Tenant::get() {
            Some(_) => format!("SELECT {} FROM {} WHERE tenant_id = ?", columns, table),
            None => format!("SELECT {} FROM {}", columns, table),
        }
    }

    /// Build a scoped INSERT query (auto-adds tenant_id)
    pub fn insert_columns(columns: &str) -> String {
        if Tenant::is_set() {
            format!("tenant_id, {}", columns)
        } else {
            columns.to_string()
        }
    }

    /// Build a scoped UPDATE query
    pub fn update(table: &str, set_clause: &str) -> String {
        match Tenant::get() {
            Some(_) => format!("UPDATE {} SET {} WHERE tenant_id = ?", table, set_clause),
            None => format!("UPDATE {} SET {}", table, set_clause),
        }
    }

    /// Build a scoped DELETE query
    pub fn delete(table: &str) -> String {
        match Tenant::get() {
            Some(_) => format!("DELETE FROM {} WHERE tenant_id = ?", table),
            None => format!("DELETE FROM {}", table),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GUARD / MIDDLEWARE HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Guard that ensures tenant is set before proceeding
pub struct TenantGuard {
    extractor: Arc<TenantExtractor>,
}

impl TenantGuard {
    /// Create a new tenant guard
    pub fn new(extractor: TenantExtractor) -> Self {
        Self {
            extractor: Arc::new(extractor),
        }
    }

    /// Check request and set tenant context
    pub fn check(&self, headers: &HeaderMap, uri: &str, host: Option<&str>) -> Result<String, TenantError> {
        let tenant = self.extractor.extract(headers, uri, host)?
            .ok_or(TenantError::NotFound)?;
        
        Tenant::set(&tenant);
        Ok(tenant)
    }
}

impl Clone for TenantGuard {
    fn clone(&self) -> Self {
        Self {
            extractor: Arc::clone(&self.extractor),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_tenant_set_get_clear() {
        Tenant::clear();
        assert!(Tenant::get().is_none());
        
        Tenant::set("tenant_123");
        assert_eq!(Tenant::get(), Some("tenant_123".to_string()));
        
        Tenant::clear();
        assert!(Tenant::get().is_none());
    }

    #[test]
    fn test_tenant_require() {
        Tenant::clear();
        assert!(Tenant::require().is_err());
        
        Tenant::set("tenant_456");
        assert_eq!(Tenant::require().unwrap(), "tenant_456");
        Tenant::clear();
    }

    #[test]
    fn test_tenant_with() {
        Tenant::clear();
        
        let result = Tenant::with("temp_tenant", || {
            Tenant::get()
        });
        
        assert_eq!(result, Some("temp_tenant".to_string()));
        assert!(Tenant::get().is_none()); // Restored to none
    }

    #[test]
    fn test_tenant_with_preserves_previous() {
        Tenant::set("original");
        
        Tenant::with("temp", || {
            assert_eq!(Tenant::get(), Some("temp".to_string()));
        });
        
        assert_eq!(Tenant::get(), Some("original".to_string()));
        Tenant::clear();
    }

    #[test]
    fn test_extract_header() {
        let extractor = TenantExtractor::header("X-Tenant-ID");
        let mut headers = HeaderMap::new();
        headers.insert("X-Tenant-ID", HeaderValue::from_static("acme"));
        
        let result = extractor.extract(&headers, "/api/users", None);
        assert_eq!(result.unwrap(), Some("acme".to_string()));
    }

    #[test]
    fn test_extract_header_missing() {
        let extractor = TenantExtractor::header("X-Tenant-ID");
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api/users", None);
        assert!(result.is_err()); // Required by default
    }

    #[test]
    fn test_extract_header_optional() {
        let extractor = TenantExtractor::header("X-Tenant-ID").optional();
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api/users", None);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_extract_subdomain() {
        let extractor = TenantExtractor::subdomain("example.com");
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api", Some("acme.example.com"));
        assert_eq!(result.unwrap(), Some("acme".to_string()));
    }

    #[test]
    fn test_extract_subdomain_no_subdomain() {
        let extractor = TenantExtractor::subdomain("example.com").optional();
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api", Some("example.com"));
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_extract_subdomain_with_port() {
        let extractor = TenantExtractor::subdomain("example.com");
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api", Some("acme.example.com:3000"));
        assert_eq!(result.unwrap(), Some("acme".to_string()));
    }

    #[test]
    fn test_extract_path_prefix() {
        let extractor = TenantExtractor::path_prefix();
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/acme/api/users", None);
        assert_eq!(result.unwrap(), Some("acme".to_string()));
    }

    #[test]
    fn test_extract_query_param() {
        let extractor = TenantExtractor::new(TenantStrategy::QueryParam("org".to_string()));
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api/users?org=acme&page=1", None);
        assert_eq!(result.unwrap(), Some("acme".to_string()));
    }

    #[test]
    fn test_extract_fixed() {
        let extractor = TenantExtractor::new(TenantStrategy::Fixed("default".to_string()));
        let headers = HeaderMap::new();
        
        let result = extractor.extract(&headers, "/api", None);
        assert_eq!(result.unwrap(), Some("default".to_string()));
    }

    #[test]
    fn test_extract_chain() {
        let extractor = TenantExtractor::new(TenantStrategy::Chain(vec![
            TenantStrategy::Header("X-Tenant-ID".to_string()),
            TenantStrategy::QueryParam("tenant".to_string()),
            TenantStrategy::Fixed("default".to_string()),
        ]));
        
        // No header, no query param → falls back to fixed
        let headers = HeaderMap::new();
        let result = extractor.extract(&headers, "/api", None);
        assert_eq!(result.unwrap(), Some("default".to_string()));
        
        // Has header → uses header
        let mut headers = HeaderMap::new();
        headers.insert("X-Tenant-ID", HeaderValue::from_static("from_header"));
        let result = extractor.extract(&headers, "/api", None);
        assert_eq!(result.unwrap(), Some("from_header".to_string()));
    }

    #[test]
    fn test_tenant_info_new() {
        let info = TenantInfo::new("t_123", "Acme Corp");
        assert_eq!(info.id, "t_123");
        assert_eq!(info.name, "Acme Corp");
        assert_eq!(info.slug, "acme-corp");
        assert!(info.active);
    }

    #[test]
    fn test_tenant_query_select() {
        Tenant::clear();
        let sql = TenantQuery::select("users", "*");
        assert_eq!(sql, "SELECT * FROM users");
        
        Tenant::set("t_123");
        let sql = TenantQuery::select("users", "*");
        assert_eq!(sql, "SELECT * FROM users WHERE tenant_id = ?");
        Tenant::clear();
    }

    #[test]
    fn test_tenant_query_delete() {
        Tenant::set("t_123");
        let sql = TenantQuery::delete("users");
        assert_eq!(sql, "DELETE FROM users WHERE tenant_id = ?");
        Tenant::clear();
    }

    #[test]
    fn test_tenant_error_display() {
        assert_eq!(TenantError::NotSet.to_string(), "Tenant not set in context");
        assert_eq!(TenantError::NotFound.to_string(), "Tenant not found in request");
        assert_eq!(TenantError::Invalid("bad".into()).to_string(), "Invalid tenant ID: bad");
    }

    #[test]
    fn test_tenant_guard() {
        Tenant::clear();
        
        let guard = TenantGuard::new(TenantExtractor::header("X-Tenant"));
        let mut headers = HeaderMap::new();
        headers.insert("X-Tenant", HeaderValue::from_static("guarded"));
        
        let result = guard.check(&headers, "/api", None);
        assert_eq!(result.unwrap(), "guarded");
        assert_eq!(Tenant::get(), Some("guarded".to_string()));
        
        Tenant::clear();
    }

    #[test]
    fn test_is_set() {
        Tenant::clear();
        assert!(!Tenant::is_set());
        
        Tenant::set("test");
        assert!(Tenant::is_set());
        
        Tenant::clear();
    }
}
