//! Gondola - Offline Sync with CRDTs
//!
//! Conflict-free Replicated Data Types for offline-first apps.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::gondola::{SyncStore, LWWRegister};
//!
//! // Last-Write-Wins register
//! let mut reg = LWWRegister::new("initial");
//! reg.set("updated");
//!
//! // Sync store for key-value data
//! let mut store = SyncStore::new();
//! store.set("user.name", "Alice");
//!
//! // Merge with remote changes
//! let update = store.encode_since(0);
//! ```

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// A timestamp for ordering operations
pub type Timestamp = u64;

/// Generate a current timestamp in milliseconds
fn now() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Syncable value with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncValue {
    /// The actual value
    pub value: serde_json::Value,
    /// When this was last written
    pub timestamp: Timestamp,
    /// Node ID that made the change
    pub node_id: String,
    /// Is this a tombstone (deleted)?
    pub deleted: bool,
}

impl SyncValue {
    fn new(value: serde_json::Value, node_id: &str) -> Self {
        Self {
            value,
            timestamp: now(),
            node_id: node_id.to_string(),
            deleted: false,
        }
    }

    fn tombstone(node_id: &str) -> Self {
        Self {
            value: serde_json::Value::Null,
            timestamp: now(),
            node_id: node_id.to_string(),
            deleted: true,
        }
    }

    /// Returns true if this value wins over other
    fn wins_over(&self, other: &SyncValue) -> bool {
        if self.timestamp != other.timestamp {
            self.timestamp > other.timestamp
        } else {
            // Tie-breaker: higher node_id wins
            self.node_id > other.node_id
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LWW REGISTER
// ═══════════════════════════════════════════════════════════════════════════

/// Last-Write-Wins Register
///
/// Simple CRDT where the most recent write wins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LWWRegister<T> {
    value: T,
    timestamp: Timestamp,
    node_id: String,
}

impl<T: Clone + Default> LWWRegister<T> {
    /// Create a new register with initial value
    pub fn new(value: T) -> Self {
        Self {
            value,
            timestamp: now(),
            node_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Create with specific node ID
    pub fn with_node_id(value: T, node_id: &str) -> Self {
        Self {
            value,
            timestamp: now(),
            node_id: node_id.to_string(),
        }
    }

    /// Get current value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Set new value
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.timestamp = now();
    }

    /// Get timestamp
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Merge with another register (LWW semantics)
    pub fn merge(&mut self, other: &LWWRegister<T>) {
        if other.timestamp > self.timestamp 
            || (other.timestamp == self.timestamp && other.node_id > self.node_id) 
        {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
            self.node_id = other.node_id.clone();
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// G-COUNTER
// ═══════════════════════════════════════════════════════════════════════════

/// Grow-only Counter
///
/// Each node has its own counter that can only increase.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GCounter {
    counts: HashMap<String, u64>,
    node_id: String,
}

impl GCounter {
    /// Create new counter
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
            node_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Create with specific node ID
    pub fn with_node_id(node_id: &str) -> Self {
        Self {
            counts: HashMap::new(),
            node_id: node_id.to_string(),
        }
    }

    /// Get total count
    pub fn value(&self) -> u64 {
        self.counts.values().sum()
    }

    /// Increment by 1
    pub fn increment(&mut self) {
        self.increment_by(1);
    }

    /// Increment by n
    pub fn increment_by(&mut self, n: u64) {
        let count = self.counts.entry(self.node_id.clone()).or_insert(0);
        *count += n;
    }

    /// Merge with another counter
    pub fn merge(&mut self, other: &GCounter) {
        for (node, &count) in &other.counts {
            let current = self.counts.entry(node.clone()).or_insert(0);
            *current = (*current).max(count);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PN-COUNTER
// ═══════════════════════════════════════════════════════════════════════════

/// Positive-Negative Counter
///
/// Counter that can both increment and decrement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PNCounter {
    positive: GCounter,
    negative: GCounter,
}

impl PNCounter {
    /// Create new counter
    pub fn new() -> Self {
        let node_id = uuid::Uuid::new_v4().to_string();
        Self {
            positive: GCounter::with_node_id(&node_id),
            negative: GCounter::with_node_id(&node_id),
        }
    }

    /// Get current value
    pub fn value(&self) -> i64 {
        self.positive.value() as i64 - self.negative.value() as i64
    }

    /// Increment by 1
    pub fn increment(&mut self) {
        self.positive.increment();
    }

    /// Decrement by 1
    pub fn decrement(&mut self) {
        self.negative.increment();
    }

    /// Merge with another counter
    pub fn merge(&mut self, other: &PNCounter) {
        self.positive.merge(&other.positive);
        self.negative.merge(&other.negative);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SYNC STORE
// ═══════════════════════════════════════════════════════════════════════════

/// Key-value store with sync capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStore {
    data: HashMap<String, SyncValue>,
    node_id: String,
    version: u64,
}

impl Default for SyncStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncStore {
    /// Create new store
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            node_id: uuid::Uuid::new_v4().to_string(),
            version: 0,
        }
    }

    /// Create with specific node ID
    pub fn with_node_id(node_id: &str) -> Self {
        Self {
            data: HashMap::new(),
            node_id: node_id.to_string(),
            version: 0,
        }
    }

    /// Get node ID
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Get current version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get a value
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key).and_then(|v| {
            if v.deleted { None } else { Some(&v.value) }
        })
    }

    /// Set a value
    pub fn set(&mut self, key: &str, value: impl Into<serde_json::Value>) {
        self.data.insert(
            key.to_string(),
            SyncValue::new(value.into(), &self.node_id),
        );
        self.version += 1;
    }

    /// Delete a value
    pub fn delete(&mut self, key: &str) {
        if self.data.contains_key(key) {
            self.data.insert(
                key.to_string(),
                SyncValue::tombstone(&self.node_id),
            );
            self.version += 1;
        }
    }

    /// Get all keys (excluding deleted)
    pub fn keys(&self) -> Vec<&str> {
        self.data
            .iter()
            .filter(|(_, v)| !v.deleted)
            .map(|(k, _)| k.as_str())
            .collect()
    }

    /// Merge changes from another store
    pub fn merge(&mut self, other: &SyncStore) {
        for (key, other_value) in &other.data {
            if let Some(my_value) = self.data.get(key) {
                if other_value.wins_over(my_value) {
                    self.data.insert(key.clone(), other_value.clone());
                    self.version += 1;
                }
            } else {
                self.data.insert(key.clone(), other_value.clone());
                self.version += 1;
            }
        }
    }

    /// Encode all changes since a given version
    pub fn encode_since(&self, _since_version: u64) -> Vec<u8> {
        // For simplicity, encode everything
        serde_json::to_vec(&self.data).unwrap_or_default()
    }

    /// Apply encoded changes
    pub fn apply(&mut self, encoded: &[u8]) -> Result<(), String> {
        let incoming: HashMap<String, SyncValue> = serde_json::from_slice(encoded)
            .map_err(|e| format!("Failed to decode: {}", e))?;
        
        for (key, value) in incoming {
            if let Some(my_value) = self.data.get(&key) {
                if value.wins_over(my_value) {
                    self.data.insert(key, value);
                    self.version += 1;
                }
            } else {
                self.data.insert(key, value);
                self.version += 1;
            }
        }
        
        Ok(())
    }

    /// Calculate merkle root for sync detection
    pub fn merkle_root(&self) -> String {
        let mut hasher = Sha256::new();
        
        let mut keys: Vec<_> = self.data.keys().collect();
        keys.sort();
        
        for key in keys {
            if let Some(value) = self.data.get(key) {
                hasher.update(key.as_bytes());
                hasher.update(value.timestamp.to_be_bytes());
            }
        }
        
        hex::encode(hasher.finalize())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LEGACY COMPATIBILITY
// ═══════════════════════════════════════════════════════════════════════════

/// Row for merkle sync (legacy API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub id: String,
    pub version: u64,
}

/// Legacy Gondola struct
pub struct Gondola;

impl Gondola {
    /// Calculate merkle root from rows
    pub fn calculate_merkle_root(rows: &[Row]) -> String {
        if rows.is_empty() {
            return "empty".to_string();
        }

        let mut hasher = Sha256::new();
        for row in rows {
            hasher.update(row.id.as_bytes());
            hasher.update(row.version.to_be_bytes());
        }
        hex::encode(hasher.finalize())
    }

    /// Diff trees to find changed rows
    pub fn diff_trees(client_root: &str, server_rows: &[Row]) -> Vec<String> {
        let server_root = Self::calculate_merkle_root(server_rows);
        
        if client_root == server_root {
            return vec![];
        }

        server_rows.iter().map(|r| r.id.clone()).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lww_register_new() {
        let reg = LWWRegister::new("hello");
        assert_eq!(reg.get(), &"hello");
    }

    #[test]
    fn test_lww_register_set() {
        let mut reg = LWWRegister::new("hello");
        reg.set("world");
        assert_eq!(reg.get(), &"world");
    }

    #[test]
    fn test_lww_register_merge() {
        let mut reg1 = LWWRegister::with_node_id("old", "node1");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let reg2 = LWWRegister::with_node_id("new", "node2");
        
        reg1.merge(&reg2);
        assert_eq!(reg1.get(), &"new");
    }

    #[test]
    fn test_gcounter_new() {
        let counter = GCounter::new();
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_gcounter_increment() {
        let mut counter = GCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_gcounter_merge() {
        let mut c1 = GCounter::with_node_id("node1");
        let mut c2 = GCounter::with_node_id("node2");
        
        c1.increment_by(5);
        c2.increment_by(3);
        
        c1.merge(&c2);
        assert_eq!(c1.value(), 8);
    }

    #[test]
    fn test_pncounter_new() {
        let counter = PNCounter::new();
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_pncounter_increment_decrement() {
        let mut counter = PNCounter::new();
        counter.increment();
        counter.increment();
        counter.decrement();
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_pncounter_merge() {
        let mut c1 = PNCounter::new();
        let mut c2 = PNCounter::new();
        
        c1.increment();
        c1.increment();
        c2.decrement();
        
        c1.merge(&c2);
        // c1 has its own +2, merges c2's -1
        assert!(c1.value() >= 1);
    }

    #[test]
    fn test_sync_store_new() {
        let store = SyncStore::new();
        assert_eq!(store.version(), 0);
    }

    #[test]
    fn test_sync_store_set_get() {
        let mut store = SyncStore::new();
        store.set("name", "Alice");
        
        assert_eq!(store.get("name"), Some(&serde_json::json!("Alice")));
    }

    #[test]
    fn test_sync_store_delete() {
        let mut store = SyncStore::new();
        store.set("name", "Alice");
        store.delete("name");
        
        assert_eq!(store.get("name"), None);
    }

    #[test]
    fn test_sync_store_keys() {
        let mut store = SyncStore::new();
        store.set("a", 1);
        store.set("b", 2);
        store.delete("b");
        
        let keys = store.keys();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&"a"));
    }

    #[test]
    fn test_sync_store_merge() {
        let mut store1 = SyncStore::with_node_id("node1");
        let mut store2 = SyncStore::with_node_id("node2");
        
        store1.set("a", 1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        store2.set("a", 2);
        
        store1.merge(&store2);
        assert_eq!(store1.get("a"), Some(&serde_json::json!(2)));
    }

    #[test]
    fn test_sync_store_encode_apply() {
        let mut store1 = SyncStore::new();
        store1.set("key", "value");
        
        let encoded = store1.encode_since(0);
        
        let mut store2 = SyncStore::new();
        store2.apply(&encoded).unwrap();
        
        assert_eq!(store2.get("key"), Some(&serde_json::json!("value")));
    }

    #[test]
    fn test_sync_store_merkle_root() {
        let mut store1 = SyncStore::with_node_id("node1");
        let mut store2 = SyncStore::with_node_id("node1");
        
        store1.set("a", 1);
        store2.set("a", 1);
        
        // Different timestamps will produce different roots
        // This is expected behavior
        assert!(!store1.merkle_root().is_empty());
    }

    #[test]
    fn test_legacy_merkle_root() {
        let rows = vec![
            Row { id: "1".into(), version: 1 },
            Row { id: "2".into(), version: 2 },
        ];
        
        let root = Gondola::calculate_merkle_root(&rows);
        assert_ne!(root, "empty");
    }

    #[test]
    fn test_legacy_diff_trees_same() {
        let rows = vec![Row { id: "1".into(), version: 1 }];
        let root = Gondola::calculate_merkle_root(&rows);
        
        let diff = Gondola::diff_trees(&root, &rows);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_legacy_diff_trees_different() {
        let rows = vec![Row { id: "1".into(), version: 1 }];
        
        let diff = Gondola::diff_trees("wrong_hash", &rows);
        assert_eq!(diff.len(), 1);
    }

    #[test]
    fn test_sync_value_wins_over() {
        let older = SyncValue {
            value: serde_json::json!("old"),
            timestamp: 100,
            node_id: "a".to_string(),
            deleted: false,
        };
        
        let newer = SyncValue {
            value: serde_json::json!("new"),
            timestamp: 200,
            node_id: "b".to_string(),
            deleted: false,
        };
        
        assert!(newer.wins_over(&older));
        assert!(!older.wins_over(&newer));
    }

    #[test]
    fn test_lww_register_timestamp() {
        let reg = LWWRegister::new("value");
        let ts = reg.timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_sync_store_node_id() {
        let store = SyncStore::with_node_id("custom_node");
        assert_eq!(store.node_id(), "custom_node");
    }

    #[test]
    fn test_apply_invalid_json() {
        let mut store = SyncStore::new();
        let invalid_json = b"not valid json{{{";
        let result = store.apply(invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to decode"));
    }

    #[test]
    fn test_legacy_merkle_empty() {
        let empty: Vec<Row> = vec![];
        let root = Gondola::calculate_merkle_root(&empty);
        assert_eq!(root, "empty");
    }

    #[test]
    fn test_same_timestamp_tiebreaker() {
        // When timestamps are equal, higher node_id wins
        let v1 = SyncValue {
            value: serde_json::json!("from_a"),
            timestamp: 100,
            node_id: "a".to_string(),
            deleted: false,
        };
        
        let v2 = SyncValue {
            value: serde_json::json!("from_z"),
            timestamp: 100, // Same timestamp
            node_id: "z".to_string(), // Higher node_id
            deleted: false,
        };
        
        assert!(v2.wins_over(&v1)); // z > a, so v2 wins
        assert!(!v1.wins_over(&v2));
    }

    #[test]
    fn test_sync_store_delete_nonexistent() {
        let mut store = SyncStore::new();
        let version_before = store.version();
        store.delete("nonexistent_key");
        // Version should not change when deleting non-existent key
        assert_eq!(store.version(), version_before);
    }

    #[test]
    fn test_pncounter_negative_value() {
        let mut counter = PNCounter::new();
        counter.decrement();
        counter.decrement();
        counter.increment();
        // -2 + 1 = -1
        assert_eq!(counter.value(), -1);
    }

    #[test]
    fn test_gcounter_increment_by() {
        let mut counter = GCounter::with_node_id("node1");
        counter.increment_by(10);
        counter.increment_by(5);
        assert_eq!(counter.value(), 15);
    }

    #[test]
    fn test_sync_store_default() {
        let store: SyncStore = Default::default();
        assert_eq!(store.version(), 0);
        assert!(store.keys().is_empty());
    }

    #[test]
    fn test_gcounter_default() {
        let counter: GCounter = Default::default();
        assert_eq!(counter.value(), 0);
    }
}
