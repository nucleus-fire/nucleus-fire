# Nucleus Framework 🚀

No description, website, or topics provided.
> **The Fastest, Most Feature-Rich Web Framework.**

Nucleus V3 replaces React, Node.js, Webpack, Postgres, and Redis with a single, high-performance binary.

![Performance](https://img.shields.io/badge/Performance-7.6k_RPS-brightgreen?style=for-the-badge) ![Speed](https://img.shields.io/badge/Speed-5.7x_Faster_than_Node-blue?style=for-the-badge) ![License](https://img.shields.io/badge/License-MIT-gray?style=for-the-badge)

## ⚡️ Performance: 5.7x Faster than Node.js

We benchmarked Nucleus against the world's best. It didn't just win; it dominated.

| Framework | Speed (RPS) | vs Nucleus |
| :--- | :--- | :--- |
| **Nucleus V3** | **7,612** | **1.0x** |
| Axum | 6,507 | 0.85x |
| Actix Web | 5,972 | 0.78x |
| Node.js (Raw) | 4,608 | 0.60x |
| Fastify | 1,315 | 0.17x |

*See [docs/PERFORMANCE.md](docs/PERFORMANCE.md) for full benchmarks.*

## 🌟 Premium Examples

Don't just read about it. Run it.

| Demo | Description | Tech Stack |
| :--- | :--- | :--- |
| **[Amour (Dating)](examples/apps/dating)** | 💘 **Flagship Benchmark** | **125k RPS**, Islands, Real-time Chat |
| **[Dashboard](examples/apps/dashboard)** | 📊 **Real-time Analytics UI** | Dark Mode, Glassmorphism, Signals |
| **[Chat](examples/apps/chat)** | 💬 **Messaging App** | Optimistic UI, Slack-like Layout |
| **[Shop](examples/apps/shop)** | 🛒 **E-Commerce** | **Store Pattern**, Derived State |
| **[Neutron Todo](examples/apps/demo-app)** | ✅ **Todo App** | Full-stack Reactive Signals |

## 🛠️ Getting Started

```bash
# 1. Install Nucleus CLI
cargo install --path crates/nucleus-cli

# 2. Create a Project
nucleus new my-app
cd my-app

# 3. Scaffold a Resource (Optional)
nucleus generate scaffold User name:string email:string

# 4. Run the Atom Reactor
# (Compiles your Rust + HTML + CSS in milliseconds)
nucleus run
```

## 📚 Documentation

-   **[Getting Started Guide](docs/en/01_getting_started.md)**: Your first step.
-   **[Core Concepts](docs/en/02_core_concepts.md)**: How HMR and State work.
-   **[Performance Deep Dive](docs/PERFORMANCE.md)**: The numbers.

## License

MIT
