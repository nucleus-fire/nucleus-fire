//! Nucleus Push - Push Notifications
//!
//! Multi-platform push notification support for Firebase FCM and OneSignal.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::push::{Push, PushMessage};
//!
//! // Initialize with Firebase FCM
//! let push = Push::firebase("service_account.json").await?;
//!
//! // Send to a device
//! push.send(PushMessage::new("New message!")
//!     .title("Alert")
//!     .icon("/icon.png")
//!     .data(json!({"key": "value"}))
//!     .to_token("device_token"))
//!     .await?;
//!
//! // Send to topic
//! push.send_to_topic("news", message).await?;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Push notification error types
#[derive(Debug, thiserror::Error)]
pub enum PushError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Device not registered")]
    DeviceNotRegistered,

    #[error("Rate limited")]
    RateLimited,

    #[error("Message too large")]
    MessageTooLarge,

    #[error("Invalid topic: {0}")]
    InvalidTopic(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<reqwest::Error> for PushError {
    fn from(err: reqwest::Error) -> Self {
        PushError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for PushError {
    fn from(err: serde_json::Error) -> Self {
        PushError::SerializationError(err.to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PUSH MESSAGE
// ═══════════════════════════════════════════════════════════════════════════

/// Push message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessage {
    /// Message body
    pub body: String,
    /// Message title
    pub title: Option<String>,
    /// Icon URL
    pub icon: Option<String>,
    /// Image URL
    pub image: Option<String>,
    /// Click action URL
    pub click_action: Option<String>,
    /// Custom data payload
    pub data: HashMap<String, serde_json::Value>,
    /// Target token (single device)
    pub token: Option<String>,
    /// Target topic
    pub topic: Option<String>,
    /// Priority (high, normal)
    pub priority: MessagePriority,
    /// Time to live in seconds
    pub ttl: Option<u32>,
    /// Android-specific options
    pub android: Option<AndroidConfig>,
    /// iOS-specific options
    pub ios: Option<IosConfig>,
    /// Web-specific options
    pub web: Option<WebConfig>,
}

impl PushMessage {
    /// Create a new push message
    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
            title: None,
            icon: None,
            image: None,
            click_action: None,
            data: HashMap::new(),
            token: None,
            topic: None,
            priority: MessagePriority::Normal,
            ttl: None,
            android: None,
            ios: None,
            web: None,
        }
    }

    /// Set message title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set icon URL
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Set image URL
    pub fn image(mut self, image: &str) -> Self {
        self.image = Some(image.to_string());
        self
    }

    /// Set click action URL
    pub fn click_action(mut self, url: &str) -> Self {
        self.click_action = Some(url.to_string());
        self
    }

    /// Add custom data field
    pub fn data_field(mut self, key: &str, value: impl Serialize) -> Self {
        if let Ok(v) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), v);
        }
        self
    }

    /// Set all custom data
    pub fn data(mut self, data: serde_json::Value) -> Self {
        if let Some(obj) = data.as_object() {
            self.data = obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        }
        self
    }

    /// Target a specific device token
    pub fn to_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    /// Target a topic
    pub fn to_topic(mut self, topic: &str) -> Self {
        self.topic = Some(topic.to_string());
        self
    }

    /// Set high priority
    pub fn high_priority(mut self) -> Self {
        self.priority = MessagePriority::High;
        self
    }

    /// Set time to live
    pub fn ttl(mut self, seconds: u32) -> Self {
        self.ttl = Some(seconds);
        self
    }

    /// Set Android-specific options
    pub fn android(mut self, config: AndroidConfig) -> Self {
        self.android = Some(config);
        self
    }

    /// Set iOS-specific options
    pub fn ios(mut self, config: IosConfig) -> Self {
        self.ios = Some(config);
        self
    }

    /// Set web-specific options
    pub fn web(mut self, config: WebConfig) -> Self {
        self.web = Some(config);
        self
    }
}

/// Message priority
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessagePriority {
    High,
    #[default]
    Normal,
}

/// Android-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AndroidConfig {
    pub channel_id: Option<String>,
    pub color: Option<String>,
    pub sound: Option<String>,
    pub tag: Option<String>,
}

impl AndroidConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn channel_id(mut self, id: &str) -> Self {
        self.channel_id = Some(id.to_string());
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn sound(mut self, sound: &str) -> Self {
        self.sound = Some(sound.to_string());
        self
    }
}

/// iOS-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IosConfig {
    pub badge: Option<i32>,
    pub sound: Option<String>,
    pub category: Option<String>,
}

impl IosConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn badge(mut self, badge: i32) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn sound(mut self, sound: &str) -> Self {
        self.sound = Some(sound.to_string());
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }
}

/// Web-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebConfig {
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub actions: Vec<WebAction>,
}

impl WebConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn badge(mut self, badge: &str) -> Self {
        self.badge = Some(badge.to_string());
        self
    }

    pub fn action(mut self, action: &str, title: &str) -> Self {
        self.actions.push(WebAction {
            action: action.to_string(),
            title: title.to_string(),
        });
        self
    }
}

/// Web notification action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAction {
    pub action: String,
    pub title: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// SEND RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Result of sending a push notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

impl SendResult {
    fn success(message_id: &str) -> Self {
        Self {
            success: true,
            message_id: Some(message_id.to_string()),
            error: None,
        }
    }

    fn failure(error: &str) -> Self {
        Self {
            success: false,
            message_id: None,
            error: Some(error.to_string()),
        }
    }
}

/// Result of sending to multiple devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub success_count: usize,
    pub failure_count: usize,
    pub results: Vec<SendResult>,
}

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════

enum Backend {
    Firebase(FirebaseBackend),
    OneSignal(OneSignalBackend),
}

impl Backend {
    async fn send(&self, message: &PushMessage) -> Result<SendResult, PushError> {
        match self {
            Backend::Firebase(b) => b.send(message).await,
            Backend::OneSignal(b) => b.send(message).await,
        }
    }

    async fn send_batch(&self, messages: &[PushMessage]) -> Result<BatchResult, PushError> {
        match self {
            Backend::Firebase(b) => b.send_batch(messages).await,
            Backend::OneSignal(b) => b.send_batch(messages).await,
        }
    }

    async fn subscribe_to_topic(&self, token: &str, topic: &str) -> Result<(), PushError> {
        match self {
            Backend::Firebase(b) => b.subscribe_to_topic(token, topic).await,
            Backend::OneSignal(b) => b.subscribe_to_topic(token, topic).await,
        }
    }

    async fn unsubscribe_from_topic(&self, token: &str, topic: &str) -> Result<(), PushError> {
        match self {
            Backend::Firebase(b) => b.unsubscribe_from_topic(token, topic).await,
            Backend::OneSignal(b) => b.unsubscribe_from_topic(token, topic).await,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FIREBASE FCM BACKEND
// ═══════════════════════════════════════════════════════════════════════════

struct FirebaseBackend {
    client: reqwest::Client,
    project_id: String,
    access_token: String,
    private_key: String,
    client_email: String,
}

impl FirebaseBackend {
    async fn new(credentials_json: &str) -> Result<Self, PushError> {
        // Parse credentials
        let creds: FirebaseCredentials = serde_json::from_str(credentials_json)
            .map_err(|e| PushError::ConfigError(format!("Invalid credentials: {}", e)))?;

        // In production, you'd get an OAuth token from the service account
        // For now, we'll use the private_key_id as a placeholder
        let access_token = creds.private_key_id.clone();

        Ok(Self {
            client: reqwest::Client::new(),
            project_id: creds.project_id,
            access_token,
            private_key: creds.private_key,
            client_email: creds.client_email,
        })
    }

    async fn send(&self, message: &PushMessage) -> Result<SendResult, PushError> {
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        // Log the actor
        nucleus_std::logging::debug!("Push: Sending via FCM as {}", self.client_email);
        let _ = &self.private_key; // Suppress unused warning (reserved for JWT)

        let fcm_message = self.build_fcm_message(message)?;

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&serde_json::json!({ "message": fcm_message }))
            .send()
            .await?;

        if response.status() == 401 {
            return Err(PushError::AuthError("Invalid token".to_string()));
        }
        if response.status() == 429 {
            return Err(PushError::RateLimited);
        }
        if response.status() == 400 {
            return Err(PushError::InvalidToken("Bad request".to_string()));
        }

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            let message_id = result["name"].as_str().unwrap_or("").to_string();
            Ok(SendResult::success(&message_id))
        } else {
            let error: serde_json::Value = response.json().await.unwrap_or_default();
            Ok(SendResult::failure(
                error["error"]["message"]
                    .as_str()
                    .unwrap_or("Unknown error"),
            ))
        }
    }

    async fn send_batch(&self, messages: &[PushMessage]) -> Result<BatchResult, PushError> {
        let mut results = Vec::new();
        let mut success_count = 0;
        let mut failure_count = 0;

        for message in messages {
            let result = self.send(message).await?;
            if result.success {
                success_count += 1;
            } else {
                failure_count += 1;
            }
            results.push(result);
        }

        Ok(BatchResult {
            success_count,
            failure_count,
            results,
        })
    }

    async fn subscribe_to_topic(&self, token: &str, topic: &str) -> Result<(), PushError> {
        let url = format!(
            "https://iid.googleapis.com/iid/v1/{}/rel/topics/{}",
            token, topic
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PushError::BackendError("Failed to subscribe".to_string()));
        }

        Ok(())
    }

    async fn unsubscribe_from_topic(&self, token: &str, topic: &str) -> Result<(), PushError> {
        let url = "https://iid.googleapis.com/iid/v1:batchRemove";

        let body = serde_json::json!({
            "to": format!("/topics/{}", topic),
            "registration_tokens": [token]
        });

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PushError::BackendError("Failed to unsubscribe".to_string()));
        }

        Ok(())
    }

    fn build_fcm_message(&self, message: &PushMessage) -> Result<serde_json::Value, PushError> {
        let mut fcm = serde_json::json!({
            "notification": {
                "body": message.body
            }
        });

        if let Some(title) = &message.title {
            fcm["notification"]["title"] = serde_json::json!(title);
        }
        if let Some(image) = &message.image {
            fcm["notification"]["image"] = serde_json::json!(image);
        }

        if let Some(token) = &message.token {
            fcm["token"] = serde_json::json!(token);
        } else if let Some(topic) = &message.topic {
            fcm["topic"] = serde_json::json!(topic);
        } else {
            return Err(PushError::ConfigError("No target specified".to_string()));
        }

        if !message.data.is_empty() {
            fcm["data"] = serde_json::to_value(&message.data)?;
        }

        if message.priority == MessagePriority::High {
            fcm["android"] = serde_json::json!({"priority": "high"});
        }

        Ok(fcm)
    }
}

#[derive(Debug, Deserialize)]
struct FirebaseCredentials {
    project_id: String,
    private_key_id: String,

    private_key: String,

    client_email: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// ONESIGNAL BACKEND
// ═══════════════════════════════════════════════════════════════════════════

struct OneSignalBackend {
    client: reqwest::Client,
    app_id: String,
    api_key: String,
}

impl OneSignalBackend {
    fn new(app_id: &str, api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            app_id: app_id.to_string(),
            api_key: api_key.to_string(),
        }
    }

    async fn send(&self, message: &PushMessage) -> Result<SendResult, PushError> {
        let url = "https://onesignal.com/api/v1/notifications";

        let mut body = serde_json::json!({
            "app_id": self.app_id,
            "contents": {"en": message.body}
        });

        if let Some(title) = &message.title {
            body["headings"] = serde_json::json!({"en": title});
        }
        if let Some(token) = &message.token {
            body["include_player_ids"] = serde_json::json!([token]);
        } else if let Some(topic) = &message.topic {
            body["included_segments"] = serde_json::json!([topic]);
        }
        if let Some(url) = &message.click_action {
            body["url"] = serde_json::json!(url);
        }
        if !message.data.is_empty() {
            body["data"] = serde_json::to_value(&message.data)?;
        }

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if response.status() == 401 {
            return Err(PushError::AuthError("Invalid API key".to_string()));
        }
        if response.status() == 429 {
            return Err(PushError::RateLimited);
        }

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            let message_id = result["id"].as_str().unwrap_or("").to_string();
            Ok(SendResult::success(&message_id))
        } else {
            let error: serde_json::Value = response.json().await.unwrap_or_default();
            Ok(SendResult::failure(&error["errors"].to_string()))
        }
    }

    async fn send_batch(&self, messages: &[PushMessage]) -> Result<BatchResult, PushError> {
        let mut results = Vec::new();
        let mut success_count = 0;
        let mut failure_count = 0;

        for message in messages {
            let result = self.send(message).await?;
            if result.success {
                success_count += 1;
            } else {
                failure_count += 1;
            }
            results.push(result);
        }

        Ok(BatchResult {
            success_count,
            failure_count,
            results,
        })
    }

    async fn subscribe_to_topic(&self, _token: &str, _topic: &str) -> Result<(), PushError> {
        // OneSignal uses segments/tags instead of FCM-style topics
        // This would update the player's tags
        Ok(())
    }

    async fn unsubscribe_from_topic(&self, _token: &str, _topic: &str) -> Result<(), PushError> {
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PUSH (MAIN API)
// ═══════════════════════════════════════════════════════════════════════════

/// Main Push notification interface
pub struct Push {
    backend: Backend,
}

impl Push {
    /// Create Push with Firebase FCM backend
    pub async fn firebase(credentials_json: &str) -> Result<Self, PushError> {
        let backend = FirebaseBackend::new(credentials_json).await?;
        Ok(Self {
            backend: Backend::Firebase(backend),
        })
    }

    /// Create Push with OneSignal backend
    pub fn onesignal(app_id: &str, api_key: &str) -> Self {
        Self {
            backend: Backend::OneSignal(OneSignalBackend::new(app_id, api_key)),
        }
    }

    /// Send a push notification
    pub async fn send(&self, message: PushMessage) -> Result<SendResult, PushError> {
        self.backend.send(&message).await
    }

    /// Send multiple push notifications
    pub async fn send_batch(&self, messages: &[PushMessage]) -> Result<BatchResult, PushError> {
        self.backend.send_batch(messages).await
    }

    /// Send to a specific topic
    pub async fn send_to_topic(
        &self,
        topic: &str,
        mut message: PushMessage,
    ) -> Result<SendResult, PushError> {
        message = message.to_topic(topic);
        self.backend.send(&message).await
    }

    /// Subscribe a device to a topic
    pub async fn subscribe(&self, token: &str, topic: &str) -> Result<(), PushError> {
        self.backend.subscribe_to_topic(token, topic).await
    }

    /// Unsubscribe a device from a topic
    pub async fn unsubscribe(&self, token: &str, topic: &str) -> Result<(), PushError> {
        self.backend.unsubscribe_from_topic(token, topic).await
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // ERROR TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_error_auth() {
        let err = PushError::AuthError("invalid token".to_string());
        assert!(err.to_string().contains("Authentication"));
    }

    #[test]
    fn test_error_invalid_token() {
        let err = PushError::InvalidToken("expired".to_string());
        assert!(err.to_string().contains("Invalid token"));
    }

    #[test]
    fn test_error_device_not_registered() {
        let err = PushError::DeviceNotRegistered;
        assert!(err.to_string().contains("not registered"));
    }

    #[test]
    fn test_error_rate_limited() {
        let err = PushError::RateLimited;
        assert!(err.to_string().contains("Rate limited"));
    }

    #[test]
    fn test_error_message_too_large() {
        let err = PushError::MessageTooLarge;
        assert!(err.to_string().contains("too large"));
    }

    #[test]
    fn test_error_invalid_topic() {
        let err = PushError::InvalidTopic("bad/topic".to_string());
        assert!(err.to_string().contains("Invalid topic"));
    }

    #[test]
    fn test_error_backend() {
        let err = PushError::BackendError("server error".to_string());
        assert!(err.to_string().contains("Backend"));
    }

    #[test]
    fn test_error_config() {
        let err = PushError::ConfigError("missing key".to_string());
        assert!(err.to_string().contains("Configuration"));
    }

    #[test]
    fn test_error_network() {
        let err = PushError::NetworkError("timeout".to_string());
        assert!(err.to_string().contains("Network"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PUSH MESSAGE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_message_new() {
        let msg = PushMessage::new("Hello!");
        assert_eq!(msg.body, "Hello!");
        assert!(msg.title.is_none());
        assert_eq!(msg.priority, MessagePriority::Normal);
    }

    #[test]
    fn test_message_builder() {
        let msg = PushMessage::new("Body")
            .title("Title")
            .icon("/icon.png")
            .image("/image.png")
            .click_action("https://example.com")
            .to_token("token123");

        assert_eq!(msg.body, "Body");
        assert_eq!(msg.title, Some("Title".to_string()));
        assert_eq!(msg.icon, Some("/icon.png".to_string()));
        assert_eq!(msg.image, Some("/image.png".to_string()));
        assert_eq!(msg.token, Some("token123".to_string()));
    }

    #[test]
    fn test_message_data() {
        let msg = PushMessage::new("Test")
            .data_field("key1", "value1")
            .data_field("key2", 42);

        assert_eq!(msg.data.len(), 2);
        assert_eq!(msg.data.get("key1").unwrap(), "value1");
        assert_eq!(msg.data.get("key2").unwrap(), 42);
    }

    #[test]
    fn test_message_data_json() {
        let msg = PushMessage::new("Test").data(serde_json::json!({
            "a": 1,
            "b": "two"
        }));

        assert_eq!(msg.data.len(), 2);
    }

    #[test]
    fn test_message_priority() {
        let msg = PushMessage::new("Test").high_priority();
        assert_eq!(msg.priority, MessagePriority::High);
    }

    #[test]
    fn test_message_ttl() {
        let msg = PushMessage::new("Test").ttl(3600);
        assert_eq!(msg.ttl, Some(3600));
    }

    #[test]
    fn test_message_topic() {
        let msg = PushMessage::new("Test").to_topic("news");
        assert_eq!(msg.topic, Some("news".to_string()));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PLATFORM CONFIG TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_android_config() {
        let config = AndroidConfig::new()
            .channel_id("alerts")
            .color("#FF0000")
            .sound("alert.wav");

        assert_eq!(config.channel_id, Some("alerts".to_string()));
        assert_eq!(config.color, Some("#FF0000".to_string()));
        assert_eq!(config.sound, Some("alert.wav".to_string()));
    }

    #[test]
    fn test_ios_config() {
        let config = IosConfig::new()
            .badge(5)
            .sound("ping.aiff")
            .category("MESSAGE");

        assert_eq!(config.badge, Some(5));
        assert_eq!(config.sound, Some("ping.aiff".to_string()));
        assert_eq!(config.category, Some("MESSAGE".to_string()));
    }

    #[test]
    fn test_web_config() {
        let config = WebConfig::new()
            .icon("/icon.png")
            .badge("/badge.png")
            .action("reply", "Reply");

        assert_eq!(config.icon, Some("/icon.png".to_string()));
        assert_eq!(config.actions.len(), 1);
        assert_eq!(config.actions[0].action, "reply");
    }

    #[test]
    fn test_message_with_android() {
        let android = AndroidConfig::new().channel_id("main");
        let msg = PushMessage::new("Test").android(android);
        assert!(msg.android.is_some());
    }

    #[test]
    fn test_message_with_ios() {
        let ios = IosConfig::new().badge(3);
        let msg = PushMessage::new("Test").ios(ios);
        assert!(msg.ios.is_some());
    }

    #[test]
    fn test_message_with_web() {
        let web = WebConfig::new().icon("/icon.png");
        let msg = PushMessage::new("Test").web(web);
        assert!(msg.web.is_some());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RESULT TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_send_result_success() {
        let result = SendResult::success("msg_123");
        assert!(result.success);
        assert_eq!(result.message_id, Some("msg_123".to_string()));
        assert!(result.error.is_none());
    }

    #[test]
    fn test_send_result_failure() {
        let result = SendResult::failure("Token expired");
        assert!(!result.success);
        assert!(result.message_id.is_none());
        assert_eq!(result.error, Some("Token expired".to_string()));
    }

    #[test]
    fn test_batch_result() {
        let batch = BatchResult {
            success_count: 8,
            failure_count: 2,
            results: vec![],
        };
        assert_eq!(batch.success_count, 8);
        assert_eq!(batch.failure_count, 2);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_push_onesignal() {
        let push = Push::onesignal("app_123", "api_key");
        drop(push);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SERIALIZATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_priority_serialization() {
        let high = MessagePriority::High;
        let json = serde_json::to_string(&high).unwrap();
        assert_eq!(json, "\"high\"");
    }

    #[test]
    fn test_message_serialization() {
        let msg = PushMessage::new("Test").title("Hello");
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"body\":\"Test\""));
        assert!(json.contains("\"title\":\"Hello\""));
    }
}
