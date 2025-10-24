# CLAUDE.md - Jounce Development Guide

**Version**: v0.4.0 "Reactive" (Phase 12 Complete)
**Current Phase**: Phase 13 - Style System & CSS DSL
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

## 📍 Current Status (v0.4.0 "Reactive")

**✅ Complete & Production-Ready**:
- Core compiler (lexer, parser, type checker, code gen)
- Multi-file projects with `./` and `../` imports
- **Fine-grained reactivity system** (signal, computed, effect, batch) (**NEW v0.4.0**)
- Standard library (JSON, DateTime, Crypto, File I/O, YAML) - 100% tested
- Developer tools (CLI, LSP, test framework, watch, HMR, cache)
- Smart cache invalidation with dependency tracking
- String concatenation with `+` operator
- 599/604 tests passing (99.2%)
- 5 packages (router, http, forms, store, i18n)
- 102x faster builds with compilation cache

**✅ Phase 12 Complete (v0.4.0 Released)**:
- Reactivity: signal(), computed(), effect(), batch()
- Automatic dependency tracking
- 29/29 runtime tests (100%)
- 22/22 integration tests (100%)
- 3 example apps with full documentation
- 74KB user documentation (User Guide, API Ref, Migration Guide)

**⚠️ Blocking Issues for v1.0**:
- No style system yet ← **Phase 13 Target**
- Only 5 packages (need 50+) ← **Phase 14-15**
- Need more example apps ← **Phase 15-16**

---

## 🎉 Phase 12: Reactive State Management (COMPLETE)

**Status**: ✅ Complete
**Completed**: October 24, 2025
**Release**: v0.4.0 "Reactive"

### Achievements

✅ **Reactive Primitives Implemented**:
- `signal<T>(value)` - Mutable reactive state
- `computed<T>(() => expr)` - Auto-updating derived values
- `effect(() => {})` - Side effects with dependency tracking
- `batch(() => {})` - Update batching for performance

✅ **Complete Test Coverage**:
- 29/29 runtime tests (100%)
- 22/22 integration tests (100%)
- All edge cases covered

✅ **Documentation & Examples**:
- User Guide (13KB)
- API Reference (11KB)
- Migration Guide (10KB)
- 3 example apps (counter, todo, form validation)
- Release notes

### Success Criteria: All Met

- ✅ Signals work with all types
- ✅ Computed values auto-update
- ✅ Effects re-run on dependency changes
- ✅ Batch updates optimize performance
- ✅ Example apps demonstrate patterns
- ✅ 51+ tests for reactivity

**See**: [User Guide](docs/guides/REACTIVITY_USER_GUIDE.md) | [API Ref](docs/api/REACTIVITY_API.md) | [Examples](examples/)

---

## 🎯 Phase 13: Style System & CSS DSL (NEXT)

**Goal**: Add first-class style blocks for component styling
**Timeline**: 2-3 weeks
**Target**: v0.5.0

See ROADMAP.md for Phase 13 details.

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

**Total**: 599/604 (99.2%)
- Core: 525/530 (99.1%)
- Stdlib: 74/74 (100%)
- Reactivity Runtime: 29/29 (100%)
- Reactivity Integration: 22/22 (100%)

**Target Phase 13**: 620+ tests (add style system tests)

---

## 🎯 Phase 13 TODO List (START HERE)

**Use TodoWrite to track these tasks as you work through them!**

### Week 1: Design & Syntax (3-4 days)

- [ ] **Task 1: Research CSS-in-JS patterns** (~4 hours)
  - Study Styled Components, Emotion, vanilla-extract
  - Review CSS Modules and scoped CSS approaches
  - Research CSS variable best practices
  - Design Jounce style block syntax

- [ ] **Task 2: Design style system specification** (~4 hours)
  - Write `docs/design/STYLE_SYSTEM.md`
  - Define `style {}` block syntax
  - Define theme system (`theme {}` blocks)
  - CSS generation strategy (scoped classes, CSS variables)
  - Hot reload integration plan

- [ ] **Task 3: Add style keyword to lexer** (~2 hours)
  - Add `style` and `theme` keywords
  - Update token types
  - Test lexer with style blocks

### Week 2: Parser & AST (4-5 days)

- [ ] **Task 4: Parse style blocks** (~8 hours)
  - Add StyleBlock AST node
  - Parse CSS properties (property: value)
  - Handle nested selectors (&:hover, &.active)
  - Parse theme blocks
  - Type checking for style values
  - Error messages for invalid CSS

- [ ] **Task 5: CSS code generation** (~8 hours)
  - Generate scoped CSS classes (component_hash)
  - Convert style blocks to CSS strings
  - Handle CSS variable generation for themes
  - Output to dist/styles.css
  - Inject style imports in HTML

### Week 3: Testing & Examples (4-5 days)

- [ ] **Task 6: Write comprehensive tests** (~8 hours)
  - 15+ integration tests for style generation
  - Test scoped class names
  - Test nested selectors
  - Test theme variables
  - Test hot reload with styles
  - Edge cases (duplicate styles, invalid CSS)

- [ ] **Task 7: Build example apps** (~8 hours)
  - Styled button component example
  - Theme switcher app (dark/light mode)
  - Styled todo app (reuse Phase 12 todo with styles)
  - Document each example

- [ ] **Task 8: Write documentation** (~4 hours)
  - User guide for style system
  - API reference for style/theme syntax
  - Migration guide (adding styles to existing apps)
  - Best practices (when to use styles vs. CSS files)

### Phase 13 Completion Checklist

- [ ] All 8 tasks complete
- [ ] 15+ style system tests passing
- [ ] 3 example apps with styled components
- [ ] Documentation written
- [ ] All existing 599 tests still passing
- [ ] Committed and pushed to GitHub
- [ ] ROADMAP.md updated
- [ ] Ready for Phase 14

### Example Syntax (Target)

```jounce
// Define a theme
theme DarkMode {
  primary: #1a1a1a;
  text: #ffffff;
  accent: #3b82f6;
}

// Define component styles
style Button {
  background: theme.primary;
  color: theme.text;
  padding: 10px 20px;
  border-radius: 4px;
  border: none;
  cursor: pointer;

  &:hover {
    background: theme.accent;
  }

  &.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

// Use in component
fn render_button() {
  return html`<button class="${Button}">Click Me</button>`;
}
```

---

## 📋 Detailed Task Breakdown

---

## 📚 Phase 12 Archive

**Completed October 24, 2025** - Fine-Grained Reactivity System

**Release**: v0.4.0 "Reactive"

**Achievements**:
- ✅ Complete reactivity system (signal, computed, effect, batch)
- ✅ Automatic dependency tracking
- ✅ Lambda expression code generation
- ✅ 29/29 runtime tests (100%)
- ✅ 22/22 integration tests (100%)
- ✅ 3 example apps (counter, todo, form validation)
- ✅ 74KB comprehensive documentation

**Tasks Complete**: 8/8 (100%)
- Task 1: ✅ Research Solid.js reactivity patterns
- Task 2: ✅ Design reactivity specification (docs/design/REACTIVITY_SYSTEM.md)
- Task 3: ✅ Implement signal runtime (runtime/reactivity.js)
- Task 4: ✅ Add reactivity syntax to parser
- Task 5: ✅ Generate reactive code (js_emitter.rs)
- Task 6: ✅ Write comprehensive tests (22 integration, 29 runtime)
- Task 7: ✅ Build example apps (3 complete apps)
- Task 8: ✅ Write documentation (User Guide, API Ref, Migration Guide)

**Documentation**:
- [User Guide](docs/guides/REACTIVITY_USER_GUIDE.md) - 13KB, 50 pages
- [API Reference](docs/api/REACTIVITY_API.md) - 11KB
- [Migration Guide](docs/guides/REACTIVITY_MIGRATION.md) - 10KB
- [Release Notes](RELEASE_NOTES.md)

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
**Current Focus**: Phase 13 - Style System & CSS DSL
**Latest Release**: v0.4.0 "Reactive" (Phase 12 Complete)
**Next Milestone**: v0.5.0 with style system (2-3 weeks)
