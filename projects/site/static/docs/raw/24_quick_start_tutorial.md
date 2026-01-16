# Quick Start: Build Your First App

> Go from zero to running app in under 5 minutes.

## Prerequisites

- Rust 1.70+ installed ([rustup.rs](https://rustup.rs))
- A terminal

---

## Step 1: Install Nucleus CLI

```bash
# Clone the repository
git clone https://github.com/example/nucleus-lang.git
cd nucleus-lang

# Build and install the CLI
cargo install --path crates/nucleus-cli
```

Verify installation:
```bash
nucleus --version
# nucleus 0.1.0
```

---

## Step 2: Create Your Project

```bash
nucleus new my-blog
cd my-blog
```

**What was created:**
```
my-blog/
├── Cargo.toml          # Rust dependencies
├── nucleus.config      # Framework config
├── src/
│   ├── views/          # Your pages (NCL files)
│   │   └── index.ncl   # Homepage
│   ├── logic/          # Business logic (Rust)
│   └── assets/         # Images to optimize
├── static/             # Public files (CSS, JS)
└── migrations/         # Database schemas
```

---

## Step 3: Start Development Server

```bash
nucleus run
```

Open **http://localhost:3000** - you should see "Welcome to Nucleus!"

> 💡 **Hot Reload**: Edit any file and the browser refreshes automatically.

---

## Step 4: Create Your First Page

Create `src/views/about.ncl`:

```html
<n:view title="About Us" description="Learn about our company">
    <main>
        <h1>About Us</h1>
        <p>We build amazing things with Nucleus.</p>
        
        <n:link href="/">Back to Home</n:link>
    </main>
    
    <style>
        main {
            max-width: 600px;
            margin: 2rem auto;
            font-family: system-ui;
        }
        h1 {
            color: #3b82f6;
        }
    </style>
</n:view>
```

Visit **http://localhost:3000/about** - your new page is live!

---

## Step 5: Add Data (Optional)

### Create a Database Table

```bash
nucleus db new create_posts
```

Edit `migrations/YYYYMMDD_create_posts.sql`:
```sql
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Seed data
INSERT INTO posts (title, content) VALUES 
    ('Hello World', 'My first blog post'),
    ('Getting Started', 'Learning Nucleus is fun!');
```

Run migration:
```bash
nucleus db up
```

### Display Data in View

Create `src/views/posts.ncl`:
```html
<n:view title="Blog Posts">
    <!-- Using Active Record to fetch all posts in one line -->
    <n:model posts="db::Post::query().order_by('id', 'DESC').all().await" />
    
    <main>
        <h1>Blog</h1>
        
        <n:for item="post" in="posts">
            <article>
                <h2>{post.title}</h2>
                <p>{post.content}</p>
            </article>
        </n:for>
    </main>
</n:view>
```

Create `src/logic/db.rs`:
```rust
use serde::Serialize;
use nucleus_std::impl_model;

#[derive(Serialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
}

// Enable Active Record for Post
// Enable Active Record for Post
nucleus_std::photon::model!(Post, "posts");
```

> **Note**: Don't forget to initialize the database in `src/main.rs`:
> ```rust
> nucleus_std::photon::init_db("sqlite:nucleus.db").await.ok();
> ```

Visit **http://localhost:3000/posts**!

---

## Step 7: Add Interactivity (Islands)

Nucleus supports client-side WASM "Islands" for zero-latency interactions without full page loads.

Create `src/views/components/Counter.ncl`:
```html
<n:island>
    <script lang="rs">
        use nucleus_std::neutron::Signal;
        
        // State lives on the client!
        let count = Signal::new(0);
        
        fn increment() {
            count.update(|c| c + 1);
        }
    </script>
    
    <div class="counter">
        <p>Count: <n:text value="count" /></p>
        <button on:click="increment()">+1</button>
    </div>
    
    <style>
        .counter { padding: 1rem; border: 1px solid #ddd; }
        button { background: #000; color: #fff; padding: 0.5rem 1rem; }
    </style>
</n:island>
```

Usage in `index.ncl`:
```html
<n:include src="src/views/components/Counter.ncl" />
```

This compiles to Wasm and hydrates partially on the client!

---

## Step 6: Build for Production

```bash
nucleus build
```

This creates an optimized binary at `target/release/server`.

Run it:
```bash
./target/release/server
```

---

## Next Steps

| What to Learn | Documentation |
|---------------|---------------|
| All NCL tags & syntax | [Syntax Reference](#19_syntax_reference) |
| Database operations | [Database Guide](#20_database_guide) |
| User authentication | [Authentication Guide](#21_authentication_guide) |
| Building APIs | [API Development](#22_api_development) |
| Deploying to production | [Deployment Guide](#23_deployment_guide) |

---

## Common Commands

| Command | Description |
|---------|-------------|
| `nucleus new <name>` | Create new project |
| `nucleus run` | Start dev server (HMR) |
| `nucleus build` | Compile for production |
| `nucleus deploy` | Generate Dockerfile |
| `nucleus db new <name>` | Create migration |
| `nucleus db up` | Run migrations |
| `nucleus test` | Run tests |

---

## Troubleshooting

### "Command not found: nucleus"
Make sure Cargo's bin directory is in your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### "Failed to connect to database"
The database file `nucleus.db` is created automatically. If you deleted it:
```bash
nucleus db up
```

### "Port 3000 already in use"
Change the port:
```bash
PORT=3001 nucleus run
```
