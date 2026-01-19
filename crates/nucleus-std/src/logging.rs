//! Nucleus Logging Module
//!
//! Provides structured logging with `tracing` integration:
//! - Multiple output formats (pretty, JSON, compact)
//! - Environment-based filtering
//! - Request tracing spans
//! - Performance timing
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::logging::{init, LogConfig, LogLevel, LogFormat};
//! use nucleus_std::logging::{info, warn, error, debug, span, Level};
//!
//! // Initialize logging
//! init(LogConfig::default());
//!
//! // Use macros
//! info!(user_id = %user.id, "User logged in");
//! error!(error = ?err, "Database connection failed");
//!
//! // Create spans for request tracing
//! let span = span!(Level::INFO, "request", method = "GET", path = "/api/users");
//! let _guard = span.enter();
//! ```

pub use tracing::{debug, error, info, span, trace, warn, Level};

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Log level configuration
#[derive(Debug, Clone, Copy, Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
            LogLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            LogLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
            LogLevel::Warn => tracing_subscriber::filter::LevelFilter::WARN,
            LogLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
        }
    }
}

/// Log output format
#[derive(Debug, Clone, Copy, Default)]
pub enum LogFormat {
    /// Human-readable colorful output (default)
    #[default]
    Pretty,
    /// JSON output for log aggregators
    Json,
    /// Compact single-line output
    Compact,
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Minimum log level
    pub level: LogLevel,
    /// Output format
    pub format: LogFormat,
    /// Include target module in output
    pub include_target: bool,
    /// Include file and line number
    pub include_file: bool,
    /// Include timestamps
    pub include_time: bool,
    /// Application name for logs
    pub app_name: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            include_target: true,
            include_file: false,
            include_time: true,
            app_name: "nucleus".to_string(),
        }
    }
}

impl LogConfig {
    /// Create development config (debug level, pretty format)
    pub fn development() -> Self {
        Self {
            level: LogLevel::Debug,
            format: LogFormat::Pretty,
            include_file: true,
            ..Default::default()
        }
    }

    /// Create production config (info level, JSON format)
    pub fn production() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Json,
            include_target: true,
            include_file: false,
            include_time: true,
            app_name: "nucleus".to_string(),
        }
    }

    /// Set log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set output format
    pub fn format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Set application name
    pub fn app_name(mut self, name: &str) -> Self {
        self.app_name = name.to_string();
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// INITIALIZATION
// ═══════════════════════════════════════════════════════════════════════════

/// Initialize the logging system with the given configuration
pub fn init(config: LogConfig) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("{}", tracing::Level::from(config.level))));

    match config.format {
        LogFormat::Json => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .json()
                        .with_target(config.include_target)
                        .with_file(config.include_file)
                        .with_line_number(config.include_file),
                )
                .try_init()
                .ok();
        }
        LogFormat::Pretty => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .pretty()
                        .with_target(config.include_target)
                        .with_file(config.include_file)
                        .with_line_number(config.include_file),
                )
                .try_init()
                .ok();
        }
        LogFormat::Compact => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .compact()
                        .with_target(config.include_target)
                        .with_file(config.include_file)
                        .with_line_number(config.include_file),
                )
                .try_init()
                .ok();
        }
    }
}

/// Initialize with default development settings
pub fn init_dev() {
    init(LogConfig::development());
}

/// Initialize with production settings
pub fn init_prod() {
    init(LogConfig::production());
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST TRACING
// ═══════════════════════════════════════════════════════════════════════════

/// Create a span for HTTP request tracing
#[macro_export]
macro_rules! request_span {
    ($method:expr, $path:expr) => {
        tracing::span!(
            tracing::Level::INFO,
            "http_request",
            method = %$method,
            path = %$path
        )
    };
    ($method:expr, $path:expr, $($field:tt)*) => {
        tracing::span!(
            tracing::Level::INFO,
            "http_request",
            method = %$method,
            path = %$path,
            $($field)*
        )
    };
}

/// Log a request completion with timing
pub fn log_request(method: &str, path: &str, status: u16, duration_ms: u128) {
    if status >= 500 {
        error!(
            method = %method,
            path = %path,
            status = status,
            duration_ms = duration_ms,
            "Request failed"
        );
    } else if status >= 400 {
        warn!(
            method = %method,
            path = %path,
            status = status,
            duration_ms = duration_ms,
            "Request error"
        );
    } else {
        info!(
            method = %method,
            path = %path,
            status = status,
            duration_ms = duration_ms,
            "Request completed"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TIMING HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Timer for measuring operation duration
pub struct Timer {
    start: std::time::Instant,
    operation: String,
}

impl Timer {
    /// Start a new timer
    pub fn start(operation: &str) -> Self {
        Self {
            start: std::time::Instant::now(),
            operation: operation.to_string(),
        }
    }

    /// Get elapsed duration in milliseconds
    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    /// Log elapsed time at info level
    pub fn log_elapsed(&self) {
        info!(
            operation = %self.operation,
            duration_ms = self.elapsed_ms(),
            "Operation completed"
        );
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        debug!(
            operation = %self.operation,
            duration_ms = self.elapsed_ms(),
            "Timer dropped"
        );
    }
}

/// Measure and log an async operation
pub async fn timed<T, F, Fut>(operation: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let timer = Timer::start(operation);
    let result = f().await;
    timer.log_elapsed();
    result
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_config_defaults() {
        let config = LogConfig::default();
        assert!(matches!(config.level, LogLevel::Info));
        assert!(matches!(config.format, LogFormat::Pretty));
    }

    #[test]
    fn test_log_config_builder() {
        let config = LogConfig::default()
            .level(LogLevel::Debug)
            .format(LogFormat::Json)
            .app_name("test");

        assert!(matches!(config.level, LogLevel::Debug));
        assert!(matches!(config.format, LogFormat::Json));
        assert_eq!(config.app_name, "test");
    }

    #[test]
    fn test_timer() {
        let timer = Timer::start("test_op");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10);
    }
}
