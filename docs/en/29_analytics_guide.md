# Analytics Guide

Track user behavior with Beacon - a privacy-first analytics module.

---

## Quick Start

```rust
use nucleus_std::beacon::{Beacon, AnalyticsProvider};

// Initialize with a provider
let beacon = Beacon::new(AnalyticsProvider::InMemory);

// Track events
beacon.track("button_click", serde_json::json!({
    "button_id": "signup",
    "page": "/pricing"
})).await;

// Track page views
beacon.page_view("/dashboard").await;
```

---

## Providers

Beacon supports multiple analytics backends:

| Provider | Use Case |
|----------|----------|
| `InMemory` | Testing and development |
| `File { path }` | Local file logging (JSONL) |
| `Webhook { url }` | POST events to any endpoint |
| `Plausible { domain }` | Privacy-friendly SaaS |
| `Disabled` | No-op for production opt-out |

### Environment Configuration

```bash
# Disable analytics
ANALYTICS_PROVIDER=disabled

# In-memory (testing)
ANALYTICS_PROVIDER=memory

# File logging
ANALYTICS_PROVIDER=file:/var/log/analytics.jsonl

# Webhook
ANALYTICS_PROVIDER=webhook:https://analytics.example.com/events

# Plausible
ANALYTICS_PROVIDER=plausible:mysite.com
PLAUSIBLE_HOST=https://plausible.io  # Optional custom host
```

```rust
// Auto-configure from environment
let beacon = Beacon::from_env();
```

---

## Core API

### Identify Users

```rust
let mut traits = HashMap::new();
traits.insert("email".to_string(), json!("user@example.com"));
traits.insert("plan".to_string(), json!("pro"));

beacon.identify("user_123", traits);
```

### Track Events

```rust
// Simple event
beacon.track("purchase", json!({
    "product_id": "abc123",
    "price": 29.99
})).await;

// Click tracking
beacon.click("buy_button").await;

// Error tracking
beacon.error("Payment failed", Some(json!({
    "error_code": "card_declined"
}))).await;
```

### Flush Events

```rust
// Force send all buffered events
beacon.flush().await;
```

---

## Testing

```rust
#[tokio::test]
async fn test_analytics() {
    let beacon = Beacon::new(AnalyticsProvider::InMemory);
    
    beacon.track("test_event", json!({})).await;
    
    let events = beacon.get_events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "test_event");
    
    beacon.clear_events();
}
```

---

## Privacy Considerations

- **No PII by default**: Beacon doesn't collect personal data unless you explicitly track it
- **Plausible integration**: Uses a privacy-respecting analytics platform
- **Self-hosted option**: Use file or webhook providers for complete control
- **GDPR-friendly**: Easy to implement consent flows with `Disabled` provider
