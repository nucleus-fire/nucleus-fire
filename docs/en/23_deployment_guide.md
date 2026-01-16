# Deployment & Hosting Guide

> Complete guide to hosting and deploying Nucleus applications to production.

## Quick Start

The fastest way to prepare your app for deployment:

```bash
nucleus deploy
```

This interactive command:
- 🎯 **Guides you** through platform selection (Docker, Fly.io, Railway, Render)
- 📦 **Generates** optimized Dockerfile and platform-specific configs
- 🔔 **Notifies you** when preparation completes

Or specify a target directly:

```bash
nucleus deploy --target fly      # For Fly.io
nucleus deploy --target railway  # For Railway
nucleus deploy --target docker   # For self-hosting
nucleus deploy --target manual   # Generate all configs
```

---

## Hosting Options

| Provider | Type | Effort | Cost | Recommended For |
|----------|------|--------|------|-----------------|
| **Railway** | PaaS | Very Low | $5/mo+ | Quick starts, hobby projects |
| **Render** | PaaS | Low | $7/mo+ | Managed scaling |
| **Fly.io** | PaaS | Low | Pay-as-you-go | Global distribution |
| **DigitalOcean** | VPS / App Platform | Medium | $4/mo+ | Control & predictable pricing |
| **Google Cloud Run** | Serverless | Medium | Pay-as-you-go | Auto-scaling to zero |
| **AWS/Azure** | Enterprise | High | Variable | Enterprise compliance |

---

## Cloud Deployments & Hosting

### Railway (Recommended)

Railway detects the Dockerfile automatically.

1. Create a GitHub repo for your Nucleus app.
2. Sign up at [railway.app](https://railway.app).
3. Click "New Project" > "Deploy from GitHub repo".
4. Add Environment Variable:
   - `DATABASE_URL` (Railway can provision a dedicated PostgreSQL/Redis for you)
   - `JWT_SECRET`
5. Railway will automatically build and deploy using the `Dockerfile`.

### Render

1. Create a `render.yaml` in your root:

```yaml
services:
  - type: web
    name: nucleus-app
    env: docker
    plan: free
    region: ohio
    envVars:
      - key: PORT
        value: 3000
      - key: DATABASE_URL
        fromDatabase:
          name: nucleus-db
          property: connectionString

databases:
  - name: nucleus-db
    plan: free
    databaseName: nucleus
    user: nucleus
```

2. Connect your GitHub account to Render.
3. Select "Blueprints" and use the `render.yaml`.

### Google Cloud Run

```bash
# Build and push
gcloud builds submit --tag gcr.io/PROJECT_ID/myapp

# Deploy
gcloud run deploy myapp \
    --image gcr.io/PROJECT_ID/myapp \
    --platform managed \
    --region us-central1 \
    --allow-unauthenticated \
    --set-env-vars "DATABASE_URL=..."
```

### AWS ECS

```json
{
  "family": "myapp",
  "containerDefinitions": [{
    "name": "myapp",
    "image": "123456789.dkr.ecr.us-east-1.amazonaws.com/myapp:latest",
    "portMappings": [{
      "containerPort": 3000,
      "hostPort": 3000
    }],
    "environment": [
      { "name": "PORT", "value": "3000" }
    ],
    "memory": 512,
    "cpu": 256
  }]
}
```

### DigitalOcean App Platform

```yaml
# .do/app.yaml
name: myapp
services:
  - name: web
    dockerfile_path: Dockerfile
    http_port: 3000
    instance_size_slug: basic-xxs
    instance_count: 1
    routes:
      - path: /
    envs:
      - key: DATABASE_URL
        value: ${database.DATABASE_URL}
```

### Fly.io

```toml
# fly.toml
app = "myapp"
primary_region = "lax"

[build]
  dockerfile = "Dockerfile"

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 0

[[vm]]
  cpu_kind = "shared"
  cpus = 1
  memory_mb = 256
```

```bash

---

## Self-Managed / Manual Hosting

### Docker Deployment

#### 1. Generate Dockerfile

```bash
nucleus deploy
```

This creates an optimized multi-stage Dockerfile:

```dockerfile
# Stage 1: Build
FROM rust:1.76-buster as builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin server

# Stage 2: Runtime
# We use Google's Distroless image for maximum security and minimal size (~20MB)
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/nucleus.config /app/nucleus.config
COPY --from=builder /app/static /app/static

ENV PORT=3000
EXPOSE 3000

CMD ["./server"]
```

#### 2. Build & Run

```bash
docker build -t my-nucleus-app .
docker run -d -p 3000:3000 my-nucleus-app
```

### Binary Deployment (VPS)

#### 1. Build optimized binary

```bash
# Cargo.toml
[profile.release]
strip = true
lto = true
codegen-units = 1
```

```bash
cargo build --release
```

#### 2. Deploy to Server (e.g. EC2, DigitalOcean Droplet)

```bash
# Copy binary and assets
scp target/release/server user@server:/opt/myapp/
scp -r static user@server:/opt/myapp/

# Run with systemd
# ...see systemd config below...
```

### Nginx Reverse Proxy (Recommended for VPS)

```nginx
server {
    listen 80;
    server_name example.com;
    
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
    }
}
```

---

## Database in Production

### SQLite (Simple)

For SQLite in production:

1. **Use WAL mode** for better concurrency:
   ```sql
   PRAGMA journal_mode=WAL;
   ```

2. **Mount persistent volume** in Docker:
   ```yaml
   volumes:
     - db-data:/app/data
   ```

3. **Backup regularly**:
   ```bash
   sqlite3 nucleus.db ".backup /backups/nucleus-$(date +%Y%m%d).db"
   ```

### PostgreSQL (Scale)

For high-traffic apps, migrate to PostgreSQL:

1. Update `Cargo.toml`:
   ```toml
   sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
   ```

2. Update connection:
   ```rust
   let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
   ```

3. Managed options:
   - **Neon** (serverless)
   - **Supabase**
   - **Railway**
   - **AWS RDS**

---

## Environment Variables

### Required for Production

| Variable | Description | Example |
|----------|-------------|---------|
| `PORT` | Server port | `3000` |
| `DATABASE_URL` | DB connection | `sqlite:nucleus.db` |
| `JWT_SECRET` | Auth secret | `your-256-bit-secret` |
| `RUST_LOG` | Log level | `info` |

### Optional

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Bind address | `0.0.0.0` |
| `WORKERS` | Thread count | CPU cores |
| `MAX_CONNECTIONS` | DB pool size | `5` |

### Example `.env.production`

```env
PORT=3000
HOST=0.0.0.0
DATABASE_URL=sqlite:/app/nucleus.db
JWT_SECRET=your-very-long-random-secret-at-least-32-chars
RUST_LOG=info
```

---

## Monitoring

### Health Check Endpoint

```rust
async fn health() -> &'static str {
    "OK"
}

let app = Router::new()
    .route("/health", get(health))
    // ...
```

### Logging

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();
```

### Metrics (Prometheus)

```rust
use metrics_exporter_prometheus::PrometheusBuilder;

PrometheusBuilder::new()
    .with_http_listener(([0, 0, 0, 0], 9090))
    .install()
    .expect("failed to install Prometheus recorder");
```

---

## Performance Tuning

### 1. Enable Compression

```rust
use tower_http::compression::CompressionLayer;

let app = Router::new()
    // routes...
    .layer(CompressionLayer::new());
```

### 2. Static Asset Caching

```rust
use tower_http::set_header::SetResponseHeaderLayer;

let app = Router::new()
    .nest_service("/static", ServeDir::new("static"))
    .layer(SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable")
    ));
```

### 3. Connection Pooling

```rust
let pool = SqlitePoolOptions::new()
    .max_connections(10)
    .connect("sqlite:nucleus.db")
    .await?;
```

---

## Security Checklist

- [ ] HTTPS enabled (via nginx/reverse proxy)
- [ ] Secure headers set (HSTS, X-Content-Type-Options, etc.)
- [ ] Environment variables not in code
- [ ] JWT_SECRET is long and random
- [ ] Database has proper permissions
- [ ] Rate limiting enabled
- [ ] CORS configured correctly
- [ ] Input validation on all endpoints
- [ ] SQL injection prevention (use parameterized queries)
- [ ] Regular security updates

---

## Graceful Shutdown

Handle termination signals properly to avoid dropping in-flight requests.

### Signal Handler

```rust
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    tracing::info!("Shutdown signal received, draining connections...");
}
```

### Usage with Axum

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;
```

### Drain Timeout

Give in-flight requests time to complete:

```rust
// Kubernetes/Docker: Set terminationGracePeriodSeconds
# docker-compose.yml
services:
  app:
    stop_grace_period: 30s
```

---

## Zero-Downtime Deploys

### Blue-Green Deployment

```bash
# Start new version on different port
./server-new --port 3001 &

# Health check
curl http://localhost:3001/health

# Switch nginx upstream
sed -i 's/3000/3001/' /etc/nginx/sites-available/myapp
nginx -s reload

# Stop old version
kill $OLD_PID
```

### Rolling Update (Docker Swarm/K8s)

```bash
docker service update --image myapp:v2 myapp
```
