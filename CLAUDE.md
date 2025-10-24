# CLAUDE.md - Jounce Development Guide

**Version**: 0.3.0 → v1.0.0
**Current Phase**: Phase 11 - Module System & Multi-File Support
**Last Updated**: October 24, 2025

---

## 🔄 Development Workflow (THE LOOP)

This is our development loop from v0.3.0 → v1.0.0:

1. **Work on Current Phase** (in this file)
   - Follow tasks listed below
   - Track progress with TodoWrite
   - Test as you go
   - Commit frequently

2. **Complete Phase Checklist**
   - ✅ All tasks done
   - ✅ Tests passing
   - ✅ Documentation written
   - ✅ Example code works

3. **Update ROADMAP.md**
   - Open ROADMAP.md
   - Check off completed phase
   - Note any new discoveries
   - Update current status

4. **Push to GitHub**
   - Commit with detailed message
   - Push to main branch
   - Update version if needed

5. **Start Next Phase**
   - Move to next phase in ROADMAP
   - Update this file with new phase
   - Repeat the loop

**Goal**: Continue this loop until Jounce can easily make apps, then build portfolio of apps.

---

## 📍 Current Status (v0.3.0)

**✅ Complete & Production-Ready**:
- Core compiler (lexer, parser, type checker, code gen)
- Standard library (JSON, DateTime, Crypto, File I/O, YAML) - 100% tested
- Developer tools (CLI, LSP, test framework, watch, HMR, cache)
- 638/638 tests passing (100%)
- 5 packages (router, http, forms, store, i18n)
- 102x faster builds with compilation cache

**⚠️ Blocking Issues**:
- Multi-file projects unclear (module system needs work)
- No reactivity system (signals/effects)
- Only 5 packages (need 50+)
- No example apps yet

---

## 🎯 Phase 11: Module System & Multi-File Support

**Goal**: Enable multi-file projects with clear import/export semantics
**Timeline**: 2-3 weeks
**Deliverable**: v0.3.1 with multi-file support

### Tasks

#### Week 1: Document & Design

- [ ] **Task 1: Document current module behavior**
  - Test `use` statements with local files
  - Test `use` statements with packages
  - Document what works vs what's broken
  - Create `docs/design/MODULE_SYSTEM.md`

- [ ] **Task 2: Design export keyword**
  - Define syntax: `export fn`, `export struct`, `export enum`
  - Design module path resolution (./file.jnc vs package::Item)
  - Write specification for import/export rules
  - Add examples to spec

- [ ] **Task 3: Test multi-file scenarios**
  - Create test project: math_lib/
    - `lib.jnc` (exports functions)
    - `main.jnc` (imports from lib)
  - Document current errors
  - List required compiler changes

#### Week 2: Implementation

- [ ] **Task 4: Implement export parsing**
  - Add `export` keyword to lexer (token.rs)
  - Parse `export` modifier in parser (parser.rs)
  - Update AST to track exported items (ast.rs)
  - Test: export functions, structs, enums

- [ ] **Task 5: Generate JavaScript exports**
  - Update js_emitter.rs to emit `export` statements
  - Handle `export fn` → `export function`
  - Handle `export struct` → `export class`
  - Test: compiled JS has correct exports

- [ ] **Task 6: Implement cross-file imports**
  - Parse `use ./file.jnc` syntax
  - Resolve file paths relative to importing file
  - Load and parse imported files
  - Build dependency graph

- [ ] **Task 7: Cache invalidation**
  - Track file dependencies in cache
  - Invalidate cache when dependencies change
  - Test: changing imported file triggers recompile
  - Add dependency tracking tests

#### Week 3: CLI & Examples

- [ ] **Task 8: Update CLI for directories**
  - `jnc compile src/` compiles all .jnc files in directory
  - Auto-detect entry point (main.jnc or first file)
  - Support `--entry` flag for custom entry point
  - Test with multi-file projects

- [ ] **Task 9: Build multi-file example app**
  - Create `examples/todo-app-multi-file/`
  - Files:
    - `main.jnc` - Entry point, calls other modules
    - `components.jnc` - JSX components
    - `api.jnc` - Server functions (@server)
    - `utils.jnc` - Helper functions
  - Document import/export usage

- [ ] **Task 10: Write module system docs**
  - Complete `docs/design/MODULE_SYSTEM.md`
  - Add section to GETTING_STARTED.md
  - Code examples for each use case
  - Best practices guide

- [ ] **Task 11: Testing**
  - Write 20+ tests for import/export
  - Test circular dependencies (should error)
  - Test deep nesting (a imports b imports c)
  - Test package imports still work

### Success Criteria

- ✅ Multi-file todo app compiles and runs
- ✅ Documentation complete (MODULE_SYSTEM.md)
- ✅ 20+ tests for import/export passing
- ✅ Cache correctly invalidates dependencies
- ✅ CLI `jnc compile src/` works
- ✅ All existing 638 tests still passing

### Files to Modify

- `src/token.rs` - Add `Export` token
- `src/parser.rs` - Parse export modifier
- `src/ast.rs` - Track exported items
- `src/js_emitter.rs` - Generate export statements
- `src/main.rs` - CLI support for directories
- `src/cache/dependency_graph.rs` - Track file dependencies
- `docs/design/MODULE_SYSTEM.md` - New documentation

---

## 🚀 Quick Commands

```bash
# Build & test
cargo build --release && cargo test

# Test specific module
cargo test --lib module_tests

# Compile single file
./target/release/jnc compile app.jnc

# Compile directory (after Phase 11)
./target/release/jnc compile src/

# Run with cache stats
./target/release/jnc compile app.jnc --verbose

# Watch mode
./target/release/jnc watch src --output dist
```

---

## 📂 Key Files Reference

### Core Compiler
- `src/lib.rs` - Main library interface
- `src/main.rs` - CLI entry point (1340 lines)
- `src/lexer.rs` - Tokenization
- `src/parser.rs` - AST construction
- `src/js_emitter.rs` - JavaScript code generation
- `src/type_checker.rs` - Type checking & borrow checking

### Module System (Phase 11 Focus)
- `src/token.rs` - Token definitions
- `src/ast.rs` - AST node definitions
- `src/cache/dependency_graph.rs` - Dependency tracking

### Standard Library
- `src/stdlib/json.rs` - JSON parsing (7 tests)
- `src/stdlib/time.rs` - DateTime (15 tests)
- `src/stdlib/crypto.rs` - Cryptography (25 tests)
- `src/stdlib/fs.rs` - File I/O (10 tests)
- `src/stdlib/yaml.rs` - YAML parsing (15 tests)

### Testing
- `tests/basic_tests.jnc` - Basic language tests
- `tests/test_json_parser.jnc` - JSON tests
- `tests/test_datetime.jnc` - DateTime tests
- `tests/test_crypto.jnc` - Crypto tests
- `tests/test_fs.jnc` - File I/O tests
- `tests/test_yaml.jnc` - YAML tests

---

## 🔧 Development Patterns

### Adding Language Features
1. Update token.rs if new keyword
2. Update ast.rs with new AST node
3. Update parser.rs to parse new syntax
4. Update js_emitter.rs to generate JavaScript
5. Update type_checker.rs if type rules needed
6. Add tests in tests/ directory
7. Update documentation

### Debugging Tests
```bash
# Run specific test with verbose output
jnc test --verbose --filter "test_name"

# Run all tests in a file
jnc test tests/my_tests.jnc

# See compiled JavaScript
jnc compile test.jnc
cat dist/server.js
```

### Debugging Compilation
```bash
# Enable verbose compiler output
RUST_LOG=debug cargo run --bin jnc -- compile app.jnc

# Check AST structure
cargo test --lib parse_specific_syntax -- --nocapture
```

---

## 📊 Test Status

**Total**: 638/638 (100%)
- Core: 564/564 (100%)
- Stdlib: 74/74 (100%)
  - JSON: 7/7
  - DateTime: 15/15
  - Crypto: 25/25
  - File I/O: 10/10
  - YAML: 15/15
  - JSX: 24/24

**Target after Phase 11**: 658+ tests (add 20+ module system tests)

---

## 🎯 Next Steps (START HERE)

### This Week (Tasks 1-3)

**Task 1: Document current module behavior** (2-3 hours)
- Create `examples/test-modules/` directory
- Try: `use ./math.jnc` and see what happens
- Try: `use jounce_http::HttpClient` and verify it works
- Document findings in `docs/design/MODULE_SYSTEM.md`

**Task 2: Design export keyword** (2-3 hours)
- Write syntax specification
- Define how exports work in JavaScript output
- Add examples of export usage
- Consider edge cases (what can't be exported?)

**Task 3: Test multi-file scenarios** (2-3 hours)
- Create small multi-file project
- Document every error you encounter
- List what compiler changes are needed
- Prioritize changes by importance

### This Weekend (Tasks 4-5)

**Task 4: Implement export parsing** (4-6 hours)
- Add `Export` token to lexer
- Parse export modifier in parser
- Update AST with export flag
- Write tests for export parsing

**Task 5: Generate JavaScript exports** (3-4 hours)
- Update js_emitter.rs
- Test with various export types
- Verify JavaScript is valid

---

## 📚 Archive

**Phase 10 Summary** (COMPLETE - Oct 24, 2025):
- Sprint 1: Fixed all 9 YAML tests → 100% stdlib coverage
- Sprint 2: Activated cache → 102x faster builds
- Sprint 3: Complete documentation → v0.3.0 released
- Sprint 4: Production CLI → colors, stats, HMR

**Detailed History**: See `docs/archive/CLAUDE_PHASE10.md`

---

**Last Updated**: October 24, 2025
**Current Focus**: Phase 11, Task 1 - Document module system
**Next Milestone**: v0.3.1 with multi-file support (2-3 weeks)
