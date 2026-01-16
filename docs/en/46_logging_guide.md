# Structured Logging Guide

Nucleus Logging provides production-ready structured logging with tracing integration.

## Quick Start

```rust
use nucleus_std::logging::{init as init_logging, LogConfig, LogLevel, LogFormat};

// Initialize logging
init_logging(LogConfig::default());

// Use standard tracing macros
tracing::info!("Server started on port 8080");
tracing::warn!(user_id = 123, "Login attempt failed");
tracing::error!(error = ?err, "Database connection failed");
```

## Configuration

### Basic Setup

```rust
let config = LogConfig::new()
    .with_level(LogLevel::Info)
    .with_format(LogFormat::Pretty);

init_logging(config);
```

### Log Levels

```rust
// Development (verbose)
LogConfig::new().with_level(LogLevel::Debug)

// Production (minimal)
LogConfig::new().with_level(LogLevel::Warn)

// Available levels (least to most verbose):
// Error -> Warn -> Info -> Debug -> Trace
```

### Output Formats

```rust
// Pretty (human-readable, good for development)
LogConfig::new().with_format(LogFormat::Pretty)
// Output: 2024-01-15T10:30:45Z INFO mycrate::server Server started port=8080

// JSON (machine-readable, good for log aggregation)
LogConfig::new().with_format(LogFormat::Json)
// Output: {"timestamp":"2024-01-15T10:30:45Z","level":"INFO","message":"Server started","port":8080}

// Compact (minimal)
LogConfig::new().with_format(LogFormat::Compact)
```

### Environment-Based Config

```rust
let config = if cfg!(debug_assertions) {
    LogConfig::development()  // Debug level, Pretty format
} else {
    LogConfig::production()   // Info level, JSON format
};

init_logging(config);
```

## Logging Macros

### Basic Logging

```rust
use tracing::{trace, debug, info, warn, error};

trace!("Very verbose debugging info");
debug!("Debugging info");
info!("General information");
warn!("Warning message");
error!("Error message");
```

### Structured Fields

```rust
// Named fields
info!(user_id = 123, action = "login", "User logged in");

// Debug-formatted values
error!(error = ?some_error, "Operation failed");

// Display-formatted values
info!(name = %user.name, "User created");
```

### Spans (Request Tracing)

```rust
use tracing::{span, Level};

// Create a span for a request
let span = span!(Level::INFO, "request", 
    request_id = %uuid::Uuid::new_v4(),
    method = "GET",
    path = "/users"
);

let _guard = span.enter();

// All logs in this scope include span context
info!("Processing request");
debug!("Fetching from database");
info!("Request completed");
```

## Practical Examples

### Request Logging Middleware

```rust
use axum::{middleware::Next, extract::Request, response::Response};
use tracing::{info, info_span, Instrument};

async fn logging_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_id = uuid::Uuid::new_v4();
    
    let span = info_span!(
        "http_request",
        %request_id,
        method = %method,
        path = %uri.path(),
    );
    
    let start = std::time::Instant::now();
    
    let response = next.run(request).instrument(span).await;
    
    let duration = start.elapsed();
    let status = response.status().as_u16();
    
    info!(
        %request_id,
        %status,
        duration_ms = duration.as_millis(),
        "Request completed"
    );
    
    response
}
```

### Database Query Logging

```rust
async fn execute_query<T>(query: &str, params: &[Value]) -> Result<T, Error> {
    let span = tracing::info_span!("db_query", query = %query);
    let _guard = span.enter();
    
    let start = std::time::Instant::now();
    
    let result = db.execute(query, params).await;
    
    let duration = start.elapsed();
    
    match &result {
        Ok(_) => {
            tracing::debug!(
                duration_ms = duration.as_millis(),
                "Query executed successfully"
            );
        }
        Err(e) => {
            tracing::error!(
                duration_ms = duration.as_millis(),
                error = ?e,
                "Query failed"
            );
        }
    }
    
    result
}
```

### Error Context

```rust
use tracing::error;

fn process_payment(order_id: i32, amount: f64) -> Result<(), PaymentError> {
    match gateway.charge(amount) {
        Ok(transaction_id) => {
            tracing::info!(
                order_id,
                amount,
                %transaction_id,
                "Payment successful"
            );
            Ok(())
        }
        Err(e) => {
            tracing::error!(
                order_id,
                amount,
                error = ?e,
                "Payment failed"
            );
            Err(PaymentError::ChargeFailed(e))
        }
    }
}
```

### Audit Logging

```rust
fn audit_log(action: &str, user_id: i32, resource: &str, details: &str) {
    tracing::info!(
        target: "audit",  // Separate log target
        action,
        user_id,
        resource,
        details,
        timestamp = %chrono::Utc::now(),
        "Audit event"
    );
}

// Usage
audit_log("delete", 123, "users/456", "Deleted user account");
```

## Output Examples

### Pretty Format

```
2024-01-15T10:30:45.123Z  INFO request{id=abc123 method=GET path=/users}: nucleus::handler Request started
2024-01-15T10:30:45.125Z DEBUG request{id=abc123 method=GET path=/users}: nucleus::db Executing query
2024-01-15T10:30:45.145Z  INFO request{id=abc123 method=GET path=/users}: nucleus::handler Request completed status=200 duration_ms=22
```

### JSON Format

```json
{"timestamp":"2024-01-15T10:30:45.123Z","level":"INFO","target":"nucleus::handler","message":"Request started","request_id":"abc123","method":"GET","path":"/users"}
{"timestamp":"2024-01-15T10:30:45.125Z","level":"DEBUG","target":"nucleus::db","message":"Executing query","request_id":"abc123"}
{"timestamp":"2024-01-15T10:30:45.145Z","level":"INFO","target":"nucleus::handler","message":"Request completed","request_id":"abc123","status":200,"duration_ms":22}
```

## Log Aggregation

### Sending to External Services

```rust
// Logs to stdout as JSON, pipe to log aggregator
LogConfig::new()
    .with_format(LogFormat::Json)
    .with_level(LogLevel::Info);

// In production, use:
// ./myapp 2>&1 | vector  # Vector
// ./myapp 2>&1 | fluent-bit  # Fluent Bit
```

### Structured Fields for Querying

```rust
// Add consistent fields for easy querying
tracing::info!(
    service = "payment-api",
    environment = "production",
    version = env!("CARGO_PKG_VERSION"),
    user_id,
    order_id,
    amount,
    "Payment processed"
);
```

## Best Practices

1. **Use structured fields** - Easier to query than parsing strings
2. **Include request IDs** - Trace requests across services
3. **Use appropriate levels** - Debug for dev, Info/Warn for prod
4. **Add context with spans** - Group related log entries
5. **Use JSON in production** - Machine-parseable for log aggregation
6. **Don't log sensitive data** - Never log passwords, tokens, PII
