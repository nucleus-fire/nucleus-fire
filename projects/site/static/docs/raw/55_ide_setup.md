# IDE Setup Guide

Nucleus provides a first-class developer experience with the **Nucleus Language Server (LSP)**. This guide covers how to set up your editor for `.ncl` support, syntax highlighting, and auto-completion.

## 1. VS Code

We provide an official VS Code extension.

### Installation

1.  Open VS Code.
2.  Go to the **Extensions** view (`Ctrl+Shift+X`).
3.  Search for **"Nucleus"**.
4.  Install the **Nucleus Language Support** extension.

### Manual Configuration

If you are using the CLI-based LSP manually, add this to your `settings.json`:

```json
{
  "nucleus.lsp.path": "nucleus-lsp",
  "files.associations": {
    "*.ncl": "nucleus"
  }
}
```

## 2. Neovim

You can use the built-in LSP client in Neovim (v0.8+).

### Using `nvim-lspconfig`

Add the following to your `init.lua`:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.nucleus then
  configs.nucleus = {
    default_config = {
      cmd = { 'nucleus-lsp' },
      filetypes = { 'nucleus', 'ncl' },
      root_dir = lspconfig.util.root_pattern('nucleus.config', '.git'),
      settings = {},
    },
  }
end

lspconfig.nucleus.setup {}
```

### Treesitter Support

For syntax highlighting, ensure you have the Nucleus parser installed:

```bash
:TSInstall nucleus
```

## 3. Zed

Zed supports Nucleus out of the box via the extension.

1.  Open the Command Palette (`Cmd+Shift+P`).
2.  Search for **"Install Extension"**.
3.  Select **"Nucleus"**.

## 4. Other Editors

For any editor that supports LSP (Sublime Text, Emacs, Helix), configure it to start the `nucleus-lsp` binary for `.ncl` files.

**Command**: `nucleus-lsp`
**Filetypes**: `.ncl`
