# NCL Syntax Reference

> Complete reference for Nucleus Component Language (`.ncl`) syntax.

## File Structure

Every `.ncl` file represents a **page** or **component**. Files must have exactly one `<n:view>` root element.

```html
<n:view title="Page Title" description="SEO description">
    <!-- Content here -->
</n:view>
```

---

## Core Tags

### `<n:view>` - Page Definition

The root element for every page. Automatically generates HTML5 boilerplate.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `title` | String | Yes | Page title (appears in browser tab) |
| `description` | String | No | Meta description for SEO |
| `layout` | String | No | Layout template to use |
| `guard` | String | No | Authentication guard function |

**Example:**
```html
<n:view title="Dashboard" description="User dashboard" guard="require_auth">
    <h1>Welcome</h1>
</n:view>
```

**Generated Output:**
```html
<!DOCTYPE html>
<html>
<head>
    <title>Dashboard</title>
    <meta name="description" content="User dashboard">
    <meta property="og:title" content="Dashboard">
    <meta property="og:description" content="User dashboard">
    <meta property="og:type" content="website">
    <meta name="twitter:card" content="summary_large_image">
</head>
<body>
    <h1>Welcome</h1>
</body>
</html>
```

---

### `<n:model>` - Data Binding

Loads server-side data into the template context.

| Attribute | Type | Description |
|-----------|------|-------------|
| `name="expression"` | Rust expr | Binds result of expression to variable `name` |

**Examples:**
```html
<!-- Simple query -->
<n:model users="db::get_all_users().await" />

<!-- Multiple bindings -->
<n:model 
    users="db::get_users().await"
    posts="db::get_posts().await"
    stats="analytics::get_stats()"
/>

<!-- With error handling -->
<n:model user="db::get_user(id).await.unwrap_or_default()" />
```

**Usage in template:**
```html
<n:model users="db::get_users().await" />

<ul>
    <n:for item="user" in="users">
        <li>{user.name}</li>
    </n:for>
</ul>
```

---

### `<n:text>` - Text Rendering

Renders text content, either from i18n keys or expressions.

| Attribute | Type | Description |
|-----------|------|-------------|
| `key` | String | i18n key from `content.deck` |
| `value` | Expression | Rust expression to render |
| `escape` | Boolean | HTML escape output (default: true) |

**Examples:**
```html
<!-- Localized text -->
<n:text key="welcome_message" />

<!-- Variable output -->
<n:text value="user.name" />

<!-- Raw HTML (be careful!) -->
<n:text value="post.html_content" escape="false" />
```

---

### `<n:for>` - Iteration

Loops over collections.

| Attribute | Type | Description |
|-----------|------|-------------|
| `item` | String | Variable name for current item |
| `in` | String | Collection variable to iterate |
| `index` | String | Optional: Variable name for index |

**Examples:**
```html
<!-- Basic loop -->
<n:for item="post" in="posts">
    <article>
        <h2>{post.title}</h2>
        <p>{post.excerpt}</p>
    </article>
</n:for>

<!-- With index -->
<n:for item="item" in="items" index="i">
    <li>#{i + 1}: {item.name}</li>
</n:for>

<!-- Nested loops -->
<n:for item="category" in="categories">
    <section>
        <h2>{category.name}</h2>
        <n:for item="product" in="category.products">
            <div>{product.name}</div>
        </n:for>
    </section>
</n:for>
```

---

### `<n:if>` - Conditionals

Conditional rendering.

| Attribute | Type | Description |
|-----------|------|-------------|
| `condition` | Boolean expr | Condition to evaluate |

**Examples:**
```html
<!-- Simple condition -->
<n:if condition="user.is_admin">
    <button>Delete</button>
</n:if>

<!-- With else (use two if blocks) -->
<n:if condition="user.is_authenticated">
    <a href="/profile">Profile</a>
</n:if>
<n:if condition="!user.is_authenticated">
    <a href="/login">Login</a>
</n:if>

<!-- Complex conditions -->
<n:if condition="items.len() > 0 && user.can_view">
    <ul>...</ul>
</n:if>
```

---

### `<n:image>` - Optimized Images

Renders images with automatic optimization hints.

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | String | Required | Image path |
| `alt` | String | Required | Accessibility text |
| `width` | Number | Auto | Image width |
| `height` | Number | Auto | Image height |
| `lazy` | Boolean | true | Lazy loading |
| `priority` | Boolean | false | Preload (for above-fold) |

**Examples:**
```html
<!-- Basic usage -->
<n:image src="/assets/hero.webp" alt="Hero banner" />

<!-- Above-fold image (preload) -->
<n:image src="/assets/logo.webp" alt="Logo" priority="true" />

<!-- With dimensions -->
<n:image src="/assets/avatar.webp" alt="User avatar" width="64" height="64" />
```

**Generated Output:**
```html
<img 
    src="/assets/hero.webp" 
    alt="Hero banner" 
    loading="lazy" 
    decoding="async"
/>
```

---

### `<n:link>` - Navigation Links

Client-side navigation with prefetching.

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `href` | String | Required | Target URL |
| `prefetch` | Boolean | true | Preload on hover |
| `replace` | Boolean | false | Replace history entry |

**Examples:**
```html
<!-- Basic link -->
<n:link href="/about">About Us</n:link>

<!-- No prefetch -->
<n:link href="/heavy-page" prefetch="false">Heavy Page</n:link>

<!-- External link (auto-detected) -->
<n:link href="https://example.com">External</n:link>
```

---

### `<n:form>` - Form Handling

Server-side form processing.

| Attribute | Type | Description |
|-----------|------|-------------|
| `action` | String | Handler function |
| `method` | String | HTTP method (POST, PUT, DELETE) |
| `validate` | Boolean | Enable validation |

**Examples:**
```html
<n:form action="handlers::create_user" method="POST" validate="true">
    <input type="text" name="email" required />
    <input type="password" name="password" minlength="8" />
    <button type="submit">Register</button>
</n:form>
```

---

### `<n:script>` - Server-Side Code

Rust code executed during server-side rendering.

**Example:**
```html
<n:script>
    let timestamp = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let greeting = if hour < 12 { "Good morning" } else { "Hello" };
</n:script>

<p>{greeting}, the date is {timestamp}</p>
```

---

### `<n:style>` - Scoped CSS

CSS scoped to the current component.

**Example:**
```html
<n:style>
    .card {
        background: #222;
        border-radius: 8px;
        padding: 1rem;
    }
    
    .card:hover {
        transform: translateY(-2px);
    }
</n:style>
```

---

## Template Expressions

### Interpolation

Use `{expression}` syntax for inline values:

```html
<h1>Hello, {user.name}!</h1>
<p>You have {messages.len()} messages</p>
<time datetime="{post.date}">{post.formatted_date}</time>
```

### Operators

| Operator | Example | Description |
|----------|---------|-------------|
| `+`, `-`, `*`, `/` | `{a + b}` | Arithmetic |
| `==`, `!=`, `<`, `>` | `{a == b}` | Comparison |
| `&&`, `\|\|`, `!` | `{a && b}` | Logical |
| `.` | `{user.email}` | Field access |
| `()` | `{calc()}` | Method call |

---

## Islands Architecture (Client-Side Hydration)
### `<n:island>`
Renders an interactive component with JavaScript hydration.

| Attribute | Description |
|-----------|-------------|
| `src` | Path to the component file (e.g., `"components/counter.ncl"`) |
| `client:load` | Hydrate immediately on page load |
| `client:visible` | Hydrate when element enters viewport (Lazy) |
| `client:idle` | Hydrate when main thread is idle |

**Example:**
```html
<!-- Hydrate immediately -->
<n:island src="src/views/components/counter.ncl" client:load />

<!-- Lazy load this heavy interactive widget -->
<n:island src="src/views/components/map_widget.ncl" client:visible />
```

---

## Layouts and Nested Routing
### `<n:layout>`
Defines a wrapper template for views. Support nested layouts via `<n:outlet />` or `<n:slot />`.

**`src/layouts/dashboard.ncl`:**
```html
<n:layout>
    <div class="dashboard-grid">
        <sidebar>...</sidebar>
        <main>
            <!-- Content form the view is injected here -->
            <n:slot /> 
        </main>
    </div>
</n:layout>
```

**Using a Layout:**
```html
<n:view layout="dashboard" title="Analytics">
    <h1>Chart Area</h1>
</n:view>
```

---

## Components
### `<n:component>`
Defines a reusable UI fragment.

**Props Injection**: Attributes passed to specific `src` components are injected as variables into the component scope.

**`src/components/UserCard.ncl`:**
```html
<!-- `name` and `role` are variables available in scope -->
<div class="card">
    <h3>{name}</h3>
    <p>{role}</p>
</div>
```

**Usage:**
```html
<n:include src="src/components/UserCard.ncl" name="Alice" role="Admin" />
```

---

## File Naming Conventions

| Pattern | Route | Description |
|---------|-------|-------------|
| `index.ncl` | `/` | Homepage |
| `about.ncl` | `/about` | Static page |
| `blog/index.ncl` | `/blog` | Section index |
| `blog/post.ncl` | `/blog/post` | Nested page |
| `users/[id].ncl` | `/users/:id` | Dynamic route |
| `[...slug].ncl` | `/*slug` | Catch-all route |

---

## Best Practices

1. **Always set `alt` on images** - Required for accessibility
2. **Use `<n:model>` for data fetching** - Keeps logic out of templates
3. **Prefer `<n:link>` over `<a>`** - Enables prefetching
4. **Break large pages into components** - Improves maintainability
5. **Use layouts for shared structure** - DRY principle
