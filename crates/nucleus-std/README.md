# Nucleus Standard Library (`nucleus-std`)

The standard library for the Nucleus framework, providing a comprehensive suite of utilities and modules for building modern distributed applications.

## Features

### 🧠 AI & Agents [NEW]
- **AI Agents**: Build autonomous agents with `nucleus_std::agent`. Features a robust ReAct loop, tool usage, and memory management.
- **MCP Protocol**: Full implementation of the Model Context Protocol (`nucleus_std::mcp`) for standardizing AI tool interactions.
- **Neural Client**: OpenAI-compatible LLM client (`nucleus_std::neural`).

### 🚀 Core Utilities
- **Fortress**: Rate limiting and security.
- **Photon**: Database ORM and query builder.
- **Pulse**: Background job processing.
- **Stream**: Real-time WebSocket communication.
- **Vault**: Financial transaction and ledger management.

## Installation

```toml
[dependencies]
nucleus-std = "0.1.0"
```

## Example: Simple Agent

```rust
use nucleus_std::agent::Agent;
use nucleus_std::neural::Neural;

let agent = Agent::new(Neural::new("key"));
let response = agent.run("Hello world").await?;
```
