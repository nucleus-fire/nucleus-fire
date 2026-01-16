# Playground Guide

> Write, preview, and share Nucleus code directly in your browser.

## Overview

The Nucleus Playground is an interactive environment for experimenting with Nucleus templates (.ncl) without installing anything locally. It features:

- **Monaco Editor** - Full syntax highlighting and IntelliSense
- **Live Preview** - Real-time HTML rendering
- **Example Library** - Pre-built templates to learn from
- **URL Sharing** - Share your creations with others
- **Local Persistence** - Your code survives page refreshes

## Getting Started

Navigate to [/playground](/playground) to access the playground.

### Interface Overview

```
┌─────────────────────────────────────────────────────────┐
│  ⚡ Nucleus / Playground    [Examples ▼]    [▶ Run]    │
├─────────────────────────────────────────────────────────┤
│ ┌─────────────────────┐ │ ┌─────────────────────────┐ │
│ │ 📄 main.ncl  🎨 css │ │ │ 👁 Preview  🔧 HTML    │ │
│ ├─────────────────────┤ │ ├─────────────────────────┤ │
│ │                     │ │ │                         │ │
│ │   <n:view>          │ │ │   ┌─────────────────┐   │ │
│ │     <h1>Hello</h1>  │ │ │   │  Hello World!   │   │ │
│ │   </n:view>         │ │ │   └─────────────────┘   │ │
│ │                     │ │ │                         │ │
│ └─────────────────────┘ │ └─────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│  ● Ready    |    Ctrl+Enter to run    |    v3.5       │
└─────────────────────────────────────────────────────────┘
```

## Writing Code

### NCL Tab

Write your Nucleus template in the NCL tab:

```xml
<n:view title="My App">
    <main class="container">
        <h1>Welcome to Nucleus!</h1>
        <Button variant="primary">Get Started</Button>
    </main>
</n:view>
```

### CSS Tab

Add custom styles in the CSS tab:

```css
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 100vh;
    background: #0f0f1a;
    color: white;
}
```

## Available Components

The playground supports these built-in components:

| Component | Usage |
|-----------|-------|
| `<Button>` | `<Button variant="primary">Click</Button>` |
| `<TextInput>` | `<TextInput name="email" label="Email" />` |
| `<Checkbox>` | `<Checkbox name="agree" label="I agree" />` |
| `<Card>` | `<Card variant="glass">Content</Card>` |
| `<StatCard>` | `<StatCard value="100" label="Users" />` |

### Button Variants

```xml
<Button variant="primary">Primary</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="ghost">Ghost</Button>
<Button variant="gradient">Gradient</Button>
```

### Button Sizes

```xml
<Button size="small">Small</Button>
<Button size="medium">Medium</Button>
<Button size="large">Large</Button>
```

## Example Templates

Use the dropdown to load pre-built examples:

### Basics
- **Hello World** - Simple welcome page
- **Counter** - Interactive counter with state
- **Form** - Contact form with validation

### Components
- **Card** - Card component variants
- **Button** - Button showcase
- **Wizard** - Multi-step form

### Advanced
- **Dashboard** - Admin dashboard layout
- **Auth** - Login form design

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl + Enter` | Run code |
| `Ctrl + S` | Save to browser storage |
| `Ctrl + Shift + F` | Format code |

## Sharing

1. Click the **Share** button in the header
2. Copy the generated URL
3. Share with others - they'll see your exact code

The URL contains your code encoded in the hash, so no server storage is needed.

## Tips & Tricks

### 1. Use the CSS Tab for Styling

All styles in the CSS tab are automatically injected into the preview:

```css
/* Custom gradient text */
.gradient-text {
    background: linear-gradient(135deg, #6366f1, #a855f7);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}
```

### 2. Add Interactivity

You can include `<script>` tags for dynamic behavior:

```xml
<n:view title="Interactive">
    <button id="btn">Click me</button>
    <p id="count">0</p>
    <script>
        let count = 0;
        document.getElementById('btn').onclick = () => {
            count++;
            document.getElementById('count').textContent = count;
        };
    </script>
</n:view>
```

### 3. Preview HTML Output

Switch to the **HTML Output** tab to see the generated HTML - useful for learning how components render.

### 4. Forms with Validation

Create forms with built-in validation:

```xml
<n:form action="/submit">
    <TextInput name="email" type="email" label="Email" required="true" />
    <TextInput name="password" type="password" label="Password" required="true" />
    <Button type="submit" variant="gradient">Submit</Button>
</n:form>
```

## Limitations

The playground is for prototyping and learning. Some limitations:

- **No Backend** - Forms won't actually submit
- **No Database** - `<n:model>` and `<n:action>` are stripped
- **Client-Side Only** - Server-side features don't work
- **Simplified Compilation** - Uses regex-based transformation

For full functionality, use the Nucleus CLI locally:

```bash
cargo install nucleus-cli
nucleus new my-app
cd my-app && make dev
```

## Related Guides

- [Quick Start Guide](#01_gettingstarted) - Install and create your first app
- [Components](#10_components_guide) - Full component reference
- [Forms Guide](#07_forms_and_validation) - Form validation deep dive
