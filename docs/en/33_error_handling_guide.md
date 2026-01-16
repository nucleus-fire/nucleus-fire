# Error Handling Guide

Comprehensive error handling is essential for building robust applications. Nucleus provides integrated error handling with beautiful diagnostics.

## Overview

Nucleus uses **Miette** for user-friendly error messages that include:
- Contextual error messages
- Source code snippets
- Helpful suggestions
- "Did you mean?" corrections

## Quick Example

```rust
use miette::{miette, Result, IntoDiagnostic};

#[server]
async fn get_user(id: i64) -> Result<User> {
    let user = photon::query("SELECT * FROM users WHERE id = ?", [id])
        .fetch_one()
        .await
        .into_diagnostic()?;
    
    Ok(user)
}
```

## Error Types

### Using Result with Miette

```rust
use miette::{miette, Result, IntoDiagnostic, Diagnostic};
use thiserror::Error;

// Define custom errors
#[derive(Error, Diagnostic, Debug)]
pub enum AppError {
    #[error("User not found: {id}")]
    #[diagnostic(code(app::user_not_found))]
    UserNotFound { id: i64 },
    
    #[error("Invalid email format")]
    #[diagnostic(
        code(app::invalid_email),
        help("Email must contain @ symbol")
    )]
    InvalidEmail,
    
    #[error("Database error")]
    #[diagnostic(code(app::db_error))]
    DatabaseError(#[from] sqlx::Error),
}
```

### Returning Errors

```rust
#[server]
async fn update_user(id: i64, email: String) -> Result<User, AppError> {
    // Validation
    if !email.contains('@') {
        return Err(AppError::InvalidEmail);
    }
    
    let user = photon::query("UPDATE users SET email = ? WHERE id = ?", [&email, id])
        .execute()
        .await?;
    
    if user.rows_affected() == 0 {
        return Err(AppError::UserNotFound { id });
    }
    
    Ok(user)
}
```

## HTTP Error Responses

### Status Codes

```rust
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::UserNotFound { .. } => StatusCode::NOT_FOUND,
            AppError::InvalidEmail => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        let body = json!({
            "error": self.to_string(),
            "code": format!("{:?}", self),
        });
        
        (status, Json(body)).into_response()
    }
}
```

### Error JSON Response

```json
{
  "error": "User not found: 42",
  "code": "app::user_not_found"
}
```

## Form Validation Errors

Display errors inline with forms:

```html
<n:view>
<n:form action="/register">
    <div class="field">
        <label>Email</label>
        <input type="email" name="email" required />
        <n:if cond="errors.email">
            <span class="error">{errors.email}</span>
        </n:if>
    </div>
    
    <div class="field">
        <label>Password</label>
        <input type="password" name="password" minlength="8" required />
        <n:if cond="errors.password">
            <span class="error">{errors.password}</span>
        </n:if>
    </div>
    
    <button type="submit">Register</button>
</n:form>
</n:view>
```

### Server-side Validation

```rust
#[action]
async fn register(form: FormData) -> Result<Redirect, ValidationErrors> {
    let mut errors = ValidationErrors::new();
    
    if form.email.is_empty() {
        errors.add("email", "Email is required");
    } else if !is_valid_email(&form.email) {
        errors.add("email", "Invalid email format");
    }
    
    if form.password.len() < 8 {
        errors.add("password", "Password must be at least 8 characters");
    }
    
    if errors.is_empty() {
        // Create user...
        Ok(Redirect::to("/dashboard"))
    } else {
        Err(errors)
    }
}
```

## Compiler Errors

Nucleus provides rich error messages at compile time:

```
error[NCL001]: Unknown attribute: 'n:clck' on element
  ┌─ src/views/button.ncl:5:10
  │
5 │   <button n:clck="submit()">
  │           ^^^^^^ Did you mean: 'n:click'?
  │
  = help: Available event handlers: n:click, n:submit, n:change
```

## Try/Catch Pattern

For complex operations:

```rust
#[server]
async fn complex_operation() -> Result<Response> {
    let result = async {
        let user = get_user(1).await?;
        let orders = get_orders(user.id).await?;
        process_orders(&orders).await?;
        Ok(())
    }.await;
    
    match result {
        Ok(_) => Ok(Json(json!({"status": "success"}))),
        Err(e) => {
            eprintln!("Operation failed: {}", e);
            Ok(Json(json!({"status": "error", "message": e.to_string()})))
        }
    }
}
```

## Logging Errors

```rust
use tracing::{error, warn, info};

#[server]
async fn risky_operation() -> Result<()> {
    match do_something().await {
        Ok(result) => {
            info!("Operation succeeded: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            Err(e)
        }
    }
}
```

## Custom Error Pages

Create `src/views/404.ncl`:

```html
<n:view title="Page Not Found">
<n:layout name="layout">
    <div class="error-page">
        <h1>404</h1>
        <p>Page not found</p>
        <a href="/">← Back to home</a>
    </div>
</n:layout>
</n:view>
```

Create `src/views/500.ncl`:

```html
<n:view title="Server Error">
<n:layout name="layout">
    <div class="error-page">
        <h1>500</h1>
        <p>Something went wrong</p>
        <a href="/">← Back to home</a>
    </div>
</n:layout>
</n:view>
```

## Best Practices

1. **Be Specific** - Use custom error types for different scenarios
2. **Provide Context** - Include relevant IDs and values in errors
3. **Suggest Fixes** - Use `help` attribute in diagnostics
4. **Log Appropriately** - Log errors for debugging, don't expose internals to users
5. **Graceful Degradation** - Handle errors without crashing the app

## Related Guides

- [Forms & Validation](#07_forms_and_validation) - Form errors
- [API Development](#22_api_development) - API error responses
- [Troubleshooting](#08_troubleshooting) - Common issues
