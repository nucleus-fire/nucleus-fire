# Middleware Guide

Middleware allows you to run code before and after request handlers, enabling cross-cutting concerns like authentication, logging, and rate limiting.

## Overview

Nucleus middleware is defined in `src/middleware.rs` and automatically applied to all routes.

## Quick Example

```rust
// src/middleware.rs
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};

pub async fn global_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // Before request
    let start = std::time::Instant::now();
    
    // Process request
    let response = next.run(request).await;
    
    // After request
    let duration = start.elapsed();
    println!("Request took {:?}", duration);
    
    response
}
```

## Creating Middleware

### 1. Create the File

Create `src/middleware.rs` in your project:

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};

pub async fn global_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // Your middleware logic here
    next.run(request).await
}
```

### 2. Automatic Detection

Nucleus automatically detects `src/middleware.rs` and applies it during build.

## Common Middleware Patterns

### Authentication Check

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use nucleus_std::fortress;

pub async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Get auth header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    if let Some(token) = auth_header {
        // Verify token
        if fortress::verify_token(token).is_ok() {
            return Ok(next.run(request).await);
        }
    }
    
    // Public routes - no auth required
    let path = request.uri().path();
    if path == "/" || path.starts_with("/public") {
        return Ok(next.run(request).await);
    }
    
    Err(StatusCode::UNAUTHORIZED)
}
```

### Request Logging

```rust
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use chrono::Utc;

pub async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    println!(
        "[{}] {} {} - {} ({:?})",
        Utc::now().format("%Y-%m-%d %H:%M:%S"),
        method,
        uri,
        status,
        duration
    );
    
    response
}
```

### Rate Limiting

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

lazy_static::lazy_static! {
    static ref RATE_LIMITER: Mutex<HashMap<String, (u32, Instant)>> = 
        Mutex::new(HashMap::new());
}

const MAX_REQUESTS: u32 = 100;
const WINDOW: Duration = Duration::from_secs(60);

pub async fn rate_limit_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let ip = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    
    let mut limiter = RATE_LIMITER.lock().unwrap();
    let now = Instant::now();
    
    let (count, window_start) = limiter
        .entry(ip.clone())
        .or_insert((0, now));
    
    // Reset window if expired
    if now.duration_since(*window_start) > WINDOW {
        *count = 0;
        *window_start = now;
    }
    
    *count += 1;
    
    if *count > MAX_REQUESTS {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}
```

### CORS Headers

```rust
use axum::{
    http::{Request, HeaderValue},
    middleware::Next,
    response::Response,
};

pub async fn cors_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_static("*")
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS")
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type, Authorization")
    );
    
    response
}
```

### Security Headers

```rust
pub async fn security_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin")
    );
    
    response
}
```

## Middleware Order

Middleware runs in the order defined. For a typical app:

1. Logging (first - tracks all requests)
2. Security headers
3. Rate limiting
4. Authentication
5. Your route handlers
6. (Response goes back through in reverse)

## Error Handling

Return errors as status codes:

```rust
pub async fn middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Check something
    if some_condition_fails {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Or return custom response
    if other_condition {
        return Err(StatusCode::FORBIDDEN);
    }
    
    Ok(next.run(request).await)
}
```

## Accessing Request Data

```rust
pub async fn middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // Headers
    let user_agent = request.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok());
    
    // Path
    let path = request.uri().path();
    
    // Query string
    let query = request.uri().query();
    
    // Method
    let method = request.method();
    
    next.run(request).await
}
```

## Best Practices

1. **Keep it fast** - Middleware runs on every request
2. **Handle errors gracefully** - Return proper HTTP status codes
3. **Don't block** - Use async operations
4. **Log wisely** - Don't log sensitive data
5. **Test thoroughly** - Middleware affects all routes

## Related Guides

- [Authentication](#21_authentication_guide) - Auth patterns
- [API Development](#22_api_development) - Route handling
- [Performance](#13_performance_benchmarks) - Optimization
