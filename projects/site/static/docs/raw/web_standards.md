# Web Standards & Compliance

Nucleus is built to be "Standard by Default." We enforce strict adherence to HTTP and web standards to ensure your application performs predictably in any environment (CDNs, proxies, browsers).

---

## HTTP Compliance

### RFC 7231: HTTP/1.1 Semantics

#### `Date` Header
Every response served by Nucleus includes a strictly formatted `Date` header:
```
Date: Tue, 15 Nov 1994 08:12:31 GMT
```
- **Implementation**: High-performance cached clock updates every second to minimize per-request overhead while maintaining compliance.

#### `Content-Type`
Nucleus strictly enforces `Content-Type` headers:

| Route Type | Content-Type |
|------------|--------------|
| Benchmark routes | `text/plain` |
| JSON APIs | `application/json; charset=utf-8` |
| Views (HTML) | `text/html; charset=utf-8` |
| Static files | Auto-detected from extension |

This prevents browser MIME sniffing vulnerabilities and ensures correct client parsing.

#### Method Handling
- **GET**: Safe, idempotent reads
- **POST**: State-changing actions, form submissions
- **PUT**: Full resource replacement (idempotent)
- **PATCH**: Partial updates
- **DELETE**: Resource removal (idempotent)
- **HEAD**: Metadata only (auto-generated from GET)
- **OPTIONS**: CORS preflight (auto-handled)

---

## Caching Headers

### Cache-Control
Nucleus automatically sets appropriate caching:

```
# Static assets (immutable content-hashed files)
Cache-Control: public, max-age=31536000, immutable

# API responses (no caching by default)
Cache-Control: no-store

# Dynamic pages
Cache-Control: private, no-cache
```

### ETag Support
Entity tags for efficient validation:
```rust
// Automatic ETag generation
let response = Response::new(body)
    .with_etag();  // Generates hash-based ETag

// Manual ETag
response.set_header("ETag", format!("\"{}\"", version));
```

### Last-Modified
For file-based resources:
```
Last-Modified: Wed, 21 Oct 2025 07:28:00 GMT
```

### Conditional Requests
Nucleus handles `If-None-Match` and `If-Modified-Since` automatically, returning `304 Not Modified` when appropriate.

---

## Compression

### Supported Algorithms

| Algorithm | Priority | Use Case |
|-----------|----------|----------|
| Brotli (`br`) | 1 (preferred) | Best compression ratio |
| Gzip (`gzip`) | 2 | Universal support |
| Deflate | 3 | Legacy fallback |

### Content Negotiation
```
Accept-Encoding: gzip, deflate, br

# Response
Content-Encoding: br
Vary: Accept-Encoding
```

Nucleus automatically compresses responses based on:
- `Accept-Encoding` header
- Content type (text, JSON, HTML)
- Response size (minimum threshold: 1KB)

---

## Security Headers

The **Fortress** middleware injects security headers automatically:

### Content Security Policy
```
Content-Security-Policy: 
  default-src 'self';
  script-src 'self' 'unsafe-inline';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: https:;
  connect-src 'self' wss:;
  frame-ancestors 'none'
```

### Other Security Headers
| Header | Value | Purpose |
|--------|-------|---------|
| `X-Frame-Options` | `DENY` | Prevents clickjacking |
| `X-Content-Type-Options` | `nosniff` | Prevents MIME sniffing |
| `X-XSS-Protection` | `1; mode=block` | XSS filter (legacy browsers) |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Controls referrer info |
| `Permissions-Policy` | `camera=(), microphone=()` | Restricts browser features |

### Strict Transport Security
For HTTPS deployments:
```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

---

## CORS (RFC 6454)

### Automatic Preflight Handling
```rust
// In nucleus.config
[cors]
allowed_origins = ["https://example.com", "https://app.example.com"]
allowed_methods = ["GET", "POST", "PUT", "DELETE"]
allowed_headers = ["Content-Type", "Authorization"]
max_age = 86400
credentials = true
```

### Response Headers
```
Access-Control-Allow-Origin: https://example.com
Access-Control-Allow-Methods: GET, POST, PUT, DELETE
Access-Control-Allow-Headers: Content-Type, Authorization
Access-Control-Allow-Credentials: true
Access-Control-Max-Age: 86400
```

---

## Cookie Handling (RFC 6265)

### Secure Cookie Defaults
```rust
Session::new()
    .cookie_name("nucleus_session")
    .http_only(true)      // Not accessible via JavaScript
    .secure(true)         // HTTPS only
    .same_site(SameSite::Lax)  // CSRF protection
    .max_age(Duration::hours(24))
```

### SameSite Options
| Value | Behavior |
|-------|----------|
| `Strict` | Cookie sent only to same site |
| `Lax` | Sent with top-level navigations |
| `None` | Sent cross-site (requires `Secure`) |

---

## WebSocket Protocol (RFC 6455)

### Handshake Compliance
```
GET /ws HTTP/1.1
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
Sec-WebSocket-Version: 13
```

### Frame Types Supported
- Text frames (UTF-8)
- Binary frames
- Ping/Pong (keep-alive)
- Close frames (graceful shutdown)

---

## Range Requests (RFC 7233)

Nucleus supports partial content for large files:

```
# Request
Range: bytes=0-1023

# Response
HTTP/1.1 206 Partial Content
Content-Range: bytes 0-1023/10240
Content-Length: 1024
```

Useful for:
- Video streaming
- Large file downloads
- Resume interrupted transfers

---

## Content Negotiation

### Accept Header
```rust
// Automatic format selection
async fn get_user(accept: Accept) -> Response {
    let user = User::find(1).await?;
    
    match accept.best() {
        "application/json" => Json(user).into_response(),
        "text/html" => Html(render_user(user)).into_response(),
        _ => Json(user).into_response(),
    }
}
```

### Language Negotiation
```
Accept-Language: en-US,en;q=0.9,es;q=0.8
```
Integrated with Polyglot for i18n responses.

---

## Framework Signature

By default, Nucleus proudly signs responses:
```
X-Powered-By: Nucleus/3.5
Server: Nucleus
```

To disable (for security through obscurity):
```toml
[server]
omit_signature = true
hide_server_header = true
```

---

## HTTP/2 Support

Nucleus supports HTTP/2 when running behind a TLS-terminating proxy:

| Feature | Support |
|---------|---------|
| Multiplexing | ✅ |
| Header compression (HPACK) | ✅ |
| Server push | ❌ (deprecated) |
| Stream prioritization | ✅ |

---

## Compliance Checklist

| Standard | Status |
|----------|--------|
| RFC 7230 (HTTP/1.1 Message Syntax) | ✅ |
| RFC 7231 (HTTP/1.1 Semantics) | ✅ |
| RFC 7232 (Conditional Requests) | ✅ |
| RFC 7233 (Range Requests) | ✅ |
| RFC 7234 (Caching) | ✅ |
| RFC 7235 (Authentication) | ✅ |
| RFC 6265 (Cookies) | ✅ |
| RFC 6454 (Origin) | ✅ |
| RFC 6455 (WebSocket) | ✅ |
| RFC 7540 (HTTP/2) | ✅ (via proxy) |
