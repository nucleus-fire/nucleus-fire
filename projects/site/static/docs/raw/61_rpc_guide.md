# RPC & Server Functions Guide

Server Functions (RPC) allow you to call backend Rust functions directly from your frontend code with full type safety.

---

## Overview

Server Functions are Rust functions marked with `#[server]` that execute on the server but can be called from client-side code. The Nucleus compiler automatically:
- Generates client-side stubs
- Handles serialization/deserialization
- Manages HTTP transport
- Preserves type safety end-to-end

---

## Basic Usage

### Defining a Server Function

```rust
// src/services/users.rs
use nucleus_std::server;

#[server]
pub async fn get_user(id: i64) -> Result<User, AppError> {
    let user = User::find(id).await?;
    Ok(user)
}

#[server]
pub async fn create_user(email: String, name: String) -> Result<User, AppError> {
    let user = User::create()
        .set("email", &email)
        .set("name", &name)
        .save()
        .await?;
    Ok(user)
}
```

### Calling from Client

```html
<!-- src/views/profile.ncl -->
<n:view>
    <n:island client:load>
        <n:script>
            // Called via generated RPC
            let user = get_user(123).await?;
        </n:script>
        
        <div class="profile">
            <h1>{user.name}</h1>
        </div>
    </n:island>
</n:view>
```

---

## Function Parameters

### Supported Types

Server functions support any type that implements `Serialize` and `Deserialize`:

```rust
#[server]
pub async fn process_order(
    items: Vec<OrderItem>,       // Complex types
    customer_id: i64,            // Primitives
    notes: Option<String>,       // Optionals
    metadata: HashMap<String, String>,  // Collections
) -> Result<Order, AppError> {
    // ...
}
```

### Validation

Combine with the `Validate` derive for automatic input validation:

```rust
use nucleus_std::forms::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserInput {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 2, max = 100))]
    name: String,
    
    #[validate(range(min = 18))]
    age: i32,
}

#[server]
pub async fn create_user(input: CreateUserInput) -> Result<User, AppError> {
    input.validate()?;  // Throws if invalid
    // ...
}
```

---

## Error Handling

### Custom Error Types

Define typed errors that propagate to the client:

```rust
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    
    #[error("Email already registered")]
    EmailTaken,
    
    #[error("Invalid input: {0}")]
    ValidationError(String),
}

#[server]
pub async fn register(email: String) -> Result<User, UserError> {
    if User::exists_by_email(&email).await? {
        return Err(UserError::EmailTaken);
    }
    // ...
}
```

### Client-Side Error Handling

```html
<n:island client:load>
    <n:script>
        match register(email).await {
            Ok(user) => {
                // Success
            }
            Err(UserError::EmailTaken) => {
                show_error("Email already registered");
            }
            Err(e) => {
                show_error(format!("Error: {}", e));
            }
        }
    </n:script>
</n:island>
```

---

## Authentication

### Accessing Current User

Server functions can access the authentication context:

```rust
use nucleus_std::fortress::AuthUser;

#[server]
pub async fn get_my_profile(auth: AuthUser) -> Result<Profile, AppError> {
    // auth.id, auth.email, auth.roles are available
    let profile = Profile::find_by_user(auth.id).await?;
    Ok(profile)
}

#[server]
pub async fn admin_action(auth: AuthUser) -> Result<(), AppError> {
    if !auth.roles.contains(&"admin".to_string()) {
        return Err(AppError::Forbidden);
    }
    // Admin-only logic
    Ok(())
}
```

### Optional Auth

For functions that work with or without authentication:

```rust
use nucleus_std::fortress::OptionalAuth;

#[server]
pub async fn get_content(auth: OptionalAuth) -> Result<Content, AppError> {
    let content = Content::public().await?;
    
    if let Some(user) = auth.user {
        // Include personalized content
        content.personalize_for(user.id);
    }
    
    Ok(content)
}
```

---

## Caching

### Cached Results

Cache expensive computations:

```rust
use nucleus_std::cache::cached;

#[server]
#[cached(ttl = "5m", key = "stats")]
pub async fn get_dashboard_stats() -> Result<Stats, AppError> {
    // Expensive aggregation
    let stats = Stats::compute().await?;
    Ok(stats)
}

#[server]
#[cached(ttl = "1h", key = "user:{id}")]
pub async fn get_user_cached(id: i64) -> Result<User, AppError> {
    User::find(id).await
}
```

---

## Rate Limiting

Protect server functions from abuse:

```rust
use nucleus_std::fortress::RateLimiter;

#[server]
#[rate_limit(requests = 100, window = "1m")]
pub async fn api_endpoint() -> Result<Data, AppError> {
    // Limited to 100 requests per minute
    Ok(Data::fetch().await?)
}

#[server]
#[rate_limit(requests = 5, window = "1h", key = "user:{auth.id}")]
pub async fn expensive_operation(auth: AuthUser) -> Result<(), AppError> {
    // 5 per user per hour
    Ok(())
}
```

---

## Background Execution

For long-running operations, queue as a background job:

```rust
use nucleus_std::pulse::{Pulse, Job};

#[server]
pub async fn start_report(auth: AuthUser) -> Result<JobId, AppError> {
    let job_id = Pulse::enqueue(Job::new("generate_report")
        .with_data(json!({ "user_id": auth.id }))
    ).await?;
    
    Ok(job_id)
}

#[server]
pub async fn check_job_status(job_id: String) -> Result<JobStatus, AppError> {
    let status = Pulse::status(&job_id).await?;
    Ok(status)
}
```

---

## Best Practices

### 1. Keep Functions Focused
```rust
// ✅ Good - single responsibility
#[server]
pub async fn get_user(id: i64) -> Result<User, AppError> { ... }

#[server]
pub async fn update_user(id: i64, data: UpdateUser) -> Result<User, AppError> { ... }

// ❌ Bad - too many responsibilities
#[server]
pub async fn manage_user(action: String, id: i64, data: Option<Value>) { ... }
```

### 2. Use Typed Inputs
```rust
// ✅ Good - structured input
#[derive(Deserialize)]
pub struct CreatePostInput {
    title: String,
    content: String,
    tags: Vec<String>,
}

#[server]
pub async fn create_post(input: CreatePostInput) -> Result<Post, AppError> { ... }

// ❌ Bad - loose parameters
#[server]
pub async fn create_post(title: String, content: String, tags: String) { ... }
```

### 3. Return Structured Responses
```rust
// ✅ Good - typed response
#[derive(Serialize)]
pub struct PaginatedUsers {
    users: Vec<User>,
    total: i64,
    page: i32,
}

#[server]
pub async fn list_users(page: i32) -> Result<PaginatedUsers, AppError> { ... }
```

---

## Transport Details

Under the hood, server functions use:
- **Protocol**: HTTP POST
- **Encoding**: JSON (MessagePack for binary)
- **Endpoint**: `/_rpc/{function_name}`
- **Headers**: `Content-Type: application/json`

The compiler generates all transport code automatically—you never interact with HTTP directly.
