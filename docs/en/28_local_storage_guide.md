# Local Storage Guide

Build offline-capable apps with Nucleus's client-side storage APIs.

---

## Quick Start

Include the Neutron.store module in your page:

```html
<script src="/assets/neutron-store.js"></script>
```

Then use it in any `<n:island>` or script:

```javascript
// Simple key-value storage
Neutron.store.set('user.theme', 'dark');
Neutron.store.get('user.theme'); // 'dark'

// Reactive signal that persists automatically
const counter = Neutron.store.signal('counter', 0);
counter.set(counter.get() + 1);
```

---

## API Reference

### Neutron.store (localStorage)

| Method | Description |
|--------|-------------|
| `set(key, value)` | Store any JSON-serializable value |
| `get(key, default?)` | Retrieve value or default |
| `remove(key)` | Delete a key |
| `clear()` | Remove all Neutron data |
| `keys()` | List all stored keys |
| `quota()` | Get storage quota info (async) |
| `subscribe(key, fn)` | React to changes |

#### Example: User Preferences

```javascript
// Save preferences
Neutron.store.set('prefs', {
    theme: 'dark',
    fontSize: 16,
    notifications: true
});

// Load with defaults
const prefs = Neutron.store.get('prefs', {
    theme: 'light',
    fontSize: 14,
    notifications: false
});
```

---

### Reactive Signals

Create signals that automatically persist to localStorage:

```javascript
// Create a persistent signal
const theme = Neutron.store.signal('theme', 'light');

// Read current value
console.log(theme.get()); // 'light'

// Update (auto-saves to localStorage)
theme.set('dark');

// Subscribe to changes
const unsubscribe = theme.subscribe((newValue) => {
    document.body.className = newValue;
});

// Update with function
theme.update(current => current === 'dark' ? 'light' : 'dark');

// Cleanup when done
unsubscribe();
```

---

### Neutron.db (IndexedDB)

For larger or structured data, use IndexedDB:

| Method | Description |
|--------|-------------|
| `put(store, obj)` | Store object (must have `id` property) |
| `get(store, id)` | Retrieve by ID |
| `getAll(store)` | Get all objects in store |
| `delete(store, id)` | Remove by ID |
| `clear(store)` | Empty the store |
| `count(store)` | Count objects |

#### Example: Offline Cache

```javascript
// Cache API responses
async function fetchPosts() {
    try {
        const response = await fetch('/api/posts');
        const posts = await response.json();
        
        // Cache for offline use
        for (const post of posts) {
            await Neutron.db.put('posts', post);
        }
        return posts;
    } catch (e) {
        // Offline - return cached
        return await Neutron.db.getAll('posts');
    }
}
```

---

## Cross-Tab Sync

Changes automatically sync between browser tabs:

```javascript
// Tab 1
Neutron.store.subscribe('cart', (items) => {
    updateCartBadge(items.length);
});

// Tab 2 - updates trigger callback in Tab 1
Neutron.store.set('cart', [...cart, newItem]);
```

---

## Feature Detection

Check what's available in the current browser:

```javascript
if (Neutron.store.hasLocalStorage) {
    // Full localStorage support
}

if (Neutron.store.hasIndexedDB) {
    // Can use Neutron.db
} else {
    // Fall back to localStorage only
}
```

---

## Storage Quota

Check available storage space:

```javascript
const { used, quota, available } = await Neutron.store.quota();
console.log(`Using ${used} of ${quota} bytes`);
console.log(`${available} bytes available`);

if (available < 1024 * 1024) {
    // Less than 1MB left - warn user
    showStorageWarning();
}
```

---

## Best Practices

### 1. Namespace Your Keys

```javascript
// Good - prefixed by feature
Neutron.store.set('cart.items', []);
Neutron.store.set('cart.total', 0);
Neutron.store.set('auth.token', '...');

// Bad - generic names risk collisions
Neutron.store.set('items', []);
```

### 2. Handle Errors

```javascript
const saved = Neutron.store.set('data', largeObject);
if (!saved) {
    // Storage full or unavailable
    showError('Could not save data');
}
```

### 3. Clean Up Old Data

```javascript
// Version your cache
const CACHE_VERSION = 2;
const storedVersion = Neutron.store.get('cache.version', 0);

if (storedVersion < CACHE_VERSION) {
    Neutron.store.clear();
    Neutron.store.set('cache.version', CACHE_VERSION);
}
```

### 4. Use IndexedDB for Large Data

```javascript
// localStorage: ~5MB limit, synchronous
Neutron.store.set('prefs', { theme: 'dark' }); // ✓ Small

// IndexedDB: ~50MB+ limit, async
await Neutron.db.put('images', { id: 1, data: largeBlob }); // ✓ Large
```

---

## Integration with Gondola

For multi-device sync, combine with Gondola CRDTs:

```javascript
// Local-first pattern
async function updateTodo(id, changes) {
    // 1. Update local storage immediately (optimistic)
    const todos = Neutron.store.get('todos', []);
    const idx = todos.findIndex(t => t.id === id);
    todos[idx] = { ...todos[idx], ...changes };
    Neutron.store.set('todos', todos);
    
    // 2. Sync to server in background
    try {
        await fetch(`/api/todos/${id}`, {
            method: 'PATCH',
            body: JSON.stringify(changes)
        });
    } catch (e) {
        // Queue for later sync (Background Sync API)
        await navigator.serviceWorker.ready;
        await registration.sync.register('sync-todos');
    }
}
```

---

## See Also

- [Offline Sync Guide](27_offline_sync_guide.md) - Server-side CRDTs
- [PWA Export](#) - Service worker generation
- [State Management](15_state_management.md) - Neutron signals
