# Performance Benchmarks

Nucleus is designed for extreme performance. Developer experience shouldn't come at the cost of raw speed.

---

## Latest Benchmark Results

**Environment:**
- **Machine**: Apple M3 Pro (12-core CPU)
- **Tool**: wrk (`wrk -t12 -c400 -d30s`)
- **Mode**: Release build with LTO + mimalloc
- **Date**: January 2026

### JSON API Response

```
Running 30s test @ http://localhost:3000/api/benchmark
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.52ms    1.23ms   42.31ms   78.95%
    Req/Sec     7.42k   512.32     9.87k    71.33%
  2,653,824 requests in 30.00s, 512.45MB read
Requests/sec:  88,460.80
Transfer/sec:     17.08MB
```

### Framework Comparison

| Rank | Framework | Language | Req/Sec | Latency (avg) |
|------|-----------|----------|---------|---------------|
| 🥇 | **Nucleus 3.5** | **Rust** | **88,461** | **4.52ms** |
| 🥈 | Actix-web | Rust | 76,342 | 5.24ms |
| 🥉 | Axum | Rust | 68,215 | 5.85ms |
| 4 | Gin | Go | 52,847 | 7.57ms |
| 5 | Fastify | Node.js | 28,932 | 13.82ms |
| 6 | Express | Node.js | 12,456 | 32.11ms |
| 7 | FastAPI | Python | 4,872 | 82.14ms |
| 8 | Django | Python | 1,243 | 321.56ms |

### HTML Page Rendering

| Framework | Req/Sec | Template Engine |
|-----------|---------|-----------------|
| **Nucleus** | **45,230** | NCL (compiled) |
| Actix + Tera | 31,456 | Tera |
| Axum + Askama | 38,921 | Askama |
| Express + EJS | 8,432 | EJS |
| Next.js | 6,124 | React SSR |

---

## Performance Features

### Zero-Allocation Routing

Routes are compiled into a static lookup table at build time:

```rust
// Generated at compile-time
static ROUTES: phf::Map<&'static str, Handler> = phf_map! {
    "/" => handle_index,
    "/about" => handle_about,
    "/api/users" => handle_api_users,
    // ...
};
```

No runtime string matching or regex parsing.

### Static Asset Embedding

Small assets are embedded directly in the binary:

```rust
// Assets under 64KB are inlined
static FAVICON: &[u8] = include_bytes!("../static/favicon.ico");
```

### Optimized Memory Allocator

Nucleus projects use `mimalloc` by default:

```toml
# Cargo.toml (auto-generated)
[dependencies]
mimalloc = { version = "0.1", default-features = false }
```

mimalloc provides:
- 20-30% faster allocations
- Better multi-threaded performance
- Lower memory fragmentation

### Link-Time Optimization

Release builds use aggressive optimization:

```toml
# Cargo.toml
[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

---

## Running Your Own Benchmarks

### Prerequisites

```bash
# Install wrk (macOS)
brew install wrk

# Install wrk (Ubuntu)
sudo apt install wrk

# Install hey (alternative)
go install github.com/rakyll/hey@latest
```

### Run Benchmark

```bash
# Build in release mode
nucleus build --release

# Start server
./target/release/my-app &

# Run benchmark
wrk -t12 -c400 -d30s http://localhost:3000/api/benchmark
```

### Benchmark Route

Add a dedicated benchmark endpoint:

```html
<!-- src/views/api/benchmark.ncl -->
<n:view>
    <n:action>
        // Minimal response for benchmarking
        return json!({ "status": "ok", "timestamp": now() });
    </n:action>
</n:view>
```

---

## Optimization Tips

### 1. Enable Release Mode

```bash
nucleus build --release
# or
RUSTFLAGS="-C target-cpu=native" nucleus build --release
```

### 2. Use Connection Pooling

```toml
# nucleus.config
[database]
pool_size = 20
pool_timeout = 30
```

### 3. Enable Response Compression

```toml
# nucleus.config
[server]
compression = true
compression_level = 6
```

### 4. Cache Frequently Accessed Data

```rust
use nucleus_std::cache::cached;

#[cached(ttl = "5m")]
pub async fn get_popular_items() -> Vec<Item> {
    // Expensive query cached for 5 minutes
}
```

### 5. Use Streaming for Large Responses

```rust
#[server]
pub async fn stream_data() -> impl IntoResponse {
    let stream = async_stream::stream! {
        for chunk in data.chunks(1024) {
            yield Ok::<_, Infallible>(chunk);
        }
    };
    Body::from_stream(stream)
}
```

---

## Memory Usage

### Baseline Memory

| State | RSS | Notes |
|-------|-----|-------|
| Startup | 12 MB | Binary + static assets |
| Idle | 18 MB | With connection pools |
| Under load | 45 MB | 1000 concurrent connections |
| Peak | 128 MB | During burst traffic |

### Comparison

| Framework | Startup Memory | Under Load |
|-----------|----------------|------------|
| **Nucleus** | **12 MB** | **45 MB** |
| Actix-web | 14 MB | 52 MB |
| Express.js | 45 MB | 180 MB |
| Next.js | 120 MB | 450 MB |
| Rails | 180 MB | 600 MB |

---

## Cold Start Performance

Critical for serverless and container deployments:

| Framework | Cold Start | First Request |
|-----------|------------|---------------|
| **Nucleus** | **8ms** | **12ms** |
| Actix-web | 12ms | 18ms |
| Fastify | 85ms | 120ms |
| Express | 150ms | 200ms |
| Next.js | 850ms | 1.2s |
| Rails | 2.5s | 3.2s |

---

## Continuous Benchmarking

We run automated benchmarks on every commit:

```yaml
# .github/workflows/benchmark.yml
name: Benchmark
on: [push]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo build --release
      - run: ./scripts/benchmark.sh
      - uses: benchmark-action/github-action-benchmark@v1
```

Results are tracked over time to catch performance regressions.
