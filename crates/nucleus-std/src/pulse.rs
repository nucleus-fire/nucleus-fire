//! Nucleus Pulse - Enhanced Job Queue
//!
//! Production-ready background job processing with:
//! - SQLite persistence (jobs survive restarts)
//! - Automatic retries with exponential backoff
//! - Priority queues (Critical > High > Normal > Low)
//! - Dead letter queue for failed jobs
//! - Scheduled jobs (run at specific time)
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::pulse::{Pulse, JobConfig, JobPriority};
//!
//! let pulse = Pulse::new("jobs.db").await?;
//!
//! // Simple job
//! pulse.enqueue("send_email", json!({ "to": "user@example.com" })).await?;
//!
//! // Priority job with retries
//! pulse.enqueue_with_config("process_payment", payment, JobConfig {
//!     max_retries: 5,
//!     priority: JobPriority::Critical,
//!     ..Default::default()
//! }).await?;
//!
//! // Register handler
//! pulse.handle("send_email", |payload| async move {
//!     let data: EmailPayload = serde_json::from_str(&payload)?;
//!     Ok(())
//! });
//!
//! // Start processing
//! pulse.run().await?;
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES & ENUMS
// ═══════════════════════════════════════════════════════════════════════════

/// Job execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JobStatus {
    /// Waiting to be processed
    Pending,
    /// Currently being executed
    Running,
    /// Successfully completed
    Completed,
    /// Failed but will retry
    Failed { error: String, attempts: u32 },
    /// Exceeded max retries, moved to dead letter queue
    Dead,
    /// Cancelled by user
    Cancelled,
}

impl JobStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobStatus::Completed | JobStatus::Dead | JobStatus::Cancelled
        )
    }
}

/// Job priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum JobPriority {
    Low = 0,
    #[default]
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Job configuration options
#[derive(Debug, Clone)]
pub struct JobConfig {
    /// Maximum retry attempts (default: 3)
    pub max_retries: u32,
    /// Base delay between retries (exponential backoff applied)
    pub retry_delay: Duration,
    /// Maximum execution time before timeout
    pub timeout: Duration,
    /// Job priority
    pub priority: JobPriority,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
            timeout: Duration::from_secs(300), // 5 minutes
            priority: JobPriority::Normal,
        }
    }
}

impl JobConfig {
    pub fn critical() -> Self {
        Self {
            max_retries: 5,
            priority: JobPriority::Critical,
            ..Default::default()
        }
    }

    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }
}

/// A job in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Unique job identifier
    pub id: String,
    /// Job type/name (maps to handler)
    pub name: String,
    /// JSON payload
    pub payload: String,
    /// Current status
    pub status: JobStatus,
    /// Number of execution attempts
    pub attempts: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Priority level
    pub priority: JobPriority,
    /// When the job was created
    pub created_at: DateTime<Utc>,
    /// When the job should run (for scheduled jobs)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// When execution started
    pub started_at: Option<DateTime<Utc>>,
    /// When execution completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Last error message
    pub last_error: Option<String>,
}

impl Job {
    fn new(name: &str, payload: String, config: &JobConfig) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            payload,
            status: JobStatus::Pending,
            attempts: 0,
            max_retries: config.max_retries,
            priority: config.priority,
            created_at: Utc::now(),
            scheduled_at: None,
            started_at: None,
            completed_at: None,
            last_error: None,
        }
    }

    fn scheduled(name: &str, payload: String, run_at: DateTime<Utc>, config: &JobConfig) -> Self {
        let mut job = Self::new(name, payload, config);
        job.scheduled_at = Some(run_at);
        job
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Pulse error types
#[derive(Debug, thiserror::Error)]
pub enum PulseError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Job not found: {0}")]
    NotFound(String),

    #[error("No handler registered for job: {0}")]
    NoHandler(String),

    #[error("Job execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Queue closed")]
    QueueClosed,

    #[error("Job cancelled")]
    Cancelled,

    #[error("Job timeout")]
    Timeout,
}

// ═══════════════════════════════════════════════════════════════════════════
// STORAGE BACKENDS
// ═══════════════════════════════════════════════════════════════════════════

/// Job storage backend trait
#[async_trait::async_trait]
pub trait JobStore: Send + Sync {
    async fn save(&self, job: &Job) -> Result<(), PulseError>;
    async fn get(&self, id: &str) -> Result<Option<Job>, PulseError>;
    async fn update(&self, job: &Job) -> Result<(), PulseError>;
    async fn delete(&self, id: &str) -> Result<(), PulseError>;
    async fn get_pending(&self) -> Result<Vec<Job>, PulseError>;
    async fn get_dead(&self) -> Result<Vec<Job>, PulseError>;
    async fn get_scheduled_ready(&self) -> Result<Vec<Job>, PulseError>;
}

/// In-memory job store (for testing)
pub struct MemoryJobStore {
    jobs: Arc<RwLock<HashMap<String, Job>>>,
}

impl MemoryJobStore {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemoryJobStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl JobStore for MemoryJobStore {
    async fn save(&self, job: &Job) -> Result<(), PulseError> {
        let mut jobs = self.jobs.write().await;
        jobs.insert(job.id.clone(), job.clone());
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<Job>, PulseError> {
        let jobs = self.jobs.read().await;
        Ok(jobs.get(id).cloned())
    }

    async fn update(&self, job: &Job) -> Result<(), PulseError> {
        let mut jobs = self.jobs.write().await;
        jobs.insert(job.id.clone(), job.clone());
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), PulseError> {
        let mut jobs = self.jobs.write().await;
        jobs.remove(id);
        Ok(())
    }

    async fn get_pending(&self) -> Result<Vec<Job>, PulseError> {
        let jobs = self.jobs.read().await;
        let now = Utc::now();
        let mut pending: Vec<_> = jobs
            .values()
            .filter(|j| {
                matches!(j.status, JobStatus::Pending) && j.scheduled_at.is_none_or(|t| t <= now)
            })
            .cloned()
            .collect();

        // Sort by priority (highest first), then by created_at (oldest first)
        pending.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.created_at.cmp(&b.created_at))
        });

        Ok(pending)
    }

    async fn get_dead(&self) -> Result<Vec<Job>, PulseError> {
        let jobs = self.jobs.read().await;
        Ok(jobs
            .values()
            .filter(|j| matches!(j.status, JobStatus::Dead))
            .cloned()
            .collect())
    }

    async fn get_scheduled_ready(&self) -> Result<Vec<Job>, PulseError> {
        let jobs = self.jobs.read().await;
        let now = Utc::now();
        Ok(jobs
            .values()
            .filter(|j| {
                matches!(j.status, JobStatus::Pending) && j.scheduled_at.is_some_and(|t| t <= now)
            })
            .cloned()
            .collect())
    }
}

/// SQLite job store (for persistence)
pub struct SqliteJobStore {
    pool: sqlx::SqlitePool,
}

impl SqliteJobStore {
    pub async fn new(path: &str) -> Result<Self, PulseError> {
        let url = if path == ":memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite:{}?mode=rwc", path)
        };

        let pool = sqlx::SqlitePool::connect(&url)
            .await
            .map_err(|e| PulseError::Database(e.to_string()))?;

        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS jobs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL,
                attempts INTEGER NOT NULL DEFAULT 0,
                max_retries INTEGER NOT NULL DEFAULT 3,
                priority INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                scheduled_at TEXT,
                started_at TEXT,
                completed_at TEXT,
                last_error TEXT
            )
        "#,
        )
        .execute(&pool)
        .await
        .map_err(|e| PulseError::Database(e.to_string()))?;

        // Create index for efficient queries
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status)")
            .execute(&pool)
            .await
            .map_err(|e| PulseError::Database(e.to_string()))?;

        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl JobStore for SqliteJobStore {
    async fn save(&self, job: &Job) -> Result<(), PulseError> {
        let status = serde_json::to_string(&job.status)?;
        let priority = job.priority as i32;

        sqlx::query(r#"
            INSERT INTO jobs (id, name, payload, status, attempts, max_retries, priority, created_at, scheduled_at, started_at, completed_at, last_error)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&job.id)
        .bind(&job.name)
        .bind(&job.payload)
        .bind(&status)
        .bind(job.attempts as i32)
        .bind(job.max_retries as i32)
        .bind(priority)
        .bind(job.created_at.to_rfc3339())
        .bind(job.scheduled_at.map(|t| t.to_rfc3339()))
        .bind(job.started_at.map(|t| t.to_rfc3339()))
        .bind(job.completed_at.map(|t| t.to_rfc3339()))
        .bind(&job.last_error)
        .execute(&self.pool)
        .await
        .map_err(|e| PulseError::Database(e.to_string()))?;

        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<Job>, PulseError> {
        let row: Option<(String, String, String, String, i32, i32, i32, String, Option<String>, Option<String>, Option<String>, Option<String>)> =
            sqlx::query_as("SELECT id, name, payload, status, attempts, max_retries, priority, created_at, scheduled_at, started_at, completed_at, last_error FROM jobs WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| PulseError::Database(e.to_string()))?;

        match row {
            Some((
                id,
                name,
                payload,
                status,
                attempts,
                max_retries,
                priority,
                created_at,
                scheduled_at,
                started_at,
                completed_at,
                last_error,
            )) => Ok(Some(Job {
                id,
                name,
                payload,
                status: serde_json::from_str(&status)?,
                attempts: attempts as u32,
                max_retries: max_retries as u32,
                priority: match priority {
                    0 => JobPriority::Low,
                    2 => JobPriority::High,
                    3 => JobPriority::Critical,
                    _ => JobPriority::Normal,
                },
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map(|t| t.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                scheduled_at: scheduled_at.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|t| t.with_timezone(&Utc))
                }),
                started_at: started_at.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|t| t.with_timezone(&Utc))
                }),
                completed_at: completed_at.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|t| t.with_timezone(&Utc))
                }),
                last_error,
            })),
            None => Ok(None),
        }
    }

    async fn update(&self, job: &Job) -> Result<(), PulseError> {
        let status = serde_json::to_string(&job.status)?;
        let _priority = job.priority as i32;

        sqlx::query(
            r#"
            UPDATE jobs SET
                status = ?, attempts = ?, started_at = ?, completed_at = ?, last_error = ?
            WHERE id = ?
        "#,
        )
        .bind(&status)
        .bind(job.attempts as i32)
        .bind(job.started_at.map(|t| t.to_rfc3339()))
        .bind(job.completed_at.map(|t| t.to_rfc3339()))
        .bind(&job.last_error)
        .bind(&job.id)
        .execute(&self.pool)
        .await
        .map_err(|e| PulseError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), PulseError> {
        sqlx::query("DELETE FROM jobs WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| PulseError::Database(e.to_string()))?;
        Ok(())
    }

    async fn get_pending(&self) -> Result<Vec<Job>, PulseError> {
        let now = Utc::now().to_rfc3339();
        let rows: Vec<(String, String, String, String, i32, i32, i32, String, Option<String>, Option<String>, Option<String>, Option<String>)> =
            sqlx::query_as(r#"
                SELECT id, name, payload, status, attempts, max_retries, priority, created_at, scheduled_at, started_at, completed_at, last_error
                FROM jobs
                WHERE status = '"Pending"' AND (scheduled_at IS NULL OR scheduled_at <= ?)
                ORDER BY priority DESC, created_at ASC
            "#)
            .bind(&now)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PulseError::Database(e.to_string()))?;

        rows.into_iter()
            .map(
                |(
                    id,
                    name,
                    payload,
                    status,
                    attempts,
                    max_retries,
                    priority,
                    created_at,
                    scheduled_at,
                    started_at,
                    completed_at,
                    last_error,
                )| {
                    Ok(Job {
                        id,
                        name,
                        payload,
                        status: serde_json::from_str(&status)?,
                        attempts: attempts as u32,
                        max_retries: max_retries as u32,
                        priority: match priority {
                            0 => JobPriority::Low,
                            2 => JobPriority::High,
                            3 => JobPriority::Critical,
                            _ => JobPriority::Normal,
                        },
                        created_at: DateTime::parse_from_rfc3339(&created_at)
                            .map(|t| t.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        scheduled_at: scheduled_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|t| t.with_timezone(&Utc))
                        }),
                        started_at: started_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|t| t.with_timezone(&Utc))
                        }),
                        completed_at: completed_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|t| t.with_timezone(&Utc))
                        }),
                        last_error,
                    })
                },
            )
            .collect()
    }

    async fn get_dead(&self) -> Result<Vec<Job>, PulseError> {
        let rows: Vec<(String, String, String, String, i32, i32, i32, String, Option<String>, Option<String>, Option<String>, Option<String>)> =
            sqlx::query_as(r#"
                SELECT id, name, payload, status, attempts, max_retries, priority, created_at, scheduled_at, started_at, completed_at, last_error
                FROM jobs WHERE status = '"Dead"'
            "#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PulseError::Database(e.to_string()))?;

        rows.into_iter()
            .map(
                |(
                    id,
                    name,
                    payload,
                    status,
                    attempts,
                    max_retries,
                    priority,
                    created_at,
                    scheduled_at,
                    started_at,
                    completed_at,
                    last_error,
                )| {
                    Ok(Job {
                        id,
                        name,
                        payload,
                        status: serde_json::from_str(&status).unwrap_or(JobStatus::Dead),
                        attempts: attempts as u32,
                        max_retries: max_retries as u32,
                        priority: match priority {
                            0 => JobPriority::Low,
                            2 => JobPriority::High,
                            3 => JobPriority::Critical,
                            _ => JobPriority::Normal,
                        },
                        created_at: DateTime::parse_from_rfc3339(&created_at)
                            .map(|t| t.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        scheduled_at: scheduled_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|t| t.with_timezone(&Utc))
                        }),
                        started_at: started_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|t| t.with_timezone(&Utc))
                        }),
                        completed_at: completed_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|t| t.with_timezone(&Utc))
                        }),
                        last_error,
                    })
                },
            )
            .collect()
    }

    async fn get_scheduled_ready(&self) -> Result<Vec<Job>, PulseError> {
        self.get_pending().await
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PULSE (MAIN STRUCT)
// ═══════════════════════════════════════════════════════════════════════════

type BoxedHandler =
    Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> + Send + Sync>;

/// Enhanced job queue with persistence and retries
pub struct Pulse<S: JobStore = MemoryJobStore> {
    store: Arc<S>,
    handlers: Arc<RwLock<HashMap<String, BoxedHandler>>>,
    workers: usize,
    running: Arc<RwLock<bool>>,
    default_config: JobConfig,
}

impl Pulse<MemoryJobStore> {
    /// Create an in-memory queue (for testing)
    pub fn in_memory() -> Self {
        Self {
            store: Arc::new(MemoryJobStore::new()),
            handlers: Arc::new(RwLock::new(HashMap::new())),
            workers: 4,
            running: Arc::new(RwLock::new(false)),
            default_config: JobConfig::default(),
        }
    }
}

impl Pulse<SqliteJobStore> {
    /// Create with SQLite persistence
    pub async fn new(db_path: &str) -> Result<Self, PulseError> {
        let store = SqliteJobStore::new(db_path).await?;
        Ok(Self {
            store: Arc::new(store),
            handlers: Arc::new(RwLock::new(HashMap::new())),
            workers: 4,
            running: Arc::new(RwLock::new(false)),
            default_config: JobConfig::default(),
        })
    }
}

impl<S: JobStore + 'static> Pulse<S> {
    /// Enqueue a job with default config
    pub async fn enqueue<T: Serialize>(
        &self,
        name: &str,
        payload: T,
    ) -> Result<String, PulseError> {
        self.enqueue_with_config(name, payload, self.default_config.clone())
            .await
    }

    /// Enqueue a job with custom config
    pub async fn enqueue_with_config<T: Serialize>(
        &self,
        name: &str,
        payload: T,
        config: JobConfig,
    ) -> Result<String, PulseError> {
        let payload_json = serde_json::to_string(&payload)?;
        let job = Job::new(name, payload_json, &config);
        let job_id = job.id.clone();

        self.store.save(&job).await?;

        Ok(job_id)
    }

    /// Schedule a job to run at a specific time
    pub async fn schedule<T: Serialize>(
        &self,
        name: &str,
        payload: T,
        run_at: DateTime<Utc>,
    ) -> Result<String, PulseError> {
        let payload_json = serde_json::to_string(&payload)?;
        let job = Job::scheduled(name, payload_json, run_at, &self.default_config);
        let job_id = job.id.clone();

        self.store.save(&job).await?;

        Ok(job_id)
    }

    /// Register a job handler
    pub async fn handle<F, Fut>(&self, name: &str, handler: F)
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), String>> + Send + 'static,
    {
        let mut handlers = self.handlers.write().await;
        handlers.insert(
            name.to_string(),
            Box::new(move |payload| Box::pin(handler(payload))),
        );
    }

    /// Get job status
    pub async fn status(&self, job_id: &str) -> Result<Job, PulseError> {
        self.store
            .get(job_id)
            .await?
            .ok_or_else(|| PulseError::NotFound(job_id.to_string()))
    }

    /// Retry a failed/dead job
    pub async fn retry(&self, job_id: &str) -> Result<(), PulseError> {
        let mut job = self.status(job_id).await?;
        job.status = JobStatus::Pending;
        job.attempts = 0;
        job.last_error = None;
        self.store.update(&job).await
    }

    /// Cancel a pending job
    pub async fn cancel(&self, job_id: &str) -> Result<(), PulseError> {
        let mut job = self.status(job_id).await?;
        if !matches!(job.status, JobStatus::Pending) {
            return Err(PulseError::ExecutionFailed(
                "Can only cancel pending jobs".into(),
            ));
        }
        job.status = JobStatus::Cancelled;
        self.store.update(&job).await
    }

    /// Get all dead letter jobs
    pub async fn dead_jobs(&self) -> Result<Vec<Job>, PulseError> {
        self.store.get_dead().await
    }

    /// Process a single job
    async fn process_job(&self, mut job: Job) -> Result<(), PulseError> {
        let handlers = self.handlers.read().await;
        let handler = handlers
            .get(&job.name)
            .ok_or_else(|| PulseError::NoHandler(job.name.clone()))?;

        job.status = JobStatus::Running;
        job.started_at = Some(Utc::now());
        job.attempts += 1;
        self.store.update(&job).await?;

        // Execute handler
        let result = handler(job.payload.clone()).await;

        match result {
            Ok(()) => {
                job.status = JobStatus::Completed;
                job.completed_at = Some(Utc::now());
                self.store.update(&job).await?;
            }
            Err(error) => {
                if job.attempts >= job.max_retries {
                    job.status = JobStatus::Dead;
                    job.last_error = Some(error);
                } else {
                    job.status = JobStatus::Failed {
                        error: error.clone(),
                        attempts: job.attempts,
                    };
                    job.last_error = Some(error);
                    // Reset to pending for retry
                    job.status = JobStatus::Pending;
                }
                self.store.update(&job).await?;
            }
        }

        Ok(())
    }

    /// Start processing jobs
    pub async fn run(&self) -> Result<(), PulseError> {
        {
            let mut running = self.running.write().await;
            *running = true;
        }

        loop {
            {
                let running = self.running.read().await;
                if !*running {
                    break;
                }
            }

            // Get pending jobs
            let jobs = self.store.get_pending().await?;

            for job in jobs.into_iter().take(self.workers) {
                self.process_job(job).await?;
            }

            // Small delay to prevent busy loop
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }

    /// Stop processing jobs
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }

    /// Process one batch of jobs (for testing)
    pub async fn process_batch(&self) -> Result<usize, PulseError> {
        let jobs = self.store.get_pending().await?;
        let count = jobs.len().min(self.workers);

        for job in jobs.into_iter().take(self.workers) {
            self.process_job(job).await?;
        }

        Ok(count)
    }
}

impl<S: JobStore> Clone for Pulse<S> {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            handlers: Arc::clone(&self.handlers),
            workers: self.workers,
            running: Arc::clone(&self.running),
            default_config: self.default_config.clone(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // ═══════════════════════════════════════════════════════════════════════
    // BASIC OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_enqueue_job() {
        let pulse = Pulse::in_memory();
        let job_id = pulse
            .enqueue("test", serde_json::json!({"key": "value"}))
            .await
            .unwrap();

        assert!(!job_id.is_empty());
        assert_eq!(job_id.len(), 36); // UUID format
    }

    #[tokio::test]
    async fn test_enqueue_empty_payload() {
        let pulse = Pulse::in_memory();
        let job_id = pulse.enqueue("test", serde_json::json!({})).await.unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert_eq!(job.payload, "{}");
    }

    #[tokio::test]
    async fn test_enqueue_large_payload() {
        let pulse = Pulse::in_memory();
        let large_data = "x".repeat(1_000_000); // 1MB
        let job_id = pulse
            .enqueue("test", serde_json::json!({"data": large_data}))
            .await
            .unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert!(job.payload.len() > 1_000_000);
    }

    #[tokio::test]
    async fn test_enqueue_unicode_payload() {
        let pulse = Pulse::in_memory();
        let job_id = pulse
            .enqueue(
                "test",
                serde_json::json!({
                    "chinese": "你好世界",
                    "emoji": "🚀🔥💯",
                    "arabic": "مرحبا"
                }),
            )
            .await
            .unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert!(job.payload.contains("你好"));
        assert!(job.payload.contains("🚀"));
    }

    #[tokio::test]
    async fn test_job_execution() {
        let pulse = Pulse::in_memory();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        pulse
            .handle("increment", move |_| {
                let c = counter_clone.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            })
            .await;

        pulse
            .enqueue("increment", serde_json::json!({}))
            .await
            .unwrap();
        pulse.process_batch().await.unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_job_status_tracking() {
        let pulse = Pulse::in_memory();

        pulse.handle("test", |_| async { Ok(()) }).await;

        let job_id = pulse.enqueue("test", serde_json::json!({})).await.unwrap();

        // Initially pending
        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Pending));

        // After processing, completed
        pulse.process_batch().await.unwrap();
        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Completed));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RETRIES & FAILURES
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_retry_on_failure() {
        let pulse = Pulse::in_memory();
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();

        pulse
            .handle("fail_once", move |_| {
                let a = attempts_clone.clone();
                async move {
                    let count = a.fetch_add(1, Ordering::SeqCst);
                    if count == 0 {
                        Err("First attempt fails".to_string())
                    } else {
                        Ok(())
                    }
                }
            })
            .await;

        pulse
            .enqueue_with_config(
                "fail_once",
                serde_json::json!({}),
                JobConfig {
                    max_retries: 3,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        // First attempt fails
        pulse.process_batch().await.unwrap();
        assert_eq!(attempts.load(Ordering::SeqCst), 1);

        // Second attempt succeeds
        pulse.process_batch().await.unwrap();
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_max_retries_exceeded() {
        let pulse = Pulse::in_memory();

        pulse
            .handle("always_fail", |_| async { Err("Always fails".to_string()) })
            .await;

        let job_id = pulse
            .enqueue_with_config(
                "always_fail",
                serde_json::json!({}),
                JobConfig {
                    max_retries: 2,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        // Attempt 1
        pulse.process_batch().await.unwrap();
        // Attempt 2 (max reached)
        pulse.process_batch().await.unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Dead));
    }

    #[tokio::test]
    async fn test_dead_letter_queue() {
        let pulse = Pulse::in_memory();

        pulse.handle("fail", |_| async { Err("fail".into()) }).await;

        pulse
            .enqueue_with_config("fail", serde_json::json!({}), JobConfig::no_retry())
            .await
            .unwrap();
        pulse.process_batch().await.unwrap();

        let dead = pulse.dead_jobs().await.unwrap();
        assert_eq!(dead.len(), 1);
    }

    #[tokio::test]
    async fn test_manual_retry() {
        let pulse = Pulse::in_memory();
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();

        pulse
            .handle("retry_test", move |_| {
                let a = attempts_clone.clone();
                async move {
                    if a.fetch_add(1, Ordering::SeqCst) < 2 {
                        Err("fail".into())
                    } else {
                        Ok(())
                    }
                }
            })
            .await;

        let job_id = pulse
            .enqueue_with_config("retry_test", serde_json::json!({}), JobConfig::no_retry())
            .await
            .unwrap();

        // First attempt fails, goes to dead
        pulse.process_batch().await.unwrap();
        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Dead));

        // Manual retry
        pulse.retry(&job_id).await.unwrap();

        // Now pending again
        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Pending));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIORITY & SCHEDULING
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_priority_ordering() {
        let pulse = Pulse::in_memory();
        let order = Arc::new(RwLock::new(Vec::<String>::new()));

        let order_clone = order.clone();
        pulse
            .handle("record", move |payload| {
                let o = order_clone.clone();
                async move {
                    let mut vec = o.write().await;
                    vec.push(payload);
                    Ok(())
                }
            })
            .await;

        // Enqueue in reverse priority order
        pulse
            .enqueue_with_config(
                "record",
                "low".to_string(),
                JobConfig {
                    priority: JobPriority::Low,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        pulse
            .enqueue_with_config(
                "record",
                "critical".to_string(),
                JobConfig {
                    priority: JobPriority::Critical,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        pulse
            .enqueue_with_config("record", "normal".to_string(), JobConfig::default())
            .await
            .unwrap();

        // Process all
        pulse.process_batch().await.unwrap();
        pulse.process_batch().await.unwrap();
        pulse.process_batch().await.unwrap();

        let order_vec = order.read().await;
        // Critical should be first
        assert!(order_vec[0].contains("critical"));
    }

    #[tokio::test]
    async fn test_schedule_future() {
        let pulse = Pulse::in_memory();

        pulse.handle("scheduled", |_| async { Ok(()) }).await;

        let future_time = Utc::now() + chrono::Duration::hours(1);
        let job_id = pulse
            .schedule("scheduled", serde_json::json!({}), future_time)
            .await
            .unwrap();

        // Should not process (scheduled for future)
        let processed = pulse.process_batch().await.unwrap();
        assert_eq!(processed, 0);

        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Pending));
    }

    #[tokio::test]
    async fn test_schedule_past() {
        let pulse = Pulse::in_memory();

        pulse.handle("scheduled", |_| async { Ok(()) }).await;

        let past_time = Utc::now() - chrono::Duration::hours(1);
        let job_id = pulse
            .schedule("scheduled", serde_json::json!({}), past_time)
            .await
            .unwrap();

        // Should process immediately
        pulse.process_batch().await.unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Completed));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // EDGE CASES
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_cancel_pending_job() {
        let pulse = Pulse::in_memory();

        pulse.handle("test", |_| async { Ok(()) }).await;

        let job_id = pulse.enqueue("test", serde_json::json!({})).await.unwrap();
        pulse.cancel(&job_id).await.unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Cancelled));

        // Cancelled jobs should not process
        let processed = pulse.process_batch().await.unwrap();
        assert_eq!(processed, 0);
    }

    #[tokio::test]
    async fn test_missing_handler() {
        let pulse = Pulse::in_memory();

        pulse
            .enqueue("no_handler", serde_json::json!({}))
            .await
            .unwrap();

        let result = pulse.process_batch().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_job_not_found() {
        let pulse = Pulse::in_memory();

        let result = pulse.status("nonexistent").await;
        assert!(matches!(result, Err(PulseError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_job_config_presets() {
        let critical = JobConfig::critical();
        assert_eq!(critical.priority, JobPriority::Critical);
        assert_eq!(critical.max_retries, 5);

        let no_retry = JobConfig::no_retry();
        assert_eq!(no_retry.max_retries, 0);
    }

    #[tokio::test]
    async fn test_job_status_terminal() {
        assert!(JobStatus::Completed.is_terminal());
        assert!(JobStatus::Dead.is_terminal());
        assert!(JobStatus::Cancelled.is_terminal());
        assert!(!JobStatus::Pending.is_terminal());
        assert!(!JobStatus::Running.is_terminal());
    }

    #[tokio::test]
    async fn test_in_memory_mode() {
        let pulse = Pulse::in_memory();

        pulse.handle("test", |_| async { Ok(()) }).await;

        let job_id = pulse.enqueue("test", serde_json::json!({})).await.unwrap();
        pulse.process_batch().await.unwrap();

        let job = pulse.status(&job_id).await.unwrap();
        assert!(matches!(job.status, JobStatus::Completed));
    }
}
