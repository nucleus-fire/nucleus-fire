# CLI Reference

Complete reference for all Nucleus CLI commands, options, and usage examples.

---

## Installation

```bash
cargo install nucleus-cli
```

Verify:
```bash
nucleus --version
```

---

## Commands Overview

| Command | Description |
|---------|-------------|
| [`nucleus new`](#nucleus-new) | Create a new project |
| [`nucleus dev`](#nucleus-dev) | Development server with hot reload |
| [`nucleus run`](#nucleus-run) | Start development server (interpreter mode) |
| [`nucleus build`](#nucleus-build) | Build for production |
| [`nucleus deploy`](#nucleus-deploy) | Deploy to any platform |
| [`nucleus deploy init`](#nucleus-deploy-init) | Generate deployment files (Dockerfile, fly.toml) |
| [`nucleus export`](#nucleus-export) | Static site generation (SSG) |
| [`nucleus publish`](#nucleus-publish) | Publish static site to platform |
| [`nucleus test`](#nucleus-test) | Run tests |
| [`nucleus db`](#nucleus-db) | Database operations |
| [`nucleus generate`](#nucleus-generate) | Code generators |
| [`nucleus install`](#nucleus-install) | Install dependencies |
| [`nucleus console`](#nucleus-console) | Interactive REPL for database queries |
| [`nucleus studio`](#nucleus-studio) | Web-based database management UI |
| [`nucleus browser`](#nucleus-browser) | Headless browser automation |

---

## nucleus new

Create a new Nucleus project.

### Usage

```bash
nucleus new <name> [options]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | Yes | Project name (also directory name) |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--template` | `-t` | Template to use (`default`, `api`, `minimal`) |
| `--database` | `-d` | Database type (`postgres`, `mysql`, `sqlite`) |
| `--no-git` | | Skip git initialization |

### Examples

```bash
# Basic project
nucleus new my-app

# API-only project (no views)
nucleus new my-api --template api

# With SQLite database
nucleus new my-app --database sqlite

# Minimal (no examples)
nucleus new my-app --template minimal
```

### Generated Structure

```
my-app/
├── Cargo.toml
├── nucleus.config
├── src/
│   ├── main.rs
│   ├── views/
│   │   ├── layout.ncl
│   │   └── index.ncl
│   └── logic/
│       └── mod.rs
├── static/
│   └── assets/
└── migrations/
```

---

## nucleus dev

**Recommended for development.** Starts a development server with automatic file watching and hot reload.

### Usage

```bash
nucleus dev
```

Or with Make:
```bash
make dev
```

### What It Does

1. Runs initial `nucleus build` to generate code
2. Starts `cargo run --bin site` in the background
3. Watches for file changes in:
   - `src/views/*.ncl`
   - `src/components/*.ncl`
   - `src/assets/*.css`
   - `static/assets/*`
4. Automatically rebuilds and restarts the server on changes
5. 500ms debounce to prevent rapid rebuilds

### Example

```bash
cd my-project
nucleus dev

# Output:
# ⚛️  Starting Nucleus Dev Server with HMR...
# 📦 Initial build...
# ✅ Build Complete.
# 🚀 Server started. Watching for changes...
#    Press Ctrl+C to stop.
```

When you edit a `.ncl` file:
```
🔄 Change detected, rebuilding...
✅ Rebuild complete!
🚀 Server restarted.
```

### Installing the CLI

To use `nucleus dev` directly (instead of `cargo run -p nucleus-cli -- dev`):

```bash
# From the nucleus-lang directory
cargo install --path crates/nucleus-cli

# Now you can run from any project
nucleus dev
```

---

## nucleus run

Start the development server in interpreter mode (reads .ncl files at runtime).

> **Note:** For most development, use [`nucleus dev`](#nucleus-dev) instead, which provides file watching and automatic rebuilds.

### Usage

```bash
nucleus run [options]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--port` | `-p` | 3000 | Port to listen on |
| `--host` | `-h` | 0.0.0.0 | Host to bind to |
| `--release` | `-r` | false | Run in release mode |
| `--no-reload` | | false | Disable hot reload |

### Examples

```bash
# Default (port 3000)
nucleus run

# Custom port
nucleus run --port 8080

# Production mode (no hot reload)
nucleus run --release

# Localhost only
nucleus run --host 127.0.0.1
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Log level (`trace`, `debug`, `info`, `warn`, `error`) |
| `NUCLEUS_ENV` | Environment (`development`, `production`) |

```bash
RUST_LOG=debug nucleus run
```

---

## nucleus build

Build the application for production.

### Usage

```bash
nucleus build [options]
```

### Options

| Option | Description |
|--------|-------------|
| `--target` | Compilation target (e.g., `x86_64-unknown-linux-musl`) |
| `--features` | Cargo features to enable |

### Examples

```bash
# Standard build
nucleus build

# Static Linux binary
nucleus build --target x86_64-unknown-linux-musl

# With specific features
nucleus build --features "postgres,redis"
```

### Output

Binary is generated at `target/release/<project-name>`.

---

## nucleus deploy

Interactive deployment wizard with multi-platform support.

### Usage

```bash
nucleus deploy [options]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--target` | `-t` | Target platform: `docker`, `fly`, `railway`, `render`, `manual` |

### Supported Platforms

| Platform | Description |
|----------|-------------|
| **Docker** | Generate Dockerfile for self-hosting |
| **Fly.io** | Generate fly.toml for global edge deployment |
| **Railway** | Generate railway.json for simple PaaS |
| **Render** | Generate render.yaml for managed hosting |
| **Manual** | Generate all config files at once |

### Examples

```bash
# Interactive mode (prompts for platform)
nucleus deploy

# Direct Docker deployment
nucleus deploy --target docker

# Fly.io deployment
nucleus deploy --target fly

# Generate all platform configs
nucleus deploy --target manual
```

### What Gets Generated

| Platform | Files Generated |
|----------|-----------------|
| Docker | `Dockerfile`, `.dockerignore` |
| Fly.io | `fly.toml`, `Dockerfile` |
| Railway | `railway.json`, `Dockerfile` |
| Render | `render.yaml`, `Dockerfile` |
| Manual | All of the above |

### Features

- ✨ **Interactive wizard** with beautiful terminal UI
- 🎯 **Platform detection** (detects existing fly/railway CLIs)
- 🔔 **Desktop notifications** when deployment prep completes
- 📦 **Auto-generation** of optimized multi-stage Dockerfile

### Next Steps After Deploy

```bash
# Docker
docker build -t my-app .
docker run -p 3000:3000 my-app

# Fly.io
fly launch && fly deploy

# Railway
railway up

# Render
# Push to GitHub, Render auto-deploys from render.yaml
```

---

## nucleus deploy init

Generate deployment configuration files without the interactive wizard.

### Usage

```bash
nucleus deploy init
```

### What Gets Generated

| File | Description |
|------|-------------|
| `Dockerfile` | Multi-stage optimized Docker build |
| `.dockerignore` | Excludes unnecessary files |
| `fly.toml` | Fly.io deployment configuration |

### Example

```bash
cd my-project
nucleus deploy init

# Output:
# ✅ Generated Dockerfile
# ✅ Generated .dockerignore
# ✅ Generated fly.toml
```

---

## nucleus export

Export your application as a static site (SSG).

### Usage

```bash
nucleus export [options]
```

### Options

| Option | Description |
|--------|-------------|
| `-o, --output` | Output directory (default: `dist`) |
| `--wizard` | Run interactive wizard |
| `--incremental` | Only rebuild changed files |
| `--base-url` | Base URL for generated links |
| `--platform` | Target: `netlify`, `vercel`, `cloudflare`, `github` |
| `--pwa` | Generate PWA assets (manifest, service worker) |
| `--pwa-name` | Custom name for the PWA |

### Examples

```bash
# Basic export
nucleus export

# With PWA support
nucleus export --pwa --pwa-name "My App"

# Incremental build
nucleus export --incremental

# For Netlify
nucleus export --platform netlify

# Custom output directory
nucleus export -o build
```

### PWA Assets Generated

When `--pwa` is used:

| File | Description |
|------|-------------|
| `manifest.json` | Web App Manifest for install prompts |
| `sw.js` | Service Worker with cache-first strategy |
| `offline.html` | Offline fallback page |
| `assets/neutron-store.js` | Client-side storage library |

---

## nucleus publish

Publish your static site to a hosting platform.

### Usage

```bash
nucleus publish [options]
```

### Options

| Option | Description |
|--------|-------------|
| `-p, --platform` | Target platform |

### Supported Platforms

- **Netlify** - Requires Netlify CLI authenticated
- **Vercel** - Requires Vercel CLI authenticated
- **Cloudflare Pages** - Requires Wrangler authenticated
- **GitHub Pages** - Pushes to `gh-pages` branch

---

## nucleus test

Run tests using the Guardian test runner.

### Usage

```bash
nucleus test [options] [filter]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `filter` | No | Only run tests matching this pattern |

### Options

| Option | Description |
|--------|-------------|
| `--unit` | Run unit tests only |
| `--integration` | Run integration tests only |
| `--coverage` | Generate coverage report |

### Examples

```bash
# Run all tests
nucleus test

# Run specific test
nucleus test user_creation

# With coverage
nucleus test --coverage
```

---

## nucleus db

Database migration commands.

### Subcommands

| Subcommand | Description |
|------------|-------------|
| `nucleus db new <name>` | Create a new migration file |
| `nucleus db up` | Run all pending migrations |
| `nucleus db down` | Rollback the last migration |
| `nucleus db status` | Show migration status |
| `nucleus db reset` | Drop and recreate database |

### nucleus db new

```bash
nucleus db new <name>
```

Creates a migration file in `migrations/`:

```
migrations/
└── 20241224120000_create_users.sql
```

Example:
```bash
nucleus db new create_users
nucleus db new add_email_to_users
```

### nucleus db up

```bash
nucleus db up [options]
```

Options:
| Option | Description |
|--------|-------------|
| `--step <n>` | Run only n migrations |
| `--dry-run` | Show SQL without executing |

### nucleus db down

```bash
nucleus db down [options]
```

Options:
| Option | Description |
|--------|-------------|
| `--step <n>` | Rollback n migrations |
| `--all` | Rollback all migrations |

### nucleus db status

```bash
nucleus db status
```

Output:
```
✅ 001_create_users        (applied 2024-12-24)
✅ 002_create_posts        (applied 2024-12-24)
⏳ 003_add_comments        (pending)
```

---

## nucleus generate

Code generation commands.

### Subcommands

| Subcommand | Description |
|------------|-------------|
| `generate scaffold` | Full CRUD scaffold |
| `generate model` | Model struct only |
| `generate migration` | Migration file only |
| `generate payments` | Payment components |

### nucleus generate scaffold

Generate a complete CRUD scaffold with model, views, and logic.

```bash
nucleus generate scaffold <name> [fields...]
```

#### Field Format

```
field_name:type
```

#### Supported Types

| Type | Rust | SQL | Example |
|------|------|-----|---------|
| `string` | `String` | `TEXT NOT NULL` | `name:string` |
| `text` | `String` | `TEXT` | `bio:text` |
| `int` | `i32` | `INTEGER NOT NULL` | `count:int` |
| `bigint` | `i64` | `BIGINT` | `views:bigint` |
| `float` | `f64` | `REAL` | `price:float` |
| `bool` | `bool` | `BOOLEAN NOT NULL` | `active:bool` |
| `datetime` | `DateTime<Utc>` | `TIMESTAMP` | `published_at:datetime` |
| `uuid` | `Uuid` | `UUID` | `external_id:uuid` |

#### Example

```bash
nucleus generate scaffold Post title:string body:text views:int published:bool
```

Generates:
```
src/logic/post.rs          # Model + CRUD functions
src/views/posts/index.ncl  # List view
src/views/posts/show.ncl   # Detail view
src/views/posts/new.ncl    # Create form
src/views/posts/edit.ncl   # Edit form
migrations/xxx_create_posts.sql  # Migration
```

### nucleus generate model

Generate only the model struct without views.

```bash
nucleus generate model <name> [fields...]
```

Example:
```bash
nucleus generate model Comment body:text post_id:int user_id:int
```

### nucleus generate payments

Generate payment integration (Stripe + Crypto).

```bash
nucleus generate payments [options]
```

Options:
| Option | Description |
|--------|-------------|
| `--subscription` | Include subscription pricing table |
| `--crypto` | Include crypto payment support |

---

## nucleus install

Install Nucleus modules and Rust crates.

### Usage

```bash
nucleus install <package>
```

### Examples

```bash
# Install a Rust crate
nucleus install serde

# Install a Nucleus module from registry
nucleus install @nucleus/auth

# Install from URL
nucleus install https://github.com/user/module
```

---

## nucleus console

Interactive REPL for database queries and exploration.

### Usage

```bash
nucleus console [options]
```

### Options

| Option | Description |
|--------|-------------|
| `--database` | Path to SQLite database file |

### Examples

```bash
# Start console with default database
nucleus console

# Specify database
nucleus console --database site.db
```

### Console Commands

| Command | Description |
|---------|-------------|
| `.tables` | List all tables |
| `.schema` | Show table schemas |
| `.quit` | Exit the console |
| SQL query | Execute SQL and show results |

### Example Session

```
nucleus> .tables
┌─────────────────────┐
│ Tables              │
├─────────────────────┤
│ users               │
│ posts               │
│ migrations          │
└─────────────────────┘

nucleus> SELECT * FROM users LIMIT 2;
┌────┬───────────────┬─────────────────────┐
│ id │ email         │ created_at          │
├────┼───────────────┼─────────────────────┤
│ 1  │ alice@foo.com │ 2024-01-15 10:30:00 │
│ 2  │ bob@bar.com   │ 2024-01-16 14:22:00 │
└────┴───────────────┴─────────────────────┘
```

---

## nucleus studio

Web-based database management UI with real-time editing.

### Usage

```bash
nucleus studio [options]
```

### Options

| Option | Description |
|--------|-------------|
| `--database` | Path to SQLite database file |
| `--port` | Web server port (default: 4000) |

### Examples

```bash
# Start studio on default port
nucleus studio --database site.db

# Custom port
nucleus studio --database site.db --port 8080
```

### Features

- 📊 **Table browser** - View and edit data inline
- 🔍 **SQL editor** - Execute custom queries with syntax highlighting
- 📝 **Schema viewer** - Explore table structures
- ➕ **CRUD operations** - Add, edit, delete rows via UI
- 📱 **Mobile responsive** - Works on tablet and mobile

---

## nucleus browser

Headless browser automation for testing and scraping.

### Usage

```bash
nucleus browser <command> [options]
```

### Subcommands

| Subcommand | Description |
|------------|-------------|
| `screenshot` | Capture a page screenshot |
| `pdf` | Generate PDF from page |
| `scrape` | Extract data from page |

### Examples

```bash
# Screenshot a page
nucleus browser screenshot https://example.com --output screen.png

# Generate PDF
nucleus browser pdf https://example.com --output page.pdf

# Scrape with selector
nucleus browser scrape https://example.com --selector "h1"
```

---

## Configuration File

The `nucleus.config` file supports all CLI defaults:

```toml
# Server
port = 3000
host = "0.0.0.0"
mode = "development"

# Database
database = "${DATABASE_URL}"

# Security
secret_key = "${SECRET_KEY}"
omit_signature = false

# Features
hot_reload = true

# Build
[build]
target = "x86_64-unknown-linux-gnu"
features = ["postgres"]
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Configuration error |
| 4 | Build error |
| 5 | Database error |

---

## See Also

- [Getting Started](#01_getting_started)
- [Configuration](#configuration)
- [Deployment Guide](#23_deployment_guide)
