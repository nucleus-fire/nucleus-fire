# State Management (Neutron)

Nucleus provides **Neutron**, a world-class reactive state management system with fine-grained reactivity, computed values, batching, and zero-boilerplate stores.

> [!NOTE]
> **Fine-Grained Reactivity**: Unlike Redux or Context that trigger full re-renders, Neutron only updates the specific dependencies that change.

---

## Quick Start (The "Easy Way")

The `#[store]` macro is the standard way to manage state in Nucleus. It automatically generates reactive signals for your struct fields.

```rust
use nucleus_std::neutron::{store, Signal, Global};

// 1. Define your state
#[store]
struct AppState {
    count: i32,
    user: Option<String>,
}

// 2. Initialize
let state = AppState::new(0, Some("Alice".to_string()));

// 3. React
println!("Count: {}", state.count.get()); // Read
state.count.set(5);                       // Write
state.count.modify(|c| *c += 1);          // Update (in-place)
```

---

## Global State

For global singletons (like current user, theme, or app config), use the `Global` wrapper. It is thread-safe and lazy-initialized.

```rust
use nucleus_std::neutron::{Global, Signal};

// Define global state
static THEME: Global<Signal<String>> = Global::new(|| Signal::new("dark".to_string()));

fn toggle_theme() {
    let current = THEME.get();
    let new_theme = if current == "dark" { "light" } else { "dark" };
    THEME.set(new_theme.to_string());
}
```

---

## API Reference

### Store Pattern (`#[store]`)

Organize related state into stores. The macro wraps each field in a `Signal<T>` and generates a `new()` constructor.

```rust
#[store]
pub struct TodoStore {
    todos: Vec<String>,
    filter: String,
}

// Generated API:
// - TodoStore::new(todos: Vec<String>, filter: String)
// - store.todos.get() -> Vec<String>
// - store.todos.set(Vec<String>)
// - store.todos.modify(|v| ...)
```

### Signal<T>

The atomic unit of reactive state.

| Method | Description |
|--------|-------------|
| `.get()` | Read value & track dependency |
| `.set(val)` | Set value & notify dependents |
| `.modify(f)` | **Recommended**: Update in-place (no clone) |
| `.update(f)` | Modify in-place (alias for modify) |
| `.get_untracked()` | Read without tracking |
| `.set_if_changed(val)` | Set only if different |

```rust
// In-place modification is highly efficient for complex types
state.todos.modify(|list| {
    list.push("New Item".to_string());
});
```

### Computed<T>

Values derived from other signals. They update automatically and only when necessary (memoized).

```rust
use nucleus_std::neutron::{computed, Signal};

let count = Signal::new(10);
let doubled = computed(count.clone(), |c| c * 2);

assert_eq!(doubled.get(), 20);
count.set(20);
assert_eq!(doubled.get(), 40);
```

#### Multiple Dependencies

For computed values that depend on multiple signals:

```rust
let full_name = Computed::new({
    let first = state.first_name.clone();
    let last = state.last_name.clone();
    move || format!("{} {}", first.get(), last.get())
});

// Updates when either first_name OR last_name changes
state.first_name.set("Jane".to_string());
assert_eq!(full_name.get(), "Jane Doe");
```

### Effects (`create_effect`)

Side effects that run when dependencies change.

```rust
create_effect({
    let count = state.count.clone();
    move || {
        println!("Count changed to: {}", count.get());
    }
});
```

### Batching

Group multiple updates into a single re-render/notify pass.

```rust
use nucleus_std::neutron::batch;

batch(|| {
    state.first_name.set("John".to_string());
    state.last_name.set("Doe".to_string());
}); // Effects run only once
```

---

## Under the Hood (Advanced)

If you need manual control without the `#[store]` macro, you can create signals directly.

```rust
use nucleus_std::neutron::Signal;

let count = Signal::new(0);
let name = Signal::new("Alice".to_string());
```

### Thread Safety

Neutron is fully thread-safe (`Send + Sync`). You can pass signals between threads or use them in async tasks.

```rust
let counter = Signal::new(0);
std::thread::spawn(move || {
    counter.modify(|c| *c += 1);
});
```

---

## Comparison

| Feature | Neutron | Redux | React Context | Svelte 5 |
|---------|---------|-------|---------------|----------|
| **Syntax** | `#[store]` struct | Reducers/Actions | Providers | Runes |
| **Updates** | Fine-grained | Full Tree | Full Tree | Fine-grained |
| **Boilerplate**| Low | High | Medium | Low |
| **Perf** | 🚀 Native | JS Overhead | JS Overhead | 🚀 JS |

---

## FAQ

**Q: Can I put a Store inside another Store?**
A: Yes! Stores are just structs.

**Q: How do I handle async data?**
A: Use "Resource" patterns or simply set signals inside `async` blocks.

```rust
let data = Signal::new(None);
tokio::spawn(async move {
    let result = fetch_api().await;
    data.set(Some(result));
});
```
