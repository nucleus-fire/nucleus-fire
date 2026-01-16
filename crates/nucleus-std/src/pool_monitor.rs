//! Nucleus Database Pool Monitor
//!
//! Connection pool visibility and health monitoring:
//! - Pool statistics (active, idle, waiting)
//! - Connection health checks
//! - Slow query detection
//! - Pool sizing recommendations
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::pool_monitor::PoolMonitor;
//!
//! // Create monitor from pool
//! let monitor = PoolMonitor::new(pool.clone());
//!
//! // Get pool stats
//! let stats = monitor.stats().await;
//! println!("Active: {}, Idle: {}", stats.active, stats.idle);
//!
//! // Check pool health
//! let health = monitor.health_check().await;
//! println!("Healthy: {}", health.is_healthy);
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// POOL STATISTICS
// ═══════════════════════════════════════════════════════════════════════════

/// Connection pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    /// Number of connections currently in use
    pub active: u32,
    /// Number of idle connections available
    pub idle: u32,
    /// Total pool capacity
    pub max_connections: u32,
    /// Number of tasks waiting for a connection
    pub pending_connections: u32,
    /// Pool utilization percentage
    pub utilization_percent: f64,
    /// Timestamp of stats collection
    pub collected_at: DateTime<Utc>,
}

impl PoolStats {
    /// Check if pool is under pressure (>80% utilized)
    pub fn is_under_pressure(&self) -> bool {
        self.utilization_percent > 80.0
    }
    
    /// Check if pool is exhausted (100% utilized)
    pub fn is_exhausted(&self) -> bool {
        self.active >= self.max_connections
    }
    
    /// Check if there are available connections
    pub fn has_available(&self) -> bool {
        self.idle > 0 || self.active < self.max_connections
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// POOL HEALTH
// ═══════════════════════════════════════════════════════════════════════════

/// Pool health status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PoolHealthStatus {
    /// Pool is healthy and responsive
    Healthy,
    /// Pool is functional but degraded
    Degraded,
    /// Pool is unhealthy
    Unhealthy,
}

/// Pool health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolHealth {
    /// Overall health status
    pub status: PoolHealthStatus,
    /// Whether the pool can serve requests
    pub is_healthy: bool,
    /// Connection test result
    pub connection_test_passed: bool,
    /// Connection test latency in ms
    pub connection_latency_ms: u64,
    /// Current pool stats
    pub stats: PoolStats,
    /// Issues detected
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Check timestamp
    pub checked_at: DateTime<Utc>,
}

impl PoolHealth {
    /// Get HTTP status code for this health check
    pub fn http_status(&self) -> u16 {
        match self.status {
            PoolHealthStatus::Healthy => 200,
            PoolHealthStatus::Degraded => 200,
            PoolHealthStatus::Unhealthy => 503,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// QUERY METRICS
// ═══════════════════════════════════════════════════════════════════════════

/// Metrics for a tracked query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetrics {
    /// Query pattern/identifier
    pub query: String,
    /// Number of executions
    pub execution_count: u64,
    /// Total execution time
    pub total_time_ms: u64,
    /// Average execution time
    pub avg_time_ms: u64,
    /// Maximum execution time
    pub max_time_ms: u64,
    /// Minimum execution time
    pub min_time_ms: u64,
    /// Number of slow executions (>100ms)
    pub slow_count: u64,
}

/// Query tracker for monitoring slow queries
#[derive(Debug, Clone, Default)]
struct QueryTracker {
    query: String,
    executions: u64,
    total_ms: u64,
    max_ms: u64,
    min_ms: u64,
    slow_count: u64,
}

impl QueryTracker {
    fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            min_ms: u64::MAX,
            ..Default::default()
        }
    }
    
    fn record(&mut self, duration_ms: u64, slow_threshold_ms: u64) {
        self.executions += 1;
        self.total_ms += duration_ms;
        self.max_ms = self.max_ms.max(duration_ms);
        self.min_ms = self.min_ms.min(duration_ms);
        if duration_ms > slow_threshold_ms {
            self.slow_count += 1;
        }
    }
    
    fn to_metrics(&self) -> QueryMetrics {
        QueryMetrics {
            query: self.query.clone(),
            execution_count: self.executions,
            total_time_ms: self.total_ms,
            avg_time_ms: if self.executions > 0 { self.total_ms / self.executions } else { 0 },
            max_time_ms: self.max_ms,
            min_time_ms: if self.min_ms == u64::MAX { 0 } else { self.min_ms },
            slow_count: self.slow_count,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// POOL MONITOR
// ═══════════════════════════════════════════════════════════════════════════

/// Database connection pool monitor
pub struct PoolMonitor {
    pool: SqlitePool,
    slow_query_threshold_ms: u64,
    queries: Arc<RwLock<std::collections::HashMap<String, QueryTracker>>>,
    max_connections: u32,
}

impl PoolMonitor {
    /// Create a new pool monitor
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            slow_query_threshold_ms: 100,
            queries: Arc::new(RwLock::new(std::collections::HashMap::new())),
            max_connections: 10, // Default SQLite pool size
        }
    }
    
    /// Set slow query threshold
    pub fn with_slow_threshold(mut self, threshold_ms: u64) -> Self {
        self.slow_query_threshold_ms = threshold_ms;
        self
    }
    
    /// Set max connections for accurate stats
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }
    
    /// Get current pool statistics
    pub fn stats(&self) -> PoolStats {
        let size = self.pool.size();
        let num_idle = self.pool.num_idle();
        let active = size - num_idle as u32;
        
        PoolStats {
            active,
            idle: num_idle as u32,
            max_connections: self.max_connections,
            pending_connections: 0, // SQLx doesn't expose this directly
            utilization_percent: (active as f64 / self.max_connections as f64) * 100.0,
            collected_at: Utc::now(),
        }
    }
    
    /// Perform a health check on the pool
    pub async fn health_check(&self) -> PoolHealth {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Get current stats
        let stats = self.stats();
        
        // Test connection with timing
        let start = Instant::now();
        let connection_test = sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await;
        let latency = start.elapsed().as_millis() as u64;
        
        let connection_passed = connection_test.is_ok();
        
        // Analyze stats
        if stats.is_exhausted() {
            issues.push("Pool is exhausted - no connections available".to_string());
            recommendations.push("Increase max_connections in pool configuration".to_string());
        } else if stats.is_under_pressure() {
            issues.push(format!("Pool utilization is high: {:.1}%", stats.utilization_percent));
            recommendations.push("Consider increasing pool size or optimizing queries".to_string());
        }
        
        if latency > 100 {
            issues.push(format!("High connection latency: {}ms", latency));
            recommendations.push("Check database server load and network".to_string());
        }
        
        if !connection_passed {
            issues.push("Connection test failed".to_string());
        }
        
        // Determine status
        let status = if !connection_passed {
            PoolHealthStatus::Unhealthy
        } else if stats.is_exhausted() || latency > 500 {
            PoolHealthStatus::Degraded
        } else {
            PoolHealthStatus::Healthy
        };
        
        PoolHealth {
            status,
            is_healthy: connection_passed && !stats.is_exhausted(),
            connection_test_passed: connection_passed,
            connection_latency_ms: latency,
            stats,
            issues,
            recommendations,
            checked_at: Utc::now(),
        }
    }
    
    /// Record a query execution for metrics
    pub async fn record_query(&self, query: &str, duration: Duration) {
        let duration_ms = duration.as_millis() as u64;
        let mut queries = self.queries.write().await;
        
        let tracker = queries
            .entry(query.to_string())
            .or_insert_with(|| QueryTracker::new(query));
        
        tracker.record(duration_ms, self.slow_query_threshold_ms);
    }
    
    /// Get metrics for all tracked queries
    pub async fn query_metrics(&self) -> Vec<QueryMetrics> {
        let queries = self.queries.read().await;
        queries.values().map(|t| t.to_metrics()).collect()
    }
    
    /// Get slow queries (queries that have exceeded threshold)
    pub async fn slow_queries(&self) -> Vec<QueryMetrics> {
        let queries = self.queries.read().await;
        queries
            .values()
            .filter(|t| t.slow_count > 0)
            .map(|t| t.to_metrics())
            .collect()
    }
    
    /// Clear query metrics
    pub async fn clear_metrics(&self) {
        let mut queries = self.queries.write().await;
        queries.clear();
    }
    
    /// Get pool sizing recommendation based on usage
    pub fn sizing_recommendation(&self) -> PoolSizingRecommendation {
        let stats = self.stats();
        
        let recommended_min = (stats.active as f64 * 1.2).ceil() as u32;
        let recommended_max = (stats.active as f64 * 2.0).ceil() as u32;
        
        let action = if stats.is_exhausted() {
            "INCREASE"
        } else if stats.utilization_percent < 20.0 && self.max_connections > 5 {
            "DECREASE"
        } else {
            "NONE"
        };
        
        PoolSizingRecommendation {
            current_max: self.max_connections,
            recommended_min: recommended_min.max(2),
            recommended_max: recommended_max.max(5),
            current_utilization: stats.utilization_percent,
            action: action.to_string(),
        }
    }
    
    /// Get the underlying pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

impl Clone for PoolMonitor {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            slow_query_threshold_ms: self.slow_query_threshold_ms,
            queries: Arc::clone(&self.queries),
            max_connections: self.max_connections,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// POOL SIZING RECOMMENDATION
// ═══════════════════════════════════════════════════════════════════════════

/// Pool sizing recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolSizingRecommendation {
    /// Current max connections
    pub current_max: u32,
    /// Recommended minimum
    pub recommended_min: u32,
    /// Recommended maximum  
    pub recommended_max: u32,
    /// Current utilization
    pub current_utilization: f64,
    /// Recommended action: INCREASE, DECREASE, or NONE
    pub action: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// DASHBOARD DATA
// ═══════════════════════════════════════════════════════════════════════════

/// Complete dashboard data for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDashboard {
    /// Pool health
    pub health: PoolHealth,
    /// Slow queries
    pub slow_queries: Vec<QueryMetrics>,
    /// Sizing recommendation
    pub sizing: PoolSizingRecommendation,
    /// Dashboard generated at
    pub generated_at: DateTime<Utc>,
}

impl PoolMonitor {
    /// Get complete dashboard data
    pub async fn dashboard(&self) -> PoolDashboard {
        PoolDashboard {
            health: self.health_check().await,
            slow_queries: self.slow_queries().await,
            sizing: self.sizing_recommendation(),
            generated_at: Utc::now(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    // ═══════════════════════════════════════════════════════════════════════
    // POOL STATS TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_pool_stats_under_pressure() {
        let stats = PoolStats {
            active: 9,
            idle: 1,
            max_connections: 10,
            pending_connections: 0,
            utilization_percent: 90.0,
            collected_at: Utc::now(),
        };
        
        assert!(stats.is_under_pressure());
        assert!(!stats.is_exhausted());
    }
    
    #[test]
    fn test_pool_stats_exhausted() {
        let stats = PoolStats {
            active: 10,
            idle: 0,
            max_connections: 10,
            pending_connections: 2,
            utilization_percent: 100.0,
            collected_at: Utc::now(),
        };
        
        assert!(stats.is_exhausted());
        assert!(!stats.has_available());
    }
    
    #[test]
    fn test_pool_stats_healthy() {
        let stats = PoolStats {
            active: 3,
            idle: 7,
            max_connections: 10,
            pending_connections: 0,
            utilization_percent: 30.0,
            collected_at: Utc::now(),
        };
        
        assert!(!stats.is_under_pressure());
        assert!(!stats.is_exhausted());
        assert!(stats.has_available());
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // POOL HEALTH TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_pool_health_http_status() {
        let stats = PoolStats {
            active: 0,
            idle: 10,
            max_connections: 10,
            pending_connections: 0,
            utilization_percent: 0.0,
            collected_at: Utc::now(),
        };
        
        let healthy = PoolHealth {
            status: PoolHealthStatus::Healthy,
            is_healthy: true,
            connection_test_passed: true,
            connection_latency_ms: 5,
            stats: stats.clone(),
            issues: vec![],
            recommendations: vec![],
            checked_at: Utc::now(),
        };
        
        assert_eq!(healthy.http_status(), 200);
        
        let unhealthy = PoolHealth {
            status: PoolHealthStatus::Unhealthy,
            is_healthy: false,
            connection_test_passed: false,
            connection_latency_ms: 0,
            stats,
            issues: vec!["Connection failed".to_string()],
            recommendations: vec![],
            checked_at: Utc::now(),
        };
        
        assert_eq!(unhealthy.http_status(), 503);
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // QUERY TRACKER TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_query_tracker() {
        let mut tracker = QueryTracker::new("SELECT * FROM users");
        
        tracker.record(10, 100);
        tracker.record(20, 100);
        tracker.record(150, 100); // Slow
        
        let metrics = tracker.to_metrics();
        
        assert_eq!(metrics.execution_count, 3);
        assert_eq!(metrics.total_time_ms, 180);
        assert_eq!(metrics.avg_time_ms, 60);
        assert_eq!(metrics.max_time_ms, 150);
        assert_eq!(metrics.min_time_ms, 10);
        assert_eq!(metrics.slow_count, 1);
    }
    
    #[test]
    fn test_query_tracker_empty() {
        let tracker = QueryTracker::new("SELECT 1");
        let metrics = tracker.to_metrics();
        
        assert_eq!(metrics.execution_count, 0);
        assert_eq!(metrics.avg_time_ms, 0);
        assert_eq!(metrics.min_time_ms, 0);
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // SIZING RECOMMENDATION TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_sizing_recommendation_serializable() {
        let rec = PoolSizingRecommendation {
            current_max: 10,
            recommended_min: 5,
            recommended_max: 15,
            current_utilization: 50.0,
            action: "NONE".to_string(),
        };
        
        let json = serde_json::to_string(&rec).unwrap();
        assert!(json.contains("current_max"));
        assert!(json.contains("10"));
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // POOL MONITOR TESTS (require DB)
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_pool_monitor_creation() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool)
            .with_slow_threshold(50)
            .with_max_connections(20);
        
        assert_eq!(monitor.slow_query_threshold_ms, 50);
        assert_eq!(monitor.max_connections, 20);
    }
    
    #[tokio::test]
    async fn test_pool_monitor_stats() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool);
        
        let stats = monitor.stats();
        assert!(stats.max_connections > 0);
        assert!(stats.utilization_percent >= 0.0);
    }
    
    #[tokio::test]
    async fn test_pool_monitor_health_check() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool);
        
        let health = monitor.health_check().await;
        
        assert!(health.connection_test_passed);
        assert!(health.is_healthy);
        assert_eq!(health.status, PoolHealthStatus::Healthy);
        assert!(health.connection_latency_ms < 1000);
    }
    
    #[tokio::test]
    async fn test_pool_monitor_record_query() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool);
        
        monitor.record_query("SELECT * FROM users", Duration::from_millis(50)).await;
        monitor.record_query("SELECT * FROM users", Duration::from_millis(150)).await;
        monitor.record_query("INSERT INTO posts", Duration::from_millis(30)).await;
        
        let metrics = monitor.query_metrics().await;
        assert_eq!(metrics.len(), 2);
        
        let slow = monitor.slow_queries().await;
        assert_eq!(slow.len(), 1);
    }
    
    #[tokio::test]
    async fn test_pool_monitor_clear_metrics() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool);
        
        monitor.record_query("SELECT 1", Duration::from_millis(10)).await;
        assert_eq!(monitor.query_metrics().await.len(), 1);
        
        monitor.clear_metrics().await;
        assert_eq!(monitor.query_metrics().await.len(), 0);
    }
    
    #[tokio::test]
    async fn test_pool_monitor_dashboard() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool);
        
        let dashboard = monitor.dashboard().await;
        
        assert!(dashboard.health.is_healthy);
        assert!(dashboard.sizing.current_max > 0);
    }
    
    #[tokio::test]
    async fn test_pool_monitor_clone() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor1 = PoolMonitor::new(pool);
        let monitor2 = monitor1.clone();
        
        monitor1.record_query("SELECT 1", Duration::from_millis(10)).await;
        
        // Clone shares metrics
        assert_eq!(monitor2.query_metrics().await.len(), 1);
    }
    
    #[tokio::test]
    async fn test_pool_health_serializable() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let monitor = PoolMonitor::new(pool);
        
        let health = monitor.health_check().await;
        let json = serde_json::to_string(&health).unwrap();
        
        assert!(json.contains("is_healthy"));
        assert!(json.contains("connection_latency_ms"));
    }
}
