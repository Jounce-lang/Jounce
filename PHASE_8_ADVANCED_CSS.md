# Phase 8: Advanced CSS Features

**Status**: 🚀 STARTING
**Goal**: Extend CSS system with modern features and advanced utilities
**Estimated Time**: ~2-3 weeks (15-20 hours)

---

## Overview

Phase 8 builds on the solid foundation of Phase 7.5 to add:
1. **Modern CSS features** - Grid helpers, container queries
2. **Design system tools** - Token system, theming
3. **Developer experience** - Dark mode, advanced utilities

---

## 📋 Sprint Breakdown

### Sprint 1: CSS Grid Helpers & Container Queries (5-6h)
**Goal**: Add comprehensive grid utilities and modern container queries

**Tasks**:
1. **Task 1.1**: Grid Container Utilities (2h)
   - `grid-cols-*` (1-12 columns, auto, etc.)
   - `grid-rows-*` (1-6 rows, auto)
   - `grid-flow-row`, `grid-flow-col`, `grid-flow-dense`
   - `auto-cols-*`, `auto-rows-*`

2. **Task 1.2**: Grid Item Utilities (1.5h)
   - `col-span-*` (1-12, full, auto)
   - `row-span-*` (1-6, full, auto)
   - `col-start-*`, `col-end-*`
   - `row-start-*`, `row-end-*`
   - `grid-area` utilities

3. **Task 1.3**: Gap Utilities (30min)
   - `gap-*` (already done, verify)
   - `gap-x-*`, `gap-y-*`
   - Integration with spacing scale

4. **Task 1.4**: Container Queries (2h)
   - `@container` support in parser/lexer
   - Container query utilities (`@container (min-width: ...)`)
   - Container type utilities (`container-type`, `container-name`)

**Deliverables**:
- 50+ new grid utilities
- Container query support
- Tests for all utilities
- Documentation updates

---

### Sprint 2: Design Tokens & Theme System (4-5h)
**Goal**: JSON/YAML design token support with theme switching

**Tasks**:
1. **Task 2.1**: Design Token Parser (2h)
   - Load tokens from JSON files
   - Load tokens from YAML files (add yaml dependency)
   - Token schema: colors, spacing, typography, shadows, etc.
   - Merge with existing config

2. **Task 2.2**: Theme Switching (1.5h)
   - Multiple theme support in config
   - CSS custom properties generation
   - Theme-aware utilities
   - Runtime theme switching utilities

3. **Task 2.3**: Dark Mode Utilities (1h)
   - `dark:` variant (like `hover:`, `md:`)
   - `@media (prefers-color-scheme: dark)` support
   - Dark mode color utilities
   - Dark mode examples

4. **Task 2.4**: Documentation (30min)
   - Token file examples
   - Theme switching guide
   - Dark mode guide

**Deliverables**:
- Design token loader
- Theme system
- Dark mode variant
- Example token files

---

### Sprint 3: Advanced Utilities & Polish (4-5h)
**Goal**: CSS custom properties, advanced features, optimization

**Tasks**:
1. **Task 3.1**: CSS Custom Properties (2h)
   - Generate CSS variables from config
   - Variable utilities (`var-*`)
   - Custom property support in css! blocks
   - Theme variable generation

2. **Task 3.2**: Advanced Color Utilities (1h)
   - Opacity modifiers (`bg-blue-500/50` = 50% opacity)
   - Arbitrary values (`bg-[#1da1f2]`)
   - Color mix utilities

3. **Task 3.3**: Print Utilities (30min)
   - `print:` variant
   - `@media print` support
   - Print-specific utilities (`print-hidden`, `print-only`)

4. **Task 3.4**: Accessibility Utilities (1h)
   - `sr-only` (screen reader only)
   - `not-sr-only`
   - Focus utilities (`focus-visible:`, `focus-within:`)
   - High contrast mode support

5. **Task 3.5**: Documentation & Examples (30min)
   - Update guides
   - Create examples
   - Performance benchmarks

**Deliverables**:
- CSS custom properties system
- Advanced utilities
- Print styles
- A11y utilities

---

## 🎯 Success Criteria

**Phase 8 Complete When**:
- ✅ 50+ new grid utilities
- ✅ Container query support
- ✅ Design token system
- ✅ Theme switching works
- ✅ Dark mode variant
- ✅ CSS custom properties
- ✅ Print utilities
- ✅ A11y utilities
- ✅ All tests passing
- ✅ Documentation complete

---

## 📊 Scope Decisions

**In Scope** (Achievable, High Impact):
- ✅ CSS Grid utilities
- ✅ Container queries
- ✅ Design tokens (JSON/YAML)
- ✅ Dark mode
- ✅ CSS custom properties
- ✅ Print styles
- ✅ A11y utilities

**Out of Scope** (Too Complex or Low Priority):
- ❌ Sass/SCSS compilation
- ❌ Figma integration
- ❌ Storybook integration
- ❌ Visual regression testing
- ❌ CSS-in-JS styled() API

These can be future enhancements (Phase 9+) if needed.

---

## 📁 Expected File Changes

**New Files**:
- `src/design_tokens.rs` (design token loader)
- `src/theme_generator.rs` (theme system)
- `benches/grid_utilities.rs` (performance tests)
- `examples/design-tokens.json` (example tokens)
- `examples/themes/` (theme examples)
- `test_grid_utilities.jnc`
- `test_container_queries.jnc`
- `test_dark_mode.jnc`

**Modified Files**:
- `src/utility_generator.rs` (grid utilities, dark mode variant)
- `src/utility_config.rs` (token support)
- `src/lexer.rs` (container query support)
- `src/parser.rs` (container query support)
- `src/ast.rs` (container query AST nodes)
- `src/css_generator.rs` (custom properties, container queries)
- `Cargo.toml` (yaml dependency)
- `docs/guides/UTILITY_CLASSES.md` (updates)
- `CLAUDE.md` (phase 8 status)

---

## 🚀 Getting Started

**First Task**: Sprint 1, Task 1.1 - Grid Container Utilities (~2h)

Ready to begin Phase 8? 🎨
