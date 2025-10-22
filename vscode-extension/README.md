# RavensOne for Visual Studio Code

Official Visual Studio Code extension for the **RavensOne** full-stack programming language.

## 🚀 Features

### Intelligent Code Completion
- Context-aware completions based on cursor position
- 7 distinct completion contexts (namespace, member access, JSX, etc.)
- 70+ documented stdlib functions
- Smart filtering by type compatibility

### Rich Type Information
- Hover to see comprehensive type info for functions, variables, components, structs, and enums
- Real-time parameter hints while typing function calls
- Inlay type hints for variables without explicit types
- Parameter name hints in function calls

### Advanced Navigation
- **Go to Definition** (F12) - Jump to symbol definitions
- **Find References** (Shift+F12) - Find all usages of a symbol
- **Rename Symbol** (F2) - Safe cross-file renaming with validation
- **Document Symbols** (Cmd+Shift+O) - Outline view of file structure

### Code Actions & Quick Fixes
- Auto-correct typos with "Did you mean?" suggestions
- Prefix unused variables with `_`
- Add type casts automatically
- Add missing semicolons
- Add missing type annotations
- Remove unused imports

### Code Formatting
- Format Document (Shift+Alt+F)
- Format on Save (configurable)
- Intelligent JSX formatting with multi-line layout
- Smart attribute formatting

### Diagnostics
- 18 error codes (E001-E018) covering common errors
- 5 warning codes (W001-W005) for code quality
- "Did you mean?" suggestions for typos
- Beautiful error messages with source snippets

### Syntax Highlighting
- Full language support including JSX
- Keywords, types, operators, strings, numbers
- JSX tags, attributes, and expressions
- Annotations (@server, @client)
- Comments and documentation

### Extension Commands
- **RavensOne: Compile Current File** - Compile .raven file to JavaScript
- **RavensOne: Watch Current File** - Auto-recompile on save
- **RavensOne: Format Document** - Format code
- **RavensOne: Run Tests** - Run compiler tests
- **RavensOne: Show Profiling** - Display compilation timing breakdown

## 📦 Installation

### Prerequisites
You need the RavensOne compiler installed and available in your PATH:

```bash
# Check if raven is installed
raven --version
```

If not installed, follow the [RavensOne Installation Guide](https://github.com/ravensone/ravensone).

### Install Extension
1. Open VS Code
2. Press `Cmd+Shift+X` (Mac) or `Ctrl+Shift+X` (Windows/Linux)
3. Search for "RavensOne"
4. Click "Install"

## 🎯 Getting Started

1. Create a new `.raven` file
2. Start typing - you'll get completions automatically
3. Hover over symbols to see type information
4. Use `F12` to jump to definitions
5. Press `Shift+Alt+F` to format your code

### Example RavensOne Code

```raven
use std::collections::Vec;

@server
fn process_data(input: String) -> Result<Vec<i32>, String> {
    let numbers = vec![1, 2, 3, 4, 5];

    match numbers.len() {
        0 => Err("Empty list"),
        n => Ok(numbers)
    }
}

@client
fn render_ui() -> JSX {
    let count = Signal::new(0);

    <div class="container">
        <h1>Counter: {count.get()}</h1>
        <button onClick={|| count.update(|c| c + 1)}>
            Increment
        </button>
    </div>
}
```

## ⚙️ Extension Settings

This extension contributes the following settings:

- `ravensone.lspPath`: Path to the RavensOne compiler binary (default: `"raven"`)
- `ravensone.enableInlayHints`: Enable/disable inlay hints (default: `true`)
- `ravensone.enableTypeHints`: Show type hints for variables (default: `true`)
- `ravensone.enableParameterHints`: Show parameter names in function calls (default: `true`)
- `ravensone.formatOnSave`: Automatically format files on save (default: `false`)
- `ravensone.trace.server`: LSP server trace level - `"off"`, `"messages"`, `"verbose"` (default: `"off"`)

### Example Configuration

```json
{
  "ravensone.lspPath": "/usr/local/bin/raven",
  "ravensone.enableInlayHints": true,
  "ravensone.formatOnSave": true
}
```

## 🎨 Language Features

### Supported Language Constructs
- Variables and constants (`let`, `const`, `mut`)
- Functions (`fn`, async functions)
- Structs and enums
- Pattern matching (`match`)
- JSX syntax
- Annotations (`@server`, `@client`)
- Modules and imports (`use`, `mod`)
- Standard library (70+ functions)
- Reactive state (`Signal`)

### LSP Features
- ✅ Completions
- ✅ Hover Information
- ✅ Signature Help
- ✅ Go to Definition
- ✅ Find References
- ✅ Rename Symbol
- ✅ Document Symbols
- ✅ Code Actions
- ✅ Formatting
- ✅ Diagnostics
- ✅ Inlay Hints

## 🐛 Known Issues

- The RavensOne compiler must be installed separately
- LSP server requires `raven lsp` command to be available
- Icon placeholder (will be updated in next release)

## 📝 Release Notes

### 0.1.0 (Initial Release)

**Features:**
- ✅ Full LSP integration with 8 major features
- ✅ Context-aware completions (7 contexts)
- ✅ Hover information for 7+ symbol types
- ✅ Signature help with parameter tracking
- ✅ Code actions with 6 quick fixes
- ✅ Advanced navigation (Go-to-def, Find refs, Rename, Document symbols)
- ✅ Code formatting (textDocument/formatting + rangeFormatting)
- ✅ Comprehensive diagnostics (23 error/warning codes)
- ✅ Inlay hints (type + parameter hints)
- ✅ Beautiful syntax highlighting
- ✅ 5 extension commands
- ✅ 6 configurable settings

## 🤝 Contributing

Found a bug or have a feature request? Please open an issue on the [GitHub repository](https://github.com/ravensone/ravensone).

## 📄 License

MIT License - See LICENSE file for details.

## 🔗 Resources

- [RavensOne Documentation](https://github.com/ravensone/ravensone)
- [Language Guide](https://github.com/ravensone/ravensone/docs)
- [Standard Library API](https://github.com/ravensone/ravensone/docs/guides/STDLIB_API_REFERENCE.md)
- [Report Issues](https://github.com/ravensone/ravensone/issues)

---

**Made with ❤️ by the RavensOne team**
