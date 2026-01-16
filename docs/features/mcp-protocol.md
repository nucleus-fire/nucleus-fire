# Model Context Protocol (MCP) in Nucleus

Nucleus implements the Model Context Protocol (MCP), an open standard for connecting AI systems with data and tools. This implementation is strictly compliant with the JSON-RPC 2.0 specification.

## Core Concepts

### 1. Primitives
- **Tools**: Executable functions that can be called by an Agent (e.g., `get_weather`, `read_file`).
- **Resources**: Data sources that can be read by an Agent (e.g., files, API endpoints).
- **Prompts**: Reusable prompt templates.

### 2. Transport Layer
Nucleus provides a flexible transport abstraction `McpTransport`.

- **StreamTransport**: Generic async transport working over any `AsyncRead + AsyncWrite`.
  - **Stdio**: Use standard input/output for local process communication.
  - **TCP/Unix Sockets**: Connect over network sockets.

## Building an MCP Server

You can easily build an MCP server to expose tools from your Nucleus application.

```rust
use nucleus_std::mcp::{McpServer, Tool, Content};
use serde_json::json;

#[tokio::main]
async fn main() {
    let server = McpServer::new("my-server", "1.0.0");
    
    server.register_tool(
        Tool {
            name: "echo".into(),
            description: "Echoes back the input".into(),
            input_schema: json!({ "properties": { "msg": { "type": "string" } } })
        },
        |args| async move {
            let msg = args["msg"].as_str().unwrap_or("");
            Ok(vec![Content::Text { text: msg.to_string() }])
        }
    ).await;
    
    // Connect to transport (e.g., Stdio)
    // server.serve(transport).await;
}
```

## Protocol Details

### JSON-RPC 2.0
All messages adhere to the standard:
- **Requests**: `{ "jsonrpc": "2.0", "method": "...", "params": ..., "id": 1 }`
- **Responses**: `{ "jsonrpc": "2.0", "result": ..., "id": 1 }`
- **Errors**: `{ "jsonrpc": "2.0", "error": { "code": -32xxx, "message": "..." }, "id": 1 }`

### Client Capabilities
The Nucleus `McpClient` supports:
- Tool discovery (`tools/list`)
- Tool execution (`tools/call`)
- Dynamic sampling (optional)
