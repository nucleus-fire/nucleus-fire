# Job Queue Guide (Pulse)

Nucleus Pulse provides production-ready background job processing with persistence, retries, and priority queues.

## Quick Start

```rust
use nucleus_std::pulse::{Pulse, JobConfig, JobPriority};

// Create persistent queue
let pulse = Pulse::new("jobs.db").await?;

// Register handler
pulse.handle("send_email", |payload| async move {
    let data: EmailPayload = serde_json::from_str(&payload)?;
    Postman::send(&data.to, &data.subject, &data.body).await?;
    Ok(())
}).await;

// Enqueue job
pulse.enqueue("send_email", json!({
    "to": "user@example.com",
    "subject": "Welcome!",
    "body": "Thanks for signing up"
})).await?;

// Start processing
pulse.run().await?;
```

## Features

### Persistence
Jobs survive server restarts with SQLite storage:

```rust
// Persistent (production)
let pulse = Pulse::new("jobs.db").await?;

// In-memory (testing)
let pulse = Pulse::in_memory();
```

### Worker Concurrency

Control how many jobs run in parallel:

```rust
// Default: number of CPU cores
let pulse = Pulse::new("jobs.db").await?;

// Custom: 4 concurrent workers
let pulse = Pulse::new("jobs.db")
    .await?
    .with_workers(4);

// Single-threaded (for debugging)
let pulse = Pulse::new("jobs.db")
    .await?
    .with_workers(1);
```

### Graceful Shutdown

```rust
// Stop accepting new jobs, finish current ones
pulse.shutdown_graceful().await?;

// Force stop immediately (jobs will retry on restart)
pulse.shutdown_now().await?;
```

### Priority Queues

```rust
// Critical jobs run first
pulse.enqueue_with_config("payment", data, JobConfig {
    priority: JobPriority::Critical,
    max_retries: 5,
    ..Default::default()
}).await?;

// Priority levels: Critical > High > Normal > Low
```

### Automatic Retries

```rust
let config = JobConfig {
    max_retries: 3,
    retry_delay: Duration::from_secs(5), // Exponential backoff applied
    timeout: Duration::from_secs(300),
    ..Default::default()
};
```

### Scheduled Jobs

```rust
use chrono::Utc;

// Run in 1 hour
let run_at = Utc::now() + chrono::Duration::hours(1);
pulse.schedule("reminder", data, run_at).await?;
```

### Dead Letter Queue

Failed jobs (exceeded max retries) go to the dead letter queue:

```rust
// Get failed jobs
let dead = pulse.dead_jobs().await?;

for job in dead {
    println!("Failed: {} - {}", job.name, job.last_error.unwrap_or_default());
    
    // Retry manually
    pulse.retry(&job.id).await?;
}
```

### Job Status

```rust
let job = pulse.status(&job_id).await?;

match job.status {
    JobStatus::Pending => println!("Waiting..."),
    JobStatus::Running => println!("Processing..."),
    JobStatus::Completed => println!("Done!"),
    JobStatus::Dead => println!("Failed after {} attempts", job.attempts),
    _ => {}
}
```

## Config Presets

```rust
// Critical jobs with 5 retries
JobConfig::critical()

// No retries (fail fast)
JobConfig::no_retry()

// Default: 3 retries, normal priority
JobConfig::default()
```

## Best Practices

1. **Idempotent Handlers**: Jobs may run more than once on retry
2. **Payload Size**: Keep payloads small, store large data elsewhere
3. **Timeouts**: Set reasonable timeouts to prevent stuck jobs
4. **Monitoring**: Regularly check dead letter queue
