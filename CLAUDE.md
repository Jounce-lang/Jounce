# CLAUDE.md - Jounce Development Guide

**Version**: v0.3.1 (Phase 11 Complete)
**Current Phase**: Phase 12 - Reactive State Management
**Last Updated**: October 24, 2025

---

## 🔄 Development Workflow (THE LOOP)

This is our development loop from v0.3.1 → v1.0.0:

1. **Work on Current Phase** - Follow tasks, track with TodoWrite, commit frequently
2. **Complete Phase Checklist** - All tasks done, tests passing, docs written
3. **Update ROADMAP.md** - Check off phase, note discoveries
4. **Push to GitHub** - Commit with detailed message
5. **Start Next Phase** - Move to next phase, repeat loop

**Goal**: Continue this loop until Jounce can easily make apps, then build portfolio of apps.

---

## 📍 Current Status (v0.3.1)

**✅ Complete & Production-Ready**:
- Core compiler (lexer, parser, type checker, code gen)
- **Multi-file projects** with `./` and `../` imports (**NEW**)
- Standard library (JSON, DateTime, Crypto, File I/O, YAML) - 100% tested
- Developer tools (CLI, LSP, test framework, watch, HMR, cache)
- **Smart cache invalidation** with dependency tracking (**NEW**)
- **String concatenation** with `+` operator (**NEW**)
- 638/638 tests passing (100%)
- 5 packages (router, http, forms, store, i18n)
- 102x faster builds with compilation cache

**✅ Phase 11 Complete**:
- Multi-file imports: `use ./math`, `use ../utils`
- Nested imports: main → types, storage → types
- Working example: `examples/todo-app-multi-file/`
- Documentation: `docs/guides/MODULE_SYSTEM.md`
- Dependency tracking in cache

**⚠️ Blocking Issues for v1.0**:
- No reactivity system (signals/effects) ← **Phase 12 Target**
- Only 5 packages (need 50+) ← **Phase 13-15**
- No example apps yet ← **Phase 16-20**

---

## 🎯 Phase 12: Reactive State Management (NEXT)

**Goal**: Add signals/effects for reactive UIs (like Solid.js)
**Timeline**: 2-3 weeks
**Deliverable**: v0.4.0 with reactivity

### Key Features

- `signal<T>(value)` - Create reactive state
- `computed<T>(() => expr)` - Derived state
- `effect(() => { })` - Side effects that re-run
- `batch(() => { })` - Batch multiple updates
- Auto-tracking dependencies

### Example

```jounce
let count = signal(0);
let doubled = computed(() => count.value * 2);

effect(() => {
    console.log("Count: " + count.value.to_string());
    console.log("Doubled: " + doubled.value.to_string());
});

count.value = 5;  // Logs: Count: 5, Doubled: 10
```

### Success Criteria

- ✅ Signals work with primitive types
- ✅ Computed values auto-update
- ✅ Effects re-run when dependencies change
- ✅ Batch updates prevent redundant work
- ✅ Example counter app works
- ✅ 20+ tests for reactivity

---

## 🚀 Quick Commands

```bash
# Build & test
cargo build --release && cargo test

# Compile multi-file project
cd my-app/ && jnc compile main.jnc

# Run tests
jnc test --verbose

# Watch mode
jnc watch src --output dist
```

---

## 📂 Key Files

### Core Compiler
- `src/lib.rs` - Library interface
- `src/main.rs` - CLI (1340 lines)
- `src/lexer.rs`, `parser.rs`, `js_emitter.rs`
- `src/type_checker.rs` - Type checking

### Module System (Phase 11 - Complete)
- `src/module_loader.rs` - Import resolution
- `src/cache/mod.rs` - Dependency tracking
- `docs/guides/MODULE_SYSTEM.md` - User docs

### Standard Library
- `src/stdlib/json.rs` (7 tests)
- `src/stdlib/time.rs` (15 tests)
- `src/stdlib/crypto.rs` (25 tests)
- `src/stdlib/fs.rs` (10 tests)
- `src/stdlib/yaml.rs` (15 tests)

---

## 📊 Test Status

**Total**: 638/638 (100%)
- Core: 564/564 (100%)
- Stdlib: 74/74 (100%)

**Target Phase 12**: 658+ tests (add reactivity tests)

---

## 🎯 Phase 12 TODO List (START HERE)

**Use TodoWrite to track these tasks as you work through them!**

### Week 1: Design & Research (3-4 days)

- [ ] **Task 1: Research Solid.js reactivity** (~4 hours)
  - Study Solid.js signal implementation
  - Read reactivity pattern documentation
  - Understand dependency tracking algorithms
  - Design Jounce reactivity API

- [ ] **Task 2: Design reactivity specification** (~4 hours)
  - Write `docs/design/REACTIVITY_SYSTEM.md`
  - Define signal, computed, effect semantics
  - JavaScript runtime code generation plan
  - Memory management strategy

- [ ] **Task 3: Implement signal runtime** (~8-12 hours)
  - Add `runtime/reactivity.js`
  - Implement Signal class
  - Implement Computed class
  - Implement Effect class
  - Test runtime in isolation

### Week 2: Parser & Codegen (4-5 days)

- [ ] **Task 4: Add reactivity syntax** (~8 hours)
  - Parse `signal()`, `computed()`, `effect()`
  - Update AST nodes
  - Type checking for reactive types
  - Error messages for misuse

- [ ] **Task 5: Generate reactive code** (~8 hours)
  - js_emitter.rs generates Signal/Computed/Effect
  - Property access → `.value`
  - Handle batching
  - Test generated JavaScript

### Week 3: Testing & Examples (4-5 days)

- [ ] **Task 6: Write comprehensive tests** (~8 hours)
  - 20+ tests for signals, computed, effects
  - Test dependency tracking
  - Test batching
  - Test memory cleanup
  - Edge cases (circular dependencies, etc.)

- [ ] **Task 7: Build example apps** (~8 hours)
  - Counter app (simple signal demo)
  - Todo app with reactivity
  - Form validation example
  - Document each example

- [ ] **Task 8: Write documentation** (~4 hours)
  - User guide for reactivity
  - API reference
  - Migration guide from non-reactive code
  - Performance considerations

### Phase 12 Completion Checklist

- [ ] All 8 tasks complete
- [ ] 20+ reactivity tests passing
- [ ] 3 example apps working
- [ ] Documentation written
- [ ] All existing 638 tests still passing
- [ ] Committed and pushed to GitHub
- [ ] ROADMAP.md updated
- [ ] Ready for Phase 13

---

## 📋 Detailed Task Breakdown

---

## 📚 Phase 11 Archive

**Completed October 24, 2025** - Module System & Multi-File Support

**Achievements**:
- ✅ Local file imports with `./` and `../`
- ✅ Nested imports (recursive)
- ✅ Smart cache invalidation
- ✅ String concatenation with `+`
- ✅ Working multi-file todo app
- ✅ Comprehensive documentation

**Tasks Complete**: 8/11 (3 deferred as enhancements)
- Task 1: ✅ Documented module behavior
- Task 2: ✅ Designed export keyword spec
- Task 3: ✅ Tested multi-file scenarios
- Task 4: ⏸️ Export parsing (deferred)
- Task 5: ⏸️ JS exports (deferred)
- Task 6: ✅ Cross-file imports
- Task 7: ✅ Cache invalidation
- Task 8: ⏸️ CLI directories (deferred)
- Task 9: ✅ Multi-file example app
- Task 10: ✅ Module system docs
- Task 11: ✅ Testing & validation

**Details**: See `docs/guides/MODULE_SYSTEM.md`

---

**Last Updated**: October 24, 2025
**Current Focus**: Phase 12 - Reactive State Management
**Next Milestone**: v0.4.0 with reactivity (2-3 weeks)
