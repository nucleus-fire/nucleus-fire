# Getting Started with Nucleus

This guide will take you from zero to a fully functional Nucleus application in under 10 minutes.

## Prerequisites

Before starting, ensure you have:

- **Rust** 1.75 or later ([install](https://rustup.rs))
- **Cargo** (included with Rust)
- A terminal/command prompt

```bash
# Verify installation
rustc --version  # Should be 1.75+
cargo --version
```

---

## Installation

Install the Nucleus CLI globally:

```bash
cargo install nucleus-cli
```

Verify the installation:

```bash
nucleus -v
# nucleus 0.1.0
```

---

## Create Your First Project

### 1. Scaffold a New Project

```bash
nucleus new my-app
cd my-app
```

This creates the following structure:

```
my-app/
├── Cargo.toml           # Rust dependencies
├── nucleus.config       # Framework configuration
├── content.deck         # i18n Content Deck
├── src/
│   ├── main.rs          # Application entry point
│   ├── assets/          # Source assets (processed by pipeline)
│   ├── logic/           # Business logic & Models
│   │   └── mod.rs
│   ├── models/          # Database structs
│   ├── vendor/          # Vendor modules
│   └── views/           # NCL templates
│       ├── layout.ncl   # Shared layout
│       └── index.ncl    # Home page
├── static/              # Optimized static assets (output)
└── migrations/          # Database migrations
```

### 2. Start the Development Server

```bash
nucleus dev
```

Output:
```
⚛️  Starting Nucleus Reactor...
...
✅ Build Complete.
🚀 Server started. Watching for changes...
```

Open [http://localhost:3000](http://localhost:3000) to see your app.

### 3. Edit Your First View

Open `src/views/index.ncl` and modify it:

```html
<n:view title="My First Nucleus App">
    <n:layout name="layout">
        <h1>Hello, Nucleus!</h1>
        <p>Edit this file and save to see changes.</p>
    </n:layout>
</n:view>
```

The page updates automatically — the dev server watches for file changes, rebuilds, and restarts.

---

## Project Configuration

The `nucleus.config` file controls your application:

```toml
version = "1.0.0"

[server]
port = 3000
host = "0.0.0.0"
# environment = "development"

[database]
url = "sqlite:nucleus.db"
# url = "${DATABASE_URL}"

[app]
name = "my-app"
secret_key = "${SECRET_KEY}"

[performance]
compression = true
inline_critical_css = true
```

### Environment Variables

Nucleus supports `${VAR}` syntax for environment variables:

```toml
[database]
url = "${DATABASE_URL}"

[app]
secret_key = "${SECRET_KEY:-default_dev_key}"
```

The `:-` syntax provides a default if the variable is unset.

---

## Views & Templates

### NCL Syntax (Nucleus Component Language)

NCL files compile to optimized HTML with zero runtime overhead.

#### Basic View

```html
<n:view title="Page Title">
    <n:layout name="layout">
        <!-- Your content here -->
        <h1>Welcome</h1>
    </n:layout>
</n:view>
```

#### View Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `title` | String | Page `<title>` tag |
| `description` | String | Meta description for SEO |
| `layout` | String | Layout file to use (without `.ncl`) |

### Layouts

Layouts define shared structure. Use `<n:slot>` to mark content insertion points:

```html
<!-- src/views/layout.ncl -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{title}</title>
    <link rel="stylesheet" href="/assets/style.css">
</head>
<body>
    <nav>...</nav>
    <main>
        <n:slot name="content" />
    </main>
    <footer>...</footer>
</body>
</html>
```

### Components

Create reusable components in the `views/components/` directory:

```html
<!-- src/views/components/button.ncl -->
<n:component name="Button">
    <button class="btn {class}" type="{type}">
        <n:slot />
    </button>
</n:component>
```

Use in views:

```html
<n:Button class="primary" type="submit">
    Click Me
</n:Button>
```

---

## Routing

Routes are automatically generated from your view files:

| File | Route |
|------|-------|
| `views/index.ncl` | `/` |
| `views/about.ncl` | `/about` |
| `views/blog/index.ncl` | `/blog` |
| `views/blog/[slug].ncl` | `/blog/:slug` (dynamic) |
| `views/api/users.ncl` | `/api/users` |

### Dynamic Routes

Use `[param]` syntax for dynamic segments:

```
views/
├── users/
├── [id].ncl        → /users/:id
└── [id]/edit.ncl   → /users/:id/edit
```

Access parameters in your logic:

```rust
#[server]
async fn get_user(id: i64) -> Result<User> {
    User::find(id).await
}
```

---

## Server Functions

Use the `#[server]` macro to create type-safe server-only functions:

```rust
use nucleus_std::server;
use nucleus_std::errors::Result;

#[server]
async fn create_post(title: String, content: String) -> Result<Post> {
    // This code ONLY runs on the server
    // It's automatically replaced with an RPC call on the client
    
    let post = Post::create(CreatePost { title, content }).await?;
    Ok(post)
}
```

### How It Works

1. On the **server**, the function executes normally
2. On the **client** (WASM), it's replaced with an HTTP call to the server
3. Type safety is maintained end-to-end

---

## Database

### Configuration

```toml
# nucleus.config
[database]
url = "postgres://user:pass@localhost/mydb"
```

Supported databases:
- PostgreSQL (`postgres://`)
- MySQL (`mysql://`)
- SQLite (`sqlite://./data.db`)

### The Photon Query Builder

```rust
use nucleus_std::photon::*;

// Find all
let users = User::query().fetch().await?;

// Find by ID
let user = User::find(1).await?;

// Filtering
let active = User::query()
    .filter("status", "active")
    .filter("role", "admin")
    .fetch()
    .await?;

// Ordering & Pagination
let recent = User::query()
    .order_by("created_at", Desc)
    .limit(10)
    .offset(20)
    .fetch()
    .await?;

// Create
let user = User::create(CreateUser {
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
}).await?;

// Update
User::update(1, UpdateUser {
    name: Some("Alice Smith".to_string()),
    ..Default::default()
}).await?;

// Delete
User::delete(1).await?;
```

### Migrations

```bash
# Create a migration
nucleus db new create_users

# Run pending migrations
nucleus db up

# Rollback last migration
nucleus db down
```

Migration file example:

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- DOWN
DROP TABLE users;
```

---

## Authentication

### Password Hashing

```rust
use nucleus_std::fortress::*;

// Hash a password (Argon2id)
let hash = hash_password("user_password")?;

// Verify a password
let valid = verify_password("user_password", &hash)?;
```

### JWT Tokens

```rust
use nucleus_std::fortress::*;
use std::time::Duration;

// Generate token
let token = generate_token(&user.id.to_string(), Duration::from_secs(86400))?;

// Verify token
let claims = verify_token(&token)?;
let user_id = claims.sub; // Subject (user ID)
```

### Role-Based Access Control

```rust
use nucleus_std::fortress::{Role, Permission, check_permission};

let admin = Role::new("admin")
    .with_permission(Permission::Read)
    .with_permission(Permission::Write)
    .with_permission(Permission::Delete);

let can_delete = check_permission(&user, Permission::Delete);
```

---

## Forms & Validation

### Basic Form

```html
<n:form action="/register" method="POST">
    <input name="email" type="email" required />
    <input name="password" type="password" minlength="8" required />
    <button type="submit">Register</button>
</n:form>
```

### Model Binding

Forms automatically bind to Rust structs:

```rust
#[derive(Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[server]
async fn register(form: RegisterForm) -> Result<User> {
    let hash = hash_password(&form.password)?;
    User::create(CreateUser {
        email: form.email,
        password_hash: hash,
    }).await
}
```

---

## CLI Reference

### Commands

| Command | Description |
|---------|-------------|
| `nucleus new <name>` | Create a new project |
| `nucleus dev` | Development server with hot reload |
| `nucleus build` | Build for production |
| `nucleus test` | Run tests |
| `nucleus db new <name>` | Create migration |
| `nucleus db up` | Run pending migrations |
| `nucleus db down` | Rollback migration |
| `nucleus generate scaffold <name> [fields...]` | Generate CRUD scaffold |
| `nucleus generate model <name> [fields...]` | Generate model only |
| `nucleus generate payments` | Generate payment components |

### Generate Fields

Field format: `name:type`

| Type | Rust Type | SQL Type |
|------|-----------|----------|
| `string` | `String` | `TEXT` |
| `int` | `i32` | `INTEGER` |
| `float` | `f64` | `REAL` |
| `bool` | `bool` | `BOOLEAN` |
| `bigint` | `i64` | `BIGINT` |

Example:

```bash
nucleus generate scaffold Post title:string body:string views:int published:bool
```

---

## Deployment

### Build for Production

```bash
cargo build --release
```

The binary at `target/release/my-app` is self-contained.

### Run in Production

```bash
# Set environment
export DATABASE_URL="postgres://..."
export SECRET_KEY="your-secret-key"
export RUST_LOG="info"

# Run
./target/release/my-app
```

### Docker

```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/my-app /app/
COPY --from=builder /app/static /static
EXPOSE 3000
CMD ["/app/my-app"]
```

### Recommended Specs

| Traffic | CPU | RAM | RPS |
|---------|-----|-----|-----|
| Low | 1 vCPU | 512MB | 5,000 |
| Medium | 2 vCPU | 1GB | 15,000 |
| High | 4 vCPU | 2GB | 40,000+ |

---

## Next Steps

- [Core Concepts](#02_core_concepts) - Deep dive into the architecture
- [Database Guide](#20_database_guide) - Advanced queries and relations
- [Authentication Guide](#21_authentication_guide) - Sessions, OAuth, and more
- [API Development](#22_api_development) - Building REST and GraphQL APIs
