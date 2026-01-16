# Web Standards & Compliance

Nucleus is built to be "Standard by Default." We enforce strict adherence to HTTP standards to ensure your application performs predictably in any environment (CDNs, Proxies, Browsers).

## HTTP Compliance (RFC 7231)

### `Date` Header
Every response served by Nucleus (including dynamic API routes) includes a strictly formatted `Date` header (e.g., `Tue, 15 Nov 1994 08:12:31 GMT`).
- **Implementation**: We use a high-performance cached clock that updates every second to minimize per-request overhead while maintaining compliance.

### `Content-Type`
Nucleus strictly enforces `Content-Type` headers.
- **Benchmark Routes**: `text/plain`
- **JSON APIs**: `application/json`
- **Views**: `text/html`

This prevents browser MIMO sniffing vulnerabilities and ensures correct client parsing.

## Security Headers
The **Fortress** middleware injects battle-hardened security headers into every response automatically:

- `Content-Security-Policy`:
  - `script-src`: `'self' 'unsafe-inline'` (Allows hydration/HMR)
  - `style-src`: `'self' 'unsafe-inline'` (Allows dynamic bindings/animations)
- `X-Frame-Options`: `DENY` (Prevents Clickjacking).
- `X-Content-Type-Options`: `nosniff`.
- `Referrer-Policy`: `strict-origin-when-cross-origin`.

## Framework Signature
By default, Nucleus proudly signs responses with:
`X-Powered-By: Nucleus`

To disable this (e.g., for security through obscurity), update your config:

```toml
[server]
omit_signature = true
```
