# Offline Sync Guide

Build offline-first apps with Gondola - CRDTs for conflict-free synchronization.

---

## What are CRDTs?

**Conflict-free Replicated Data Types** allow multiple users to edit the same data simultaneously without conflicts. Changes merge automatically using mathematical properties, not timestamps or locking.

---

## Quick Start

```rust
use nucleus_std::gondola::{SyncStore, LWWRegister, GCounter};

// Key-value store that syncs
let mut store = SyncStore::new();
store.set("user.name", "Alice");
store.set("user.score", 100);

// Get values
let name = store.get("user.name"); // Some("Alice")
```

---

## CRDT Types

### LWW Register (Last-Write-Wins)

Simple value that resolves conflicts by timestamp.

```rust
use nucleus_std::gondola::LWWRegister;

let mut reg = LWWRegister::new("initial value");

// Update value
reg.set("new value");

// Get current value
let value = reg.get(); // "new value"

// Merge with another register (newer wins)
let other = LWWRegister::new("other value");
reg.merge(&other);
```

### G-Counter (Grow-only)

Counter that can only increase. Used for likes, views, etc.

```rust
use nucleus_std::gondola::GCounter;

let mut counter = GCounter::new();

counter.increment();
counter.increment_by(5);

let total = counter.value(); // 6

// Merge counters from different nodes
let mut remote = GCounter::new();
remote.increment_by(10);

counter.merge(&remote);
// Total now includes both nodes' counts
```

### PN-Counter (Positive-Negative)

Counter that supports both increment and decrement.

```rust
use nucleus_std::gondola::PNCounter;

let mut counter = PNCounter::new();

counter.increment();
counter.increment();
counter.decrement();

let value = counter.value(); // 1
```

---

## SyncStore

A complete key-value store with sync capabilities.

### Basic Operations

```rust
use nucleus_std::gondola::SyncStore;

let mut store = SyncStore::new();

// Set values (any JSON-serializable type)
store.set("user.name", "Alice");
store.set("user.age", 30);
store.set("settings.theme", "dark");

// Get values
let name = store.get("user.name"); // Some(Value::String("Alice"))

// Delete values
store.delete("settings.theme");

// List keys
let keys = store.keys(); // ["user.name", "user.age"]
```

### Synchronization

```rust
// Encode changes for transmission
let changes = store.encode_since(0);

// On another device, apply changes
let mut remote_store = SyncStore::new();
remote_store.apply(&changes)?;

// Or merge entire stores
remote_store.merge(&store);
```

### Conflict Detection

```rust
// Merkle root for quick comparison
let root = store.merkle_root();

// Compare with remote
if local_root != remote_root {
    // Stores have diverged, sync needed
    let changes = remote.encode_since(0);
    local.apply(&changes)?;
}
```

---

## Sync Protocol

### 1. Initial Sync

```rust
// Client connects
let client_root = client_store.merkle_root();

// Server compares
if client_root != server_root {
    let changes = server_store.encode_since(0);
    // Send changes to client
}
```

### 2. Incremental Sync

```rust
// Client tracks version
let last_version = client_store.version();

// Only get changes since last sync
let changes = server_store.encode_since(last_version);
client_store.apply(&changes)?;
```

### 3. Bi-directional Sync

```rust
// Client sends their changes
let client_changes = client_store.encode_since(last_sync);

// Server applies and responds with its changes
server_store.apply(&client_changes)?;
let server_changes = server_store.encode_since(last_sync);

// Client applies server changes
client_store.apply(&server_changes)?;
```

---

## Node Identity

Each store has a unique node ID for conflict resolution.

```rust
// Auto-generated ID
let store = SyncStore::new();
println!("Node: {}", store.node_id());

// Explicit ID (for persistent identity)
let store = SyncStore::with_node_id("device_abc123");
```

---

## Use Cases

| Scenario | CRDT Type |
|----------|-----------|
| Like/view counts | GCounter |
| User settings | LWWRegister |
| Shopping cart | SyncStore |
| Collaborative text | (Would need specialized text CRDT) |
| Inventory levels | PNCounter |

---

## Testing

```rust
#[test]
fn test_offline_sync() {
    let mut device1 = SyncStore::with_node_id("phone");
    let mut device2 = SyncStore::with_node_id("laptop");
    
    // Both devices make changes offline
    device1.set("note", "From phone");
    device2.set("note", "From laptop");
    
    // Later, sync (newer timestamp wins)
    device1.merge(&device2);
    
    // Both have consistent state
}
```
