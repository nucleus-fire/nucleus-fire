# WebSocket & Real-Time Guide

Real-time communication is essential for modern web applications. Nucleus provides the **Stream** module for WebSocket support.

## Overview

The Stream module enables:
- Real-time bidirectional communication
- Channel-based message broadcasting
- Connection state management
- Automatic reconnection handling

## Quick Example

```rust
use nucleus_std::stream::{StreamHandler, StreamHub, WebSocket, SocketMessage};

pub struct ChatHandler;

#[async_trait]
impl StreamHandler for ChatHandler {
    async fn on_message(&self, hub: &StreamHub, socket: &WebSocket, msg: SocketMessage) {
        if let SocketMessage::Text(text) = msg {
            // Broadcast to "general" room
            hub.broadcast_to_room("general", SocketMessage::Text(text)).await.ok();
        }
    }

    async fn on_connect(&self, hub: &StreamHub, socket: &WebSocket) {
        hub.join_room(&socket.id, "general").await.ok();
    }
    
    async fn on_disconnect(&self, _hub: &StreamHub, _socket_id: &str) {}
}
```

## Setting Up WebSockets

### 1. Create a Handler

In your view file, define a WebSocket channel handler:

```rust
use nucleus_std::stream::{StreamHandler, StreamHub, WebSocket, SocketMessage};
use serde_json::Value;

pub struct ChatHandler;

#[async_trait]
impl StreamHandler for ChatHandler {
    async fn on_connect(&self, hub: &StreamHub, socket: &WebSocket) {
        println!("Client connected: {}", socket.id);
        hub.join_room(&socket.id, "chat").await.ok();
    }

    async fn on_message(&self, hub: &StreamHub, socket: &WebSocket, msg: SocketMessage) {
        if let Option::Some(data) = msg.as_json::<Value>() {
            // Broadcast json to room
            hub.broadcast_to_room("chat", SocketMessage::json(&data).unwrap()).await.ok();
        }
    }
    
    async fn on_disconnect(&self, hub: &StreamHub, client_id: &str) {
        println!("Client disconnected: {}", client_id);
    }
}
```

### 2. Client-Side Connection

Connect from JavaScript:

```javascript
// Connect to WebSocket
const ws = new WebSocket('ws://localhost:3000/ws/chat');

ws.onopen = () => {
    console.log('Connected!');
    ws.send(JSON.stringify({ type: 'join', room: 'general' }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Received:', data);
};

ws.onclose = () => {
    console.log('Disconnected');
};
```

### 3. Using n:client for Reactive Updates

```html
<n:client>
    use nucleus_std::stream::subscribe;
    
    let messages = Signal::new(Vec::new());
    
    // Subscribe to channel
    subscribe("chat", move |msg| {
        messages.modify(|m| m.push(msg));
    });
</n:client>

<div id="messages">
    <n:for item="msg" in="messages">
        <div class="message">{msg.text}</div>
    </n:for>
</div>
```

## StreamHub API Reference

| Method | Description |
|--------|-------------|
| `hub.join_room(socket_id, room)` | Add socket to a room |
| `hub.leave_room(socket_id, room)` | Remove socket from room |
| `hub.broadcast_to_room(room, msg)` | Send to all in room |
| `hub.send_to(socket_id, msg)` | Send to specific socket |
| `hub.send_to_user(user_id, msg)` | Send to all sockets for user |
| `hub.online_users()` | Get list of online user IDs |

## Common Patterns

### Chat Room

```rust
// In StreamHandler implementation
async fn on_message(&self, hub: &StreamHub, socket: &WebSocket, msg: SocketMessage) {
    if let SocketMessage::Text(text) = msg {
        let room = format!("room:{}", "lobby"); // dynamic logic here
        
        let response = json!({
            "user": socket.user_id.clone().unwrap_or("anon".into()),
            "text": text,
            "timestamp": Utc::now()
        });
        
        hub.broadcast_to_room(&room, SocketMessage::json(&response).unwrap()).await.ok();
    }
}
```

### Live Notifications

```rust
// Send notification to a specific user (across all their devices)
async fn send_notification(hub: &StreamHub, user_id: &str, notification: Notification) {
    hub.send_to_user(
        user_id, 
        SocketMessage::json(&notification).unwrap()
    ).await.ok();
}
```

### Live Dashboard

```rust
// Server sends updates every second using StreamHub
use nucleus_std::stream::{StreamHub, SocketMessage};
use std::sync::Arc;

async fn start_metrics_stream(hub: Arc<StreamHub>) {
    loop {
        let metrics = get_system_metrics().await;
        hub.broadcast_to_room("metrics", SocketMessage::json(&metrics).unwrap())
            .await.ok();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
```

### Authenticated WebSocket Connection

```rust
async fn on_connect(&self, hub: &StreamHub, socket: &WebSocket) {
    // Validate token from query string
    if let Some(token) = socket.query.get("token") {
        let secret = std::env::var("SECRET_KEY").unwrap_or_default();
        if let Some(user_id) = Fortress::extract_user_id(token, &secret) {
            // Authenticated - join user-specific room
            hub.join_room(&socket.id, &format!("user:{}", user_id)).await.ok();
            return;
        }
    }
    // Reject unauthenticated connection
    hub.disconnect(&socket.id).await.ok();
}
```

## Error Handling

```rust
match channel.broadcast("event", data).await {
    Ok(_) => println!("Message sent"),
    Err(e) => eprintln!("Failed to send: {}", e),
}
```

## Best Practices

1. **Use Channels for Organization** - Group related messages by channel
2. **Handle Disconnections** - Implement `on_disconnect` to clean up state
3. **Validate Messages** - Always validate incoming data before processing
4. **Rate Limiting** - Implement rate limiting for public channels
5. **Authentication** - Require authentication for sensitive channels

## Related Guides

- [API Development](#22_api_development) - REST endpoints
- [State Management](#15_state_management) - Reactive updates
- [Authentication](#21_authentication_guide) - Securing connections
