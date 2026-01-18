# Tooling & Editor Support

Nucleus provides a comprehensive developer experience through its VS Code extension, Language Server, and CLI tools.

---

## VS Code Extension

The official **Nucleus Framework** extension (`nucleus-lang`) offers full-featured language support for `.ncl` files.

### Installation

1. Open VS Code
2. Go to Extensions (`Ctrl+Shift+X`)
3. Search for "**Nucleus Framework**"
4. Click **Install**

Or install via CLI:
```bash
code --install-extension nucleus-framework.nucleus-lang
```

### Features

#### Syntax Highlighting
Comprehensive highlighting for all NCL elements:

| Category | Tags |
|----------|------|
| **Structural** | `n:view`, `n:layout`, `n:slot`, `n:outlet`, `n:include` |
| **Control Flow** | `n:for`, `n:if`, `n:else` |
| **Data** | `n:model`, `n:action`, `n:load` |
| **Client-Side** | `n:island`, `n:hydrate`, `n:client`, `n:script` |
| **Elements** | `n:form`, `n:image`, `n:link`, `n:text` |
| **Components** | `n:component`, `n:props`, PascalCase custom components |

#### Embedded Language Support
The extension provides proper syntax highlighting for:
- **Rust** inside `<n:action>`, `<n:load>`, and `{ }` expressions
- **CSS** inside `<style>` and `<style scoped>` blocks
- **JavaScript** inside `<n:script>` blocks

#### IntelliSense (LSP)
The built-in Language Server provides:
- **Hover Documentation**: See docs for any Nucleus tag
- **Auto-completion**: Tags, attributes, and values
- **Signature Help**: Parameter hints for complex tags
- **Content.deck Keys**: Autocomplete for i18n translation keys

#### Code Snippets

| Prefix | Description |
|--------|-------------|
| `nview` | New view with layout |
| `ncomp` | New component with slot |
| `nform` | Form with fields |
| `nisland` | Client-side island |
| `nsignal` | Neutron signal |
| `nlink` | Client navigation link |
| `naction` | Server action block |
| `nquery` | Photon database query |
| `nconfig` | nucleus.config template |

### Commands

Access via Command Palette (`Ctrl+Shift+P`):

| Command | Description |
|---------|-------------|
| `Nucleus: Start Dev Server` | Run `nucleus dev` |
| `Nucleus: Build for Production` | Run `nucleus build` |
| `Nucleus: Run Tests` | Run `nucleus test` |
| `Nucleus: Create New Project` | Run `nucleus new` |
| `Nucleus: Generate Scaffold` | Create model + views |
| `Nucleus: Generate Model` | Create model file |
| `Nucleus: Generate Migration` | Create migration |
| `Nucleus: Run Migrations` | Run `nucleus db up` |
| `Nucleus: Rollback Migration` | Run `nucleus db down` |
| `Nucleus: Migration Status` | Show pending migrations |
| `Nucleus: Deploy` | Deploy application |
| `Nucleus: Static Export` | Export as static site |
| `Nucleus: Restart Language Server` | Restart LSP |

### Configuration

In VS Code settings (`settings.json`):

```json
{
  "nucleus.lsp.enabled": true,
  "nucleus.lsp.path": "nucleus-lsp",
  "nucleus.cli.path": "nucleus",
  "nucleus.dev.port": 3000
}
```

---

## Language Server (LSP)

Nucleus includes a standalone Language Server for editor-agnostic support.

### Installation

The LSP is bundled with the Nucleus CLI:
```bash
# Already installed with nucleus
nucleus-lsp --version
```

### Capabilities

| Feature | Status |
|---------|--------|
| Syntax validation | ✅ |
| Hover documentation | ✅ |
| Go to definition | ✅ |
| Auto-completion | ✅ |
| Signature help | ✅ |
| Document symbols | ✅ |
| Workspace symbols | ✅ |
| Diagnostics | ✅ |
| Code actions | ✅ |

### Editor Integration

#### Neovim (nvim-lspconfig)

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.nucleus then
  configs.nucleus = {
    default_config = {
      cmd = { 'nucleus-lsp' },
      filetypes = { 'nucleus', 'ncl' },
      root_dir = lspconfig.util.root_pattern('nucleus.config', '.git'),
    },
  }
end

lspconfig.nucleus.setup {}
```

#### Helix

In `~/.config/helix/languages.toml`:
```toml
[[language]]
name = "nucleus"
scope = "source.ncl"
file-types = ["ncl"]
language-servers = ["nucleus-lsp"]

[language-server.nucleus-lsp]
command = "nucleus-lsp"
```

#### Sublime Text (LSP)

In LSP settings:
```json
{
  "clients": {
    "nucleus": {
      "enabled": true,
      "command": ["nucleus-lsp"],
      "selector": "source.ncl"
    }
  }
}
```

---

## CLI Tools

The `nucleus` CLI is your primary development tool.

### Core Commands

| Command | Description |
|---------|-------------|
| `nucleus new <name>` | Create new project |
| `nucleus dev` | Start dev server with HMR |
| `nucleus run` | Start production server |
| `nucleus build` | Compile for production |
| `nucleus test` | Run test suite |
| `nucleus check` | Type-check without building |

### Database Commands

| Command | Description |
|---------|-------------|
| `nucleus db new <name>` | Create migration |
| `nucleus db up` | Apply migrations |
| `nucleus db down` | Rollback last migration |
| `nucleus db status` | Show migration status |
| `nucleus db seed` | Run seeders |

### Generator Commands

| Command | Description |
|---------|-------------|
| `nucleus generate scaffold <name>` | Full CRUD |
| `nucleus generate model <name>` | Model only |
| `nucleus generate view <name>` | View only |
| `nucleus generate migration <name>` | Migration only |
| `nucleus generate component <name>` | Component only |

### Deployment Commands

| Command | Description |
|---------|-------------|
| `nucleus deploy` | Interactive deployment |
| `nucleus export` | Static site generation |
| `nucleus publish` | Publish to hosting |

See [CLI Reference](#17_cli_reference) for complete documentation.

---

## DevTools

### Browser DevTools

Nucleus integrates with browser developer tools:
- **Network Panel**: All requests show route names
- **Console**: Structured logging with component context
- **Performance**: HMR timing and hydration metrics

### Debug Mode

Enable verbose output:
```bash
NUCLEUS_DEBUG=1 nucleus dev
```

### Performance Profiling

```bash
nucleus run --profile
# Generates profile.json for Chrome DevTools import
```
