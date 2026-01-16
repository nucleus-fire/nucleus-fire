# Tooling & Editor Support

Nucleus provides a first-class developer experience through its dedicated VS Code Extension and CLI tools.

## VS Code Extension

The official Nucleus extension offers comprehensive language support for `.ncl` (View) files and Nucleus-specific Rust macros.

### Syntax Highlighting
Full syntax highlighting for:
- **Nucleus Tags**: `n:view`, `n:for`, `n:if`, `n:else`, `n:slot`, `n:text`.
- **Embedded Rust**: Code inside `{ ... }` blocks or strings is highlighted as Rust.
- **Attributes**: Special highlighting for `n:model`, `n:on` directives.

### IntelliSense (LSP)
The extension includes a built-in Language Server that provides:
- **Signature Help**: Hover over Nucleus tags to see docs.
- **Tag Completion**: Auto-close tags and suggest available attributes.
- **Feature Awareness**: Dynamic completion for `content.deck` localization keys. 
    - Type `<n:text key="` and immediately see a list of your app's translation keys (e.g., `app.welcome`).

### Snippets
 ускорьте разработку with built-in snippets:

| Trigger | Description | Output |
| :--- | :--- | :--- |
| `nview` | Create a new View component | `<n:view title="...">...</n:view>` |
| `ntext` | Insert localized text | `<n:text key="..." />` |
| `nslot` | Define a slot | `<n:slot name="..." />` |
| `nform` | Skeleton POST form | `<form method="POST">...</form>` |
| `server` | Rust: Server Action | `#[server] pub async fn ...` |
| `nerror` | Rust: Error Enum | `pub enum NucleusError { ... }` |

## CLI Tools
(See [CLI Reference](#17_cli_reference) for full details)

- `nucleus run`: Starts the development server with hot-reload.
- `nucleus new`: Scaffolds new projects or sites.
- `nucleus generate`: Creates models, controllers, and views.
