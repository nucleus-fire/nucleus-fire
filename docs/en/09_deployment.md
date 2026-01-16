# Deployment Guide

Nucleus is designed for production-ready deployment with minimal configuration.

---

## Quick Deploy

### 1. Build for Production

```bash
cd your-project
nucleus build
```

This creates an optimized binary at `./target/release/server`.

### 2. Run Production Server

```bash
./target/release/server
```

---

## Docker Deployment

### Auto-Generated Dockerfile

Run in your project root:

```bash
nucleus deploy
```

If you don't have a `Dockerfile`, Nucleus generates an optimized one:

```dockerfile
# Builder stage
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage (minimal image)
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/server /server
COPY --from=builder /app/static /static
COPY --from=builder /app/nucleus.config /nucleus.config
CMD ["/server"]
```

### Build and Run

```bash
# Build image
docker build -t my-nucleus-app .

# Run container
docker run -p 3000:3000 -d my-nucleus-app
```

---

## Cloud Deployment

### Railway

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway init
railway up
```

### Fly.io

```bash
# Install Fly CLI
curl -L https://fly.io/install.sh | sh

# Launch app
fly launch
fly deploy
```

### AWS ECS / Fargate

1. Build and push Docker image to ECR
2. Create ECS task definition
3. Deploy service with load balancer

---

## Environment Configuration

### Production Settings

```toml
# nucleus.config
[server]
port = 3000
host = "0.0.0.0"

[database]
url = "postgres://user:pass@db.example.com/myapp"

[app]
env = "production"
secret_key = "${SECRET_KEY}"  # Use environment variable
```

### Environment Variables

```bash
# Set secrets via environment
export SECRET_KEY="your-production-secret"
export DATABASE_URL="postgres://..."
```

---

## Health Checks

Nucleus provides built-in health endpoints:

```bash
# Check if server is alive
curl http://localhost:3000/health
# Returns: {"status": "ok"}

# Check database connectivity
curl http://localhost:3000/health/db
# Returns: {"status": "ok", "latency_ms": 2}
```

---

## SSL/TLS

### Using Reverse Proxy (Recommended)

Use Nginx, Caddy, or cloud load balancers:

```nginx
# Nginx config
server {
    listen 443 ssl;
    server_name example.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Caddy (Auto SSL)

```
example.com {
    reverse_proxy localhost:3000
}
```

---

## Monitoring

### Prometheus Metrics

Enable metrics endpoint:

```toml
[monitoring]
prometheus = true
```

Scrape from `/metrics`.

### Logging

```bash
# JSON logs for production
RUST_LOG=info,site=debug cargo run --release

# Log to file
./server 2>&1 | tee app.log
```

---

## Database Migrations in Production

Migrations run automatically on server start. For manual control:

```bash
# Run pending migrations
nucleus migrate

# Rollback last migration
nucleus migrate:rollback

# Check migration status
nucleus migrate:status
```

---

## Scaling

### Horizontal Scaling

Nucleus is stateless - run multiple instances behind a load balancer:

```yaml
# docker-compose.yml
services:
  app:
    image: my-nucleus-app
    deploy:
      replicas: 4
    ports:
      - "3000"
```

### Connection Pooling

Configure database pool size:

```toml
[database]
url = "postgres://..."
pool_size = 20
```

---

## Checklist

- [ ] Set `env = "production"` in config
- [ ] Use strong `secret_key`
- [ ] Configure database connection pooling
- [ ] Set up SSL/TLS via reverse proxy
- [ ] Configure health checks
- [ ] Set up log aggregation
- [ ] Configure monitoring/alerting
- [ ] Test deployment with load testing
- [ ] Set up backup strategy for database
