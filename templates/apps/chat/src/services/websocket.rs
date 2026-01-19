use async_trait::async_trait;
use nucleus_std::stream::{SocketMessage, StreamHandler, StreamHub, WebSocket};
use serde_json::Value;

pub struct ChatHandler;

#[async_trait]
impl StreamHandler for ChatHandler {
    async fn on_connect(&self, hub: &StreamHub, socket: &WebSocket) {
        // Auto-join "general" room for demo
        hub.join_room(&socket.id, "general").await.ok();
    }

    async fn on_message(&self, hub: &StreamHub, _socket: &WebSocket, msg: SocketMessage) {
        if let Some(json) = msg.as_json::<Value>() {
            // Check if it has a room, otherwise default to "general"
            let room = json
                .get("room")
                .and_then(|r: &Value| r.as_str())
                .unwrap_or("general");

            // Broadcast to the room
            hub.broadcast_to_room(room, SocketMessage::json(&json).unwrap())
                .await
                .ok();
        } else if let SocketMessage::Text(text) = msg {
            // Simple text broadcast
            hub.broadcast_to_room("general", SocketMessage::Text(text))
                .await
                .ok();
        }
    }

    async fn on_disconnect(&self, _hub: &StreamHub, _client_id: &str) {
        // Cleanup if needed
    }
}
