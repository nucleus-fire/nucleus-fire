//! Request Profiler Middleware
//!
//! Automatically profiles requests in development mode, tracking:
//! - Request timing
//! - Response status
//! - Database queries (when integrated)
//! - Slow request warnings

#![allow(clippy::type_complexity)]

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Profiler configuration
#[derive(Debug, Clone)]
pub struct ProfilerConfig {
    /// Enable profiling (defaults to true in dev)
    pub enabled: bool,
    /// Slow request threshold in milliseconds
    pub slow_threshold_ms: u64,
    /// Log format
    pub format: LogFormat,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enabled: cfg!(debug_assertions),
            slow_threshold_ms: 100,
            format: LogFormat::Pretty,
        }
    }
}

/// Log output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable with colors
    Pretty,
    /// JSON output for log aggregators
    Json,
    /// Single line compact
    Compact,
}

// ═══════════════════════════════════════════════════════════════════════════
// PROFILER MIDDLEWARE
// ═══════════════════════════════════════════════════════════════════════════

/// Profile a request with default configuration
pub async fn profile(request: Request, next: Next) -> Response {
    profile_with_config(request, next, ProfilerConfig::default()).await
}

/// Profile a request with custom configuration
pub async fn profile_with_config(
    request: Request,
    next: Next,
    config: ProfilerConfig,
) -> Response {
    if !config.enabled {
        return next.run(request).await;
    }

    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();
    
    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    
    let status = response.status();
    let is_slow = duration.as_millis() > config.slow_threshold_ms as u128;
    
    match config.format {
        LogFormat::Pretty => log_pretty(method.as_ref(), &path, status, duration_ms, is_slow),
        LogFormat::Json => log_json(method.as_ref(), &path, status, duration_ms, is_slow),
        LogFormat::Compact => log_compact(method.as_ref(), &path, status, duration_ms, is_slow),
    }

    let mut response = response;
    response.headers_mut().insert(
        "x-response-time", 
        format!("{:.2}ms", duration_ms).parse().unwrap_or(axum::http::HeaderValue::from_static("0ms"))
    );
    
    response
}

// ═══════════════════════════════════════════════════════════════════════════
// LOGGING FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

fn status_color(status: StatusCode) -> &'static str {
    if status.is_success() {
        "\x1b[32m"  // Green
    } else if status.is_redirection() {
        "\x1b[36m"  // Cyan
    } else if status.is_client_error() {
        "\x1b[33m"  // Yellow
    } else {
        "\x1b[31m"  // Red
    }
}

fn log_pretty(method: &str, path: &str, status: StatusCode, duration_ms: f64, is_slow: bool) {
    let status_clr = status_color(status);
    let time_clr = if is_slow { "\x1b[31m" } else { "\x1b[90m" };
    let slow_marker = if is_slow { " ⚠️ SLOW" } else { "" };
    
    println!(
        "  {status_clr}⬢\x1b[0m {} {} {status_clr}{}\x1b[0m {time_clr}{:.2}ms\x1b[0m{slow_marker}",
        method, path, status.as_u16(), duration_ms
    );
}

fn log_json(method: &str, path: &str, status: StatusCode, duration_ms: f64, is_slow: bool) {
    println!(
        r#"{{"method":"{}","path":"{}","status":{},"duration_ms":{:.2},"slow":{}}}"#,
        method, path, status.as_u16(), duration_ms, is_slow
    );
}

fn log_compact(method: &str, path: &str, status: StatusCode, duration_ms: f64, is_slow: bool) {
    let marker = if is_slow { "!" } else { "" };
    println!("{}{} {} {} {:.0}ms", marker, method, path, status.as_u16(), duration_ms);
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_config_default() {
        let config = ProfilerConfig::default();
        assert!(config.enabled);
        assert_eq!(config.slow_threshold_ms, 100);
        assert_eq!(config.format, LogFormat::Pretty);
    }

    #[test]
    fn test_status_color() {
        assert_eq!(status_color(StatusCode::OK), "\x1b[32m");
        assert_eq!(status_color(StatusCode::NOT_FOUND), "\x1b[33m");
        assert_eq!(status_color(StatusCode::INTERNAL_SERVER_ERROR), "\x1b[31m");
    }

    #[test]
    fn test_log_formats() {
        // Just ensure they don't panic
        log_pretty("GET", "/test", StatusCode::OK, 50.0, false);
        log_json("POST", "/api", StatusCode::CREATED, 10.0, false);
        log_compact("GET", "/", StatusCode::OK, 5.0, false);
    }
}
