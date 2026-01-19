//! Nucleus Cache Module
//!
//! Provides in-memory caching with TTL (Time-To-Live) support:
//! - Thread-safe cache operations
//! - Automatic expiration
//! - Pattern-based invalidation
//! - Lazy loading helpers
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::cache::{Cache, cached};
//! use std::time::Duration;
//!
//! let cache = Cache::<String>::new(Duration::from_secs(300)); // 5 min TTL
//!
//! // Direct usage
//! cache.set("key", "value".to_string());
//! let value = cache.get("key");
//!
//! // With lazy loading
//! let result = cached(&cache, "expensive", || async {
//!     expensive_operation().await
//! }).await;
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Cache error types
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Cache lock poisoned")]
    LockPoisoned,

    #[error("Cache connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Cache operation timeout")]
    Timeout,

    #[error("Key expired: {0}")]
    Expired(String),
}

impl From<serde_json::Error> for CacheError {
    fn from(err: serde_json::Error) -> Self {
        CacheError::SerializationError(err.to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CACHE ENTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Cache entry with expiration time
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CACHE
// ═══════════════════════════════════════════════════════════════════════════

/// Thread-safe in-memory cache with TTL support
pub struct Cache<T: Clone> {
    entries: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    default_ttl: Duration,
}

impl<T: Clone> Cache<T> {
    /// Create a new cache with the specified default TTL
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
        }
    }

    /// Create a cache with 5 minute TTL
    pub fn short() -> Self {
        Self::new(Duration::from_secs(300))
    }

    /// Create a cache with 1 hour TTL
    pub fn medium() -> Self {
        Self::new(Duration::from_secs(3600))
    }

    /// Create a cache with 24 hour TTL
    pub fn long() -> Self {
        Self::new(Duration::from_secs(86400))
    }

    /// Get a value from the cache if it exists and hasn't expired
    pub fn get(&self, key: &str) -> Option<T> {
        let entries = self.entries.read().unwrap();
        entries.get(key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(entry.value.clone())
            }
        })
    }

    /// Set a value with the default TTL
    pub fn set(&self, key: &str, value: T) {
        self.set_with_ttl(key, value, self.default_ttl);
    }

    /// Set a value with a custom TTL
    pub fn set_with_ttl(&self, key: &str, value: T, ttl: Duration) {
        let mut entries = self.entries.write().unwrap();
        entries.insert(
            key.to_string(),
            CacheEntry {
                value,
                expires_at: Instant::now() + ttl,
            },
        );
    }

    /// Check if a key exists and hasn't expired
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Delete a specific key
    pub fn delete(&self, key: &str) -> Option<T> {
        let mut entries = self.entries.write().unwrap();
        entries.remove(key).map(|e| e.value)
    }

    /// Delete all keys matching a prefix
    pub fn invalidate_prefix(&self, prefix: &str) {
        let mut entries = self.entries.write().unwrap();
        entries.retain(|key, _| !key.starts_with(prefix));
    }

    /// Delete all keys matching a pattern (simple glob with *)
    pub fn invalidate_pattern(&self, pattern: &str) {
        let mut entries = self.entries.write().unwrap();

        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            entries.retain(|key, _| !match parts.as_slice() {
                [prefix, ""] => key.starts_with(prefix),
                ["", suffix] => key.ends_with(suffix),
                [prefix, suffix] => key.starts_with(prefix) && key.ends_with(suffix),
                _ => false,
            });
        } else {
            entries.remove(pattern);
        }
    }

    /// Clear all entries
    pub fn clear(&self) {
        let mut entries = self.entries.write().unwrap();
        entries.clear();
    }

    /// Remove expired entries
    pub fn cleanup(&self) -> usize {
        let mut entries = self.entries.write().unwrap();
        let before = entries.len();
        entries.retain(|_, entry| !entry.is_expired());
        before - entries.len()
    }

    /// Get the number of entries (including expired)
    pub fn len(&self) -> usize {
        self.entries.read().unwrap().len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.read().unwrap().is_empty()
    }

    /// Get or set with a closure (synchronous)
    pub fn get_or_set<F>(&self, key: &str, compute: F) -> T
    where
        F: FnOnce() -> T,
    {
        if let Some(value) = self.get(key) {
            return value;
        }

        let value = compute();
        self.set(key, value.clone());
        value
    }
}

impl<T: Clone> Clone for Cache<T> {
    fn clone(&self) -> Self {
        Self {
            entries: Arc::clone(&self.entries),
            default_ttl: self.default_ttl,
        }
    }
}

impl<T: Clone> Default for Cache<T> {
    fn default() -> Self {
        Self::medium()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ASYNC HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Cache a value with lazy async loading
pub async fn cached<T, F, Fut>(cache: &Cache<T>, key: &str, compute: F) -> T
where
    T: Clone,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    if let Some(value) = cache.get(key) {
        return value;
    }

    let value = compute().await;
    cache.set(key, value.clone());
    value
}

/// Cache a value with custom TTL
pub async fn cached_with_ttl<T, F, Fut>(cache: &Cache<T>, key: &str, ttl: Duration, compute: F) -> T
where
    T: Clone,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    if let Some(value) = cache.get(key) {
        return value;
    }

    let value = compute().await;
    cache.set_with_ttl(key, value.clone(), ttl);
    value
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPED CACHE KEYS
// ═══════════════════════════════════════════════════════════════════════════

/// Helper for building cache keys with type safety
pub struct CacheKey;

impl CacheKey {
    /// Create a user-scoped key
    pub fn user(user_id: &str, suffix: &str) -> String {
        format!("user:{}:{}", user_id, suffix)
    }

    /// Create a model-scoped key
    pub fn model(model: &str, id: &str) -> String {
        format!("{}:{}", model, id)
    }

    /// Create a query-scoped key
    pub fn query(table: &str, params: &[(&str, &str)]) -> String {
        let params_str: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        format!("query:{}:{}", table, params_str)
    }

    /// Create a session key
    pub fn session(session_id: &str) -> String {
        format!("session:{}", session_id)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("key1", "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache = Cache::<String>::new(Duration::from_millis(10));

        cache.set("key", "value".to_string());
        assert!(cache.has("key"));

        std::thread::sleep(Duration::from_millis(20));
        assert!(!cache.has("key"));
    }

    #[test]
    fn test_cache_invalidation() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("user:1:profile", "data1".to_string());
        cache.set("user:1:settings", "data2".to_string());
        cache.set("user:2:profile", "data3".to_string());

        cache.invalidate_prefix("user:1:");

        assert!(!cache.has("user:1:profile"));
        assert!(!cache.has("user:1:settings"));
        assert!(cache.has("user:2:profile"));
    }

    #[test]
    fn test_cache_get_or_set() {
        let cache = Cache::<i32>::new(Duration::from_secs(60));

        let mut calls = 0;
        let value1 = cache.get_or_set("key", || {
            calls += 1;
            42
        });

        let value2 = cache.get_or_set("key", || {
            calls += 1;
            99
        });

        assert_eq!(value1, 42);
        assert_eq!(value2, 42);
        assert_eq!(calls, 1); // Only called once
    }

    #[test]
    fn test_cache_key_helpers() {
        assert_eq!(CacheKey::user("123", "profile"), "user:123:profile");
        assert_eq!(CacheKey::model("User", "456"), "User:456");
        assert_eq!(CacheKey::session("abc"), "session:abc");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EDGE CASE TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_cache_cleanup() {
        let cache = Cache::<String>::new(Duration::from_millis(10));

        cache.set("key1", "value1".to_string());
        cache.set("key2", "value2".to_string());
        cache.set("key3", "value3".to_string());

        assert_eq!(cache.len(), 3);

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(20));

        // Cleanup should remove all expired
        let removed = cache.cleanup();
        assert_eq!(removed, 3);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_cleanup_mixed() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        // Set with short TTL
        cache.set_with_ttl("short", "expires".to_string(), Duration::from_millis(10));
        // Set with long TTL
        cache.set("long", "stays".to_string());

        std::thread::sleep(Duration::from_millis(20));

        let removed = cache.cleanup();
        assert_eq!(removed, 1);
        assert!(!cache.has("short"));
        assert!(cache.has("long"));
    }

    #[test]
    fn test_cache_pattern_invalidation_prefix() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("api:users:1", "a".to_string());
        cache.set("api:users:2", "b".to_string());
        cache.set("api:posts:1", "c".to_string());

        cache.invalidate_pattern("api:users:*");

        assert!(!cache.has("api:users:1"));
        assert!(!cache.has("api:users:2"));
        assert!(cache.has("api:posts:1"));
    }

    #[test]
    fn test_cache_pattern_invalidation_suffix() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("user_profile", "a".to_string());
        cache.set("admin_profile", "b".to_string());
        cache.set("user_settings", "c".to_string());

        cache.invalidate_pattern("*_profile");

        assert!(!cache.has("user_profile"));
        assert!(!cache.has("admin_profile"));
        assert!(cache.has("user_settings"));
    }

    #[test]
    fn test_cache_pattern_invalidation_contains() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("pre_match_suf", "a".to_string());
        cache.set("pre_other_suf", "b".to_string());
        cache.set("pre_match_other", "c".to_string());

        cache.invalidate_pattern("pre_*_suf");

        assert!(!cache.has("pre_match_suf"));
        assert!(!cache.has("pre_other_suf"));
        assert!(cache.has("pre_match_other"));
    }

    #[test]
    fn test_cache_empty_key() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("", "empty_key".to_string());
        assert_eq!(cache.get(""), Some("empty_key".to_string()));
    }

    #[test]
    fn test_cache_unicode_key() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("用户:123", "chinese_key".to_string());
        cache.set("🔑emoji", "emoji_key".to_string());

        assert_eq!(cache.get("用户:123"), Some("chinese_key".to_string()));
        assert_eq!(cache.get("🔑emoji"), Some("emoji_key".to_string()));
    }

    #[test]
    fn test_cache_overwrite() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("key", "value1".to_string());
        assert_eq!(cache.get("key"), Some("value1".to_string()));

        cache.set("key", "value2".to_string());
        assert_eq!(cache.get("key"), Some("value2".to_string()));
    }

    #[test]
    fn test_cache_delete() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("key", "value".to_string());
        assert!(cache.has("key"));

        let deleted = cache.delete("key");
        assert_eq!(deleted, Some("value".to_string()));
        assert!(!cache.has("key"));

        // Delete non-existent
        let deleted = cache.delete("nonexistent");
        assert_eq!(deleted, None);
    }

    #[test]
    fn test_cache_clear() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        cache.set("key1", "value1".to_string());
        cache.set("key2", "value2".to_string());
        cache.set("key3", "value3".to_string());

        assert!(!cache.is_empty());
        assert_eq!(cache.len(), 3);

        cache.clear();

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_custom_ttl() {
        let cache = Cache::<String>::new(Duration::from_secs(60));

        // Set with very short custom TTL
        cache.set_with_ttl("short", "expires".to_string(), Duration::from_millis(10));
        // Set with default TTL
        cache.set("long", "stays".to_string());

        assert!(cache.has("short"));
        assert!(cache.has("long"));

        std::thread::sleep(Duration::from_millis(20));

        assert!(!cache.has("short"));
        assert!(cache.has("long"));
    }

    #[test]
    fn test_cache_clone_shares_state() {
        let cache1 = Cache::<String>::new(Duration::from_secs(60));
        let cache2 = cache1.clone();

        cache1.set("key", "value".to_string());

        // cache2 should see the same data
        assert_eq!(cache2.get("key"), Some("value".to_string()));

        // Modifications via cache2 should be visible to cache1
        cache2.delete("key");
        assert!(!cache1.has("key"));
    }

    #[test]
    fn test_cache_presets() {
        let short = Cache::<String>::short();
        let medium = Cache::<String>::medium();
        let long = Cache::<String>::long();
        let default = Cache::<String>::default();

        assert_eq!(short.default_ttl, Duration::from_secs(300));
        assert_eq!(medium.default_ttl, Duration::from_secs(3600));
        assert_eq!(long.default_ttl, Duration::from_secs(86400));
        assert_eq!(default.default_ttl, Duration::from_secs(3600)); // medium
    }

    #[test]
    fn test_cache_key_query() {
        let key = CacheKey::query("users", &[("status", "active"), ("role", "admin")]);
        assert_eq!(key, "query:users:status=active&role=admin");

        // Empty params
        let key2 = CacheKey::query("posts", &[]);
        assert_eq!(key2, "query:posts:");
    }

    #[test]
    fn test_cache_complex_value() {
        #[derive(Clone, PartialEq, Debug)]
        struct User {
            id: i32,
            name: String,
        }

        let cache = Cache::<User>::new(Duration::from_secs(60));
        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };

        cache.set("user:1", user.clone());

        let retrieved = cache.get("user:1");
        assert_eq!(retrieved, Some(user));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ERROR TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_cache_error_key_not_found() {
        let err = CacheError::KeyNotFound("missing_key".to_string());
        assert!(err.to_string().contains("missing_key"));
        assert!(err.to_string().contains("Key not found"));
    }

    #[test]
    fn test_cache_error_serialization() {
        let err = CacheError::SerializationError("invalid JSON".to_string());
        assert!(err.to_string().contains("Serialization error"));
    }

    #[test]
    fn test_cache_error_lock_poisoned() {
        let err = CacheError::LockPoisoned;
        assert!(err.to_string().contains("lock poisoned"));
    }

    #[test]
    fn test_cache_error_connection_failed() {
        let err = CacheError::ConnectionFailed("Redis unreachable".to_string());
        assert!(err.to_string().contains("Redis unreachable"));
    }

    #[test]
    fn test_cache_error_timeout() {
        let err = CacheError::Timeout;
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn test_cache_error_expired() {
        let err = CacheError::Expired("user:123".to_string());
        assert!(err.to_string().contains("user:123"));
        assert!(err.to_string().contains("expired"));
    }
}
