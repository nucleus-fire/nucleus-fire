# Model Context Protocol (MCP) in Nucleus

Nucleus features a **first-class implementation** of the Model Context Protocol (MCP), an open standard that creates a universal language for AI models to interact with data and tools.

Instead of building custom integrations for every data source (Postgres, Linear, GitHub), you build **one MCP Server**, and any MCP-compliant client (Nucleus Agents, Claude Desktop, IDEs) can use it.

---

## 🏗️ Architecture

Nucleus implements the full JSON-RPC 2.0 lifecycle for MCP.

```mermaid
graph TD
    Client[MCP Client (e.g., Nucleus Agent)] <-->|Transport Layer| Server[MCP Server]
    
    subgraph "Nucleus Application"
        Server -->|Exposes| Tools[Tools (Functions)]
        Server -->|Exposes| Resources[Resources (Data)]
        Server -->|Exposes| Prompts[Prompts (Templates)]
    end
    
    Tools -.-> Database[(Database)]
    Tools -.-> API[External APIs]
    Resources -.-> Files[FileSystem]
```

### Core Concepts

1.  **Host**: The application that initiates the connection (e.g., your Nucleus Agent).
2.  **Server**: The application that provides context (tools, resources).
3.  **Transport**: The communication channel (Stdio, SSE, HTTP).

---

## 🔌 Building an MCP Server

You can turn any Nucleus application into an MCP server in minutes. This allows you to expose your internal business logic as tools for AI agents.

### Example: A "FileSystem" MCP Server

This server allows an agent to utilize file system operations safely.

```rust
use nucleus_std::mcp::{McpServer, Tool, Content};
use serde_json::json;

#[tokio::main]
async fn main() {
    // 1. Create the Server
    let server = McpServer::new("nucleus-fs-server", "1.0.0");
    
    // 2. Register 'list_files' Tool
    server.register_tool(
        Tool {
            name: "list_files".into(),
            description: "List files in the current directory".into(),
            input_schema: json!({ 
                "properties": { 
                    "path": { "type": "string", "default": "." } 
                } 
            })
        },
        |args| async move {
            let path = args["path"].as_str().unwrap_or(".");
            // Perform safe FS logic here...
            let files = vec!["Cargo.toml", "src/", "README.md"]; 
            
            // Return content as Text or Image
            Ok(vec![Content::Text { text: files.join("\n") }])
        }
    ).await;
    
    // 3. Register 'read_file' Tool
    server.register_tool(
        Tool {
            name: "read_file".into(),
            description: "Read contents of a file".into(),
            input_schema: json!({ 
                "required": ["path"],
                "properties": { "path": { "type": "string" } } 
            })
        },
        |args| async move {
            let path = args["path"].as_str().unwrap();
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read: {}", e))?;
                
            Ok(vec![Content::Text { text: content }])
        }
    ).await;

    println!("✅ MCP Server running on Stdio");
    
    // 4. Start the Transport Loop (Stdio for local usage)
    // In a real app, you would block here on the transport
    // nucleus_std::mcp::transport::StdioTransport::serve(server).await;
}
```

---

## 🤝 Protocol Details

Nucleus enforces strict adherence to **JSON-RPC 2.0** to ensure compatibility with the wider ecosystem.

### Message Flow
1.  **Client** sends `tools/list` request.
2.  **Server** responds with available tool definitions.
3.  **Client** sends `tools/call` request with `{ name: "read_file", arguments: { "path": "foo.txt" } }`.
4.  **Server** executes logic and returns `content` or `error`.

### Transport Layers
Nucleus abstracts the transport layer, allowing you to switch between:
*   **Stdio**: Great for local processes (Agent spawning a sub-process).
*   **SSE (Server-Sent Events)**: Great for remote web-based agents (Plans for v4.0).
*   **Custom**: Implement the `McpTransport` trait to run over WebSockets, NATS, or other channels.

---

## 🔮 Future Roadmap

*   **Resources Subscription**: Allow agents to "subscribe" to file changes (e.g., auto-reloading context when you save a file).
*   **Prompt Library**: Share standardized system prompts across your organization via the MCP Prompts primitive.
*   **Distributed MCP**: Connect a swarm of Nucleus Agents across a cluster using the NATS transport.
