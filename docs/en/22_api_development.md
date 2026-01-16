# API Development Guide

> Complete guide to building REST APIs with Nucleus.

## Overview

Nucleus supports building APIs alongside server-rendered pages. APIs are defined in the `src/logic/` directory and exposed through route handlers.

---

## Quick Start

### 1. Create API Module

```rust
// src/logic/api.rs
use axum::{
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Json<Self> {
        Json(Self {
            success: true,
            data: Some(data),
            error: None,
        })
    }
    
    pub fn error(message: String) -> (StatusCode, Json<Self>) {
        (StatusCode::BAD_REQUEST, Json(Self {
            success: false,
            data: None,
            error: Some(message),
        }))
    }
}
```

### 2. Define Endpoints

```rust
// src/logic/api/users.rs
use axum::{
    extract::{Path, State, Query},
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

// GET /api/users
pub async fn list_users(
    State(pool): State<SqlitePool>,
) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, email, name FROM users")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    
    Json(users)
}

// GET /api/users/:id
pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT id, email, name FROM users WHERE id = ?", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(user))
}

// POST /api/users
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let hash = nucleus_std::fortress::hash_password(&input.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash, name) VALUES (?, ?, ?)",
        input.email,
        hash,
        input.name
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    let user = User {
        id: result.last_insert_rowid(),
        email: input.email,
        name: input.name,
    };
    
    Ok((StatusCode::CREATED, Json(user)))
}

// PUT /api/users/:id
pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query!(
        "UPDATE users SET name = ? WHERE id = ?",
        input.name,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    get_user(State(pool), Path(id)).await
}

// DELETE /api/users/:id
pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> StatusCode {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&pool)
        .await;
    
    match result {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
```

---

## Route Registration

### In Your Main App

```rust
use axum::{Router, routing::{get, post, put, delete}};

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(api::users::list_users).post(api::users::create_user))
        .route("/users/:id", get(api::users::get_user)
            .put(api::users::update_user)
            .delete(api::users::delete_user))
        .route("/posts", get(api::posts::list_posts).post(api::posts::create_post))
        .route("/posts/:id", get(api::posts::get_post))
}

// In main
let app = Router::new()
    .nest("/api", api_routes())
    // ... other routes
    .with_state(app_state);
```

---

## Request Handling

### Path Parameters

```rust
// GET /api/posts/:post_id/comments/:comment_id
pub async fn get_comment(
    Path((post_id, comment_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    // ...
}
```

### Query Parameters

```rust
#[derive(Deserialize)]
pub struct ListParams {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    sort: Option<String>,
}

pub async fn list_posts(
    Query(params): Query<ListParams>,
) -> Json<Vec<Post>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;
    
    // Build query with params...
}
```

### Headers

```rust
use axum::http::HeaderMap;

pub async fn process_webhook(
    headers: HeaderMap,
    body: String,
) -> StatusCode {
    let signature = headers
        .get("X-Webhook-Signature")
        .and_then(|h| h.to_str().ok());
    
    // Verify signature...
    StatusCode::OK
}
```

### Request Body Types

```rust
// JSON
pub async fn create_json(Json(data): Json<CreateRequest>) -> impl IntoResponse { }

// Form data
pub async fn create_form(Form(data): Form<CreateRequest>) -> impl IntoResponse { }

// Raw bytes
pub async fn upload(body: Bytes) -> impl IntoResponse { }

// Multipart
use axum::extract::Multipart;
pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        // Process file...
    }
}
```

---

## Response Types

### JSON Response

```rust
#[derive(Serialize)]
struct User { id: i64, name: String }

pub async fn get_user() -> Json<User> {
    Json(User { id: 1, name: "Alice".into() })
}
```

### Status Code with Body

```rust
pub async fn create() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(user))
}
```

### Empty Response

```rust
pub async fn delete() -> StatusCode {
    StatusCode::NO_CONTENT
}
```

### Custom Headers

```rust
use axum::http::header;

pub async fn download() -> impl IntoResponse {
    let body = "file contents";
    
    (
        [(header::CONTENT_TYPE, "application/octet-stream"),
         (header::CONTENT_DISPOSITION, "attachment; filename=\"data.txt\"")],
        body
    )
}
```

---

## Error Handling
Nucleus standardizes error handling using the `NucleusError` enum and the `anyhow`/`thiserror` pattern.

### Using `NucleusError`
The `nucleus-std` crate exports a unified error type `NucleusError` and a result alias `Result<T>`.

```rust
use nucleus_std::errors::{NucleusError, Result};

pub fn complex_operation() -> Result<()> {
    // Database errors are automatically converted
    let user = sqlx::query!("...").fetch_one(&pool)?;
    
    // Config errors
    let config = Config::try_load()?;
    
    // Custom errors use specific variants
    if user.is_banned {
        return Err(NucleusError::ValidationError("User is banned".into()));
    }
    
    Ok(())
}
```

### Mapping to HTTP Responses
You can impl `IntoResponse` for `NucleusError` to automatically map it to status codes.

```rust
impl IntoResponse for NucleusError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            NucleusError::ValidationError(m) => (StatusCode::BAD_REQUEST, m),
            NucleusError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration Error".into()),
            NucleusError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error".into()),
            NucleusError::NetworkError(_) => (StatusCode::BAD_GATEWAY, "Upstream Error".into()),
            // ... handle others
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".into()),
        };
        
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
```

### Pattern Matching
You can match on specific error variants for granular control:
```rust
match Stripe::checkout(...) {
    Ok(url) => Redirect::to(&url),
    Err(NucleusError::PaymentError(msg)) => {
        // Handle payment failure specifically
        Html(format!("Payment Failed: {}", msg))
    },
    Err(e) => {
        // Handle system errors (network, config, etc)
        error!("System error: {}", e);
        Html("Something went wrong".into())
    }
}
```

---

## Pagination

### Standard Pattern

```rust
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Serialize)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

pub async fn list_users(
    Query(params): Query<ListParams>,
    State(pool): State<SqlitePool>,
) -> Json<PaginatedResponse<User>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    
    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users LIMIT ? OFFSET ?",
        per_page,
        (page - 1) * per_page
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();
    
    Json(PaginatedResponse {
        data: users,
        pagination: Pagination {
            page,
            per_page,
            total,
            total_pages: (total as f64 / per_page as f64).ceil() as i64,
        },
    })
}
```

---

## Versioning

### URL Versioning

```rust
let app = Router::new()
    .nest("/api/v1", v1_routes())
    .nest("/api/v2", v2_routes());
```

### Header Versioning

```rust
pub async fn get_user(headers: HeaderMap) -> impl IntoResponse {
    let version = headers
        .get("API-Version")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("1");
    
    match version {
        "2" => Json(UserV2 { /* ... */ }),
        _ => Json(UserV1 { /* ... */ }),
    }
}
```

---

## Documentation (OpenAPI)

### Generate Schema

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct User {
    /// Unique user identifier
    pub id: i64,
    /// User's email address
    pub email: String,
    /// Display name
    pub name: Option<String>,
}
```

### Serve OpenAPI Spec

```rust
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(list_users, get_user, create_user),
    components(schemas(User, CreateUserRequest))
)]
struct ApiDoc;

let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
```

---

## Testing APIs

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    
    #[tokio::test]
    async fn test_create_user() {
        let app = create_test_app().await;
        let server = TestServer::new(app).unwrap();
        
        let response = server
            .post("/api/users")
            .json(&json!({
                "email": "test@example.com",
                "password": "password123"
            }))
            .await;
        
        assert_eq!(response.status_code(), StatusCode::CREATED);
        
        let user: User = response.json();
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_get_nonexistent_user() {
        let app = create_test_app().await;
        let server = TestServer::new(app).unwrap();
        
        let response = server.get("/api/users/99999").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
```

---

## Rate Limiting

```rust
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

let config = GovernorConfigBuilder::default()
    .per_second(10)
    .burst_size(20)
    .finish()
    .unwrap();

let app = Router::new()
    .nest("/api", api_routes())
    .layer(ServiceBuilder::new().layer(GovernorLayer { config }));
```

---

## CORS Configuration

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

let app = Router::new()
    .nest("/api", api_routes())
    .layer(cors);
```

---

## Financial Integrations

Nucleus provides built-in modules for Payments (Stripe) and Blockchain (EVM) interactions via `nucleus-std`.

### Stripe Payments

The `nucleus_std::payments::Stripe` module simplifies payment flows.

```rust
use nucleus_std::payments::{Stripe, LineItem};

// One-time Checkout
pub async fn create_checkout() -> Json<Value> {
    let url = Stripe::checkout(
        "https://myapp.com/success",
        "https://myapp.com/cancel",
        vec![
            LineItem { price: Some("price_123".into()), quantity: 1 }
        ],
        "payment", 
        Some("unique_idempotency_key"),
        Some("customer@example.com")
    ).await.unwrap();

    Json(json!({ "url": url }))
}

// Webhook Verification
pub async fn webhook(headers: HeaderMap, body: String) -> StatusCode {
    let sig = headers.get("Stripe-Signature").unwrap().to_str().unwrap();
    let secret = "whsec_...";
    
    match Stripe::verify_webhook(&body, sig, secret) {
        Ok(true) => StatusCode::OK,
        _ => StatusCode::UNAUTHORIZED,
    }
}
```

### Blockchain (EVM)

The `nucleus_std::chain::Chain` module handles EVM-compatible interactions.

```rust
use nucleus_std::chain::Chain;

// Verify "Sign In With Ethereum" (EIP-191)
pub async fn verify_login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let valid = Chain::verify_signature(
        &payload.message, 
        &payload.signature, 
        &payload.address
    ).unwrap_or(false);

    if valid {
        // Issue session cookie
    }
}

// Get Native Balance (ETH/MATIC)
pub async fn check_funds(address: String) -> Json<Value> {
    let balance = Chain::get_native_balance(&address).await.unwrap();
    Json(json!({ "balance": balance }))
}
```

Configuration for keys and RPC URLs is managed via `nucleus.config`.

---

## Server Actions (Code Isolation)

Nucleus supports **Server Actions**—functions that look like normal Rust calls but are transformed at compile-time to run securely on the server.

### Features
1.  **Security**: Secrets in the function body are **stripped** from the client WASM bundle.
2.  **Ease of Use**: Call them directly from client code.
3.  **Efficiency**: Transformed into efficient RPC calls automatically.

### Usage

Mark any `async` function with `#[server]`.

```rust
use nucleus_std::server;
use nucleus_std::errors::Result;

#[server]
pub async fn secret_action(user_id: String) -> Result<String> {
    // 🛡️ ENTIRE BODY REMOVED ON CLIENT
    
    // Server-only logic with secrets
    let db_pass = std::env!("DB_PASSWORD"); 
    let user = sqlx::query!("SELECT * FROM users WHERE id = ?", user_id)
        .fetch_one(&pool)
        .await?;
        
    Ok(format!("Hello {}", user.name))
}
```

### How it works
- **Server Build**: The function body is preserved.
- **Client (WASM) Build**: The body is replaced with:
  ```rust
  pub async fn secret_action(user_id: String) -> Result<String> {
      nucleus_std::rpc::call("secret_action", (user_id)).await
  }
  ```

