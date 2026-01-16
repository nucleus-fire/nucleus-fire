# Authentication Guide

Complete guide to authentication, authorization, and security in Nucleus using the Fortress middleware.

---

## Overview

Nucleus provides the **Fortress** module for:

- Password hashing (Argon2id)
- JWT token generation and verification
- Role-based access control (RBAC)
- Session management
- Security headers

---

## Setup

### Import

```rust
use nucleus_std::fortress::*;
```

### Configuration

```toml
# nucleus.config

# Required for JWT signing
secret_key = "${SECRET_KEY}"

# Optional: Session settings
[session]
duration = 86400        # 24 hours in seconds
refresh_threshold = 3600  # Refresh if < 1 hour left
cookie_name = "session"
secure = true           # HTTPS only
http_only = true
same_site = "strict"    # "strict", "lax", or "none"
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

## JWT Tokens

### Generate Token

```rust
use nucleus_std::fortress::Fortress;

// Get secret from environment
let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

// Generate HMAC token (use with user_id)
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

### Token Notes

This helper generates a simple HMAC-SHA256 signature of the user ID using your secret key. It is **not** a full JWT implementation and does not store expiration or custom claims within the token string itself.

If you need full JWT support with claims, consider using the `jsonwebtoken` crate directly or check for updates to Fortress.

### Token Refresh Flow

For production apps, use short-lived access tokens with longer-lived refresh tokens:

```rust
use nucleus_std::fortress::Fortress;

#[server]
async fn refresh_token(refresh_token: String) -> Result<TokenPair> {
    let secret = std::env::var("SECRET_KEY")?;
    
    // Verify the refresh token is valid
    let user_id = Fortress::extract_user_id(&refresh_token, &secret)
        .ok_or(NucleusError::Auth("Invalid refresh token".into()))?;
    
    // Check if refresh token is in database (not revoked)
    let stored = RefreshToken::find_by_token(&refresh_token).await?
        .ok_or(NucleusError::Auth("Token revoked".into()))?;
    
    // Revoke old refresh token (rotate)
    stored.revoke().await?;
    
    // Generate new token pair
    let access = Fortress::generate_token(&user_id, &secret);
    let refresh = Fortress::generate_token(&format!("ref:{}", user_id), &secret);
    
    // Store new refresh token
    RefreshToken::create(&user_id, &refresh).await?;
    
    Ok(TokenPair { access, refresh })
}
```

**Recommended Expiration Times:**
- Access Token: 15 minutes
- Refresh Token: 7 days

---

## Middleware Authentication

### Protecting Routes

### Protecting Routes

To protect routes, use the `require_auth` middleware. This ensures a valid Bearer token is present.

```rust
use nucleus_std::fortress::require_auth;
use axum::middleware;

// In your router setup
let protected_routes = Router::new()
    .route("/dashboard", get(dashboard))
    .route("/profile", get(profile))
    .route_layer(middleware::from_fn(require_auth));
```

### Extracting User from Request

Use the `AuthUser` extractor to get the authenticated user ID.

```rust
use nucleus_std::fortress::AuthUser;

async fn dashboard(auth: AuthUser) -> impl IntoResponse {
    // auth.user_id is available (validated from token)
    let user_id = auth.user_id;
    let user = User::find(user_id.parse().unwrap()).await?;
    
    // Render dashboard with user context
}
```

### Optional Authentication

Use `OptionalAuth` for routes that work for both guests and logged-in users.

```rust
use nucleus_std::fortress::OptionalAuth;

async fn public_page(auth: OptionalAuth) -> impl IntoResponse {
    if let Some(user_id) = auth.user_id {
        // Logged in - show personalized content
    } else {
        // Guest - show public content
    }
}
```

---

## Role-Based Access Control (RBAC)

### Define Roles

```rust
use nucleus_std::fortress::{Role, Permission};

let admin = Role::new("admin")
    .with_permission(Permission::Read)
    .with_permission(Permission::Write)
    .with_permission(Permission::Delete)
    .with_permission(Permission::Admin);

let moderator = Role::new("moderator")
    .with_permission(Permission::Read)
    .with_permission(Permission::Write)
    .with_permission(Permission::Custom("moderate".into()));

let user = Role::new("user")
    .with_permission(Permission::Read);
```

### Check Permissions

```rust
use nucleus_std::fortress::check_permission;

#[server]
async fn delete_post(auth: AuthUser, post_id: i64) -> Result<()> {
    let user = User::find(auth.user_id.parse()?).await?;
    
    // Check permission
    if !check_permission(&user, Permission::Delete) {
        return Err(NucleusError::Auth("Insufficient permissions".into()));
    }
    
    Post::delete(post_id).await?;
    Ok(())
}
```

### Role-Based Middleware

```rust
use nucleus_std::fortress::RequireRole;

let admin_routes = Router::new()
    .route("/admin/users", get(list_users))
    .route("/admin/settings", get(settings))
    .layer(RequireRole::new("admin"));
```

### Storing Roles in Database

```rust
// User model with role
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub role: String,  // "admin", "moderator", "user"
}

// Check role
impl User {
    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }
    
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}
```

---

## Sessions

### Cookie-Based Sessions

```rust
use nucleus_std::fortress::{Session, SessionStore};

// Create session
let session = Session::create(&user_id.to_string()).await?;

// Set cookie in response
response.headers_mut().insert(
    "Set-Cookie",
    format!("session={}; HttpOnly; Secure; SameSite=Strict", session.id)
        .parse()
        .unwrap()
);
```

### Session Middleware

```rust
use nucleus_std::fortress::SessionMiddleware;

let app = Router::new()
    .route("/", get(home))
    .layer(SessionMiddleware::new());
```

### Access Session in Handlers

```rust
use nucleus_std::fortress::CurrentSession;

async fn profile(session: CurrentSession) -> impl IntoResponse {
    if let Some(user_id) = session.get::<String>("user_id") {
        let user = User::find(user_id.parse().unwrap()).await?;
        // Show profile
    } else {
        // Redirect to login
    }
}
```

### Session Operations

```rust
// Store value
session.set("user_id", &user.id.to_string()).await?;
session.set("theme", "dark").await?;

// Get value
let user_id: Option<String> = session.get("user_id");

// Remove value
session.remove("temp_data").await?;

// Destroy session (logout)
session.destroy().await?;
```

---

## OAuth / Social Login

### Configuration

```toml
# nucleus.config

[oauth.google]
client_id = "${GOOGLE_CLIENT_ID}"
client_secret = "${GOOGLE_CLIENT_SECRET}"
redirect_uri = "https://myapp.com/auth/google/callback"

[oauth.github]
client_id = "${GITHUB_CLIENT_ID}"
client_secret = "${GITHUB_CLIENT_SECRET}"
redirect_uri = "https://myapp.com/auth/github/callback"
```

### OAuth Routes

```rust
use nucleus_std::fortress::oauth::{GoogleAuth, GitHubAuth};

// Redirect to provider
#[server]
async fn google_login() -> Redirect {
    let url = GoogleAuth::authorization_url(&["email", "profile"]);
    Redirect::to(&url)
}

// Handle callback
#[server]
async fn google_callback(code: String) -> Result<LoginResponse> {
    let google_user = GoogleAuth::exchange_code(&code).await?;
    
    // Find or create user
    let user = User::find_or_create_by_oauth(
        "google",
        &google_user.id,
        &google_user.email,
        &google_user.name,
    ).await?;
    
    let token = generate_token(&user.id.to_string(), Duration::from_secs(86400))?;
    
    Ok(LoginResponse { token, user })
}
```

---

## Security Headers

Fortress automatically adds these headers:

| Header | Value | Purpose |
|--------|-------|---------|
| `Content-Security-Policy` | Configurable | Prevent XSS |
| `X-Content-Type-Options` | `nosniff` | Prevent MIME sniffing |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `X-XSS-Protection` | `1; mode=block` | XSS filter |
| `Strict-Transport-Security` | `max-age=31536000` | Force HTTPS |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Control referrer |

### Custom CSP

```toml
[csp]
default_src = ["'self'"]
script_src = ["'self'", "https://trusted-cdn.com"]
style_src = ["'self'", "https://fonts.googleapis.com"]
img_src = ["'self'", "data:", "https:"]
font_src = ["'self'", "https://fonts.gstatic.com"]
connect_src = ["'self'", "https://api.example.com"]
```

---

## Rate Limiting

```toml
[rate_limit]
enabled = true
requests = 100      # Max requests
window = 60         # Per window (seconds)
by = "ip"           # "ip" or "user"
whitelist = ["127.0.0.1"]

# Different limits per endpoint
[rate_limit.api]
requests = 1000
window = 60

[rate_limit.auth]
requests = 5
window = 300        # 5 attempts per 5 minutes
```

---

## Best Practices

### 1. Never Store Plain Passwords

```rust
// ✅ Always hash
let hash = hash_password(&password)?;

// ❌ Never store raw password
user.password = password; // WRONG!
```

### 2. Use HTTPS in Production

```toml
[session]
secure = true  # Cookie only sent over HTTPS
```

### 3. Short-Lived Access Tokens

```rust
// Access token: 15 minutes
let access = generate_token(&id, Duration::from_secs(900))?;

// Refresh token: 7 days
let refresh = generate_token(&id, Duration::from_secs(604800))?;
```

### 4. Validate All Input

```rust
#[server]
async fn register(email: String, password: String) -> Result<User> {
    // Validate email format
    if !email.contains('@') {
        return Err(NucleusError::Validation("Invalid email".into()));
    }
    
    // Validate password strength
    if password.len() < 8 {
        return Err(NucleusError::Validation("Password too short".into()));
    }
    
    // ...
}
```

---

## See Also

- [Getting Started](#01_getting_started)
- [Database Guide](#20_database_guide)
- [API Development](#22_api_development)
