# Multi-tenancy Guide

Build SaaS applications with row-level security using Nucleus tenant support.

## Quick Start

```rust
use nucleus_std::tenant::{Tenant, TenantExtractor};

// In your middleware
let extractor = TenantExtractor::header("X-Tenant-ID");
let tenant_id = extractor.extract(&headers, uri, host)?;
Tenant::set(&tenant_id.unwrap());

// In your handlers - all queries automatically scoped
let users = User::find_all().await?; // Filtered by tenant_id

// Clean up at request end
Tenant::clear();
```

## Tenant Strategies

### Header-Based (Recommended for APIs)

```rust
let extractor = TenantExtractor::header("X-Tenant-ID");
// Request: GET /api/users -H "X-Tenant-ID: acme"
```

### Subdomain-Based (Web Apps)

```rust
let extractor = TenantExtractor::subdomain("myapp.com");
// Request: https://acme.myapp.com/dashboard → tenant: "acme"
```

### Path Prefix

```rust
let extractor = TenantExtractor::path_prefix();
// Request: /acme/api/users → tenant: "acme"
```

### Query Parameter

```rust
let extractor = TenantExtractor::new(TenantStrategy::QueryParam("org".into()));
// Request: /api/users?org=acme → tenant: "acme"
```

### Chain (Try Multiple)

```rust
let extractor = TenantExtractor::new(TenantStrategy::Chain(vec![
    TenantStrategy::Header("X-Tenant-ID".into()),
    TenantStrategy::QueryParam("tenant".into()),
    TenantStrategy::Fixed("default".into()),
]));
// Tries each strategy in order, first match wins
```

## Tenant Context

### Set/Get/Clear

```rust
Tenant::set("tenant_123");
let id = Tenant::get(); // Some("tenant_123")
Tenant::clear();
```

### Scoped Execution

```rust
// Run code in a specific tenant context
Tenant::with("temp_tenant", || {
    // All queries here scoped to temp_tenant
    do_something();
});
// Previous tenant restored after block
```

### Require Tenant

```rust
let tenant = Tenant::require()?; // Error if not set
```

## Query Helpers

Automatically scope SQL queries to current tenant:

```rust
use nucleus_std::tenant::TenantQuery;

Tenant::set("acme");

let sql = TenantQuery::select("users", "*");
// "SELECT * FROM users WHERE tenant_id = ?"

let sql = TenantQuery::delete("users");
// "DELETE FROM users WHERE tenant_id = ?"
```

## Middleware Integration

### Using TenantGuard

```rust
use nucleus_std::tenant::{TenantGuard, TenantExtractor};

let guard = TenantGuard::new(TenantExtractor::header("X-Tenant-ID"));

async fn my_handler(headers: HeaderMap) -> Result<impl IntoResponse, TenantError> {
    let tenant = guard.check(&headers, &uri, host.as_deref())?;
    // tenant is now set
    
    // ... handle request
    
    Tenant::clear();
    Ok(response)
}
```

### Axum Middleware Example

```rust
async fn tenant_middleware<B>(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let extractor = TenantExtractor::header("X-Tenant-ID");
    
    match extractor.extract(&headers, request.uri().path(), None) {
        Ok(Some(tenant)) => {
            Tenant::set(&tenant);
            let response = next.run(request).await;
            Tenant::clear();
            response
        }
        Ok(None) | Err(_) => {
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}
```

## Database Schema

Add `tenant_id` column to all tenant-scoped tables:

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    email TEXT NOT NULL,
    name TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure queries are always efficient
    INDEX idx_users_tenant (tenant_id)
);

-- Unique constraints should include tenant_id
CREATE UNIQUE INDEX idx_users_email ON users(tenant_id, email);
```

## Tenant Info

For admin/management features:

```rust
use nucleus_std::tenant::TenantInfo;

let tenant = TenantInfo::new("t_123", "Acme Corporation");
// tenant.slug = "acme-corporation"
// tenant.active = true
```

## Error Handling

```rust
use nucleus_std::tenant::TenantError;

match result {
    Err(TenantError::NotSet) => "Tenant context not initialized",
    Err(TenantError::NotFound) => "No tenant in request",
    Err(TenantError::Invalid(id)) => "Invalid tenant ID",
    Err(TenantError::Inactive) => "Tenant suspended",
    Err(TenantError::AccessDenied) => "Access denied",
    _ => "Unknown error",
}
```

## Optional Tenants

For routes that may or may not require tenant:

```rust
let extractor = TenantExtractor::header("X-Tenant-ID").optional();

match extractor.extract(&headers, uri, host)? {
    Some(tenant) => Tenant::set(&tenant),
    None => { /* Public endpoint */ }
}
```

## Testing

```rust
#[test]
fn test_with_tenant() {
    Tenant::with("test_tenant", || {
        assert_eq!(Tenant::get(), Some("test_tenant".to_string()));
        // Run tests that require tenant context
    });
}
```

## Best Practices

1. **Always clear tenant** at request end to prevent leakage
2. **Use indexes** on `tenant_id` columns for performance
3. **Include tenant_id** in unique constraints
4. **Validate tenant access** before setting context
5. **Use `Tenant::with()`** for temporary context switches
