# Style System Migration Guide

**Version**: Phase 13 (v0.5.0)
**Last Updated**: October 24, 2025

This guide helps you add styles to existing Jounce applications or migrate from external CSS.

---

## Table of Contents

1. [Migration Overview](#migration-overview)
2. [Step-by-Step Migration](#step-by-step-migration)
3. [From External CSS](#from-external-css)
4. [From Inline Styles](#from-inline-styles)
5. [Adding Themes](#adding-themes)
6. [Common Patterns](#common-patterns)
7. [Troubleshooting Migration](#troubleshooting-migration)

---

## Migration Overview

### What's Changing?

Before Phase 13, you had two options:
1. **External CSS files** - Separate `.css` files
2. **Inline styles** - CSS in HTML templates

After Phase 13, you can:
3. **Style blocks in `.jnc`** - Co-located with your code
4. **Theme blocks** - Centralized color management

### Benefits of Migration

✅ **Co-location**: Styles next to component logic
✅ **Scoped styles**: No global CSS conflicts
✅ **Type safety**: Theme references checked at compile time
✅ **Build-time generation**: Zero runtime overhead
✅ **Theming**: Easy dark mode / light mode

### What's Compatible?

- ✅ All existing `.jnc` code works unchanged
- ✅ External CSS still works
- ✅ Can mix style blocks with external CSS
- ✅ Gradual migration - no "all or nothing"

---

## Step-by-Step Migration

### Step 1: Identify Your Styles

Before:

```
my-app/
  ├── main.jnc          # Application code
  ├── styles.css        # External styles
  └── index.html        # HTML with inline styles
```

**styles.css**:
```css
.button {
    background: #3b82f6;
    color: #ffffff;
    padding: 12px 24px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
}

.button:hover {
    background: #2563eb;
}

.button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
```

### Step 2: Extract Common Colors to Theme

Create a theme block at the top of `main.jnc`:

```jounce
theme AppColors {
    primary: #3b82f6;
    primaryHover: #2563eb;
    text: #ffffff;
}
```

### Step 3: Convert CSS to Style Block

Add style blocks after themes:

```jounce
theme AppColors {
    primary: #3b82f6;
    primaryHover: #2563eb;
    text: #ffffff;
}

style Button {
    background: theme.AppColors.primary;
    color: theme.AppColors.text;
    padding: 12px 24px;
    border-radius: 6px;
    border: none;
    cursor: pointer;

    &:hover {
        background: theme.AppColors.primaryHover;
    }

    &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
}

fn main() {
    println!("Migrated to style blocks!");
}
```

### Step 4: Compile and Check Output

```bash
jnc compile my-app/main.jnc
cat dist/styles.css
```

Output:

```css
:root {
  --AppColors-primary: #3b82f6;
  --AppColors-primaryHover: #2563eb;
  --AppColors-text: #ffffff;
}

.Button_707eab {
  background: var(--AppColors-primary);
  color: var(--AppColors-text);
  padding: 12px 24px;
  border-radius: 6px;
  border: none;
  cursor: pointer;
}

.Button_707eab:hover {
  background: var(--AppColors-primaryHover);
}

.Button_707eab:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
```

### Step 5: Update HTML Class Names

Before:

```html
<button class="button">Click Me</button>
```

After:

```html
<button class="Button_707eab">Click Me</button>
```

**Note**: Find the exact class name in `dist/styles.css`.

### Step 6: Remove External CSS

Once migrated, you can delete `styles.css` and remove the `<link>` from HTML:

```html
<!-- Before -->
<link rel="stylesheet" href="styles.css">

<!-- After: styles are in dist/styles.css from compilation -->
<link rel="stylesheet" href="dist/styles.css">
```

---

## From External CSS

### Example: Migrating a Card Component

**Before (styles.css)**:

```css
.card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 12px;
    padding: 24px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    transition: all 0.3s ease;
}

.card:hover {
    box-shadow: 0 10px 15px rgba(0,0,0,0.15);
    transform: translateY(-4px);
}

.card.highlighted {
    border-color: #3b82f6;
    background: #eff6ff;
}
```

**After (main.jnc)**:

```jounce
theme CardColors {
    bg: #ffffff;
    border: #e2e8f0;
    borderHighlight: #3b82f6;
    bgHighlight: #eff6ff;
    shadow: rgba(0,0,0,0.1);
    shadowHover: rgba(0,0,0,0.15);
}

style Card {
    background: theme.CardColors.bg;
    border: 1px solid theme.CardColors.border;
    border-radius: 12px;
    padding: 24px;
    box-shadow: 0 4px 6px theme.CardColors.shadow;
    transition: all 0.3s ease;

    &:hover {
        box-shadow: 0 10px 15px theme.CardColors.shadowHover;
        transform: translateY(-4px);
    }

    &.highlighted {
        border-color: theme.CardColors.borderHighlight;
        background: theme.CardColors.bgHighlight;
    }
}
```

**HTML Changes**:

```html
<!-- Before -->
<div class="card">Content</div>
<div class="card highlighted">Highlighted</div>

<!-- After -->
<div class="Card_a1b2c3">Content</div>
<div class="Card_a1b2c3 highlighted">Highlighted</div>
```

---

## From Inline Styles

### Example: Migrating Inline HTML Styles

**Before (index.html)**:

```html
<div style="display: flex; flex-direction: column; padding: 48px 24px; min-height: 100vh; background: #f8fafc;">
    <h1 style="font-size: 32px; font-weight: 700; color: #1f2937; margin-bottom: 16px;">
        My App
    </h1>
    <p style="font-size: 16px; color: #64748b; line-height: 1.5;">
        Welcome to my app!
    </p>
</div>
```

**After (main.jnc)**:

```jounce
theme AppTheme {
    bg: #f8fafc;
    textPrimary: #1f2937;
    textSecondary: #64748b;
}

style Container {
    display: flex;
    flex-direction: column;
    padding: 48px 24px;
    min-height: 100vh;
    background: theme.AppTheme.bg;
}

style Title {
    font-size: 32px;
    font-weight: 700;
    color: theme.AppTheme.textPrimary;
    margin-bottom: 16px;
}

style Description {
    font-size: 16px;
    color: theme.AppTheme.textSecondary;
    line-height: 1.5;
}
```

**After (index.html)**:

```html
<div class="Container_x1y2z3">
    <h1 class="Title_a4b5c6">My App</h1>
    <p class="Description_d7e8f9">Welcome to my app!</p>
</div>
```

---

## Adding Themes

### Step 1: Identify Repeated Colors

Look for colors used multiple times in your CSS:

```css
/* Before */
.header { background: #3b82f6; }
.button { background: #3b82f6; }
.link { color: #3b82f6; }

.header:hover { background: #2563eb; }
.button:hover { background: #2563eb; }
.link:hover { color: #2563eb; }
```

### Step 2: Extract to Theme

```jounce
theme Brand {
    primary: #3b82f6;
    primaryHover: #2563eb;
}
```

### Step 3: Replace Hardcoded Colors

```jounce
style Header {
    background: theme.Brand.primary;

    &:hover {
        background: theme.Brand.primaryHover;
    }
}

style Button {
    background: theme.Brand.primary;

    &:hover {
        background: theme.Brand.primaryHover;
    }
}

style Link {
    color: theme.Brand.primary;

    &:hover {
        color: theme.Brand.primaryHover;
    }
}
```

### Step 4: Add Dark Mode Support

Create a second theme:

```jounce
theme LightMode {
    bg: #ffffff;
    text: #1f2937;
    primary: #3b82f6;
}

theme DarkMode {
    bg: #1f2937;
    text: #f9fafb;
    primary: #60a5fa;
}

// Reference LightMode by default
style App {
    background: theme.LightMode.bg;
    color: theme.LightMode.text;
}

style Button {
    background: theme.LightMode.primary;
}
```

### Step 5: Runtime Theme Switching

Add CSS to override theme variables:

```css
/* Add to your HTML or external CSS */
:root.dark-theme {
    --LightMode-bg: var(--DarkMode-bg);
    --LightMode-text: var(--DarkMode-text);
    --LightMode-primary: var(--DarkMode-primary);
}
```

```javascript
// Toggle dark mode
function toggleDarkMode() {
    document.documentElement.classList.toggle('dark-theme');
}
```

---

## Common Patterns

### Pattern 1: Button Variants

**Before (CSS)**:

```css
.btn { background: gray; }
.btn-primary { background: blue; }
.btn-danger { background: red; }
.btn-large { padding: 16px 32px; }
```

**After (Jounce)**:

```jounce
theme ButtonColors {
    default: gray;
    primary: blue;
    danger: red;
}

style Button {
    background: theme.ButtonColors.default;
    padding: 12px 24px;

    &.primary {
        background: theme.ButtonColors.primary;
    }

    &.danger {
        background: theme.ButtonColors.danger;
    }

    &.large {
        padding: 16px 32px;
    }
}
```

### Pattern 2: Responsive Utilities

**Before (CSS)**:

```css
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 24px;
}

.flex { display: flex; }
.flex-col { flex-direction: column; }
.gap-4 { gap: 16px; }
```

**After (Jounce)**:

```jounce
style Container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 24px;
}

style Flex {
    display: flex;
}

style FlexCol {
    display: flex;
    flex-direction: column;
}

style Gap4 {
    gap: 16px;
}
```

**Note**: For utility classes, you might prefer keeping external CSS. Jounce styles work best for **component-specific styles**.

### Pattern 3: Form Elements

**Before (CSS)**:

```css
.input {
    padding: 12px 16px;
    border: 2px solid #cbd5e1;
    border-radius: 8px;
}

.input:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59,130,246,0.1);
}

.input:disabled {
    background: #f1f5f9;
    cursor: not-allowed;
}
```

**After (Jounce)**:

```jounce
theme InputColors {
    border: #cbd5e1;
    borderFocus: #3b82f6;
    bgDisabled: #f1f5f9;
}

style Input {
    padding: 12px 16px;
    border: 2px solid theme.InputColors.border;
    border-radius: 8px;

    &:focus {
        border-color: theme.InputColors.borderFocus;
        box-shadow: 0 0 0 3px rgba(59,130,246,0.1);
    }

    &:disabled {
        background: theme.InputColors.bgDisabled;
        cursor: not-allowed;
    }
}
```

---

## Troubleshooting Migration

### Issue: "Styles not appearing after migration"

**Solution**:
1. Check that `dist/styles.css` is generated: `ls dist/styles.css`
2. Verify HTML links to `dist/styles.css`:
   ```html
   <link rel="stylesheet" href="dist/styles.css">
   ```
3. Find correct class names in generated CSS:
   ```bash
   cat dist/styles.css | grep "^\\."
   ```

### Issue: "Class names changed after recompile"

**Cause**: You renamed the style block.

**Solution**: Keep style block names stable:

```jounce
// Before
style Btn { /* ... */ }  // → .Btn_abc123

// After renaming
style Button { /* ... */ }  // → .Button_def456 (different hash!)
```

If you need to rename, update HTML references too.

### Issue: "Parser error with hyphenated properties"

**Cause**: Property name contains a Jounce keyword.

**Example**:
```jounce
style List {
    list-style: none;  // ERROR: 'style' is keyword
}
```

**Solution**: Remove the property or use external CSS.

### Issue: "Theme reference not working"

**Checklist**:
- ✅ Theme defined before style block?
- ✅ Theme name matches exactly (case-sensitive)?
- ✅ Property exists in theme?

```jounce
// ✓ Correct order
theme MyTheme {
    primary: blue;
}

style Button {
    background: theme.MyTheme.primary;  // ✓ Works
}

// ✗ Wrong order
style Button {
    background: theme.MyTheme.primary;  // ✗ Error: theme not defined
}

theme MyTheme {
    primary: blue;
}
```

### Issue: "Need to combine modifiers"

**Want**: Button that's both `primary` AND `large`

**Before (CSS)**:
```css
.btn.primary.large { /* combined styles */ }
```

**Jounce Limitation**: Can't combine in one selector.

**Workaround**: Use separate classes in HTML:

```html
<button class="Button_707eab primary large">Click Me</button>
```

CSS:
```css
.Button_707eab { /* base */ }
.Button_707eab.primary { /* primary styles */ }
.Button_707eab.large { /* large styles */ }
/* Both classes apply! */
```

---

## Migration Checklist

Use this checklist when migrating a component:

- [ ] Identify all CSS rules for the component
- [ ] Extract repeated colors to a theme block
- [ ] Create style block with component name
- [ ] Convert CSS properties to style block syntax
- [ ] Convert `:hover`, `:focus`, etc. to nested selectors
- [ ] Replace hardcoded colors with theme references
- [ ] Compile and verify CSS output
- [ ] Update HTML with new class names
- [ ] Test component in browser
- [ ] Remove old CSS rules
- [ ] Commit changes

---

## Next Steps

- **User Guide**: Learn more style system features
- **API Reference**: Complete syntax reference
- **Examples**: See complete migrated apps in `/examples/styled-*`

---

**Migration Complete!** 🎉
Your app now uses Jounce's style system with scoped styles and theming.
