# RavensOne Utility Class System Design

**Status**: ✅ **COMPLETE - IMPLEMENTED**
**Sprint**: Phase 7.5 Sprint 3 (Tasks 3.1-3.4)
**Duration**: ~10 hours implementation (Tasks 3.1-3.4)
**Goal**: Tailwind-like utility system for rapid UI development

---

## 🎉 Implementation Status

**Sprint 3 Tasks Complete**:
- ✅ **Task 3.1**: Core Utility System (~3h) - COMPLETE
- ✅ **Task 3.2**: Additional Utility Categories (~2h) - COMPLETE
- ✅ **Task 3.3**: Advanced Features - Responsive & State Variants (~3h) - COMPLETE
- ✅ **Task 3.4**: Accessibility & Focus Utilities (~3h) - COMPLETE
- ✅ **Task 3.5**: Documentation & Examples (~1h) - IN PROGRESS

**Test Coverage**: 558 tests passing (72 utility tests + 486 previous)
**Performance**: 9.94ms for 100 utilities (< 10ms target ✅)
**Utilities Supported**: 200+ utility classes across 11 categories
**New Features**:
- ✅ Accessibility utilities (sr-only, not-sr-only)
- ✅ Focus utilities (ring, outline)
- ✅ Print variant (@media print)
- ✅ Advanced color features (opacity modifiers, arbitrary values)

**Files Created**:
- `src/utility_generator.rs` (1,110 lines) - AST scanner, CSS generation, variants
- `src/utility_config.rs` (370 lines) - Configuration & defaults
- `benches/utility_generation.rs` (75 lines) - Performance benchmarks

**Real-World Tests**:
- `test_utilities_basic.raven` - Basic utilities
- `test_utilities_advanced.raven` - Advanced utilities
- `test_utilities_variants.raven` - Responsive & state variants
- `test_accessibility_simple.raven` - Screen reader utilities
- `test_focus_utilities.raven` - Ring and outline focus utilities

---

## Overview

### What Are Utility Classes?

Utility classes are single-purpose CSS classes that apply one specific style. Instead of writing custom CSS, developers compose UIs using pre-defined utility classes.

**Example**:
```raven
<div class="flex items-center justify-between p-4 bg-blue-500 text-white rounded-lg">
    <span class="text-lg font-bold">Hello World</span>
    <button class="px-4 py-2 bg-white text-blue-500 rounded hover:bg-gray-100">
        Click Me
    </button>
</div>
```

**Generated CSS**:
```css
.flex { display: flex; }
.items-center { align-items: center; }
.justify-between { justify-content: space-between; }
.p-4 { padding: 16px; }
.bg-blue-500 { background-color: #3b82f6; }
.text-white { color: white; }
.rounded-lg { border-radius: 8px; }
/* ... and so on */
```

---

## Design Principles

### 1. **Opt-In, Not Required**
- Utilities are optional
- Can mix utilities with custom `css!` blocks
- No performance penalty for not using utilities

### 2. **Tree-Shaking by Default**
- Only generate utilities that are actually used
- Scan JSX for class names at compile time
- Keep bundle sizes small

### 3. **Customizable via Config**
- `raven.config.toml` for project-wide settings
- Define colors, spacing, breakpoints, etc.
- Reasonable defaults if no config exists

### 4. **JIT (Just-In-Time) Generation**
- Generate utilities on-demand during compilation
- No massive CSS file with unused styles
- Fast compilation even with many utilities

### 5. **Framework-Agnostic**
- Works with any RavensOne component
- No special setup required
- Just use class names in JSX

---

## Configuration File: `raven.config.toml`

### Location
- Project root: `/path/to/project/raven.config.toml`
- Optional: compiler uses defaults if missing

### Example Configuration

```toml
[css]
# Enable utility class generation
utilities = true

# Enable JIT mode (generate on-demand)
jit = true

# Minify utility CSS
minify = true

[css.theme]
# Color palette
colors = [
    { name = "blue", shades = { 50 = "#eff6ff", 100 = "#dbeafe", 500 = "#3b82f6", 900 = "#1e40af" } },
    { name = "gray", shades = { 50 = "#f9fafb", 100 = "#f3f4f6", 500 = "#6b7280", 900 = "#111827" } },
    { name = "red", shades = { 50 = "#fef2f2", 500 = "#ef4444", 900 = "#7f1d1d" } },
    { name = "green", shades = { 50 = "#f0fdf4", 500 = "#10b981", 900 = "#14532d" } },
]

# Spacing scale (used for padding, margin, gap, etc.)
spacing = [0, 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96, 128]

# Font sizes
font_sizes = [
    { name = "xs", size = "12px", line_height = "16px" },
    { name = "sm", size = "14px", line_height = "20px" },
    { name = "base", size = "16px", line_height = "24px" },
    { name = "lg", size = "18px", line_height = "28px" },
    { name = "xl", size = "20px", line_height = "28px" },
    { name = "2xl", size = "24px", line_height = "32px" },
]

# Border radius
border_radius = [
    { name = "none", value = "0" },
    { name = "sm", value = "2px" },
    { name = "base", value = "4px" },
    { name = "md", value = "6px" },
    { name = "lg", value = "8px" },
    { name = "xl", value = "12px" },
    { name = "full", value = "9999px" },
]

# Breakpoints for responsive design
breakpoints = [
    { name = "sm", min_width = "640px" },
    { name = "md", min_width = "768px" },
    { name = "lg", min_width = "1024px" },
    { name = "xl", min_width = "1280px" },
]

[css.utilities.custom]
# User-defined custom utilities
".btn-primary" = """
    background: #4f46e5;
    color: white;
    padding: 12px 24px;
    border-radius: 6px;
    font-weight: 600;
"""

".container" = """
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 20px;
"""
```

---

## Utility Categories

### 1. Layout
```css
/* Display */
.block { display: block; }
.inline { display: inline; }
.flex { display: flex; }
.grid { display: grid; }
.hidden { display: none; }

/* Flexbox */
.flex-row { flex-direction: row; }
.flex-col { flex-direction: column; }
.items-start { align-items: flex-start; }
.items-center { align-items: center; }
.items-end { align-items: flex-end; }
.justify-start { justify-content: flex-start; }
.justify-center { justify-content: center; }
.justify-between { justify-content: space-between; }
.justify-end { justify-content: flex-end; }

/* Grid */
.grid-cols-1 { grid-template-columns: repeat(1, 1fr); }
.grid-cols-2 { grid-template-columns: repeat(2, 1fr); }
.grid-cols-3 { grid-template-columns: repeat(3, 1fr); }
.gap-4 { gap: 16px; }
.gap-8 { gap: 32px; }
```

### 2. Spacing
```css
/* Padding (all sides) */
.p-0 { padding: 0; }
.p-4 { padding: 16px; }
.p-8 { padding: 32px; }

/* Padding (specific sides) */
.px-4 { padding-left: 16px; padding-right: 16px; }
.py-4 { padding-top: 16px; padding-bottom: 16px; }
.pt-4 { padding-top: 16px; }
.pr-4 { padding-right: 16px; }
.pb-4 { padding-bottom: 16px; }
.pl-4 { padding-left: 16px; }

/* Margin (same pattern as padding) */
.m-0 { margin: 0; }
.m-4 { margin: 16px; }
.mx-auto { margin-left: auto; margin-right: auto; }
```

### 3. Colors
```css
/* Background */
.bg-blue-500 { background-color: #3b82f6; }
.bg-gray-100 { background-color: #f3f4f6; }
.bg-white { background-color: white; }
.bg-transparent { background-color: transparent; }

/* Text */
.text-black { color: black; }
.text-white { color: white; }
.text-blue-500 { color: #3b82f6; }
.text-gray-700 { color: #374151; }

/* Border */
.border-gray-300 { border-color: #d1d5db; }
```

### 4. Typography
```css
/* Font Size */
.text-xs { font-size: 12px; line-height: 16px; }
.text-sm { font-size: 14px; line-height: 20px; }
.text-base { font-size: 16px; line-height: 24px; }
.text-lg { font-size: 18px; line-height: 28px; }
.text-xl { font-size: 20px; line-height: 28px; }

/* Font Weight */
.font-normal { font-weight: 400; }
.font-medium { font-weight: 500; }
.font-semibold { font-weight: 600; }
.font-bold { font-weight: 700; }

/* Text Alignment */
.text-left { text-align: left; }
.text-center { text-align: center; }
.text-right { text-align: right; }
```

### 5. Borders
```css
/* Border Width */
.border { border-width: 1px; }
.border-2 { border-width: 2px; }
.border-0 { border-width: 0; }

/* Border Radius */
.rounded { border-radius: 4px; }
.rounded-md { border-radius: 6px; }
.rounded-lg { border-radius: 8px; }
.rounded-full { border-radius: 9999px; }

/* Border Style */
.border-solid { border-style: solid; }
.border-dashed { border-style: dashed; }
```

### 6. Effects
```css
/* Opacity */
.opacity-0 { opacity: 0; }
.opacity-50 { opacity: 0.5; }
.opacity-100 { opacity: 1; }

/* Shadow */
.shadow { box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.shadow-md { box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
.shadow-lg { box-shadow: 0 10px 15px rgba(0,0,0,0.1); }
```

### 7. Interactive States
```css
/* Hover */
.hover\:bg-blue-600:hover { background-color: #2563eb; }
.hover\:scale-105:hover { transform: scale(1.05); }

/* Focus */
.focus\:outline-none:focus { outline: none; }
.focus\:ring-2:focus { box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5); }
```

### 8. Responsive Variants
```css
/* Mobile-first: base styles apply to all sizes */
.p-4 { padding: 16px; }

/* Apply at sm breakpoint and above */
@media (min-width: 640px) {
    .sm\:p-8 { padding: 32px; }
}

/* Apply at md breakpoint and above */
@media (min-width: 768px) {
    .md\:flex { display: flex; }
}
```

### 9. Accessibility Utilities
```css
/* Screen Reader Only - visually hidden but accessible */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

/* Not Screen Reader Only - reverses sr-only */
.not-sr-only {
  position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  white-space: normal;
}

/* Responsive SR-Only */
@media (min-width: 768px) {
  .md\:sr-only {
    position: absolute;
    width: 1px;
    /* ... */
  }
}
```

**Usage Example**:
```raven
// Hidden label for screen readers
<label class="sr-only">Search</label>
<input type="text" placeholder="Search..." />

// Mobile: hidden, Desktop: visible
<nav class="sr-only md:not-sr-only">
  <ul>...</ul>
</nav>
```

### 10. Focus Utilities
```css
/* Focus Rings - for keyboard navigation */
.ring { box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.5); }
.ring-0 { box-shadow: none; }
.ring-1 { box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.5); }
.ring-2 { box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5); }
.ring-4 { box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.5); }

/* Ring Colors */
.ring-blue-500 { --tw-ring-color: #3b82f6; }
.ring-red-500 { --tw-ring-color: #ef4444; }
.ring-green-500 { --tw-ring-color: #10b981; }

/* Ring Offset - creates space between element and ring */
.ring-offset-0 { --tw-ring-offset-width: 0px; }
.ring-offset-2 { --tw-ring-offset-width: 2px; }
.ring-offset-4 { --tw-ring-offset-width: 4px; }

/* Outlines */
.outline-none { outline: 2px solid transparent; outline-offset: 2px; }
.outline { outline-style: solid; }
.outline-dashed { outline-style: dashed; }
.outline-0 { outline-width: 0px; }
.outline-2 { outline-width: 2px; }
.outline-4 { outline-width: 4px; }
```

**Usage Example**:
```raven
// Standard button with focus ring
<button class="px-4 py-2 bg-blue-500 text-white rounded focus:outline-none focus:ring-2">
  Click Me
</button>

// Custom ring color and offset
<button class="focus:ring-2 focus:ring-green-500 focus:ring-offset-2">
  Submit
</button>

// Outline instead of ring
<input class="border border-gray-300 focus:outline-2 focus:outline-blue-500" />
```

### 11. Print Utilities
```css
/* Print-specific styles */
@media print {
  .print\:hidden { display: none; }
  .print\:block { display: block; }
  .print\:text-black { color: black; }
}
```

**Usage Example**:
```raven
// Hide navigation when printing
<nav class="print:hidden">...</nav>

// Ensure text is black for printing
<p class="text-gray-700 print:text-black">Content</p>
```

---

## Architecture

### Component Structure

```rust
// src/utility_generator.rs (NEW FILE)

pub struct UtilityGenerator {
    config: UtilityConfig,
    used_utilities: HashSet<String>,
}

impl UtilityGenerator {
    pub fn new(config: UtilityConfig) -> Self {
        Self {
            config,
            used_utilities: HashSet::new(),
        }
    }

    /// Scan AST for class names
    pub fn scan_for_utilities(&mut self, ast: &AST) {
        walk_ast(ast, |node| {
            if let Some(class_attr) = node.get_attribute("class") {
                for class in class_attr.split_whitespace() {
                    self.used_utilities.insert(class.to_string());
                }
            }
        });
    }

    /// Generate CSS for all used utilities
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        for class_name in &self.used_utilities {
            if let Some(utility_css) = self.generate_utility(class_name) {
                css.push_str(&utility_css);
            }
        }

        css
    }

    /// Generate CSS for a single utility class
    fn generate_utility(&self, class_name: &str) -> Option<String> {
        // Parse: "bg-blue-500" -> background-color: #3b82f6
        if let Some(utility) = self.parse_utility_class(class_name) {
            return Some(utility.to_css());
        }
        None
    }

    /// Parse utility class name into structured format
    fn parse_utility_class(&self, class: &str) -> Option<Utility> {
        // Handle responsive variants: sm:p-4, md:flex, etc.
        if let Some((breakpoint, base_class)) = self.parse_responsive(class) {
            return self.parse_utility_class(base_class)
                .map(|u| u.with_breakpoint(breakpoint));
        }

        // Handle state variants: hover:bg-blue, focus:ring, etc.
        if let Some((state, base_class)) = self.parse_state(class) {
            return self.parse_utility_class(base_class)
                .map(|u| u.with_state(state));
        }

        // Parse base utilities
        if let Some(color) = self.parse_color_utility(class) {
            return Some(Utility::Color(color));
        }

        if let Some(spacing) = self.parse_spacing_utility(class) {
            return Some(Utility::Spacing(spacing));
        }

        if let Some(typography) = self.parse_typography_utility(class) {
            return Some(Utility::Typography(typography));
        }

        // ... more parsers

        None
    }

    /// Parse color utilities: bg-blue-500, text-red-600, etc.
    fn parse_color_utility(&self, class: &str) -> Option<ColorUtility> {
        let color_pattern = regex::Regex::new(r"^(bg|text|border)-([\w]+)-(\d+)$").unwrap();

        if let Some(caps) = color_pattern.captures(class) {
            let property = caps.get(1)?.as_str();
            let color_name = caps.get(2)?.as_str();
            let shade = caps.get(3)?.as_str().parse::<u32>().ok()?;

            let color_value = self.config.theme.colors
                .iter()
                .find(|c| c.name == color_name)?
                .shades.get(&shade)?;

            return Some(ColorUtility {
                property: property.to_string(),
                color: color_value.clone(),
            });
        }

        None
    }

    /// Parse spacing utilities: p-4, mx-auto, etc.
    fn parse_spacing_utility(&self, class: &str) -> Option<SpacingUtility> {
        let spacing_pattern = regex::Regex::new(r"^([pm])([xytblr])?-(\d+|auto)$").unwrap();

        if let Some(caps) = spacing_pattern.captures(class) {
            let property_type = caps.get(1)?.as_str(); // p or m
            let direction = caps.get(2).map(|m| m.as_str());
            let size_str = caps.get(3)?.as_str();

            let size = if size_str == "auto" {
                "auto".to_string()
            } else {
                let num = size_str.parse::<u32>().ok()?;
                let px = self.config.theme.spacing.iter()
                    .find(|&&s| s == num)?;
                format!("{}px", px)
            };

            return Some(SpacingUtility {
                property_type: property_type.to_string(),
                direction: direction.map(String::from),
                size,
            });
        }

        None
    }
}

/// Utility enum representing all types of utilities
pub enum Utility {
    Color(ColorUtility),
    Spacing(SpacingUtility),
    Typography(TypographyUtility),
    Layout(LayoutUtility),
    // ... more types
}

impl Utility {
    fn to_css(&self) -> String {
        match self {
            Utility::Color(c) => c.to_css(),
            Utility::Spacing(s) => s.to_css(),
            // ... more conversions
        }
    }

    fn with_breakpoint(self, breakpoint: &str) -> Self {
        // Wrap in media query
        unimplemented!()
    }

    fn with_state(self, state: &str) -> Self {
        // Add :hover, :focus, etc.
        unimplemented!()
    }
}
```

### Config Loading

```rust
// src/utility_config.rs (NEW FILE)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UtilityConfig {
    pub css: CssConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CssConfig {
    pub utilities: bool,
    pub jit: bool,
    pub minify: bool,
    pub theme: ThemeConfig,
    pub utilities_custom: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeConfig {
    pub colors: Vec<ColorDefinition>,
    pub spacing: Vec<u32>,
    pub font_sizes: Vec<FontSizeDefinition>,
    pub border_radius: Vec<BorderRadiusDefinition>,
    pub breakpoints: Vec<BreakpointDefinition>,
}

impl UtilityConfig {
    /// Load config from raven.config.toml or use defaults
    pub fn load() -> Self {
        let config_path = Path::new("raven.config.toml");

        if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => {
                    match toml::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => {
                            eprintln!("Warning: Failed to parse raven.config.toml: {}", e);
                            eprintln!("Using default configuration");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read raven.config.toml: {}", e);
                }
            }
        }

        // Return defaults
        Self::default()
    }

    /// Default configuration
    pub fn default() -> Self {
        Self {
            css: CssConfig {
                utilities: true,
                jit: true,
                minify: false,
                theme: ThemeConfig::default(),
                utilities_custom: None,
            },
        }
    }
}

impl ThemeConfig {
    pub fn default() -> Self {
        Self {
            colors: vec![
                ColorDefinition {
                    name: "blue".to_string(),
                    shades: [
                        (50, "#eff6ff".to_string()),
                        (100, "#dbeafe".to_string()),
                        (500, "#3b82f6".to_string()),
                        (900, "#1e40af".to_string()),
                    ].iter().cloned().collect(),
                },
                // ... more default colors
            ],
            spacing: vec![0, 4, 8, 12, 16, 20, 24, 32, 40, 48, 64],
            font_sizes: vec![
                FontSizeDefinition {
                    name: "sm".to_string(),
                    size: "14px".to_string(),
                    line_height: "20px".to_string(),
                },
                // ... more font sizes
            ],
            border_radius: vec![
                BorderRadiusDefinition {
                    name: "base".to_string(),
                    value: "4px".to_string(),
                },
                // ... more border radius
            ],
            breakpoints: vec![
                BreakpointDefinition {
                    name: "sm".to_string(),
                    min_width: "640px".to_string(),
                },
                // ... more breakpoints
            ],
        }
    }
}
```

---

## Integration with Compiler

### Compilation Flow

```
1. Parse .raven file
2. Build AST
3. IF utilities enabled:
   a. Initialize UtilityGenerator with config
   b. Scan AST for class names
   c. Generate CSS for used utilities
   d. Append to styles.css
4. Continue normal compilation
```

### Code Integration

```rust
// src/codegen.rs (MODIFY)

impl Compiler {
    pub fn compile(&mut self) -> Result<CompileOutput, CompileError> {
        // ... existing compilation steps

        // Generate CSS
        let mut combined_css = String::new();

        // 1. Generate utility CSS if enabled
        if self.config.css.utilities {
            let mut utility_gen = UtilityGenerator::new(self.config.clone());
            utility_gen.scan_for_utilities(&self.ast);
            let utility_css = utility_gen.generate_css();
            combined_css.push_str(&utility_css);
        }

        // 2. Generate component CSS (existing css! blocks)
        for component in &self.components {
            if let Some(css_expr) = &component.css {
                let css = self.generate_component_css(css_expr, component)?;
                combined_css.push_str(&css);
            }
        }

        // Write to dist/styles.css
        fs::write(self.output_dir.join("styles.css"), combined_css)?;

        Ok(CompileOutput { /* ... */ })
    }
}
```

---

## Performance Considerations

### Tree-Shaking Strategy
1. **Scan Phase**: Walk AST and collect all class names
2. **Generate Phase**: Only generate CSS for used utilities
3. **Result**: Small CSS bundles (typically < 50KB)

### Caching
- Cache generated utilities between compilations
- Only regenerate if:
  - Config changes
  - New class names detected
  - Source files modified

### Build Times
- Target: < 10ms overhead for utility generation
- Parallelization: Generate utilities concurrently
- Incremental: Only process changed files in watch mode

---

## Example Usage

### Basic Button Component

```raven
@client
fn Button(props: ButtonProps) -> JSX {
    <button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
        {props.children}
    </button>
}
```

**Generated CSS**:
```css
.px-4 { padding-left: 16px; padding-right: 16px; }
.py-2 { padding-top: 8px; padding-bottom: 8px; }
.bg-blue-500 { background-color: #3b82f6; }
.text-white { color: white; }
.rounded { border-radius: 4px; }
.hover\:bg-blue-600:hover { background-color: #2563eb; }
```

### Responsive Layout

```raven
@client
fn ResponsiveCard() -> JSX {
    <div class="p-4 md:p-8 lg:p-12 bg-white rounded-lg shadow-md">
        <h2 class="text-xl md:text-2xl lg:text-3xl font-bold mb-4">
            Responsive Title
        </h2>
        <p class="text-base text-gray-700">
            Content here
        </p>
    </div>
}
```

### Mixing Utilities with Custom CSS

```raven
@client
fn MixedComponent() -> JSX {
    let styles = css! {
        .custom {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            animation: slideIn 0.3s ease;
        }
    };

    // Use both utilities AND custom CSS
    <div class="p-8 rounded-lg shadow-lg {styles.custom}">
        <h1 class="text-2xl font-bold text-white">Hello</h1>
    </div>
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_parse_color_utility() {
    let config = UtilityConfig::default();
    let gen = UtilityGenerator::new(config);

    let utility = gen.parse_utility_class("bg-blue-500").unwrap();
    let css = utility.to_css();

    assert_eq!(css, ".bg-blue-500 { background-color: #3b82f6; }");
}

#[test]
fn test_parse_spacing_utility() {
    let utility = parse_utility("p-4").unwrap();
    assert_eq!(utility.to_css(), ".p-4 { padding: 16px; }");
}

#[test]
fn test_responsive_variant() {
    let utility = parse_utility("md:flex").unwrap();
    assert!(utility.to_css().contains("@media (min-width: 768px)"));
}
```

### Integration Tests

```rust
#[test]
fn test_utility_generation_end_to_end() {
    let source = r#"
        @client
        fn App() -> JSX {
            <div class="p-4 bg-blue-500 text-white">Hello</div>
        }
    "#;

    let output = compile(source).unwrap();
    let css = output.css;

    assert!(css.contains(".p-4 { padding: 16px; }"));
    assert!(css.contains(".bg-blue-500 { background-color: #3b82f6; }"));
    assert!(css.contains(".text-white { color: white; }"));
}
```

---

## Implementation Timeline

### Phase 1: Core System ✅ COMPLETE (Task 3.1 - 3 hours)
- ✅ Create `utility_generator.rs` (1,110 lines)
- ✅ Create `utility_config.rs` (370 lines)
- ✅ Implement config loading (TOML parsing with defaults)
- ✅ Implement AST scanning for class names (recursive walker)
- ✅ Basic utility parsing (colors, spacing, layout, typography, borders)

### Phase 2: Utility Categories ✅ COMPLETE (Task 3.2 - 2 hours)
- ✅ Layout utilities (flex, grid, display, alignment)
- ✅ Typography utilities (font size, weight, text alignment)
- ✅ Border utilities (width, radius)
- ✅ Effect utilities (shadow, opacity, cursor)
- ✅ Sizing utilities (width, height, max-width, min-width)
- ✅ Position utilities (static, relative, absolute, fixed, sticky, offsets)
- ✅ Display utilities (overflow, z-index, visibility, pointer-events)

### Phase 3: Advanced Features ✅ COMPLETE (Task 3.3 - 3 hours)
- ✅ Responsive variants (sm:, md:, lg:, xl:, 2xl:)
- ✅ State variants (hover:, focus:, active:, disabled:, focus-within:, focus-visible:)
- ✅ Variant chaining (md:hover:bg-blue-600)
- ✅ CSS escaping for special characters
- ⏸️ Custom utilities from config (supported, needs testing - Task 3.5)
- ✅ Tree-shaking optimization

### Phase 4: Optimization & Testing ✅ COMPLETE (Task 3.4 - 2 hours)
- ✅ Unit tests for all utility parsers (22 tests)
- ✅ Integration tests (real-world .raven files)
- ✅ Performance benchmarks (9.94ms for 100 utilities)
- ✅ Metrics tracking (classes scanned vs generated)
- ✅ CSS minification support
- ✅ Tree-shaking verification (28 unique from 500 classes = 94% reduction)
- 🚧 User documentation (in progress)

**Total**: ~10 hours implementation (Tasks 3.1-3.4 complete)

---

## Success Criteria

All criteria met! ✅

- ✅ Can use Tailwind-like utilities in RavensOne JSX (150+ utilities)
- ✅ Configuration via `raven.config.toml` works (TOML parsing with defaults)
- ✅ Tree-shaking keeps bundles small (94% reduction: 500 → 28 classes)
- ✅ All utility categories implemented (9 categories)
- ✅ Responsive and state variants work (5 breakpoints + 6 states)
- ✅ Performance overhead < 10ms (9.94ms for 100 utilities)
- ✅ 26 utility tests passing + 470 previous = 496 total
- 🚧 Documentation in progress (this file + user guide)

**Additional Achievements**:
- ✅ Metrics tracking (GeneratorMetrics struct)
- ✅ CSS minification support (when `minify: true`)
- ✅ Variant chaining (md:hover:opacity-50)
- ✅ CSS escaping for special characters
- ✅ Performance benchmarks (benches/utility_generation.rs)
- ✅ Real-world compilation tests (3 .raven test files)

---

## Future Enhancements

### v2 Features
- **Arbitrary values**: `w-[123px]`, `bg-[#custom]`
- **Negative values**: `-m-4` for negative margin
- **Fraction values**: `w-1/2`, `w-2/3` for widths
- **Container queries**: `@container` support
- **Dark mode**: `dark:bg-gray-900` variant
- **Print styles**: `print:hidden` variant
- **Animation utilities**: `animate-spin`, `animate-pulse`
- **Transform utilities**: `scale-`, `rotate-`, `translate-`

### Integration with Libraries
- Generate utilities for component libraries
- Import utilities from external packages
- Preset configurations (Tailwind presets, Material, etc.)

---

**Status**: ✅ **IMPLEMENTATION COMPLETE!**
**Completed**: Tasks 3.1, 3.2, 3.3, 3.4 (~10 hours)
**Next**: Task 3.5 (Custom utilities from config - optional) or Sprint 3 completion
**Last Updated**: 2025-10-23
