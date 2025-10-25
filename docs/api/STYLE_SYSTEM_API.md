# Style System API Reference

**Version**: Phase 13 (v0.5.0)
**Last Updated**: October 24, 2025

Complete syntax reference for Jounce's Style System.

---

## Table of Contents

1. [Theme Block Syntax](#theme-block-syntax)
2. [Style Block Syntax](#style-block-syntax)
3. [Nested Selector Syntax](#nested-selector-syntax)
4. [Theme Reference Syntax](#theme-reference-syntax)
5. [CSS Property Support](#css-property-support)
6. [Generated Output](#generated-output)
7. [Limitations](#limitations)

---

## Theme Block Syntax

### Basic Structure

```bnf
<theme-block> ::= "theme" <identifier> "{" <theme-properties> "}"

<theme-properties> ::= (<theme-property> ";")*

<theme-property> ::= <identifier> ":" <css-value>
```

### Example

```jounce
theme ThemeName {
    property1: value1;
    property2: value2;
}
```

### Theme Naming Rules

- **Theme name**: PascalCase identifier
  - Valid: `DarkMode`, `LightTheme`, `BrandColors`
  - Invalid: `dark-mode`, `light_theme`, `123Theme`

- **Property names**: camelCase identifier
  - Valid: `primary`, `textColor`, `bgDark`
  - Invalid: `primary-color`, `text_color`

### Property Values

Any valid CSS value:

```jounce
theme Examples {
    // Colors
    hex: #3b82f6;
    rgb: rgb(59,130,246);
    rgba: rgba(59,130,246,0.5);
    named: blue;

    // Sizes
    pixels: 16px;
    ems: 1.5em;
    rems: 2rem;
    percent: 100%;

    // Keywords
    auto: auto;
    none: none;
    inherit: inherit;

    // Complex values
    shadow: 0 4px 6px rgba(0,0,0,0.1);
    gradient: linear-gradient(to bottom,#fff,#000);
}
```

### Multiple Themes

```jounce
theme Light {
    bg: #ffffff;
}

theme Dark {
    bg: #000000;
}

theme Brand {
    primary: #3b82f6;
}
```

**Output**: All themes in one `:root` block:

```css
:root {
  --Light-bg: #ffffff;
  --Dark-bg: #000000;
  --Brand-primary: #3b82f6;
}
```

---

## Style Block Syntax

### Basic Structure

```bnf
<style-block> ::= "style" <identifier> "{" <style-content> "}"

<style-content> ::= (<style-property> | <nested-selector>)*

<style-property> ::= <property-name> ":" <property-value> ";"

<property-name> ::= <identifier> ("-" <identifier>)*

<property-value> ::= <literal-value> | <theme-reference>
```

### Example

```jounce
style ComponentName {
    property: value;
    hyphenated-property: value;
    another-property: theme.ThemeName.property;

    &:hover {
        property: value;
    }
}
```

### Component Naming Rules

- **Style name**: PascalCase identifier
  - Valid: `Button`, `Card`, `UserProfile`
  - Invalid: `button`, `user-profile`, `123Card`

### Property Names

Standard CSS properties with hyphens:

```jounce
style Example {
    // Single word
    padding: 12px;
    margin: 24px;
    display: flex;

    // Hyphenated
    background-color: #ffffff;
    border-radius: 8px;
    font-size: 16px;
    line-height: 1.5;
    text-align: center;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}
```

**Note**: Some hyphenated properties containing keywords (like `list-style`) may cause parser errors. See [Limitations](#limitations).

### Property Values

#### Literal Values

```jounce
style Example {
    // Colors
    color: #3b82f6;
    background: rgb(255,255,255);
    border-color: rgba(0,0,0,0.1);

    // Sizes
    width: 100px;
    height: 50%;
    font-size: 1.5rem;

    // Multiple values
    padding: 10px 20px;
    margin: 10px 20px 30px 40px;

    // Complex values
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    background: linear-gradient(to bottom,#fff,#000);
    transform: translateY(-2px);
}
```

#### Theme References

```jounce
style Example {
    background: theme.ThemeName.property;
    color: theme.Colors.primary;
}
```

See [Theme Reference Syntax](#theme-reference-syntax) for details.

---

## Nested Selector Syntax

### Basic Structure

```bnf
<nested-selector> ::= "&" <selector-type> "{" <style-properties> "}"

<selector-type> ::= ":" <pseudo-class>
                 | "." <class-name>
```

### Pseudo-Classes

```bnf
<pseudo-class> ::= "hover" | "focus" | "active" | "disabled" | etc.
```

Example:

```jounce
style Button {
    background: blue;

    &:hover {
        background: darkblue;
    }

    &:focus {
        outline: 2px solid blue;
    }

    &:active {
        transform: scale(0.98);
    }

    &:disabled {
        opacity: 0.5;
    }

    &:enabled {
        cursor: pointer;
    }

    &:visited {
        color: purple;
    }

    &:checked {
        background: green;
    }
}
```

**Supported Pseudo-Classes**:
- `:hover`, `:focus`, `:active`
- `:disabled`, `:enabled`, `:checked`
- `:visited`, `:link`
- Any valid CSS pseudo-class

### Class Modifiers

```bnf
<class-name> ::= <identifier>
```

Example:

```jounce
style Button {
    background: gray;

    &.primary {
        background: blue;
    }

    &.secondary {
        background: purple;
    }

    &.danger {
        background: red;
    }

    &.large {
        padding: 16px 32px;
    }

    &.small {
        padding: 8px 16px;
    }
}
```

**Generated CSS**:

```css
.Button_707eab { background: gray; }
.Button_707eab.primary { background: blue; }
.Button_707eab.secondary { background: purple; }
.Button_707eab.danger { background: red; }
.Button_707eab.large { padding: 16px 32px; }
.Button_707eab.small { padding: 8px 16px; }
```

**HTML Usage**:

```html
<button class="Button_707eab">Default</button>
<button class="Button_707eab primary">Primary</button>
<button class="Button_707eab danger large">Large Danger</button>
```

---

## Theme Reference Syntax

### Basic Structure

```bnf
<theme-reference> ::= "theme" "." <theme-name> "." <property-name>

<theme-name> ::= <identifier>

<property-name> ::= <identifier>
```

### Example

```jounce
theme MyTheme {
    primary: #3b82f6;
    text: #1f2937;
}

style Button {
    background: theme.MyTheme.primary;
    color: theme.MyTheme.text;
}
```

### Generated CSS

```css
:root {
  --MyTheme-primary: #3b82f6;
  --MyTheme-text: #1f2937;
}

.Button_707eab {
  background: var(--MyTheme-primary);
  color: var(--MyTheme-text);
}
```

### Rules

1. **Theme must be defined** before it's referenced
2. **Theme name is case-sensitive**: `theme.MyTheme` ≠ `theme.mytheme`
3. **Property must exist** in theme
4. **Can be used anywhere** a CSS value is expected

### In Nested Selectors

```jounce
theme Hover {
    default: blue;
    hover: darkblue;
}

style Button {
    background: theme.Hover.default;

    &:hover {
        background: theme.Hover.hover;
    }

    &.active {
        background: theme.Hover.hover;
    }
}
```

---

## CSS Property Support

### Supported Properties

All standard CSS properties:

```jounce
style Example {
    // Layout
    display: flex;
    position: relative;
    top: 0;
    left: 0;
    width: 100%;
    height: 100vh;

    // Flexbox
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 16px;

    // Spacing
    padding: 12px 24px;
    margin: 0 auto;

    // Typography
    font-family: system-ui,sans-serif;
    font-size: 16px;
    font-weight: 600;
    line-height: 1.5;
    text-align: center;
    text-decoration: none;
    color: #1f2937;

    // Background
    background: #ffffff;
    background-color: #f8fafc;
    background-image: url(/bg.png);

    // Border
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    border-top: 2px solid blue;

    // Effects
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    opacity: 0.9;
    transform: translateY(-2px);
    transition: all 0.2s ease;

    // Misc
    cursor: pointer;
    overflow: hidden;
    z-index: 10;
}
```

### Value Spacing

The compiler handles spacing in complex values:

```jounce
style Example {
    // Multiple values - auto-spaced
    padding: 10px 20px;           // → "10px 20px"
    margin: 5px 10px 15px 20px;   // → "5px 10px 15px 20px"

    // Hex colors - no spaces
    color: #1a1a1a;               // → "#1a1a1a"
    background: #ffffff;          // → "#ffffff"

    // Functions - preserved
    background: linear-gradient(to bottom,#fff,#000);
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    transform: translateY(-2px);
}
```

---

## Generated Output

### File Structure

After compilation:

```
dist/
  ├── server.js
  ├── client.js
  ├── app.wasm
  ├── styles.css      ← Generated CSS
  └── index.html
```

### CSS Output Format

```css
/* Jounce Utility Classes */

/* Themes */
:root {
  --ThemeName-property: value;
  --ThemeName-property2: value2;
}

/* Style blocks */
.ComponentName_hash {
  property: value;
}

.ComponentName_hash:pseudo-class {
  property: value;
}

.ComponentName_hash.modifier {
  property: value;
}
```

### Scoped Class Name Format

```
{ComponentName}_{hash}
```

- **ComponentName**: Your style block name
- **hash**: First 6 characters of SHA-256 hash
- **Example**: `Button_707eab`, `Card_a1b2c3`

### Hash Calculation

```rust
SHA256(component_name)[0..6]
```

**Properties**:
- ✅ **Stable**: Same name → same hash
- ✅ **Unique**: Different names → different hashes
- ✅ **Short**: Only 6 characters
- ✅ **Collision-resistant**: SHA-256 based

---

## Limitations

### Known Issues

#### 1. Properties Containing Keywords

Some CSS properties contain Jounce keywords and may cause parser errors:

❌ **Problematic**:
```jounce
style List {
    list-style: none;  // 'style' is a keyword
}
```

**Workaround**: Remove the property or use external CSS.

#### 2. Nested Pseudo-Class Combinations

Currently not supported:

❌ **Not Supported**:
```jounce
style Button {
    &.primary:hover {  // Combining class + pseudo-class
        background: blue;
    }
}
```

✅ **Workaround**:
```jounce
style Button {
    &.primary {
        background: lightblue;
    }

    &:hover {
        opacity: 0.9;  // Applies to all buttons
    }
}
```

#### 3. Advanced CSS Selectors

Not currently supported:

- `&:nth-child(n)` - Index-based selectors
- `&::before`, `&::after` - Pseudo-elements
- `&:not(.class)` - Negation
- `& > child` - Child combinators

**Workaround**: Use external CSS for advanced selectors.

### Unsupported Features (Future)

- **Media queries**: Use external CSS
- **Animations**: Use external CSS or inline `@keyframes`
- **CSS variables**: Themes compile to CSS variables, but you can't define custom ones
- **CSS imports**: Use build tools

---

## Next Steps

- **User Guide**: Learn how to use the style system
- **Migration Guide**: Add styles to existing apps
- **Examples**: See complete apps in `/examples/styled-*`

---

**Phase 13 API Reference Complete!** 🎉
