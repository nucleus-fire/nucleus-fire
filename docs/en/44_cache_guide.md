# In-Memory Cache Guide

Nucleus Cache provides a high-performance, thread-safe in-memory caching struct with TTL support.

## Quick Start

```rust
use nucleus_std::cache::Cache;
use std::time::Duration;

// Create cache with 1 hour TTL
let cache = Cache::new(Duration::from_secs(3600));

// Or use a preset
let cache = Cache::medium(); // 1 hour

// Store value (Synchronous)
cache.set("user:123", &user);

// Retrieve value (Synchronous)
if let Some(user) = cache.get("user:123") {
    println!("Found user: {:?}", user);
}
```

## Basic Operations

### Constructors

```rust
use std::time::Duration;

// Custom TTL
let cache = Cache::new(Duration::from_secs(60));

// Presets
let short = Cache::short();   // 5 minutes
let medium = Cache::medium(); // 1 hour
let long = Cache::long();     // 24 hours
```

### Set and Get

```rust
// Set with default TTL
cache.set("key", &value);

// Set with custom TTL
cache.set_with_ttl("session:abc", &session, Duration::from_secs(300));

// Get (returns Option<T>)
// Note: T must be Clone
let value = cache.get("key");
```

### Check Existence

```rust
if cache.has("key") {
    println!("Key exists");
}
```

### Deletion

```rust
// Delete single key
cache.delete("key");

// Clear all entries
cache.clear();
```

### Pattern Invalidation

```rust
// Invalidate by prefix
cache.invalidate_prefix("user:");

// Invalidate by pattern (glob)
cache.invalidate_pattern("*:cache");
```

## Advanced Usage

### Cache-Aside (Sync)

```rust
let user = cache.get_or_set("user:123", || {
    // Computed if missing or expired
    fetch_user_sync(123)
});
```

### Async Decoration

Use the helper functions for async computations:

```rust
use nucleus_std::cache::cached;

// Lazy load asynchronously
let user = cached(&cache, "user:123", || async {
    db.find_user(123).await
}).await;
```

## Typed Keys

Type-safe key generation using `CacheKey`:

```rust
use nucleus_std::cache::CacheKey;

// User-scoped key
let key = CacheKey::user("123", "profile"); // "user:123:profile"

// Session key
let key = CacheKey::session("abc"); // "sess:abc"

cache.set(&key, &value);
```

## Thread Safety

`Cache<T>` is thread-safe (uses `Arc<RwLock>`) and can be cloned cheaply to share across threads/tasks.

```rust
let cache = Cache::medium();

// Clone shares the same underlying storage
let cache_clone = cache.clone();

tokio::spawn(async move {
    // Safe to use in other tasks
    cache_clone.set("bg_key", &value);
});
```

## Best Practices

1. **Cloneable Types**: Stored types `T` must implement `Clone`.
2. **Short TTLs**: For volatile data, use `Cache::short()`.
3. **Invalidation**: Use `invalidate_prefix` for group deletion.
4. **Memory Usage**: Cache stores everything in RAM; use `len()` to monitor size.
