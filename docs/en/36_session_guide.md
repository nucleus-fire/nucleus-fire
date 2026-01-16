# Session Management Guide

Nucleus provides secure, cookie-based session management with flash messages and CSRF protection.

## Quick Start

```rust
use nucleus_std::session::{SessionManager, MemorySessionStore, SessionConfig};

// Create session manager
let manager = SessionManager::new(
    MemorySessionStore::new(),
    SessionConfig::default()
);

// Start new session
let mut session = manager.start();

// Set data
session.set("user_id", user.id);

// Get data
let user_id: Option<String> = session.get("user_id");

// Save session
manager.save(&session).await;
```

## In NCL Actions

```xml
<n:action>
    // Session is injected by middleware
    
    // Set value
    session.set("cart_items", 5);
    
    // Get value
    let count: i32 = session.get("cart_items").unwrap_or(0);
    
    // Flash message (one-time)
    session.flash("success", "Item added to cart!");
</n:action>
```

## Flash Messages

Flash messages are displayed once and then removed:

```rust
// Set flash
session.flash("success", "Profile updated!");
session.flash("error", "Invalid password");

// Get and consume (returns None on second call)
if let Some(msg) = session.get_flash("success") {
    // Display to user
}

// Check without consuming
if session.has_flash("error") {
    // ...
}
```

## Authentication

```rust
// Log in user
session.login("user_123", true); // true = regenerate session ID

// Check authentication
if session.is_authenticated() {
    let user_id = session.user_id();
    // ...
}

// Log out
session.logout();
```

## CSRF Protection

```rust
// Get token (include in forms)
let token = session.csrf_token();

// In your template
// <input type="hidden" name="_csrf" value="{{ csrf_token }}">

// Verify on submission
if !session.verify_csrf(&submitted_token) {
    return Err("Invalid CSRF token");
}

// Regenerate token
session.regenerate_csrf();
```

## Session Lifecycle

```rust
// Regenerate session ID (prevents session fixation)
session.regenerate();

// Destroy session completely
session.destroy();

// Check status
if session.is_destroyed() {
    // ...
}
if session.is_modified() {
    // Needs to be saved
}
```

## Configuration

```rust
// Default config
let config = SessionConfig::default();

// Development (less strict)
let config = SessionConfig::development();

// Production (strict security)
let config = SessionConfig::production();

// Custom
let config = SessionConfig::default()
    .ttl_hours(48)
    .cookie_name("my_app_session");
```

### Config Options

| Option | Default | Description |
|--------|---------|-------------|
| `cookie_name` | `nucleus_session` | Cookie name |
| `cookie_secure` | `true` | HTTPS only |
| `cookie_http_only` | `true` | No JS access |
| `cookie_same_site` | `Lax` | SameSite policy |
| `ttl` | 24 hours | Session lifetime |
| `regenerate_on_login` | `true` | New ID on login |

## Storing Complex Data

```rust
#[derive(Serialize, Deserialize)]
struct Cart {
    items: Vec<Item>,
    total: f64,
}

// Store struct
let cart = Cart { items: vec![], total: 0.0 };
session.set("cart", &cart);

// Retrieve struct
let cart: Option<Cart> = session.get("cart");
```

## Session Cleanup

```rust
// Periodically cleanup expired sessions
let cleaned = manager.cleanup().await;
println!("Cleaned {} expired sessions", cleaned);

// Run in scheduler
scheduler.hourly("session_cleanup", || async {
    manager.cleanup().await;
});
```

## Security Best Practices

1. **Always use HTTPS** in production (`cookie_secure: true`)
2. **Regenerate session ID** on login to prevent fixation
3. **Short TTL** for sensitive applications
4. **CSRF tokens** for all state-changing forms
5. **HttpOnly cookies** to prevent XSS token theft
