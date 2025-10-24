# Change Log

All notable changes to the "Jounce" extension will be documented in this file.

## [0.1.0] - 2025-10-22

### Initial Release 🎉

This is the first official release of the Jounce VS Code extension!

### Features

#### LSP Integration (8 Major Features)
- **Context-Aware Completions** - 7 distinct completion contexts
  - Namespace access (after `::`)
  - Member access (after `.`)
  - JSX tags (after `<`)
  - JSX attributes (inside tags)
  - Statement start
  - Function calls
  - General context
- **Hover Information** - Rich type info for 7+ symbol types
  - Functions with full signatures
  - Components with parameters
  - Variables with type annotations
  - Constants
  - Structs with field definitions
  - Enums with variant listings
  - Stdlib functions with documentation
- **Signature Help** - Real-time parameter hints
  - Automatic function call detection
  - Parameter index tracking
  - Nested call support
- **Code Actions** - 6 quick fixes
  - "Did you mean?" typo corrections
  - Prefix unused variables with `_`
  - Add type casts
  - Add missing semicolons
  - Add type annotations
  - Remove unused imports
- **Advanced Navigation**
  - Go to Definition (F12)
  - Find References (Shift+F12)
  - Rename Symbol (F2)
  - Document Symbols (Cmd+Shift+O)
- **Code Formatting**
  - Format Document (Shift+Alt+F)
  - Format on Save (configurable)
  - Intelligent JSX multi-line layout
- **Diagnostics** - 23 error/warning codes
  - 18 error codes (E001-E018)
  - 5 warning codes (W001-W005)
  - "Did you mean?" suggestions
- **Inlay Hints** - Rust Analyzer style
  - Type hints for variables
  - Parameter hints in function calls

#### Syntax Highlighting
- Keywords (fn, let, if, match, etc.)
- Types (i32, f64, String, Vec, etc.)
- Operators (::, ->, =>, ..., etc.)
- Strings and numbers
- Comments (line and block)
- JSX elements, attributes, and expressions
- Annotations (@server, @client)

#### Extension Commands
- **Compile Current File** - Run `raven compile`
- **Watch Current File** - Run `raven watch` with auto-recompile
- **Format Document** - Format code
- **Run Tests** - Run `cargo test`
- **Show Profiling** - Display compilation timing

#### Settings (6 Configurable Options)
- `jounce.lspPath` - Path to raven binary
- `jounce.enableInlayHints` - Toggle inlay hints
- `jounce.enableTypeHints` - Toggle type hints
- `jounce.enableParameterHints` - Toggle parameter hints
- `jounce.formatOnSave` - Auto-format on save
- `jounce.trace.server` - LSP trace level

### Performance
- Activation time: < 500ms
- LSP response time: < 100ms for most operations
- Compilation speed: 96,292 compilations/sec (small programs)

### Requirements
- VS Code 1.80.0 or higher
- Jounce compiler (`raven`) in PATH

### Known Limitations
- Requires separate Jounce compiler installation
- Icon placeholder (will be updated in future release)

---

## [Unreleased]

### Planned Features
- Semantic highlighting
- Snippets for common patterns
- Custom extension icon
- Marketplace publication
- Auto-update notifications

---

**Note**: This extension is part of the Jounce project. For compiler changes, see the main [Jounce CHANGELOG](https://github.com/jounce/jounce/blob/main/CHANGELOG.md).
