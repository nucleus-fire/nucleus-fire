# Testing Utilities Guide

Nucleus provides comprehensive testing utilities including MockServer, TestClient, and Factory pattern for test data generation.

## Quick Start

```rust
use nucleus_std::testing::{MockServer, TestClient, Factory};

#[tokio::test]
async fn test_api_integration() {
    // Start mock server
    let mock = MockServer::start().await.unwrap();
    mock.expect("GET", "/api/users/1")
        .respond_with_json(json!({"id": 1, "name": "Test"}))
        .mount()
        .await;

    // Create test client
    let client = TestClient::new(&mock.url());
    let response = client.get("/api/users/1").await.unwrap();
    
    assert_eq!(response.status(), 200);
    let user: serde_json::Value = response.json().unwrap();
    assert_eq!(user["name"], "Test");
}
```

## MockServer

MockServer lets you mock external HTTP APIs for testing.

### Starting a Server

```rust
let mock = MockServer::start().await?;
println!("Mock server running at: {}", mock.url());
```

### Creating Expectations

```rust
// Basic expectation
mock.expect("GET", "/api/users")
    .respond_with_status(200)
    .respond_with_json(json!({"users": []}))
    .mount()
    .await;

// With request matching
mock.expect("POST", "/api/users")
    .with_header("Content-Type", "application/json")
    .with_body(json!({"name": "Alice"}))
    .respond_with_status(201)
    .respond_with_json(json!({"id": 1, "name": "Alice"}))
    .mount()
    .await;
```

### Response Configuration

```rust
mock.expect("GET", "/api/data")
    .respond_with_status(200)
    .respond_with_json(json!({"data": "value"}))
    .respond_with_header("X-Custom", "header-value")
    .mount()
    .await;
```

### Expected Call Count

```rust
mock.expect("GET", "/api/health")
    .respond_with_status(200)
    .times(3)  // Expect exactly 3 calls
    .mount()
    .await;

// Later, verify expectations
mock.verify().await?;
```

### Reset and Call Count

```rust
// Check how many times an endpoint was called
let count = mock.call_count("GET", "/api/users").await;
println!("Called {} times", count);

// Reset all expectations
mock.reset().await;
```

## TestClient

TestClient simplifies making HTTP requests in tests.

### Creating a Client

```rust
let client = TestClient::new("http://localhost:3000");
```

### Adding Authentication

```rust
let client = TestClient::new("http://localhost:3000")
    .with_auth("jwt_token_here");
```

### Adding Headers and Cookies

```rust
let client = TestClient::new("http://localhost:3000")
    .with_header("X-API-Key", "secret")
    .with_cookie("session", "abc123")
    .with_cookie("theme", "dark");
```

### Making Requests

```rust
// GET request
let response = client.get("/api/users").await?;

// POST with JSON body
let response = client.post("/api/users", &json!({
    "name": "Alice",
    "email": "alice@example.com"
})).await?;

// PUT request
let response = client.put("/api/users/1", &updated_user).await?;

// PATCH request
let response = client.patch("/api/users/1", &partial_update).await?;

// DELETE request
let response = client.delete("/api/users/1").await?;

// Form data
let response = client.post_form("/login", &[
    ("username", "alice"),
    ("password", "secret")
]).await?;
```

### Working with Responses

```rust
let response = client.get("/api/users/1").await?;

// Check status
assert_eq!(response.status(), 200);
assert!(response.is_success());      // 2xx
assert!(response.is_redirect());     // 3xx
assert!(response.is_client_error()); // 4xx
assert!(response.is_server_error()); // 5xx

// Get headers
let content_type = response.header("content-type");

// Parse JSON
let user: User = response.json()?;

// Raw text
let text = response.text();
```

### Fluent Assertions

```rust
client.get("/api/users/1").await?
    .assert_status(200)
    .assert_json_has("email")
    .assert_contains("@example.com");
```

## Factory Pattern

Factory helps generate consistent test data.

### User Factory

```rust
// Default user
let user = Factory::user().build();
// Random ID, email, name="Test User", role="user", active=true

// Customized user
let admin = Factory::user()
    .id("admin_1")
    .email("admin@company.com")
    .name("Admin User")
    .admin()  // Sets role to "admin"
    .build();

// Inactive user
let inactive = Factory::user().inactive().build();
```

### Random Data Generation

```rust
// Random string
let code = Factory::random_string(16);

// Random email
let email = Factory::random_email(); // e.g., "abc12345@test.com"

// Random ID
let id = Factory::random_id(); // 12-character string
```

### Generating Sequences

```rust
// Generate 10 users
let users: Vec<UserFactory> = Factory::sequence(10, |i| {
    Factory::user()
        .id(&format!("user_{}", i))
        .email(&format!("user{}@test.com", i))
        .build()
});
```

### Converting to JSON

```rust
let user = Factory::user().email("test@test.com").build();
let json = user.to_json();
// Use in mock responses or request bodies
```

## JSON Assertions

```rust
use nucleus_std::testing::{assert_json_eq, assert_json_contains};

// Exact match
assert_json_eq(
    &json!({"id": 1, "name": "Test"}),
    &json!({"id": 1, "name": "Test"})
);

// Subset match
assert_json_contains(
    &json!({"id": 1, "name": "Test", "extra": true}),
    &json!({"id": 1, "name": "Test"})
);
```

## Async Waiting

```rust
use nucleus_std::testing::wait_for;

// Wait for a condition with timeout
wait_for(5000, || async {
    // Return true when condition is met
    check_something().await
}).await?;
```

## Complete Example

```rust
use nucleus_std::testing::{MockServer, TestClient, Factory, assert_json_contains};

#[tokio::test]
async fn test_user_creation_flow() {
    // Setup mock for external email service
    let email_mock = MockServer::start().await.unwrap();
    email_mock.expect("POST", "/api/send")
        .respond_with_status(200)
        .times(1)
        .mount()
        .await;

    // Create test user data
    let new_user = Factory::user()
        .email("newuser@test.com")
        .name("New User")
        .build();

    // Test the API
    let client = TestClient::new("http://localhost:3000");
    
    let response = client.post("/api/users", &new_user.to_json()).await.unwrap();
    
    response
        .assert_status(201)
        .assert_json_has("id")
        .assert_json_has("email");

    let created: serde_json::Value = response.json().unwrap();
    assert_json_contains(&created, &json!({
        "email": "newuser@test.com",
        "name": "New User"
    }));

    // Verify email was sent
    email_mock.verify().await.unwrap();
}

#[tokio::test]
async fn test_authentication_required() {
    let client = TestClient::new("http://localhost:3000");
    
    // Without auth - should fail
    let response = client.get("/api/protected").await.unwrap();
    assert_eq!(response.status(), 401);
    
    // With auth - should succeed
    let authed_client = client.with_auth("valid_token");
    let response = authed_client.get("/api/protected").await.unwrap();
    assert_eq!(response.status(), 200);
}
```

## Error Handling

```rust
use nucleus_std::testing::TestError;

match client.get("/api/data").await {
    Ok(response) => handle(response),
    Err(TestError::RequestError(msg)) => println!("Request failed: {}", msg),
    Err(TestError::Timeout) => println!("Request timed out"),
    Err(e) => println!("Other error: {}", e),
}
```
