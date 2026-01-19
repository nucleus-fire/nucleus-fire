//! Beacon - Privacy-First Analytics
//!
//! Easy-to-implement analytics with multiple provider support.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::beacon::{Beacon, AnalyticsProvider};
//!
//! // Initialize with webhook
//! let beacon = Beacon::new(AnalyticsProvider::Webhook {
//!     url: "https://analytics.example.com/events".to_string(),
//! });
//!
//! // Track events
//! beacon.track("button_click", json!({ "button_id": "signup" })).await;
//! beacon.page_view("/dashboard").await;
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
// use std::fs::OpenOptions;
// use std::io::Write;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Analytics event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    /// Event name
    pub name: String,
    /// Event properties
    pub properties: HashMap<String, Value>,
    /// Timestamp (Unix millis)
    pub timestamp: u64,
    /// User ID if identified
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
}

impl AnalyticsEvent {
    /// Create a new event
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            properties: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            user_id: None,
            session_id: None,
        }
    }

    /// Add a property
    pub fn property(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.properties.insert(key.to_string(), value.into());
        self
    }

    /// Set user ID
    pub fn with_user(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }
}

/// Analytics provider configuration
#[derive(Debug, Clone)]
pub enum AnalyticsProvider {
    /// In-memory storage (for testing)
    InMemory,
    /// File-based logging
    File { path: PathBuf },
    /// Webhook (POST to URL)
    Webhook { url: String },
    /// Plausible Analytics (privacy-friendly)
    Plausible { domain: String, api_host: Option<String> },
    /// Disabled (no-op)
    Disabled,
}

// ═══════════════════════════════════════════════════════════════════════════
// BEACON
// ═══════════════════════════════════════════════════════════════════════════

/// Analytics tracker
#[derive(Clone)]
pub struct Beacon {
    provider: AnalyticsProvider,
    /// In-memory event storage (for InMemory provider and buffering)
    events: Arc<Mutex<Vec<AnalyticsEvent>>>,
    /// Current user ID
    user_id: Arc<Mutex<Option<String>>>,
    /// HTTP client
    client: reqwest::Client,
}

impl std::fmt::Debug for Beacon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Beacon")
            .field("provider", &self.provider)
            .finish()
    }
}

impl Beacon {
    /// Create a new Beacon with the specified provider
    pub fn new(provider: AnalyticsProvider) -> Self {
        Self {
            provider,
            events: Arc::new(Mutex::new(Vec::new())),
            user_id: Arc::new(Mutex::new(None)),
            client: reqwest::Client::new(),
        }
    }

    /// Create from environment variable ANALYTICS_PROVIDER
    ///
    /// Supported values:
    /// - `disabled` - No analytics
    /// - `memory` - In-memory (testing)
    /// - `file:/path/to/file.jsonl` - File logging
    /// - `webhook:https://url` - Webhook
    /// - `plausible:domain.com` - Plausible Analytics
    pub fn from_env() -> Self {
        let provider = std::env::var("ANALYTICS_PROVIDER").unwrap_or_else(|_| "disabled".to_string());
        
        let provider = if provider == "disabled" {
            AnalyticsProvider::Disabled
        } else if provider == "memory" {
            AnalyticsProvider::InMemory
        } else if let Some(path) = provider.strip_prefix("file:") {
            AnalyticsProvider::File { path: PathBuf::from(path) }
        } else if let Some(url) = provider.strip_prefix("webhook:") {
            AnalyticsProvider::Webhook { url: url.to_string() }
        } else if let Some(domain) = provider.strip_prefix("plausible:") {
            AnalyticsProvider::Plausible { 
                domain: domain.to_string(), 
                api_host: std::env::var("PLAUSIBLE_HOST").ok(),
            }
        } else {
            AnalyticsProvider::Disabled
        };

        Self::new(provider)
    }

    /// Identify the current user
    pub fn identify(&self, user_id: &str, traits: HashMap<String, Value>) {
        *self.user_id.lock().unwrap() = Some(user_id.to_string());
        
        // Track identify event
        let mut event = AnalyticsEvent::new("identify");
        event.user_id = Some(user_id.to_string());
        event.properties = traits;
        
        self.store_event(event);
    }

    /// Track a custom event
    pub async fn track(&self, event_name: &str, properties: Value) {
        let mut event = AnalyticsEvent::new(event_name);
        
        if let Value::Object(map) = properties {
            for (k, v) in map {
                event.properties.insert(k, v);
            }
        }
        
        event.user_id = self.user_id.lock().unwrap().clone();
        
        self.send_event(event).await;
    }

    /// Track a page view
    pub async fn page_view(&self, path: &str) {
        let event = AnalyticsEvent::new("page_view")
            .property("path", path);
        
        self.send_event(event).await;
    }

    /// Track a click event
    pub async fn click(&self, element: &str) {
        let event = AnalyticsEvent::new("click")
            .property("element", element);
        
        self.send_event(event).await;
    }

    /// Track an error
    pub async fn error(&self, message: &str, details: Option<Value>) {
        let mut event = AnalyticsEvent::new("error")
            .property("message", message);
        
        if let Some(d) = details {
            event.properties.insert("details".to_string(), d);
        }
        
        self.send_event(event).await;
    }

    /// Flush all buffered events
    pub async fn flush(&self) {
        let events: Vec<AnalyticsEvent> = {
            let mut lock = self.events.lock().unwrap();
            std::mem::take(&mut *lock)
        };

        for event in events {
            self.send_to_provider(&event).await;
        }
    }

    /// Get all stored events (InMemory provider only)
    pub fn get_events(&self) -> Vec<AnalyticsEvent> {
        self.events.lock().unwrap().clone()
    }

    /// Clear all stored events
    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }

    // ─────────────────────────────────────────────────────────────────────────
    // INTERNAL
    // ─────────────────────────────────────────────────────────────────────────

    fn store_event(&self, event: AnalyticsEvent) {
        self.events.lock().unwrap().push(event);
    }

    async fn send_event(&self, event: AnalyticsEvent) {
        match &self.provider {
            AnalyticsProvider::InMemory => {
                self.store_event(event);
            }
            _ => {
                self.send_to_provider(&event).await;
            }
        }
    }

    async fn send_to_provider(&self, event: &AnalyticsEvent) {
        match &self.provider {
            AnalyticsProvider::Disabled => {}
            
            AnalyticsProvider::InMemory => {
                self.store_event(event.clone());
            }
            
            AnalyticsProvider::File { path } => {
                use tokio::io::AsyncWriteExt;
                if let Ok(mut file) = tokio::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .await
                {
                    if let Ok(json) = serde_json::to_string(event) {
                        let _ = file.write_all(format!("{}\n", json).as_bytes()).await;
                    }
                }
            }
            
            AnalyticsProvider::Webhook { url } => {
                let _ = self.client
                    .post(url)
                    .json(event)
                    .send()
                    .await;
            }
            
            AnalyticsProvider::Plausible { domain, api_host } => {
                let host = api_host.as_deref().unwrap_or("https://plausible.io");
                let url = format!("{}/api/event", host);
                
                let payload = serde_json::json!({
                    "name": event.name,
                    "url": format!("https://{}{}", domain, 
                        event.properties.get("path").and_then(|v| v.as_str()).unwrap_or("/")),
                    "domain": domain,
                    "props": event.properties,
                });
                
                let _ = self.client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .send()
                    .await;
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    
    // Mutex to serialize tests that modify environment variables
    static ENV_MUTEX: Mutex<()> = Mutex::new(());
    
    // Helper to run a test with exclusive access to env vars
    fn with_env_lock<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = ENV_MUTEX.lock().unwrap();
        f()
    }

    #[test]
    fn test_analytics_event_new() {
        let event = AnalyticsEvent::new("test_event");
        assert_eq!(event.name, "test_event");
        assert!(event.timestamp > 0);
        assert!(event.properties.is_empty());
    }

    #[test]
    fn test_analytics_event_property() {
        let event = AnalyticsEvent::new("click")
            .property("button_id", "submit")
            .property("page", "/signup");
        
        assert_eq!(event.properties.get("button_id").unwrap(), "submit");
        assert_eq!(event.properties.get("page").unwrap(), "/signup");
    }

    #[test]
    fn test_analytics_event_with_user() {
        let event = AnalyticsEvent::new("purchase")
            .with_user("user_123");
        
        assert_eq!(event.user_id, Some("user_123".to_string()));
    }

    #[test]
    fn test_beacon_inmemory() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        let event = AnalyticsEvent::new("test");
        beacon.store_event(event);
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "test");
    }

    #[test]
    fn test_beacon_identify() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        let mut traits = HashMap::new();
        traits.insert("email".to_string(), Value::String("test@example.com".to_string()));
        
        beacon.identify("user_456", traits);
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "identify");
        assert_eq!(events[0].user_id, Some("user_456".to_string()));
    }

    #[tokio::test]
    async fn test_beacon_track() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        beacon.track("button_click", serde_json::json!({
            "button_id": "signup"
        })).await;
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "button_click");
        assert_eq!(events[0].properties.get("button_id").unwrap(), "signup");
    }

    #[tokio::test]
    async fn test_beacon_page_view() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        beacon.page_view("/dashboard").await;
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "page_view");
        assert_eq!(events[0].properties.get("path").unwrap(), "/dashboard");
    }

    #[test]
    fn test_beacon_clear_events() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        beacon.store_event(AnalyticsEvent::new("test1"));
        beacon.store_event(AnalyticsEvent::new("test2"));
        
        assert_eq!(beacon.get_events().len(), 2);
        
        beacon.clear_events();
        
        assert_eq!(beacon.get_events().len(), 0);
    }

    #[test]
    fn test_beacon_disabled_provider() {
        let beacon = Beacon::new(AnalyticsProvider::Disabled);
        // Should not panic
        beacon.store_event(AnalyticsEvent::new("test"));
    }

    #[test]
    fn test_from_env_disabled() {
        with_env_lock(|| {
            std::env::remove_var("ANALYTICS_PROVIDER");
            let beacon = Beacon::from_env();
            assert!(matches!(beacon.provider, AnalyticsProvider::Disabled));
        });
    }

    #[test]
    fn test_from_env_memory() {
        with_env_lock(|| {
            std::env::set_var("ANALYTICS_PROVIDER", "memory");
            let beacon = Beacon::from_env();
            assert!(matches!(beacon.provider, AnalyticsProvider::InMemory));
            std::env::remove_var("ANALYTICS_PROVIDER");
        });
    }

    #[test]
    fn test_from_env_file() {
        with_env_lock(|| {
            std::env::set_var("ANALYTICS_PROVIDER", "file:/tmp/analytics.jsonl");
            let beacon = Beacon::from_env();
            assert!(matches!(beacon.provider, AnalyticsProvider::File { .. }));
            std::env::remove_var("ANALYTICS_PROVIDER");
        });
    }

    #[test]
    fn test_from_env_webhook() {
        with_env_lock(|| {
            std::env::set_var("ANALYTICS_PROVIDER", "webhook:https://example.com/events");
            let beacon = Beacon::from_env();
            assert!(matches!(beacon.provider, AnalyticsProvider::Webhook { .. }));
            std::env::remove_var("ANALYTICS_PROVIDER");
        });
    }

    #[test]
    fn test_from_env_plausible() {
        with_env_lock(|| {
            std::env::set_var("ANALYTICS_PROVIDER", "plausible:mysite.com");
            let beacon = Beacon::from_env();
            assert!(matches!(beacon.provider, AnalyticsProvider::Plausible { .. }));
            std::env::remove_var("ANALYTICS_PROVIDER");
        });
    }

    #[tokio::test]
    async fn test_beacon_click() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        beacon.click("signup_button").await;
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "click");
        assert_eq!(events[0].properties.get("element").unwrap(), "signup_button");
    }

    #[tokio::test]
    async fn test_beacon_error_with_details() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        beacon.error("Something went wrong", Some(serde_json::json!({
            "code": 500,
            "stack": "trace"
        }))).await;
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "error");
        assert_eq!(events[0].properties.get("message").unwrap(), "Something went wrong");
        assert!(events[0].properties.contains_key("details"));
    }

    #[tokio::test]
    async fn test_beacon_error_without_details() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        beacon.error("Simple error", None).await;
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "error");
        assert!(!events[0].properties.contains_key("details"));
    }

    #[tokio::test]
    async fn test_beacon_flush() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        // Store events directly (simulating buffering)
        beacon.store_event(AnalyticsEvent::new("event1"));
        beacon.store_event(AnalyticsEvent::new("event2"));
        
        assert_eq!(beacon.get_events().len(), 2);
        
        // Flush clears for InMemory (sends to provider, which re-stores)
        beacon.flush().await;
        
        // After flush, events should be processed (for InMemory, re-stored)
        assert!(beacon.get_events().len() >= 2);
    }

    #[tokio::test]
    async fn test_beacon_track_with_user() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        
        beacon.identify("user_123", HashMap::new());
        beacon.track("purchase", serde_json::json!({"item": "book"})).await;
        
        let events = beacon.get_events();
        // Should have identify + purchase events
        assert!(events.len() >= 2);
        
        // Find the purchase event and verify user_id
        let purchase = events.iter().find(|e| e.name == "purchase").unwrap();
        assert_eq!(purchase.user_id, Some("user_123".to_string()));
    }

    #[test]
    fn test_beacon_track_non_object_properties() {
        // Track with a non-object value should not panic
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        // This tests line 172 where we check if properties is an object
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            beacon.track("test", serde_json::json!("string_value")).await;
        });
        
        let events = beacon.get_events();
        assert_eq!(events.len(), 1);
        // Properties should be empty since input wasn't an object
        assert!(events[0].properties.is_empty());
    }

    #[test]
    fn test_from_env_plausible_with_host() {
        with_env_lock(|| {
            std::env::set_var("ANALYTICS_PROVIDER", "plausible:mysite.com");
            std::env::set_var("PLAUSIBLE_HOST", "https://custom.plausible.io");
            
            let beacon = Beacon::from_env();
            if let AnalyticsProvider::Plausible { domain, api_host } = beacon.provider {
                assert_eq!(domain, "mysite.com");
                assert_eq!(api_host, Some("https://custom.plausible.io".to_string()));
            } else {
                panic!("Expected Plausible provider");
            }
            
            std::env::remove_var("ANALYTICS_PROVIDER");
            std::env::remove_var("PLAUSIBLE_HOST");
        });
    }

    #[test]
    fn test_beacon_debug() {
        let beacon = Beacon::new(AnalyticsProvider::InMemory);
        let debug_str = format!("{:?}", beacon);
        assert!(debug_str.contains("Beacon"));
        assert!(debug_str.contains("InMemory"));
    }

    #[test]
    fn test_from_env_invalid_provider() {
        with_env_lock(|| {
            std::env::set_var("ANALYTICS_PROVIDER", "unknown_provider");
            let beacon = Beacon::from_env();
            // Invalid provider falls back to Disabled
            assert!(matches!(beacon.provider, AnalyticsProvider::Disabled));
            std::env::remove_var("ANALYTICS_PROVIDER");
        });
    }
}
