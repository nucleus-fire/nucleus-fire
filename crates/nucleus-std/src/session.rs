//! Nucleus Session Management
//!
//! Cookie-based session management with:
//! - Secure session storage (memory or database)
//! - Flash messages (one-time notifications)
//! - CSRF protection
//! - Authentication helpers
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::session::{Session, MemorySessionStore, SessionConfig};
//!
//! // In your middleware
//! let store = MemorySessionStore::new();
//! let config = SessionConfig::default();
//!
//! // In an action
//! session.set("user_id", user.id);
//! session.flash("success", "Welcome back!");
//!
//! let user_id: Option<String> = session.get("user_id");
//! let message = session.get_flash("success"); // Consumed after reading
//! ```

use chrono::{DateTime, Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Session error types
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    NotFound(String),

    #[error("Session expired")]
    Expired,

    #[error("Invalid CSRF token")]
    InvalidCsrfToken,

    #[error("Authentication required")]
    Unauthenticated,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Session storage error: {0}")]
    StorageError(String),

    #[error("Session deserialization failed: {0}")]
    DeserializationError(String),

    #[error("Session cookie invalid")]
    InvalidCookie,
}

impl From<serde_json::Error> for SessionError {
    fn from(err: serde_json::Error) -> Self {
        SessionError::DeserializationError(err.to_string())
    }
}

// HTTP response implementation for axum
impl axum::response::IntoResponse for SessionError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        use axum::Json;
        use serde_json::json;

        let (status, msg) = match &self {
            SessionError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            SessionError::Expired => (StatusCode::UNAUTHORIZED, self.to_string()),
            SessionError::InvalidCsrfToken => {
                (StatusCode::FORBIDDEN, "Invalid CSRF token".to_string())
            }
            SessionError::Unauthenticated => (
                StatusCode::UNAUTHORIZED,
                "Authentication required".to_string(),
            ),
            SessionError::PermissionDenied(m) => (StatusCode::FORBIDDEN, m.clone()),
            SessionError::StorageError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Session storage error".to_string(),
            ),
            SessionError::DeserializationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Session error".to_string(),
            ),
            SessionError::InvalidCookie => (
                StatusCode::BAD_REQUEST,
                "Invalid session cookie".to_string(),
            ),
        };

        (status, Json(json!({ "error": msg }))).into_response()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SESSION DATA
// ═══════════════════════════════════════════════════════════════════════════

/// Session data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// Session ID
    pub id: String,
    /// User data
    pub data: HashMap<String, serde_json::Value>,
    /// Flash messages (consumed on read)
    pub flash: HashMap<String, String>,
    /// CSRF token
    pub csrf_token: String,
    /// Authenticated user ID
    pub user_id: Option<String>,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Expiration time
    pub expires_at: DateTime<Utc>,
}

impl SessionData {
    fn new(ttl: Duration) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            data: HashMap::new(),
            flash: HashMap::new(),
            csrf_token: Uuid::new_v4().to_string(),
            user_id: None,
            created_at: now,
            expires_at: now + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    fn regenerate_id(&mut self) {
        self.id = Uuid::new_v4().to_string();
    }

    fn regenerate_csrf(&mut self) {
        self.csrf_token = Uuid::new_v4().to_string();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SESSION STORE TRAIT
// ═══════════════════════════════════════════════════════════════════════════

/// Session storage backend trait
#[async_trait::async_trait]
pub trait SessionStore: Send + Sync {
    async fn get(&self, id: &str) -> Option<SessionData>;
    async fn set(&self, id: &str, data: SessionData);
    async fn delete(&self, id: &str);
    async fn cleanup_expired(&self) -> usize;
}

// ═══════════════════════════════════════════════════════════════════════════
// MEMORY SESSION STORE
// ═══════════════════════════════════════════════════════════════════════════

/// In-memory session store
pub struct MemorySessionStore {
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
}

impl MemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MemorySessionStore {
    fn clone(&self) -> Self {
        Self {
            sessions: Arc::clone(&self.sessions),
        }
    }
}

#[async_trait::async_trait]
impl SessionStore for MemorySessionStore {
    async fn get(&self, id: &str) -> Option<SessionData> {
        let sessions = self.sessions.read().await;
        sessions.get(id).filter(|s| !s.is_expired()).cloned()
    }

    async fn set(&self, id: &str, data: SessionData) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(id.to_string(), data);
    }

    async fn delete(&self, id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(id);
    }

    async fn cleanup_expired(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let before = sessions.len();
        sessions.retain(|_, s| !s.is_expired());
        before - sessions.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SESSION CONFIG
// ═══════════════════════════════════════════════════════════════════════════

/// Cookie same-site policy
#[derive(Debug, Clone, Copy, Default)]
pub enum SameSite {
    Strict,
    #[default]
    Lax,
    None,
}

/// Session configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Cookie name
    pub cookie_name: String,
    /// Cookie path
    pub cookie_path: String,
    /// Secure cookie (HTTPS only)
    pub cookie_secure: bool,
    /// HTTP only (not accessible via JavaScript)
    pub cookie_http_only: bool,
    /// Same-site policy
    pub cookie_same_site: SameSite,
    /// Session TTL
    pub ttl: Duration,
    /// Regenerate session ID on login
    pub regenerate_on_login: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: "nucleus_session".to_string(),
            cookie_path: "/".to_string(),
            cookie_secure: true,
            cookie_http_only: true,
            cookie_same_site: SameSite::Lax,
            ttl: Duration::hours(24),
            regenerate_on_login: true,
        }
    }
}

impl SessionConfig {
    /// Development config (less strict for local testing)
    pub fn development() -> Self {
        Self {
            cookie_secure: false,
            ..Default::default()
        }
    }

    /// Production config
    pub fn production() -> Self {
        Self {
            cookie_secure: true,
            cookie_same_site: SameSite::Strict,
            ..Default::default()
        }
    }

    /// Set TTL in hours
    pub fn ttl_hours(mut self, hours: i64) -> Self {
        self.ttl = Duration::hours(hours);
        self
    }

    /// Set cookie name
    pub fn cookie_name(mut self, name: &str) -> Self {
        self.cookie_name = name.to_string();
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SESSION
// ═══════════════════════════════════════════════════════════════════════════

/// Session handle for request processing
pub struct Session {
    data: SessionData,
    modified: bool,
    destroyed: bool,
}

impl Session {
    /// Create a new session
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: SessionData::new(ttl),
            modified: true,
            destroyed: false,
        }
    }

    /// Load from existing session data
    pub fn from_data(data: SessionData) -> Self {
        Self {
            data,
            modified: false,
            destroyed: false,
        }
    }

    /// Get session ID
    pub fn id(&self) -> &str {
        &self.data.id
    }

    /// Get a value from the session
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .data
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Set a value in the session
    pub fn set<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.data.data.insert(key.to_string(), json_value);
            self.modified = true;
        }
    }

    /// Remove a key from the session
    pub fn remove(&mut self, key: &str) -> bool {
        let removed = self.data.data.remove(key).is_some();
        if removed {
            self.modified = true;
        }
        removed
    }

    /// Check if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.data.data.contains_key(key)
    }

    /// Clear all session data
    pub fn clear(&mut self) {
        self.data.data.clear();
        self.modified = true;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FLASH MESSAGES
    // ═══════════════════════════════════════════════════════════════════════

    /// Set a flash message (one-time notification)
    pub fn flash(&mut self, key: &str, message: &str) {
        self.data.flash.insert(key.to_string(), message.to_string());
        self.modified = true;
    }

    /// Get and consume a flash message
    pub fn get_flash(&mut self, key: &str) -> Option<String> {
        let msg = self.data.flash.remove(key);
        if msg.is_some() {
            self.modified = true;
        }
        msg
    }

    /// Check if flash message exists (without consuming)
    pub fn has_flash(&self, key: &str) -> bool {
        self.data.flash.contains_key(key)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CSRF PROTECTION
    // ═══════════════════════════════════════════════════════════════════════

    /// Get CSRF token
    pub fn csrf_token(&self) -> &str {
        &self.data.csrf_token
    }

    /// Verify CSRF token
    pub fn verify_csrf(&self, token: &str) -> bool {
        // Constant-time comparison to prevent timing attacks
        let a = self.data.csrf_token.as_bytes();
        let b = token.as_bytes();

        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }

    /// Regenerate CSRF token
    pub fn regenerate_csrf(&mut self) {
        self.data.regenerate_csrf();
        self.modified = true;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // AUTHENTICATION
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.data.user_id.is_some()
    }

    /// Get authenticated user ID
    pub fn user_id(&self) -> Option<&str> {
        self.data.user_id.as_deref()
    }

    /// Log in user (sets user_id and optionally regenerates session)
    pub fn login(&mut self, user_id: &str, regenerate: bool) {
        if regenerate {
            self.regenerate();
        }
        self.data.user_id = Some(user_id.to_string());
        self.modified = true;
    }

    /// Log out user
    pub fn logout(&mut self) {
        self.data.user_id = None;
        self.regenerate();
        self.modified = true;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SESSION LIFECYCLE
    // ═══════════════════════════════════════════════════════════════════════

    /// Regenerate session ID (prevents session fixation attacks)
    pub fn regenerate(&mut self) {
        self.data.regenerate_id();
        self.modified = true;
    }

    /// Destroy the session
    pub fn destroy(&mut self) {
        self.destroyed = true;
        self.modified = true;
    }

    /// Check if session is destroyed
    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    /// Check if session was modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get underlying data
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Consume and get underlying data
    pub fn into_data(self) -> SessionData {
        self.data
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SESSION MANAGER
// ═══════════════════════════════════════════════════════════════════════════

/// Session manager for handling session lifecycle
pub struct SessionManager<S: SessionStore> {
    store: S,
    config: SessionConfig,
}

impl<S: SessionStore> SessionManager<S> {
    pub fn new(store: S, config: SessionConfig) -> Self {
        Self { store, config }
    }

    /// Start a new session
    pub fn start(&self) -> Session {
        Session::new(self.config.ttl)
    }

    /// Load session by ID
    pub async fn load(&self, session_id: &str) -> Option<Session> {
        self.store.get(session_id).await.map(Session::from_data)
    }

    /// Save session
    pub async fn save(&self, session: &Session) {
        if session.is_destroyed() {
            self.store.delete(session.id()).await;
        } else if session.is_modified() {
            self.store.set(session.id(), session.data().clone()).await;
        }
    }

    /// Cleanup expired sessions
    pub async fn cleanup(&self) -> usize {
        self.store.cleanup_expired().await
    }

    /// Get config
    pub fn config(&self) -> &SessionConfig {
        &self.config
    }
}

impl<S: SessionStore + Clone> Clone for SessionManager<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            config: self.config.clone(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // BASIC OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_session_create() {
        let session = Session::new(Duration::hours(1));
        assert!(!session.id().is_empty());
        assert!(!session.is_authenticated());
    }

    #[test]
    fn test_session_get_set() {
        let mut session = Session::new(Duration::hours(1));

        session.set("key", "value");
        let value: Option<String> = session.get("key");
        assert_eq!(value, Some("value".to_string()));
    }

    #[test]
    fn test_session_remove() {
        let mut session = Session::new(Duration::hours(1));

        session.set("key", "value");
        assert!(session.has("key"));

        session.remove("key");
        assert!(!session.has("key"));
    }

    #[test]
    fn test_session_clear() {
        let mut session = Session::new(Duration::hours(1));

        session.set("key1", "value1");
        session.set("key2", "value2");
        session.clear();

        assert!(!session.has("key1"));
        assert!(!session.has("key2"));
    }

    #[test]
    fn test_session_empty_key() {
        let mut session = Session::new(Duration::hours(1));

        session.set("", "empty_key_value");
        let value: Option<String> = session.get("");
        assert_eq!(value, Some("empty_key_value".to_string()));
    }

    #[test]
    fn test_session_unicode_key() {
        let mut session = Session::new(Duration::hours(1));

        session.set("用户", "chinese");
        session.set("🔑", "emoji");

        assert_eq!(session.get::<String>("用户"), Some("chinese".to_string()));
        assert_eq!(session.get::<String>("🔑"), Some("emoji".to_string()));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DATA TYPES
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_session_string() {
        let mut session = Session::new(Duration::hours(1));
        session.set("string", "hello world");
        assert_eq!(
            session.get::<String>("string"),
            Some("hello world".to_string())
        );
    }

    #[test]
    fn test_session_number() {
        let mut session = Session::new(Duration::hours(1));

        session.set("int", 42i32);
        session.set("float", 3.14f64);

        assert_eq!(session.get::<i32>("int"), Some(42));
        assert_eq!(session.get::<f64>("float"), Some(3.14));
    }

    #[test]
    fn test_session_bool() {
        let mut session = Session::new(Duration::hours(1));

        session.set("yes", true);
        session.set("no", false);

        assert_eq!(session.get::<bool>("yes"), Some(true));
        assert_eq!(session.get::<bool>("no"), Some(false));
    }

    #[test]
    fn test_session_vec() {
        let mut session = Session::new(Duration::hours(1));

        session.set("items", vec![1, 2, 3]);
        let items: Option<Vec<i32>> = session.get("items");
        assert_eq!(items, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_session_struct() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct User {
            id: i32,
            name: String,
        }

        let mut session = Session::new(Duration::hours(1));
        let user = User {
            id: 1,
            name: "Alice".into(),
        };

        session.set("user", user.clone());
        let retrieved: Option<User> = session.get("user");
        assert_eq!(retrieved, Some(user));
    }

    #[test]
    fn test_session_nested() {
        let mut session = Session::new(Duration::hours(1));

        let nested = serde_json::json!({
            "level1": {
                "level2": {
                    "value": 42
                }
            }
        });

        session.set("nested", nested.clone());
        let retrieved: Option<serde_json::Value> = session.get("nested");
        assert_eq!(retrieved, Some(nested));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FLASH MESSAGES
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_flash_set_get() {
        let mut session = Session::new(Duration::hours(1));

        session.flash("success", "Welcome!");
        let msg = session.get_flash("success");
        assert_eq!(msg, Some("Welcome!".to_string()));
    }

    #[test]
    fn test_flash_consumed() {
        let mut session = Session::new(Duration::hours(1));

        session.flash("msg", "Hello");

        // First read returns the message
        assert_eq!(session.get_flash("msg"), Some("Hello".to_string()));

        // Second read returns None (consumed)
        assert_eq!(session.get_flash("msg"), None);
    }

    #[test]
    fn test_flash_multiple() {
        let mut session = Session::new(Duration::hours(1));

        session.flash("success", "Saved!");
        session.flash("info", "Note this.");
        session.flash("error", "Oops!");

        assert_eq!(session.get_flash("success"), Some("Saved!".to_string()));
        assert_eq!(session.get_flash("info"), Some("Note this.".to_string()));
        assert_eq!(session.get_flash("error"), Some("Oops!".to_string()));
    }

    #[test]
    fn test_has_flash() {
        let mut session = Session::new(Duration::hours(1));

        session.flash("key", "value");

        assert!(session.has_flash("key"));
        assert!(!session.has_flash("other"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // AUTHENTICATION
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_login_sets_user() {
        let mut session = Session::new(Duration::hours(1));

        session.login("user_123", false);

        assert!(session.is_authenticated());
        assert_eq!(session.user_id(), Some("user_123"));
    }

    #[test]
    fn test_logout_clears_user() {
        let mut session = Session::new(Duration::hours(1));

        session.login("user_123", false);
        session.logout();

        assert!(!session.is_authenticated());
        assert_eq!(session.user_id(), None);
    }

    #[test]
    fn test_is_authenticated() {
        let mut session = Session::new(Duration::hours(1));

        assert!(!session.is_authenticated());
        session.login("user", false);
        assert!(session.is_authenticated());
    }

    #[test]
    fn test_regenerate_on_login() {
        let mut session = Session::new(Duration::hours(1));
        let original_id = session.id().to_string();

        session.login("user_123", true);

        // Session ID should have changed
        assert_ne!(session.id(), original_id);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CSRF PROTECTION
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_csrf_token_generated() {
        let session = Session::new(Duration::hours(1));

        let token = session.csrf_token();
        assert!(!token.is_empty());
        assert_eq!(token.len(), 36); // UUID format
    }

    #[test]
    fn test_csrf_verify_valid() {
        let session = Session::new(Duration::hours(1));
        let token = session.csrf_token().to_string();

        assert!(session.verify_csrf(&token));
    }

    #[test]
    fn test_csrf_verify_invalid() {
        let session = Session::new(Duration::hours(1));

        assert!(!session.verify_csrf("wrong_token"));
        assert!(!session.verify_csrf(""));
    }

    #[test]
    fn test_csrf_token_stable() {
        let session = Session::new(Duration::hours(1));

        let token1 = session.csrf_token().to_string();
        let token2 = session.csrf_token().to_string();

        assert_eq!(token1, token2);
    }

    #[test]
    fn test_csrf_regenerate() {
        let mut session = Session::new(Duration::hours(1));
        let original = session.csrf_token().to_string();

        session.regenerate_csrf();

        assert_ne!(session.csrf_token(), original);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SESSION LIFECYCLE
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_session_regenerate() {
        let mut session = Session::new(Duration::hours(1));
        session.set("key", "value");
        let original_id = session.id().to_string();

        session.regenerate();

        // ID changed
        assert_ne!(session.id(), original_id);
        // Data preserved
        assert_eq!(session.get::<String>("key"), Some("value".to_string()));
    }

    #[test]
    fn test_session_destroy() {
        let mut session = Session::new(Duration::hours(1));

        assert!(!session.is_destroyed());
        session.destroy();
        assert!(session.is_destroyed());
    }

    #[test]
    fn test_session_modified() {
        let session = Session::new(Duration::hours(1));

        // New session is modified
        assert!(session.is_modified());

        // Load from data is not modified
        let session2 = Session::from_data(session.into_data());
        assert!(!session2.is_modified());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SESSION STORE
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_memory_store_set_get() {
        let store = MemorySessionStore::new();
        let session = Session::new(Duration::hours(1));

        store.set(session.id(), session.data().clone()).await;

        let loaded = store.get(session.id()).await;
        assert!(loaded.is_some());
    }

    #[tokio::test]
    async fn test_memory_store_delete() {
        let store = MemorySessionStore::new();
        let session = Session::new(Duration::hours(1));

        store.set(session.id(), session.data().clone()).await;
        store.delete(session.id()).await;

        assert!(store.get(session.id()).await.is_none());
    }

    #[tokio::test]
    async fn test_memory_store_cleanup() {
        let store = MemorySessionStore::new();

        // Create expired session
        let mut data = SessionData::new(Duration::hours(-1));
        data.expires_at = Utc::now() - Duration::hours(1);
        let expired_id = data.id.clone();
        store.set(&expired_id, data).await;

        // Create valid session
        let valid = SessionData::new(Duration::hours(1));
        let valid_id = valid.id.clone();
        store.set(&valid_id, valid).await;

        let cleaned = store.cleanup_expired().await;
        assert_eq!(cleaned, 1);

        // Valid session still exists
        assert!(store.get(&valid_id).await.is_some());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CONFIG
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_config_default() {
        let config = SessionConfig::default();

        assert_eq!(config.cookie_name, "nucleus_session");
        assert!(config.cookie_http_only);
        assert!(config.cookie_secure);
    }

    #[test]
    fn test_config_development() {
        let config = SessionConfig::development();

        assert!(!config.cookie_secure);
    }

    #[test]
    fn test_config_production() {
        let config = SessionConfig::production();

        assert!(config.cookie_secure);
        assert!(matches!(config.cookie_same_site, SameSite::Strict));
    }

    #[test]
    fn test_config_builder() {
        let config = SessionConfig::default()
            .ttl_hours(48)
            .cookie_name("my_session");

        assert_eq!(config.ttl, Duration::hours(48));
        assert_eq!(config.cookie_name, "my_session");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SESSION MANAGER
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_session_manager_start() {
        let manager = SessionManager::new(MemorySessionStore::new(), SessionConfig::default());

        let session = manager.start();
        assert!(!session.id().is_empty());
    }

    #[tokio::test]
    async fn test_session_manager_save_load() {
        let manager = SessionManager::new(MemorySessionStore::new(), SessionConfig::default());

        let mut session = manager.start();
        session.set("key", "value");
        let id = session.id().to_string();

        manager.save(&session).await;

        let loaded = manager.load(&id).await.unwrap();
        assert_eq!(loaded.get::<String>("key"), Some("value".to_string()));
    }

    #[tokio::test]
    async fn test_session_manager_destroy() {
        let manager = SessionManager::new(MemorySessionStore::new(), SessionConfig::default());

        let mut session = manager.start();
        let id = session.id().to_string();

        manager.save(&session).await;

        session.destroy();
        manager.save(&session).await;

        assert!(manager.load(&id).await.is_none());
    }
}
