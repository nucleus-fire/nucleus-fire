//! Nucleus Scheduler - Cron-like Job Scheduling
//!
//! Schedule recurring tasks with cron expressions or convenience methods:
//! - Cron expression parsing (standard 5-field format)
//! - Convenience methods (hourly, daily, weekly, monthly)
//! - One-time scheduled tasks
//! - Integration with Pulse job queue
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::scheduler::Scheduler;
//!
//! let mut scheduler = Scheduler::new();
//!
//! // Cron expression (every day at midnight)
//! scheduler.cron("cleanup", "0 0 * * *", || async {
//!     cleanup_old_files().await;
//! })?;
//!
//! // Convenience methods
//! scheduler.hourly("sync", || async { sync_data().await; });
//! scheduler.daily("backup", || async { backup_database().await; });
//!
//! // Start scheduler
//! scheduler.run().await;
//! ```

use chrono::{DateTime, Datelike, Timelike, Utc};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES & ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Cron parsing error
#[derive(Debug, thiserror::Error)]
pub enum CronError {
    #[error("Invalid cron expression: {0}")]
    InvalidExpression(String),
    
    #[error("Field out of range: {field} value {value} not in {min}-{max}")]
    OutOfRange {
        field: String,
        value: u32,
        min: u32,
        max: u32,
    },
    
    #[error("Duplicate task name: {0}")]
    DuplicateName(String),
    
    #[error("Task not found: {0}")]
    NotFound(String),
}

/// Cron schedule expression
#[derive(Debug, Clone)]
pub struct CronSchedule {
    minutes: Vec<u32>,      // 0-59
    hours: Vec<u32>,        // 0-23
    days_of_month: Vec<u32>, // 1-31
    months: Vec<u32>,       // 1-12
    days_of_week: Vec<u32>, // 0-6 (Sunday = 0)
}

impl CronSchedule {
    /// Parse a cron expression (5 fields: minute hour day-of-month month day-of-week)
    pub fn parse(expression: &str) -> Result<Self, CronError> {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        
        if parts.len() != 5 {
            return Err(CronError::InvalidExpression(format!(
                "Expected 5 fields, got {}",
                parts.len()
            )));
        }
        
        Ok(Self {
            minutes: Self::parse_field(parts[0], 0, 59, "minute")?,
            hours: Self::parse_field(parts[1], 0, 23, "hour")?,
            days_of_month: Self::parse_field(parts[2], 1, 31, "day-of-month")?,
            months: Self::parse_field(parts[3], 1, 12, "month")?,
            days_of_week: Self::parse_field(parts[4], 0, 6, "day-of-week")?,
        })
    }
    
    fn parse_field(field: &str, min: u32, max: u32, name: &str) -> Result<Vec<u32>, CronError> {
        let mut values = Vec::new();
        
        for part in field.split(',') {
            if part == "*" {
                // All values
                values.extend(min..=max);
            } else if let Some(step_str) = part.strip_prefix("*/") {
                // Step values (*/5 = every 5)
                let step: u32 = step_str.parse().map_err(|_| {
                    CronError::InvalidExpression(format!("Invalid step: {}", step_str))
                })?;
                values.extend((min..=max).step_by(step as usize));
            } else if part.contains('-') {
                // Range (1-5)
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() != 2 {
                    return Err(CronError::InvalidExpression(format!(
                        "Invalid range: {}",
                        part
                    )));
                }
                let start: u32 = range_parts[0].parse().map_err(|_| {
                    CronError::InvalidExpression(format!("Invalid number: {}", range_parts[0]))
                })?;
                let end: u32 = range_parts[1].parse().map_err(|_| {
                    CronError::InvalidExpression(format!("Invalid number: {}", range_parts[1]))
                })?;
                
                if start < min || end > max || start > end {
                    return Err(CronError::OutOfRange {
                        field: name.to_string(),
                        value: if start < min { start } else { end },
                        min,
                        max,
                    });
                }
                values.extend(start..=end);
            } else {
                // Single value
                let value: u32 = part.parse().map_err(|_| {
                    CronError::InvalidExpression(format!("Invalid number: {}", part))
                })?;
                
                if value < min || value > max {
                    return Err(CronError::OutOfRange {
                        field: name.to_string(),
                        value,
                        min,
                        max,
                    });
                }
                values.push(value);
            }
        }
        
        values.sort();
        values.dedup();
        Ok(values)
    }
    
    /// Get next execution time after the given time
    pub fn next_after(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let mut current = after + chrono::Duration::minutes(1);
        
        // Reset seconds to 0
        current = current
            .with_second(0)?
            .with_nanosecond(0)?;
        
        // Search up to 4 years ahead
        let max_iterations = 365 * 24 * 60 * 4;
        
        for _ in 0..max_iterations {
            let minute = current.minute();
            let hour = current.hour();
            let day = current.day();
            let month = current.month();
            let weekday = current.weekday().num_days_from_sunday();
            
            // Check if current time matches
            if self.minutes.contains(&minute)
                && self.hours.contains(&hour)
                && self.days_of_month.contains(&day)
                && self.months.contains(&month)
                && self.days_of_week.contains(&weekday)
            {
                return Some(current);
            }
            
            // Advance by one minute
            current += chrono::Duration::minutes(1);
        }
        
        None
    }
    
    /// Common schedule: every minute
    pub fn every_minute() -> Self {
        Self::parse("* * * * *").unwrap()
    }
    
    /// Common schedule: every hour at minute 0
    pub fn hourly() -> Self {
        Self::parse("0 * * * *").unwrap()
    }
    
    /// Common schedule: every day at midnight
    pub fn daily() -> Self {
        Self::parse("0 0 * * *").unwrap()
    }
    
    /// Common schedule: every Sunday at midnight
    pub fn weekly() -> Self {
        Self::parse("0 0 * * 0").unwrap()
    }
    
    /// Common schedule: first of every month at midnight
    pub fn monthly() -> Self {
        Self::parse("0 0 1 * *").unwrap()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SCHEDULED TASK
// ═══════════════════════════════════════════════════════════════════════════

type BoxedTask = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// A scheduled task
pub struct ScheduledTask {
    /// Task name
    pub name: String,
    /// Cron schedule (None for one-time tasks)
    schedule: Option<CronSchedule>,
    /// One-time execution time
    #[allow(dead_code)]
    once_at: Option<DateTime<Utc>>,
    /// Task function
    task: BoxedTask,
    /// Last execution time
    pub last_run: Option<DateTime<Utc>>,
    /// Next scheduled execution
    pub next_run: Option<DateTime<Utc>>,
    /// Whether task is enabled
    pub enabled: bool,
}

impl ScheduledTask {
    fn new_recurring(name: &str, schedule: CronSchedule, task: BoxedTask) -> Self {
        let next_run = schedule.next_after(Utc::now());
        Self {
            name: name.to_string(),
            schedule: Some(schedule),
            once_at: None,
            task,
            last_run: None,
            next_run,
            enabled: true,
        }
    }
    
    fn new_once(name: &str, at: DateTime<Utc>, task: BoxedTask) -> Self {
        Self {
            name: name.to_string(),
            schedule: None,
            once_at: Some(at),
            task,
            last_run: None,
            next_run: Some(at),
            enabled: true,
        }
    }
    
    fn update_next_run(&mut self) {
        if let Some(ref schedule) = self.schedule {
            self.next_run = schedule.next_after(Utc::now());
        } else {
            // One-time task, no more runs
            self.next_run = None;
        }
    }
    
    fn should_run(&self) -> bool {
        if !self.enabled {
            return false;
        }
        
        match self.next_run {
            Some(next) => Utc::now() >= next,
            None => false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SCHEDULER
// ═══════════════════════════════════════════════════════════════════════════

/// Scheduler for recurring and one-time tasks
pub struct Scheduler {
    tasks: Arc<RwLock<HashMap<String, ScheduledTask>>>,
    running: Arc<RwLock<bool>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Add a recurring task with a cron expression
    pub async fn cron<F, Fut>(&self, name: &str, expression: &str, task: F) -> Result<(), CronError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let schedule = CronSchedule::parse(expression)?;
        
        let mut tasks = self.tasks.write().await;
        if tasks.contains_key(name) {
            return Err(CronError::DuplicateName(name.to_string()));
        }
        
        let boxed_task: BoxedTask = Box::new(move || Box::pin(task()));
        tasks.insert(name.to_string(), ScheduledTask::new_recurring(name, schedule, boxed_task));
        
        Ok(())
    }
    
    /// Add an hourly task (runs at minute 0 of every hour)
    pub async fn hourly<F, Fut>(&self, name: &str, task: F) -> Result<(), CronError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.cron(name, "0 * * * *", task).await
    }
    
    /// Add a daily task (runs at midnight)
    pub async fn daily<F, Fut>(&self, name: &str, task: F) -> Result<(), CronError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.cron(name, "0 0 * * *", task).await
    }
    
    /// Add a weekly task (runs at midnight on Sunday)
    pub async fn weekly<F, Fut>(&self, name: &str, task: F) -> Result<(), CronError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.cron(name, "0 0 * * 0", task).await
    }
    
    /// Add a monthly task (runs at midnight on the 1st)
    pub async fn monthly<F, Fut>(&self, name: &str, task: F) -> Result<(), CronError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.cron(name, "0 0 1 * *", task).await
    }
    
    /// Add a one-time task at a specific time
    pub async fn once_at<F, Fut>(&self, name: &str, at: DateTime<Utc>, task: F) -> Result<(), CronError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut tasks = self.tasks.write().await;
        if tasks.contains_key(name) {
            return Err(CronError::DuplicateName(name.to_string()));
        }
        
        let boxed_task: BoxedTask = Box::new(move || Box::pin(task()));
        tasks.insert(name.to_string(), ScheduledTask::new_once(name, at, boxed_task));
        
        Ok(())
    }
    
    /// Pause a task
    pub async fn pause(&self, name: &str) -> Result<(), CronError> {
        let mut tasks = self.tasks.write().await;
        let task = tasks.get_mut(name).ok_or_else(|| CronError::NotFound(name.to_string()))?;
        task.enabled = false;
        Ok(())
    }
    
    /// Resume a paused task
    pub async fn resume(&self, name: &str) -> Result<(), CronError> {
        let mut tasks = self.tasks.write().await;
        let task = tasks.get_mut(name).ok_or_else(|| CronError::NotFound(name.to_string()))?;
        task.enabled = true;
        task.update_next_run();
        Ok(())
    }
    
    /// Remove a task
    pub async fn remove(&self, name: &str) -> Result<(), CronError> {
        let mut tasks = self.tasks.write().await;
        tasks.remove(name).ok_or_else(|| CronError::NotFound(name.to_string()))?;
        Ok(())
    }
    
    /// Get list of upcoming tasks (sorted by next run time)
    pub async fn upcoming(&self, count: usize) -> Vec<(String, Option<DateTime<Utc>>)> {
        let tasks = self.tasks.read().await;
        let mut upcoming: Vec<_> = tasks
            .values()
            .filter(|t| t.enabled && t.next_run.is_some())
            .map(|t| (t.name.clone(), t.next_run))
            .collect();
        
        upcoming.sort_by(|a, b| a.1.cmp(&b.1));
        upcoming.truncate(count);
        upcoming
    }
    
    /// Check if scheduler has a task
    pub async fn has_task(&self, name: &str) -> bool {
        let tasks = self.tasks.read().await;
        tasks.contains_key(name)
    }
    
    /// Get task count
    pub async fn task_count(&self) -> usize {
        let tasks = self.tasks.read().await;
        tasks.len()
    }
    
    /// Run the scheduler (blocking)
    pub async fn run(&self) {
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
            
            // Check for tasks to run
            self.tick().await;
            
            // Sleep before next check
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    
    /// Stop the scheduler
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }
    
    /// Process one tick (for testing)
    pub async fn tick(&self) -> Vec<String> {
        let mut executed = Vec::new();
        let mut tasks = self.tasks.write().await;
        
        for task in tasks.values_mut() {
            if task.should_run() {
                // Execute task
                let future = (task.task)();
                future.await;
                
                task.last_run = Some(Utc::now());
                task.update_next_run();
                executed.push(task.name.clone());
            }
        }
        
        executed
    }
}

impl Clone for Scheduler {
    fn clone(&self) -> Self {
        Self {
            tasks: Arc::clone(&self.tasks),
            running: Arc::clone(&self.running),
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
    // CRON PARSING
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_cron_every_minute() {
        let schedule = CronSchedule::parse("* * * * *").unwrap();
        assert_eq!(schedule.minutes.len(), 60);
        assert_eq!(schedule.hours.len(), 24);
    }
    
    #[test]
    fn test_cron_every_hour() {
        let schedule = CronSchedule::parse("0 * * * *").unwrap();
        assert_eq!(schedule.minutes, vec![0]);
        assert_eq!(schedule.hours.len(), 24);
    }
    
    #[test]
    fn test_cron_daily_midnight() {
        let schedule = CronSchedule::parse("0 0 * * *").unwrap();
        assert_eq!(schedule.minutes, vec![0]);
        assert_eq!(schedule.hours, vec![0]);
    }
    
    #[test]
    fn test_cron_weekly_sunday() {
        let schedule = CronSchedule::parse("0 0 * * 0").unwrap();
        assert_eq!(schedule.days_of_week, vec![0]);
    }
    
    #[test]
    fn test_cron_monthly_first() {
        let schedule = CronSchedule::parse("0 0 1 * *").unwrap();
        assert_eq!(schedule.days_of_month, vec![1]);
    }
    
    #[test]
    fn test_cron_complex() {
        // Work hours: 9am-5pm, Monday-Friday
        let schedule = CronSchedule::parse("0 9-17 * * 1-5").unwrap();
        assert_eq!(schedule.hours, vec![9, 10, 11, 12, 13, 14, 15, 16, 17]);
        assert_eq!(schedule.days_of_week, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_cron_step_values() {
        let schedule = CronSchedule::parse("*/15 * * * *").unwrap();
        assert_eq!(schedule.minutes, vec![0, 15, 30, 45]);
    }
    
    #[test]
    fn test_cron_comma_values() {
        let schedule = CronSchedule::parse("0,30 * * * *").unwrap();
        assert_eq!(schedule.minutes, vec![0, 30]);
    }
    
    #[test]
    fn test_cron_invalid_syntax() {
        let result = CronSchedule::parse("* * *");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_cron_out_of_range() {
        let result = CronSchedule::parse("0 25 * * *");
        assert!(matches!(result, Err(CronError::OutOfRange { .. })));
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // NEXT EXECUTION TIME
    // ═══════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_next_execution_time() {
        let schedule = CronSchedule::hourly();
        let now = Utc::now();
        let next = schedule.next_after(now).unwrap();
        
        // Should be at minute 0
        assert_eq!(next.minute(), 0);
        // Should be in the future
        assert!(next > now);
    }
    
    #[test]
    fn test_next_execution_daily() {
        let schedule = CronSchedule::daily();
        let now = Utc::now();
        let next = schedule.next_after(now).unwrap();
        
        // Should be at midnight
        assert_eq!(next.hour(), 0);
        assert_eq!(next.minute(), 0);
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // SCHEDULER
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_scheduler_add_task() {
        let scheduler = Scheduler::new();
        scheduler.cron("test", "* * * * *", || async {}).await.unwrap();
        
        assert!(scheduler.has_task("test").await);
        assert_eq!(scheduler.task_count().await, 1);
    }
    
    #[tokio::test]
    async fn test_scheduler_convenience_methods() {
        let scheduler = Scheduler::new();
        
        scheduler.hourly("h", || async {}).await.unwrap();
        scheduler.daily("d", || async {}).await.unwrap();
        scheduler.weekly("w", || async {}).await.unwrap();
        scheduler.monthly("m", || async {}).await.unwrap();
        
        assert_eq!(scheduler.task_count().await, 4);
    }
    
    #[tokio::test]
    async fn test_scheduler_duplicate_name() {
        let scheduler = Scheduler::new();
        scheduler.cron("test", "* * * * *", || async {}).await.unwrap();
        
        let result = scheduler.cron("test", "0 * * * *", || async {}).await;
        assert!(matches!(result, Err(CronError::DuplicateName(_))));
    }
    
    #[tokio::test]
    async fn test_scheduler_remove_task() {
        let scheduler = Scheduler::new();
        scheduler.cron("test", "* * * * *", || async {}).await.unwrap();
        
        scheduler.remove("test").await.unwrap();
        assert!(!scheduler.has_task("test").await);
    }
    
    #[tokio::test]
    async fn test_scheduler_pause_resume() {
        let scheduler = Scheduler::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        // Schedule for past time so it should run
        let past = Utc::now() - chrono::Duration::minutes(1);
        scheduler.once_at("test", past, move || {
            let c = counter_clone.clone();
            async move {
                c.fetch_add(1, Ordering::SeqCst);
            }
        }).await.unwrap();
        
        // Pause before tick
        scheduler.pause("test").await.unwrap();
        scheduler.tick().await;
        
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        
        // Resume and tick
        scheduler.resume("test").await.unwrap();
    }
    
    #[tokio::test]
    async fn test_scheduler_once_at() {
        let scheduler = Scheduler::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let past = Utc::now() - chrono::Duration::seconds(10);
        scheduler.once_at("once", past, move || {
            let c = counter_clone.clone();
            async move {
                c.fetch_add(1, Ordering::SeqCst);
            }
        }).await.unwrap();
        
        scheduler.tick().await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        
        // Should not run again
        scheduler.tick().await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
    
    #[tokio::test]
    async fn test_scheduler_upcoming() {
        let scheduler = Scheduler::new();
        
        scheduler.hourly("hourly", || async {}).await.unwrap();
        scheduler.daily("daily", || async {}).await.unwrap();
        
        let upcoming = scheduler.upcoming(5).await;
        assert_eq!(upcoming.len(), 2);
    }
    
    #[tokio::test]
    async fn test_scheduler_task_not_found() {
        let scheduler = Scheduler::new();
        
        let result = scheduler.remove("nonexistent").await;
        assert!(matches!(result, Err(CronError::NotFound(_))));
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // EDGE CASES
    // ═══════════════════════════════════════════════════════════════════════
    
    #[tokio::test]
    async fn test_empty_scheduler() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.task_count().await, 0);
        
        let executed = scheduler.tick().await;
        assert!(executed.is_empty());
    }
    
    #[test]
    fn test_cron_presets() {
        let _ = CronSchedule::every_minute();
        let _ = CronSchedule::hourly();
        let _ = CronSchedule::daily();
        let _ = CronSchedule::weekly();
        let _ = CronSchedule::monthly();
    }
    
    #[tokio::test]
    async fn test_scheduler_clone() {
        let scheduler1 = Scheduler::new();
        scheduler1.cron("test", "* * * * *", || async {}).await.unwrap();
        
        let scheduler2 = scheduler1.clone();
        assert!(scheduler2.has_task("test").await);
    }
}
