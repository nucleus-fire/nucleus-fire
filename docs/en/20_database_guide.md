# Database Guide (Photon)

Complete guide to database operations in Nucleus using the Photon query builder.

---

## Setup

### Configuration

Add your database URL to `nucleus.config`:

```toml
[database]
# PostgreSQL
url = "postgres://user:password@localhost:5432/mydb"

# MySQL
url = "mysql://user:password@localhost:3306/mydb"

# SQLite
url = "sqlite:./data.db"

# Using environment variable (recommended)
url = "${DATABASE_URL}"
```

### Connection Pool

Configure pool behavior for production deployments:

```toml
[database]
url = "${DATABASE_URL}"

# Pool configuration
max_connections = 10      # Maximum pool size (default: 10)
min_connections = 2       # Keep alive connections (default: 0)
connect_timeout = 5       # Seconds to wait for connection
idle_timeout = 300        # Seconds before idle connection is closed
max_lifetime = 3600       # Maximum connection lifetime in seconds
```

### Connection Failure Handling

```rust
use nucleus_std::photon::{init_db_with_options, DbOptions};

// Retry connection with exponential backoff
let options = DbOptions::default()
    .max_retries(3)
    .retry_delay(Duration::from_secs(2));

init_db_with_options("postgres://...", options).await?;
```

### Initializing

```rust
use nucleus_std::photon::{init_db, db};

// Initialize at app startup
init_db("sqlite:./data.db").await?;

// Access anywhere
let pool = db();
```

---

## CLI Commands

| Command | Description |
|---------|-------------|
| `nucleus db init` | Create migrations directory |
| `nucleus db new <name>` | Create a new migration file |
| `nucleus db up` | Apply all pending migrations |
| `nucleus db up --step N` | Apply N migrations |
| `nucleus db down` | Rollback last migration |
| `nucleus db down --step N` | Rollback N migrations |
| `nucleus db status` | Show migration status |

### Example Workflow

```bash
# Initialize migrations folder
nucleus db init

# Create your first migration
nucleus db new create_users

# Edit migrations/20241225_create_users.sql
# Then apply
nucleus db up

# Check status
nucleus db status
# ✅ 20241225_create_users.sql (applied)
```

---

## Models

### Defining Models

```rust
use nucleus_std::{impl_model, photon::Model};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl_model!(User, "users");
```

### Generated Methods

The `impl_model!` macro generates:

```rust
impl User {
    fn table_name() -> &'static str;      // "users"
    fn query() -> Builder<'_>;            // Start query builder
    async fn find<T>(id: i64) -> Result<Option<T>, Error>;
    async fn delete_by_id(id: i64) -> Result<u64, Error>;
}
```

---

## Query Builder

### Basic Queries

```rust
use nucleus_std::photon::{Model, Op};

// Find all users
let users = User::query().all::<User>().await?;

// Find by ID
let user = User::query()
    .r#where("id", 1)
    .first::<User>()
    .await?;

// Find with conditions
let admins = User::query()
    .filter_op("role", Op::Eq, "admin")
    .filter_op("active", Op::Eq, true)
    .all::<User>()
    .await?;
```

### Operators

| Operator | SQL | Example |
|----------|-----|---------|
| `Op::Eq` | `=` | `.filter_op("age", Op::Eq, 25)` |
| `Op::Ne` | `!=` | `.filter_op("status", Op::Ne, "banned")` |
| `Op::Gt` | `>` | `.filter_op("score", Op::Gt, 100)` |
| `Op::Gte` | `>=` | `.filter_op("age", Op::Gte, 18)` |
| `Op::Lt` | `<` | `.filter_op("price", Op::Lt, 50)` |
| `Op::Lte` | `<=` | `.filter_op("quantity", Op::Lte, 10)` |
| `Op::Like` | `LIKE` | `.filter_op("name", Op::Like, "%smith%")` |
| `Op::ILike` | `ILIKE` | `.filter_op("email", Op::ILike, "%@gmail.com")` |
| `Op::IsNull` | `IS NULL` | `.filter_op("deleted_at", Op::IsNull, 0)` |
| `Op::IsNotNull` | `IS NOT NULL` | `.filter_op("verified_at", Op::IsNotNull, 0)` |

### OR Conditions

```rust
// Find active OR admin users
let users = User::query()
    .filter_op("status", Op::Eq, "active")
    .or_where("role", "admin")
    .all::<User>()
    .await?;
```

### Ordering & Pagination

```rust
let users = User::query()
    .order_by("created_at", "DESC")
    .limit(10)
    .offset(20)
    .all::<User>()
    .await?;
```

### Count & Exists

```rust
// Count matching rows
let count = User::query()
    .filter_op("status", Op::Eq, "active")
    .count()
    .await?;

// Check if any match
let has_admins = User::query()
    .filter_op("role", Op::Eq, "admin")
    .exists()
    .await?;
```

### Joins

```rust
// INNER JOIN
let posts = Post::query()
    .join("users", "posts.user_id", "users.id")
    .all::<Post>()
    .await?;

// LEFT JOIN
let posts = Post::query()
    .left_join("comments", "posts.id", "comments.post_id")
    .all::<Post>()
    .await?;
```

---

## CRUD Operations

### Create

```rust
let result = User::query()
    .insert()
    .value("name", "Alice")
    .value("email", "alice@example.com")
    .execute()
    .await?;

println!("Created {} rows", result);
```

### Read

```rust
// All
let users = User::query().all::<User>().await?;

// First match
let user = User::query()
    .filter_op("email", Op::Eq, "alice@example.com")
    .first::<User>()
    .await?;
```

### Update

```rust
let affected = User::query()
    .update()
    .value("name", "Alice Smith")
    .r#where("id", 1)
    .execute()
    .await?;
```

### Delete

```rust
let deleted = User::query()
    .delete()
    .r#where("id", 1)
    .execute()
    .await?;
```

---

## Transactions

```rust
use nucleus_std::photon::transaction;

transaction(|tx| Box::pin(async move {
    sqlx::query("UPDATE accounts SET balance = balance - 100 WHERE id = ?")
        .bind(1)
        .execute(&mut **tx)
        .await?;
    
    sqlx::query("UPDATE accounts SET balance = balance + 100 WHERE id = ?")
        .bind(2)
        .execute(&mut **tx)
        .await?;
    
    Ok(())
})).await?;
```

Transactions automatically:
- **Commit** on success (Ok)
- **Rollback** on error (Err) or panic

---

## Migrations

### Migration File Format

```sql
-- Migration: create_users
-- Created: 2024-12-25 15:30:00 UTC

-- UP
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);

-- DOWN
DROP TABLE users;
```

### Programmatic Migrations

```rust
use nucleus_std::photon::migrations::{run_migrations, rollback, migration_status};

// Apply all pending
let applied = run_migrations("./migrations").await?;

// Rollback last 2
let rolled_back = rollback(2).await?;

// Check status
let status = migration_status("./migrations").await?;
for m in status {
    println!("{}: {}", m.name, if m.applied { "✅" } else { "⏳" });
}
```

---

## Relationships

### Defining

```rust
use nucleus_std::photon::{HasMany, BelongsTo, Model};

struct User { id: i64 }
struct Post { id: i64, user_id: i64 }

impl HasMany<Post> for User {
    fn foreign_key() -> &'static str { "user_id" }
    fn get_id(&self) -> i64 { self.id }
}

impl BelongsTo<User> for Post {
    fn foreign_key() -> &'static str { "user_id" }
    fn get_foreign_key_value(&self) -> i64 { self.user_id }
}
```

### Eager Loading

```rust
let users = User::query()
    .include("posts")
    .all::<User>()
    .await?;
```

---

## Multi-Database Support

Photon auto-detects the database from the URL:

| URL Prefix | Database |
|------------|----------|
| `postgres://`, `postgresql://` | PostgreSQL |
| `mysql://`, `mariadb://` | MySQL |
| `sqlite://`, `sqlite:` | SQLite |

Query placeholders adapt automatically:
- PostgreSQL: `$1, $2, $3`
- MySQL/SQLite: `?, ?, ?`

---

## Best Practices

### 1. Use Parameterized Queries

```rust
// ✅ Safe
.filter_op("email", Op::Eq, user_input)

// ❌ Never do this
format!("WHERE email = '{}'", user_input)
```

### 2. Use Transactions for Multi-Step Operations

```rust
transaction(|tx| Box::pin(async move {
    // All or nothing
    Ok(())
})).await?;
```

### 3. Index Frequently Queried Columns

```sql
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_posts_user_id ON posts(user_id);
```

---

## See Also

- [Getting Started](#01_getting_started)
- [Standard Library Reference](#04_stdlib_reference)
- [Authentication Guide](#21_authentication_guide)
