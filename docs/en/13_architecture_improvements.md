# Architecture Overview

Understanding the internal architecture of Nucleus helps you build better applications and troubleshoot issues.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Nucleus Framework                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   CLI       │  │   NCC       │  │   LSP               │  │
│  │  (nucleus)  │  │  (Compiler) │  │  (Language Server)  │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                     │             │
│         ▼                ▼                     ▼             │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                    Atom Reactor                       │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────────┐  │   │
│  │  │   Axum     │  │   Tokio    │  │   Tower        │  │   │
│  │  │   (HTTP)   │  │  (Runtime) │  │  (Middleware)  │  │   │
│  │  └────────────┘  └────────────┘  └────────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
│                              │                               │
│         ┌────────────────────┼────────────────────┐         │
│         ▼                    ▼                    ▼         │
│  ┌────────────┐      ┌────────────┐       ┌────────────┐   │
│  │  Photon    │      │  Neutron   │       │  Fortress  │   │
│  │  (Database)│      │  (State)   │       │  (Security)│   │
│  └────────────┘      └────────────┘       └────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## Component Deep Dive

### 1. CLI (`nucleus`)

The command-line interface orchestrates development and deployment.

**Key Responsibilities:**
- Project scaffolding (`nucleus new`)
- Development server with HMR (`nucleus dev`)
- Production builds (`nucleus build`)
- Database migrations (`nucleus db`)
- Code generation (`nucleus generate`)

**Implementation:**
- Written in Rust using `clap` for argument parsing
- Uses `tokio` for async operations
- Parallel file processing with `rayon`

### 2. Compiler (`ncc`)

The Nucleus Component Compiler transforms `.ncl` files into Rust code.

**Pipeline:**
```
NCL Source → Lexer → Parser → AST → Analyzer → CodeGen → Rust
```

**Key Features:**
- Zero-copy parsing with `nom`
- Type inference for template expressions
- Dead code elimination
- Compile-time route generation

**AST Nodes:**
```rust
enum NclNode {
    View { title: String, layout: Option<String>, children: Vec<NclNode> },
    Component { name: String, props: Vec<Prop>, children: Vec<NclNode> },
    Element { tag: String, attrs: Vec<Attr>, children: Vec<NclNode> },
    Text(String),
    Interpolation(Expression),
    ForLoop { var: String, iterable: Expression, body: Vec<NclNode> },
    Conditional { condition: Expression, then: Vec<NclNode>, else_: Option<Vec<NclNode>> },
    Action(RustCode),
    Island { hydration: HydrationType, children: Vec<NclNode> },
}
```

### 3. Atom Reactor (Runtime)

The HTTP server and request handler.

**Built on:**
- **Axum**: HTTP routing and handler composition
- **Tokio**: Async runtime (multi-threaded by default)
- **Tower**: Middleware stack
- **Hyper**: HTTP/1.1 and HTTP/2

**Request Flow:**
```
Request
  │
  ▼
┌──────────────────┐
│   Tower Stack    │
│  ┌────────────┐  │
│  │  Logging   │  │
│  ├────────────┤  │
│  │  Security  │  │
│  ├────────────┤  │
│  │  Session   │  │
│  ├────────────┤  │
│  │  CORS      │  │
│  └────────────┘  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   Router         │◄─── Static route table
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   Handler        │◄─── Generated from NCL
└────────┬─────────┘
         │
         ▼
     Response
```

---

## Standard Library Architecture

### Module Organization

```
nucleus-std/
├── lib.rs              # Public exports
├── config.rs           # Configuration
├── errors.rs           # Error types
│
├── photon/             # Database
│   ├── mod.rs
│   ├── query.rs
│   ├── migrations.rs
│   └── relations.rs
│
├── fortress.rs         # Security
├── neutron.rs          # State management
├── session.rs          # Sessions
├── cache.rs            # Caching
├── pulse.rs            # Job queue
├── stream.rs           # WebSockets
├── neural.rs           # AI/LLM
└── ...
```

### Dependency Graph

```
                    config
                      │
         ┌───────────┴───────────┐
         │                       │
      photon                 fortress
         │                       │
    ┌────┴────┐             ┌────┴────┐
    │         │             │         │
  pulse    neutron      session    oauth
    │         │             │
    └────┬────┘             │
         │                  │
      stream ───────────────┘
```

---

## Memory Model

### Per-Request Arena

Each request gets a dedicated memory arena:

```rust
// Conceptual model
async fn handle_request(req: Request) -> Response {
    let arena = Arena::new();  // Request-local allocator
    
    // All allocations use the arena
    let user = arena.alloc(User::load()?);
    let posts = arena.alloc(Post::for_user(user)?);
    let html = arena.alloc(render(posts)?);
    
    let response = Response::new(html);
    
    // Arena freed atomically - no GC pauses
    drop(arena);
    
    response
}
```

### Benefits:
- No garbage collection pauses
- Predictable latency
- Efficient memory reuse

---

## Compilation Model

### AOT Compilation

Nucleus uses Ahead-of-Time compilation:

```
Development                          Production
    │                                    │
    ▼                                    ▼
┌─────────┐                        ┌─────────┐
│   NCL   │                        │   NCL   │
└────┬────┘                        └────┬────┘
     │ ncc                              │ ncc
     ▼                                  ▼
┌─────────┐                        ┌─────────┐
│  Rust   │                        │  Rust   │
└────┬────┘                        └────┬────┘
     │ rustc (debug)                    │ rustc (release)
     ▼                                  ▼
┌─────────┐                        ┌─────────┐
│ Binary  │                        │ Binary  │
│ (debug) │                        │ (opt)   │
└─────────┘                        └─────────┘
   12 MB                              8 MB
```

### Interpreter Mode (Dev)

In development, changes bypass full compilation:

```
File Save → AST Diff → Patch Runtime → HMR Signal → Browser Update
```

---

## Concurrency Model

### Tokio Runtime

Default configuration:
```rust
// Auto-configured based on CPU cores
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    .enable_all()
    .build()?;
```

### Task Hierarchy

```
Main Thread
    │
    ├── HTTP Acceptor
    │       │
    │       ├── Request Handler (spawned)
    │       ├── Request Handler (spawned)
    │       └── Request Handler (spawned)
    │
    ├── Background Jobs
    │       │
    │       ├── Job Worker 1
    │       └── Job Worker 2
    │
    └── Scheduler
            │
            └── Cron Tasks
```

---

## Extension Points

### Custom Middleware

```rust
// src/middleware.rs
pub fn custom_middleware() -> impl Layer {
    ServiceBuilder::new()
        .layer(MyCustomLayer)
        .layer(AnotherLayer)
}
```

### Custom Extractors

```rust
pub struct CurrentUser(pub User);

impl<S> FromRequestParts<S> for CurrentUser {
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Response> {
        // Extract user from session
    }
}
```

### Plugin System (Future)

```rust
// Planned for v4
nucleus::plugin!({
    name: "my-plugin",
    hooks: {
        on_request: |req| { ... },
        on_response: |res| { ... },
    }
});
```

---

## Performance Optimizations

| Optimization | Implementation |
|--------------|----------------|
| Static routing | Compile-time `phf` maps |
| Asset embedding | `include_bytes!` for small files |
| Connection pooling | `sqlx` with configurable pool |
| Response compression | Brotli/gzip via tower |
| HTTP/2 multiplexing | Hyper automatic upgrade |
| Memory allocator | mimalloc by default |
| LTO | Fat LTO in release builds |
