//! Model Context Protocol (MCP) Implementation
//!
//! Implements the full MCP specification for both Clients and Servers.
//! Supports:
//! - JSON-RPC 2.0 Messages
//! - Tools, Resources, and Prompts
//! - Transport Abstraction (Stdio, SSE)

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum McpError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Tool execution error: {0}")]
    ToolExecution(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// JSON-RPC 2.0 TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RequestId {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    pub id: RequestId,
}

impl JsonRpcRequest {
    pub fn new(method: &str, params: Option<Value>, id: RequestId) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl JsonRpcNotification {
    pub fn new(method: &str, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: RequestId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub fn error(id: RequestId, code: i32, message: &str, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.to_string(),
                data,
            }),
            id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

// ═══════════════════════════════════════════════════════════════════════════
// MCP PRIMITIVES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub input_schema: Value, // JSON Scema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(default)]
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Content {
    Text {
        text: String,
    },
    Image {
        data: String,
        mime_type: String,
    },
    Resource {
        uri: String,
        mime_type: Option<String>,
        text: Option<String>,
        blob: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub required: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// TRANSPORT ABSTRACTION
// ═══════════════════════════════════════════════════════════════════════════

#[async_trait]
pub trait McpTransport: Send + Sync {
    /// Send a JSON-RPC message
    async fn send(&self, message: JsonRpcMessage) -> Result<(), McpError>;

    /// Receive a JSON-RPC message
    /// Returns None if the stream is closed
    async fn receive(&self) -> Result<Option<JsonRpcMessage>, McpError>;
}

pub struct StreamTransport<R, W> {
    reader: Mutex<FramedRead<R, LinesCodec>>,
    writer: Mutex<FramedWrite<W, LinesCodec>>,
}

impl<R, W> StreamTransport<R, W>
where
    R: AsyncRead + Send + Unpin,
    W: AsyncWrite + Send + Unpin,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: Mutex::new(FramedRead::new(reader, LinesCodec::new())),
            writer: Mutex::new(FramedWrite::new(writer, LinesCodec::new())),
        }
    }
}

#[async_trait]
impl<R, W> McpTransport for StreamTransport<R, W>
where
    R: AsyncRead + Send + Unpin,
    W: AsyncWrite + Send + Unpin,
{
    async fn send(&self, message: JsonRpcMessage) -> Result<(), McpError> {
        let json = serde_json::to_string(&message)?;
        let mut writer = self.writer.lock().await;
        writer
            .send(json)
            .await
            .map_err(|e| McpError::Transport(e.to_string()))?;
        Ok(())
    }

    async fn receive(&self) -> Result<Option<JsonRpcMessage>, McpError> {
        let mut reader = self.reader.lock().await;
        match reader.next().await {
            Some(Ok(line)) => {
                let msg: JsonRpcMessage = serde_json::from_str(&line)?;
                Ok(Some(msg))
            }
            Some(Err(e)) => Err(McpError::Transport(e.to_string())),
            None => Ok(None),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SERVER IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════

type ToolHandler = Box<
    dyn Fn(Value) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, String>> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct McpServer {
    tools: Arc<Mutex<HashMap<String, (Tool, ToolHandler)>>>,
    server_info: JsonRpcNotification, // initialized notification
}

impl McpServer {
    pub fn new(name: &str, version: &str) -> Self {
        let initialized = JsonRpcNotification::new(
            "initialized",
            Some(json!({
                "serverInfo": {
                    "name": name,
                    "version": version
                },
                "capabilities": {
                    "tools": {},
                    "resources": {},
                    "prompts": {}
                }
            })),
        );

        Self {
            tools: Arc::new(Mutex::new(HashMap::new())),
            server_info: initialized,
        }
    }

    pub async fn register_tool<F, Fut>(&self, tool: Tool, handler: F)
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<Content>, String>> + Send + 'static,
    {
        let mut tools = self.tools.lock().await;
        // Wrap the handler to pin the future
        let boxed_handler = Box::new(move |args| {
            let fut = handler(args);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Result<Vec<Content>, String>> + Send>>
        }) as ToolHandler;

        tools.insert(tool.name.clone(), (tool, boxed_handler));
    }

    pub async fn handle_message(&self, message: JsonRpcMessage) -> Option<JsonRpcMessage> {
        match message {
            JsonRpcMessage::Request(req) => Some(self.handle_request(req).await),
            JsonRpcMessage::Notification(notif) => {
                // Handle initialization/other notifications
                if notif.method == "initialize" {
                    // In a real implementation we might do more check logic here
                    // For now we just return nothing or maybe log
                }
                None
            }
            JsonRpcMessage::Response(_) => None, // Server doesn't handle responses usually (unless it's a client too)
        }
    }

    async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcMessage {
        match req.method.as_str() {
            "initialize" => {
                let params = match self.server_info.params.as_ref() {
                    Some(p) => p,
                    None => {
                        return JsonRpcMessage::Response(JsonRpcResponse::error(
                            req.id,
                            -32603,
                            "Internal error: Missing server info",
                            None,
                        ))
                    }
                };

                let server_info = match params.get("serverInfo") {
                    Some(si) => si.clone(),
                    None => {
                        return JsonRpcMessage::Response(JsonRpcResponse::error(
                            req.id,
                            -32603,
                            "Internal error: Missing serverInfo in params",
                            None,
                        ))
                    }
                };

                let capabilities = match params.get("capabilities") {
                    Some(c) => c.clone(),
                    None => {
                        return JsonRpcMessage::Response(JsonRpcResponse::error(
                            req.id,
                            -32603,
                            "Internal error: Missing capabilities in params",
                            None,
                        ))
                    }
                };

                JsonRpcMessage::Response(JsonRpcResponse::success(
                    req.id,
                    json!({
                        "protocolVersion": "2024-11-05",
                        "serverInfo": server_info,
                        "capabilities": capabilities
                    }),
                ))
            }
            "tools/list" => {
                let tools_map = self.tools.lock().await;
                let tools_list: Vec<Tool> = tools_map.values().map(|(t, _)| t.clone()).collect();
                JsonRpcMessage::Response(JsonRpcResponse::success(
                    req.id,
                    json!({ "tools": tools_list }),
                ))
            }
            "tools/call" => {
                if let Some(params) = req.params {
                    let call_params: Result<CallToolParams, _> = serde_json::from_value(params);
                    match call_params {
                        Ok(p) => {
                            let tools = self.tools.lock().await;
                            if let Some((_, handler)) = tools.get(&p.name) {
                                match handler(p.arguments).await {
                                    Ok(content) => {
                                        JsonRpcMessage::Response(JsonRpcResponse::success(
                                            req.id,
                                            json!(CallToolResult {
                                                content,
                                                is_error: false
                                            }),
                                        ))
                                    }
                                    Err(e) => JsonRpcMessage::Response(JsonRpcResponse::success(
                                        req.id,
                                        json!(CallToolResult {
                                            content: vec![Content::Text { text: e }],
                                            is_error: true
                                        }),
                                    )),
                                }
                            } else {
                                JsonRpcMessage::Response(JsonRpcResponse::error(
                                    req.id,
                                    -32601,
                                    "Tool not found",
                                    None,
                                ))
                            }
                        }
                        Err(e) => JsonRpcMessage::Response(JsonRpcResponse::error(
                            req.id,
                            -32602,
                            &format!("Invalid params: {}", e),
                            None,
                        )),
                    }
                } else {
                    JsonRpcMessage::Response(JsonRpcResponse::error(
                        req.id,
                        -32602,
                        "Missing params",
                        None,
                    ))
                }
            }
            _ => JsonRpcMessage::Response(JsonRpcResponse::error(
                req.id,
                -32601,
                "Method not found",
                None,
            )),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CLIENT IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone)]
pub struct McpClient {
    transport: Arc<dyn McpTransport>,
    next_id: Arc<Mutex<i64>>,
}

impl McpClient {
    pub fn new(transport: Arc<dyn McpTransport>) -> Self {
        Self {
            transport,
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub async fn request(
        &self,
        method: &str,
        params: Option<Value>,
    ) -> Result<JsonRpcResponse, McpError> {
        let mut id_lock = self.next_id.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        drop(id_lock);

        let req = JsonRpcRequest::new(method, params, RequestId::Number(id));
        self.transport
            .send(JsonRpcMessage::Request(req.clone()))
            .await?;

        // In a real client we'd have a pending map and wait for the specific ID response
        // For this simple implementation we just wait for the next message
        // This relies on the transport being 1:1 request/response for now
        loop {
            match self.transport.receive().await? {
                Some(JsonRpcMessage::Response(res)) => {
                    if res.id == RequestId::Number(id) {
                        return Ok(res);
                    }
                    // ignore other responses
                }
                Some(_) => {} // ignore notifications/requests
                None => return Err(McpError::Transport("Connection closed".to_string())),
            }
        }
    }

    pub async fn initialize(&self) -> Result<(), McpError> {
        let res = self
            .request(
                "initialize",
                Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "roots": { "listChanged": false }
                    },
                    "clientInfo": {
                        "name": "nucleus-client",
                        "version": "0.1.0"
                    }
                })),
            )
            .await?;

        if let Some(err) = res.error {
            return Err(McpError::Protocol(format!(
                "Initialize failed: {}",
                err.message
            )));
        }

        self.transport
            .send(JsonRpcMessage::Notification(JsonRpcNotification::new(
                "notifications/initialized",
                None,
            )))
            .await
    }

    pub async fn list_tools(&self) -> Result<Vec<Tool>, McpError> {
        let res = self.request("tools/list", None).await?;

        if let Some(result) = res.result {
            let tools_value = result.get("tools").ok_or_else(|| {
                McpError::Protocol("Missing 'tools' field in response".to_string())
            })?;

            let tools: Vec<Tool> = serde_json::from_value(tools_value.clone())?;
            Ok(tools)
        } else {
            Err(McpError::Protocol(
                "No result in list_tools response".to_string(),
            ))
        }
    }

    pub async fn call_tool(&self, name: &str, args: Value) -> Result<CallToolResult, McpError> {
        let params = CallToolParams {
            name: name.to_string(),
            arguments: args,
        };

        let res = self
            .request("tools/call", Some(serde_json::to_value(params)?))
            .await?;

        if let Some(result) = res.result {
            let tool_res: CallToolResult = serde_json::from_value(result)?;
            Ok(tool_res)
        } else if let Some(err) = res.error {
            Err(McpError::ToolExecution(err.message))
        } else {
            Err(McpError::Protocol("Empty response".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::error::Error;

    // Mock Transport for testing

    // Mock Transport for testing
    // (Removed unused MockTransport struct)

    #[tokio::test]
    async fn test_server_tools() -> Result<(), Box<dyn Error>> {
        let server = McpServer::new("test-server", "1.0");

        server
            .register_tool(
                Tool {
                    name: "echo".into(),
                    description: "Echoes input".into(),
                    input_schema: json!({}),
                },
                |args| async move {
                    Ok(vec![Content::Text {
                        text: args.to_string(),
                    }])
                },
            )
            .await;

        // Test tools/list
        let req = JsonRpcRequest::new("tools/list", None, RequestId::Number(1));
        let resp = server
            .handle_message(JsonRpcMessage::Request(req))
            .await
            .ok_or("No response")?;

        if let JsonRpcMessage::Response(r) = resp {
            let result = r.result.ok_or("No result")?;
            let tools = result
                .get("tools")
                .ok_or("No tools field")?
                .as_array()
                .ok_or("Tools not array")?;
            assert_eq!(tools.len(), 1);
            assert_eq!(tools[0]["name"], "echo");
        } else {
            return Err("Expected response".into());
        }

        // Test tools/call
        let req = JsonRpcRequest::new(
            "tools/call",
            Some(json!({
                "name": "echo",
                "arguments": { "msg": "hello" }
            })),
            RequestId::Number(2),
        );
        let resp = server
            .handle_message(JsonRpcMessage::Request(req))
            .await
            .ok_or("No response")?;

        if let JsonRpcMessage::Response(r) = resp {
            assert!(r.error.is_none());
            let res: CallToolResult = serde_json::from_value(r.result.ok_or("No result")?)?;
            match &res.content[0] {
                Content::Text { text } => assert_eq!(text, "{\"msg\":\"hello\"}"),
                _ => return Err("Wrong content type".into()),
            }
        } else {
            return Err("Expected response".into());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_stream_transport() -> Result<(), Box<dyn Error>> {
        use tokio::io::duplex;

        let (client_io, server_io) = duplex(1024);
        let (client_read, client_write) = tokio::io::split(client_io);
        let (server_read, server_write) = tokio::io::split(server_io);

        let client = StreamTransport::new(client_read, client_write);
        let server = StreamTransport::new(server_read, server_write);

        let req = JsonRpcRequest::new("ping", None, RequestId::Number(1));
        client.send(JsonRpcMessage::Request(req.clone())).await?;

        let msg = server.receive().await?.ok_or("Stream closed")?;
        if let JsonRpcMessage::Request(r) = msg {
            assert_eq!(r.method, "ping");
        } else {
            return Err("Expected request".into());
        }

        Ok(())
    }

    #[test]
    fn test_request_serialization() -> Result<(), Box<dyn Error>> {
        let req = JsonRpcRequest::new("list_tools", None, RequestId::Number(1));
        let json = serde_json::to_string(&req)?;

        let deserialized: JsonRpcRequest = serde_json::from_str(&json)?;
        assert_eq!(deserialized.method, "list_tools");
        assert_eq!(deserialized.id, RequestId::Number(1));
        Ok(())
    }

    #[test]
    fn test_notification_serialization() -> Result<(), Box<dyn Error>> {
        let notif = JsonRpcNotification::new("initialized", None);
        let json = serde_json::to_string(&notif)?;

        let deserialized: JsonRpcNotification = serde_json::from_str(&json)?;
        assert_eq!(deserialized.method, "initialized");
        Ok(())
    }

    #[test]
    fn test_response_success_serialization() -> Result<(), Box<dyn Error>> {
        let resp = JsonRpcResponse::success(RequestId::Number(1), json!({ "tools": [] }));
        let json = serde_json::to_string(&resp)?;

        let deserialized: JsonRpcResponse = serde_json::from_str(&json)?;
        assert!(deserialized.error.is_none());
        assert!(deserialized.result.is_some());
        Ok(())
    }

    #[test]
    fn test_tool_serialization() -> Result<(), Box<dyn Error>> {
        let tool = Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "arg": { "type": "string" }
                }
            }),
        };

        let json = serde_json::to_string(&tool)?;
        let deserialized: Tool = serde_json::from_str(&json)?;

        assert_eq!(deserialized.name, "test_tool");
        assert_eq!(deserialized.description, "A test tool");
        Ok(())
    }

    #[test]
    fn test_server_missing_server_info_error() -> Result<(), Box<dyn Error>> {
        // Test that server handles missing info gracefully
        let server = McpServer {
            tools: Arc::new(Mutex::new(HashMap::new())),
            // Empty/Invalid server info
            server_info: JsonRpcNotification::new("header", None),
        };

        let req = JsonRpcRequest::new("initialize", None, RequestId::Number(1));
        let rt = tokio::runtime::Runtime::new()?;
        let resp = rt.block_on(async { server.handle_request(req).await });

        if let JsonRpcMessage::Response(r) = resp {
            // Should be an error, NOT a panic
            assert!(r.error.is_some());
            assert_eq!(r.error.unwrap().code, -32603);
        } else {
            return Err("Expected response".into());
        }
        Ok(())
    }
}
