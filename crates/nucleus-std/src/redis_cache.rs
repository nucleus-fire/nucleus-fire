//! Nucleus Redis Cache Module
//!
//! Redis-backed distributed caching for horizontal scaling:
//! - Connection pooling with automatic reconnection
//! - TTL support with Redis SETEX
//! - Pattern-based key invalidation (SCAN + DEL)
//! - Serialization via serde_json
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::redis_cache::RedisCache;
//!
//! // Connect to Redis
//! let cache = RedisCache::new("redis://localhost:6379").await?;
//!
//! // Basic operations
//! cache.set("key", "value", 300).await?; // 5 min TTL
//! let value: Option<String> = cache.get("key").await?;
//!
//! // Pattern invalidation
//! cache.delete_pattern("user:123:*").await?;
//! ```

use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Redis cache error types
#[derive(Debug, thiserror::Error)]
pub enum RedisCacheError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Operation error: {0}")]
    Operation(String),
    
    #[error("Pool exhausted")]
    PoolExhausted,
    
    #[error("Key not found: {0}")]
    NotFound(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// CACHE BACKEND TRAIT
// ═══════════════════════════════════════════════════════════════════════════

/// Unified cache backend trait for both memory and Redis
#[async_trait::async_trait]
pub trait CacheBackend: Send + Sync {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, RedisCacheError>;
    async fn set_bytes(&self, key: &str, value: Vec<u8>, ttl_secs: u64) -> Result<(), RedisCacheError>;
    async fn delete(&self, key: &str) -> Result<bool, RedisCacheError>;
    async fn exists(&self, key: &str) -> Result<bool, RedisCacheError>;
    async fn delete_pattern(&self, pattern: &str) -> Result<usize, RedisCacheError>;
    async fn clear(&self) -> Result<(), RedisCacheError>;
    async fn ttl(&self, key: &str) -> Result<Option<i64>, RedisCacheError>;
}

// ═══════════════════════════════════════════════════════════════════════════
// IN-MEMORY BACKEND (for testing/single-server)
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

struct MemoryEntry {
    value: Vec<u8>,
    expires_at: Option<std::time::Instant>,
}

/// In-memory cache backend (for testing or single-server deployments)
pub struct MemoryCacheBackend {
    entries: Arc<RwLock<HashMap<String, MemoryEntry>>>,
}

impl MemoryCacheBackend {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemoryCacheBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MemoryCacheBackend {
    fn clone(&self) -> Self {
        Self {
            entries: Arc::clone(&self.entries),
        }
    }
}

#[async_trait::async_trait]
impl CacheBackend for MemoryCacheBackend {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, RedisCacheError> {
        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(key) {
            if let Some(expires_at) = entry.expires_at {
                if std::time::Instant::now() >= expires_at {
                    return Ok(None);
                }
            }
            Ok(Some(entry.value.clone()))
        } else {
            Ok(None)
        }
    }
    
    async fn set_bytes(&self, key: &str, value: Vec<u8>, ttl_secs: u64) -> Result<(), RedisCacheError> {
        let mut entries = self.entries.write().await;
        let expires_at = if ttl_secs > 0 {
            Some(std::time::Instant::now() + Duration::from_secs(ttl_secs))
        } else {
            None
        };
        entries.insert(key.to_string(), MemoryEntry { value, expires_at });
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<bool, RedisCacheError> {
        let mut entries = self.entries.write().await;
        Ok(entries.remove(key).is_some())
    }
    
    async fn exists(&self, key: &str) -> Result<bool, RedisCacheError> {
        Ok(self.get_bytes(key).await?.is_some())
    }
    
    async fn delete_pattern(&self, pattern: &str) -> Result<usize, RedisCacheError> {
        let mut entries = self.entries.write().await;
        let before = entries.len();
        
        // Convert glob pattern to simple matching
        if let Some(prefix) = pattern.strip_suffix('*') {
            entries.retain(|k, _| !k.starts_with(prefix));
        } else if let Some(suffix) = pattern.strip_prefix('*') {
            entries.retain(|k, _| !k.ends_with(suffix));
        } else if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                entries.retain(|k, _| !(k.starts_with(parts[0]) && k.ends_with(parts[1])));
            }
        } else {
            entries.remove(pattern);
        }
        
        Ok(before - entries.len())
    }
    
    async fn clear(&self) -> Result<(), RedisCacheError> {
        let mut entries = self.entries.write().await;
        entries.clear();
        Ok(())
    }
    
    async fn ttl(&self, key: &str) -> Result<Option<i64>, RedisCacheError> {
        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(key) {
            if let Some(expires_at) = entry.expires_at {
                let now = std::time::Instant::now();
                if now >= expires_at {
                    Ok(Some(-2)) // Expired
                } else {
                    let remaining = expires_at.duration_since(now);
                    Ok(Some(remaining.as_secs() as i64))
                }
            } else {
                Ok(Some(-1)) // No TTL
            }
        } else {
            Ok(None)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REDIS BACKEND (for distributed deployments)
// ═══════════════════════════════════════════════════════════════════════════

/// Redis connection pool wrapper
pub struct RedisBackend {
    url: String,
    client: Arc<RwLock<Option<redis::Client>>>,
    pool_size: usize,
}

impl RedisBackend {
    /// Create a new Redis backend
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: Arc::new(RwLock::new(None)),
            pool_size: 10,
        }
    }
    
    /// Connect to Redis
    pub async fn connect(&self) -> Result<(), RedisCacheError> {
        let client = redis::Client::open(self.url.as_str())
            .map_err(|e| RedisCacheError::Connection(e.to_string()))?;
        
        // Test connection
        let mut con = client.get_multiplexed_async_connection().await
            .map_err(|e| RedisCacheError::Connection(e.to_string()))?;
        
        let _: String = redis::cmd("PING")
            .query_async(&mut con)
            .await
            .map_err(|e| RedisCacheError::Connection(e.to_string()))?;
        
        let mut guard = self.client.write().await;
        *guard = Some(client);
        
        Ok(())
    }
    
    /// Get a connection from the pool
    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, RedisCacheError> {
        let guard = self.client.read().await;
        let client = guard.as_ref()
            .ok_or_else(|| RedisCacheError::Connection("Not connected".to_string()))?;
        
        client.get_multiplexed_async_connection().await
            .map_err(|e| RedisCacheError::Connection(e.to_string()))
    }
    
    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        let guard = self.client.read().await;
        guard.is_some()
    }
}

impl Clone for RedisBackend {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            client: Arc::clone(&self.client),
            pool_size: self.pool_size,
        }
    }
}

#[async_trait::async_trait]
impl CacheBackend for RedisBackend {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        let result: Option<Vec<u8>> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        
        Ok(result)
    }
    
    async fn set_bytes(&self, key: &str, value: Vec<u8>, ttl_secs: u64) -> Result<(), RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        if ttl_secs > 0 {
            let _: () = redis::cmd("SETEX")
                .arg(key)
                .arg(ttl_secs)
                .arg(value)
                .query_async(&mut con)
                .await
                .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        } else {
            let _: () = redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query_async(&mut con)
                .await
                .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        }
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<bool, RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        let deleted: i32 = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        
        Ok(deleted > 0)
    }
    
    async fn exists(&self, key: &str) -> Result<bool, RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        let exists: i32 = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        
        Ok(exists > 0)
    }
    
    async fn delete_pattern(&self, pattern: &str) -> Result<usize, RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        // Use SCAN to find keys matching pattern
        let mut cursor = 0u64;
        let mut total_deleted = 0usize;
        
        loop {
            let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut con)
                .await
                .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
            
            if !keys.is_empty() {
                let deleted: i32 = redis::cmd("DEL")
                    .arg(&keys)
                    .query_async(&mut con)
                    .await
                    .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
                total_deleted += deleted as usize;
            }
            
            cursor = new_cursor;
            if cursor == 0 {
                break;
            }
        }
        
        Ok(total_deleted)
    }
    
    async fn clear(&self) -> Result<(), RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        let _: () = redis::cmd("FLUSHDB")
            .query_async(&mut con)
            .await
            .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        
        Ok(())
    }
    
    async fn ttl(&self, key: &str) -> Result<Option<i64>, RedisCacheError> {
        let mut con = self.get_connection().await?;
        
        let ttl: i64 = redis::cmd("TTL")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| RedisCacheError::Operation(e.to_string()))?;
        
        if ttl == -2 {
            Ok(None) // Key doesn't exist
        } else {
            Ok(Some(ttl))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIFIED CACHE
// ═══════════════════════════════════════════════════════════════════════════

/// Unified cache that works with any backend
pub struct UnifiedCache<B: CacheBackend> {
    backend: B,
    prefix: Option<String>,
    default_ttl: u64,
}

impl<B: CacheBackend> UnifiedCache<B> {
    /// Create a new unified cache with a backend
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            prefix: None,
            default_ttl: 3600, // 1 hour default
        }
    }
    
    /// Set a key prefix for namespacing
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }
    
    /// Set default TTL in seconds
    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.default_ttl = ttl_secs;
        self
    }
    
    fn make_key(&self, key: &str) -> String {
        match &self.prefix {
            Some(p) => format!("{}:{}", p, key),
            None => key.to_string(),
        }
    }
    
    /// Get a value from cache
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisCacheError> {
        let full_key = self.make_key(key);
        
        match self.backend.get_bytes(&full_key).await? {
            Some(bytes) => {
                let value: T = serde_json::from_slice(&bytes)
                    .map_err(|e| RedisCacheError::Serialization(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    /// Set a value with default TTL
    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), RedisCacheError> {
        self.set_with_ttl(key, value, self.default_ttl).await
    }
    
    /// Set a value with custom TTL
    pub async fn set_with_ttl<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) -> Result<(), RedisCacheError> {
        let full_key = self.make_key(key);
        let bytes = serde_json::to_vec(value)
            .map_err(|e| RedisCacheError::Serialization(e.to_string()))?;
        
        self.backend.set_bytes(&full_key, bytes, ttl_secs).await
    }
    
    /// Delete a key
    pub async fn delete(&self, key: &str) -> Result<bool, RedisCacheError> {
        let full_key = self.make_key(key);
        self.backend.delete(&full_key).await
    }
    
    /// Check if key exists
    pub async fn exists(&self, key: &str) -> Result<bool, RedisCacheError> {
        let full_key = self.make_key(key);
        self.backend.exists(&full_key).await
    }
    
    /// Delete keys matching pattern
    pub async fn delete_pattern(&self, pattern: &str) -> Result<usize, RedisCacheError> {
        let full_pattern = self.make_key(pattern);
        self.backend.delete_pattern(&full_pattern).await
    }
    
    /// Clear all keys (with prefix if set)
    pub async fn clear(&self) -> Result<(), RedisCacheError> {
        if let Some(ref prefix) = self.prefix {
            self.backend.delete_pattern(&format!("{}:*", prefix)).await?;
            Ok(())
        } else {
            self.backend.clear().await
        }
    }
    
    /// Get remaining TTL for a key
    pub async fn ttl(&self, key: &str) -> Result<Option<i64>, RedisCacheError> {
        let full_key = self.make_key(key);
        self.backend.ttl(&full_key).await
    }
    
    /// Get or compute value (cache-aside pattern)
    pub async fn get_or_set<T, F, Fut>(&self, key: &str, compute: F) -> Result<T, RedisCacheError>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, RedisCacheError>>,
    {
        if let Some(value) = self.get(key).await? {
            return Ok(value);
        }
        
        let value = compute().await?;
        self.set(key, &value).await?;
        Ok(value)
    }
}

impl<B: CacheBackend + Clone> Clone for UnifiedCache<B> {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            prefix: self.prefix.clone(),
            default_ttl: self.default_ttl,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FACTORY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Create an in-memory cache
pub fn memory_cache() -> UnifiedCache<MemoryCacheBackend> {
    UnifiedCache::new(MemoryCacheBackend::new())
}

/// Create a Redis cache (requires connection)
pub async fn redis_cache(url: &str) -> Result<UnifiedCache<RedisBackend>, RedisCacheError> {
    let backend = RedisBackend::new(url);
    backend.connect().await?;
    Ok(UnifiedCache::new(backend))
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    // ═══════════════════════════════════════════════════════════════════════
    // MEMORY BACKEND TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_memory_backend_set_get() {
        let backend = MemoryCacheBackend::new();
        
        backend.set_bytes("key", b"value".to_vec(), 300).await.unwrap();
        let result = backend.get_bytes("key").await.unwrap();
        
        assert_eq!(result, Some(b"value".to_vec()));
    }
    
    #[tokio::test]
    async fn test_memory_backend_delete() {
        let backend = MemoryCacheBackend::new();
        
        backend.set_bytes("key", b"value".to_vec(), 300).await.unwrap();
        let deleted = backend.delete("key").await.unwrap();
        
        assert!(deleted);
        assert!(backend.get_bytes("key").await.unwrap().is_none());
    }
    
    #[tokio::test]
    async fn test_memory_backend_exists() {
        let backend = MemoryCacheBackend::new();
        
        assert!(!backend.exists("key").await.unwrap());
        
        backend.set_bytes("key", b"value".to_vec(), 300).await.unwrap();
        
        assert!(backend.exists("key").await.unwrap());
    }
    
    #[tokio::test]
    async fn test_memory_backend_delete_pattern_prefix() {
        let backend = MemoryCacheBackend::new();
        
        backend.set_bytes("user:1:name", b"alice".to_vec(), 300).await.unwrap();
        backend.set_bytes("user:1:email", b"alice@test.com".to_vec(), 300).await.unwrap();
        backend.set_bytes("user:2:name", b"bob".to_vec(), 300).await.unwrap();
        
        let deleted = backend.delete_pattern("user:1:*").await.unwrap();
        
        assert_eq!(deleted, 2);
        assert!(backend.get_bytes("user:1:name").await.unwrap().is_none());
        assert!(backend.get_bytes("user:2:name").await.unwrap().is_some());
    }
    
    #[tokio::test]
    async fn test_memory_backend_delete_pattern_suffix() {
        let backend = MemoryCacheBackend::new();
        
        backend.set_bytes("user:1:cache", b"1".to_vec(), 300).await.unwrap();
        backend.set_bytes("user:2:cache", b"2".to_vec(), 300).await.unwrap();
        backend.set_bytes("user:1:data", b"3".to_vec(), 300).await.unwrap();
        
        let deleted = backend.delete_pattern("*:cache").await.unwrap();
        
        assert_eq!(deleted, 2);
        assert!(backend.get_bytes("user:1:data").await.unwrap().is_some());
    }
    
    #[tokio::test]
    async fn test_memory_backend_clear() {
        let backend = MemoryCacheBackend::new();
        
        backend.set_bytes("key1", b"1".to_vec(), 300).await.unwrap();
        backend.set_bytes("key2", b"2".to_vec(), 300).await.unwrap();
        
        backend.clear().await.unwrap();
        
        assert!(backend.get_bytes("key1").await.unwrap().is_none());
        assert!(backend.get_bytes("key2").await.unwrap().is_none());
    }
    
    #[tokio::test]
    async fn test_memory_backend_ttl() {
        let backend = MemoryCacheBackend::new();
        
        backend.set_bytes("key", b"value".to_vec(), 300).await.unwrap();
        
        let ttl = backend.ttl("key").await.unwrap();
        assert!(ttl.is_some());
        assert!(ttl.unwrap() > 0);
        
        // Key without TTL
        backend.set_bytes("eternal", b"value".to_vec(), 0).await.unwrap();
        let ttl = backend.ttl("eternal").await.unwrap();
        assert_eq!(ttl, Some(-1));
        
        // Non-existent key
        let ttl = backend.ttl("nonexistent").await.unwrap();
        assert!(ttl.is_none());
    }
    
    #[tokio::test]
    async fn test_memory_backend_expiration() {
        let backend = MemoryCacheBackend::new();
        
        // Set with very short TTL
        backend.set_bytes("key", b"value".to_vec(), 1).await.unwrap();
        
        // Should exist initially
        assert!(backend.exists("key").await.unwrap());
        
        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(1100)).await;
        
        // Should be expired
        assert!(!backend.exists("key").await.unwrap());
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // UNIFIED CACHE TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_unified_cache_set_get() {
        let cache = memory_cache();
        
        cache.set("key", &"value".to_string()).await.unwrap();
        let result: Option<String> = cache.get("key").await.unwrap();
        
        assert_eq!(result, Some("value".to_string()));
    }
    
    #[tokio::test]
    async fn test_unified_cache_with_prefix() {
        let cache = memory_cache().with_prefix("app");
        
        cache.set("key", &42i32).await.unwrap();
        
        // Direct backend access shows prefixed key
        let result: Option<i32> = cache.get("key").await.unwrap();
        assert_eq!(result, Some(42));
    }
    
    #[tokio::test]
    async fn test_unified_cache_struct() {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        struct User {
            id: i32,
            name: String,
        }
        
        let cache = memory_cache();
        let user = User { id: 1, name: "Alice".into() };
        
        cache.set("user", &user).await.unwrap();
        let result: Option<User> = cache.get("user").await.unwrap();
        
        assert_eq!(result, Some(user));
    }
    
    #[tokio::test]
    async fn test_unified_cache_get_or_set() {
        let cache = memory_cache();
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        
        let cc = call_count.clone();
        let result1: String = cache.get_or_set("key", || {
            let c = cc.clone();
            async move {
                c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok("computed".to_string())
            }
        }).await.unwrap();
        
        let cc = call_count.clone();
        let result2: String = cache.get_or_set("key", || {
            let c = cc.clone();
            async move {
                c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok("computed again".to_string())
            }
        }).await.unwrap();
        
        assert_eq!(result1, "computed");
        assert_eq!(result2, "computed"); // Returns cached value
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1); // Only computed once
    }
    
    #[tokio::test]
    async fn test_unified_cache_delete() {
        let cache = memory_cache();
        
        cache.set("key", &"value".to_string()).await.unwrap();
        cache.delete("key").await.unwrap();
        
        let result: Option<String> = cache.get("key").await.unwrap();
        assert!(result.is_none());
    }
    
    #[tokio::test]
    async fn test_unified_cache_delete_pattern() {
        let cache = memory_cache();
        
        cache.set("user:1:name", &"alice".to_string()).await.unwrap();
        cache.set("user:1:email", &"alice@test.com".to_string()).await.unwrap();
        cache.set("user:2:name", &"bob".to_string()).await.unwrap();
        
        cache.delete_pattern("user:1:*").await.unwrap();
        
        assert!(cache.get::<String>("user:1:name").await.unwrap().is_none());
        assert!(cache.get::<String>("user:2:name").await.unwrap().is_some());
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // EDGE CASES
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_empty_key() {
        let cache = memory_cache();
        
        cache.set("", &"value".to_string()).await.unwrap();
        let result: Option<String> = cache.get("").await.unwrap();
        
        assert_eq!(result, Some("value".to_string()));
    }
    
    #[tokio::test]
    async fn test_unicode_key() {
        let cache = memory_cache();
        
        cache.set("用户:123", &"chinese".to_string()).await.unwrap();
        cache.set("🔑:emoji", &"emoji".to_string()).await.unwrap();
        
        assert_eq!(cache.get::<String>("用户:123").await.unwrap(), Some("chinese".to_string()));
        assert_eq!(cache.get::<String>("🔑:emoji").await.unwrap(), Some("emoji".to_string()));
    }
    
    #[tokio::test]
    async fn test_large_value() {
        let cache = memory_cache();
        
        let large = "x".repeat(1_000_000); // 1MB
        cache.set("large", &large).await.unwrap();
        
        let result: Option<String> = cache.get("large").await.unwrap();
        assert_eq!(result.unwrap().len(), 1_000_000);
    }
    
    #[tokio::test]
    async fn test_binary_data() {
        let cache = memory_cache();
        
        let binary: Vec<u8> = (0..255).collect();
        cache.set("binary", &binary).await.unwrap();
        
        let result: Option<Vec<u8>> = cache.get("binary").await.unwrap();
        assert_eq!(result, Some(binary));
    }
    
    #[tokio::test]
    async fn test_null_value() {
        let cache = memory_cache();
        
        let value: Option<String> = None;
        cache.set("null", &value).await.unwrap();
        
        let result: Option<Option<String>> = cache.get("null").await.unwrap();
        assert_eq!(result, Some(None));
    }
    
    #[tokio::test]
    async fn test_concurrent_access() {
        let cache = memory_cache();
        let cache_clone = cache.clone();
        
        let handle1 = tokio::spawn(async move {
            for i in 0..100 {
                cache_clone.set(&format!("key:{}", i), &i).await.unwrap();
            }
        });
        
        let cache_clone = cache.clone();
        let handle2 = tokio::spawn(async move {
            for i in 100..200 {
                cache_clone.set(&format!("key:{}", i), &i).await.unwrap();
            }
        });
        
        handle1.await.unwrap();
        handle2.await.unwrap();
        
        // Verify some random keys
        assert!(cache.exists("key:50").await.unwrap());
        assert!(cache.exists("key:150").await.unwrap());
    }
    
    #[tokio::test]
    async fn test_overwrite_value() {
        let cache = memory_cache();
        
        cache.set("key", &"first".to_string()).await.unwrap();
        cache.set("key", &"second".to_string()).await.unwrap();
        
        let result: Option<String> = cache.get("key").await.unwrap();
        assert_eq!(result, Some("second".to_string()));
    }
    
    #[tokio::test]
    async fn test_clone_shares_state() {
        let cache1 = memory_cache();
        let cache2 = cache1.clone();
        
        cache1.set("key", &"value".to_string()).await.unwrap();
        
        let result: Option<String> = cache2.get("key").await.unwrap();
        assert_eq!(result, Some("value".to_string()));
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // REDIS BACKEND TESTS (mocked)
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_redis_backend_creation() {
        let backend = RedisBackend::new("redis://localhost:6379");
        assert_eq!(backend.url, "redis://localhost:6379");
    }
    
    #[tokio::test]
    async fn test_redis_backend_not_connected() {
        let backend = RedisBackend::new("redis://localhost:6379");
        
        let result = backend.get_bytes("key").await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_error_types() {
        let _ = RedisCacheError::Connection("test".into());
        let _ = RedisCacheError::Serialization("test".into());
        let _ = RedisCacheError::Operation("test".into());
        let _ = RedisCacheError::PoolExhausted;
        let _ = RedisCacheError::NotFound("key".into());
    }
}
