//! Nucleus Middleware Module
//!
//! Provides HTTP middleware utilities and helpers:
//! - Request logging with tracing
//! - Security headers
//! - CORS configuration
//! - Rate limiting integration
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::middleware::{request_logger, apply_security_headers};
//!
//! let app = Router::new()
//!     .route("/", get(handler))
//!     .layer(middleware::from_fn(request_logger));
//! ```

use axum::{
    body::Body,
    http::{Request, Response, HeaderValue, StatusCode},
    middleware::Next,
};
use std::time::Instant;
use crate::logging::{info, warn, error};

/// Re-export common types for ease of use
pub type NucleusRequest = Request<Body>;
pub type NucleusResponse = Response<Body>;
pub type NucleusNext = Next;

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST LOGGING
// ═══════════════════════════════════════════════════════════════════════════

/// Logging middleware with structured tracing
pub async fn request_logger(
    request: NucleusRequest,
    next: NucleusNext,
) -> NucleusResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();
    let start = Instant::now();
    
    // Execute request
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    // Log based on status code
    if status.is_server_error() {
        error!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %duration.as_millis(),
            "Request failed"
        );
    } else if status.is_client_error() {
        warn!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %duration.as_millis(),
            "Client error"
        );
    } else {
        info!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %duration.as_millis(),
            "Request completed"
        );
    }
    
    response
}

/// Legacy logger helper (for backward compatibility)
pub fn log_request(
    method: &axum::http::Method, 
    uri: &axum::http::Uri, 
    duration: std::time::Duration, 
    status: StatusCode
) {
    info!(
        method = %method,
        path = %uri.path(),
        status = %status.as_u16(),
        duration_ms = %duration.as_millis(),
        "Request completed"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// SECURITY HEADERS
// ═══════════════════════════════════════════════════════════════════════════

/// Apply standard security headers to response
pub fn apply_security_headers(response: &mut NucleusResponse) {
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("Strict-Transport-Security", HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
}

/// Security headers middleware
pub async fn security_headers_middleware(
    request: NucleusRequest,
    next: NucleusNext,
) -> NucleusResponse {
    let mut response = next.run(request).await;
    apply_security_headers(&mut response);
    response
}

// ═══════════════════════════════════════════════════════════════════════════
// CORS
// ═══════════════════════════════════════════════════════════════════════════

/// CORS configuration
#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allow_origins: Vec<String>,
    pub allow_methods: Vec<String>,
    pub allow_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: u32,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allow_origins: vec!["*".to_string()],
            allow_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            allow_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
            ],
            allow_credentials: false,
            max_age: 86400,
        }
    }
}

impl CorsConfig {
    /// Create strict CORS (no wildcards)
    pub fn strict(origins: Vec<&str>) -> Self {
        Self {
            allow_origins: origins.iter().map(|s| s.to_string()).collect(),
            allow_credentials: true,
            ..Default::default()
        }
    }
}

/// Apply CORS headers to response
pub fn apply_cors(response: &mut NucleusResponse) {
    apply_cors_with_config(response, &CorsConfig::default());
}

/// Apply CORS with custom configuration
pub fn apply_cors_with_config(response: &mut NucleusResponse, config: &CorsConfig) {
    let headers = response.headers_mut();
    
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str(&config.allow_origins.join(", ")).unwrap_or(HeaderValue::from_static("*")),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_str(&config.allow_methods.join(", ")).unwrap_or(HeaderValue::from_static("GET, POST")),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_str(&config.allow_headers.join(", ")).unwrap_or(HeaderValue::from_static("Content-Type")),
    );
    
    if config.allow_credentials {
        headers.insert(
            "Access-Control-Allow-Credentials",
            HeaderValue::from_static("true"),
        );
    }
    
    headers.insert(
        "Access-Control-Max-Age",
        HeaderValue::from_str(&config.max_age.to_string()).unwrap_or(HeaderValue::from_static("86400")),
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// RATE LIMITING MIDDLEWARE
// ═══════════════════════════════════════════════════════════════════════════

use crate::fortress::RateLimiter;

/// Create rate limiting middleware
pub fn rate_limit_middleware(
    limiter: RateLimiter,
) -> impl Fn(NucleusRequest, NucleusNext) -> std::pin::Pin<Box<dyn std::future::Future<Output = NucleusResponse> + Send>> + Clone {
    move |request: NucleusRequest, next: NucleusNext| {
        let limiter = limiter.clone();
        Box::pin(async move {
            // Extract client IP from request
            let client_ip = request
                .headers()
                .get("X-Forwarded-For")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.split(',').next().unwrap_or("unknown"))
                .unwrap_or("unknown")
                .to_string();
            
            let result = limiter.check(&client_ip);
            
            if !result.allowed {
                warn!(
                    client_ip = %client_ip,
                    "Rate limit exceeded"
                );
                
                let mut response = Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(Body::from("Too Many Requests"))
                    .unwrap();
                
                result.apply_headers(&mut response);
                return response;
            }
            
            let mut response = next.run(request).await;
            result.apply_headers(&mut response);
            response
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cors_config_default() {
        let config = CorsConfig::default();
        assert!(config.allow_origins.contains(&"*".to_string()));
        assert!(!config.allow_credentials);
    }
    
    #[test]
    fn test_cors_config_strict() {
        let config = CorsConfig::strict(vec!["https://example.com"]);
        assert!(config.allow_origins.contains(&"https://example.com".to_string()));
        assert!(config.allow_credentials);
    }
}
