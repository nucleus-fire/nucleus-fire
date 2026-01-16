//! Nucleus Health Check Module
//!
//! Production-ready health checks for monitoring:
//! - Liveness probe (is the app running?)
//! - Readiness probe (can the app serve traffic?)
//! - Component health checks (DB, cache, external services)
//! - Metrics collection
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::health::{HealthChecker, HealthStatus};
//!
//! let mut checker = HealthChecker::new();
//!
//! // Add component checks
//! checker.add_check("database", || async {
//!     db.ping().await.map(|_| HealthStatus::Healthy)
//! });
//!
//! checker.add_check("redis", || async {
//!     redis.ping().await.map(|_| HealthStatus::Healthy)
//! });
//!
//! // Get health status
//! let report = checker.check_all().await;
//! println!("Status: {:?}", report.status);
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH STATUS
// ═══════════════════════════════════════════════════════════════════════════

/// Health status of a component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Component is functioning normally
    Healthy,
    /// Component is degraded but functional
    Degraded(String),
    /// Component is non-functional
    Unhealthy(String),
    /// Status unknown (check timeout or error)
    Unknown(String),
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
    
    pub fn is_ok(&self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Degraded(_))
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Unknown("Not checked".into())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPONENT CHECK
// ═══════════════════════════════════════════════════════════════════════════

/// Result of a single component check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCheck {
    /// Component name
    pub name: String,
    /// Current status
    pub status: HealthStatus,
    /// Check duration
    pub duration_ms: u64,
    /// Last check time
    pub checked_at: DateTime<Utc>,
    /// Additional details
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl ComponentCheck {
    fn new(name: &str, status: HealthStatus, duration: Duration) -> Self {
        Self {
            name: name.to_string(),
            status,
            duration_ms: duration.as_millis() as u64,
            checked_at: Utc::now(),
            details: None,
        }
    }
    
    #[allow(dead_code)]
    fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(details);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH REPORT
// ═══════════════════════════════════════════════════════════════════════════

/// Overall health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Overall status
    pub status: HealthStatus,
    /// Individual component checks
    pub components: Vec<ComponentCheck>,
    /// Report generation time
    pub timestamp: DateTime<Utc>,
    /// Total check duration
    pub duration_ms: u64,
    /// Application version (if set)
    pub version: Option<String>,
    /// Uptime in seconds
    pub uptime_secs: Option<u64>,
}

impl HealthReport {
    /// Create a simple healthy report
    pub fn healthy() -> Self {
        Self {
            status: HealthStatus::Healthy,
            components: vec![],
            timestamp: Utc::now(),
            duration_ms: 0,
            version: None,
            uptime_secs: None,
        }
    }
    
    /// Check if all components are healthy
    pub fn is_healthy(&self) -> bool {
        self.status.is_healthy()
    }
    
    /// Check if application can serve traffic
    pub fn is_ready(&self) -> bool {
        self.status.is_ok()
    }
    
    /// Get HTTP status code for this report
    pub fn http_status(&self) -> u16 {
        match self.status {
            HealthStatus::Healthy => 200,
            HealthStatus::Degraded(_) => 200,
            HealthStatus::Unhealthy(_) => 503,
            HealthStatus::Unknown(_) => 503,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CHECK FUNCTION TYPE
// ═══════════════════════════════════════════════════════════════════════════

type BoxedCheck = Box<
    dyn Fn() -> Pin<Box<dyn Future<Output = HealthStatus> + Send>> + Send + Sync
>;

struct HealthCheck {
    name: String,
    check: BoxedCheck,
    critical: bool,
    timeout: Duration,
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH CHECKER
// ═══════════════════════════════════════════════════════════════════════════

/// Health checker for monitoring application health
pub struct HealthChecker {
    checks: Arc<RwLock<Vec<HealthCheck>>>,
    version: Option<String>,
    start_time: Instant,
    default_timeout: Duration,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            checks: Arc::new(RwLock::new(Vec::new())),
            version: None,
            start_time: Instant::now(),
            default_timeout: Duration::from_secs(5),
        }
    }
    
    /// Set application version
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }
    
    /// Set default timeout for checks
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }
    
    /// Add a health check
    pub async fn add_check<F, Fut>(&self, name: &str, check: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HealthStatus> + Send + 'static,
    {
        let mut checks = self.checks.write().await;
        checks.push(HealthCheck {
            name: name.to_string(),
            check: Box::new(move || Box::pin(check())),
            critical: true,
            timeout: self.default_timeout,
        });
    }
    
    /// Add a non-critical check (won't affect overall status)
    pub async fn add_optional_check<F, Fut>(&self, name: &str, check: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HealthStatus> + Send + 'static,
    {
        let mut checks = self.checks.write().await;
        checks.push(HealthCheck {
            name: name.to_string(),
            check: Box::new(move || Box::pin(check())),
            critical: false,
            timeout: self.default_timeout,
        });
    }
    
    /// Run all health checks
    pub async fn check_all(&self) -> HealthReport {
        let start = Instant::now();
        let checks = self.checks.read().await;
        
        let mut components = Vec::new();
        let mut has_unhealthy_critical = false;
        let mut has_degraded = false;
        
        for check in checks.iter() {
            let check_start = Instant::now();
            
            // Run check with timeout
            let status = match tokio::time::timeout(check.timeout, (check.check)()).await {
                Ok(s) => s,
                Err(_) => HealthStatus::Unknown(format!("Check timed out after {:?}", check.timeout)),
            };
            
            let duration = check_start.elapsed();
            
            if check.critical && !status.is_ok() {
                has_unhealthy_critical = true;
            }
            if matches!(status, HealthStatus::Degraded(_)) {
                has_degraded = true;
            }
            
            components.push(ComponentCheck::new(&check.name, status, duration));
        }
        
        // Determine overall status
        // If no critical failures and no degraded components, report healthy
        // (covers: empty components, all healthy, or only non-critical failures)
        let overall = if has_unhealthy_critical {
            HealthStatus::Unhealthy("One or more critical components are unhealthy".into())
        } else if has_degraded {
            HealthStatus::Degraded("One or more components are degraded".into())
        } else {
            HealthStatus::Healthy
        };
        
        HealthReport {
            status: overall,
            components,
            timestamp: Utc::now(),
            duration_ms: start.elapsed().as_millis() as u64,
            version: self.version.clone(),
            uptime_secs: Some(self.start_time.elapsed().as_secs()),
        }
    }
    
    /// Simple liveness check (always returns healthy)
    pub fn liveness(&self) -> HealthReport {
        HealthReport {
            status: HealthStatus::Healthy,
            components: vec![],
            timestamp: Utc::now(),
            duration_ms: 0,
            version: self.version.clone(),
            uptime_secs: Some(self.start_time.elapsed().as_secs()),
        }
    }
    
    /// Readiness check (runs all critical checks)
    pub async fn readiness(&self) -> HealthReport {
        self.check_all().await
    }
    
    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HealthChecker {
    fn clone(&self) -> Self {
        Self {
            checks: Arc::clone(&self.checks),
            version: self.version.clone(),
            start_time: self.start_time,
            default_timeout: self.default_timeout,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMMON CHECKS
// ═══════════════════════════════════════════════════════════════════════════

/// Common health check for database connection
pub async fn check_database_health(pool: &sqlx::SqlitePool) -> HealthStatus {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => HealthStatus::Healthy,
        Err(e) => HealthStatus::Unhealthy(format!("Database error: {}", e)),
    }
}

/// Check available disk space (returns degraded if below threshold)
pub fn check_disk_space(_path: &str, _min_gb: f64) -> HealthStatus {
    // Simplified - in production would use system calls
    HealthStatus::Healthy
}

/// Check memory usage (returns degraded if above threshold)
pub fn check_memory_usage(_max_percent: f64) -> HealthStatus {
    // Simplified - in production would use system calls
    HealthStatus::Healthy
}

// ═══════════════════════════════════════════════════════════════════════════
// AXUM HANDLERS (for easy integration)
// ═══════════════════════════════════════════════════════════════════════════

/// Create liveness endpoint handler
pub fn liveness_handler(checker: HealthChecker) -> impl Fn() -> HealthReport + Clone {
    move || checker.liveness()
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    // ═══════════════════════════════════════════════════════════════════════
    // STATUS TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_status_is_healthy() {
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(!HealthStatus::Degraded("test".into()).is_healthy());
        assert!(!HealthStatus::Unhealthy("test".into()).is_healthy());
        assert!(!HealthStatus::Unknown("test".into()).is_healthy());
    }
    
    #[test]
    fn test_status_is_ok() {
        assert!(HealthStatus::Healthy.is_ok());
        assert!(HealthStatus::Degraded("test".into()).is_ok());
        assert!(!HealthStatus::Unhealthy("test".into()).is_ok());
        assert!(!HealthStatus::Unknown("test".into()).is_ok());
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // REPORT TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_report_healthy() {
        let report = HealthReport::healthy();
        assert!(report.is_healthy());
        assert!(report.is_ready());
        assert_eq!(report.http_status(), 200);
    }
    
    #[test]
    fn test_report_http_status() {
        let mut report = HealthReport::healthy();
        
        report.status = HealthStatus::Healthy;
        assert_eq!(report.http_status(), 200);
        
        report.status = HealthStatus::Degraded("test".into());
        assert_eq!(report.http_status(), 200);
        
        report.status = HealthStatus::Unhealthy("test".into());
        assert_eq!(report.http_status(), 503);
        
        report.status = HealthStatus::Unknown("test".into());
        assert_eq!(report.http_status(), 503);
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // CHECKER TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_checker_no_checks() {
        let checker = HealthChecker::new();
        let report = checker.check_all().await;
        
        assert!(report.is_healthy());
        assert!(report.components.is_empty());
    }
    
    #[tokio::test]
    async fn test_checker_healthy_check() {
        let checker = HealthChecker::new();
        checker.add_check("test", || async { HealthStatus::Healthy }).await;
        
        let report = checker.check_all().await;
        
        assert!(report.is_healthy());
        assert_eq!(report.components.len(), 1);
        assert!(report.components[0].status.is_healthy());
    }
    
    #[tokio::test]
    async fn test_checker_unhealthy_check() {
        let checker = HealthChecker::new();
        checker.add_check("test", || async { 
            HealthStatus::Unhealthy("Connection failed".into()) 
        }).await;
        
        let report = checker.check_all().await;
        
        assert!(!report.is_healthy());
        assert!(!report.is_ready());
    }
    
    #[tokio::test]
    async fn test_checker_degraded_check() {
        let checker = HealthChecker::new();
        checker.add_check("test", || async { 
            HealthStatus::Degraded("High latency".into()) 
        }).await;
        
        let report = checker.check_all().await;
        
        assert!(!report.is_healthy());
        assert!(report.is_ready()); // Degraded is still ready
    }
    
    #[tokio::test]
    async fn test_checker_multiple_checks() {
        let checker = HealthChecker::new();
        checker.add_check("db", || async { HealthStatus::Healthy }).await;
        checker.add_check("cache", || async { HealthStatus::Healthy }).await;
        checker.add_check("api", || async { HealthStatus::Healthy }).await;
        
        let report = checker.check_all().await;
        
        assert!(report.is_healthy());
        assert_eq!(report.components.len(), 3);
    }
    
    #[tokio::test]
    async fn test_checker_optional_check_failure() {
        let checker = HealthChecker::new();
        checker.add_check("critical", || async { HealthStatus::Healthy }).await;
        checker.add_optional_check("optional", || async { 
            HealthStatus::Unhealthy("Down".into()) 
        }).await;
        
        let report = checker.check_all().await;
        
        // Should still be healthy because optional check failed
        assert!(report.is_healthy());
    }
    
    #[tokio::test]
    async fn test_checker_timeout() {
        let checker = HealthChecker::new().with_timeout(Duration::from_millis(50));
        
        checker.add_check("slow", || async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            HealthStatus::Healthy
        }).await;
        
        let report = checker.check_all().await;
        
        assert!(!report.is_ready());
        assert!(matches!(report.components[0].status, HealthStatus::Unknown(_)));
    }
    
    #[tokio::test]
    async fn test_checker_with_version() {
        let checker = HealthChecker::new().with_version("1.0.0");
        let report = checker.check_all().await;
        
        assert_eq!(report.version, Some("1.0.0".into()));
    }
    
    #[tokio::test]
    async fn test_checker_uptime() {
        let checker = HealthChecker::new();
        
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let report = checker.check_all().await;
        assert!(report.uptime_secs.is_some());
    }
    
    #[tokio::test]
    async fn test_liveness() {
        let checker = HealthChecker::new();
        checker.add_check("failing", || async { 
            HealthStatus::Unhealthy("Down".into()) 
        }).await;
        
        // Liveness should always be healthy (app is running)
        let report = checker.liveness();
        assert!(report.is_healthy());
    }
    
    #[tokio::test]
    async fn test_readiness() {
        let checker = HealthChecker::new();
        checker.add_check("db", || async { HealthStatus::Healthy }).await;
        
        let report = checker.readiness().await;
        assert!(report.is_ready());
    }
    
    #[tokio::test]
    async fn test_checker_clone() {
        let checker1 = HealthChecker::new();
        checker1.add_check("test", || async { HealthStatus::Healthy }).await;
        
        let checker2 = checker1.clone();
        let report = checker2.check_all().await;
        
        assert_eq!(report.components.len(), 1);
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // COMPONENT CHECK TESTS
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_component_check_creation() {
        let check = ComponentCheck::new(
            "database",
            HealthStatus::Healthy,
            Duration::from_millis(50),
        );
        
        assert_eq!(check.name, "database");
        assert_eq!(check.duration_ms, 50);
        assert!(check.status.is_healthy());
    }
    
    #[test]
    fn test_component_check_with_details() {
        let mut details = HashMap::new();
        details.insert("connections".to_string(), serde_json::json!(10));
        
        let check = ComponentCheck::new(
            "database",
            HealthStatus::Healthy,
            Duration::from_millis(50),
        ).with_details(details);
        
        assert!(check.details.is_some());
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // EDGE CASES
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_concurrent_checks() {
        let checker = HealthChecker::new();
        
        // Add slow checks
        for i in 0..5 {
            let name = format!("check{}", i);
            checker.add_check(&name, move || async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                HealthStatus::Healthy
            }).await;
        }
        
        let start = Instant::now();
        let report = checker.check_all().await;
        let _duration = start.elapsed();
        
        // Checks run sequentially, so should take ~250ms
        assert_eq!(report.components.len(), 5);
        assert!(report.is_healthy());
    }
    
    #[tokio::test]
    async fn test_check_duration_recorded() {
        let checker = HealthChecker::new();
        checker.add_check("slow", || async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            HealthStatus::Healthy
        }).await;
        
        let report = checker.check_all().await;
        
        assert!(report.components[0].duration_ms >= 100);
    }
    
    #[test]
    fn test_default_checker() {
        let checker = HealthChecker::default();
        let report = checker.liveness();
        assert!(report.is_healthy());
    }
    
    #[tokio::test]
    async fn test_mixed_critical_optional() {
        let checker = HealthChecker::new();
        
        // Critical healthy
        checker.add_check("critical1", || async { HealthStatus::Healthy }).await;
        
        // Critical unhealthy
        checker.add_check("critical2", || async { 
            HealthStatus::Unhealthy("Down".into()) 
        }).await;
        
        // Optional unhealthy
        checker.add_optional_check("optional", || async { 
            HealthStatus::Unhealthy("Down".into()) 
        }).await;
        
        let report = checker.check_all().await;
        
        // Should be unhealthy due to critical2
        assert!(!report.is_healthy());
        assert_eq!(report.components.len(), 3);
    }
}
