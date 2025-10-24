# CLAUDE.md - AI Assistant Guide for Jounce

## 📌 Project Overview

**Jounce** is a full-stack programming language that compiles `.jnc` source files into JavaScript (server + client) and WebAssembly.

**Key Innovation**: Write ONE `.jnc` file → Get separate `server.js` + `client.js` + `app.wasm` + `index.html` with automatic RPC generation.

## Quick Facts

- **Language**: Rust (compiler/toolchain)
- **Main Binary**: `jnc` (compiled from src/main.rs)
- **Library**: `jounce_compiler` (src/lib.rs)
- **File Extension**: `.jnc` (Jounce source files)
- **Config Files**: `jounce.toml` (package manifests)
- **Version**: 0.2.0
- **Test Coverage**: 558 tests (100% pass rate)
- **Compilation Speed**: 96,292 compilations/sec

## What's Working (100% Complete)

### Language Features
- ✅ **JSX** - Full JSX support with components
- ✅ **Async/Await** - Complete async support
- ✅ **Generics** - Generic functions with type erasure
- ✅ **Traits** - Full trait system with bounds
- ✅ **Pattern Matching** - Option<T>, Result<T,E>, destructuring
- ✅ **Closures** - Typed closures with capture
- ✅ **Recursion** - All patterns (factorial, fibonacci, mutual)
- ✅ **Try Operator (?)** - Ergonomic error propagation
- ✅ **Control Flow** - Unlimited nesting depth
- ✅ **Iteration** - For loops, while loops, ranges
- ✅ **Arrays** - Sized arrays [T; N] syntax

### CSS Integration (Phases 7.5 & 8)
- ✅ **css! macro** - Native CSS in Jounce
- ✅ **Scoped styles** - Hash-based class names
- ✅ **Selectors** - Class, ID, element, pseudo (`:hover`, `:focus`)
- ✅ **CSS nesting** - `&` selector support
- ✅ **Media queries** - `@media` with complex conditions
- ✅ **Keyframe animations** - `@keyframes` with scoped names
- ✅ **Dynamic CSS values** - `{expr}` syntax for variables
- ✅ **Utility classes** - 150+ Tailwind-like utilities
- ✅ **Variants** - Responsive (`sm:`, `md:`, etc.) and state (`hover:`, `focus:`)
- ✅ **Custom utilities** - User-defined utilities from config
- ✅ **Design tokens** - JSON/YAML token loading
- ✅ **Theme switching** - CSS custom properties
- ✅ **Dark mode** - `dark:` variant support

### Developer Experience
- ✅ **LSP** - 8 major features (completions, hover, formatting, etc.)
- ✅ **Watch mode** - Auto-recompile with debouncing
- ✅ **Code formatting** - `jnc fmt` command
- ✅ **VS Code extension** - Full IDE support
- ✅ **Package manager** - `init`, `add`, `remove`, `tree` commands

### Examples & Documentation
- ✅ **48 examples** - Complete examples from basics to advanced
- ✅ **Comprehensive docs** - Guides, API reference, tutorials
- ✅ **Real-world apps** - Todo, e-commerce, social, etc.

## Compiler Pipeline

```
.jnc source
    ↓
[Lexer] → [Parser] → [Semantic Analyzer] → [Type Checker] → [Borrow Checker]
    ↓
[Code Splitter] → [RPC Generator]
    ↓
[JS Emitter] → [WASM Generator] → [CSS Generator] → [Utility Generator]
    ↓
Output: dist/server.js, dist/client.js, dist/app.wasm, dist/index.html, dist/styles.css
```

## Development Commands

### Building & Testing
```bash
cargo build --release              # Build compiler
cargo test                         # Run all tests (558 passing)
cargo bench                        # Run benchmarks
```

### Compiling .jnc Files
```bash
./target/release/jnc compile app.jnc
./target/release/jnc compile app.jnc --minify
./target/release/jnc compile app.jnc --profile
```

### Watch Mode
```bash
jnc watch app.jnc                  # Watch & auto-recompile
```

### Code Formatting
```bash
jnc fmt file.jnc                   # Print formatted output
jnc fmt --write file.jnc           # Format in place
```

### Package Management
```bash
jnc init                           # Initialize new project
jnc add package-name               # Add dependency
jnc remove package-name            # Remove dependency
jnc tree                           # Show dependency tree
```

## 🗺️ Current Roadmap - Phase 9

**Goal**: Production-Ready Ecosystem & Advanced Features

### Sprint 1: Performance Optimization (NEXT)
**Duration**: ~8 hours
**Objectives**:
1. **Incremental Compilation** - Cache AST and type information between builds
2. **Parallel Compilation** - Multi-threaded compilation for large projects
3. **Build Caching** - Smart dependency tracking and invalidation
4. **Bundle Optimization** - Tree-shaking, dead code elimination, code splitting
5. **Benchmarking Suite** - Comprehensive performance metrics

### Sprint 2: Developer Tools Enhancement
**Duration**: ~10 hours
**Objectives**:
1. **Debugger Integration** - Source maps, breakpoints, step-through debugging
2. **Error Messages** - Improved error messages with suggestions and hints
3. **LSP Enhancements** - Refactoring tools, auto-imports, code lens
4. **Test Framework** - Built-in unit testing and integration testing
5. **REPL** - Interactive Jounce shell for experimentation

### Sprint 3: Standard Library Expansion
**Duration**: ~12 hours
**Objectives**:
1. **Collections** - Vector, HashMap, HashSet, BTreeMap, etc.
2. **File I/O** - Reading/writing files, directory operations
3. **Networking** - HTTP client/server, WebSocket support
4. **Date/Time** - Comprehensive date and time handling
5. **Cryptography** - Hashing, encryption, secure random
6. **JSON/YAML** - Native parsing and serialization

### Sprint 4: WebAssembly Enhancement
**Duration**: ~10 hours
**Objectives**:
1. **WASM Optimization** - Smaller bundle sizes, better performance
2. **WASM Interop** - Better JS ↔ WASM communication
3. **Memory Management** - Efficient memory allocation and GC
4. **SIMD Support** - Vector operations for performance-critical code
5. **Threading** - Web Workers and SharedArrayBuffer support

### Sprint 5: Registry & Distribution
**Duration**: ~8 hours
**Objectives**:
1. **Deploy Registry Backend** - Launch production package registry
2. **Publishing Workflow** - Streamlined package publishing
3. **Versioning & Semver** - Proper version resolution and conflicts
4. **Private Packages** - Support for private/scoped packages
5. **Package Discovery** - Search, categories, documentation

### Sprint 6: Production Hardening
**Duration**: ~6 hours
**Objectives**:
1. **Security Audit** - Review and fix security vulnerabilities
2. **Stability Testing** - Edge cases, error handling, recovery
3. **Documentation Polish** - Complete all guides and tutorials
4. **Migration Guide** - Tools and docs for migrating existing code
5. **1.0 Release Prep** - Changelog, release notes, announcement

## File Structure

**Core Implementation**:
```
src/
├── main.rs                      - CLI entry point
├── lib.rs                       - Library interface
├── lexer.rs                     - Tokenization with JSX & CSS
├── parser.rs                    - Recursive descent parser
├── ast.rs                       - Abstract Syntax Tree
├── semantic_analyzer.rs         - Scope resolution
├── type_checker.rs              - Hindley-Milner type inference
├── borrow_checker.rs            - Memory safety analysis
├── codegen.rs                   - Code generation coordination
├── js_emitter.rs                - JavaScript emission
├── css_generator.rs             - CSS code generation
├── utility_generator.rs         - Utility class generation
├── utility_config.rs            - Utility configuration
├── design_tokens.rs             - Design token parsing
├── formatter.rs                 - Code formatting
├── watcher.rs                   - File watching
├── lsp/mod.rs                   - Language Server Protocol
└── package_manager/             - Package management
    ├── mod.rs                   - Package manager core
    └── registry.rs              - Registry client
```

**Configuration**:
```
jounce.toml                      - Package manifest
jounce.config.toml               - Project configuration (optional)
```

**Examples & Documentation**:
```
examples/                        - 48 complete examples
docs/                            - Comprehensive documentation
docs/archive/                    - Historical phase archives
```

## Code Style Guidelines

### Rust Code (Compiler)
- Use `rustfmt` for formatting
- Prefer explicit types in public APIs
- Document public functions with `///`
- Use Result<T, E> for fallible operations
- Avoid unwrap() in production code paths

### Jounce Code (Examples/Tests)
- 4-space indentation
- Explicit return statements preferred
- Type annotations on function signatures
- Component names in PascalCase
- Function names in snake_case

## Git Workflow

### Commit Message Style
```
feat: Add feature description
fix: Fix bug description
docs: Update documentation
perf: Performance improvement
refactor: Code refactoring
test: Add or update tests
```

## Resources

- **Main Docs**: README.md, GETTING_STARTED.md
- **Phase Archives**: `docs/archive/` (Complete history of Phases 1-8)
- **Guides**: `docs/guides/` (CSS_SYNTAX.md, LSP_FEATURES.md, etc.)
- **API Reference**: `docs/guides/STDLIB_API_REFERENCE.md`
- **Examples**: `examples/` (48 complete examples organized by topic)

## Version History

- **0.2.0** - Current version with full CSS system and utilities
- **0.1.0** - Initial alpha release with core language features

## Notes for AI Assistant

- **Project renamed from RavensOne to Jounce** ✅ COMPLETE
- **File extension changed from .raven to .jnc** ✅ COMPLETE
- **Binary renamed from raven to jnc** ✅ COMPLETE
- **Package manifests renamed from raven.toml to jounce.toml** ✅ COMPLETE
- **All historical documentation archived in docs/archive/** ✅ COMPLETE
- **All runtime references verified and updated** (458 files total)

---

**Last Updated**: 2025-10-23 (Project Rename Complete & Verified)
**Compiler Version**: 0.2.0
**Status**: 🎉 **Ready for Phase 9!** All core features complete, 558 tests passing.

**Next**: Phase 9 Sprint 1 - Performance Optimization
