# Health Check Guide

Nucleus provides production-ready health checks for monitoring liveness, readiness, and component health.

## Quick Start

```rust
use nucleus_std::health::{HealthChecker, HealthStatus};

let checker = HealthChecker::new().with_version("1.0.0");

// Add checks
checker.add_check("database", || async {
    match db.ping().await {
        Ok(_) => HealthStatus::Healthy,
        Err(e) => HealthStatus::Unhealthy(e.to_string()),
    }
}).await;

// Check health
let report = checker.check_all().await;
println!("Status: {:?}", report.status);
```

## Kubernetes Integration

Health checks are designed for Kubernetes probes:

```rust
// Liveness: Is the app running?
// GET /health/live
let report = checker.liveness();
// Returns 200 if app is running (always healthy)

// Readiness: Can the app serve traffic?
// GET /health/ready
let report = checker.readiness().await;
// Returns 200 if all critical components are healthy
```

## Health Status

```rust
pub enum HealthStatus {
    Healthy,              // Component is functioning normally
    Degraded(String),     // Component is impaired but functional
    Unhealthy(String),    // Component is non-functional
    Unknown(String),      // Status unknown (timeout, error)
}

// Status helpers
status.is_healthy() // true only for Healthy
status.is_ok()      // true for Healthy or Degraded
```

## Adding Health Checks

### Critical Checks

Failures make the application "not ready":

```rust
checker.add_check("database", || async {
    HealthStatus::Healthy
}).await;
```

### Optional Checks

Failures are reported but don't affect readiness:

```rust
checker.add_optional_check("external_api", || async {
    // Non-critical dependency
    HealthStatus::Healthy
}).await;
```

## Common Checks

### Database

```rust
checker.add_check("database", {
    let pool = pool.clone();
    move || {
        let p = pool.clone();
        async move {
            match sqlx::query("SELECT 1").execute(&p).await {
                Ok(_) => HealthStatus::Healthy,
                Err(e) => HealthStatus::Unhealthy(format!("DB error: {}", e)),
            }
        }
    }
}).await;
```

### Redis

```rust
checker.add_check("redis", {
    let client = redis_client.clone();
    move || {
        let c = client.clone();
        async move {
            match c.ping().await {
                Ok(_) => HealthStatus::Healthy,
                Err(e) => HealthStatus::Unhealthy(format!("Redis error: {}", e)),
            }
        }
    }
}).await;
```

### External API

```rust
checker.add_optional_check("stripe_api", || async {
    match reqwest::get("https://api.stripe.com/health").await {
        Ok(r) if r.status().is_success() => HealthStatus::Healthy,
        Ok(r) => HealthStatus::Degraded(format!("Status: {}", r.status())),
        Err(e) => HealthStatus::Unhealthy(e.to_string()),
    }
}).await;
```

### Latency Check

```rust
checker.add_check("database_latency", {
    let pool = pool.clone();
    move || {
        let p = pool.clone();
        async move {
            let start = std::time::Instant::now();
            match sqlx::query("SELECT 1").execute(&p).await {
                Ok(_) => {
                    let latency = start.elapsed().as_millis();
                    if latency > 500 {
                        HealthStatus::Degraded(format!("High latency: {}ms", latency))
                    } else {
                        HealthStatus::Healthy
                    }
                }
                Err(e) => HealthStatus::Unhealthy(e.to_string()),
            }
        }
    }
}).await;
```

## Configuration

```rust
let checker = HealthChecker::new()
    .with_version("1.2.3")           // App version in reports
    .with_timeout(Duration::from_secs(5)); // Check timeout
```

## Health Report

```rust
let report = checker.check_all().await;

// Overall status
println!("Status: {:?}", report.status);

// Version
println!("Version: {:?}", report.version);

// Uptime
println!("Uptime: {} seconds", report.uptime_secs.unwrap_or(0));

// Duration
println!("Check took: {}ms", report.duration_ms);

// Individual components
for check in &report.components {
    println!("{}: {:?} ({}ms)", check.name, check.status, check.duration_ms);
}

// HTTP response code
let status_code = report.http_status(); // 200 or 503
```

## Axum Integration

```rust
use axum::{Router, routing::get, Json};

async fn health_live(checker: Extension<HealthChecker>) -> Json<HealthReport> {
    Json(checker.liveness())
}

async fn health_ready(checker: Extension<HealthChecker>) -> (StatusCode, Json<HealthReport>) {
    let report = checker.readiness().await;
    let code = StatusCode::from_u16(report.http_status()).unwrap();
    (code, Json(report))
}

let app = Router::new()
    .route("/health/live", get(health_live))
    .route("/health/ready", get(health_ready))
    .layer(Extension(checker));
```

## JSON Response

```json
{
  "status": "Healthy",
  "components": [
    {
      "name": "database",
      "status": "Healthy",
      "duration_ms": 5,
      "checked_at": "2024-01-15T10:30:00Z"
    },
    {
      "name": "redis",
      "status": "Degraded",
      "duration_ms": 150,
      "checked_at": "2024-01-15T10:30:00Z"
    }
  ],
  "timestamp": "2024-01-15T10:30:00Z",
  "duration_ms": 155,
  "version": "1.2.3",
  "uptime_secs": 86400
}
```

## Kubernetes Configuration

```yaml
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: app
    livenessProbe:
      httpGet:
        path: /health/live
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 10
    readinessProbe:
      httpGet:
        path: /health/ready
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 5
```

## Best Practices

1. **Keep liveness simple** - just verify the process is running
2. **Check critical deps in readiness** - DB, cache, etc.
3. **Set appropriate timeouts** - don't block deploys
4. **Use optional checks** for non-critical services
5. **Monitor check latency** - slow checks indicate problems
6. **Include version** - helps with rollout verification
