# Configuration Reference

Complete reference for all configuration options in `nucleus.config`.

---

## File Format

The configuration file uses TOML syntax:

```toml
# Comments start with #
key = "value"
number = 3000
boolean = true

[section]
nested_key = "nested value"
```

---

## Environment Variables

Use `${VAR}` syntax to reference environment variables:

```toml
database = "${DATABASE_URL}"
secret_key = "${SECRET_KEY}"
```

### Default Values

Provide defaults with `:-` syntax:

```toml
port = "${PORT:-3000}"
mode = "${NODE_ENV:-development}"
```

---

## Server Configuration

### port

| Type | Default | Description |
|------|---------|-------------|
| `integer` | `3000` | HTTP server port |

```toml
port = 8080
```

### host

| Type | Default | Description |
|------|---------|-------------|
| `string` | `"0.0.0.0"` | Network interface to bind |

```toml
host = "127.0.0.1"  # Localhost only
host = "0.0.0.0"    # All interfaces (production)
```

### mode

| Type | Default | Description |
|------|---------|-------------|
| `string` | `"development"` | Application mode |

Values:
- `development` - Debug logging, hot reload enabled
- `production` - Optimized, security hardened
- `test` - Testing mode

```toml
mode = "production"
```

### workers

| Type | Default | Description |
|------|---------|-------------|
| `integer` | CPU cores | Number of worker threads |

```toml
workers = 4
```

---

## Database Configuration

### database

| Type | Default | Description |
|------|---------|-------------|
| `string` | `null` | Database connection URL |

Supported formats:

```toml
# PostgreSQL
database = "postgres://user:pass@localhost:5432/mydb"

# MySQL
database = "mysql://user:pass@localhost:3306/mydb"

# SQLite
database = "sqlite://./data.db"
database = "sqlite://:memory:"  # In-memory

# With SSL
database = "postgres://user:pass@host/db?sslmode=require"
```

### pool_size

| Type | Default | Description |
|------|---------|-------------|
| `integer` | `10` | Maximum database connections |

```toml
pool_size = 20
```

### pool_timeout

| Type | Default | Description |
|------|---------|-------------|
| `integer` | `30` | Connection timeout in seconds |

```toml
pool_timeout = 10
```

---

## Security Configuration

### secret_key

| Type | Default | Description |
|------|---------|-------------|
| `string` | **Required in prod** | Key for JWT signing and encryption |

⚠️ **Security Warning**: Never commit secrets to version control!

```toml
secret_key = "${SECRET_KEY}"
```

Generate a secure key:
```bash
openssl rand -hex 32
```

### omit_signature

| Type | Default | Description |
|------|---------|-------------|
| `boolean` | `false` | Hide `X-Powered-By: Nucleus` header |

```toml
omit_signature = true  # Don't reveal framework identity
```

### cors

| Type | Default | Description |
|------|---------|-------------|
| `table` | Disabled | CORS configuration |

```toml
[cors]
origins = ["https://example.com", "https://app.example.com"]
methods = ["GET", "POST", "PUT", "DELETE"]
headers = ["Content-Type", "Authorization"]
credentials = true
max_age = 86400
```

### csp

| Type | Default | Description |
|------|---------|-------------|
| `table` | Strict defaults | Content Security Policy |

```toml
[csp]
default_src = ["'self'"]
script_src = ["'self'", "https://cdn.example.com"]
style_src = ["'self'", "https://fonts.googleapis.com"]
img_src = ["'self'", "data:", "https:"]
font_src = ["'self'", "https://fonts.gstatic.com"]
connect_src = ["'self'", "wss://api.example.com"]
frame_ancestors = ["'none'"]
```

### rate_limit

| Type | Default | Description |
|------|---------|-------------|
| `table` | Disabled | Rate limiting configuration |

```toml
[rate_limit]
enabled = true
requests = 100        # Max requests
window = 60           # Per window (seconds)
by = "ip"             # "ip" or "user"
whitelist = ["127.0.0.1"]
```

---

## Features Configuration

### hot_reload

| Type | Default | Description |
|------|---------|-------------|
| `boolean` | `true` (dev) | Enable hot module reloading |

```toml
hot_reload = true
```

### compression

| Type | Default | Description |
|------|---------|-------------|
| `boolean` | `true` (prod) | Enable gzip/brotli compression |

```toml
compression = true
```

### cache

| Type | Default | Description |
|------|---------|-------------|
| `table` | Disabled | Static file caching |

```toml
[cache]
enabled = true
max_age = 31536000    # 1 year for static assets
etag = true
```

---

## Performance Configuration

### [performance]

```toml
[performance]
compression = true            # Enable gzip/brotli compression
inline_critical_css = true    # Inline above-the-fold CSS
```

### [performance.cache]

Cache-Control header settings for optimal browser caching:

```toml
[performance.cache]
css_max_age = 31536000      # 1 year (in seconds)
js_max_age = 31536000       # 1 year
font_max_age = 31536000     # 1 year
image_max_age = 604800      # 1 week
html_no_cache = true        # HTML pages should not be cached
immutable = true            # Add 'immutable' directive for versioned assets
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `css_max_age` | `integer` | `31536000` | Max-age for CSS files (1 year) |
| `js_max_age` | `integer` | `31536000` | Max-age for JavaScript files |
| `font_max_age` | `integer` | `31536000` | Max-age for font files |
| `image_max_age` | `integer` | `604800` | Max-age for images (1 week) |
| `html_no_cache` | `boolean` | `true` | Disable caching for HTML |
| `immutable` | `boolean` | `true` | Add immutable directive |

### [performance.fonts]

Font loading optimization to prevent render-blocking:

```toml
[performance.fonts]
display_swap = true         # Use font-display: swap (prevents FOIT)
preconnect = true           # Add <link rel="preconnect"> hints
async_load = true           # Load fonts asynchronously (non-render-blocking)
google_fonts_url = "https://fonts.googleapis.com/css2?family=Inter:wght@400;600&display=swap"
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `display_swap` | `boolean` | `true` | Use `font-display: swap` |
| `preconnect` | `boolean` | `true` | Add preconnect hints |
| `async_load` | `boolean` | `true` | Non-render-blocking loading |
| `google_fonts_url` | `string` | `null` | Google Fonts URL (optional) |

### How It Works

When these settings are enabled, Nucleus automatically:

1. **Preconnect**: Adds `<link rel="preconnect">` to external font origins
2. **Font Display**: Appends `&display=swap` to Google Fonts URLs
3. **Async Loading**: Uses the media swap trick:
   ```html
   <link href="..." rel="stylesheet" media="print" onload="this.media='all'">
   ```
4. **Cache Headers**: Sets optimal `Cache-Control` headers:
   ```
   Cache-Control: public, max-age=31536000, immutable
   ```

---

### log_format

| Type | Default | Description |
|------|---------|-------------|
| `string` | `"pretty"` | Log output format |

Values: `pretty`, `json`, `compact`

```toml
log_format = "json"  # For production log aggregation
```

---

## Email Configuration

### [email]

```toml
[email]
provider = "smtp"     # "smtp", "sendgrid", "mailgun", "ses"
from = "noreply@example.com"

# SMTP settings
host = "smtp.example.com"
port = 587
username = "${SMTP_USER}"
password = "${SMTP_PASS}"
tls = true

# Or API-based provider
api_key = "${SENDGRID_API_KEY}"
```

---

## Payments Configuration

### [payments]

```toml
[payments]
# Stripe
stripe_key = "${STRIPE_SECRET_KEY}"
stripe_webhook_secret = "${STRIPE_WEBHOOK_SECRET}"

# Crypto (optional)
crypto_enabled = true
eth_rpc = "https://mainnet.infura.io/v3/${INFURA_KEY}"
```

---

## Build Configuration

### [build]

```toml
[build]
target = "x86_64-unknown-linux-gnu"
features = ["postgres", "redis"]
optimize = true          # Enable LTO
strip = true             # Strip debug symbols
```

---

## Static Files

### [static]

```toml
[static]
path = "./static"        # Static files directory
prefix = "/assets"       # URL prefix
max_age = 86400          # Cache duration (seconds)
```

---

## Complete Example

```toml
# nucleus.config

# Server
port = "${PORT:-3000}"
host = "0.0.0.0"
mode = "${NUCLEUS_ENV:-development}"
workers = 4

# Database
database = "${DATABASE_URL}"
pool_size = 20

# Security
secret_key = "${SECRET_KEY}"
omit_signature = true

[cors]
origins = ["https://example.com"]
credentials = true

[csp]
default_src = ["'self'"]
script_src = ["'self'"]
style_src = ["'self'", "https://fonts.googleapis.com"]

[rate_limit]
enabled = true
requests = 100
window = 60

# Features
hot_reload = false
compression = true

[cache]
enabled = true
max_age = 31536000

# Logging
log_level = "info"
log_format = "json"

# Email
[email]
provider = "smtp"
host = "${SMTP_HOST}"
port = 587
username = "${SMTP_USER}"
password = "${SMTP_PASS}"
from = "noreply@example.com"

# Payments
[payments]
stripe_key = "${STRIPE_SECRET_KEY}"

# Static files
[static]
path = "./static"
prefix = "/assets"

# Build
[build]
target = "x86_64-unknown-linux-musl"
optimize = true
```

---

## See Also

- [Getting Started](#01_getting_started)
- [CLI Reference](#17_cli_reference)
- [Deployment Guide](#23_deployment_guide)
