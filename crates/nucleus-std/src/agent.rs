//! AI Agent Framework
//!
//! High-level abstraction for building autonomous agents that can:
//! - Use tools (MCP and local)
//! - Maintain conversation history
//! - Reason and plan (ReAct loop)

use crate::neural::{Neural, ChatMessage, NeuralError};
use crate::mcp::Tool; // Removed unused Content
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;
use tracing::{info, warn, debug, instrument}; // Add tracing

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Neural error: {0}")]
    Neural(#[from] NeuralError),
    
    #[error("Tool error: {0}")]
    Tool(String),
    
    #[error("Task limit reached ({0} steps)")]
    TaskLimitReached(usize),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// AGENT
// ═══════════════════════════════════════════════════════════════════════════

type AgentToolHandler = Box<dyn Fn(Value) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send >> + Send + Sync>;

#[derive(Clone)]
pub struct Agent {
    neural: Neural,
    tools: Arc<Mutex<HashMap<String, (Tool, AgentToolHandler)>>>,
    history: Arc<Mutex<Vec<ChatMessage>>>,
    system_prompt: String,
    max_steps: usize,
}

/// Builder for robust Agent configuration
pub struct AgentBuilder {
    neural: Neural,
    system_prompt: String,
    max_steps: usize,
    temperature: Option<f32>,
    tools: HashMap<String, (Tool, AgentToolHandler)>,
}

impl AgentBuilder {
    pub fn new(neural: Neural) -> Self {
        Self {
            neural,
            system_prompt: "You are a helpful AI assistant.".to_string(),
            max_steps: 10,
            temperature: None,
            tools: HashMap::new(),
        }
    }

    pub fn system(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }
    
    pub fn max_steps(mut self, steps: usize) -> Self {
        self.max_steps = steps;
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_tool<F, Fut>(mut self, tool: Tool, handler: F) -> Self
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, String>> + Send + 'static,
    {
        let boxed_handler = Box::new(move |args| {
            let fut = handler(args);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Result<String, String>> + Send>>
        }) as AgentToolHandler;
        
        self.tools.insert(tool.name.clone(), (tool, boxed_handler));
        self
    }

    pub fn build(self) -> Agent {
        // Configure neural client with specific agent settings if provided
        let mut neural = self.neural;
        if let Some(t) = self.temperature {
            neural = neural.with_temperature(t);
        }

        Agent {
            neural,
            tools: Arc::new(Mutex::new(self.tools)),
            history: Arc::new(Mutex::new(Vec::new())),
            system_prompt: self.system_prompt,
            max_steps: self.max_steps,
        }
    }
}

impl Agent {
    /// Create a new agent builder
    pub fn builder(neural: Neural) -> AgentBuilder {
        AgentBuilder::new(neural)
    }

    /// Legacy constructor for backward compatibility, prefers builder()
    pub fn new(neural: Neural) -> Self {
        AgentBuilder::new(neural).build()
    }

    /// Register a tool dynamically (post-build)
    pub async fn register_tool<F, Fut>(&self, tool: Tool, handler: F) 
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, String>> + Send + 'static,
    {
        let mut tools = self.tools.lock().await;
        let boxed_handler = Box::new(move |args| {
            let fut = handler(args);
            Box::pin(fut) as Pin<Box<dyn Future<Output = Result<String, String>> + Send>>
        }) as AgentToolHandler;
        
        info!(tool = %tool.name, "Registering dynamic tool");
        tools.insert(tool.name.clone(), (tool, boxed_handler));
    }

    /// Run the agent loop with a user prompt
    #[instrument(skip(self), fields(prompt_len = prompt.len()))]
    pub async fn run(&self, prompt: &str) -> Result<String, ImportError> {
        let mut history = self.history.lock().await;
        
        // Initialize history if empty
        if history.is_empty() {
            debug!("Initializing conversation with system prompt");
            history.push(ChatMessage::system(&self.system_prompt));
        }
        history.push(ChatMessage::user(prompt));

        let mut steps = 0;
        loop {
            if steps >= self.max_steps {
                warn!("Agent reached max steps limit: {}", self.max_steps);
                return Err(ImportError::TaskLimitReached(self.max_steps));
            }
            steps += 1;

            debug!(step = steps, "Thinking...");
            // call LLM
            let response = self.neural.chat(history.clone()).await?;
            history.push(ChatMessage::assistant(&response));
            
            info!(response_len = response.len(), "Agent response received");

            // Check if tool call is needed
            if let Some((tool_name, args_str)) = self.parse_action(&response) {
                let tools = self.tools.lock().await;
                if let Some((_, handler)) = tools.get(&tool_name) {
                    info!(tool = %tool_name, "Calling tool");
                    let args: Value = serde_json::from_str(&args_str)
                        .unwrap_or(Value::Null); 
                    
                    let result = match handler(args).await {
                        Ok(res) => {
                            debug!("Tool success");
                            res
                        },
                        Err(e) => {
                            warn!(error = %e, "Tool failed");
                            format!("Error: {}", e)
                        }
                    };

                    history.push(ChatMessage::user(&format!("Observation: {}", result)));
                } else {
                    warn!(tool = %tool_name, "Tool not found");
                    history.push(ChatMessage::user(&format!("Error: Tool '{}' not found. Available tools: {:?}", tool_name, tools.keys())));
                }
            } else {
                debug!("No action detected, task complete");
                return Ok(response);
            }
        }
    }

    fn parse_action(&self, response: &str) -> Option<(String, String)> {
        // Very naive parser for "Action: name({args})"
        // In production use regex or better LLM structured output
        if let Some(idx) = response.find("Action: ") {
            let rest = &response[idx + 8..];
            if let Some(open_paren) = rest.find('(') {
                let name = &rest[..open_paren];
                if let Some(close_paren) = rest.rfind(')') {
                    let args = &rest[open_paren+1..close_paren];
                    return Some((name.trim().to_string(), args.trim().to_string()));
                }
            }
        }
        None
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_agent_loop() {
        // 1. Setup Mock LLM
        let server = MockServer::start().await.unwrap();
        
        // Response 1: Tool Call
        server.expect("POST", "/chat/completions")
            .times(1)
            .respond_with_json(json!({
                "choices": [{
                    "message": {
                        "content": "Action: reverse_string({\"input\": \"hello\"})"
                    }
                }]
            }))
            .mount().await;

        // Response 2: Final Answer
        server.expect("POST", "/chat/completions")
            .times(1)
            .respond_with_json(json!({
                "choices": [{
                    "message": {
                        "content": "olleh"
                    }
                }]
            }))
            .mount().await;

        // 2. Setup Agent
        let neural = Neural::new("test-key")
            .with_base_url(&server.url());
            
        let agent = Agent::builder(neural)
            .build();
        
        // 3. Register Tool
        agent.register_tool(
            Tool {
                name: "reverse_string".into(),
                description: "Reverses a string".into(),
                input_schema: json!({})
            },
            |args| async move {
                let input = args["input"].as_str().unwrap();
                let reversed: String = input.chars().rev().collect();
                Ok(reversed)
            }
        ).await;

        // 4. Run
        let result = agent.run("Reverse 'hello'").await.unwrap();
        
        assert_eq!(result, "olleh");
    }
}
