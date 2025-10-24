# CLAUDE.md - AI Assistant Guide for Jounce

## 📌 Project Overview

**Jounce** is a full-stack programming language that compiles `.jnc` source files into JavaScript + WebAssembly.

**Key Innovation**: Write ONE `.jnc` file → Get `server.js` + `client.js` + `app.wasm` + `index.html` with automatic RPC generation.

## Quick Facts

- **Version**: 0.2.0
- **Test Coverage**: 558 tests (100% pass rate)
- **Main Binary**: `jnc` (Rust compiler)
- **File Extension**: `.jnc`
- **Config Files**: `jounce.toml`

## What's Complete

✅ **Language Core**: JSX, async/await, generics, traits, pattern matching, closures, recursion, try operator
✅ **CSS System**: css! macro, scoped styles, nesting, media queries, animations, 150+ utility classes, variants, dark mode
✅ **Dev Tools**: LSP (8 features), watch mode, formatting, VS Code extension, package manager
✅ **Examples**: 48 complete examples from basics to advanced

📚 **Full feature list**: See `docs/archive/` for complete Phase 1-8 history

## Development Commands

```bash
# Build & Test
cargo build --release
cargo test
cargo bench

# Compile
jnc compile app.jnc [--minify] [--profile]
jnc watch app.jnc
jnc fmt [--write] file.jnc

# Package Management
jnc init
jnc add/remove package-name
jnc tree
```

## 🚀 Phase 9 Roadmap - Production Ready

### Sprint 1: Performance Optimization (COMPLETE!)
🎯 **Goal**: Faster incremental builds
- ✅ Cache infrastructure (AST caching, dependency graph, disk cache)
- ✅ Parallel compilation with Rayon (multi-file support)
- ✅ Benchmarking suite & performance measurement
- ✅ 564 tests passing (100% pass rate)

**Benchmark Results**:
- Cold cache: ~5.96ms compile time
- Warm cache: ~5.87ms (cache hit working, 90% hit rate)
- Parallel compilation: Scales with CPU cores
- Cache overhead: Minimal (~1.5% on cold, 0% on warm)

### Sprint 2: Developer Tools (NEARLY COMPLETE!)
🎯 **Goal**: Enhanced developer experience
- ✅ Error reporting system verified (production-ready: 873 lines, rich diagnostics, 23 error codes)
- ✅ Source maps verified (production-ready: 356 lines, v3 spec, VLQ encoding)
- ✅ LSP refactoring verified (production-ready: 4480 lines, rename, find-refs, go-to-def)
- ✅ Test framework designed (comprehensive 357-line design doc)
- ✅ Test runner implemented (314 lines: discovery, runner, 13 assertions)
- ⏳ REPL infrastructure (deferred to Sprint 3)

**Production-Ready Features**:
- 📝 **Error Diagnostics** (873 lines): Rich error messages with colors, source snippets, "did you mean?" suggestions, 18 error codes + 5 warning codes
- 🔍 **Source Maps** (356 lines): Full Source Map v3 implementation with VLQ encoding, inline & external support
- 🔧 **LSP Refactoring** (4480 lines): Rename symbol, find all references, go to definition, code actions framework
- 🧪 **Test Framework** (314 lines): Test discovery, runner generation, 13 assertion functions, async support

**Test Framework Features**:
- Test discovery (scans for test_* functions)
- Test runner generation (JavaScript execution code)
- 13 assertion functions (assert, assert_eq, assert_contains, etc.)
- Async test support
- Beautiful output with ✓/✗ symbols and timing

### Sprint 3: Standard Library (~12h)
Collections, File I/O, networking, date/time, crypto, JSON/YAML

### Sprint 4: WebAssembly Enhancement (~10h)
WASM optimization, interop, memory management, SIMD, threading

### Sprint 5: Registry & Distribution (~8h)
Deploy registry backend, publishing workflow, private packages

### Sprint 6: Production Hardening (~6h)
Security audit, stability testing, docs polish, 1.0 release prep

## Key Files

```
src/
├── main.rs, lib.rs              - CLI & library interface
├── lexer.rs, parser.rs, ast.rs  - Frontend
├── semantic_analyzer.rs         - Scope resolution
├── type_checker.rs              - Type inference
├── borrow_checker.rs            - Memory safety
├── codegen.rs, js_emitter.rs    - Code generation
├── css_generator.rs             - CSS generation
├── utility_generator.rs         - Utility classes
├── cache/                       - Compilation cache (NEW!)
│   ├── mod.rs                   - Cache manager
│   ├── ast_cache.rs             - AST caching
│   ├── dependency_graph.rs      - Dependency tracking
│   ├── disk_cache.rs            - Persistent storage
│   └── compile_cached.rs        - Cached compilation
├── lsp/mod.rs                   - Language server
└── package_manager/             - Package management
```

## Resources

- **Docs**: `README.md`, `docs/guides/`, `docs/archive/`
- **Examples**: `examples/` (48 complete examples)
- **API Reference**: `docs/guides/STDLIB_API_REFERENCE.md`

---

**Last Updated**: 2025-10-23
**Status**: 🎉 Phase 9 Sprint 2 NEARLY COMPLETE! Error system ✅, source maps ✅, LSP ✅, test framework ✅ (design + implementation). 5/6 major features done!
