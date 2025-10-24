# Jounce Language Support for VSCode

Official VSCode extension for the Jounce programming language - the AI-first full-stack web framework.

## Features

### Syntax Highlighting ✨
Beautiful syntax highlighting for `.jnc` files with support for:
- Keywords (`component`, `let`, `fn`, `server`, `client`)
- Types (`Int`, `Float`, `String`, `Bool`, etc.)
- Reactive primitives (`Signal`, `Computed`, `Effect`, `Resource`)
- JSX/TSX-like component syntax
- Comments, strings, numbers

### Code Snippets 🚀
Fast scaffolding with built-in snippets:
- `comp` - Create a new component
- `sig` - Create a Signal
- `eff` - Create an Effect
- `serverfn` - Create a server function
- `jsx` - Create JSX elements
- ... and more!

### Smart Bracket Matching 🎯
Auto-closing and surrounding pairs for:
- Curly braces `{}`
- Brackets `[]`
- Parentheses `()`
- JSX tags `<>`
- Quotes `""` and `''`

### Code Folding 📂
Fold/unfold regions for better code organization

## Installation

### From VSCode Marketplace (Coming Soon)
1. Open VSCode
2. Press `Ctrl+P` (or `Cmd+P` on Mac)
3. Type `ext install jounce.jnc-lang`
4. Press Enter

### From Source
```bash
cd vscode-raven
npm install
npm run compile
# Press F5 to launch extension development host
```

## Requirements

- VSCode 1.80.0 or higher
- Jounce compiler installed (optional, for compilation features)

## Extension Settings

This extension contributes the following settings:

* `raven.compilerPath`: Path to the Jounce compiler executable (default: `raven`)
* `raven.enableLinting`: Enable real-time linting (default: `true`)
* `raven.formatOnSave`: Automatically format on save (default: `false`)
* `raven.trace.server`: Traces communication with language server (default: `off`)

## Commands

- **Jounce: Compile File** - Compile the current `.jnc` file
- **Jounce: Type Check** - Run type checker without compiling
- **Jounce: Format Document** - Format the current document
- **Jounce: New Component** - Create a new component from template

## Roadmap

### Version 0.2.0 (In Progress)
- [ ] Language Server Protocol (LSP) support
- [ ] IntelliSense / autocomplete
- [ ] Hover documentation
- [ ] Go to definition
- [ ] Find references
- [ ] Rename symbol

### Version 0.3.0
- [ ] Real-time error diagnostics
- [ ] Quick fixes and code actions
- [ ] Refactoring support
- [ ] Debugger integration

### Version 0.4.0
- [ ] Integrated terminal for `raven` commands
- [ ] Component preview panel
- [ ] File watchers for HMR
- [ ] Performance profiling tools

## Known Issues

- Language Server Protocol implementation is in progress
- Some advanced IntelliSense features are not yet available
- Debugger integration is planned for future releases

## Contributing

Contributions are welcome! Please check out our [GitHub repository](https://github.com/jrezin1201/Jounce).

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Release Notes

### 0.1.0 (Initial Release)

- ✨ Syntax highlighting for `.jnc` files
- 📝 Code snippets for common patterns
- 🎯 Auto-closing brackets and tags
- 📂 Code folding support
- 🎨 Jounce Dark theme

---

## About Jounce

Jounce is a modern full-stack web framework designed for AI-assisted development. It features:

- **Type-safe** with Hindley-Milner type inference
- **Reactive** state management with Signals
- **Server-side rendering** (SSR) with progressive hydration
- **Compiles to WebAssembly** for near-native performance
- **Single-file components** with JSX-like syntax
- **Server functions** for seamless full-stack development

Learn more at [jounce.dev](https://jounce.dev) (coming soon)

## License

MIT License - see LICENSE file for details

---

**Enjoy coding with Jounce!** 🎉
