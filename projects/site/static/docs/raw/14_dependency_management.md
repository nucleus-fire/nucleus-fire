# Unified Dependency Management

Nucleus V3 introduces a radical simplification of dependency management. We believe that managing dependencies should be uniform, regardless of whether you are installing a backend library or a frontend component.

## The `nucleus install` Command

The unified command handles everything:

```bash
nucleus install <package>
```

### 1. Rust Crates
If you specify a crate name (e.g., `serde`, `tokio`, `reqwest`), Nucleus detects this is a Rust dependency and wraps `cargo add`.

```bash
nucleus install serde
# Result: Adds 'serde' to Cargo.toml and updates Cargo.lock
```

### 2. Nucleus Modules
If you specify a URL (Git repository), Nucleus treats it as a **Nucleus Module**.

```bash
nucleus install https://github.com/nucleus-ui/navbar
# Result: Clones the repository into src/vendor/navbar
```

## "Vendor by Default" Philosophy

Modern web development often suffers from "dependency hell" and fragility due to remote registries going down or packages being unassigned. Nucleus adopts a **Vendor by Default** approach for frontend modules.

*   **Offline First**: Modules are downloaded directly into your source tree (`src/vendor`).
*   **Reproducibility**: You commit your `src/vendor` folder. Your CI/CD pipeline never breaks because a remote package was deleted.
*   **Zero Config**: You do not need to edit a manifest file to use these modules.

## Auto-Discovery

The Nucleus Compiler (`ncc`) and Reactor (`atom`) are built to automatically discover modules in `src/vendor`.

If you install a module named `navbar`:
1.  It lives in `src/vendor/navbar/`.
2.  The compiler scans this directory.
3.  You can immediately use `<n:navbar>` in any of your views.

No `import` statements. No configuration mapping. It just works.
