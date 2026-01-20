# Dependency Management

Nucleus introduces unified dependency management. Managing dependencies should be uniform whether you're installing backend libraries or frontend components.

---

## The `nucleus install` Command

A single command for all dependencies:

```bash
nucleus install <package>
```

### Rust Crates

For Rust dependencies, Nucleus wraps `cargo add`:

```bash
# Install a crate
nucleus install serde
# → cargo add serde

# With features
nucleus install serde --features derive
# → cargo add serde --features derive

# Specific version
nucleus install tokio@1.35
# → cargo add tokio@1.35

# Dev dependency
nucleus install --dev insta
# → cargo add --dev insta
```

### Nucleus Modules

For Nucleus-specific modules (components, plugins), use a URL:

```bash
# GitHub repository
nucleus install https://github.com/example-org/navbar
# → Clones to src/vendor/navbar

# Shortened syntax
nucleus install example-org/navbar
# → Clones from github.com/example-org/navbar

# Specific branch/tag
nucleus install nucleus-ui/navbar@v2.0
```

---

## Vendor by Default Philosophy

Modern web development suffers from "dependency hell" due to:
- Remote registries going down
- Packages being unpublished
- Breaking changes in minor versions
- Supply chain attacks

Nucleus adopts **Vendor by Default** for frontend modules.

### How It Works

```
nucleus install nucleus-ui/navbar
```

Creates:
```
src/vendor/
└── navbar/
    ├── navbar.ncl
    ├── styles.css
    └── README.md
```

### Benefits

| Aspect | Traditional | Vendor by Default |
|--------|-------------|-------------------|
| **Availability** | Depends on registry | Always available |
| **Reproducibility** | Lock files help | Guaranteed |
| **Auditability** | Must fetch | Code is local |
| **Offline builds** | Usually fails | Always works |
| **CI/CD stability** | Can break | Never breaks |

### Trade-offs

| Concern | Solution |
|---------|----------|
| Repo size | Use `.gitattributes` for large assets |
| Updates | `nucleus update <module>` command |
| Security patches | `nucleus audit` checks for known issues |

---

## Auto-Discovery

The compiler automatically discovers vendored modules:

```
src/vendor/
└── navbar/
    └── navbar.ncl
```

Use immediately in any view:

```html
<!-- No import needed -->
<n:navbar title="My App" />
```

### Discovery Rules

1. Directory name = component name (lowercase)
2. Entry file = `{name}.ncl` or `index.ncl`
3. Scoped styles in `styles.css` or inline `<style scoped>`

---

## Updating Dependencies

### Update a Specific Module

```bash
nucleus update navbar
# Fetches latest from origin, updates src/vendor/navbar
```

### Update All Modules

```bash
nucleus update --all
```

### Check for Updates

```bash
nucleus outdated
# Shows modules with available updates
```

---

## Cargo.toml Management

Nucleus manages your `Cargo.toml` automatically:

```bash
nucleus install tokio sqlx serde
```

Results in:

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
serde = { version = "1.0", features = ["derive"] }
```

### Feature Detection

Nucleus automatically adds common features:

| Crate | Auto-added Features |
|-------|---------------------|
| `tokio` | `["full"]` |
| `serde` | `["derive"]` |
| `sqlx` | `["runtime-tokio", "sqlite"]` |
| `tracing` | `["std"]` |

Override with explicit features:

```bash
nucleus install tokio --features "rt-multi-thread,io-util"
```

---

## Removing Dependencies

### Remove Rust Crate

```bash
nucleus remove serde
# → cargo remove serde
```

### Remove Vendored Module

```bash
nucleus remove navbar
# → Deletes src/vendor/navbar
```

---

## Private Registries

For enterprise deployments with private registries:

```toml
# .cargo/config.toml
[registries.private]
index = "https://registry.example.com/index"
token = "Bearer ${CARGO_REGISTRY_TOKEN}"
```

```bash
nucleus install --registry private my-company-auth
```

---

## Security Auditing

Built-in security scanning:

```bash
# Scan for known vulnerabilities
nucleus audit

# Example output
  Crate     │ Version │ Advisory │ Severity
────────────┼─────────┼──────────┼──────────
  openssl   │ 0.10.54 │ CVE-2023-│ HIGH
            │         │ 1234     │
            │         │          │
  Suggested: Update to openssl@0.10.60
```

### Automated Audits

Add to CI:
```yaml
- name: Security Audit
  run: nucleus audit --deny-warnings
```

---

## Lock Files

Nucleus uses Cargo's existing lock file:

```
Cargo.lock  # Rust dependencies (committed)
```

For vendored modules, the code itself is the lock—no separate lock file needed.

---

## Creating Publishable Modules

To create a module others can install:

### 1. Create Module

```bash
nucleus new --module my-widget
```

### 2. Structure

```
my-widget/
├── my-widget.ncl
├── styles.css
├── README.md
└── package.json    # Optional metadata
```

### 3. Publish

Push to GitHub:
```bash
git init
git remote add origin https://github.com/yourname/my-widget
git push -u origin main
```

Others install with:
```bash
nucleus install yourname/my-widget
```

---

## Best Practices

### 1. Pin Major Versions

```bash
nucleus install serde@1
# Allows 1.x updates, blocks 2.0
```

### 2. Audit Before Release

```bash
nucleus audit && nucleus build --release
```

### 3. Use Workspaces for Monorepos

```toml
# Cargo.toml
[workspace]
members = [
    "apps/web",
    "apps/api",
    "packages/shared",
]
```

### 4. Document Vendored Dependencies

```
src/vendor/
├── README.md    # List all vendored modules
├── navbar/
└── footer/
```
