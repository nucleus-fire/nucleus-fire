# WebSocket Rooms Guide

Nucleus Stream provides production-ready WebSocket handling with rooms, broadcast, and presence tracking.

## Quick Start

```rust
use nucleus_std::stream::{StreamHub, WebSocket, SocketMessage};

let hub = StreamHub::new();

// On client connect
hub.register(socket.clone()).await;

// Join a chat room
hub.join_room(&socket.id, "general").await?;

// Broadcast message to room
hub.broadcast_to_room("general", SocketMessage::text("Hello everyone!")).await?;

// Send private message
hub.send_to(&target_socket_id, SocketMessage::text("Private message")).await?;

// On disconnect
hub.unregister(&socket.id).await;
```

## Core Concepts

### StreamHub

The central hub managing all connections and rooms:

```rust
let hub = StreamHub::new();

// Hub is Clone - share across handlers
let hub_clone = hub.clone(); // Same underlying state
```

### WebSocket Handle

Represents a connected client:

```rust
// Anonymous socket
let socket = WebSocket::new(id, sender);

// Authenticated socket
let socket = WebSocket::authenticated(id, "user_123".to_string(), sender);

// With metadata
let socket = socket.with_metadata("ip", "192.168.1.1");
```

### Messages

```rust
// Text message
let msg = SocketMessage::text("Hello");

// JSON message
let msg = SocketMessage::json(&MyData { value: 42 })?;

// Binary data
let msg = SocketMessage::Binary(vec![0, 1, 2, 3]);

// Parse JSON from message
let data: MyData = message.as_json().unwrap();
```

## Room Management

### Join/Leave

```rust
// Join a room (creates if doesn't exist)
hub.join_room(&socket.id, "room_name").await?;

// Leave a room
hub.leave_room(&socket.id, "room_name").await?;

// Unregister leaves all rooms automatically
hub.unregister(&socket.id).await;
```

### Room Information

```rust
// Get room members
let members = hub.get_room_members("room_name").await;

// Get rooms a socket is in
let rooms = hub.get_socket_rooms(&socket.id).await;

// List all rooms
let all_rooms = hub.list_rooms().await;

// Get room details
if let Some(room) = hub.get_room("room_name").await {
    println!("Members: {}", room.size());
}
```

## Messaging

### Direct Messages

```rust
// Send to specific socket
hub.send_to(&socket_id, SocketMessage::text("Hello")).await?;
```

### Room Broadcast

```rust
// To all in room
hub.broadcast_to_room("room", SocketMessage::text("Hello")).await?;

// To all except sender
hub.broadcast_to_room_except("room", msg, &sender_id).await?;
```

### Global Broadcast

```rust
// To all connected sockets
let sent = hub.broadcast_all(SocketMessage::text("Server message")).await;
println!("Sent to {} clients", sent);
```

### User Messaging

```rust
// Send to all sockets of a user (multi-device)
hub.send_to_user("user_123", SocketMessage::text("Notification")).await?;
```

## Presence Tracking

```rust
// Check if user is online
if hub.is_user_online("user_123").await {
    println!("User is connected");
}

// Get all online users
let online = hub.online_users().await;
println!("{} users online", online.len());

// Get connection counts
let total = hub.connection_count().await;
let users = hub.online_user_count().await;
```

## Implementing a Handler

```rust
use nucleus_std::stream::{StreamHandler, StreamHub, WebSocket, SocketMessage};

struct ChatHandler;

#[async_trait]
impl StreamHandler for ChatHandler {
    async fn on_connect(&self, hub: &StreamHub, socket: &WebSocket) {
        println!("Connected: {}", socket.id);
        hub.join_room(&socket.id, "lobby").await.ok();
    }
    
    async fn on_message(&self, hub: &StreamHub, socket: &WebSocket, msg: SocketMessage) {
        if let SocketMessage::Text(text) = msg {
            // Echo to room
            hub.broadcast_to_room_except("lobby", SocketMessage::text(&text), &socket.id).await.ok();
        }
    }
    
    async fn on_disconnect(&self, hub: &StreamHub, socket_id: &str) {
        println!("Disconnected: {}", socket_id);
    }
}
```

## Chat Room Example

```rust
struct ChatRoom {
    hub: StreamHub,
}

impl ChatRoom {
    async fn handle_message(&self, socket: &WebSocket, text: &str) {
        #[derive(Deserialize)]
        struct ChatMessage {
            action: String,
            room: Option<String>,
            text: Option<String>,
        }
        
        let msg: ChatMessage = serde_json::from_str(text).unwrap();
        
        match msg.action.as_str() {
            "join" => {
                let room = msg.room.unwrap();
                self.hub.join_room(&socket.id, &room).await.ok();
                self.hub.broadcast_to_room(&room, SocketMessage::text("User joined")).await.ok();
            }
            "leave" => {
                let room = msg.room.unwrap();
                self.hub.broadcast_to_room(&room, SocketMessage::text("User left")).await.ok();
                self.hub.leave_room(&socket.id, &room).await.ok();
            }
            "message" => {
                let room = msg.room.unwrap();
                let text = msg.text.unwrap();
                self.hub.broadcast_to_room(&room, SocketMessage::text(&text)).await.ok();
            }
            _ => {}
        }
    }
}
```

## Error Handling

```rust
use nucleus_std::stream::StreamError;

match hub.send_to(&socket_id, msg).await {
    Ok(()) => println!("Sent"),
    Err(StreamError::SocketNotFound(id)) => println!("Socket {} disconnected", id),
    Err(StreamError::SendFailed(id)) => println!("Failed to send to {}", id),
    Err(e) => println!("Error: {}", e),
}
```

## Best Practices

1. **Use rooms** to group related connections
2. **Authenticate sockets** for user-specific features
3. **Handle errors** gracefully (sockets can disconnect anytime)
4. **Use presence** for online indicators
5. **Prefer broadcast_except** to avoid echo to sender
