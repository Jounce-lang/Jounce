# jounce-theme

Theme management and dark/light mode toggling for Jounce applications.

## Features

- ✅ **Dark/Light Mode** - Easy toggle between themes
- ✅ **CSS Variables** - Manage CSS custom properties
- ✅ **Theme Presets** - Built-in light, dark, and high-contrast themes
- ✅ **Custom Themes** - Builder pattern for creating themes
- ✅ **Persistence** - Save user preferences to localStorage
- ✅ **System Detection** - Auto-detect OS dark mode preference

## Installation

```bash
jnc pkg add jounce-theme
```

## Quick Start

```jounce
use jounce_theme::{init_theme_system, toggle_mode, is_dark_mode};

fn main() {
    // Initialize theme system with default themes
    init_theme_system();

    // Toggle between light and dark mode
    toggle_mode();

    // Check current mode
    if is_dark_mode() {
        println!("Dark mode is active");
    }
}
```

## Usage

### Initialize Theme System

```jounce
use jounce_theme::{init_theme_system, init_default_themes};

// Full initialization (recommended)
// - Registers default themes
// - Loads saved preference from localStorage
// - Falls back to system preference if no saved preference
init_theme_system();

// Or initialize with just default themes
init_default_themes();
```

### Toggle Dark/Light Mode

```jounce
use jounce_theme::{toggle_mode, set_mode, get_mode, is_dark_mode, ThemeMode};

// Toggle between light and dark
let new_mode = toggle_mode();

// Set specific mode
set_mode(ThemeMode::Dark);
set_mode(ThemeMode::Light);

// Get current mode
let current = get_mode();

// Check if dark mode
if is_dark_mode() {
    println!("Using dark theme");
}
```

### Manage CSS Variables

```jounce
use jounce_theme::{set_css_var, get_css_var, set_css_vars, remove_css_var};

// Set single variable
set_css_var("primary-color", "#3b82f6");

// Get variable value
let color = get_css_var("primary-color");

// Set multiple variables
let vars = Map::from([
    ("primary", "#3b82f6"),
    ("secondary", "#8b5cf6"),
    ("background", "#ffffff"),
]);
set_css_vars(vars);

// Remove variable
remove_css_var("old-variable");
```

### Use Theme Presets

```jounce
use jounce_theme::{light_theme, dark_theme, high_contrast_theme, register_theme};

// Register built-in themes
register_theme(light_theme());
register_theme(dark_theme());
register_theme(high_contrast_theme());

// Light theme colors:
// - background: #ffffff
// - foreground: #000000
// - primary: #3b82f6
// - secondary: #8b5cf6

// Dark theme colors:
// - background: #0f172a
// - foreground: #f1f5f9
// - primary: #3b82f6
// - secondary: #8b5cf6

// High contrast:
// - background: #000000
// - foreground: #ffffff
// - primary: #00ffff
// - border: #ffffff
```

### Create Custom Themes

```jounce
use jounce_theme::{ThemeBuilder, register_theme, set_current_theme};

// Use builder pattern
let custom = ThemeBuilder::new("ocean")
    .color("background", "#003f5c")
    .color("foreground", "#ffffff")
    .color("primary", "#00d4ff")
    .color("secondary", "#7a5fff")
    .color("accent", "#00ffa3")
    .font("sans", "Inter, system-ui, sans-serif")
    .font("mono", "Fira Code, monospace")
    .spacing("sm", "0.5rem")
    .spacing("md", "1rem")
    .spacing("lg", "2rem")
    .breakpoint("md", "768px")
    .breakpoint("lg", "1024px")
    .build();

// Register and activate
register_theme(custom);
set_current_theme("ocean");
```

### Switch Themes

```jounce
use jounce_theme::{set_current_theme, get_current_theme, get_theme};

// Set active theme
set_current_theme("dark");
set_current_theme("light");
set_current_theme("high-contrast");
set_current_theme("ocean"); // Custom theme

// Get current theme
if let Some(theme) = get_current_theme() {
    println!("Active theme: {}", theme.name);
}

// Get specific theme
if let Some(theme) = get_theme("dark") {
    println!("Dark theme has {} colors", theme.colors.len());
}
```

### Persist Preferences

```jounce
use jounce_theme::{load_theme_preference, clear_theme_preference};

// Preferences are automatically saved when theme changes
set_current_theme("dark"); // Saved to localStorage

// Load saved preference (called automatically by init_theme_system)
load_theme_preference();

// Clear saved preference
clear_theme_preference();
```

### System Preference Detection

```jounce
use jounce_theme::{prefers_dark_mode, init_from_system_preference};

// Check OS preference
if prefers_dark_mode() {
    println!("User prefers dark mode");
}

// Initialize from system preference
init_from_system_preference();
```

## Theme Structure

```jounce
struct Theme {
    name: string,
    colors: Map<string, string>,      // Color palette
    fonts: Map<string, string>,       // Font families
    spacing: Map<string, string>,     // Spacing scale
    breakpoints: Map<string, string>, // Responsive breakpoints
}
```

### Default Theme Variables

All themes include these CSS variables:

**Colors:**
- `--color-background` - Main background color
- `--color-foreground` - Main text color
- `--color-primary` - Primary brand color
- `--color-secondary` - Secondary brand color
- `--color-accent` - Accent color
- `--color-muted` - Muted text color
- `--color-border` - Border color
- `--color-error` - Error state color
- `--color-warning` - Warning state color
- `--color-success` - Success state color

**Fonts:**
- `--font-sans` - Sans-serif font stack
- `--font-serif` - Serif font stack
- `--font-mono` - Monospace font stack

**Spacing:**
- `--spacing-xs` - Extra small (0.25rem)
- `--spacing-sm` - Small (0.5rem)
- `--spacing-md` - Medium (1rem)
- `--spacing-lg` - Large (1.5rem)
- `--spacing-xl` - Extra large (2rem)

**Breakpoints:**
- `--breakpoint-sm` - Small screens (640px)
- `--breakpoint-md` - Medium screens (768px)
- `--breakpoint-lg` - Large screens (1024px)
- `--breakpoint-xl` - Extra large screens (1280px)

## Integration with Phase 13 Style System

The theme package works seamlessly with Jounce's built-in style system:

```jounce
// Define theme
theme AppTheme {
    primary: var(--color-primary);
    background: var(--color-background);
    text: var(--color-foreground);
}

// Use in styles
style Button {
    background: theme.AppTheme.primary;
    color: white;
    padding: var(--spacing-md);
}

// Theme variables update automatically when theme changes
```

## Complete Example

```jounce
use jounce_theme::{
    init_theme_system, toggle_mode, is_dark_mode,
    ThemeBuilder, register_theme, set_current_theme
};

fn main() {
    // Initialize with defaults
    init_theme_system();

    // Create custom theme
    let brand_theme = ThemeBuilder::new("brand")
        .color("background", "#f8fafc")
        .color("foreground", "#0f172a")
        .color("primary", "#6366f1")
        .color("secondary", "#ec4899")
        .build();

    register_theme(brand_theme);
    set_current_theme("brand");

    // Add toggle button
    let toggle_btn = create_button("Toggle Dark Mode", || {
        toggle_mode();
        update_ui();
    });

    // Update UI based on theme
    fn update_ui() {
        if is_dark_mode() {
            println!("🌙 Dark mode active");
        } else {
            println!("☀️ Light mode active");
        }
    }
}
```

## API Reference

### Theme Management

- `register_theme(theme: Theme) -> bool` - Register a new theme
- `get_theme(name: string) -> Option<Theme>` - Get theme by name
- `get_current_theme() -> Option<Theme>` - Get active theme
- `set_current_theme(name: string) -> bool` - Set active theme

### Mode Control

- `get_mode() -> ThemeMode` - Get current mode (Light/Dark)
- `set_mode(mode: ThemeMode)` - Set mode
- `toggle_mode() -> ThemeMode` - Toggle between light/dark
- `is_dark_mode() -> bool` - Check if dark mode active

### CSS Variables

- `set_css_var(name: string, value: string)` - Set CSS variable
- `get_css_var(name: string) -> string` - Get CSS variable value
- `remove_css_var(name: string)` - Remove CSS variable
- `set_css_vars(vars: Map<string, string>)` - Set multiple variables

### Presets

- `light_theme() -> Theme` - Default light theme
- `dark_theme() -> Theme` - Default dark theme
- `high_contrast_theme() -> Theme` - High contrast theme

### Initialization

- `init_default_themes()` - Register default themes
- `init_theme_system()` - Full initialization (recommended)
- `init_from_system_preference()` - Use OS preference

### Persistence

- `load_theme_preference()` - Load from localStorage
- `clear_theme_preference()` - Clear saved preference

### System

- `prefers_dark_mode() -> bool` - Check OS dark mode preference

## Best Practices

1. **Always Initialize**: Call `init_theme_system()` at app startup
2. **Use Variables**: Reference CSS variables in your styles
3. **Respect User Choice**: Load saved preferences before system defaults
4. **Provide Toggle**: Give users easy way to switch themes
5. **Test Both Modes**: Ensure UI works in light and dark modes
6. **Semantic Colors**: Use role-based colors (primary, error) not literal names

## Examples

See `examples/` directory:
- `theme-toggle.jnc` - Basic dark/light toggle
- `custom-theme.jnc` - Creating custom themes
- `theme-picker.jnc` - Multi-theme selector

## License

MIT

## Version

0.1.0
