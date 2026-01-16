# Testing Guide

> Complete guide to testing Nucleus applications.

## Overview

Nucleus uses Rust's built-in testing framework. Tests are organized by type:

| Type | Location | Purpose |
|------|----------|---------|
| Unit Tests | `src/logic/*.rs` | Test individual functions |
| Integration Tests | `tests/` | Test full request/response |
| E2E Tests | `tests/e2e/` | Browser-based testing |

---

## Unit Tests

### Basic Structure

Place tests in the same file as your logic:

```rust
// src/logic/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }
}
```

### Running Tests

```bash
# All tests
nucleus test
# or
cargo test

# Specific test
cargo test test_add

# With output
cargo test -- --nocapture
```

---

## Testing Database Logic

### Setup Test Database

```rust
// src/logic/db.rs
use sqlx::SqlitePool;

pub async fn get_test_pool() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // Run migrations
    sqlx::query(include_str!("../../migrations/001_init.sql"))
        .execute(&pool)
        .await
        .unwrap();
    
    pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let pool = get_test_pool().await;
        
        let id = create_user(&pool, "test@example.com").await.unwrap();
        assert!(id > 0);
        
        let user = get_user(&pool, id).await.unwrap();
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_duplicate_email() {
        let pool = get_test_pool().await;
        
        create_user(&pool, "test@example.com").await.unwrap();
        let result = create_user(&pool, "test@example.com").await;
        
        assert!(result.is_err());
    }
}
```

---

## Integration Tests

### Test Full HTTP Endpoints

Create `tests/integration.rs`:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

// Import your app
use myapp::create_app;

#[tokio::test]
async fn test_homepage() {
    let app = create_app().await;
    
    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_not_found() {
    let app = create_app().await;
    
    let response = app
        .oneshot(Request::get("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_post() {
    let app = create_app().await;
    
    let response = app
        .oneshot(
            Request::post("/api/posts")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"title": "Test", "content": "Hello"}"#))
                .unwrap()
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### Using axum-test (Recommended)

```toml
# Cargo.toml
[dev-dependencies]
axum-test = "14"
```

```rust
use axum_test::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_user_flow() {
    let app = create_app().await;
    let server = TestServer::new(app).unwrap();
    
    // Register user
    let response = server
        .post("/api/register")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .await;
    
    assert_eq!(response.status_code(), 201);
    
    // Login
    let response = server
        .post("/api/login")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .await;
    
    assert_eq!(response.status_code(), 200);
    
    let body: serde_json::Value = response.json();
    assert!(body["token"].is_string());
}
```

---

## Testing Authentication

```rust
#[tokio::test]
async fn test_protected_route_without_token() {
    let server = TestServer::new(create_app().await).unwrap();
    
    let response = server.get("/api/profile").await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_protected_route_with_token() {
    let server = TestServer::new(create_app().await).unwrap();
    
    // Create user and get token
    let token = create_test_user_and_login(&server).await;
    
    let response = server
        .get("/api/profile")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    
    assert_eq!(response.status_code(), 200);
}
```

---

## Test Fixtures

### Create Test Data

```rust
// tests/fixtures.rs
use sqlx::SqlitePool;

pub struct TestFixtures {
    pub pool: SqlitePool,
    pub user_id: i64,
    pub admin_id: i64,
}

impl TestFixtures {
    pub async fn new() -> Self {
        let pool = create_test_pool().await;
        
        let user_id = create_user(&pool, "user@test.com", "user").await;
        let admin_id = create_user(&pool, "admin@test.com", "admin").await;
        
        Self { pool, user_id, admin_id }
    }
}

// Usage
#[tokio::test]
async fn test_with_fixtures() {
    let fixtures = TestFixtures::new().await;
    
    let user = get_user(&fixtures.pool, fixtures.user_id).await;
    assert_eq!(user.role, "user");
}
```

---

## Mocking External Services

```rust
use mockall::{automock, predicate::*};

#[automock]
trait EmailService {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

#[tokio::test]
async fn test_registration_sends_email() {
    let mut mock_email = MockEmailService::new();
    
    mock_email
        .expect_send()
        .with(eq("test@example.com"), always(), always())
        .times(1)
        .returning(|_, _, _| Ok(()));
    
    let result = register_user(mock_email, "test@example.com").await;
    assert!(result.is_ok());
}
```

---

## Test Organization

```
tests/
├── common/
│   ├── mod.rs          # Shared test utilities
│   └── fixtures.rs     # Test data
├── integration.rs      # API tests
├── auth_tests.rs       # Authentication tests
└── db_tests.rs         # Database tests
```

**`tests/common/mod.rs`:**
```rust
use sqlx::SqlitePool;

pub async fn setup() -> SqlitePool {
    dotenv::dotenv().ok();
    
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    run_migrations(&pool).await;
    pool
}

pub async fn cleanup(pool: &SqlitePool) {
    sqlx::query("DELETE FROM users").execute(pool).await.ok();
    sqlx::query("DELETE FROM posts").execute(pool).await.ok();
}
```

---

## Running Tests

```bash
# All tests
cargo test

# Specific file
cargo test --test integration

# Specific test
cargo test test_create_user

# With logs
RUST_LOG=debug cargo test -- --nocapture

# Parallel (default) or serial
cargo test -- --test-threads=1
```

---

## CI Integration

### GitHub Actions

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Cache
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: cargo test --workspace
```

---

## Best Practices

1. **Test behavior, not implementation** - Focus on inputs and outputs
2. **Use descriptive test names** - `test_user_cannot_delete_others_posts`
3. **One assertion per test** (when practical)
4. **Clean up after tests** - Use in-memory database
5. **Test edge cases** - Empty inputs, nulls, boundaries
6. **Keep tests fast** - Avoid unnecessary I/O
