# Best Practices

A comprehensive guide to building production-ready Nucleus applications.

---

## Security

### Authentication & Password Handling

**Never store plaintext passwords.** Always use Fortress for cryptographic operations:

```rust
use nucleus_std::fortress::Fortress;

// Hashing passwords (uses Argon2id)
let hash = Fortress::hash_password("user_password")?;

// Verifying passwords
match Fortress::verify_password("user_password", &stored_hash) {
    Ok(true) => println!("Valid credentials"),
    Ok(false) => println!("Invalid password"),
    Err(e) => eprintln!("Verification error: {}", e),
}
```

**Password Requirements:**
- Minimum 8 characters (configurable)
- Consider using zxcvbn for strength checking
- Never log passwords, even in debug mode

### Session Management

```rust
use nucleus_std::fortress::Session;

// Create session with expiry
let session = Session::create(&user_id, Duration::hours(24))?;

// Validate session on each request
let user_id = Session::validate(&session_token)?;

// Invalidate on logout
Session::destroy(&session_token)?;
```

### CSRF Protection

All forms should include CSRF tokens:

```html
<form method="POST">
    <n:csrf />  <!-- Automatically injects token -->
    <input type="email" name="email" required>
    <button type="submit">Submit</button>
</form>
```

### XSS Prevention

Nucleus automatically escapes output by default:

```html
<!-- Safe: Automatically escaped -->
<p>{user.bio}</p>

<!-- Dangerous: Only use with trusted content -->
<div n:html="trusted_html_content"></div>
```

### SQL Injection Prevention

Always use parameterized queries:

```rust
// ✅ Safe: Parameterized query
sqlx::query("SELECT * FROM users WHERE email = ?")
    .bind(&email)
    .fetch_one(pool)
    .await?;

// ❌ NEVER do this
let query = format!("SELECT * FROM users WHERE email = '{}'", email);
```

### Permission Checks

Implement authorization at the handler level:

```rust
<n:action>
    use nucleus_std::fortress::Fortress;
    
    // Check authentication
    let user = match Session::get_user(&headers) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };
    
    // Check authorization
    if !Fortress::check_permission(&user, "posts:write") {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }
    
    // Proceed with action...
</n:action>
```

### Rate Limiting

Protect endpoints from abuse:

```rust
use nucleus_std::shield::RateLimiter;

// In middleware.rs
pub async fn rate_limit(req: Request, next: Next) -> Response {
    let ip = req.client_ip();
    
    if !RateLimiter::check(ip, "api", 100, Duration::minutes(1)) {
        return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
    }
    
    next.run(req).await
}
```

---

## Database Best Practices

### Connection Pooling

Configure appropriate pool sizes:

```toml
# nucleus.config
[database]
url = "postgres://user:pass@localhost/myapp"
pool_size = 20           # Max connections
pool_min = 5             # Min idle connections
pool_timeout = 30        # Connection timeout (seconds)
```

### Transactions

Use transactions for multiple related writes:

```rust
use nucleus_std::photon::transaction;

// All operations are atomic
transaction(|tx| async move {
    let user = User::create(&new_user).within(&tx).await?;
    let profile = Profile::create(&UserProfile {
        user_id: user.id,
        ..default_profile
    }).within(&tx).await?;
    
    AuditLog::record("user_created", user.id).within(&tx).await?;
    
    Ok(())
}).await?;
```

### Indexing Strategy

Add indexes for frequently queried columns:

```sql
-- migrations/20251227000001_add_indexes.sql
-- UP

-- Primary lookup patterns
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

-- Foreign key relationships
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_comments_post_id ON comments(post_id);

-- Composite indexes for common queries
CREATE INDEX idx_posts_user_status ON posts(user_id, status);

-- DOWN
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_posts_user_id;
DROP INDEX IF EXISTS idx_comments_post_id;
DROP INDEX IF EXISTS idx_posts_user_status;
```

### Query Optimization

```rust
// ✅ Select only needed fields
let users = User::query()
    .select(&["id", "name", "email"])
    .filter("active", true)
    .limit(50)
    .all()
    .await?;

// ✅ Use pagination
let page = params.get("page").unwrap_or(1);
let users = User::query()
    .order_by("created_at", "DESC")
    .paginate(page, 20)
    .await?;

// ❌ Avoid N+1 queries
// Bad: Fetches posts, then user for each post
for post in posts {
    let user = User::find(post.user_id).await?;
}

// ✅ Eager load relationships
let posts = Post::query()
    .with("user")  // JOIN users
    .all()
    .await?;
```

### Soft Deletes

Prefer soft deletes for important data:

```rust
// Model with soft delete
<n:model name="User">
    id: i64
    email: String
    deleted_at: Option<DateTime>  // Null = not deleted
</n:model>

// Query excludes deleted by default
let users = User::query().all().await?;

// Include deleted records
let all_users = User::query().with_trashed().all().await?;

// Soft delete
user.delete().await?;  // Sets deleted_at

// Hard delete (permanent)
user.force_delete().await?;
```

---

## Financial Data

### Never Use Floating Point

Floats are imprecise and cause rounding errors:

```rust
// ❌ WRONG: Float arithmetic is imprecise
let price: f64 = 0.1 + 0.2;  // = 0.30000000000000004

// ✅ CORRECT: Use Decimal for money
use rust_decimal::Decimal;
use std::str::FromStr;

let price = Decimal::from_str("19.99")?;
let quantity = Decimal::from(3);
let total = price * quantity;  // 59.97 exactly

// Store as string or integer cents in database
let cents = (total * Decimal::from(100)).to_i64().unwrap();
```

### Double-Entry Accounting

Never manually adjust balances:

```rust
use nucleus_std::vault::Vault;

// ✅ Atomic transfer - debits and credits in one transaction
Vault::transfer(
    &from_account,
    &to_account,
    Decimal::from_str("100.00")?,
    "Payment for order #12345"
).await?;

// ❌ NEVER do this
from_account.balance -= amount;
to_account.balance += amount;
```

### Audit Trails

Log all financial operations:

```rust
use nucleus_std::beacon::AuditLog;

AuditLog::record("payment_processed", json!({
    "order_id": order.id,
    "amount": amount.to_string(),
    "currency": "USD",
    "method": "credit_card",
    "user_id": user.id,
})).await?;
```

---

## Performance

### Async I/O

Never block the reactor:

```rust
// ✅ Good: Non-blocking async operations
let user = User::find(id).await?;
let response = http_client.get(url).await?;
let file = tokio::fs::read("data.json").await?;

// ❌ Bad: Blocks the entire reactor
std::thread::sleep(Duration::from_secs(1));
std::fs::read("data.json");  // Sync file I/O
```

### Background Jobs

Offload heavy work to job queues:

```rust
use nucleus_std::pulse::Pulse;

// Queue email sending
Pulse::enqueue("send_email", json!({
    "to": "user@example.com",
    "template": "welcome",
    "data": { "name": user.name }
})).await?;

// Queue image processing
Pulse::enqueue("process_image", json!({
    "path": "/uploads/photo.jpg",
    "operations": ["resize:800x600", "optimize", "webp"]
})).await?;

// Queue report generation
Pulse::enqueue("generate_report", json!({
    "type": "monthly_sales",
    "month": "2025-12",
    "format": "pdf"
})).await?;
```

### Caching

Cache expensive operations:

```rust
use nucleus_std::cache::Cache;

// Cache with TTL (seconds)
let users = Cache::remember("active_users", 300, || async {
    User::query()
        .filter("active", true)
        .order_by("name", "ASC")
        .all()
        .await
}).await?;

// Cache with tags for invalidation
let post = Cache::tags(&["posts", &format!("user:{}", user_id)])
    .remember(&format!("post:{}", post_id), 3600, || async {
        Post::find(post_id).await
    }).await?;

// Invalidate by tag
Cache::invalidate_tag("posts").await?;

// Manual cache operations
Cache::set("key", &value, 600).await?;
let value: Option<T> = Cache::get("key").await?;
Cache::delete("key").await?;
```

### Response Compression

Nucleus automatically enables Brotli and Gzip compression. Ensure assets are optimized:

```bash
# During build, CSS is automatically minified
nucleus build

# Output:
# static/assets/output.css  34.0kb (before: 120kb)
```

### Database Connection Reuse

Don't create connections per request:

```rust
// ✅ Use the global pool
let pool = db();
sqlx::query("SELECT * FROM users")
    .fetch_all(pool.as_sqlite().unwrap())
    .await?;

// ❌ Don't create new connections
let pool = SqlitePool::connect("sqlite:data.db").await?;  // Wrong!
```

### External Resources
 
**Always localize external assets** (fonts, scripts, images).
 
**Why?**
1.  **Performance**: Reduces DNS lookups and TLS handshakes. Leverages HTTP/2 multiplexing on your own domain.
2.  **Privacy**: Prevents leaking visitor IP addresses to third parties (e.g., Google Fonts).
3.  **Reliability**: Your site won't break if a third-party CDN goes down.
 
```html
<!-- ❌ Bad: External dependency -->
<link href="https://fonts.googleapis.com/css2?family=Inter" rel="stylesheet">
 
<!-- ✅ Good: Localized asset -->
<style>
    @font-face {
        font-family: 'Inter';
        src: url("/assets/fonts/Inter.woff2") format("woff2");
    }
</style>
```

---

## Error Handling

### Use Result Types

Never panic in production code:

```rust
// ✅ Return Result for fallible operations
async fn get_user(id: i64) -> Result<User, AppError> {
    User::find(id)
        .await
        .map_err(|e| AppError::Database(e))?
        .ok_or(AppError::NotFound("User not found"))
}

// ❌ Avoid unwrap/expect in production paths
let user = User::find(id).await.unwrap();  // Panics on error!
```

### Custom Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };
        
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### Logging

Use structured logging:

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(password))]
async fn login(email: &str, password: &str) -> Result<User, AppError> {
    info!(email = %email, "Login attempt");
    
    match authenticate(email, password).await {
        Ok(user) => {
            info!(user_id = %user.id, "Login successful");
            Ok(user)
        }
        Err(e) => {
            warn!(email = %email, error = %e, "Login failed");
            Err(e)
        }
    }
}
```

---

## Testing

### Unit Tests

Test individual functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("user@example.com"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email(""));
    }
    
    #[tokio::test]
    async fn test_password_hashing() {
        let hash = Fortress::hash_password("password123").unwrap();
        assert!(Fortress::verify_password("password123", &hash).unwrap());
        assert!(!Fortress::verify_password("wrong", &hash).unwrap());
    }
}
```

### Integration Tests

Test database operations:

```rust
#[tokio::test]
async fn test_user_creation() {
    // Use test database
    init_test_db().await;
    
    let user = User::create(&NewUser {
        email: "test@example.com".into(),
        name: "Test User".into(),
    }).await.unwrap();
    
    assert!(user.id > 0);
    assert_eq!(user.email, "test@example.com");
    
    // Cleanup
    user.delete().await.unwrap();
}
```

### View Tests

Test HTML output:

```html
<n:view>
    <n:test>
        test "renders greeting with name" {
            let html = render_with(json!({ "name": "Alice" }));
            assert!(html.contains("Hello, Alice"));
        }
        
        test "shows login button when not authenticated" {
            let html = render_with(json!({ "user": null }));
            assert!(html.contains("Login"));
        }
    </n:test>
    
    <h1>Hello, {name}</h1>
</n:view>
```

### Mocking External Services

```rust
use nucleus_std::testing::mock;

#[tokio::test]
async fn test_email_notification() {
    // Mock the email service
    let email_mock = mock::email();
    
    // Run the code that sends email
    notify_user_signup("user@test.com").await.unwrap();
    
    // Verify email was sent
    email_mock.assert_called_once();
    email_mock.assert_sent_to("user@test.com");
}
```

---

## Code Organization

### Project Structure

```
my-app/
├── nucleus.config          # Configuration
├── Cargo.toml              # Rust dependencies
├── migrations/             # Database migrations
│   ├── 20251201_create_users.sql
│   └── 20251202_create_posts.sql
├── src/
│   ├── main.rs             # Entry point
│   ├── middleware.rs       # Custom middleware
│   ├── models/             # Data models
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── post.rs
│   ├── services/           # Business logic
│   │   ├── mod.rs
│   │   └── email.rs
│   └── views/              # View files (.ncl)
│       ├── layout.ncl
│       ├── index.ncl
│       ├── auth/
│       │   ├── login.ncl
│       │   └── register.ncl
│       └── dashboard/
│           └── index.ncl
├── static/                 # Static assets
│   ├── assets/
│   ├── css/
│   └── images/
└── tests/                  # Integration tests
    └── api_tests.rs
```

### Separation of Concerns

Keep views thin, put logic in services:

```rust
// ✅ Good: Logic in service
// src/services/user_service.rs
pub async fn register_user(data: RegisterData) -> Result<User, AppError> {
    validate_registration(&data)?;
    
    let hash = Fortress::hash_password(&data.password)?;
    
    let user = User::create(&NewUser {
        email: data.email,
        password_hash: hash,
    }).await?;
    
    send_welcome_email(&user).await?;
    
    Ok(user)
}

// In view action - just calls service
<n:action>
    use crate::services::user_service;
    
    let result = user_service::register_user(form_data).await;
    match result {
        Ok(user) => Redirect::to("/dashboard"),
        Err(e) => render_error(e),
    }
</n:action>
```

---

## Environment Configuration

### Separate Configs by Environment

```toml
# nucleus.config (base)
[app]
name = "My App"

# nucleus.production.config (production overrides)
[database]
url = "${DATABASE_URL}"  # From environment variable

[app]
env = "production"
```

### Secrets Management

Never commit secrets:

```bash
# .gitignore
nucleus.*.config
.env

# Use environment variables
export SECRET_KEY="your-production-secret"
export DATABASE_URL="postgres://..."
```

---

## Deployment Checklist

Before deploying to production:

- [ ] Set `env = "production"` in config
- [ ] Use strong, unique `secret_key`
- [ ] Enable SSL/TLS via reverse proxy
- [ ] Configure database connection pooling
- [ ] Set up log aggregation
- [ ] Configure monitoring and alerting
- [ ] Test with production-like data
- [ ] Review security headers
- [ ] Enable rate limiting on sensitive endpoints
- [ ] Set up automated backups
- [ ] Document rollback procedures

---

## 10. Local Resource Strategy

For exemplary performance, privacy, and reliability, Nucleus projects should avoid dependencies on public CDNs (e.g., Google Fonts, Tailwind CDN, JSDelivr) in production.

### Why Localize?

*   **Performance**: Eliminates DNS lookups and TLS handshakes to third-party domains.
*   **Reliability**: Your app works even if the CDN is down or blocked.
*   **Privacy**: Prevents leaking user IP addresses to third parties.
*   **Offline Capable**: Essential for PWA and local-first applications.

### Implementation Guide

#### 1. CSS Frameworks (Tailwind)
Instead of using the CDN script, set up a local build process:
1.  Initialize `package.json` and install `tailwindcss`.
2.  Create a `input.css` file with `@tailwind` directives.
3.  Run the Tailwind CLI to generate a static CSS file (e.g., `static/styles.css`).
4.  Link the generated file in your HTML: `<link href="/assets/styles.css" rel="stylesheet">`.

#### 2. Fonts
Download font files (WOFF2) and serve them from your `static/fonts` directory. Use `@font-face` within your local CSS to reference them.

#### 3. JavaScript Libraries
Vendor large libraries by downloading the minified versions into `static/vendor/` or bundling them with your build process.
