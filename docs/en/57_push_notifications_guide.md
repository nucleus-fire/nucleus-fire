# Push Notifications Guide

Send push notifications to mobile and web apps using Firebase FCM or OneSignal.

## Quick Start

```rust
use nucleus_std::push::{Push, PushMessage};

// Initialize with OneSignal
let push = Push::onesignal("your_app_id", "your_api_key");

// Send a notification
push.send(PushMessage::new("You have a new message!")
    .title("New Message")
    .to_token("device_token_here"))
    .await?;
```

## Backends

### Firebase FCM

```rust
let credentials = r#"{
    "project_id": "your-project",
    "private_key_id": "key123",
    "private_key": "-----BEGIN RSA PRIVATE KEY-----...",
    "client_email": "firebase@your-project.iam.gserviceaccount.com"
}"#;

let push = Push::firebase(credentials).await?;
```

### OneSignal

```rust
let push = Push::onesignal("app_id", "rest_api_key");
```

## Building Messages

### Basic Message

```rust
let message = PushMessage::new("Hello, World!");
```

### Full Message Configuration

```rust
let message = PushMessage::new("Your order has shipped!")
    .title("Order Update")
    .icon("/images/shipping.png")
    .image("/images/package.jpg")
    .click_action("https://myapp.com/orders/123")
    .to_token("device_token");
```

### Adding Custom Data

```rust
let message = PushMessage::new("New sale!")
    .title("Flash Sale")
    .data_field("discount", 20)
    .data_field("category", "electronics")
    .data_field("expires_at", "2024-12-31T23:59:59Z")
    .to_token("device_token");

// Or bulk data
let message = PushMessage::new("New sale!")
    .data(json!({
        "discount": 20,
        "category": "electronics",
        "product_id": "abc123"
    }))
    .to_token("device_token");
```

### Priority and TTL

```rust
let message = PushMessage::new("Urgent alert!")
    .high_priority()
    .ttl(3600)  // Expire after 1 hour
    .to_token("device_token");
```

## Targeting

### Single Device

```rust
let message = PushMessage::new("Hello!")
    .to_token("device_registration_token");

push.send(message).await?;
```

### Topic/Segment

```rust
// Send to topic
push.send_to_topic("breaking_news", 
    PushMessage::new("Breaking: Major event!").title("Breaking News")
).await?;

// Or set topic in message
let message = PushMessage::new("Weekly digest")
    .to_topic("weekly_digest");
push.send(message).await?;
```

### Multiple Devices

```rust
let messages: Vec<PushMessage> = device_tokens.iter()
    .map(|token| {
        PushMessage::new("Batch notification")
            .to_token(token)
    })
    .collect();

let result = push.send_batch(&messages).await?;
println!("Sent: {}, Failed: {}", result.success_count, result.failure_count);
```

## Platform-Specific Configuration

### Android

```rust
use nucleus_std::push::AndroidConfig;

let android = AndroidConfig::new()
    .channel_id("high_priority_channel")
    .color("#FF5722")
    .sound("notification.wav");

let message = PushMessage::new("Android notification")
    .title("Alert")
    .android(android)
    .to_token("android_token");
```

### iOS

```rust
use nucleus_std::push::IosConfig;

let ios = IosConfig::new()
    .badge(5)
    .sound("ping.aiff")
    .category("MESSAGE_CATEGORY");

let message = PushMessage::new("iOS notification")
    .title("Alert")
    .ios(ios)
    .to_token("ios_token");
```

### Web

```rust
use nucleus_std::push::WebConfig;

let web = WebConfig::new()
    .icon("/icon-192.png")
    .badge("/badge.png")
    .action("reply", "Reply")
    .action("dismiss", "Dismiss");

let message = PushMessage::new("Web notification")
    .title("Alert")
    .web(web)
    .to_token("web_push_subscription");
```

## Topic Subscriptions

### Subscribe Device

```rust
push.subscribe("device_token", "promotions").await?;
push.subscribe("device_token", "news").await?;
```

### Unsubscribe Device

```rust
push.unsubscribe("device_token", "promotions").await?;
```

## Handling Results

### Single Send

```rust
let result = push.send(message).await?;

if result.success {
    println!("Sent! Message ID: {:?}", result.message_id);
} else {
    println!("Failed: {:?}", result.error);
}
```

### Batch Send

```rust
let batch_result = push.send_batch(&messages).await?;

println!("Success: {}", batch_result.success_count);
println!("Failed: {}", batch_result.failure_count);

for (i, result) in batch_result.results.iter().enumerate() {
    if !result.success {
        println!("Message {} failed: {:?}", i, result.error);
    }
}
```

## Error Handling

```rust
use nucleus_std::push::PushError;

match push.send(message).await {
    Ok(result) if result.success => println!("Sent!"),
    Ok(result) => println!("Failed: {:?}", result.error),
    Err(PushError::AuthError(msg)) => {
        println!("Authentication failed: {}", msg);
        // Refresh credentials
    }
    Err(PushError::InvalidToken(token)) => {
        println!("Invalid token: {}", token);
        // Remove token from database
    }
    Err(PushError::DeviceNotRegistered) => {
        println!("Device unregistered");
        // Remove token from database
    }
    Err(PushError::RateLimited) => {
        println!("Rate limited, retry later");
    }
    Err(PushError::MessageTooLarge) => {
        println!("Message too large, reduce payload");
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Complete Example

```rust
use nucleus_std::push::{Push, PushMessage, PushError, AndroidConfig, IosConfig};

async fn notify_user(
    push: &Push,
    user: &User,
    title: &str,
    body: &str,
    data: serde_json::Value,
) -> Result<(), PushError> {
    // Build base message
    let mut message = PushMessage::new(body)
        .title(title)
        .data(data)
        .click_action(&format!("https://app.com/users/{}", user.id));
    
    // Add platform configs
    message = message
        .android(AndroidConfig::new().channel_id("default"))
        .ios(IosConfig::new().badge(user.unread_count));
    
    // Send to all user devices
    let mut results = Vec::new();
    for token in &user.device_tokens {
        let msg = message.clone().to_token(token);
        results.push(push.send(msg).await?);
    }
    
    // Handle failed tokens
    for (i, result) in results.iter().enumerate() {
        if !result.success {
            if let Some(error) = &result.error {
                if error.contains("NotRegistered") {
                    remove_token(&user.device_tokens[i]).await;
                }
            }
        }
    }
    
    Ok(())
}

async fn broadcast_announcement(push: &Push, announcement: &Announcement) -> Result<(), PushError> {
    let message = PushMessage::new(&announcement.summary)
        .title(&announcement.title)
        .image(&announcement.image_url)
        .high_priority()
        .data(json!({
            "announcement_id": announcement.id,
            "type": "announcement"
        }));
    
    // Send to all subscribers
    push.send_to_topic("announcements", message).await?;
    
    Ok(())
}
```
