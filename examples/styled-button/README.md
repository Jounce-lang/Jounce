# Styled Button Component Example

This example demonstrates the **Jounce Style System** with a complete button component implementation.

## Features Demonstrated

### 1. Theme Block
- Defines color palette as CSS custom properties
- 8 theme colors: primary, hover states, danger, disabled
- Automatically scoped to `:root` for global access

### 2. Style Block
- Scoped CSS class generation (`.Button_<hash>`)
- All CSS properties supported
- Automatic spacing and formatting

### 3. Nested Selectors
- **Pseudo-classes**: `:hover`, `:disabled`
- **Class modifiers**: `.secondary`, `.danger`, `.small`, `.large`
- Clean syntax with `&` parent selector

### 4. Theme References
- Reference theme values with `theme.ButtonTheme.primary`
- Compiles to `var(--ButtonTheme-primary)`
- Enables runtime theme switching

## Running the Example

```bash
# From the project root
cd examples/styled-button
jnc compile main.jnc

# Or with cargo
cargo run -- compile examples/styled-button/main.jnc
```

## Generated Output

### CSS Output (`dist/styles.css`)
```css
:root {
  --ButtonTheme-primary: #3b82f6;
  --ButtonTheme-primaryHover: #2563eb;
  --ButtonTheme-secondary: #8b5cf6;
  /* ... more theme variables */
}

.Button_<hash> {
  background: var(--ButtonTheme-primary);
  color: var(--ButtonTheme-text);
  padding: 12px 24px;
  /* ... more properties */
}

.Button_<hash>:hover {
  background: var(--ButtonTheme-primaryHover);
  transform: translateY(-2px);
  /* ... */
}

.Button_<hash>:disabled {
  background: var(--ButtonTheme-disabled);
  cursor: not-allowed;
  /* ... */
}
```

## Usage in HTML

```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="dist/styles.css">
</head>
<body>
  <!-- Primary button -->
  <button class="Button_<hash>">Click Me</button>

  <!-- Secondary button -->
  <button class="Button_<hash> secondary">Secondary</button>

  <!-- Danger button (large) -->
  <button class="Button_<hash> danger large">Delete</button>

  <!-- Disabled button -->
  <button class="Button_<hash>" disabled>Disabled</button>
</body>
</html>
```

## Key Concepts

### Scoped Class Names
The compiler generates unique class names using SHA-256 hashing:
- Input: `style Button { ... }`
- Output: `.Button_707eab { ... }` (6-char hash)
- Prevents CSS collisions across components

### Theme System
Two-tier architecture for flexible theming:
1. **Global**: CSS custom properties at `:root`
2. **Component**: Reference via `var(--ThemeName-property)`

This enables runtime theme switching without recompilation!

### Build-Time Generation
Unlike runtime CSS-in-JS libraries:
- ✅ Zero runtime overhead
- ✅ Standard CSS output
- ✅ Works with any bundler
- ✅ Full browser compatibility

## Next Steps

- Try changing theme colors and recompiling
- Add new class modifiers (e.g., `&.outline`)
- Explore the generated CSS in `dist/styles.css`
- See **theme-switcher** example for runtime theme switching
- See **styled-todo-app** for a complete application
