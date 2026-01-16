# Scheduled Jobs Guide (Scheduler)

Nucleus Scheduler provides cron-like scheduling for recurring tasks.

## Quick Start

```rust
use nucleus_std::scheduler::Scheduler;

let mut scheduler = Scheduler::new();

// Run every day at midnight
scheduler.cron("daily_backup", "0 0 * * *", || async {
    backup_database().await;
}).await?;

// Convenience method
scheduler.hourly("sync", || async {
    sync_cache().await;
}).await?;

// Start scheduler (runs forever)
scheduler.run().await;
```

## Cron Syntax

Standard 5-field cron format:

```
┌───────────── minute (0-59)
│ ┌───────────── hour (0-23)
│ │ ┌───────────── day of month (1-31)
│ │ │ ┌───────────── month (1-12)
│ │ │ │ ┌───────────── day of week (0-6, Sunday=0)
│ │ │ │ │
* * * * *
```

### Examples

| Expression | Description |
|------------|-------------|
| `* * * * *` | Every minute |
| `0 * * * *` | Every hour |
| `0 0 * * *` | Daily at midnight |
| `0 0 * * 0` | Weekly on Sunday |
| `0 0 1 * *` | Monthly on the 1st |
| `*/15 * * * *` | Every 15 minutes |
| `0 9-17 * * 1-5` | 9am-5pm, Mon-Fri |

## Convenience Methods

```rust
// Every hour at minute 0
scheduler.hourly("task", || async {}).await?;

// Every day at midnight
scheduler.daily("task", || async {}).await?;

// Every Sunday at midnight
scheduler.weekly("task", || async {}).await?;

// Every 1st of month at midnight
scheduler.monthly("task", || async {}).await?;
```

## One-Time Tasks

Schedule a task to run at a specific time:

```rust
use chrono::Utc;

let run_at = Utc::now() + chrono::Duration::hours(1);

scheduler.once_at("reminder", run_at, || async {
    send_reminder().await;
}).await?;
```

## Task Control

```rust
// Pause a task
scheduler.pause("cleanup").await?;

// Resume a task
scheduler.resume("cleanup").await?;

// Remove a task
scheduler.remove("cleanup").await?;

// Check if exists
if scheduler.has_task("cleanup").await {
    // ...
}
```

## View Upcoming Tasks

```rust
let upcoming = scheduler.upcoming(10).await;

for (name, next_run) in upcoming {
    println!("{}: {:?}", name, next_run);
}
```

## Integration with Pulse

For complex jobs that need persistence and retries, combine Scheduler with Pulse:

```rust
let pulse = Pulse::new("jobs.db").await?;
let scheduler = Scheduler::new();

// Schedule job via Pulse for persistence
scheduler.daily("backup", {
    let pulse = pulse.clone();
    move || {
        let p = pulse.clone();
        async move {
            p.enqueue("backup_job", json!({})).await.ok();
        }
    }
}).await?;
```

## Best Practices

1. **Keep Tasks Fast**: Long-running tasks block the scheduler
2. **Error Handling**: Tasks should handle errors gracefully
3. **Use Pulse for Complex Jobs**: For retries and persistence
4. **Avoid Overlaps**: Ensure tasks complete before next run
