# Authentication Guide

Complete guide to authentication, authorization, and security in Nucleus using the Fortress middleware.

---

## Overview

Nucleus provides the **Fortress** module for:

- Password hashing (Argon2id)
- JWT token generation and verification
- Role-based access control (RBAC)
- Rate limiting
- Security headers

---

## Setup

### Import

```rust
use nucleus_std::fortress::*;
```

### Configuration

The primary configuration is the `secret_key` which is used for HMAC operations and token signing.

```toml
# nucleus.config
[app]
secret_key = "${SECRET_KEY}"
```

Generate a secure secret key:

```bash
openssl rand -hex 32
```

---

## Password Hashing

### Hash a Password

```rust
use nucleus_std::fortress::Fortress;

let password = "user_password123";
let hash = Fortress::hash_password(password)?;

// Store `hash` in database (e.g., users.password_hash)
```

The hash is a string like:
```
$argon2id$v=19$m=19456,t=2,p=1$...
```

### Verify a Password

```rust
use nucleus_std::fortress::Fortress;

let valid = Fortress::verify_password(&stored_hash, "user_password123");

if valid {
    println!("Password correct!");
} else {
    println!("Invalid password");
}
```

### Complete Login Example

```rust
use nucleus_std::fortress::Fortress;
use nucleus_std::server;

#[server]
async fn register(email: String, password: String) -> Result<User> {
    // Check if user exists
    if User::query().filter("email", &email).first().await?.is_some() {
        return Err(NucleusError::Validation("Email already exists".into()));
    }
    
    // Hash password
    let hash = Fortress::hash_password(&password)?;
    
    // Create user
    let user = User::create(CreateUser {
        email,
        password_hash: hash,
    }).await?;
    
    Ok(user)
}

#[server]
async fn login(email: String, password: String) -> Result<LoginResponse> {
    // Find user
    let user = User::query()
        .filter("email", &email)
        .first()
        .await?
        .ok_or(NucleusError::Auth("Invalid credentials".into()))?;
    
    // Verify password (note: hash first, then password)
    if !Fortress::verify_password(&user.password_hash, &password) {
        return Err(NucleusError::Auth("Invalid credentials".into()));
    }
    
    // Generate token (requires secret from config)
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = Fortress::generate_token(&user.id.to_string(), &secret);
    
    Ok(LoginResponse { token, user })
}
```

---

## JWT / HMAC Tokens

### Generate Token

```rust
use nucleus_std::fortress::Fortress;

// Get secret from environment
let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

// Generate HMAC token (use with user_id)
// token consists of hex(user_id) . signature
let token = Fortress::generate_token(&user_id.to_string(), &secret);
```

### Verify Token

```rust
use nucleus_std::fortress::Fortress;

let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

// Verify returns true if token matches the expected user_id + secret
if Fortress::verify_token(&token, &user_id.to_string(), &secret) {
    println!("Valid token for user: {}", user_id);
} else {
    println!("Invalid token");
}
```

### Token Validation (Extract ID)

You can also validate a token and extract the user ID if the signature is valid.

```rust
match Fortress::validate_token(&token, &secret) {
    Ok(user_id) => println!("Token valid for user {}", user_id),
    Err(e) => println!("Token invalid: {}", e),
}
```

---

## Authorization

Nucleus supports Role-Based Access Control (RBAC) via the `Permission` and `Role` structs.

### Define Roles

```rust
use nucleus_std::fortress::{Role, Permission};
use std::collections::HashSet;

let mut permissions = HashSet::new();
permissions.insert(Permission::Read);
permissions.insert(Permission::Write);

let admin_role = Role {
    name: "admin".to_string(),
    permissions,
};
```

### Check Permissions

```rust
use nucleus_std::fortress::{Fortress, User, Permission};

// Example check
if Fortress::check_permission(&user, &Permission::Delete) {
    // delete resource
}
```

---

## OAuth / Social Login

OAuth is configured via environment variables and the `OAuthConfig` struct.

### Configuration (Environment Variables)

Set these in your `.env` or environment:

```bash
OAUTH_REDIRECT_URI=http://localhost:3000/auth/callback
GOOGLE_CLIENT_ID=...
GOOGLE_CLIENT_SECRET=...
GITHUB_CLIENT_ID=...
GITHUB_CLIENT_SECRET=...
```

### Code Example

```rust
use nucleus_std::oauth::{OAuthConfig, OAuthProvider, OAuth};

#[server]
async fn login_with_google() -> Redirect {
    let config = OAuthConfig::from_env();
    let oauth = OAuth::new(config);
    
    // Generate URL
    let (url, state) = oauth.authorize_url(OAuthProvider::Google).unwrap();
    
    // TODO: Store state in session/cookie to verify callback
    
    Redirect::to(&url)
}

#[server]
async fn google_callback(code: String, state: String) -> Result<LoginResponse> {
    let config = OAuthConfig::from_env();
    let oauth = OAuth::new(config);
    
    // Exchange code for user info
    let user = oauth.exchange_code(
        OAuthProvider::Google, 
        &code, 
        &state, 
        "expected_state_from_session"
    ).await.map_err(|e| NucleusError::Auth(e))?;
    
    // Find or create user via email/provider_id
    // ...
}
```

---

## Security Headers

To enable standard security headers (CSP, HSTS, XSS protection), use the middleware helper.

```rust
use nucleus_std::middleware::security_headers_middleware;

let app = Router::new()
    .route("/", get(home))
    .layer(middleware::from_fn(security_headers_middleware));
```

This adds:
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Content-Security-Policy` (default strict)

---

## Rate Limiting

Rate limiting is applied via middleware using `RateLimiter` and `RateLimitConfig`.

```rust
use nucleus_std::fortress::{RateLimiter, RateLimitConfig};
use nucleus_std::middleware::rate_limit_middleware;

// Create a limiter (e.g., 100 requests per minute)
let config = RateLimitConfig {
    max_requests: 100,
    window: std::time::Duration::from_secs(60),
    key_type: nucleus_std::fortress::RateLimitKey::Ip,
};
let limiter = RateLimiter::new(config);

// Apply to routes
let app = Router::new()
    .route("/api/sensitive", get(handler))
    .layer(middleware::from_fn(move |req, next| {
        rate_limit_middleware(limiter.clone())(req, next)
    }));
```

---

## Best Practices

### 1. Never Store Plain Passwords
Always use `Fortress::hash_password`.

### 2. Use HTTPS in Production
Set `[app] secret_key` securely and ensure your reverse proxy handles SSL.

### 3. Rotate Secrets
Use environment variables for all secrets (`SECRET_KEY`, `GOOGLE_CLIENT_SECRET`, etc.) and never commit them to git.

### 4. Validate All Input
Always validate input lengths, formats, and types before processing authentication logic.

---

## See Also

- [Getting Started](#01_getting_started)
- [Database Guide](#20_database_guide)
- [API Development](#22_api_development)
