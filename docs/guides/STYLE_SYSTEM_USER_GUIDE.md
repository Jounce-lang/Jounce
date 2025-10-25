# Style System User Guide

**Version**: Phase 13 (v0.5.0)
**Last Updated**: October 24, 2025

Welcome to the Jounce Style System! This guide will teach you how to style your components using Jounce's built-in CSS-in-Jounce syntax.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Quick Start](#quick-start)
3. [Theme Blocks](#theme-blocks)
4. [Style Blocks](#style-blocks)
5. [Nested Selectors](#nested-selectors)
6. [Theme References](#theme-references)
7. [Scoped Class Names](#scoped-class-names)
8. [Complete Examples](#complete-examples)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Introduction

### What is the Style System?

Jounce's Style System is a **build-time CSS generation** feature that lets you define styles directly in your `.jnc` files. It provides:

✅ **Scoped styles** - Unique class names prevent global conflicts
✅ **Theming** - CSS custom properties for runtime theme switching
✅ **Type-safe** - Theme references checked at compile time
✅ **Zero runtime** - All CSS generated at build time
✅ **Standard output** - Works with any CSS tooling

### How It Works

```jounce
// 1. Define a theme
theme DarkMode {
    primary: #3b82f6;
    text: #ffffff;
}

// 2. Define styles
style Button {
    background: theme.DarkMode.primary;
    color: theme.DarkMode.text;
    padding: 12px 24px;

    &:hover {
        opacity: 0.9;
    }
}

// 3. Use in your app
fn main() {
    println!("Styles generated at dist/styles.css!");
}
```

Compiles to:

```css
:root {
  --DarkMode-primary: #3b82f6;
  --DarkMode-text: #ffffff;
}

.Button_707eab {
  background: var(--DarkMode-primary);
  color: var(--DarkMode-text);
  padding: 12px 24px;
}

.Button_707eab:hover {
  opacity: 0.9;
}
```

---

## Quick Start

### 1. Create a Simple Button

```jounce
// my-app/main.jnc

style Button {
    background: #3b82f6;
    color: #ffffff;
    padding: 12px 24px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
}

fn main() {
    println!("Button styled!");
}
```

### 2. Compile

```bash
jnc compile my-app/main.jnc
```

### 3. Use in HTML

```html
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="dist/styles.css">
</head>
<body>
    <button class="Button_707eab">Click Me</button>
</body>
</html>
```

**Note**: The class name `Button_707eab` is generated automatically. Find it in `dist/styles.css`.

---

## Theme Blocks

### Basic Theme

Define a theme with CSS custom properties:

```jounce
theme MyTheme {
    primary: #3b82f6;
    secondary: #8b5cf6;
    text: #1f2937;
    background: #ffffff;
}
```

Generates:

```css
:root {
  --MyTheme-primary: #3b82f6;
  --MyTheme-secondary: #8b5cf6;
  --MyTheme-text: #1f2937;
  --MyTheme-background: #ffffff;
}
```

### Multiple Themes

Define multiple themes for dark/light mode:

```jounce
theme Light {
    bg: #ffffff;
    text: #000000;
}

theme Dark {
    bg: #000000;
    text: #ffffff;
}
```

### Theme Naming

- Use **PascalCase**: `LightMode`, `DarkMode`, `BrandColors`
- Use **camelCase** for properties: `primaryColor`, `bgDark`, `textLight`

---

## Style Blocks

### Basic Style Block

```jounce
style Card {
    background: #ffffff;
    padding: 24px;
    border-radius: 12px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}
```

### Multiple Properties

All standard CSS properties are supported:

```jounce
style Container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    background: #f8fafc;
    padding: 48px 24px;
    font-family: system-ui,sans-serif;
}
```

### Hyphenated Properties

CSS properties with hyphens work as expected:

```jounce
style Text {
    font-size: 16px;
    line-height: 1.5;
    text-align: center;
    text-decoration: none;
    background-color: #ffffff;
    border-radius: 8px;
}
```

---

## Nested Selectors

### Pseudo-Classes

Use `&:pseudo-class` for hover, focus, disabled, etc.:

```jounce
style Button {
    background: blue;
    cursor: pointer;

    &:hover {
        background: darkblue;
    }

    &:focus {
        outline: 2px solid blue;
    }

    &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    &:active {
        transform: scale(0.98);
    }
}
```

**Supported Pseudo-Classes**:
- `:hover`
- `:focus`
- `:active`
- `:disabled`
- `:enabled`
- `:visited`
- `:checked`
- Any valid CSS pseudo-class!

### Class Modifiers

Use `&.className` for variant styles:

```jounce
style Button {
    background: gray;

    &.primary {
        background: blue;
    }

    &.danger {
        background: red;
    }

    &.large {
        padding: 16px 32px;
        font-size: 18px;
    }
}
```

Generates:

```css
.Button_707eab { background: gray; }
.Button_707eab.primary { background: blue; }
.Button_707eab.danger { background: red; }
.Button_707eab.large { padding: 16px 32px; font-size: 18px; }
```

HTML usage:

```html
<button class="Button_707eab">Default</button>
<button class="Button_707eab primary">Primary</button>
<button class="Button_707eab danger large">Large Danger</button>
```

---

## Theme References

### Referencing Theme Values

Use `theme.ThemeName.property` to reference theme values:

```jounce
theme AppColors {
    primary: #3b82f6;
    text: #1f2937;
}

style Button {
    background: theme.AppColors.primary;
    color: theme.AppColors.text;
}
```

Compiles to:

```css
:root {
  --AppColors-primary: #3b82f6;
  --AppColors-text: #1f2937;
}

.Button_707eab {
  background: var(--AppColors-primary);
  color: var(--AppColors-text);
}
```

### Theme References in Nested Selectors

```jounce
theme Hover {
    primary: #3b82f6;
    hover: #2563eb;
}

style Button {
    background: theme.Hover.primary;

    &:hover {
        background: theme.Hover.hover;
    }
}
```

### Why Use Theme References?

1. **Runtime Switching**: Change CSS variables with JavaScript
2. **Consistency**: One source of truth for colors
3. **Type Safety**: Compiler checks theme exists
4. **Maintenance**: Update once, changes everywhere

---

## Scoped Class Names

### How Scoping Works

Every style block generates a **unique class name** using SHA-256 hashing:

```jounce
style Button { /* ... */ }
```

Becomes:

```css
.Button_707eab { /* ... */ }
```

Format: `{ComponentName}_{hash}`

### Hash Stability

- **Same input = Same hash**: Deterministic generation
- **Different names = Different hashes**: No collisions
- **6 characters**: Short but unique enough

### Finding Class Names

Check `dist/styles.css` after compilation:

```bash
jnc compile main.jnc
cat dist/styles.css | grep "^\\."
```

Output:

```
.Button_707eab {
.Card_a1b2c3 {
.Header_d4e5f6 {
```

---

## Complete Examples

### Example 1: Themed Button

```jounce
theme ButtonTheme {
    primary: #3b82f6;
    primaryHover: #2563eb;
    text: #ffffff;
}

style PrimaryButton {
    background: theme.ButtonTheme.primary;
    color: theme.ButtonTheme.text;
    padding: 12px 24px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
    font-size: 16px;
    font-weight: 600;
    transition: all 0.2s ease;

    &:hover {
        background: theme.ButtonTheme.primaryHover;
        transform: translateY(-2px);
    }

    &:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
}

fn main() {
    println!("Themed button ready!");
}
```

### Example 2: Card Component

```jounce
theme CardTheme {
    bg: #ffffff;
    border: #e2e8f0;
    shadow: rgba(0,0,0,0.1);
}

style Card {
    background: theme.CardTheme.bg;
    border: 1px solid theme.CardTheme.border;
    border-radius: 12px;
    padding: 24px;
    box-shadow: 0 4px 6px theme.CardTheme.shadow;
    transition: all 0.3s ease;

    &:hover {
        box-shadow: 0 10px 15px theme.CardTheme.shadow;
        transform: translateY(-4px);
    }
}

fn main() {
    println!("Card component styled!");
}
```

### Example 3: Input with Focus Ring

```jounce
theme InputTheme {
    border: #cbd5e1;
    focusBorder: #3b82f6;
}

style Input {
    padding: 12px 16px;
    border: 2px solid theme.InputTheme.border;
    border-radius: 8px;
    font-size: 16px;
    outline: none;
    transition: all 0.2s ease;

    &:focus {
        border-color: theme.InputTheme.focusBorder;
        box-shadow: 0 0 0 3px rgba(59,130,246,0.1);
    }

    &:disabled {
        background: #f1f5f9;
        cursor: not-allowed;
    }
}

fn main() {
    println!("Input styled!");
}
```

---

## Best Practices

### 1. Use Themes for Colors

❌ **Don't** hardcode colors:

```jounce
style Button {
    background: #3b82f6;  // BAD
}
```

✅ **Do** use themes:

```jounce
theme Colors {
    primary: #3b82f6;
}

style Button {
    background: theme.Colors.primary;  // GOOD
}
```

### 2. Organize by Component

One style block per component:

```jounce
// Button.jnc
style Button { /* ... */ }

// Card.jnc
style Card { /* ... */ }

// Header.jnc
style Header { /* ... */ }
```

### 3. Use Descriptive Names

```jounce
// Good names
style PrimaryButton { /* ... */ }
style UserProfileCard { /* ... */ }
style NavigationHeader { /* ... */ }

// Bad names
style Btn { /* ... */ }
style Thing { /* ... */ }
style X { /* ... */ }
```

### 4. Keep Styles Focused

Each style block should represent **one visual component**:

```jounce
// Good - focused
style Button {
    background: blue;
    padding: 12px;
}

style Card {
    background: white;
    padding: 24px;
}

// Bad - too broad
style App {
    /* 200 lines of CSS for entire app */
}
```

### 5. Use Nested Selectors Sparingly

Only nest when it's a **state** of the component:

```jounce
// Good
style Button {
    background: blue;

    &:hover { background: darkblue; }
    &:disabled { opacity: 0.5; }
}

// Bad - too deep
style Card {
    padding: 24px;

    &.active {
        border: 2px solid blue;

        &:hover {
            &.focused {
                /* Too nested! */
            }
        }
    }
}
```

---

## Troubleshooting

### My styles aren't appearing!

1. **Check the CSS file exists**: `ls dist/styles.css`
2. **Link CSS in HTML**: `<link rel="stylesheet" href="dist/styles.css">`
3. **Use correct class name**: Check `dist/styles.css` for generated name

### Parser errors with property names

Some CSS properties may conflict with keywords. If you get errors:

❌ Problematic:
```jounce
style List {
    list-style: none;  // ERROR: 'style' is a keyword
}
```

✅ Workaround:
- Remove the property, or
- Use a different approach in your HTML/CSS

### Theme reference not working

Make sure:
1. Theme is defined **before** style block
2. Theme name matches exactly (case-sensitive)
3. Property exists in theme

```jounce
// Define theme first
theme MyTheme {
    primary: #3b82f6;
}

// Then reference it
style Button {
    background: theme.MyTheme.primary;  // ✓ Correct
    // background: theme.myTheme.primary;  // ✗ Wrong case
    // background: theme.MyTheme.secondary;  // ✗ Doesn't exist
}
```

### Class names changing between builds

If you change the style block **name**, the hash changes:

```jounce
style Button { /* ... */ }      // → .Button_707eab
style PrimaryButton { /* ... */ }  // → .PrimaryButton_abc123
```

Keep component names stable!

---

## Next Steps

- **API Reference**: Complete syntax reference for all features
- **Migration Guide**: Add styles to existing apps
- **Examples**: See `/examples/styled-*` for complete apps

---

**Phase 13 Complete!** 🎉
You now have a complete style system for building beautiful Jounce apps.
