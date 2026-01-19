//! Nucleus Stream - WebSocket Rooms & Broadcast
//!
//! Production-ready WebSocket handling with:
//! - Room management (join/leave/broadcast)
//! - Presence tracking (who's online)
//! - Direct messaging between sockets
//! - Connection state management
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::stream::{StreamHub, WebSocket, SocketMessage};
//!
//! let hub = StreamHub::new();
//!
//! // On connection
//! hub.register(socket.clone()).await;
//! hub.join_room(&socket.id, "general").await;
//!
//! // Broadcast to room
//! hub.broadcast_to_room("general", SocketMessage::Text("Hello!".into())).await;
//!
//! // Direct message
//! hub.send_to(&target_id, SocketMessage::Text("Private".into())).await;
//!
//! // On disconnect
//! hub.unregister(&socket.id).await;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════
// MESSAGE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessage {
    /// Text message
    Text(String),
    /// Binary data
    Binary(Vec<u8>),
    /// Ping frame
    Ping(Vec<u8>),
    /// Pong frame
    Pong(Vec<u8>),
    /// Close frame
    Close,
    /// JSON payload (convenience wrapper)
    Json(serde_json::Value),
}

impl SocketMessage {
    /// Create a text message
    pub fn text(s: &str) -> Self {
        SocketMessage::Text(s.to_string())
    }

    /// Create a JSON message
    pub fn json<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        Ok(SocketMessage::Json(serde_json::to_value(value)?))
    }

    /// Try to parse as JSON
    pub fn as_json<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        match self {
            SocketMessage::Text(s) => serde_json::from_str(s).ok(),
            SocketMessage::Json(v) => serde_json::from_value(v.clone()).ok(),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WEBSOCKET HANDLE
// ═══════════════════════════════════════════════════════════════════════════

/// Represents an active WebSocket connection
#[derive(Clone)]
pub struct WebSocket {
    /// Unique connection ID
    pub id: String,
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    /// Connection metadata
    pub metadata: HashMap<String, String>,
    /// Message sender
    sender: mpsc::Sender<SocketMessage>,
}

impl WebSocket {
    /// Create a new WebSocket handle
    pub fn new(id: String, sender: mpsc::Sender<SocketMessage>) -> Self {
        Self {
            id,
            user_id: None,
            metadata: HashMap::new(),
            sender,
        }
    }

    /// Create with user ID (authenticated connection)
    pub fn authenticated(id: String, user_id: String, sender: mpsc::Sender<SocketMessage>) -> Self {
        Self {
            id,
            user_id: Some(user_id),
            metadata: HashMap::new(),
            sender,
        }
    }

    /// Set metadata value
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Send a message to this socket
    pub async fn send(&self, msg: SocketMessage) -> Result<(), StreamError> {
        self.sender
            .send(msg)
            .await
            .map_err(|_| StreamError::SendFailed(self.id.clone()))
    }

    /// Send text message
    pub async fn send_text(&self, text: &str) -> Result<(), StreamError> {
        self.send(SocketMessage::Text(text.to_string())).await
    }

    /// Send JSON message
    pub async fn send_json<T: Serialize>(&self, value: &T) -> Result<(), StreamError> {
        let msg = SocketMessage::json(value)
            .map_err(|e| StreamError::SerializationError(e.to_string()))?;
        self.send(msg).await
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Stream error types
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("Socket not found: {0}")]
    SocketNotFound(String),

    #[error("Room not found: {0}")]
    RoomNotFound(String),

    #[error("Failed to send to socket: {0}")]
    SendFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Already in room: {0}")]
    AlreadyInRoom(String),

    #[error("Not in room: {0}")]
    NotInRoom(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// ROOM
// ═══════════════════════════════════════════════════════════════════════════

/// A room containing multiple sockets
#[derive(Debug, Clone)]
pub struct Room {
    /// Room name/ID
    pub name: String,
    /// Socket IDs in this room
    pub members: HashSet<String>,
    /// Room metadata
    pub metadata: HashMap<String, String>,
}

impl Room {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            members: HashSet::new(),
            metadata: HashMap::new(),
        }
    }

    /// Number of members
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// STREAM HUB
// ═══════════════════════════════════════════════════════════════════════════

/// Central hub for managing WebSocket connections and rooms
pub struct StreamHub {
    /// All connected sockets
    sockets: Arc<RwLock<HashMap<String, WebSocket>>>,
    /// All rooms
    rooms: Arc<RwLock<HashMap<String, Room>>>,
    /// Socket -> Rooms mapping (which rooms each socket is in)
    socket_rooms: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// User -> Socket mapping (for authenticated users)
    user_sockets: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

impl StreamHub {
    /// Create a new stream hub
    pub fn new() -> Self {
        Self {
            sockets: Arc::new(RwLock::new(HashMap::new())),
            rooms: Arc::new(RwLock::new(HashMap::new())),
            socket_rooms: Arc::new(RwLock::new(HashMap::new())),
            user_sockets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CONNECTION MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════

    /// Register a new socket connection
    pub async fn register(&self, socket: WebSocket) {
        let socket_id = socket.id.clone();
        let user_id = socket.user_id.clone();

        // Add to sockets
        {
            let mut sockets = self.sockets.write().await;
            sockets.insert(socket_id.clone(), socket);
        }

        // Initialize socket rooms
        {
            let mut socket_rooms = self.socket_rooms.write().await;
            socket_rooms.insert(socket_id.clone(), HashSet::new());
        }

        // Track user socket if authenticated
        if let Some(user_id) = user_id {
            let mut user_sockets = self.user_sockets.write().await;
            user_sockets.entry(user_id).or_default().insert(socket_id);
        }
    }

    /// Unregister a socket (removes from all rooms)
    pub async fn unregister(&self, socket_id: &str) {
        // Get socket's rooms before removing
        let rooms_to_leave: Vec<String> = {
            let socket_rooms = self.socket_rooms.read().await;
            socket_rooms
                .get(socket_id)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect()
        };

        // Leave all rooms
        for room in rooms_to_leave {
            self.leave_room(socket_id, &room).await.ok();
        }

        // Remove from user sockets
        {
            let sockets = self.sockets.read().await;
            if let Some(socket) = sockets.get(socket_id) {
                if let Some(ref user_id) = socket.user_id {
                    let mut user_sockets = self.user_sockets.write().await;
                    if let Some(set) = user_sockets.get_mut(user_id) {
                        set.remove(socket_id);
                        if set.is_empty() {
                            user_sockets.remove(user_id);
                        }
                    }
                }
            }
        }

        // Remove socket
        {
            let mut sockets = self.sockets.write().await;
            sockets.remove(socket_id);
        }

        // Remove socket rooms entry
        {
            let mut socket_rooms = self.socket_rooms.write().await;
            socket_rooms.remove(socket_id);
        }
    }

    /// Get socket by ID
    pub async fn get_socket(&self, socket_id: &str) -> Option<WebSocket> {
        let sockets = self.sockets.read().await;
        sockets.get(socket_id).cloned()
    }

    /// Get all connected socket IDs
    pub async fn connected_sockets(&self) -> Vec<String> {
        let sockets = self.sockets.read().await;
        sockets.keys().cloned().collect()
    }

    /// Get total connection count
    pub async fn connection_count(&self) -> usize {
        let sockets = self.sockets.read().await;
        sockets.len()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ROOM MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════

    /// Join a room (creates if doesn't exist)
    pub async fn join_room(&self, socket_id: &str, room_name: &str) -> Result<(), StreamError> {
        // Verify socket exists
        {
            let sockets = self.sockets.read().await;
            if !sockets.contains_key(socket_id) {
                return Err(StreamError::SocketNotFound(socket_id.to_string()));
            }
        }

        // Add to room
        {
            let mut rooms = self.rooms.write().await;
            let room = rooms
                .entry(room_name.to_string())
                .or_insert_with(|| Room::new(room_name));
            room.members.insert(socket_id.to_string());
        }

        // Track socket's rooms
        {
            let mut socket_rooms = self.socket_rooms.write().await;
            if let Some(set) = socket_rooms.get_mut(socket_id) {
                set.insert(room_name.to_string());
            }
        }

        Ok(())
    }

    /// Leave a room
    pub async fn leave_room(&self, socket_id: &str, room_name: &str) -> Result<(), StreamError> {
        // Remove from room
        {
            let mut rooms = self.rooms.write().await;
            if let Some(room) = rooms.get_mut(room_name) {
                room.members.remove(socket_id);

                // Clean up empty rooms
                if room.is_empty() {
                    rooms.remove(room_name);
                }
            }
        }

        // Update socket's rooms
        {
            let mut socket_rooms = self.socket_rooms.write().await;
            if let Some(set) = socket_rooms.get_mut(socket_id) {
                set.remove(room_name);
            }
        }

        Ok(())
    }

    /// Get room by name
    pub async fn get_room(&self, room_name: &str) -> Option<Room> {
        let rooms = self.rooms.read().await;
        rooms.get(room_name).cloned()
    }

    /// Get all rooms a socket is in
    pub async fn get_socket_rooms(&self, socket_id: &str) -> Vec<String> {
        let socket_rooms = self.socket_rooms.read().await;
        socket_rooms
            .get(socket_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    /// Get members of a room
    pub async fn get_room_members(&self, room_name: &str) -> Vec<String> {
        let rooms = self.rooms.read().await;
        rooms
            .get(room_name)
            .map(|r| r.members.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all room names
    pub async fn list_rooms(&self) -> Vec<String> {
        let rooms = self.rooms.read().await;
        rooms.keys().cloned().collect()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MESSAGING
    // ═══════════════════════════════════════════════════════════════════════

    /// Send to a specific socket
    pub async fn send_to(&self, socket_id: &str, msg: SocketMessage) -> Result<(), StreamError> {
        let sockets = self.sockets.read().await;
        let socket = sockets
            .get(socket_id)
            .ok_or_else(|| StreamError::SocketNotFound(socket_id.to_string()))?;

        socket.send(msg).await
    }

    /// Broadcast to all sockets in a room
    pub async fn broadcast_to_room(
        &self,
        room_name: &str,
        msg: SocketMessage,
    ) -> Result<usize, StreamError> {
        let members = self.get_room_members(room_name).await;
        let mut sent = 0;

        for socket_id in members {
            if self.send_to(&socket_id, msg.clone()).await.is_ok() {
                sent += 1;
            }
        }

        Ok(sent)
    }

    /// Broadcast to all sockets in a room except sender
    pub async fn broadcast_to_room_except(
        &self,
        room_name: &str,
        msg: SocketMessage,
        except_id: &str,
    ) -> Result<usize, StreamError> {
        let members = self.get_room_members(room_name).await;
        let mut sent = 0;

        for socket_id in members {
            if socket_id != except_id && self.send_to(&socket_id, msg.clone()).await.is_ok() {
                sent += 1;
            }
        }

        Ok(sent)
    }

    /// Broadcast to all connected sockets
    pub async fn broadcast_all(&self, msg: SocketMessage) -> usize {
        let socket_ids = self.connected_sockets().await;
        let mut sent = 0;

        for socket_id in socket_ids {
            if self.send_to(&socket_id, msg.clone()).await.is_ok() {
                sent += 1;
            }
        }

        sent
    }

    /// Send to all sockets of a user
    pub async fn send_to_user(
        &self,
        user_id: &str,
        msg: SocketMessage,
    ) -> Result<usize, StreamError> {
        let user_sockets = self.user_sockets.read().await;
        let socket_ids = user_sockets.get(user_id).cloned().unwrap_or_default();

        let mut sent = 0;
        for socket_id in socket_ids {
            if self.send_to(&socket_id, msg.clone()).await.is_ok() {
                sent += 1;
            }
        }

        Ok(sent)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRESENCE
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if a user is online
    pub async fn is_user_online(&self, user_id: &str) -> bool {
        let user_sockets = self.user_sockets.read().await;
        user_sockets
            .get(user_id)
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }

    /// Get all online user IDs
    pub async fn online_users(&self) -> Vec<String> {
        let user_sockets = self.user_sockets.read().await;
        user_sockets.keys().cloned().collect()
    }

    /// Get online user count
    pub async fn online_user_count(&self) -> usize {
        let user_sockets = self.user_sockets.read().await;
        user_sockets.len()
    }
}

impl Default for StreamHub {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StreamHub {
    fn clone(&self) -> Self {
        Self {
            sockets: Arc::clone(&self.sockets),
            rooms: Arc::clone(&self.rooms),
            socket_rooms: Arc::clone(&self.socket_rooms),
            user_sockets: Arc::clone(&self.user_sockets),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLER TRAIT
// ═══════════════════════════════════════════════════════════════════════════

/// Trait for WebSocket event handling
#[async_trait::async_trait]
pub trait StreamHandler: Send + Sync + 'static {
    /// Called when a socket connects
    async fn on_connect(&self, hub: &StreamHub, socket: &WebSocket);

    /// Called when a message is received
    async fn on_message(&self, hub: &StreamHub, socket: &WebSocket, msg: SocketMessage);

    /// Called when a socket disconnects
    async fn on_disconnect(&self, hub: &StreamHub, socket_id: &str);
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn create_socket(id: &str) -> (WebSocket, mpsc::Receiver<SocketMessage>) {
        let (tx, rx) = mpsc::channel(32);
        (WebSocket::new(id.to_string(), tx), rx)
    }

    fn create_authenticated_socket(
        id: &str,
        user_id: &str,
    ) -> (WebSocket, mpsc::Receiver<SocketMessage>) {
        let (tx, rx) = mpsc::channel(32);
        (
            WebSocket::authenticated(id.to_string(), user_id.to_string(), tx),
            rx,
        )
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MESSAGE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_socket_message_text() {
        let msg = SocketMessage::text("hello");
        assert!(matches!(msg, SocketMessage::Text(s) if s == "hello"));
    }

    #[test]
    fn test_socket_message_json() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Data {
            value: i32,
        }

        let msg = SocketMessage::json(&Data { value: 42 }).unwrap();
        let parsed: Data = msg.as_json().unwrap();
        assert_eq!(parsed.value, 42);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // WEBSOCKET TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_websocket_send() {
        let (socket, mut rx) = create_socket("sock1");

        socket.send_text("hello").await.unwrap();

        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, SocketMessage::Text(s) if s == "hello"));
    }

    #[tokio::test]
    async fn test_websocket_metadata() {
        let (socket, _rx) = create_socket("sock1");
        let socket = socket.with_metadata("ip", "127.0.0.1");

        assert_eq!(socket.metadata.get("ip"), Some(&"127.0.0.1".to_string()));
    }

    #[test]
    fn test_websocket_authenticated() {
        let (socket, _) = create_authenticated_socket("sock1", "user123");

        assert!(socket.is_authenticated());
        assert_eq!(socket.user_id, Some("user123".to_string()));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // HUB REGISTRATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_hub_register() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_socket("sock1");

        hub.register(socket).await;

        assert_eq!(hub.connection_count().await, 1);
        assert!(hub.get_socket("sock1").await.is_some());
    }

    #[tokio::test]
    async fn test_hub_unregister() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_socket("sock1");

        hub.register(socket).await;
        hub.unregister("sock1").await;

        assert_eq!(hub.connection_count().await, 0);
        assert!(hub.get_socket("sock1").await.is_none());
    }

    #[tokio::test]
    async fn test_hub_connected_sockets() {
        let hub = StreamHub::new();
        let (s1, _) = create_socket("sock1");
        let (s2, _) = create_socket("sock2");

        hub.register(s1).await;
        hub.register(s2).await;

        let connected = hub.connected_sockets().await;
        assert_eq!(connected.len(), 2);
        assert!(connected.contains(&"sock1".to_string()));
        assert!(connected.contains(&"sock2".to_string()));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ROOM TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_join_room() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_socket("sock1");

        hub.register(socket).await;
        hub.join_room("sock1", "general").await.unwrap();

        let room = hub.get_room("general").await.unwrap();
        assert!(room.members.contains("sock1"));
    }

    #[tokio::test]
    async fn test_leave_room() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_socket("sock1");

        hub.register(socket).await;
        hub.join_room("sock1", "general").await.unwrap();
        hub.leave_room("sock1", "general").await.unwrap();

        // Room should be removed when empty
        assert!(hub.get_room("general").await.is_none());
    }

    #[tokio::test]
    async fn test_unregister_leaves_rooms() {
        let hub = StreamHub::new();
        let (s1, _) = create_socket("sock1");
        let (s2, _) = create_socket("sock2");

        hub.register(s1).await;
        hub.register(s2).await;
        hub.join_room("sock1", "room").await.unwrap();
        hub.join_room("sock2", "room").await.unwrap();

        hub.unregister("sock1").await;

        let room = hub.get_room("room").await.unwrap();
        assert!(!room.members.contains("sock1"));
        assert!(room.members.contains("sock2"));
    }

    #[tokio::test]
    async fn test_socket_rooms() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_socket("sock1");

        hub.register(socket).await;
        hub.join_room("sock1", "room1").await.unwrap();
        hub.join_room("sock1", "room2").await.unwrap();

        let rooms = hub.get_socket_rooms("sock1").await;
        assert_eq!(rooms.len(), 2);
    }

    #[tokio::test]
    async fn test_room_members() {
        let hub = StreamHub::new();
        let (s1, _) = create_socket("sock1");
        let (s2, _) = create_socket("sock2");

        hub.register(s1).await;
        hub.register(s2).await;
        hub.join_room("sock1", "room").await.unwrap();
        hub.join_room("sock2", "room").await.unwrap();

        let members = hub.get_room_members("room").await;
        assert_eq!(members.len(), 2);
    }

    #[tokio::test]
    async fn test_list_rooms() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_socket("sock1");

        hub.register(socket).await;
        hub.join_room("sock1", "room1").await.unwrap();
        hub.join_room("sock1", "room2").await.unwrap();

        let rooms = hub.list_rooms().await;
        assert_eq!(rooms.len(), 2);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MESSAGING TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_send_to() {
        let hub = StreamHub::new();
        let (socket, mut rx) = create_socket("sock1");

        hub.register(socket).await;
        hub.send_to("sock1", SocketMessage::text("hello"))
            .await
            .unwrap();

        let msg = rx.recv().await.unwrap();
        assert!(matches!(msg, SocketMessage::Text(s) if s == "hello"));
    }

    #[tokio::test]
    async fn test_broadcast_to_room() {
        let hub = StreamHub::new();
        let (s1, mut rx1) = create_socket("sock1");
        let (s2, mut rx2) = create_socket("sock2");

        hub.register(s1).await;
        hub.register(s2).await;
        hub.join_room("sock1", "room").await.unwrap();
        hub.join_room("sock2", "room").await.unwrap();

        let sent = hub
            .broadcast_to_room("room", SocketMessage::text("hello"))
            .await
            .unwrap();
        assert_eq!(sent, 2);

        assert!(rx1.recv().await.is_some());
        assert!(rx2.recv().await.is_some());
    }

    #[tokio::test]
    async fn test_broadcast_except() {
        let hub = StreamHub::new();
        let (s1, mut rx1) = create_socket("sock1");
        let (s2, mut rx2) = create_socket("sock2");

        hub.register(s1).await;
        hub.register(s2).await;
        hub.join_room("sock1", "room").await.unwrap();
        hub.join_room("sock2", "room").await.unwrap();

        let sent = hub
            .broadcast_to_room_except("room", SocketMessage::text("hello"), "sock1")
            .await
            .unwrap();
        assert_eq!(sent, 1);

        // sock1 should not receive
        assert!(rx1.try_recv().is_err());
        // sock2 should receive
        assert!(rx2.recv().await.is_some());
    }

    #[tokio::test]
    async fn test_broadcast_all() {
        let hub = StreamHub::new();
        let (s1, mut rx1) = create_socket("sock1");
        let (s2, mut rx2) = create_socket("sock2");

        hub.register(s1).await;
        hub.register(s2).await;

        let sent = hub.broadcast_all(SocketMessage::text("hello")).await;
        assert_eq!(sent, 2);

        assert!(rx1.recv().await.is_some());
        assert!(rx2.recv().await.is_some());
    }

    #[tokio::test]
    async fn test_send_to_user() {
        let hub = StreamHub::new();
        let (s1, mut rx1) = create_authenticated_socket("sock1", "user1");
        let (s2, mut rx2) = create_authenticated_socket("sock2", "user1"); // Same user, 2 sockets

        hub.register(s1).await;
        hub.register(s2).await;

        let sent = hub
            .send_to_user("user1", SocketMessage::text("hello"))
            .await
            .unwrap();
        assert_eq!(sent, 2);

        assert!(rx1.recv().await.is_some());
        assert!(rx2.recv().await.is_some());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRESENCE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_user_online() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_authenticated_socket("sock1", "user1");

        hub.register(socket).await;

        assert!(hub.is_user_online("user1").await);
        assert!(!hub.is_user_online("user2").await);
    }

    #[tokio::test]
    async fn test_online_users() {
        let hub = StreamHub::new();
        let (s1, _) = create_authenticated_socket("sock1", "user1");
        let (s2, _) = create_authenticated_socket("sock2", "user2");

        hub.register(s1).await;
        hub.register(s2).await;

        let online = hub.online_users().await;
        assert_eq!(online.len(), 2);
    }

    #[tokio::test]
    async fn test_user_offline_after_disconnect() {
        let hub = StreamHub::new();
        let (socket, _rx) = create_authenticated_socket("sock1", "user1");

        hub.register(socket).await;
        hub.unregister("sock1").await;

        assert!(!hub.is_user_online("user1").await);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // EDGE CASES
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_join_nonexistent_socket() {
        let hub = StreamHub::new();

        let result = hub.join_room("nonexistent", "room").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_to_nonexistent() {
        let hub = StreamHub::new();

        let result = hub
            .send_to("nonexistent", SocketMessage::text("hello"))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_room() {
        let hub = StreamHub::new();

        let members = hub.get_room_members("nonexistent").await;
        assert!(members.is_empty());
    }

    #[tokio::test]
    async fn test_hub_clone() {
        let hub1 = StreamHub::new();
        let hub2 = hub1.clone();
        let (socket, _rx) = create_socket("sock1");

        hub1.register(socket).await;

        // Both hubs share state
        assert_eq!(hub2.connection_count().await, 1);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let hub = StreamHub::new();

        let handles: Vec<_> = (0..100)
            .map(|i| {
                let hub = hub.clone();
                tokio::spawn(async move {
                    let (socket, _rx) = create_socket(&format!("sock{}", i));
                    hub.register(socket).await;
                    hub.join_room(&format!("sock{}", i), "room").await.unwrap();
                })
            })
            .collect();

        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(hub.connection_count().await, 100);
        let room = hub.get_room("room").await.unwrap();
        assert_eq!(room.size(), 100);
    }
}
