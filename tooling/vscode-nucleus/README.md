# Nucleus Framework for VS Code

⚡ **Full-featured language support for the Nucleus Framework** - syntax highlighting, IntelliSense, snippets, and CLI integration.

![Nucleus Version](https://img.shields.io/badge/Nucleus-v3.1-00dc82?style=flat-square)
![VS Code](https://img.shields.io/badge/VS%20Code-1.85+-007ACC?style=flat-square)

## Features

### 🎨 Syntax Highlighting

Full syntax highlighting for `.ncl` files including:
- Nucleus tags (`n:view`, `n:for`, `n:if`, `n:model`, etc.)
- Component syntax (`n:component`, `n:props`, `n:slot`)
- Island hydration directives (`client:load`, `client:visible`)
- Template expressions (`{expression}` and `{{ interpolation }}`)
- Jinja-style control flow (`{% if %}`, `{% for %}`)
- Embedded Rust, CSS, and JavaScript

### 📝 60+ Snippets

Rapid development with comprehensive snippets:

| Prefix | Description |
|--------|-------------|
| `nview` | Create a Nucleus view/page |
| `nfor` | For loop over collection |
| `nif` | Conditional rendering |
| `ncomponent` | Full component with props and scoped styles |
| `nisland` | Interactive island component |
| `nform` | Form with CSRF protection |
| `use-photon` | Import Photon ORM |
| `use-fortress` | Import Fortress auth |
| `query` | Photon query builder pattern |
| `page-basic` | Complete page template |

[View all snippets →](./snippets/snippets.json)

### 🧠 IntelliSense (LSP)

Powered by `nucleus-lsp`:
- **Completions** for all Nucleus tags and attributes
- **Hover documentation** with examples
- **Diagnostics** for invalid tags and missing attributes
- **Go-to-definition** for layouts and components
- **Document symbols** for navigation

### ⚡ CLI Integration

Run Nucleus commands directly from VS Code:

| Command | Description |
|---------|-------------|
| `Nucleus: Start Dev Server` | Run `nucleus dev` |
| `Nucleus: Build for Production` | Run `nucleus build` |
| `Nucleus: Generate Scaffold` | Create CRUD scaffold |
| `Nucleus: Run Migrations` | Run `nucleus db up` |
| `Nucleus: Deploy` | Interactive deployment wizard |

## Installation

### From VS Code Marketplace

1. Open VS Code
2. Go to Extensions (Cmd+Shift+X)
3. Search for "Nucleus Framework"
4. Click Install

### From VSIX

```bash
code --install-extension nucleus-lang-1.0.0.vsix
```

## Requirements

- **VS Code** 1.85.0 or later
- **nucleus-lsp** binary for IntelliSense (optional but recommended)

### Installing nucleus-lsp

```bash
# From the nucleus-lang repository
cargo install --path crates/nucleus-lsp
```

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `nucleus.lsp.enabled` | `true` | Enable Language Server |
| `nucleus.lsp.path` | `nucleus-lsp` | Path to LSP binary |
| `nucleus.cli.path` | `nucleus` | Path to CLI binary |
| `nucleus.dev.port` | `3000` | Default dev server port |

## Snippet Prefixes

### Views & Layouts
- `nview` - Basic view
- `nview-layout` - View with layout
- `nview-auth` - Authenticated view
- `nlayout` - Layout wrapper
- `nslot` - Content slot
- `ninclude` - Include file

### Control Flow
- `nfor` - For loop
- `nfor-index` - For loop with index
- `nif` - If conditional
- `nif-else` - If-else
- `jif` - Jinja if
- `jfor` - Jinja for

### Components
- `ncomponent` - Full component
- `nprops` - Props definition
- `nisland` - Island component
- `nclient` - Client-side Rust

### Forms & Elements
- `nform` - Form
- `nimage` - Optimized image
- `nlink` - Navigation link

### Stdlib
- `use-photon` - Photon ORM
- `use-fortress` - Auth
- `use-neutron` - State management
- `use-stream` - WebSockets
- `query` - Query builder

### Complete Patterns
- `page-basic` - Full page template
- `page-auth` - Auth page
- `component-button` - Button component
- `form-login` - Login form
- `crud-list` - CRUD list view

## Contributing

Issues and PRs welcome at [github.com/nucleus-lang/nucleus-lang](https://github.com/nucleus-lang/nucleus-lang)

## License

MIT License - [Nucleus Framework](https://github.com/nucleus-lang/nucleus-lang)
