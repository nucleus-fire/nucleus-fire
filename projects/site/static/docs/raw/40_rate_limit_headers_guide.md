# Rate Limit Headers Guide

Nucleus provides RFC-compliant rate limit headers that are automatically applied to responses.

## Quick Start

```rust
use nucleus_std::fortress::{RateLimiter, RateLimitConfig, RateLimitResult};

let limiter = RateLimiter::new(RateLimitConfig::api());

// Check rate limit
let result = limiter.check(&user_ip);

if result.is_allowed() {
    // Process request
    // Apply headers to response
    let headers = result.headers();
} else {
    // Return 429 with Retry-After
    return result.rate_limit_response();
}
```

## Headers Included

### When Allowed (2xx)

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Max requests in window |
| `X-RateLimit-Remaining` | Requests left in window |
| `X-RateLimit-Reset` | Seconds until window resets |

### When Blocked (429)

| Header | Description |
|--------|-------------|
| All above headers | Same info |
| `Retry-After` | Seconds until client can retry (RFC 7231) |

## Applying Headers

### Manual Application

```rust
let result = limiter.check(&key);

// Option 1: Get raw headers
let headers = result.headers();
for (name, value) in headers {
    response.headers_mut().insert(name, value);
}

// Option 2: Apply to axum response
result.apply_headers(&mut response);

// Option 3: Get as HashMap
let map = result.headers_map();
```

### Auto 429 Response

```rust
if !result.is_allowed() {
    return result.rate_limit_response();
}

// Returns:
// Status: 429 Too Many Requests
// Content-Type: application/json
// X-RateLimit-Limit: 60
// X-RateLimit-Remaining: 0
// X-RateLimit-Reset: 42
// Retry-After: 42
//
// Body:
// {
//   "error": "Too Many Requests",
//   "message": "Rate limit exceeded. Please retry later.",
//   "retry_after": 42,
//   "limit": 60,
//   "remaining": 0
// }
```

## Middleware Example

```rust
use axum::{
    middleware::{self, Next},
    extract::{Request, Extension},
    response::Response,
};

async fn rate_limit_middleware(
    Extension(limiter): Extension<RateLimiter>,
    request: Request,
    next: Next,
) -> Response {
    // Get client IP
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    let result = limiter.check(ip);
    
    if !result.is_allowed() {
        return result.rate_limit_response().into_response();
    }
    
    // Process request
    let mut response = next.run(request).await;
    
    // Always add rate limit headers
    result.apply_headers(&mut response);
    
    response
}

// Apply to router
let app = Router::new()
    .route("/api/*", get(api_handler))
    .layer(middleware::from_fn(rate_limit_middleware))
    .layer(Extension(limiter));
```

## Status Codes

```rust
// Check status directly
let status = result.status_code();  // 200 or 429

// Use as HTTP response code
if result.is_allowed() {
    // 200 OK
} else {
    // 429 Too Many Requests
}
```

## Configuration Presets

```rust
// API: 60 req/min
let api = RateLimitConfig::api();

// Login: 5 req/5min (strict)
let login = RateLimitConfig::login();

// Pages: 1000 req/min (lenient)
let pages = RateLimitConfig::lenient();

// Custom
let custom = RateLimitConfig {
    max_requests: 100,
    window: Duration::from_secs(60),
    key_type: RateLimitKey::Ip,
};
```

## Best Practices

1. **Always include headers** on allowed responses (helps clients self-regulate)
2. **Use Retry-After** for 429s (required by RFC 6585)
3. **Key by IP + Route** for granular limits
4. **Use different limits** for auth routes vs API
5. **Log rate limit hits** for abuse detection
