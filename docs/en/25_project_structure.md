# Project Structure

> Understanding the Nucleus project layout.

## Overview

A Nucleus project follows a clear convention-over-configuration structure:

```
my-app/
├── Cargo.toml              # Rust manifest
├── nucleus.config          # Framework configuration (TOML)
├── .env                    # Environment variables (secrets)
├── .env.example            # Example env file (committed)
├── content.deck            # i18n strings
│
├── migrations/             # Database schemas
│   └── YYYYMMDD_name.sql
│
├── src/
│   ├── views/              # Pages (NCL templates)
│   │   ├── index.ncl       # → /
│   │   ├── about.ncl       # → /about  
│   │   ├── api/            # API routes (JSON responses)
│   │   │   └── users.ncl   # → /api/users
│   │   └── blog/
│   │       ├── index.ncl   # → /blog
│   │       └── [id].ncl    # → /blog/:id
│   │
│   ├── services/           # Business logic (Rust)
│   │   ├── mod.rs
│   │   ├── db.rs
│   │   └── auth.rs
│   │
│   ├── models/             # Data structures
│   │   ├── mod.rs
│   │   └── user.rs
│   │
│   ├── components/         # Reusable UI components
│   │   └── button.ncl
│   │
│   ├── layouts/            # Page layouts
│   │   └── main.ncl
│   │
│   ├── jobs/               # Background job handlers (Pulse)
│   │   ├── mod.rs
│   │   └── email_job.rs
│   │
│   ├── guards/             # Auth guards & middleware
│   │   ├── mod.rs
│   │   └── admin_guard.rs
│   │
│   ├── events/             # Event handlers (WebSocket, etc.)
│   │   └── mod.rs
│   │
│   ├── middleware.rs       # Request middleware
│   │
│   ├── assets/             # Source images (optimized on build)
│   │   └── hero.jpg
│   │
│   └── vendor/             # Third-party modules
│       └── some-package/
│
├── static/                 # Public assets (served as-is)
│   ├── css/
│   ├── js/
│   └── images/
│
├── tests/                  # Integration tests
│   ├── integration_test.rs
│   └── api_test.rs
│
└── target/                 # Build output (gitignored)
    └── release/
        └── server          # Production binary
```

---

## Directory Details

### `src/views/` - Pages

NCL template files that map directly to routes.

| File | Route | Description |
|------|-------|-------------|
| `index.ncl` | `/` | Homepage |
| `about.ncl` | `/about` | Static page |
| `blog/index.ncl` | `/blog` | Section index |
| `blog/[id].ncl` | `/blog/:id` | Dynamic route |
| `[...slug].ncl` | `/*slug` | Catch-all |

**Example `index.ncl`:**
```html
<n:view title="Home">
    <h1>Welcome</h1>
</n:view>
```

---

### `src/services/` - Business Logic

Rust modules containing your application logic.

**Structure:**
```rust
// src/services/mod.rs
pub mod db;
pub mod auth;
pub mod email;
```

**Example `db.rs`:**
```rust
use sqlx::SqlitePool;

pub async fn get_pool() -> SqlitePool {
    SqlitePool::connect("sqlite:nucleus.db").await.unwrap()
}

pub async fn get_user(id: i64) -> Option<User> {
    let pool = get_pool().await;
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten()
}
```

---

### `src/models/` - Data Structures

Shared data types used across your application.

**Example `user.rs`:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
}
```

---

### `src/components/` - Reusable UI

NCL components that can be used across views.

**Example `button.ncl`:**
```html
<n:component>
    <button class="btn btn-{variant}" type="{type}">
        <n:slot />
    </button>
    
    <style>
        .btn { padding: 8px 16px; border-radius: 4px; }
        .btn-primary { background: #3b82f6; color: white; }
        .btn-secondary { background: #64748b; color: white; }
    </style>
</n:component>
```

**Usage:**
```html
<n:button variant="primary" type="submit">
    Save Changes
</n:button>
```

---

### `src/layouts/` - Page Layouts

Shared page structure (header, footer, navigation).

**Example `main.ncl`:**
```html
<n:layout>
    <!DOCTYPE html>
    <html>
    <head>
        <title><n:slot name="title" /> | My App</title>
        <link rel="stylesheet" href="/static/css/main.css">
    </head>
    <body>
        <header>
            <nav>
                <n:link href="/">Home</n:link>
                <n:link href="/about">About</n:link>
            </nav>
        </header>
        
        <main>
            <n:slot />
        </main>
        
        <footer>© 2024 My App</footer>
    </body>
    </html>
</n:layout>
```

**Using layout:**
```html
<n:view layout="main" title="About">
    <h1>About Us</h1>
    <p>Content here goes in the default slot.</p>
</n:view>
```

---

### `migrations/` - Database

SQL migration files for schema changes.

**Naming convention:** `YYYYMMDD_description.sql`

**Example:**
```sql
-- migrations/20250101_create_users.sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Commands:**
```bash
nucleus db new create_users  # Create migration file
nucleus db up                # Apply migrations
```

---

### `static/` - Public Assets

Files served directly without processing.

```
static/
├── css/
│   └── main.css
├── js/
│   └── app.js
├── images/
│   └── logo.svg
└── favicon.ico
```

Access at: `/static/css/main.css`

---

### `src/assets/` - Optimized Assets

Images that get processed during `nucleus build`:
- Resized (max 1920px width)
- Converted to WebP
- Output to `static/assets/`

Place source images here for automatic optimization.

---

### `src/vendor/` - External Modules

Community packages installed via `nucleus install`:

```bash
nucleus install github.com/user/nucleus-auth
```

Creates: `src/vendor/nucleus-auth/`

---

### Configuration Files

#### `nucleus.config`
```toml
[app]
name = "my-app"
port = 3000

[database]
url = "sqlite:nucleus.db"

[build]
optimize_images = true
generate_sitemap = true
```

#### `content.deck` (i18n)
```
welcome:en = Welcome to our site!
welcome:es = ¡Bienvenido a nuestro sitio!
```

---

## Recommended Organization

### Small Projects
```
src/
├── views/          # All pages
├── services/
│   └── mod.rs      # All logic in one file
└── models/
    └── mod.rs      # All models in one file
```

### Medium Projects
```
src/
├── views/
├── services/
│   ├── mod.rs
│   ├── db.rs
│   ├── auth.rs
│   └── email.rs
└── models/
    ├── mod.rs
    ├── user.rs
    └── post.rs
```

### Large Projects
```
src/
├── views/
│   ├── admin/
│   ├── api/
│   └── public/
├── services/
│   ├── admin/
│   ├── auth/
│   └── core/
├── models/
├── services/       # External integrations
├── jobs/           # Background tasks
└── middleware/
```

---

## What Goes Where?

| Type of Code | Location |
|--------------|----------|
| Page rendering | `src/views/` |
| API endpoints | `src/views/api/` |
| Database queries | `src/services/` |
| Data structures | `src/models/` |
| UI components | `src/components/` |
| Page wrappers | `src/layouts/` |
| Background jobs | `src/jobs/` |
| Auth guards | `src/guards/` |
| Event handlers | `src/events/` |
| Request processing | `src/middleware.rs` |
| Third-party code | `src/vendor/` |
| Public files | `static/` |
| Source images | `src/assets/` |
| Schema changes | `migrations/` |
| Unit tests | `tests/*.rs` |
| E2E tests | `tests/e2e/` |
