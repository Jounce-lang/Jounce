# CSS Syntax Guide - Jounce

**Version**: 0.1.0 (Phase 7.5 Sprint 1)
**Status**: ✅ FINALIZED
**Last Updated**: 2025-10-23

---

## Overview

Jounce provides native CSS integration through the `css!` macro, enabling component-scoped styles with zero runtime overhead. CSS is compiled at build time, scoped automatically, and output as separate `.css` files.

**Key Features**:
- **Scoped by default** - No global namespace pollution (CSS Modules approach)
- **Zero runtime** - All CSS generated at compile time
- **Type-safe** - Style objects are checked at compile time
- **Standard CSS** - Use familiar CSS syntax
- **Advanced features** - Nesting, pseudo-classes, media queries, animations

---

## Basic Syntax

### The css! Macro

The primary way to write CSS in Jounce is the `css!` macro:

```raven
let styles = css! {
    .button {
        background: blue;
        padding: 12px 24px;
        border-radius: 4px;
        color: white;
    }
};
```

### Using Styles in JSX

Styles are accessed as properties on the style object:

```raven
<button class={styles.button}>
    Click Me
</button>
```

### Complete Example

```raven
@client
fn Button(props: ButtonProps) -> JSX {
    let styles = css! {
        .button {
            background: #4f46e5;
            color: white;
            padding: 12px 24px;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 16px;
        }

        .button:hover {
            background: #4338ca;
        }

        .button:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
    };

    <button class={styles.button} disabled={props.disabled}>
        {props.children}
    </button>
}
```

---

## CSS Scoping

### Automatic Scoping

All CSS classes are automatically scoped to prevent naming conflicts:

**Input** (Button component):
```raven
let styles = css! {
    .button {
        background: blue;
    }
};
```

**Output** (generated CSS):
```css
.Button_button_a1b2c3 {
    background: blue;
}
```

**Scoping Format**: `{ComponentName}_{className}_{hash}`

- **ComponentName**: The name of the component/function
- **className**: The original class name
- **hash**: 6-character hash for uniqueness

### Why Scoping?

```raven
// Two components can use .button without conflicts!

// components/PrimaryButton.jnc
let styles = css! {
    .button { background: blue; }
};

// components/SecondaryButton.jnc
let styles = css! {
    .button { background: gray; }
};

// Output:
// .PrimaryButton_button_abc123 { background: blue; }
// .SecondaryButton_button_def456 { background: gray; }
```

---

## CSS Selectors

### Class Selectors (Primary)

```raven
let styles = css! {
    .button { /* styles */ }
    .icon { /* styles */ }
    .label { /* styles */ }
};

<button class={styles.button}>
    <span class={styles.icon}>→</span>
    <span class={styles.label}>Click</span>
</button>
```

### Pseudo-classes

```raven
let styles = css! {
    .button {
        background: blue;
    }

    .button:hover {
        background: darkblue;
    }

    .button:focus {
        outline: 2px solid blue;
    }

    .button:active {
        transform: scale(0.98);
    }

    .button:disabled {
        opacity: 0.5;
    }
};
```

### Pseudo-elements

```raven
let styles = css! {
    .button::before {
        content: "→";
        margin-right: 8px;
    }

    .button::after {
        content: "←";
        margin-left: 8px;
    }
};
```

### Compound Selectors

```raven
let styles = css! {
    .button.primary {
        background: blue;
    }

    .button.secondary {
        background: gray;
    }

    .button:hover:not(:disabled) {
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
};
```

---

## CSS Properties and Values

### Color Values

```raven
let styles = css! {
    .element {
        color: red;                           // Named colors
        background: #4f46e5;                  // Hex colors
        border-color: rgb(79, 70, 229);      // RGB
        box-shadow: 0 0 10px rgba(0,0,0,0.1); // RGBA
    }
};
```

### Length Values

```raven
let styles = css! {
    .element {
        width: 100px;          // Pixels
        height: 50%;           // Percentage
        padding: 1rem;         // Rem units
        margin: 2em;           // Em units
        font-size: 1.5rem;     // Rem
        line-height: 1.5;      // Unitless
    }
};
```

### Keyword Values

```raven
let styles = css! {
    .element {
        display: flex;
        position: absolute;
        overflow: hidden;
        cursor: pointer;
        font-weight: bold;
    }
};
```

### Function Values

```raven
let styles = css! {
    .element {
        transform: rotate(45deg);
        background: linear-gradient(to right, blue, green);
        clip-path: polygon(0 0, 100% 0, 100% 100%);
        filter: blur(5px);
    }
};
```

---

## Layout Properties

### Flexbox

```raven
let styles = css! {
    .container {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        gap: 16px;
    }

    .item {
        flex: 1;
        flex-shrink: 0;
    }
};
```

### Grid

```raven
let styles = css! {
    .grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-gap: 20px;
    }

    .cell {
        grid-column: span 2;
    }
};
```

### Positioning

```raven
let styles = css! {
    .relative {
        position: relative;
        top: 10px;
        left: 20px;
    }

    .absolute {
        position: absolute;
        top: 0;
        right: 0;
    }

    .fixed {
        position: fixed;
        bottom: 20px;
        right: 20px;
    }

    .sticky {
        position: sticky;
        top: 0;
    }
};
```

---

## Typography

```raven
let styles = css! {
    .text {
        font-family: "Inter", sans-serif;
        font-size: 16px;
        font-weight: 500;
        line-height: 1.5;
        letter-spacing: 0.5px;
        text-align: center;
        text-decoration: underline;
        text-transform: uppercase;
        color: #111827;
    }
};
```

---

## Spacing

```raven
let styles = css! {
    .box {
        /* Margin */
        margin: 20px;                    // All sides
        margin: 10px 20px;               // Vertical | Horizontal
        margin: 10px 20px 30px 40px;     // Top | Right | Bottom | Left
        margin-top: 10px;
        margin-left: auto;               // Center horizontally

        /* Padding */
        padding: 20px;
        padding: 10px 20px;
        padding: 10px 20px 30px 40px;
        padding-bottom: 10px;
    }
};
```

---

## Borders

```raven
let styles = css! {
    .bordered {
        border: 1px solid #e5e7eb;
        border-top: 2px solid blue;
        border-radius: 8px;
        border-top-left-radius: 4px;
        border-bottom-right-radius: 12px;
    }
};
```

---

## Backgrounds

```raven
let styles = css! {
    .background {
        background-color: #f3f4f6;
        background-image: url("/image.png");
        background-size: cover;
        background-position: center;
        background-repeat: no-repeat;
        background-attachment: fixed;

        /* Shorthand */
        background: #f3f4f6 url("/image.png") center/cover no-repeat;

        /* Gradients */
        background: linear-gradient(to right, #4f46e5, #7c3aed);
        background: radial-gradient(circle, #4f46e5, #7c3aed);
    }
};
```

---

## Shadows

```raven
let styles = css! {
    .shadowed {
        /* Box shadows */
        box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        box-shadow: 0 4px 6px rgba(0,0,0,0.1), 0 2px 4px rgba(0,0,0,0.06);

        /* Text shadows */
        text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
    }
};
```

---

## Transitions

```raven
let styles = css! {
    .animated {
        background: blue;
        transform: scale(1);
        transition: all 0.2s ease;
    }

    .animated:hover {
        background: darkblue;
        transform: scale(1.05);
    }
};
```

---

## Decision: css! Macro vs Inline Styles

### Why css! Macro? (RECOMMENDED)

| Feature | css! macro | inline style |
|---------|-----------|--------------|
| Scoping | ✅ Automatic | ⚠️ Complex |
| Reuse | ✅ Yes | ❌ No |
| Pseudo-classes | ✅ Yes (:hover, :focus) | ❌ No |
| Media queries | ✅ Yes | ❌ No |
| Animations | ✅ Yes (@keyframes) | ❌ No |
| File size | ✅ Smaller (cached CSS) | ⚠️ Larger (inline) |
| Performance | ✅ Better (separate CSS) | ⚠️ Worse (inline) |

### When to Use Inline Styles?

Inline styles will be supported in Sprint 2 for truly dynamic values:

```raven
// Coming in Sprint 2!
<div style={css! {
    background: {props.color};  // Dynamic value from props
}}>
```

---

## Best Practices

### 1. Keep Styles Close to Components

```raven
@client
fn Button(props: ButtonProps) -> JSX {
    // Define styles inside the component
    let styles = css! {
        .button { /* ... */ }
    };

    <button class={styles.button}>{props.children}</button>
}
```

### 2. Use Descriptive Class Names

```raven
// Good
let styles = css! {
    .primary-button { /* ... */ }
    .error-message { /* ... */ }
    .user-avatar { /* ... */ }
};

// Avoid
let styles = css! {
    .btn { /* ... */ }
    .err { /* ... */ }
    .img { /* ... */ }
};
```

### 3. Group Related Styles

```raven
let styles = css! {
    /* Base button */
    .button {
        padding: 12px 24px;
        border: none;
        cursor: pointer;
    }

    /* States */
    .button:hover { /* ... */ }
    .button:focus { /* ... */ }
    .button:disabled { /* ... */ }

    /* Variants */
    .button.primary { /* ... */ }
    .button.secondary { /* ... */ }
};
```

### 4. Prefer Classes Over Element Selectors

```raven
// Good - Explicit and scoped
let styles = css! {
    .title { font-size: 24px; }
    .paragraph { color: #666; }
};

// Avoid - Too generic (not supported yet)
let styles = css! {
    h1 { font-size: 24px; }
    p { color: #666; }
};
```

---

## Next Steps

**Sprint 1** (Current): Basic css! macro ✅
**Sprint 2** (Coming): Nesting, dynamic values, media queries, animations
**Sprint 3** (Coming): Utility classes, CSS modules, theming

---

## Related Guides

- **CSS_ADVANCED.md** - Nesting, media queries, animations (Sprint 2)
- **CSS_UTILITIES.md** - Tailwind-like utility classes (Sprint 3)
- **CSS_THEMING.md** - Light/dark mode and theme system (Sprint 3)

---

**Ready to style your components!** 🎨
