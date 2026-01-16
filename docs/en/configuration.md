# Configuration Reference

Complete reference for all configuration options in `nucleus.config`.

---

## File Format

The configuration file uses TOML syntax and is strictly typed. Keys must be organized into their respective table sections (`[server]`, `[database]`, etc.).

```toml
version = "1.0.0"

[server]
port = 3000
host = "0.0.0.0"
```

---

## Environment Variables

Use `${VAR}` syntax to reference environment variables:

```toml
[database]
url = "${DATABASE_URL}"

[app]
secret_key = "${SECRET_KEY}"
```

### Default Values

Provide defaults with `:-` syntax:

```toml
[server]
port = "${PORT:-3000}"
```

---

## Server Configuration

### [server]

```toml
[server]
port = 3000              # HTTP server port
host = "0.0.0.0"         # Network interface
environment = "development" # "development" or "production"
omit_signature = false   # Hide X-Powered-By header
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `port` | `integer` | `3000` | Server port |
| `host` | `string` | `"0.0.0.0"` | Bind address |
| `environment` | `string` | `"development"` | App environment |
| `omit_signature` | `boolean` | `false` | Hide framework headers |

---

## Database Configuration

### [database]

```toml
[database]
url = "sqlite:nucleus.db" # Connection string
max_connections = 5       # Connection pool size
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `url` | `string` | `sqlite:nucleus.db` | Database URL |
| `max_connections` | `integer` | `5` | Max pool connections |

Supported URL formats:
- `postgres://user:pass@host/db`
- `mysql://user:pass@host/db`
- `sqlite:./data.db`

---

## App Configuration

### [app]

```toml
[app]
name = "My App"
secret_key = "${SECRET_KEY}" # Required for auth/crypto
admin_username = "admin"
admin_password = "${ADMIN_PASSWORD}"
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | `string` | `""` | Application name |
| `secret_key` | `string` | `""` | Secret for HMAC/Encryption |
| `admin_username` | `string` | `""` | Default admin user |
| `admin_password` | `string` | `""` | Default admin password |

---

## Performance Configuration

### [performance]

```toml
[performance]
compression = true            # Enable gzip/brotli
inline_critical_css = true    # Inline above-the-fold CSS
preconnect_origins = []       # List of origins to preconnect
```

### [performance.cache]

Controls `Cache-Control` headers for static assets.

```toml
[performance.cache]
css_max_age = 31536000      # 1 year
js_max_age = 31536000       # 1 year
font_max_age = 31536000     # 1 year
image_max_age = 604800      # 1 week
html_no_cache = true        # No-cache for HTML
immutable = true            # Add immutable directive
```

### [performance.fonts]

```toml
[performance.fonts]
display_swap = true         # font-display: swap
preconnect = true           # Preconnect to font providers
async_load = true           # Non-blocking load
# google_fonts_url = "..."  # Optional override
```

---

## Payments Configuration

### [payments]

```toml
[payments]
stripe_key = "${STRIPE_KEY}"
currency = "USD"
```

---

## Chain Configuration

### [chain]

```toml
[chain]
rpc_url = "https://mainnet.infura.io/v3/..."
chain_id = 1
```

---

## Complete Example

```toml
version = "1.0.0"

[server]
port = "${PORT:-3000}"
host = "0.0.0.0"
environment = "production"
omit_signature = true

[database]
url = "${DATABASE_URL}"
max_connections = 20

[app]
name = "Production App"
secret_key = "${SECRET_KEY}"

[performance]
compression = true
inline_critical_css = true

[performance.cache]
immutable = true

[payments]
stripe_key = "${STRIPE_KEY}"
currency = "USD"
```

---

## See Also

- [Getting Started](#01_getting_started)
- [Deployment Guide](#23_deployment_guide)
