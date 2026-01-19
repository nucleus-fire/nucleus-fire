//! Nucleus Neural - AI/LLM Integration
//!
//! Built-in OpenAI-compatible LLM client:
//! - GPT-4, GPT-3.5, and compatible models
//! - Streaming responses (planned)
//! - Configurable endpoints for local models
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::neural::Neural;
//!
//! let ai = Neural::new("sk-...")
//!     .with_model("gpt-4o");
//!
//! let response = ai.ask("What is Rust?").await?;
//! ```

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST/RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Chat message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// A single chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_string(),
        }
    }

    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_string(),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
        }
    }
}

/// Completion request payload
#[derive(Debug, Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

/// Completion response
#[derive(Debug, Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

/// Token usage information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ═══════════════════════════════════════════════════════════════════════════
// NEURAL CLIENT
// ═══════════════════════════════════════════════════════════════════════════

/// AI/LLM client for OpenAI-compatible APIs
#[derive(Debug, Clone)]
pub struct Neural {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

impl Neural {
    /// Create a new Neural client with API key
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            model: "gpt-4o".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            temperature: None,
            max_tokens: None,
        }
    }

    /// Set the model to use
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    /// Set custom base URL (for local models like Ollama)
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.trim_end_matches('/').to_string();
        self
    }

    /// Set temperature (0.0 - 2.0, default varies by model)
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp.clamp(0.0, 2.0));
        self
    }

    /// Set max tokens for response
    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Get the configured model
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get the configured base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Ask the AI a simple question
    pub async fn ask(&self, prompt: &str) -> Result<String, NeuralError> {
        self.chat(vec![ChatMessage::user(prompt)]).await
    }

    /// Chat with message history (returns content only)
    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, NeuralError> {
        self.chat_detailed(messages).await.map(|r| r.content)
    }

    /// Chat with message history (returns full response with usage)
    pub async fn chat_detailed(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<NeuralResponse, NeuralError> {
        let request = CompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let url = format!("{}/chat/completions", self.base_url);

        let res = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| NeuralError::Network(e.to_string()))?;

        let status = res.status();
        if !status.is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(NeuralError::Api(format!("{}: {}", status, error_text)));
        }

        let body: CompletionResponse = res
            .json()
            .await
            .map_err(|e| NeuralError::Parse(e.to_string()))?;

        let content = body
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or(NeuralError::NoResponse)?;

        Ok(NeuralResponse {
            content,
            usage: body.usage,
        })
    }

    /// Chat with system prompt
    pub async fn chat_with_system(&self, system: &str, user: &str) -> Result<String, NeuralError> {
        self.chat(vec![ChatMessage::system(system), ChatMessage::user(user)])
            .await
    }
}

/// Full neural response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralResponse {
    pub content: String,
    pub usage: Option<Usage>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Neural error types
#[derive(Debug, thiserror::Error)]
pub enum NeuralError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("No response from model")]
    NoResponse,

    #[error("Invalid API key")]
    InvalidApiKey,
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // CLIENT CREATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_neural_creation() {
        let neural = Neural::new("sk-test-key");

        assert_eq!(neural.model(), "gpt-4o");
        assert_eq!(neural.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_neural_with_model() {
        let neural = Neural::new("sk-test").with_model("gpt-3.5-turbo");

        assert_eq!(neural.model(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_neural_with_base_url() {
        let neural = Neural::new("key").with_base_url("http://localhost:11434/v1/");

        // Should strip trailing slash
        assert_eq!(neural.base_url(), "http://localhost:11434/v1");
    }

    #[test]
    fn test_neural_with_temperature() {
        let neural = Neural::new("key").with_temperature(0.7);

        assert_eq!(neural.temperature, Some(0.7));
    }

    #[test]
    fn test_neural_temperature_clamped() {
        let neural = Neural::new("key").with_temperature(5.0);

        assert_eq!(neural.temperature, Some(2.0)); // Clamped to max

        let neural = Neural::new("key").with_temperature(-1.0);

        assert_eq!(neural.temperature, Some(0.0)); // Clamped to min
    }

    #[test]
    fn test_neural_with_max_tokens() {
        let neural = Neural::new("key").with_max_tokens(1000);

        assert_eq!(neural.max_tokens, Some(1000));
    }

    #[test]
    fn test_neural_builder_chain() {
        let neural = Neural::new("sk-xxx")
            .with_model("gpt-4")
            .with_temperature(0.5)
            .with_max_tokens(2048)
            .with_base_url("http://local:8080");

        assert_eq!(neural.model(), "gpt-4");
        assert_eq!(neural.temperature, Some(0.5));
        assert_eq!(neural.max_tokens, Some(2048));
        assert_eq!(neural.base_url(), "http://local:8080");
    }

    #[test]
    fn test_neural_clone() {
        let neural1 = Neural::new("key").with_model("gpt-4");
        let neural2 = neural1.clone();

        assert_eq!(neural2.model(), "gpt-4");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MESSAGE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_chat_message_system() {
        let msg = ChatMessage::system("You are helpful");

        assert_eq!(msg.role, Role::System);
        assert_eq!(msg.content, "You are helpful");
    }

    #[test]
    fn test_chat_message_user() {
        let msg = ChatMessage::user("Hello");

        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_chat_message_assistant() {
        let msg = ChatMessage::assistant("Hi there!");

        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.content, "Hi there!");
    }

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage::user("test");
        let json = serde_json::to_string(&msg).unwrap();

        assert!(json.contains("\"role\":\"user\""));
        assert!(json.contains("\"content\":\"test\""));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // REQUEST SERIALIZATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_completion_request_serialization() {
        let request = CompletionRequest {
            model: "gpt-4".to_string(),
            messages: vec![ChatMessage::user("Hello")],
            temperature: Some(0.7),
            max_tokens: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"model\":\"gpt-4\""));
        assert!(json.contains("\"temperature\":0.7"));
        // max_tokens should be skipped when None
        assert!(!json.contains("max_tokens"));
    }

    #[test]
    fn test_completion_request_minimal() {
        let request = CompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            temperature: None,
            max_tokens: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        // Only model and messages should be present
        assert!(json.contains("\"model\""));
        assert!(json.contains("\"messages\""));
        assert!(!json.contains("temperature"));
        assert!(!json.contains("max_tokens"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RESPONSE DESERIALIZATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_completion_response_parsing() {
        let json = r#"{
            "choices": [
                {
                    "message": {
                        "content": "Hello! How can I help you?"
                    }
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 8,
                "total_tokens": 18
            }
        }"#;

        let response: CompletionResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.choices.len(), 1);
        assert_eq!(
            response.choices[0].message.content,
            "Hello! How can I help you?"
        );
        assert!(response.usage.is_some());
        assert_eq!(response.usage.unwrap().total_tokens, 18);
    }

    #[test]
    fn test_completion_response_without_usage() {
        let json = r#"{
            "choices": [
                {
                    "message": {
                        "content": "Response"
                    }
                }
            ]
        }"#;

        let response: CompletionResponse = serde_json::from_str(json).unwrap();

        assert!(response.usage.is_none());
    }

    #[test]
    fn test_completion_response_empty_choices() {
        let json = r#"{"choices": []}"#;

        let response: CompletionResponse = serde_json::from_str(json).unwrap();

        assert!(response.choices.is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ERROR TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_neural_error_display() {
        let err = NeuralError::Network("connection refused".to_string());
        assert_eq!(err.to_string(), "Network error: connection refused");

        let err = NeuralError::Api("401 Unauthorized".to_string());
        assert_eq!(err.to_string(), "API error: 401 Unauthorized");

        let err = NeuralError::NoResponse;
        assert_eq!(err.to_string(), "No response from model");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ROLE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_role_serialization() {
        assert_eq!(serde_json::to_string(&Role::System).unwrap(), "\"system\"");
        assert_eq!(serde_json::to_string(&Role::User).unwrap(), "\"user\"");
        assert_eq!(
            serde_json::to_string(&Role::Assistant).unwrap(),
            "\"assistant\""
        );
    }

    #[test]
    fn test_role_deserialization() {
        let system: Role = serde_json::from_str("\"system\"").unwrap();
        assert_eq!(system, Role::System);

        let user: Role = serde_json::from_str("\"user\"").unwrap();
        assert_eq!(user, Role::User);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // USAGE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_usage_parsing() {
        let json = r#"{"prompt_tokens": 100, "completion_tokens": 50, "total_tokens": 150}"#;
        let usage: Usage = serde_json::from_str(json).unwrap();

        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }
}
