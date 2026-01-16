# Core Concepts

## The Reactor (`atom`)

Unlike Node.js (Event Loop) or Java (Thread per Request), Nucleus uses the **Atom Reactor**.

### Key Features
- **Async-First**: Built on `tokio`, efficient for I/O bound tasks
- **Arena Memory**: Each request gets a dedicated memory arena. When the request ends, the entire arena is freed. This eliminates Garbage Collection pauses for 99% of short-lived objects.

### Practical Example: Async Request Handling

```rust
// In your view handler, async operations don't block other requests
<n:action>
    // These run concurrently, not blocking the reactor
    let user = User::find(user_id).await?;
    let posts = Post::where("user_id", user_id).all().await?;
    
    // Memory is automatically freed when this handler completes
</n:action>
```

---

## Hot Module Replacement (HMR) & State

Nucleus V3 introduces a robust HMR system designed for productivity without data loss.

### How It Works
1. **Instant Updates**: When you save a `.ncl` file, the Reactor broadcasts an update signal to the browser
2. **State Preservation**: The client-side runtime automatically saves the current application state

### Practical Example: State-Preserving Updates

```html
<!-- src/views/counter.ncl -->
<n:view>
    <n:state name="count" default="0" />
    
    <div class="counter">
        <p>Count: {count}</p>
        <button n:click="count += 1">Increment</button>
    </div>
</n:view>
```

**What happens when you save:**
1. Current `count` value (e.g., `5`) is saved to `sessionStorage`
2. Page reloads with your changes
3. `count` is restored to `5` automatically

---

## Views, Layouts, and Components

### Views
Primary building blocks representing pages.

```html
<!-- src/views/home.ncl -->
<n:view title="Home Page">
    <h1>Welcome to Nucleus</h1>
    <p>Build faster, ship safer.</p>
</n:view>
```

### Layouts
Wrap views with consistent structure (nav, footer, etc.).

```html
<!-- src/views/layout.ncl -->
<!DOCTYPE html>
<html>
<head>
    <title>{title}</title>
    <link rel="stylesheet" href="/assets/styles.css">
</head>
<body>
    <nav><!-- Navigation --></nav>
    <n:slot name="content" />
    <footer>© 2025</footer>
</body>
</html>
```

### Using Layouts

```html
<!-- src/views/about.ncl -->
<n:view title="About Us">
    <n:layout name="layout">
        <h1>About Us</h1>
        <p>Our story...</p>
    </n:layout>
</n:view>
```

---

## Server Actions

Handle form submissions and mutations with type-safe Rust code.

### Practical Example: Newsletter Signup

```html
<n:view>
    <n:model name="Subscriber">
        email: String
    </n:model>
    
    <n:action>
        use nucleus_std::photon::{db, DatabasePool};
        use nucleus_std::sqlx;
        
        let email = params.get("email").unwrap_or(&"".to_string()).clone();
        if !email.is_empty() && email.contains('@') {
            let pool = db();
            if let DatabasePool::Sqlite(p) = pool {
                sqlx::query("INSERT INTO subscribers (email) VALUES (?)")
                    .bind(&email)
                    .execute(p)
                    .await?;
            }
        }
    </n:action>
    
    <form method="POST">
        <input type="email" name="email" placeholder="you@example.com" required>
        <button type="submit">Subscribe</button>
    </form>
</n:view>
```

---

## Database Integration (Photon)

Nucleus uses Photon for type-safe database operations.

### Quick Start

```rust
// Initialize in your app startup
nucleus_std::photon::init_db("sqlite:data.db").await?;

// Query data
let users = User::query()
    .filter("active", true)
    .order_by("created_at", "DESC")
    .limit(10)
    .all()
    .await?;
```

### Migrations
Place SQL files in `migrations/` folder:

```sql
-- migrations/20251227000000_create_users.sql
-- UP
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- DOWN
DROP TABLE users;
```

Migrations run automatically on server start.

---

## Configuration

All settings in `nucleus.config`:

```toml
[server]
port = 3000
host = "0.0.0.0"

[database]
url = "sqlite:data.db"

[app]
name = "My App"
env = "production"
secret_key = "your-secret-key"
```

Access in code:
```rust
let config = nucleus_std::config::Config::load();
println!("Running on port {}", config.server.port);
```
