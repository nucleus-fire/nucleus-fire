# Compiler Reference (`ncc`)

The Nucleus Compiler (`ncc`) transforms `.ncl` files into optimized Rust code.

---

## Core Tags

### `<n:view>`
Defines a page or component.

```html
<n:view title="Dashboard">
    <h1>Welcome back!</h1>
    <p>Here's your dashboard.</p>
</n:view>
```

**Compiles to:** Axum handler returning HTML response.

### `<n:layout>`
Wraps content in a layout template.

```html
<n:view title="About">
    <n:layout name="main">
        <h1>About Us</h1>
        <p>Our company story...</p>
    </n:layout>
</n:view>
```

### `<n:model>`
Defines a database entity and Rust struct.

```html
<n:model name="User">
    id: i64
    email: String
    name: String
    created_at: DateTime
</n:model>
```

**Compiles to:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub created_at: DateTime,
}
```

### `<n:action>`
Server-side Rust code for form handling.

```html
<n:action>
    let email = params.get("email").unwrap_or(&"".to_string()).clone();
    
    if email.contains('@') {
        println!("New subscriber: {}", email);
    }
</n:action>

<form method="POST">
    <input type="email" name="email" required>
    <button type="submit">Subscribe</button>
</form>
```

---

## Form Tags

### `<n:form>` - Type-Safe Forms

```html
<n:form>
    <n:input name="email" type="email" required="true" />
    <n:input name="password" type="password" minlength="8" />
    <n:input name="age" type="number" min="18" max="120" />
    <n:select name="country">
        <option value="us">United States</option>
        <option value="uk">United Kingdom</option>
    </n:select>
    <button type="submit">Submit</button>
</n:form>
```

**Compiles to:** Rust struct with automatic validation.

---

## Control Flow

### `<n:if>` - Conditional Rendering

```html
<n:if condition="user.is_admin">
    <div class="admin-panel">
        Admin controls here
    </div>
</n:if>

<n:if condition="!user.is_verified">
    <p>Please verify your email.</p>
</n:if>
```

### `<n:for>` - List Rendering

```html
<ul>
    <n:for each="user" in="users">
        <li>{user.name} - {user.email}</li>
    </n:for>
</ul>
```

### `<n:match>` - Pattern Matching

```html
<n:match value="order.status">
    <n:when pattern="pending">
        <span class="badge yellow">Pending</span>
    </n:when>
    <n:when pattern="shipped">
        <span class="badge blue">Shipped</span>
    </n:when>
    <n:when pattern="delivered">
        <span class="badge green">Delivered</span>
    </n:when>
</n:match>
```

---

## Data Loading

### `<n:loader>` - Server-Side Data Fetching

```html
<n:view title="User Profile">
    <n:loader>
        let user_id = params.get("id").unwrap();
        let user = User::find(user_id).await?;
        let posts = Post::where("user_id", user_id).all().await?;
    </n:loader>
    
    <h1>{user.name}</h1>
    <n:for each="post" in="posts">
        <article>{post.title}</article>
    </n:for>
</n:view>
```

---

## Client-Side Interactivity

### `<n:island>` - Reactive Islands (Neutron)

Nucleus V3 uses "Neutron" for client-side interactivity, allowing fine-grained reactivity using Rust-like Signals within island components.

```html
<!-- components/Counter.ncl -->
<n:island client:load>
    <n:script>
        let count = Signal::new(0);
        
        // Derived state
        let double = computed(count.clone(), |c| c * 2);
    </n:script>

    <div class="counter">
        <p>Count: {count}</p>
        <p>Double: {double}</p>
        <button onclick={count.update(|c| *c += 1)}>Increment</button>
    </div>
</n:island>
```

---

## Slots and Components

### `<n:slot>` - Content Injection Point

```html
<!-- In layout.ncl -->
<div class="content">
    <n:slot name="content" />
</div>

<div class="sidebar">
    <n:slot name="sidebar" />
</div>
```

### `<n:component>` - Reusable Components

```html
<!-- components/button.ncl -->
<n:component name="Button">
    <n:prop name="variant" default="primary" />
    <n:prop name="size" default="md" />
    
    <button class="btn btn-{variant} btn-{size}">
        <n:slot />
    </button>
</n:component>

<!-- Usage -->
<Button variant="danger" size="lg">Delete</Button>
```

---

## Build Commands

```bash
# Development build with HMR
nucleus run

# Production build
nucleus build

# Run tests
nucleus test

# Generate migration
nucleus generate migration create_users
```

---

## Compilation Output

| Input | Output |
|-------|--------|
| `<n:view>` | Axum handler function |
| `<n:model>` | Rust struct + SQL |
| `<n:action>` | POST handler |
| `<n:loader>` | Data fetching code |
| `<n:state>` | WASM reactive store |
