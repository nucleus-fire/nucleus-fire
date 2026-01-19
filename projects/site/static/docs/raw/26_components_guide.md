# Components Guide

Nucleus provides a production-ready component system for building reusable UI elements with props, slots, scoped styles, and full type safety.

---

## Table of Contents

1. [Defining Components](#defining-components)
2. [Props System](#props-system)
3. [Slots & Content Projection](#slots--content-projection)
4. [Using Components](#using-components)
5. [Scoped CSS](#scoped-css)
6. [Conditional Rendering](#conditional-rendering)
7. [Component Discovery](#component-discovery)
8. [Islands & Hydration](#islands--hydration)
9. [Built-in Components](#built-in-components)
10. [Error Handling](#error-handling)
11. [Best Practices](#best-practices)
12. [API Reference](#api-reference)

---

## Defining Components

Create components using the `<n:component>` element with a required `name` attribute:

```html
<n:component name="Button">
  <n:props>
    variant: String = "primary"
    size: String = "md"
  </n:props>
  
  <button class="btn btn-{{ variant }} btn-{{ size }}">
    <n:slot />
  </button>
  
  <style scoped>
    .btn {
      display: inline-flex;
      padding: 12px 24px;
      border-radius: 8px;
      cursor: pointer;
    }
  </style>
</n:component>
```

### Component Structure

| Section | Required | Description |
|---------|----------|-------------|
| `<n:component name="X">` | ✅ Yes | Component wrapper with PascalCase name |
| `<n:props>` | ❌ No | Property definitions |
| Template Content | ✅ Yes | HTML/NCL markup |
| `<style scoped>` | ❌ No | Component-scoped CSS |

### File Naming Convention

```
components/
├── Button.ncl          # <Button />
├── Card.ncl            # <Card />
├── FeatureCard.ncl     # <FeatureCard />
└── forms/
    ├── Input.ncl       # <Input />
    └── Select.ncl      # <Select />
```

---

## Props System

### Defining Props

Props are defined inside `<n:props>` with type annotations:

```html
<n:props>
  title: String                   <!-- Required (no default) -->
  count: i32 = 0                  <!-- Optional with default -->
  active: bool = true             <!-- Boolean with default -->
  price: f64 = 9.99               <!-- Float with default -->
  id: i64 = 0                     <!-- Long integer -->
</n:props>
```

### Supported Types

| Type | Rust Equivalent | Example |
|------|-----------------|---------|
| `String` | `String` | `"hello"` |
| `i32` | `i32` | `42` |
| `i64` | `i64` | `9999999999` |
| `f64` | `f64` | `3.14159` |
| `bool` | `bool` | `true` / `false` |

### Required vs Optional Props

```html
<n:props>
  <!-- Required: No default value -->
  title: String
  
  <!-- Optional: Has default value -->
  subtitle: String = "Default subtitle"
</n:props>
```

### Using Props in Templates

Access props with interpolation syntax:

```html
<n:component name="Alert">
  <n:props>
    type: String = "info"
    message: String
  </n:props>
  
  <div class="alert alert-{{ type }}">
    <span class="alert-icon">{{ type }}</span>
    <p>{{ message }}</p>
  </div>
</n:component>
```

---

## Slots & Content Projection

### Default Slot

The default slot captures all content passed to the component:

```html
<!-- Component Definition -->
<n:component name="Card">
  <div class="card">
    <n:slot />  <!-- Content goes here -->
  </div>
</n:component>

<!-- Usage -->
<Card>
  <h2>Title</h2>
  <p>This content is projected into the slot</p>
</Card>
```

### Named Slots

Use named slots for multiple content areas:

```html
<!-- Component Definition -->
<n:component name="Modal">
  <n:props>
    open: bool = false
  </n:props>
  
  <div class="modal {% if open %}open{% endif %}">
    <header class="modal-header">
      <n:slot name="header" />
    </header>
    <main class="modal-body">
      <n:slot />  <!-- Default slot -->
    </main>
    <footer class="modal-footer">
      <n:slot name="footer" />
    </footer>
  </div>
</n:component>

<!-- Usage -->
<Modal open={true}>
  <template slot="header">
    <h2>Confirm Action</h2>
  </template>
  
  <p>Are you sure you want to proceed?</p>
  
  <template slot="footer">
    <Button variant="secondary">Cancel</Button>
    <Button variant="primary">Confirm</Button>
  </template>
</Modal>
```

### Slot Fallback Content

Provide default content when no slot content is given:

```html
<n:slot name="icon">
  <span>🔔</span>  <!-- Fallback if no icon provided -->
</n:slot>
```

---

## Using Components

### Basic Usage

Components use PascalCase syntax and can be self-closing:

```html
<!-- Self-closing (no children) -->
<Button variant="primary" />

<!-- With children (slot content) -->
<Button variant="gradient">
  Click Me
</Button>
```

### Passing Props

```html
<!-- String props -->
<Card variant="highlighted" />

<!-- Interpolated props -->
<Card title="{{ pageTitle }}" />

<!-- Boolean props -->
<Toggle active />
<Toggle active={true} />
<Toggle active={false} />

<!-- Numeric props -->
<Counter initial={5} />
```

### Nesting Components

Components can contain other components:

```html
<Card variant="feature">
  <Badge icon="⚡" variant="primary">New</Badge>
  <h3>Feature Title</h3>
  <p>Description text</p>
  <Button href="/learn-more">Learn More</Button>
</Card>
```

---

## Scoped CSS

### Basic Scoped Styles

Add `scoped` attribute to isolate styles to the component:

```html
<n:component name="Alert">
  <div class="alert">
    <n:slot />
  </div>
  
  <style scoped>
    .alert {
      padding: 16px;
      border-radius: 8px;
      background: var(--bg-warning);
    }
    
    /* These styles ONLY apply to this component */
    .alert p {
      margin: 0;
    }
  </style>
</n:component>
```

### How Scoping Works

Nucleus generates a unique scope ID (e.g., `nc7a8b2c`) and prefixes all selectors:

```css
/* Original */
.alert { padding: 16px; }

/* Compiled */
.alert[data-nc7a8b2c] { padding: 16px; }
```

### CSS Custom Properties

Use CSS variables for theming:

```html
<style scoped>
  .btn {
    background: var(--accent, #00dc82);
    color: var(--text-primary, #fff);
    border-radius: var(--radius-md, 8px);
  }
</style>
```

---

## Conditional Rendering

Use `{% if %}` and `{% for %}` inside components:

```html
<n:component name="List">
  <n:props>
    items: String
    empty: String = "No items"
  </n:props>
  
  {% if items.len() > 0 %}
    <ul class="list">
      {% for item in items %}
        <li>{{ item }}</li>
      {% endfor %}
    </ul>
  {% endif %}
  
  {% if items.len() == 0 %}
    <p class="empty">{{ empty }}</p>
  {% endif %}
</n:component>
```

### Conditional Classes

```html
<div class="card {% if highlighted %}card-highlight{% endif %}">
  <n:slot />
</div>
```

---

## Component Discovery

### Automatic Discovery

Nucleus automatically discovers components in the `components/` directory:

```
src/
├── components/
│   ├── Button.ncl      → <Button />
│   ├── Card.ncl        → <Card />
│   └── ui/
│       ├── Modal.ncl   → <Modal />
│       └── Tabs.ncl    → <Tabs />
└── views/
    └── index.ncl       → Uses all components
```

### Manual Imports

For explicit imports, use `<n:include>`:

```html
<n:include src="./components/Button.ncl" />

<Button variant="primary">Click</Button>
```

---

## Islands & Hydration

### Converting Components to Islands

Make a component interactive with client-side hydration:

```html
<!-- Static Component -->
<Counter initial={0} />

<!-- Hydrated Island - loads immediately -->
<n:island src="components/Counter.ncl" client:load initial={0} />

<!-- Hydrated when visible -->
<n:island src="components/Counter.ncl" client:visible initial={0} />

<!-- Hydrated when idle -->
<n:island src="components/Counter.ncl" client:idle initial={0} />
```

### Hydration Directives

| Directive | When Hydrated |
|-----------|---------------|
| `client:load` | Immediately on page load |
| `client:visible` | When element enters viewport |
| `client:idle` | When browser is idle |
| `client:media="(min-width: 768px)"` | When media query matches |

---

## Built-in Components

### Button

```html
<Button variant="primary" size="md" href="/path">
  Click Me
</Button>
```

| Prop | Type | Default | Values |
|------|------|---------|--------|
| `variant` | String | `"primary"` | `primary`, `secondary`, `gradient` |
| `size` | String | `"md"` | `sm`, `md`, `lg` |
| `href` | String | `""` | URL (renders as `<a>`) |

### Card

```html
<Card variant="default" glass="false">
  Content here
</Card>
```

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | String | `"default"` | `default`, `feature`, `highlight` |
| `glass` | String | `"false"` | Enable glassmorphism effect |

### Badge

```html
<Badge variant="primary" icon="⚡">
  New Feature
</Badge>
```

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | String | `"default"` | `default`, `primary` |
| `icon` | String | `""` | Icon emoji or text |

### FeatureCard

```html
<FeatureCard 
  icon="🚀" 
  title="Fast" 
  description="20,000+ requests per second" 
/>
```

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `icon` | String | Yes | Icon emoji |
| `title` | String | Yes | Card title |
| `description` | String | Yes | Card description |

---

## Error Handling

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `MissingRequiredProp` | Required prop not provided | Add the missing prop |
| `UnknownComponent` | Component not found | Check spelling and file path |
| `InvalidPropType` | Wrong type for prop | Match expected type |
| `InvalidComponentName` | Not PascalCase | Rename to PascalCase |
| `CircularDependency` | A → B → A | Refactor to break cycle |

### Error Output Example

```
error[nucleus::component::missing_prop]: Component 'Button' is missing required prop 'variant'
  ┌─ src/views/index.ncl:15:5
  │
15 │ <Button size="lg" />
   │ ^^^^^^^^^^^^^^^^^^^^
   │
   help: Add the required prop: <Button variant="value" />
```

---

## Best Practices

### 1. Single Responsibility
Each component should do one thing well.

```html
<!-- ✅ Good: Single purpose -->
<Button>Click</Button>
<Icon name="star" />

<!-- ❌ Bad: Multiple purposes -->
<ButtonWithIcon icon="star">Click</ButtonWithIcon>
```

### 2. Props Over Slots for Simple Values

```html
<!-- ✅ Good: Simple value as prop -->
<Alert message="Success!" type="success" />

<!-- ❌ Unnecessary: Using slot for simple text -->
<Alert type="success">
  <template slot="message">Success!</template>
</Alert>
```

### 3. Always Use Scoped Styles

```html
<!-- ✅ Good: Scoped -->
<style scoped>
  .btn { background: blue; }
</style>

<!-- ❌ Bad: Global pollution -->
<style>
  .btn { background: blue; }
</style>
```

### 4. Document Complex Props

```html
<n:props>
  <!-- The visual style variant -->
  variant: String = "primary"
  
  <!-- Size: sm (32px), md (40px), lg (48px) -->
  size: String = "md"
</n:props>
```

### 5. Use Semantic HTML

```html
<!-- ✅ Good: Semantic -->
<n:component name="Nav">
  <nav role="navigation">
    <n:slot />
  </nav>
</n:component>

<!-- ❌ Bad: Div soup -->
<n:component name="Nav">
  <div class="nav">
    <n:slot />
  </div>
</n:component>
```

---

## API Reference

### Component Element

```html
<n:component name="ComponentName">
  <!-- Component body -->
</n:component>
```

| Attribute | Required | Description |
|-----------|----------|-------------|
| `name` | Yes | PascalCase component name |

### Props Element

```html
<n:props>
  propName: Type = "default"
</n:props>
```

### Slot Element

```html
<n:slot />
<n:slot name="slotName" />
```

| Attribute | Required | Description |
|-----------|----------|-------------|
| `name` | No | Slot name (default: unnamed) |

### Style Element

```html
<style scoped>
  /* CSS */
</style>
```

| Attribute | Required | Description |
|-----------|----------|-------------|
| `scoped` | No | Enable CSS isolation |

### Component Usage

```html
<ComponentName prop="value" boolProp>
  Slot content
</ComponentName>
```

---

## Migration Guide

### From Includes to Components

**Before (includes):**
```html
<n:include src="partials/button.ncl" variant="primary" />
```

**After (components):**
```html
<Button variant="primary">Click</Button>
```

### Benefits of Components

| Feature | Includes | Components |
|---------|----------|------------|
| Props Validation | ❌ | ✅ |
| Type Safety | ❌ | ✅ |
| Scoped CSS | ❌ | ✅ |
| Slot Content | ❌ | ✅ |
| Error Messages | Basic | Detailed |
