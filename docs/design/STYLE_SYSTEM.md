# Jounce Style System Design Specification

**Version**: 1.0
**Phase**: 13
**Target Release**: v0.5.0
**Author**: Claude (AI Assistant)
**Date**: October 24, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Goals & Principles](#goals--principles)
3. [Core Concepts](#core-concepts)
4. [Syntax Design](#syntax-design)
5. [Compilation Strategy](#compilation-strategy)
6. [Theme System](#theme-system)
7. [Integration with Compiler](#integration-with-compiler)
8. [Hot Reload Strategy](#hot-reload-strategy)
9. [Examples](#examples)
10. [Implementation Plan](#implementation-plan)

---

## Overview

The Jounce Style System provides **first-class component styling** with build-time CSS generation. It combines the simplicity of CSS-in-JS with the performance of build-time compilation.

### Key Features

- **Build-time CSS generation** - Zero runtime style injection
- **Scoped styles** - Automatic class name hashing prevents collisions
- **Themeable** - Global theme variables with runtime switching
- **Type-safe** - Theme references validated at compile time
- **Nested selectors** - Support for pseudo-classes and combinators
- **Hot reload** - Instant style updates during development

### Design Philosophy

Based on research of modern CSS-in-JS patterns:

1. **Performance First** - Like vanilla-extract, generate CSS at build time
2. **Developer Experience** - Like Styled Components, keep syntax simple and familiar
3. **Scalability** - Like CSS Modules, use scoped class names
4. **Flexibility** - Like Emotion, support dynamic theming

---

## Goals & Principles

### Primary Goals

1. **Zero Runtime Cost** - All CSS generated at build time
2. **Scoped by Default** - No global style pollution
3. **Theme Support** - Easy light/dark mode and brand theming
4. **Familiar Syntax** - CSS-like properties, minimal new concepts
5. **Type Safety** - Catch style errors at compile time

### Non-Goals (v0.5.0)

- CSS-in-JS runtime (use build-time only)
- CSS preprocessor features (mixins, functions)
- Critical CSS extraction (defer to later phase)
- CSS modules interop (Jounce styles only)

---

## Core Concepts

### 1. Style Blocks

A `style` block defines **component-scoped CSS**:

```jounce
style Button {
  background: #3b82f6;
  color: white;
  padding: 10px 20px;
  border-radius: 4px;
}
```

Compiles to scoped CSS:

```css
.Button_a1b2c3 {
  background: #3b82f6;
  color: white;
  padding: 10px 20px;
  border-radius: 4px;
}
```

### 2. Theme Blocks

A `theme` block defines **global design tokens**:

```jounce
theme DarkMode {
  primary: #1a1a1a;
  text: #ffffff;
  accent: #3b82f6;
}
```

Compiles to CSS variables:

```css
:root {
  --DarkMode-primary: #1a1a1a;
  --DarkMode-text: #ffffff;
  --DarkMode-accent: #3b82f6;
}
```

### 3. Theme References

Reference theme values in style blocks:

```jounce
style Card {
  background: theme.DarkMode.primary;
  color: theme.DarkMode.text;
}
```

Compiles to:

```css
.Card_d4e5f6 {
  background: var(--DarkMode-primary);
  color: var(--DarkMode-text);
}
```

### 4. Nested Selectors

Support pseudo-classes and state variants:

```jounce
style Button {
  background: #3b82f6;

  &:hover {
    background: #2563eb;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &.primary {
    background: #ef4444;
  }
}
```

Compiles to:

```css
.Button_a1b2c3 {
  background: #3b82f6;
}

.Button_a1b2c3:hover {
  background: #2563eb;
}

.Button_a1b2c3:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.Button_a1b2c3.primary {
  background: #ef4444;
}
```

### 5. Scoped Class Names

Every style block gets a **unique hash** appended:

- Format: `{StyleName}_{hash}`
- Hash: First 6 chars of SHA-256 of file path + style name
- Prevents collisions across files
- Stable (same file = same hash)

---

## Syntax Design

### Style Block Syntax

```ebnf
StyleBlock ::= "style" Identifier "{" StyleProperty* "}"

StyleProperty ::= PropertyName ":" PropertyValue ";"
                | NestedSelector "{" StyleProperty* "}"

PropertyName ::= Identifier ("-" Identifier)*

PropertyValue ::= LiteralValue | ThemeReference

ThemeReference ::= "theme" "." Identifier ("." Identifier)?

NestedSelector ::= "&" PseudoClass | "&" "." ClassName

PseudoClass ::= ":hover" | ":focus" | ":active" | ":disabled" | ...

ClassName ::= Identifier
```

### Theme Block Syntax

```ebnf
ThemeBlock ::= "theme" Identifier "{" ThemeProperty* "}"

ThemeProperty ::= Identifier ":" LiteralValue ";"

LiteralValue ::= ColorValue | SizeValue | StringValue | NumberValue

ColorValue ::= "#" HexDigits | "rgb(...)" | "hsl(...)"

SizeValue ::= Number ("px" | "rem" | "em" | "%")
```

### Supported CSS Properties

**Phase 13 (v0.5.0)** - Essential properties:

- Layout: `display`, `position`, `top`, `left`, `right`, `bottom`, `width`, `height`, `margin`, `padding`
- Typography: `font-family`, `font-size`, `font-weight`, `line-height`, `color`, `text-align`
- Visual: `background`, `background-color`, `border`, `border-radius`, `box-shadow`
- Interaction: `cursor`, `opacity`, `transform`, `transition`

**Future** - Advanced properties:
- Flexbox/Grid (Phase 14)
- Animations (Phase 15)
- Filters (Phase 16)

---

## Compilation Strategy

### 1. Parse Phase

1. **Lexer**: Recognize `style` and `theme` keywords
2. **Parser**: Build StyleBlock and ThemeBlock AST nodes
3. **Type Checker**: Validate theme references exist

### 2. Hash Generation

Generate stable, unique class names:

```rust
fn generate_class_name(file_path: &str, style_name: &str) -> String {
    let input = format!("{}:{}", file_path, style_name);
    let hash = sha256(&input)[..6];  // First 6 chars
    format!("{}_{}", style_name, hash)
}
```

Examples:
- `src/Button.jnc` + `Button` → `Button_a1b2c3`
- `src/Card.jnc` + `Card` → `Card_d4e5f6`
- `src/components/Modal.jnc` + `Modal` → `Modal_e7f8g9`

### 3. CSS Generation

Build CSS string from AST:

```rust
struct CssGenerator {
    output: String,
    current_selector: Option<String>,
}

impl CssGenerator {
    fn visit_style_block(&mut self, style: &StyleBlock) {
        let class_name = generate_class_name(&self.file_path, &style.name);

        // Top-level properties
        self.output.push_str(&format!(".{} {{\n", class_name));
        for prop in &style.properties {
            self.output.push_str(&format!("  {}: {};\n", prop.name, prop.value));
        }
        self.output.push_str("}\n\n");

        // Nested selectors
        for nested in &style.nested {
            let selector = match &nested.selector {
                Selector::Pseudo(pseudo) => format!(".{}:{}", class_name, pseudo),
                Selector::Class(cls) => format!(".{}.{}", class_name, cls),
            };

            self.output.push_str(&format!("{} {{\n", selector));
            for prop in &nested.properties {
                self.output.push_str(&format!("  {}: {};\n", prop.name, prop.value));
            }
            self.output.push_str("}\n\n");
        }
    }

    fn visit_theme_block(&mut self, theme: &ThemeBlock) {
        self.output.push_str(":root {\n");
        for prop in &theme.properties {
            let var_name = format!("--{}-{}", theme.name, prop.name);
            self.output.push_str(&format!("  {}: {};\n", var_name, prop.value));
        }
        self.output.push_str("}\n\n");
    }
}
```

### 4. File Output

Write to `dist/styles.css`:

```
dist/
  styles.css      ← All compiled CSS
  client.js       ← Compiled JavaScript
  index.html      ← HTML with <link rel="stylesheet" href="styles.css">
```

### 5. HTML Integration

Inject CSS link in generated HTML:

```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="styles.css">
</head>
<body>
  <div id="app"></div>
  <script type="module" src="client.js"></script>
</body>
</html>
```

---

## Theme System

### Two-Tier Architecture

**Global Themes** (`:root` scope):
```jounce
theme DarkMode {
  primary: #1a1a1a;
  text: #ffffff;
  accent: #3b82f6;
}

theme LightMode {
  primary: #ffffff;
  text: #000000;
  accent: #3b82f6;
}
```

**Component-Local Variables** (component scope):
```jounce
style Button {
  --button-padding: 10px 20px;

  background: theme.DarkMode.primary;
  color: theme.DarkMode.text;
  padding: var(--button-padding);
}
```

### Theme Switching

**Build-time** (Phase 13):
- Each theme compiles to separate CSS variables
- Developer chooses which theme to apply

**Runtime** (Future):
- JavaScript toggles CSS variable values
- Instant theme switching without recompilation

Example (future):
```jounce
fn toggle_theme() {
  if (current_theme == "dark") {
    set_theme("light");
  } else {
    set_theme("dark");
  }
}
```

### Default Values

Support fallback values for missing theme properties:

```jounce
style Button {
  // Use theme if available, fallback to #3b82f6
  background: theme.DarkMode.primary || #3b82f6;
}
```

---

## Integration with Compiler

### AST Changes

Add new node types to `src/parser.rs`:

```rust
#[derive(Debug, Clone)]
pub enum Statement {
    // ... existing variants
    StyleBlock(StyleBlock),
    ThemeBlock(ThemeBlock),
}

#[derive(Debug, Clone)]
pub struct StyleBlock {
    pub name: String,
    pub properties: Vec<StyleProperty>,
    pub nested: Vec<NestedSelector>,
}

#[derive(Debug, Clone)]
pub struct StyleProperty {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Debug, Clone)]
pub enum PropertyValue {
    Literal(String),
    ThemeRef { theme: String, property: String },
}

#[derive(Debug, Clone)]
pub struct NestedSelector {
    pub selector: Selector,
    pub properties: Vec<StyleProperty>,
}

#[derive(Debug, Clone)]
pub enum Selector {
    Pseudo(String),  // :hover, :focus, etc.
    Class(String),   // .active, .disabled, etc.
}

#[derive(Debug, Clone)]
pub struct ThemeBlock {
    pub name: String,
    pub properties: Vec<ThemeProperty>,
}

#[derive(Debug, Clone)]
pub struct ThemeProperty {
    pub name: String,
    pub value: String,
}
```

### Compiler Pipeline

```
Source Code (.jnc)
      ↓
  [Lexer] - Tokenize style/theme keywords
      ↓
  [Parser] - Build StyleBlock/ThemeBlock AST
      ↓
  [Type Checker] - Validate theme references
      ↓
  [CSS Generator] - Generate styles.css
      ↓
  [JS Emitter] - Generate client.js (unchanged)
      ↓
  [HTML Generator] - Inject <link> to styles.css
      ↓
  dist/ (styles.css, client.js, index.html)
```

### Module System Integration

**Multi-file projects**:
- Each `.jnc` file can have `style` and `theme` blocks
- Compiler collects all blocks across files
- Generates single `dist/styles.css` with all styles

**Import behavior**:
```jounce
// src/Button.jnc
style Button {
  background: #3b82f6;
}

export fn render_button() {
  return html`<button class="${Button}">Click</button>`;
}
```

```jounce
// src/main.jnc
import { render_button } from "./Button.jnc";

fn main() {
  render_button();  // Uses scoped Button class
}
```

### Cache Integration

Smart cache invalidation:
- Style block changes → Recompile CSS only
- JS code changes → Recompile JS only
- Theme changes → Recompile CSS + invalidate dependent styles

---

## Hot Reload Strategy

### CSS Hot Reload

When a style block changes:

1. **Detect change**: Watch `style {}` blocks
2. **Recompile CSS**: Generate new `styles.css`
3. **Inject update**: Replace `<link>` in browser
4. **Preserve state**: Don't reload JS (keep React state)

### Implementation

```javascript
// runtime/hmr.js
export function reloadCSS(newCssUrl) {
  const link = document.querySelector('link[rel="stylesheet"]');
  if (link) {
    link.href = newCssUrl + '?t=' + Date.now();  // Cache bust
  }
}

// Called by watch server when styles.css changes
window.__jounce_reload_css = reloadCSS;
```

### Watch Integration

```rust
// src/watch.rs
fn handle_file_change(path: &Path) {
    if has_style_changes(path) {
        // Recompile CSS
        compile_styles(&project);
        notify_browser_css_reload();
    }

    if has_code_changes(path) {
        // Recompile JS
        compile_js(&project);
        notify_browser_js_reload();
    }
}
```

---

## Examples

### Example 1: Basic Button

```jounce
style Button {
  background: #3b82f6;
  color: white;
  padding: 10px 20px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;

  &:hover {
    background: #2563eb;
  }

  &:active {
    background: #1d4ed8;
  }

  &:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }
}

fn render_button(label: str) -> str {
  return html`<button class="${Button}">${label}</button>`;
}
```

**Compiles to:**

```css
.Button_a1b2c3 {
  background: #3b82f6;
  color: white;
  padding: 10px 20px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
}

.Button_a1b2c3:hover {
  background: #2563eb;
}

.Button_a1b2c3:active {
  background: #1d4ed8;
}

.Button_a1b2c3:disabled {
  background: #9ca3af;
  cursor: not-allowed;
}
```

### Example 2: Themed Card

```jounce
theme DarkMode {
  bg: #1a1a1a;
  text: #ffffff;
  border: #333333;
}

theme LightMode {
  bg: #ffffff;
  text: #000000;
  border: #e5e5e5;
}

style Card {
  background: theme.DarkMode.bg;
  color: theme.DarkMode.text;
  border: 1px solid theme.DarkMode.border;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

fn render_card(title: str, content: str) -> str {
  return html`
    <div class="${Card}">
      <h2>${title}</h2>
      <p>${content}</p>
    </div>
  `;
}
```

**Compiles to:**

```css
:root {
  --DarkMode-bg: #1a1a1a;
  --DarkMode-text: #ffffff;
  --DarkMode-border: #333333;
  --LightMode-bg: #ffffff;
  --LightMode-text: #000000;
  --LightMode-border: #e5e5e5;
}

.Card_d4e5f6 {
  background: var(--DarkMode-bg);
  color: var(--DarkMode-text);
  border: 1px solid var(--DarkMode-border);
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}
```

### Example 3: Complex Layout

```jounce
style Container {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

style Grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;

  &.two-cols {
    grid-template-columns: repeat(2, 1fr);
  }

  &.single-col {
    grid-template-columns: 1fr;
  }
}

style GridItem {
  background: #f3f4f6;
  padding: 15px;
  border-radius: 4px;

  &:hover {
    background: #e5e7eb;
  }
}

fn render_grid(items: array<str>) -> str {
  let grid_items = items.map(item => html`
    <div class="${GridItem}">${item}</div>
  `).join("");

  return html`
    <div class="${Container}">
      <div class="${Grid}">
        ${grid_items}
      </div>
    </div>
  `;
}
```

### Example 4: Form with States

```jounce
theme FormTheme {
  input-bg: #ffffff;
  input-border: #d1d5db;
  input-focus: #3b82f6;
  error: #ef4444;
  success: #10b981;
}

style Input {
  background: theme.FormTheme.input-bg;
  border: 2px solid theme.FormTheme.input-border;
  border-radius: 4px;
  padding: 10px 12px;
  font-size: 14px;
  width: 100%;

  &:focus {
    border-color: theme.FormTheme.input-focus;
    outline: none;
  }

  &.error {
    border-color: theme.FormTheme.error;
  }

  &.success {
    border-color: theme.FormTheme.success;
  }

  &:disabled {
    background: #f3f4f6;
    cursor: not-allowed;
  }
}

style Label {
  display: block;
  font-weight: 600;
  margin-bottom: 8px;
  color: #374151;
}

style ErrorMessage {
  color: theme.FormTheme.error;
  font-size: 12px;
  margin-top: 4px;
}
```

---

## Implementation Plan

### Phase 13 Tasks (2-3 weeks)

#### Week 1: Design & Syntax (3-4 days)

**Task 1: Research CSS-in-JS patterns** ✅ COMPLETE
- Studied Styled Components, Emotion, vanilla-extract
- Reviewed CSS Modules and scoping strategies
- Researched theme systems and CSS variables

**Task 2: Design style system specification** 🚧 IN PROGRESS
- This document

**Task 3: Add style keyword to lexer** (~2 hours)
- Add `style` and `theme` tokens
- Update token type enum
- Test with sample files

#### Week 2: Parser & Code Gen (4-5 days)

**Task 4: Parse style blocks** (~8 hours)
- Add StyleBlock/ThemeBlock AST nodes
- Parse CSS property syntax
- Handle nested selectors (`&:hover`)
- Type check theme references
- Error messages

**Task 5: CSS code generation** (~8 hours)
- Implement class name hashing
- Generate scoped CSS
- Output to dist/styles.css
- Inject <link> in HTML

#### Week 3: Testing & Examples (4-5 days)

**Task 6: Write comprehensive tests** (~8 hours)
- 15+ integration tests
- Test scoped class names
- Test nested selectors
- Test theme variables
- Test hot reload
- Edge cases

**Task 7: Build example apps** (~8 hours)
- Styled button component
- Theme switcher (dark/light)
- Styled todo app
- Document each example

**Task 8: Write documentation** (~4 hours)
- User guide
- API reference
- Migration guide
- Best practices

### Success Criteria

- ✅ All 8 tasks complete
- ✅ 15+ style system tests passing
- ✅ 3 example apps with styled components
- ✅ Documentation complete
- ✅ All existing tests still passing
- ✅ Hot reload working with styles
- ✅ Ready for v0.5.0 release

---

## Future Enhancements

### Phase 14 (v0.6.0)
- Advanced layout (Flexbox, Grid properties)
- Responsive breakpoints (`@media`)
- CSS animations and transitions

### Phase 15 (v0.7.0)
- Runtime theme switching API
- CSS-in-JS dynamic styles
- Component props → style mapping

### Phase 16 (v0.8.0)
- CSS preprocessor features (variables, mixins)
- Critical CSS extraction
- CSS modules interop

---

## Appendix

### CSS Property Reference

**Layout Properties:**
```
display, position, top, right, bottom, left
width, height, min-width, min-height, max-width, max-height
margin, margin-top, margin-right, margin-bottom, margin-left
padding, padding-top, padding-right, padding-bottom, padding-left
```

**Typography Properties:**
```
font-family, font-size, font-weight, font-style
line-height, letter-spacing, text-align, text-decoration
color
```

**Visual Properties:**
```
background, background-color, background-image
border, border-radius, border-color, border-width
box-shadow, opacity
```

**Interaction Properties:**
```
cursor, transform, transition
```

### Pseudo-Class Support

**Phase 13:**
```
:hover, :focus, :active, :disabled
```

**Future:**
```
:first-child, :last-child, :nth-child()
:checked, :invalid, :valid
:before, :after
```

### Hash Algorithm

Using SHA-256 for stable, collision-resistant hashes:

```rust
use sha2::{Sha256, Digest};

fn hash_class_name(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..6].to_string()
}
```

---

**End of Specification**

**Status**: Draft for Phase 13
**Next Review**: After Task 3 (Lexer) completion
**Target**: v0.5.0 release (2-3 weeks)
