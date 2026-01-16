# Database Pool Monitor Guide

Nucleus provides full visibility into your database connection pool health and performance.

## Quick Start

```rust
use nucleus_std::pool_monitor::PoolMonitor;

// Create monitor from pool
let monitor = PoolMonitor::new(pool.clone())
    .with_max_connections(20)
    .with_slow_threshold(100); // 100ms

// Get current stats
let stats = monitor.stats();
println!("Active: {}, Idle: {}", stats.active, stats.idle);

// Health check
let health = monitor.health_check().await;
if !health.is_healthy {
    eprintln!("Pool issues: {:?}", health.issues);
}
```

## Pool Statistics

```rust
let stats = monitor.stats();

// Connection counts
println!("Active connections: {}", stats.active);
println!("Idle connections: {}", stats.idle);
println!("Max capacity: {}", stats.max_connections);

// Utilization
println!("Utilization: {:.1}%", stats.utilization_percent);

// State checks
if stats.is_exhausted() {
    alert("No connections available!");
}
if stats.is_under_pressure() {
    warn("Pool utilization > 80%");
}
```

## Health Checks

```rust
let health = monitor.health_check().await;

// Status: Healthy, Degraded, or Unhealthy
match health.status {
    PoolHealthStatus::Healthy => println!("All good"),
    PoolHealthStatus::Degraded => println!("Issues: {:?}", health.issues),
    PoolHealthStatus::Unhealthy => alert("Pool down!"),
}

// Connection test
if health.connection_test_passed {
    println!("DB responsive in {}ms", health.connection_latency_ms);
}

// Recommendations
for rec in &health.recommendations {
    println!("Suggestion: {}", rec);
}

// HTTP status (for health endpoints)
let status_code = health.http_status(); // 200 or 503
```

## Query Metrics

Track query performance over time:

```rust
// Record a query execution
monitor.record_query(
    "SELECT * FROM users WHERE id = ?",
    query_duration
).await;

// Get all query metrics
let metrics = monitor.query_metrics().await;
for m in metrics {
    println!("{}: {} executions, avg {}ms, {} slow",
        m.query, m.execution_count, m.avg_time_ms, m.slow_count);
}

// Get only slow queries
let slow = monitor.slow_queries().await;
for s in slow {
    println!("SLOW: {} - max {}ms", s.query, s.max_time_ms);
}

// Clear metrics
monitor.clear_metrics().await;
```

## Pool Sizing Recommendations

```rust
let sizing = monitor.sizing_recommendation();

println!("Current max: {}", sizing.current_max);
println!("Recommended min: {}", sizing.recommended_min);
println!("Recommended max: {}", sizing.recommended_max);
println!("Action needed: {}", sizing.action);

match sizing.action.as_str() {
    "INCREASE" => println!("Increase pool size!"),
    "DECREASE" => println!("Pool is oversized"),
    "NONE" => println!("Pool size is appropriate"),
    _ => {}
}
```

## Dashboard Endpoint

Get complete dashboard data in one call:

```rust
let dashboard = monitor.dashboard().await;

// Contains:
// - health: PoolHealth
// - slow_queries: Vec<QueryMetrics>
// - sizing: PoolSizingRecommendation
// - generated_at: DateTime

// Serialize to JSON for API
let json = serde_json::to_string(&dashboard)?;
```

## Axum Health Endpoint

```rust
use axum::{routing::get, Json, extract::Extension};

async fn pool_health(
    Extension(monitor): Extension<PoolMonitor>
) -> (StatusCode, Json<PoolHealth>) {
    let health = monitor.health_check().await;
    let code = StatusCode::from_u16(health.http_status()).unwrap();
    (code, Json(health))
}

async fn pool_dashboard(
    Extension(monitor): Extension<PoolMonitor>
) -> Json<PoolDashboard> {
    Json(monitor.dashboard().await)
}

let app = Router::new()
    .route("/health/db", get(pool_health))
    .route("/admin/pool", get(pool_dashboard))
    .layer(Extension(monitor));
```

## Kubernetes Integration

```yaml
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: app
    readinessProbe:
      httpGet:
        path: /health/db
        port: 8080
      periodSeconds: 10
      failureThreshold: 3
```

## JSON Response Format

```json
{
  "health": {
    "status": "Healthy",
    "is_healthy": true,
    "connection_test_passed": true,
    "connection_latency_ms": 5,
    "stats": {
      "active": 3,
      "idle": 7,
      "max_connections": 10,
      "utilization_percent": 30.0
    },
    "issues": [],
    "recommendations": []
  },
  "slow_queries": [
    {
      "query": "SELECT * FROM orders JOIN users...",
      "execution_count": 150,
      "avg_time_ms": 45,
      "max_time_ms": 250,
      "slow_count": 12
    }
  ],
  "sizing": {
    "current_max": 10,
    "recommended_min": 5,
    "recommended_max": 15,
    "action": "NONE"
  }
}
```

## Best Practices

1. **Set accurate max_connections** matching your pool config
2. **Monitor utilization** and alert at 80%
3. **Track slow queries** to find optimization targets
4. **Use health checks** for Kubernetes probes
5. **Size pools appropriately** based on recommendations
