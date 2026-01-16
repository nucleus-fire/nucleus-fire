# Redis Cache Guide

Nucleus provides a unified caching layer that supports both in-memory and Redis backends for horizontal scaling.

## Quick Start

```rust
use nucleus_std::redis_cache::{memory_cache, redis_cache, CacheBackend};

// In-memory (single server, testing)
let cache = memory_cache();

// Redis (distributed, production)
let cache = redis_cache("redis://localhost:6379").await?;

// Operations are identical
cache.set("key", &"value".to_string()).await?;
let value: Option<String> = cache.get("key").await?;
```

## Features

### Unified API

Both memory and Redis backends implement the same `CacheBackend` trait:

```rust
// Works with any backend
async fn cache_user<B: CacheBackend>(cache: &UnifiedCache<B>, user: &User) {
    cache.set(&format!("user:{}", user.id), user).await.unwrap();
}
```

### TTL Support

```rust
// Default TTL (1 hour)
cache.set("key", &value).await?;

// Custom TTL (5 minutes)
cache.set_with_ttl("session", &session, 300).await?;

// Configure default TTL
let cache = memory_cache().with_ttl(1800); // 30 min default
```

### Key Prefixing

Namespace your keys to avoid collisions:

```rust
let user_cache = memory_cache().with_prefix("users");
let session_cache = memory_cache().with_prefix("sessions");

// Keys are automatically prefixed: "users:123", "sessions:abc"
user_cache.set("123", &user).await?;
session_cache.set("abc", &session).await?;
```

### Pattern Invalidation

```rust
// Delete all user cache
cache.delete_pattern("user:*").await?;

// Delete by suffix
cache.delete_pattern("*:cache").await?;

// Delete specific user's data
cache.delete_pattern("user:123:*").await?;
```

### Cache-Aside Pattern

Automatically fetch on miss:

```rust
let user: User = cache.get_or_set("user:123", || async {
    // Only called if not in cache
    db.find_user(123).await
}).await?;
```

## Backend Details

### Memory Backend

Best for:
- Development/testing
- Single-server deployments
- Ephemeral data

```rust
let cache = memory_cache();
```

Features:
- Zero dependencies
- Instant access
- Automatic expiration
- Thread-safe (Arc + RwLock)

### Redis Backend

Best for:
- Multi-server deployments
- Persistent cache across restarts
- Shared state between processes

```rust
let cache = redis_cache("redis://localhost:6379").await?;

// With authentication
let cache = redis_cache("redis://:password@host:6379/0").await?;
```

Features:
- Connection pooling
- Auto-reconnection
- SCAN-based pattern deletion
- Native TTL support

## Storing Different Types

```rust
// Strings
cache.set("name", &"Alice".to_string()).await?;

// Numbers
cache.set("count", &42i32).await?;

// Structs (must implement Serialize/Deserialize)
#[derive(Serialize, Deserialize)]
struct User { id: i32, name: String }

cache.set("user:1", &User { id: 1, name: "Alice".into() }).await?;

// Collections
cache.set("items", &vec![1, 2, 3]).await?;

// Binary data
cache.set("binary", &vec![0u8; 1024]).await?;
```

## Checking TTL

```rust
let ttl = cache.ttl("key").await?;

match ttl {
    Some(secs) if secs > 0 => println!("Expires in {} seconds", secs),
    Some(-1) => println!("No expiration"),
    Some(-2) => println!("Key expired"),
    None => println!("Key doesn't exist"),
    _ => {}
}
```

## Error Handling

```rust
use nucleus_std::redis_cache::RedisCacheError;

match cache.get::<String>("key").await {
    Ok(Some(value)) => println!("Found: {}", value),
    Ok(None) => println!("Key not found"),
    Err(RedisCacheError::Connection(e)) => eprintln!("Redis down: {}", e),
    Err(RedisCacheError::Serialization(e)) => eprintln!("Bad data: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Tricky Scenarios

### Cache Stampede Prevention

When a popular cache entry expires, many requests may simultaneously try to regenerate it, overwhelming your database. Use a lock to serialize regeneration:

```rust
use nucleus_std::redis_cache::CacheLock;

async fn get_popular_data(cache: &UnifiedCache<impl CacheBackend>) -> Result<Data> {
    // Try cache first
    if let Some(data) = cache.get::<Data>("popular_data").await? {
        return Ok(data);
    }
    
    // Acquire lock to prevent stampede
    let lock = CacheLock::new(cache, "popular_data:lock");
    let _guard = lock.acquire(Duration::from_secs(5)).await?;
    
    // Double-check (another request may have filled cache while we waited)
    if let Some(data) = cache.get::<Data>("popular_data").await? {
        return Ok(data);
    }
    
    // Only one request regenerates
    let data = fetch_from_database().await?;
    cache.set("popular_data", &data).await?;
    Ok(data)
}
```

### Early Expiration (Probabilistic)

Regenerate cache *before* it expires to avoid cold cache:

```rust
// Set TTL to 5 minutes, but regenerate if < 1 minute remaining
let ttl = cache.ttl("key").await?.unwrap_or(0);
if ttl < 60 {
    // Trigger async background refresh
    tokio::spawn(async move { refresh_cache("key").await });
}
```

## Best Practices

1. **Use prefixes** to namespace your cache keys
2. **Set appropriate TTLs** to prevent stale data
3. **Handle cache misses** gracefully
4. **Use get_or_set** for cache-aside pattern
5. **Monitor Redis** in production deployments
