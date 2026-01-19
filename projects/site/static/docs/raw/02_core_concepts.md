# Core Concepts

Understanding these core concepts will help you build better Nucleus applications.

---

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

Nucleus introduces a robust HMR system designed for productivity without data loss.

### How It Works
1. **Instant Updates**: When you save a `.ncl` file, the Reactor broadcasts an update signal to the browser
2. **State Preservation**: The client-side runtime automatically saves the current application state

### Practical Example: State-Preserving Updates

```html
<!-- src/views/counter.ncl -->
<n:view>
    <n:island client:load>
        <n:script>
             let count = Signal::new(0);
        </n:script>
        
        <div class="counter">
            <p>Count: {count}</p>
            <button onclick={count.update(|c| *c += 1)}>Increment</button>
        </div>
    </n:island>
</n:view>
```

**What happens when you save:**
1. Current `count` value (e.g., `5`) is saved to `sessionStorage`
2. Page reloads with your changes
3. `count` is restored to `5` automatically

---

## Islands Architecture

Nucleus uses **Islands Architecture** for optimal performance—only interactive parts of your page ship JavaScript.

### Hydration Strategies

| Directive | Behavior |
|-----------|----------|
| `client:load` | Hydrate immediately on page load |
| `client:visible` | Hydrate when element enters viewport |
| `client:idle` | Hydrate when browser is idle |
| `client:media="(query)"` | Hydrate when media query matches |

### Example: Progressive Hydration

```html
<n:view>
    <!-- Static content - no JS shipped -->
    <header>
        <h1>Welcome</h1>
        <p>This is server-rendered HTML.</p>
    </header>
    
    <!-- Interactive island - hydrates on viewport entry -->
    <n:island client:visible>
        <n:script>
            let likes = Signal::new(0);
        </n:script>
        <button onclick={likes.update(|n| *n += 1)}>
            ❤️ {likes}
        </button>
    </n:island>
    
    <!-- Heavy component - hydrates when idle -->
    <n:island client:idle>
        <n:component name="DataChart" data={chart_data} />
    </n:island>
</n:view>
```

---

## Client-Side Navigation

The `<n:link>` component enables SPA-like navigation without full page reloads.

### How It Works
1. Click on `<n:link>` intercepts the navigation
2. Fetches the new page content via AJAX
3. Updates the DOM and browser history
4. Preserves scroll position and state

### Example

```html
<nav>
    <n:link href="/">Home</n:link>
    <n:link href="/about">About</n:link>
    <n:link href="/blog" prefetch>Blog</n:link> <!-- Prefetches on hover -->
</nav>
```

### Programmatic Navigation

```rust
// In n:script or n:action
nucleus::router::navigate("/dashboard");
nucleus::router::replace("/login"); // No history entry
```

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

## Component System

Nucleus has a powerful component system with props, slots, and scoped CSS.

### Defining Components

```html
<!-- src/components/button.ncl -->
<n:component>
    <n:props>
        variant: String = "primary",
        size: String = "md",
        disabled: bool = false
    </n:props>
    
    <button 
        class="btn btn-{variant} btn-{size}" 
        disabled={disabled}
    >
        <n:slot />
    </button>
    
    <style scoped>
        .btn { padding: 8px 16px; border-radius: 4px; }
        .btn-primary { background: #3b82f6; color: white; }
        .btn-secondary { background: #64748b; color: white; }
        .btn-sm { padding: 4px 8px; font-size: 0.875rem; }
        .btn-lg { padding: 12px 24px; font-size: 1.125rem; }
    </style>
</n:component>
```

### Using Components

```html
<n:button variant="primary" size="lg">
    Save Changes
</n:button>

<n:button variant="secondary" disabled={!is_valid}>
    Submit
</n:button>
```

### Named Slots

```html
<!-- Component definition -->
<n:component>
    <div class="card">
        <header><n:slot name="header" /></header>
        <main><n:slot /></main>
        <footer><n:slot name="footer" /></footer>
    </div>
</n:component>

<!-- Usage -->
<n:card>
    <n:slot name="header">Card Title</n:slot>
    Main content goes here
    <n:slot name="footer">Footer text</n:slot>
</n:card>
```

---

## Server Actions

Handle form submissions and mutations with type-safe Rust code.

### Basic Example

```html
<n:view>
    <n:action>
        use nucleus_std::photon::db;
        
        let email = params.get("email").unwrap_or(&"".to_string()).clone();
        if !email.is_empty() && email.contains('@') {
            sqlx::query("INSERT INTO subscribers (email) VALUES (?)")
                .bind(&email)
                .execute(db().sqlite())
                .await?;
        }
    </n:action>
    
    <form method="POST">
        <input type="email" name="email" required>
        <button type="submit">Subscribe</button>
    </form>
</n:view>
```

### With Validation

```html
<n:action>
    use nucleus_std::forms::{Form, Validate};
    
    #[derive(Form, Validate)]
    struct ContactForm {
        #[validate(email)]
        email: String,
        #[validate(length(min = 10, max = 500))]
        message: String,
    }
    
    let form = ContactForm::from_request(&request)?;
    form.validate()?;
    
    // Process valid form...
</n:action>
```

---

## Middleware Pipeline

Requests flow through a middleware pipeline before reaching your handlers.

### Request Flow

```
Request → Logging → Auth → RateLimit → CORS → Handler → Response
```

### Custom Middleware

```rust
// src/middleware.rs
use nucleus_std::middleware::{Middleware, Next};
use axum::http::Request;

pub struct TimingMiddleware;

impl Middleware for TimingMiddleware {
    async fn handle<B>(&self, req: Request<B>, next: Next<B>) -> Response {
        let start = std::time::Instant::now();
        let response = next.run(req).await;
        let duration = start.elapsed();
        
        response.headers_mut().insert(
            "X-Response-Time",
            format!("{}ms", duration.as_millis()).parse().unwrap()
        );
        response
    }
}
```

---

## Error Handling

Nucleus uses `miette` for beautiful error messages.

### Error Boundaries in Views

```html
<n:view>
    <n:error-boundary fallback="<p>Something went wrong</p>">
        <n:component name="DataTable" data={data} />
    </n:error-boundary>
</n:view>
```

### Custom Errors

```rust
use nucleus_std::errors::NucleusError;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("User not found")]
#[diagnostic(code(app::user_not_found))]
pub struct UserNotFound {
    #[help]
    pub suggestion: String,
}
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

## Signals & Stores (Neutron)

Nucleus uses fine-grained reactivity with no Virtual DOM.

### Signals
For simple reactive values:

```rust
let count = Signal::new(0);

// Read
println!("Count: {}", count.get());

// Update
count.set(5);
count.update(|c| *c += 1);
```

### Stores
For complex state:

```rust
#[derive(Store)]
struct AppState {
    user: Option<User>,
    theme: String,
    notifications: Vec<Notification>,
}

let store = Store::new(AppState::default());

// Subscribe to changes
store.subscribe(|state| {
    println!("Theme changed to: {}", state.theme);
});
```

---

## Caching Layers

Nucleus provides multiple caching strategies.

### In-Memory Cache

```rust
use nucleus_std::Cache;

let cache = Cache::new();
cache.set("user:123", user, Duration::from_secs(300));

if let Some(user) = cache.get::<User>("user:123") {
    // Cache hit
}
```

### Redis Cache

```rust
use nucleus_std::redis_cache;

let cache = redis_cache("redis://localhost:6379").await?;
cache.set("session:abc", session_data, Duration::from_secs(3600)).await?;
```

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
