# Nucleus Framework: Master Feature List

Nucleus is a high-performance, batteris-included framework designed to replace the Node.js/Next.js ecosystem with a single, optimized Rust binary.

## I. The Engine (Atom)
The core runtime and compilation strategy.
*   **AOT Compilation**: Compiles your entire application (views, logic, assets) into a single, dependency-free binary file.
*   **Zero-Allocation Routing**: Static routing tables generated at compile time.
*   **Performance**: **83,000+ Requests Per Second** (Benchmark verified). 30x faster than Python, 6x faster than Node.js.
*   **Hot Swap (Reactor)**: Instant reloading during development (Interpreter Mode).
 at compile time.
*   **Atomic Style Engine**: Zero-runtime CSS generation.
*   **Neutron State Engine**: Fine-grained reactivity with `Signal<T>` and `Store` pattern. No Virtual DOM diffing.

## 2. The Toolkit (`nucleus-std`)
A comprehensive standard library eliminating the need for external crates.

| Module | Feature | Capability |
| :--- | :--- | :--- |
| **Photon** | Database | Zero-config SQL wrapper (SQLite/SQLx). |
| **Neural** | **AI / Sentience** | Built-in LLM Client (OpenAI-compatible). |
| **Browser** | **Automation** | Built-in Headless Chrome control. |
| **Fortress** | Security | Argon2 Hashing, HMAC Tokens, RBAC, Rate Limiting with RFC headers. |
| **Pulse** | **Job Queue** | Persistent background jobs with retries & dead letter queue. |
| **Scheduler** | **Cron/Tasks** | Recurring tasks with cron expressions (hourly, daily, etc). |
| **Session** | **Sessions** | Cookie sessions, flash messages, CSRF protection. |
| **Cache** | Caching | In-memory cache with TTL & pattern invalidation. |
| **Redis Cache** | **Distributed Cache** | Redis-backed caching for horizontal scaling. |
| **Stream** | **WebSocket Rooms** | Real-time rooms, broadcast, presence tracking. |
| **Health** | **Monitoring** | Liveness/readiness probes, component checks. |
| **Pool Monitor** | **DB Visibility** | Connection pool stats, health, slow query detection. |
| **Upload** | File Uploads | Multipart handling, validation, secure storage. |
| **Logging** | Structured Logs | tracing integration with JSON/pretty output. |
| **Sonar** | Search | Embedded Full-text Search (BM25). |
| **Vault** | Finance | Double-entry ledger & Decimal types. |
| **Polyglot** | i18n | Simple localization engine. |
| **Gondola** | Sync | Merkle-tree based offline synchronization. |
| **Lens** | Media | Image processing pipeline. |
| **Postman** | Email | Transactional email sender. |
| **Beacon** | Analytics | Built-in event tracking & analytics. |
| **OAuth** | Social Login | Google, GitHub, Discord, Twitter SSO. |

## 3. Web Capabilities (Next.js Parity)
*   **Dynamic Routing**: File-system based routing with parameter support (`views/users/[id].ncl` → `/users/:id`).
*   **Image Optimization**: `<n:image src="...">` compiles to optimized `<img>` tags with lazy-loading and async decoding.
*   **Smart Links**: `<n:link>` for client-side prefetching.
*   **Rich Models**: Define data structures, methods, and attributes directly in `.ncl` files.

## 4. Developer Experience (Zenith)
*   **Nucleus CLI**: unified tool for `new`, `run`, `build`, `test`, and `deploy`.
*   **Auto-Dockerization**: `nucleus deploy` automatically generates optimized multi-stage `Dockerfile` and `.dockerignore`.
*   **Triumvirate**:
    *   **Database Migrations**: `nucleus db new` and `nucleus db up` built-in.
    *   **Middleware**: Global Request Middleware support via `src/middleware.rs`.
    *   **Interactivity**: `on:click` and client-side event binding (No Virtual DOM).
    *   **Language Server**: (Planned) LSP for .ncl files.
*   **Hyper-Diagnostics**:
    *   **Beautiful Errors**: `miette` integration for rustc-style error reporting.
    *   **Fuzzy Matching**: "Did you mean `username`?" suggestions for typos.
    *   **Schema Validation**: Compile-time checking of data access.

## 5. Future Preview (Singularity V3)
*   **DAX (Data Access Expressions)**: Concise, shape-based query syntax (`User { id, posts { title } }`) that compiles to SQL JOINs.
