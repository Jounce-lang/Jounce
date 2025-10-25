# CLAUDE.md - Jounce Development Guide

**Version**: v0.5.0 "Styled" (Phase 13 Complete)
**Current Phase**: Phase 14 - Advanced Type System
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

## 📍 Current Status (v0.5.0 "Styled")

**✅ Complete & Production-Ready**:
- Core compiler (lexer, parser, type checker, code gen)
- Multi-file projects with `./` and `../` imports
- Fine-grained reactivity system (signal, computed, effect, batch)
- **Style system with scoped CSS & themes** (style{}, theme{}) (**NEW v0.5.0**)
- Standard library (JSON, DateTime, Crypto, File I/O, YAML) - 100% tested
- Developer tools (CLI, LSP, test framework, watch, HMR, cache)
- Smart cache invalidation with dependency tracking
- String concatenation with `+` operator
- 620/635 tests passing (97.6%)
- 5 packages (router, http, forms, store, i18n)
- 102x faster builds with compilation cache

**✅ Phase 13 Complete (v0.5.0 Released)**:
- Style blocks with scoped class names (SHA-256 hashing)
- Theme blocks with CSS custom properties
- Nested selectors (:hover, :focus, :disabled, .modifiers)
- Theme references (theme.Name.property → var(--Name-property))
- 20/20 style system tests (100%)
- 3 example apps with styled components
- 34KB comprehensive documentation (User Guide, API Ref, Migration Guide)

**⚠️ Blocking Issues for v1.0**:
- Need advanced type features ← **Phase 14 Target**
- Only 5 packages (need 50+) ← **Phase 15-16**
- Need more example apps ← **Phase 16-17**

---

## 🎉 Phase 13: Style System & CSS DSL (COMPLETE)

**Status**: ✅ Complete
**Completed**: October 24, 2025
**Release**: v0.5.0 "Styled"

### Achievements

✅ **Style System Implemented**:
- `theme {}` blocks - CSS custom properties for theming
- `style {}` blocks - Scoped class names with SHA-256 hashing
- Nested selectors - `&:hover`, `&:focus`, `&.modifier`
- Theme references - `theme.Name.prop` → `var(--Name-prop)`

✅ **Complete Test Coverage**:
- 20/20 style system tests (100%)
- 18 integration tests + 2 lexer tests
- All edge cases covered (themes, styles, nesting, references)

✅ **Documentation & Examples**:
- User Guide (13KB)
- API Reference (11KB)
- Migration Guide (10KB)
- 3 example apps (styled-button, theme-switcher, styled-todo-app)
- Generated CSS: 1129-3547 bytes per app

### Success Criteria: All Met

- ✅ Theme blocks generate CSS custom properties
- ✅ Style blocks generate scoped classes
- ✅ Nested selectors work (:hover, .modifiers)
- ✅ Theme references compile to var()
- ✅ Build-time CSS generation (zero runtime)
- ✅ 20+ tests passing
- ✅ 3 example apps compiling

**See**: [User Guide](docs/guides/STYLE_SYSTEM_USER_GUIDE.md) | [API Ref](docs/api/STYLE_SYSTEM_API.md) | [Examples](examples/styled-*)

---

## 🎯 Phase 14: Advanced Type System (NEXT)

**Goal**: Enhance type system with advanced features
**Timeline**: TBD
**Target**: v0.6.0

See ROADMAP.md for Phase 14 details.

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

### Style System (Phase 13 - Complete)
- `src/ast.rs` - StyleBlock, ThemeBlock AST nodes
- `src/parser.rs` - parse_style_block(), parse_theme_block()
- `src/codegen.rs` - CSS generation with scoping
- `docs/guides/STYLE_SYSTEM_USER_GUIDE.md` - User guide
- `docs/api/STYLE_SYSTEM_API.md` - API reference

### Standard Library
- `src/stdlib/json.rs` (7 tests)
- `src/stdlib/time.rs` (15 tests)
- `src/stdlib/crypto.rs` (25 tests)
- `src/stdlib/fs.rs` (10 tests)
- `src/stdlib/yaml.rs` (15 tests)

---

## 📊 Test Status

**Total**: 620/635 (97.6%)
- Core: 525/530 (99.1%)
- Stdlib: 74/74 (100%)
- Reactivity Runtime: 29/29 (100%)
- Reactivity Integration: 22/22 (100%)
- Style System: 20/20 (100%)

**Target Phase 14**: 640+ tests (add advanced type tests)

---

## 🎯 Phase 14 TODO List (START HERE)

**Use TodoWrite to track these tasks as you work through them!**

Phase 14 details coming soon. Check ROADMAP.md for the latest planning.

### Potential Focus Areas

- Advanced type system features
- Type inference improvements
- Generic constraints
- Union types / sum types
- Pattern matching enhancements

### Phase 14 Completion Checklist (TBD)

- [ ] Design Phase 14 specification
- [ ] All tasks complete
- [ ] Tests passing
- [ ] Documentation written
- [ ] Committed and pushed to GitHub
- [ ] ROADMAP.md updated
- [ ] Ready for Phase 15

---

## 📚 Phase 13 Archive

**Completed October 24, 2025** - Style System & CSS DSL

**Release**: v0.5.0 "Styled"

**Achievements**:
- ✅ Complete style system (theme blocks, style blocks, nested selectors)
- ✅ Build-time CSS generation (zero runtime overhead)
- ✅ Scoped class names with SHA-256 hashing
- ✅ Theme references compile to CSS custom properties
- ✅ 20/20 style system tests (100%)
- ✅ 3 example apps (styled-button, theme-switcher, styled-todo-app)
- ✅ 34KB comprehensive documentation

**Tasks Complete**: 8/8 (100%)
- Task 1: ✅ Research CSS-in-JS patterns
- Task 2: ✅ Design style system specification (docs/design/STYLE_SYSTEM.md)
- Task 3: ✅ Add style and theme keywords to lexer
- Task 4: ✅ Parse style blocks with CSS properties and nested selectors
- Task 5: ✅ Generate scoped CSS classes and output to dist/styles.css
- Task 6: ✅ Write 18 integration tests (all passing)
- Task 7: ✅ Build 3 example apps with styled components
- Task 8: ✅ Write documentation (User Guide, API Ref, Migration Guide)

**Documentation**:
- [User Guide](docs/guides/STYLE_SYSTEM_USER_GUIDE.md) - 13KB
- [API Reference](docs/api/STYLE_SYSTEM_API.md) - 11KB
- [Migration Guide](docs/guides/STYLE_SYSTEM_MIGRATION.md) - 10KB

**Examples**:
- [styled-button](examples/styled-button/) - 1129 bytes CSS
- [theme-switcher](examples/theme-switcher/) - 2006 bytes CSS
- [styled-todo-app](examples/styled-todo-app/) - 3547 bytes CSS

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
**Current Focus**: Phase 14 - Advanced Type System
**Latest Release**: v0.5.0 "Styled" (Phase 13 Complete)
**Next Milestone**: v0.6.0 with advanced types (TBD)
