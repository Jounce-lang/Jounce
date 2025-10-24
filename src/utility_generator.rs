// Jounce Utility Class System - Generator
// Scans AST for utility class names and generates corresponding CSS

use crate::ast::{Expression, JsxChild, Program, Statement};
use crate::utility_config::UtilityConfig;
use regex::Regex;
use std::collections::HashSet;

/// Main utility generator that scans AST and generates CSS
pub struct UtilityGenerator {
    config: UtilityConfig,
    pub used_utilities: HashSet<String>,
    metrics: GeneratorMetrics,
}

/// Metrics for utility generation (tree-shaking verification)
#[derive(Debug, Clone, Default)]
pub struct GeneratorMetrics {
    /// Total number of class names scanned from AST
    pub classes_scanned: usize,
    /// Number of utilities successfully generated
    pub utilities_generated: usize,
    /// Number of classes that couldn't be parsed as utilities
    pub unrecognized_classes: usize,
}

impl UtilityGenerator {
    /// Create a new utility generator with the given configuration
    pub fn new(config: UtilityConfig) -> Self {
        Self {
            config,
            used_utilities: HashSet::new(),
            metrics: GeneratorMetrics::default(),
        }
    }

    /// Get metrics for utility generation
    pub fn metrics(&self) -> &GeneratorMetrics {
        &self.metrics
    }

    /// Scan AST for class names and collect all utilities
    pub fn scan_for_utilities(&mut self, program: &Program) {
        for statement in &program.statements {
            self.scan_statement(statement);
        }
    }

    /// Scan a statement for JSX elements
    fn scan_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Function(func) => {
                for stmt in &func.body.statements {
                    self.scan_statement(stmt);
                }
            }
            Statement::Component(comp) => {
                for stmt in &comp.body.statements {
                    self.scan_statement(stmt);
                }
            }
            Statement::Expression(expr) => {
                self.scan_expression(expr);
            }
            Statement::Return(ret_stmt) => {
                self.scan_expression(&ret_stmt.value);
            }
            Statement::Let(let_stmt) => {
                self.scan_expression(&let_stmt.value);
            }
            Statement::If(if_stmt) => {
                for stmt in &if_stmt.then_branch.statements {
                    self.scan_statement(stmt);
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.scan_statement(else_branch);
                }
            }
            Statement::While(while_stmt) => {
                for stmt in &while_stmt.body.statements {
                    self.scan_statement(stmt);
                }
            }
            Statement::For(for_stmt) => {
                for stmt in &for_stmt.body.statements {
                    self.scan_statement(stmt);
                }
            }
            Statement::ForIn(for_in_stmt) => {
                for stmt in &for_in_stmt.body.statements {
                    self.scan_statement(stmt);
                }
            }
            _ => {}
        }
    }

    /// Scan an expression for JSX elements
    fn scan_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::JsxElement(jsx) => {
                self.scan_jsx_element(jsx);
            }
            Expression::IfExpression(if_expr) => {
                self.scan_expression(&if_expr.then_expr);
                if let Some(else_expr) = &if_expr.else_expr {
                    self.scan_expression(else_expr);
                }
            }
            Expression::Ternary(ternary) => {
                self.scan_expression(&ternary.true_expr);
                self.scan_expression(&ternary.false_expr);
            }
            Expression::Block(block) => {
                for stmt in &block.statements {
                    self.scan_statement(stmt);
                }
            }
            Expression::Match(match_expr) => {
                for arm in &match_expr.arms {
                    self.scan_expression(&arm.body);
                }
            }
            Expression::FunctionCall(call) => {
                for arg in &call.arguments {
                    self.scan_expression(arg);
                }
            }
            Expression::ArrayLiteral(arr) => {
                for elem in &arr.elements {
                    self.scan_expression(elem);
                }
            }
            _ => {}
        }
    }

    /// Scan a JSX element and its children for class attributes
    fn scan_jsx_element(&mut self, jsx: &crate::ast::JsxElement) {
        // Scan attributes for "class" or "className"
        for attr in &jsx.opening_tag.attributes {
            if attr.name.value == "class" || attr.name.value == "className" {
                if let Expression::StringLiteral(class_str) = &attr.value {
                    // Split by whitespace and collect each class name
                    for class_name in class_str.split_whitespace() {
                        self.used_utilities.insert(class_name.to_string());
                        self.metrics.classes_scanned += 1;
                    }
                }
            }
        }

        // Recursively scan children
        for child in &jsx.children {
            match child {
                JsxChild::Element(elem) => {
                    self.scan_jsx_element(elem);
                }
                JsxChild::Expression(expr) => {
                    self.scan_expression(expr);
                }
                _ => {}
            }
        }
    }

    /// Generate CSS for all used utilities
    pub fn generate_css(&mut self) -> String {
        if !self.config.css.utilities {
            return String::new();
        }

        let mut css = String::new();
        let minify = self.config.css.minify;

        // Add header comment (unless minifying)
        if !minify {
            css.push_str("/* Jounce Utility Classes */\n\n");
        }

        // Generate CSS custom properties for theme (if enabled)
        if self.config.css.theme_mode.unwrap_or(false) {
            css.push_str(&self.generate_theme_variables());
            if !minify {
                css.push('\n');
            }
        }

        // Generate CSS for each used utility
        for class_name in &self.used_utilities.clone() {
            if let Some(utility_css) = self.generate_utility(class_name) {
                if minify {
                    // Minified: remove extra whitespace and newlines
                    let minified = utility_css
                        .lines()
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");
                    css.push_str(&minified);
                } else {
                    css.push_str(&utility_css);
                    css.push('\n');
                }
                self.metrics.utilities_generated += 1;
            } else {
                self.metrics.unrecognized_classes += 1;
            }
        }

        css
    }

    /// Generate :root block with CSS custom properties for theming
    /// Example output:
    /// ```css
    /// :root {
    ///   --color-blue-500: #3b82f6;
    ///   --color-gray-100: #f3f4f6;
    ///   --spacing-4: 4px;
    ///   --font-size-base: 16px;
    /// }
    /// ```
    fn generate_theme_variables(&self) -> String {
        let mut css = String::new();
        let minify = self.config.css.minify;

        if minify {
            css.push_str(":root{");
        } else {
            css.push_str(":root {\n");
        }

        // Generate color variables
        for color_def in &self.config.css.theme.colors {
            for (shade, hex) in &color_def.shades {
                let var_name = format!("--color-{}-{}", color_def.name, shade);
                if minify {
                    css.push_str(&format!("{}:{};", var_name, hex));
                } else {
                    css.push_str(&format!("  {}: {};\n", var_name, hex));
                }
            }
        }

        // Generate spacing variables
        for size in &self.config.css.theme.spacing {
            let var_name = format!("--spacing-{}", size);
            if minify {
                css.push_str(&format!("{}:{}px;", var_name, size));
            } else {
                css.push_str(&format!("  {}: {}px;\n", var_name, size));
            }
        }

        // Generate font size variables
        for font_size in &self.config.css.theme.font_sizes {
            let var_name = format!("--font-size-{}", font_size.name);
            if minify {
                css.push_str(&format!("{}:{};", var_name, font_size.size));
            } else {
                css.push_str(&format!("  {}: {};\n", var_name, font_size.size));
            }

            // Also add line height
            let lh_var_name = format!("--line-height-{}", font_size.name);
            if minify {
                css.push_str(&format!("{}:{};", lh_var_name, font_size.line_height));
            } else {
                css.push_str(&format!("  {}: {};\n", lh_var_name, font_size.line_height));
            }
        }

        // Generate border radius variables
        for radius in &self.config.css.theme.border_radius {
            let var_name = format!("--radius-{}", radius.name);
            if minify {
                css.push_str(&format!("{}:{};", var_name, radius.value));
            } else {
                css.push_str(&format!("  {}: {};\n", var_name, radius.value));
            }
        }

        // Generate breakpoint variables (for use in media queries)
        for breakpoint in &self.config.css.theme.breakpoints {
            let var_name = format!("--breakpoint-{}", breakpoint.name);
            if minify {
                css.push_str(&format!("{}:{};", var_name, breakpoint.min_width));
            } else {
                css.push_str(&format!("  {}: {};\n", var_name, breakpoint.min_width));
            }
        }

        if minify {
            css.push('}');
        } else {
            css.push_str("}\n");
        }

        css
    }

    /// Generate CSS for a single utility class
    fn generate_utility(&self, class_name: &str) -> Option<String> {
        // Parse variants (responsive and state)
        let (variants, base_class) = self.parse_variants(class_name);

        // Check custom utilities first
        if let Some(custom_utilities) = &self.config.css.utilities_custom {
            if let Some(custom_css) = custom_utilities.get(&base_class) {
                let css = format!(".{} {{\n{}\n}}\n", self.escape_css_class(class_name), custom_css.trim());
                return Some(self.wrap_with_variants(&css, class_name, &variants));
            }
        }

        // Generate base utility CSS
        let base_css = self.generate_base_utility(&base_class)?;

        // If there are variants, wrap the CSS appropriately
        if !variants.is_empty() {
            Some(self.wrap_with_variants(&base_css, class_name, &variants))
        } else {
            Some(base_css)
        }
    }

    /// Generate CSS for a base utility (without variants)
    fn generate_base_utility(&self, class_name: &str) -> Option<String> {
        // Try parsing as a standard utility
        if let Some(utility) = self.parse_utility_class(class_name) {
            return Some(utility.to_css());
        }

        // Try parsing layout utilities
        if let Some(layout) = self.parse_layout_utility(class_name) {
            return Some(layout);
        }

        // Try parsing typography utilities
        if let Some(typo) = self.parse_typography_utility(class_name) {
            return Some(typo);
        }

        // Try parsing border utilities
        if let Some(border) = self.parse_border_utility(class_name) {
            return Some(border);
        }

        // Try parsing sizing utilities (width, height)
        if let Some(sizing) = self.parse_sizing_utility(class_name) {
            return Some(sizing);
        }

        // Try parsing position utilities
        if let Some(position) = self.parse_position_utility(class_name) {
            return Some(position);
        }

        // Try parsing effect utilities (shadow, opacity)
        if let Some(effect) = self.parse_effect_utility(class_name) {
            return Some(effect);
        }

        // Try parsing display utilities (overflow, z-index)
        if let Some(display) = self.parse_display_utility(class_name) {
            return Some(display);
        }

        // Try parsing accessibility utilities (sr-only)
        if let Some(a11y) = self.parse_a11y_utility(class_name) {
            return Some(a11y);
        }

        // Try parsing focus utilities (ring, outline)
        if let Some(focus) = self.parse_focus_utility(class_name) {
            return Some(focus);
        }

        None
    }

    /// Parse variants from class name (e.g., "md:hover:p-4" -> (["md", "hover"], "p-4"))
    fn parse_variants(&self, class_name: &str) -> (Vec<Variant>, String) {
        let mut variants = Vec::new();
        let mut parts: Vec<&str> = class_name.split(':').collect();

        // Extract base class (last part)
        if parts.len() == 1 {
            return (variants, class_name.to_string());
        }

        let base_class = parts.pop().unwrap_or(class_name).to_string();

        // Parse each variant prefix
        for part in parts {
            if let Some(variant) = self.parse_single_variant(part) {
                variants.push(variant);
            }
        }

        (variants, base_class)
    }

    /// Parse a single variant prefix
    fn parse_single_variant(&self, prefix: &str) -> Option<Variant> {
        // Check for responsive variants
        if let Some(breakpoint) = self.config.css.theme.breakpoints.iter().find(|b| b.name == prefix) {
            return Some(Variant::Responsive {
                breakpoint: prefix.to_string(),
                min_width: breakpoint.min_width.clone(),
            });
        }

        // Check for dark mode variant
        if prefix == "dark" {
            return Some(Variant::Dark);
        }

        // Check for print variant
        if prefix == "print" {
            return Some(Variant::Print);
        }

        // Check for state variants
        match prefix {
            "hover" => Some(Variant::State("hover".to_string())),
            "focus" => Some(Variant::State("focus".to_string())),
            "active" => Some(Variant::State("active".to_string())),
            "disabled" => Some(Variant::State("disabled".to_string())),
            "focus-within" => Some(Variant::State("focus-within".to_string())),
            "focus-visible" => Some(Variant::State("focus-visible".to_string())),
            _ => None,
        }
    }

    /// Escape special characters in CSS class names
    fn escape_css_class(&self, class_name: &str) -> String {
        class_name
            .replace(':', "\\:")
            .replace('/', "\\/")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('.', "\\.")
            .replace('%', "\\%")
    }

    /// Wrap CSS with variants (responsive and/or state)
    fn wrap_with_variants(&self, base_css: &str, full_class_name: &str, variants: &[Variant]) -> String {
        if variants.is_empty() {
            return base_css.to_string();
        }

        // Extract the selector and declarations from base CSS
        // Format: ".class-name { declarations }"
        let escaped_class = self.escape_css_class(full_class_name);

        // Find the opening brace
        if let Some(brace_pos) = base_css.find('{') {
            // Get the declarations (everything after the opening brace)
            let declarations = &base_css[brace_pos..];

            // Build the selector with state variants (pseudo-classes)
            let mut selector = format!(".{}", escaped_class);
            for variant in variants.iter().rev() {
                if let Variant::State(state) = variant {
                    selector.push(':');
                    selector.push_str(state);
                }
            }

            // Prepend .dark parent for dark mode variants
            let has_dark = variants.iter().any(|v| matches!(v, Variant::Dark));
            if has_dark {
                selector = format!(".dark {}", selector);
            }

            // Build the CSS with selector and declarations
            let mut css = format!("{} {}", selector, declarations);

            // Wrap with print variant if present
            let has_print = variants.iter().any(|v| matches!(v, Variant::Print));
            if has_print {
                css = format!("@media print {{\n  {}\n}}", css.trim());
            }

            // Wrap with responsive variants (media queries) - outermost first
            for variant in variants {
                if let Variant::Responsive { min_width, .. } = variant {
                    css = format!("@media (min-width: {}) {{\n  {}\n}}", min_width, css.trim());
                }
            }

            css
        } else {
            // Fallback if we can't parse the CSS
            base_css.to_string()
        }
    }

    /// Parse utility class name into structured format
    fn parse_utility_class(&self, class: &str) -> Option<Utility> {
        // Try color utilities first
        if let Some(color) = self.parse_color_utility(class) {
            return Some(Utility::Color(color));
        }

        // Try spacing utilities
        if let Some(spacing) = self.parse_spacing_utility(class) {
            return Some(Utility::Spacing(spacing));
        }

        None
    }

    /// Parse color utilities: bg-blue-500, text-red-600, border-gray-300, etc.
    fn parse_color_utility(&self, class: &str) -> Option<ColorUtility> {
        // Pattern: (bg|text|border)-(color)-(shade)[/opacity]
        let color_pattern = Regex::new(r"^(bg|text|border)-([\w]+)-(\d+)(?:/(\d+))?$").ok()?;

        if let Some(caps) = color_pattern.captures(class) {
            let property = caps.get(1)?.as_str();
            let color_name = caps.get(2)?.as_str();
            let shade = caps.get(3)?.as_str().parse::<u32>().ok()?;
            let opacity = caps.get(4).and_then(|m| m.as_str().parse::<u32>().ok());

            // Look up color in theme
            let color = self.config.css.theme.colors
                .iter()
                .find(|c| c.name == color_name)?;

            let mut color_value = color.shades.get(&shade)?.clone();

            // Apply opacity if specified
            if let Some(opacity_percent) = opacity {
                color_value = self.apply_opacity_to_color(&color_value, opacity_percent);
            }

            return Some(ColorUtility {
                class_name: class.to_string(),
                property: property.to_string(),
                color: color_value,
            });
        }

        // Special named colors: bg-white, text-black, etc. with optional opacity
        let named_pattern = Regex::new(r"^(bg|text|border)-(white|black|transparent)(?:/(\d+))?$").ok()?;
        if let Some(caps) = named_pattern.captures(class) {
            let property = caps.get(1)?.as_str();
            let color_name = caps.get(2)?.as_str();
            let opacity = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());

            let mut color_value = color_name.to_string();

            // Apply opacity if specified
            if let Some(opacity_percent) = opacity {
                color_value = self.apply_opacity_to_color(&color_value, opacity_percent);
            }

            return Some(ColorUtility {
                class_name: class.to_string(),
                property: property.to_string(),
                color: color_value,
            });
        }

        // Arbitrary color values: bg-[#1da1f2], text-[rgb(255,0,0)], etc.
        let arbitrary_pattern = Regex::new(r"^(bg|text|border)-\[([^\]]+)\]$").ok()?;
        if let Some(caps) = arbitrary_pattern.captures(class) {
            let property = caps.get(1)?.as_str();
            let color_value = caps.get(2)?.as_str();

            // Validate color format (hex, rgb, rgba, hsl, hsla)
            if self.is_valid_color_value(color_value) {
                return Some(ColorUtility {
                    class_name: class.to_string(),
                    property: property.to_string(),
                    color: color_value.to_string(),
                });
            }
        }

        None
    }

    /// Check if a value is a valid CSS color
    fn is_valid_color_value(&self, value: &str) -> bool {
        // Hex colors: #rgb, #rrggbb, #rrggbbaa
        if value.starts_with('#') {
            let hex = value.trim_start_matches('#');
            return hex.len() == 3 || hex.len() == 6 || hex.len() == 8;
        }

        // RGB/RGBA
        if value.starts_with("rgb(") || value.starts_with("rgba(") {
            return value.ends_with(')');
        }

        // HSL/HSLA
        if value.starts_with("hsl(") || value.starts_with("hsla(") {
            return value.ends_with(')');
        }

        false
    }

    /// Apply opacity to a color value
    /// Converts hex (#rrggbb) to rgba format with opacity
    fn apply_opacity_to_color(&self, color: &str, opacity_percent: u32) -> String {
        let opacity = (opacity_percent as f32) / 100.0;

        // Handle special colors
        if color == "white" {
            return format!("rgba(255, 255, 255, {})", opacity);
        } else if color == "black" {
            return format!("rgba(0, 0, 0, {})", opacity);
        } else if color == "transparent" {
            return "transparent".to_string();
        }

        // Parse hex color
        if color.starts_with('#') {
            let hex = color.trim_start_matches('#');

            // Convert hex to RGB
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    return format!("rgba({}, {}, {}, {})", r, g, b, opacity);
                }
            }
        }

        // Fallback: return original color
        color.to_string()
    }

    /// Parse spacing utilities: p-4, mx-auto, etc.
    fn parse_spacing_utility(&self, class: &str) -> Option<SpacingUtility> {
        // Pattern: (p|m)(x|y|t|r|b|l)?-(number|auto)
        let spacing_pattern = Regex::new(r"^([pm])([xytblr])?-(\d+|auto)$").ok()?;

        if let Some(caps) = spacing_pattern.captures(class) {
            let property_type = caps.get(1)?.as_str(); // p or m
            let direction = caps.get(2).map(|m| m.as_str());
            let size_str = caps.get(3)?.as_str();

            let size = if size_str == "auto" {
                "auto".to_string()
            } else {
                let num = size_str.parse::<u32>().ok()?;
                // Check if spacing value exists in theme
                if !self.config.css.theme.spacing.contains(&num) {
                    return None;
                }
                format!("{}px", num)
            };

            return Some(SpacingUtility {
                class_name: class.to_string(),
                property_type: property_type.to_string(),
                direction: direction.map(String::from),
                size,
            });
        }

        None
    }

    /// Parse layout utilities: flex, grid, hidden, etc.
    fn parse_layout_utility(&self, class: &str) -> Option<String> {
        let css = match class {
            // Display
            "block" => ".block { display: block; }",
            "inline" => ".inline { display: inline; }",
            "inline-block" => ".inline-block { display: inline-block; }",
            "flex" => ".flex { display: flex; }",
            "inline-flex" => ".inline-flex { display: inline-flex; }",
            "grid" => ".grid { display: grid; }",
            "inline-grid" => ".inline-grid { display: inline-grid; }",
            "hidden" => ".hidden { display: none; }",

            // Flexbox
            "flex-row" => ".flex-row { flex-direction: row; }",
            "flex-row-reverse" => ".flex-row-reverse { flex-direction: row-reverse; }",
            "flex-col" => ".flex-col { flex-direction: column; }",
            "flex-col-reverse" => ".flex-col-reverse { flex-direction: column-reverse; }",
            "flex-wrap" => ".flex-wrap { flex-wrap: wrap; }",
            "flex-nowrap" => ".flex-nowrap { flex-wrap: nowrap; }",

            // Align Items
            "items-start" => ".items-start { align-items: flex-start; }",
            "items-center" => ".items-center { align-items: center; }",
            "items-end" => ".items-end { align-items: flex-end; }",
            "items-baseline" => ".items-baseline { align-items: baseline; }",
            "items-stretch" => ".items-stretch { align-items: stretch; }",

            // Justify Content
            "justify-start" => ".justify-start { justify-content: flex-start; }",
            "justify-center" => ".justify-center { justify-content: center; }",
            "justify-end" => ".justify-end { justify-content: flex-end; }",
            "justify-between" => ".justify-between { justify-content: space-between; }",
            "justify-around" => ".justify-around { justify-content: space-around; }",
            "justify-evenly" => ".justify-evenly { justify-content: space-evenly; }",

            // Grid Template Columns
            "grid-cols-1" => ".grid-cols-1 { grid-template-columns: repeat(1, 1fr); }",
            "grid-cols-2" => ".grid-cols-2 { grid-template-columns: repeat(2, 1fr); }",
            "grid-cols-3" => ".grid-cols-3 { grid-template-columns: repeat(3, 1fr); }",
            "grid-cols-4" => ".grid-cols-4 { grid-template-columns: repeat(4, 1fr); }",
            "grid-cols-5" => ".grid-cols-5 { grid-template-columns: repeat(5, 1fr); }",
            "grid-cols-6" => ".grid-cols-6 { grid-template-columns: repeat(6, 1fr); }",
            "grid-cols-7" => ".grid-cols-7 { grid-template-columns: repeat(7, 1fr); }",
            "grid-cols-8" => ".grid-cols-8 { grid-template-columns: repeat(8, 1fr); }",
            "grid-cols-9" => ".grid-cols-9 { grid-template-columns: repeat(9, 1fr); }",
            "grid-cols-10" => ".grid-cols-10 { grid-template-columns: repeat(10, 1fr); }",
            "grid-cols-11" => ".grid-cols-11 { grid-template-columns: repeat(11, 1fr); }",
            "grid-cols-12" => ".grid-cols-12 { grid-template-columns: repeat(12, 1fr); }",
            "grid-cols-none" => ".grid-cols-none { grid-template-columns: none; }",
            "grid-cols-subgrid" => ".grid-cols-subgrid { grid-template-columns: subgrid; }",

            // Grid Template Rows
            "grid-rows-1" => ".grid-rows-1 { grid-template-rows: repeat(1, 1fr); }",
            "grid-rows-2" => ".grid-rows-2 { grid-template-rows: repeat(2, 1fr); }",
            "grid-rows-3" => ".grid-rows-3 { grid-template-rows: repeat(3, 1fr); }",
            "grid-rows-4" => ".grid-rows-4 { grid-template-rows: repeat(4, 1fr); }",
            "grid-rows-5" => ".grid-rows-5 { grid-template-rows: repeat(5, 1fr); }",
            "grid-rows-6" => ".grid-rows-6 { grid-template-rows: repeat(6, 1fr); }",
            "grid-rows-none" => ".grid-rows-none { grid-template-rows: none; }",
            "grid-rows-subgrid" => ".grid-rows-subgrid { grid-template-rows: subgrid; }",

            // Grid Auto Flow
            "grid-flow-row" => ".grid-flow-row { grid-auto-flow: row; }",
            "grid-flow-col" => ".grid-flow-col { grid-auto-flow: column; }",
            "grid-flow-dense" => ".grid-flow-dense { grid-auto-flow: dense; }",
            "grid-flow-row-dense" => ".grid-flow-row-dense { grid-auto-flow: row dense; }",
            "grid-flow-col-dense" => ".grid-flow-col-dense { grid-auto-flow: column dense; }",

            // Grid Auto Columns
            "auto-cols-auto" => ".auto-cols-auto { grid-auto-columns: auto; }",
            "auto-cols-min" => ".auto-cols-min { grid-auto-columns: min-content; }",
            "auto-cols-max" => ".auto-cols-max { grid-auto-columns: max-content; }",
            "auto-cols-fr" => ".auto-cols-fr { grid-auto-columns: minmax(0, 1fr); }",

            // Grid Auto Rows
            "auto-rows-auto" => ".auto-rows-auto { grid-auto-rows: auto; }",
            "auto-rows-min" => ".auto-rows-min { grid-auto-rows: min-content; }",
            "auto-rows-max" => ".auto-rows-max { grid-auto-rows: max-content; }",
            "auto-rows-fr" => ".auto-rows-fr { grid-auto-rows: minmax(0, 1fr); }",

            // Grid Column Span
            "col-auto" => ".col-auto { grid-column: auto; }",
            "col-span-1" => ".col-span-1 { grid-column: span 1 / span 1; }",
            "col-span-2" => ".col-span-2 { grid-column: span 2 / span 2; }",
            "col-span-3" => ".col-span-3 { grid-column: span 3 / span 3; }",
            "col-span-4" => ".col-span-4 { grid-column: span 4 / span 4; }",
            "col-span-5" => ".col-span-5 { grid-column: span 5 / span 5; }",
            "col-span-6" => ".col-span-6 { grid-column: span 6 / span 6; }",
            "col-span-7" => ".col-span-7 { grid-column: span 7 / span 7; }",
            "col-span-8" => ".col-span-8 { grid-column: span 8 / span 8; }",
            "col-span-9" => ".col-span-9 { grid-column: span 9 / span 9; }",
            "col-span-10" => ".col-span-10 { grid-column: span 10 / span 10; }",
            "col-span-11" => ".col-span-11 { grid-column: span 11 / span 11; }",
            "col-span-12" => ".col-span-12 { grid-column: span 12 / span 12; }",
            "col-span-full" => ".col-span-full { grid-column: 1 / -1; }",

            // Grid Column Start/End
            "col-start-1" => ".col-start-1 { grid-column-start: 1; }",
            "col-start-2" => ".col-start-2 { grid-column-start: 2; }",
            "col-start-3" => ".col-start-3 { grid-column-start: 3; }",
            "col-start-4" => ".col-start-4 { grid-column-start: 4; }",
            "col-start-5" => ".col-start-5 { grid-column-start: 5; }",
            "col-start-6" => ".col-start-6 { grid-column-start: 6; }",
            "col-start-7" => ".col-start-7 { grid-column-start: 7; }",
            "col-start-8" => ".col-start-8 { grid-column-start: 8; }",
            "col-start-9" => ".col-start-9 { grid-column-start: 9; }",
            "col-start-10" => ".col-start-10 { grid-column-start: 10; }",
            "col-start-11" => ".col-start-11 { grid-column-start: 11; }",
            "col-start-12" => ".col-start-12 { grid-column-start: 12; }",
            "col-start-13" => ".col-start-13 { grid-column-start: 13; }",
            "col-start-auto" => ".col-start-auto { grid-column-start: auto; }",

            "col-end-1" => ".col-end-1 { grid-column-end: 1; }",
            "col-end-2" => ".col-end-2 { grid-column-end: 2; }",
            "col-end-3" => ".col-end-3 { grid-column-end: 3; }",
            "col-end-4" => ".col-end-4 { grid-column-end: 4; }",
            "col-end-5" => ".col-end-5 { grid-column-end: 5; }",
            "col-end-6" => ".col-end-6 { grid-column-end: 6; }",
            "col-end-7" => ".col-end-7 { grid-column-end: 7; }",
            "col-end-8" => ".col-end-8 { grid-column-end: 8; }",
            "col-end-9" => ".col-end-9 { grid-column-end: 9; }",
            "col-end-10" => ".col-end-10 { grid-column-end: 10; }",
            "col-end-11" => ".col-end-11 { grid-column-end: 11; }",
            "col-end-12" => ".col-end-12 { grid-column-end: 12; }",
            "col-end-13" => ".col-end-13 { grid-column-end: 13; }",
            "col-end-auto" => ".col-end-auto { grid-column-end: auto; }",

            // Grid Row Span
            "row-auto" => ".row-auto { grid-row: auto; }",
            "row-span-1" => ".row-span-1 { grid-row: span 1 / span 1; }",
            "row-span-2" => ".row-span-2 { grid-row: span 2 / span 2; }",
            "row-span-3" => ".row-span-3 { grid-row: span 3 / span 3; }",
            "row-span-4" => ".row-span-4 { grid-row: span 4 / span 4; }",
            "row-span-5" => ".row-span-5 { grid-row: span 5 / span 5; }",
            "row-span-6" => ".row-span-6 { grid-row: span 6 / span 6; }",
            "row-span-full" => ".row-span-full { grid-row: 1 / -1; }",

            // Grid Row Start/End
            "row-start-1" => ".row-start-1 { grid-row-start: 1; }",
            "row-start-2" => ".row-start-2 { grid-row-start: 2; }",
            "row-start-3" => ".row-start-3 { grid-row-start: 3; }",
            "row-start-4" => ".row-start-4 { grid-row-start: 4; }",
            "row-start-5" => ".row-start-5 { grid-row-start: 5; }",
            "row-start-6" => ".row-start-6 { grid-row-start: 6; }",
            "row-start-7" => ".row-start-7 { grid-row-start: 7; }",
            "row-start-auto" => ".row-start-auto { grid-row-start: auto; }",

            "row-end-1" => ".row-end-1 { grid-row-end: 1; }",
            "row-end-2" => ".row-end-2 { grid-row-end: 2; }",
            "row-end-3" => ".row-end-3 { grid-row-end: 3; }",
            "row-end-4" => ".row-end-4 { grid-row-end: 4; }",
            "row-end-5" => ".row-end-5 { grid-row-end: 5; }",
            "row-end-6" => ".row-end-6 { grid-row-end: 6; }",
            "row-end-7" => ".row-end-7 { grid-row-end: 7; }",
            "row-end-auto" => ".row-end-auto { grid-row-end: auto; }",

            // Gap
            _ if class.starts_with("gap-x-") => {
                if let Some(size_str) = class.strip_prefix("gap-x-") {
                    if let Ok(size) = size_str.parse::<u32>() {
                        if self.config.css.theme.spacing.contains(&size) {
                            return Some(format!(".gap-x-{} {{ column-gap: {}px; }}", size, size));
                        }
                    }
                }
                return None;
            }
            _ if class.starts_with("gap-y-") => {
                if let Some(size_str) = class.strip_prefix("gap-y-") {
                    if let Ok(size) = size_str.parse::<u32>() {
                        if self.config.css.theme.spacing.contains(&size) {
                            return Some(format!(".gap-y-{} {{ row-gap: {}px; }}", size, size));
                        }
                    }
                }
                return None;
            }
            _ if class.starts_with("gap-") => {
                if let Some(size_str) = class.strip_prefix("gap-") {
                    if let Ok(size) = size_str.parse::<u32>() {
                        if self.config.css.theme.spacing.contains(&size) {
                            return Some(format!(".gap-{} {{ gap: {}px; }}", size, size));
                        }
                    }
                }
                return None;
            }

            _ => return None,
        };

        Some(css.to_string())
    }

    /// Parse typography utilities: text-lg, font-bold, etc.
    fn parse_typography_utility(&self, class: &str) -> Option<String> {
        // Font size
        if class.starts_with("text-") {
            if let Some(size_name) = class.strip_prefix("text-") {
                // Check if it's a named font size
                if let Some(font_size) = self.config.css.theme.font_sizes.iter().find(|f| f.name == size_name) {
                    return Some(format!(
                        ".text-{} {{ font-size: {}; line-height: {}; }}",
                        size_name, font_size.size, font_size.line_height
                    ));
                }

                // Text alignment
                match size_name {
                    "left" => return Some(".text-left { text-align: left; }".to_string()),
                    "center" => return Some(".text-center { text-align: center; }".to_string()),
                    "right" => return Some(".text-right { text-align: right; }".to_string()),
                    "justify" => return Some(".text-justify { text-align: justify; }".to_string()),
                    _ => {}
                }
            }
        }

        // Font weight
        let css = match class {
            "font-thin" => ".font-thin { font-weight: 100; }",
            "font-extralight" => ".font-extralight { font-weight: 200; }",
            "font-light" => ".font-light { font-weight: 300; }",
            "font-normal" => ".font-normal { font-weight: 400; }",
            "font-medium" => ".font-medium { font-weight: 500; }",
            "font-semibold" => ".font-semibold { font-weight: 600; }",
            "font-bold" => ".font-bold { font-weight: 700; }",
            "font-extrabold" => ".font-extrabold { font-weight: 800; }",
            "font-black" => ".font-black { font-weight: 900; }",
            _ => return None,
        };

        Some(css.to_string())
    }

    /// Parse border utilities: border, rounded-lg, etc.
    fn parse_border_utility(&self, class: &str) -> Option<String> {
        // Border width
        if class == "border" {
            return Some(".border { border-width: 1px; border-style: solid; }".to_string());
        }
        if class.starts_with("border-") {
            if let Some(width_str) = class.strip_prefix("border-") {
                if let Ok(width) = width_str.parse::<u32>() {
                    return Some(format!(".border-{} {{ border-width: {}px; border-style: solid; }}", width, width));
                }
            }
        }

        // Border radius
        if class == "rounded" {
            return Some(".rounded { border-radius: 4px; }".to_string());
        }
        if class.starts_with("rounded-") {
            if let Some(radius_name) = class.strip_prefix("rounded-") {
                if let Some(radius) = self.config.css.theme.border_radius.iter().find(|r| r.name == radius_name) {
                    return Some(format!(".rounded-{} {{ border-radius: {}; }}", radius_name, radius.value));
                }
            }
        }

        None
    }

    /// Parse sizing utilities: w-*, h-*, min-w-*, max-w-*, etc.
    fn parse_sizing_utility(&self, class: &str) -> Option<String> {
        // Width utilities
        if class.starts_with("w-") {
            if let Some(size) = class.strip_prefix("w-") {
                match size {
                    "full" => return Some(".w-full { width: 100%; }".to_string()),
                    "screen" => return Some(".w-screen { width: 100vw; }".to_string()),
                    "auto" => return Some(".w-auto { width: auto; }".to_string()),
                    "fit" => return Some(".w-fit { width: fit-content; }".to_string()),
                    _ => {
                        // Parse fraction like "1/2", "1/3", etc.
                        if size.contains('/') {
                            let parts: Vec<&str> = size.split('/').collect();
                            if parts.len() == 2 {
                                if let (Ok(num), Ok(denom)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                                    let percent = (num as f64 / denom as f64) * 100.0;
                                    return Some(format!(".w-{} {{ width: {:.6}%; }}", size.replace('/', "\\/"), percent));
                                }
                            }
                        }
                        // Parse numeric value with spacing scale
                        if let Ok(num) = size.parse::<u32>() {
                            if self.config.css.theme.spacing.contains(&num) {
                                return Some(format!(".w-{} {{ width: {}px; }}", num, num));
                            }
                        }
                    }
                }
            }
        }

        // Height utilities
        if class.starts_with("h-") {
            if let Some(size) = class.strip_prefix("h-") {
                match size {
                    "full" => return Some(".h-full { height: 100%; }".to_string()),
                    "screen" => return Some(".h-screen { height: 100vh; }".to_string()),
                    "auto" => return Some(".h-auto { height: auto; }".to_string()),
                    "fit" => return Some(".h-fit { height: fit-content; }".to_string()),
                    _ => {
                        // Parse numeric value with spacing scale
                        if let Ok(num) = size.parse::<u32>() {
                            if self.config.css.theme.spacing.contains(&num) {
                                return Some(format!(".h-{} {{ height: {}px; }}", num, num));
                            }
                        }
                    }
                }
            }
        }

        // Min/Max width
        if class.starts_with("min-w-") {
            if let Some(size) = class.strip_prefix("min-w-") {
                if size == "full" {
                    return Some(".min-w-full { min-width: 100%; }".to_string());
                }
            }
        }

        if class.starts_with("max-w-") {
            if let Some(size) = class.strip_prefix("max-w-") {
                match size {
                    "none" => return Some(".max-w-none { max-width: none; }".to_string()),
                    "full" => return Some(".max-w-full { max-width: 100%; }".to_string()),
                    "screen" => return Some(".max-w-screen { max-width: 100vw; }".to_string()),
                    "xs" => return Some(".max-w-xs { max-width: 320px; }".to_string()),
                    "sm" => return Some(".max-w-sm { max-width: 384px; }".to_string()),
                    "md" => return Some(".max-w-md { max-width: 448px; }".to_string()),
                    "lg" => return Some(".max-w-lg { max-width: 512px; }".to_string()),
                    "xl" => return Some(".max-w-xl { max-width: 576px; }".to_string()),
                    "2xl" => return Some(".max-w-2xl { max-width: 672px; }".to_string()),
                    "4xl" => return Some(".max-w-4xl { max-width: 896px; }".to_string()),
                    _ => {}
                }
            }
        }

        None
    }

    /// Parse position utilities: relative, absolute, fixed, sticky, top-*, left-*, etc.
    fn parse_position_utility(&self, class: &str) -> Option<String> {
        // Position type
        let css = match class {
            "static" => ".static { position: static; }",
            "relative" => ".relative { position: relative; }",
            "absolute" => ".absolute { position: absolute; }",
            "fixed" => ".fixed { position: fixed; }",
            "sticky" => ".sticky { position: sticky; }",
            _ => {
                // Position offsets: top-*, right-*, bottom-*, left-*
                for prefix in ["top", "right", "bottom", "left"] {
                    if class.starts_with(&format!("{}-", prefix)) {
                        if let Some(value) = class.strip_prefix(&format!("{}-", prefix)) {
                            match value {
                                "0" => return Some(format!(".{}-0 {{ {}: 0; }}", prefix, prefix)),
                                "auto" => return Some(format!(".{}-auto {{ {}: auto; }}", prefix, prefix)),
                                _ => {
                                    if let Ok(num) = value.parse::<u32>() {
                                        if self.config.css.theme.spacing.contains(&num) {
                                            return Some(format!(".{}-{} {{ {}: {}px; }}", prefix, num, prefix, num));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Inset utilities
                if class == "inset-0" {
                    return Some(".inset-0 { top: 0; right: 0; bottom: 0; left: 0; }".to_string());
                }

                return None;
            }
        };

        Some(css.to_string())
    }

    /// Parse effect utilities: shadow-*, opacity-*, blur-*, etc.
    fn parse_effect_utility(&self, class: &str) -> Option<String> {
        // Shadow utilities
        if class.starts_with("shadow") {
            return match class {
                "shadow" => Some(".shadow { box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1); }".to_string()),
                "shadow-sm" => Some(".shadow-sm { box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05); }".to_string()),
                "shadow-md" => Some(".shadow-md { box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1); }".to_string()),
                "shadow-lg" => Some(".shadow-lg { box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -4px rgba(0, 0, 0, 0.1); }".to_string()),
                "shadow-xl" => Some(".shadow-xl { box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1); }".to_string()),
                "shadow-2xl" => Some(".shadow-2xl { box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25); }".to_string()),
                "shadow-none" => Some(".shadow-none { box-shadow: none; }".to_string()),
                _ => None,
            };
        }

        // Opacity utilities
        if class.starts_with("opacity-") {
            if let Some(value) = class.strip_prefix("opacity-") {
                if let Ok(opacity) = value.parse::<u32>() {
                    if opacity <= 100 {
                        let opacity_decimal = opacity as f64 / 100.0;
                        return Some(format!(".opacity-{} {{ opacity: {}; }}", opacity, opacity_decimal));
                    }
                }
            }
        }

        // Cursor utilities
        match class {
            "cursor-auto" => return Some(".cursor-auto { cursor: auto; }".to_string()),
            "cursor-pointer" => return Some(".cursor-pointer { cursor: pointer; }".to_string()),
            "cursor-not-allowed" => return Some(".cursor-not-allowed { cursor: not-allowed; }".to_string()),
            "cursor-wait" => return Some(".cursor-wait { cursor: wait; }".to_string()),
            _ => {}
        }

        None
    }

    /// Parse display utilities: overflow-*, z-index-*, etc.
    fn parse_display_utility(&self, class: &str) -> Option<String> {
        // Overflow utilities
        if class.starts_with("overflow-") {
            if let Some(value) = class.strip_prefix("overflow-") {
                return match value {
                    "auto" => Some(".overflow-auto { overflow: auto; }".to_string()),
                    "hidden" => Some(".overflow-hidden { overflow: hidden; }".to_string()),
                    "visible" => Some(".overflow-visible { overflow: visible; }".to_string()),
                    "scroll" => Some(".overflow-scroll { overflow: scroll; }".to_string()),
                    "x-auto" => Some(".overflow-x-auto { overflow-x: auto; }".to_string()),
                    "x-hidden" => Some(".overflow-x-hidden { overflow-x: hidden; }".to_string()),
                    "y-auto" => Some(".overflow-y-auto { overflow-y: auto; }".to_string()),
                    "y-hidden" => Some(".overflow-y-hidden { overflow-y: hidden; }".to_string()),
                    _ => None,
                };
            }
        }

        // Z-index utilities
        if class.starts_with("z-") {
            if let Some(value) = class.strip_prefix("z-") {
                match value {
                    "0" => return Some(".z-0 { z-index: 0; }".to_string()),
                    "10" => return Some(".z-10 { z-index: 10; }".to_string()),
                    "20" => return Some(".z-20 { z-index: 20; }".to_string()),
                    "30" => return Some(".z-30 { z-index: 30; }".to_string()),
                    "40" => return Some(".z-40 { z-index: 40; }".to_string()),
                    "50" => return Some(".z-50 { z-index: 50; }".to_string()),
                    "auto" => return Some(".z-auto { z-index: auto; }".to_string()),
                    _ => {}
                }
            }
        }

        // Pointer events
        match class {
            "pointer-events-none" => return Some(".pointer-events-none { pointer-events: none; }".to_string()),
            "pointer-events-auto" => return Some(".pointer-events-auto { pointer-events: auto; }".to_string()),
            _ => {}
        }

        // Visibility
        match class {
            "visible" => return Some(".visible { visibility: visible; }".to_string()),
            "invisible" => return Some(".invisible { visibility: hidden; }".to_string()),
            _ => {}
        }

        None
    }

    /// Parse accessibility utilities
    fn parse_a11y_utility(&self, class: &str) -> Option<String> {
        match class {
            "sr-only" => Some(
                r#".sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}"#.to_string()
            ),
            "not-sr-only" => Some(
                r#".not-sr-only {
  position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  white-space: normal;
}"#.to_string()
            ),
            _ => None,
        }
    }

    /// Parse focus utilities: ring-*, outline-*, etc.
    fn parse_focus_utility(&self, class: &str) -> Option<String> {
        // Basic ring utility (no width specified, defaults to 3px)
        if class == "ring" {
            return Some(".ring { --tw-ring-offset-width: 0px; --tw-ring-color: rgba(59, 130, 246, 0.5); box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color); }".to_string());
        }

        // Ring width utilities
        if class.starts_with("ring-") && !class.contains("offset") {
            // Check if it's a color ring (ring-blue-500, ring-red-600, etc.)
            let color_pattern = Regex::new(r"^ring-([\w]+)-(\d+)$").ok()?;
            if let Some(caps) = color_pattern.captures(class) {
                let color_name = caps.get(1)?.as_str();
                let shade = caps.get(2)?.as_str().parse::<u32>().ok()?;

                // Look up color in theme
                let color = self.config.css.theme.colors
                    .iter()
                    .find(|c| c.name == color_name)?;

                let color_value = color.shades.get(&shade)?;

                return Some(format!(
                    ".ring-{}-{} {{ --tw-ring-color: {}; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color); }}",
                    color_name, shade, color_value
                ));
            }

            // Ring width utilities (ring-0, ring-1, ring-2, etc.)
            if let Some(width) = class.strip_prefix("ring-") {
                return match width {
                    "0" => Some(".ring-0 { box-shadow: none; }".to_string()),
                    "1" => Some(".ring-1 { --tw-ring-offset-width: 0px; --tw-ring-color: rgba(59, 130, 246, 0.5); box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color); }".to_string()),
                    "2" => Some(".ring-2 { --tw-ring-offset-width: 0px; --tw-ring-color: rgba(59, 130, 246, 0.5); box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color); }".to_string()),
                    "4" => Some(".ring-4 { --tw-ring-offset-width: 0px; --tw-ring-color: rgba(59, 130, 246, 0.5); box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color); }".to_string()),
                    "8" => Some(".ring-8 { --tw-ring-offset-width: 0px; --tw-ring-color: rgba(59, 130, 246, 0.5); box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color); }".to_string()),
                    _ => None,
                };
            }
        }

        // Ring offset utilities
        if class.starts_with("ring-offset-") {
            if let Some(width) = class.strip_prefix("ring-offset-") {
                return match width {
                    "0" => Some(".ring-offset-0 { --tw-ring-offset-width: 0px; }".to_string()),
                    "1" => Some(".ring-offset-1 { --tw-ring-offset-width: 1px; }".to_string()),
                    "2" => Some(".ring-offset-2 { --tw-ring-offset-width: 2px; }".to_string()),
                    "4" => Some(".ring-offset-4 { --tw-ring-offset-width: 4px; }".to_string()),
                    "8" => Some(".ring-offset-8 { --tw-ring-offset-width: 8px; }".to_string()),
                    _ => None,
                };
            }
        }

        // Outline utilities
        if class.starts_with("outline") {
            return match class {
                "outline-none" => Some(".outline-none { outline: 2px solid transparent; outline-offset: 2px; }".to_string()),
                "outline" => Some(".outline { outline-style: solid; }".to_string()),
                "outline-dashed" => Some(".outline-dashed { outline-style: dashed; }".to_string()),
                "outline-dotted" => Some(".outline-dotted { outline-style: dotted; }".to_string()),
                "outline-0" => Some(".outline-0 { outline-width: 0px; }".to_string()),
                "outline-1" => Some(".outline-1 { outline-width: 1px; }".to_string()),
                "outline-2" => Some(".outline-2 { outline-width: 2px; }".to_string()),
                "outline-4" => Some(".outline-4 { outline-width: 4px; }".to_string()),
                "outline-8" => Some(".outline-8 { outline-width: 8px; }".to_string()),
                _ => None,
            };
        }

        None
    }
}

/// Represents a variant applied to a utility class
#[derive(Debug, Clone)]
enum Variant {
    /// Responsive variant (e.g., "md:", "lg:")
    Responsive { breakpoint: String, min_width: String },
    /// State variant (e.g., "hover:", "focus:")
    State(String),
    /// Dark mode variant (e.g., "dark:")
    Dark,
    /// Print variant (e.g., "print:")
    Print,
}

/// Represents a parsed utility class
enum Utility {
    Color(ColorUtility),
    Spacing(SpacingUtility),
}

impl Utility {
    fn to_css(&self) -> String {
        match self {
            Utility::Color(c) => c.to_css(),
            Utility::Spacing(s) => s.to_css(),
        }
    }
}

/// Color utility (bg-*, text-*, border-*)
struct ColorUtility {
    class_name: String,
    property: String,    // "bg", "text", or "border"
    color: String,       // Color value
}

impl ColorUtility {
    fn to_css(&self) -> String {
        let css_property = match self.property.as_str() {
            "bg" => "background-color",
            "text" => "color",
            "border" => "border-color",
            _ => return String::new(),
        };

        format!(".{} {{ {}: {}; }}", self.class_name, css_property, self.color)
    }
}

/// Spacing utility (p-*, m-*, px-*, etc.)
struct SpacingUtility {
    class_name: String,
    property_type: String,  // "p" or "m"
    direction: Option<String>,  // "x", "y", "t", "r", "b", "l", or None for all
    size: String,           // Size value (e.g., "16px", "auto")
}

impl SpacingUtility {
    fn to_css(&self) -> String {
        let base_property = match self.property_type.as_str() {
            "p" => "padding",
            "m" => "margin",
            _ => return String::new(),
        };

        let properties = match self.direction.as_deref() {
            None => vec![base_property.to_string()],
            Some("x") => vec![format!("{}-left", base_property), format!("{}-right", base_property)],
            Some("y") => vec![format!("{}-top", base_property), format!("{}-bottom", base_property)],
            Some("t") => vec![format!("{}-top", base_property)],
            Some("r") => vec![format!("{}-right", base_property)],
            Some("b") => vec![format!("{}-bottom", base_property)],
            Some("l") => vec![format!("{}-left", base_property)],
            _ => return String::new(),
        };

        let css_declarations: Vec<String> = properties
            .iter()
            .map(|prop| format!("{}: {}", prop, self.size))
            .collect();

        format!(".{} {{ {}; }}", self.class_name, css_declarations.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_utility() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let utility = gen.parse_color_utility("bg-blue-500").unwrap();
        let css = utility.to_css();
        assert!(css.contains("background-color"));
        assert!(css.contains("#3b82f6"));
    }

    #[test]
    fn test_parse_spacing_utility() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let utility = gen.parse_spacing_utility("p-4").unwrap();
        let css = utility.to_css();
        assert!(css.contains("padding: 4px"));
    }

    #[test]
    fn test_parse_spacing_utility_directional() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let utility = gen.parse_spacing_utility("px-8").unwrap();
        let css = utility.to_css();
        assert!(css.contains("padding-left: 8px"));
        assert!(css.contains("padding-right: 8px"));
    }

    #[test]
    fn test_parse_layout_utility() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("flex").unwrap();
        assert!(css.contains("display: flex"));
    }

    #[test]
    fn test_grid_template_columns() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        // Test various column counts
        let css = gen.parse_layout_utility("grid-cols-1").unwrap();
        assert!(css.contains("grid-template-columns: repeat(1, 1fr)"));

        let css = gen.parse_layout_utility("grid-cols-5").unwrap();
        assert!(css.contains("grid-template-columns: repeat(5, 1fr)"));

        let css = gen.parse_layout_utility("grid-cols-12").unwrap();
        assert!(css.contains("grid-template-columns: repeat(12, 1fr)"));

        // Test special values
        let css = gen.parse_layout_utility("grid-cols-none").unwrap();
        assert!(css.contains("grid-template-columns: none"));

        let css = gen.parse_layout_utility("grid-cols-subgrid").unwrap();
        assert!(css.contains("grid-template-columns: subgrid"));
    }

    #[test]
    fn test_grid_template_rows() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("grid-rows-1").unwrap();
        assert!(css.contains("grid-template-rows: repeat(1, 1fr)"));

        let css = gen.parse_layout_utility("grid-rows-6").unwrap();
        assert!(css.contains("grid-template-rows: repeat(6, 1fr)"));

        let css = gen.parse_layout_utility("grid-rows-none").unwrap();
        assert!(css.contains("grid-template-rows: none"));
    }

    #[test]
    fn test_grid_auto_flow() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("grid-flow-row").unwrap();
        assert!(css.contains("grid-auto-flow: row"));

        let css = gen.parse_layout_utility("grid-flow-col").unwrap();
        assert!(css.contains("grid-auto-flow: column"));

        let css = gen.parse_layout_utility("grid-flow-dense").unwrap();
        assert!(css.contains("grid-auto-flow: dense"));

        let css = gen.parse_layout_utility("grid-flow-row-dense").unwrap();
        assert!(css.contains("grid-auto-flow: row dense"));
    }

    #[test]
    fn test_grid_auto_columns_rows() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        // Auto columns
        let css = gen.parse_layout_utility("auto-cols-auto").unwrap();
        assert!(css.contains("grid-auto-columns: auto"));

        let css = gen.parse_layout_utility("auto-cols-min").unwrap();
        assert!(css.contains("grid-auto-columns: min-content"));

        let css = gen.parse_layout_utility("auto-cols-fr").unwrap();
        assert!(css.contains("grid-auto-columns: minmax(0, 1fr)"));

        // Auto rows
        let css = gen.parse_layout_utility("auto-rows-auto").unwrap();
        assert!(css.contains("grid-auto-rows: auto"));

        let css = gen.parse_layout_utility("auto-rows-max").unwrap();
        assert!(css.contains("grid-auto-rows: max-content"));
    }

    #[test]
    fn test_gap_utilities() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        // Regular gap
        let css = gen.parse_layout_utility("gap-4").unwrap();
        assert!(css.contains("gap: 4px"));

        // Gap-x
        let css = gen.parse_layout_utility("gap-x-8").unwrap();
        assert!(css.contains("column-gap: 8px"));

        // Gap-y
        let css = gen.parse_layout_utility("gap-y-16").unwrap();
        assert!(css.contains("row-gap: 16px"));
    }

    #[test]
    fn test_grid_column_span() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("col-span-1").unwrap();
        assert!(css.contains("grid-column: span 1 / span 1"));

        let css = gen.parse_layout_utility("col-span-6").unwrap();
        assert!(css.contains("grid-column: span 6 / span 6"));

        let css = gen.parse_layout_utility("col-span-full").unwrap();
        assert!(css.contains("grid-column: 1 / -1"));

        let css = gen.parse_layout_utility("col-auto").unwrap();
        assert!(css.contains("grid-column: auto"));
    }

    #[test]
    fn test_grid_column_start_end() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("col-start-1").unwrap();
        assert!(css.contains("grid-column-start: 1"));

        let css = gen.parse_layout_utility("col-start-13").unwrap();
        assert!(css.contains("grid-column-start: 13"));

        let css = gen.parse_layout_utility("col-end-6").unwrap();
        assert!(css.contains("grid-column-end: 6"));

        let css = gen.parse_layout_utility("col-start-auto").unwrap();
        assert!(css.contains("grid-column-start: auto"));
    }

    #[test]
    fn test_grid_row_span() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("row-span-1").unwrap();
        assert!(css.contains("grid-row: span 1 / span 1"));

        let css = gen.parse_layout_utility("row-span-6").unwrap();
        assert!(css.contains("grid-row: span 6 / span 6"));

        let css = gen.parse_layout_utility("row-span-full").unwrap();
        assert!(css.contains("grid-row: 1 / -1"));

        let css = gen.parse_layout_utility("row-auto").unwrap();
        assert!(css.contains("grid-row: auto"));
    }

    #[test]
    fn test_grid_row_start_end() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_layout_utility("row-start-1").unwrap();
        assert!(css.contains("grid-row-start: 1"));

        let css = gen.parse_layout_utility("row-start-7").unwrap();
        assert!(css.contains("grid-row-start: 7"));

        let css = gen.parse_layout_utility("row-end-4").unwrap();
        assert!(css.contains("grid-row-end: 4"));

        let css = gen.parse_layout_utility("row-end-auto").unwrap();
        assert!(css.contains("grid-row-end: auto"));
    }

    #[test]
    fn test_parse_sizing_utility_width() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_sizing_utility("w-full").unwrap();
        assert!(css.contains("width: 100%"));
    }

    #[test]
    fn test_parse_sizing_utility_height() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_sizing_utility("h-screen").unwrap();
        assert!(css.contains("height: 100vh"));
    }

    #[test]
    fn test_parse_sizing_utility_max_width() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_sizing_utility("max-w-md").unwrap();
        assert!(css.contains("max-width: 448px"));
    }

    #[test]
    fn test_parse_position_utility() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_position_utility("fixed").unwrap();
        assert!(css.contains("position: fixed"));
    }

    #[test]
    fn test_parse_position_offset() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_position_utility("top-0").unwrap();
        assert!(css.contains("top: 0"));
    }

    #[test]
    fn test_parse_effect_shadow() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_effect_utility("shadow-lg").unwrap();
        assert!(css.contains("box-shadow"));
    }

    #[test]
    fn test_parse_effect_opacity() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_effect_utility("opacity-50").unwrap();
        assert!(css.contains("opacity: 0.5"));
    }

    #[test]
    fn test_parse_display_overflow() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_display_utility("overflow-hidden").unwrap();
        assert!(css.contains("overflow: hidden"));
    }

    #[test]
    fn test_parse_display_z_index() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.parse_display_utility("z-50").unwrap();
        assert!(css.contains("z-index: 50"));
    }

    #[test]
    fn test_parse_variants_responsive() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let (variants, base_class) = gen.parse_variants("md:p-4");
        assert_eq!(base_class, "p-4");
        assert_eq!(variants.len(), 1);
        match &variants[0] {
            Variant::Responsive { breakpoint, min_width } => {
                assert_eq!(breakpoint, "md");
                assert_eq!(min_width, "768px");
            }
            _ => panic!("Expected responsive variant"),
        }
    }

    #[test]
    fn test_parse_variants_state() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let (variants, base_class) = gen.parse_variants("hover:bg-blue-500");
        assert_eq!(base_class, "bg-blue-500");
        assert_eq!(variants.len(), 1);
        match &variants[0] {
            Variant::State(state) => assert_eq!(state, "hover"),
            _ => panic!("Expected state variant"),
        }
    }

    #[test]
    fn test_parse_variants_chained() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let (variants, base_class) = gen.parse_variants("md:hover:bg-blue-600");
        assert_eq!(base_class, "bg-blue-600");
        assert_eq!(variants.len(), 2);

        // First should be responsive
        match &variants[0] {
            Variant::Responsive { breakpoint, .. } => assert_eq!(breakpoint, "md"),
            _ => panic!("Expected responsive variant first"),
        }

        // Second should be state
        match &variants[1] {
            Variant::State(state) => assert_eq!(state, "hover"),
            _ => panic!("Expected state variant second"),
        }
    }

    #[test]
    fn test_escape_css_class() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        assert_eq!(gen.escape_css_class("md:hover:p-4"), "md\\:hover\\:p-4");
        assert_eq!(gen.escape_css_class("w-1/2"), "w-1\\/2");
        assert_eq!(gen.escape_css_class("bg-[#fff]"), "bg-\\[#fff\\]");
    }

    #[test]
    fn test_wrap_with_responsive_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let base_css = ".p-4 { padding: 4px; }";
        let variants = vec![Variant::Responsive {
            breakpoint: "md".to_string(),
            min_width: "768px".to_string(),
        }];

        let css = gen.wrap_with_variants(base_css, "md:p-4", &variants);
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("md\\:p-4"));
        assert!(css.contains("padding: 4px"));
    }

    #[test]
    fn test_wrap_with_state_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let base_css = ".bg-blue-500 { background-color: #3b82f6; }";
        let variants = vec![Variant::State("hover".to_string())];

        let css = gen.wrap_with_variants(base_css, "hover:bg-blue-500", &variants);
        assert!(css.contains("hover\\:bg-blue-500:hover"));
        assert!(css.contains("background-color: #3b82f6"));
    }

    #[test]
    fn test_wrap_with_chained_variants() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let base_css = ".bg-blue-600 { background-color: #2563eb; }";
        let variants = vec![
            Variant::Responsive {
                breakpoint: "md".to_string(),
                min_width: "768px".to_string(),
            },
            Variant::State("hover".to_string()),
        ];

        let css = gen.wrap_with_variants(base_css, "md:hover:bg-blue-600", &variants);
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("md\\:hover\\:bg-blue-600:hover"));
        assert!(css.contains("background-color: #2563eb"));
    }

    #[test]
    fn test_generate_utility_with_responsive_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("lg:p-8").unwrap();
        assert!(css.contains("@media (min-width: 1024px)"));
        assert!(css.contains("lg\\:p-8"));
        assert!(css.contains("padding: 8px"));
    }

    #[test]
    fn test_generate_utility_with_state_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("hover:text-white").unwrap();
        assert!(css.contains("hover\\:text-white:hover"));
        assert!(css.contains("color: white"));
    }

    #[test]
    fn test_custom_utility_basic() {
        use std::collections::HashMap;

        let mut custom_utilities = HashMap::new();
        custom_utilities.insert(
            "btn-primary".to_string(),
            "background: #4f46e5;\n    color: white;\n    padding: 12px 24px;\n    border-radius: 6px;".to_string()
        );

        let mut config = UtilityConfig::default();
        config.css.utilities_custom = Some(custom_utilities);

        let gen = UtilityGenerator::new(config);
        let css = gen.generate_utility("btn-primary").unwrap();

        assert!(css.contains("btn-primary"));
        assert!(css.contains("background: #4f46e5"));
        assert!(css.contains("color: white"));
        assert!(css.contains("padding: 12px 24px"));
        assert!(css.contains("border-radius: 6px"));
    }

    #[test]
    fn test_custom_utility_with_responsive_variant() {
        use std::collections::HashMap;

        let mut custom_utilities = HashMap::new();
        custom_utilities.insert(
            "card-elevated".to_string(),
            "box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);\n    background: white;".to_string()
        );

        let mut config = UtilityConfig::default();
        config.css.utilities_custom = Some(custom_utilities);

        let gen = UtilityGenerator::new(config);
        let css = gen.generate_utility("md:card-elevated").unwrap();

        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("md\\:card-elevated"));
        assert!(css.contains("box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1)"));
        assert!(css.contains("background: white"));
    }

    #[test]
    fn test_custom_utility_with_state_variant() {
        use std::collections::HashMap;

        let mut custom_utilities = HashMap::new();
        custom_utilities.insert(
            "btn-secondary".to_string(),
            "background: #6b7280;\n    color: white;\n    padding: 8px 16px;".to_string()
        );

        let mut config = UtilityConfig::default();
        config.css.utilities_custom = Some(custom_utilities);

        let gen = UtilityGenerator::new(config);
        let css = gen.generate_utility("hover:btn-secondary").unwrap();

        assert!(css.contains("hover\\:btn-secondary:hover"));
        assert!(css.contains("background: #6b7280"));
        assert!(css.contains("color: white"));
        assert!(css.contains("padding: 8px 16px"));
    }

    #[test]
    fn test_custom_utility_with_chained_variants() {
        use std::collections::HashMap;

        let mut custom_utilities = HashMap::new();
        custom_utilities.insert(
            "hero-text".to_string(),
            "font-size: 3rem;\n    font-weight: bold;\n    line-height: 1.2;".to_string()
        );

        let mut config = UtilityConfig::default();
        config.css.utilities_custom = Some(custom_utilities);

        let gen = UtilityGenerator::new(config);
        let css = gen.generate_utility("lg:hover:hero-text").unwrap();

        assert!(css.contains("@media (min-width: 1024px)"));
        assert!(css.contains("lg\\:hover\\:hero-text:hover"));
        assert!(css.contains("font-size: 3rem"));
        assert!(css.contains("font-weight: bold"));
        assert!(css.contains("line-height: 1.2"));
    }

    #[test]
    fn test_custom_utility_overrides_standard_utility() {
        use std::collections::HashMap;

        // Custom utility that overrides standard "flex" utility
        let mut custom_utilities = HashMap::new();
        custom_utilities.insert(
            "flex".to_string(),
            "display: flex;\n    align-items: center;\n    justify-content: center;".to_string()
        );

        let mut config = UtilityConfig::default();
        config.css.utilities_custom = Some(custom_utilities);

        let gen = UtilityGenerator::new(config);
        let css = gen.generate_utility("flex").unwrap();

        // Should use custom utility, not standard one
        assert!(css.contains("display: flex"));
        assert!(css.contains("align-items: center"));
        assert!(css.contains("justify-content: center"));
    }

    #[test]
    fn test_generate_theme_variables() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_theme_variables();

        // Check :root block
        assert!(css.contains(":root {"));

        // Check color variables
        assert!(css.contains("--color-blue-500: #3b82f6;"));
        assert!(css.contains("--color-gray-500: #6b7280;"));

        // Check spacing variables
        assert!(css.contains("--spacing-4: 4px;"));
        assert!(css.contains("--spacing-8: 8px;"));

        // Check font size variables
        assert!(css.contains("--font-size-base:"));
        assert!(css.contains("--line-height-base:"));

        // Check border radius variables
        assert!(css.contains("--radius-"));

        // Check breakpoint variables
        assert!(css.contains("--breakpoint-sm:"));
        assert!(css.contains("--breakpoint-md:"));
    }

    #[test]
    fn test_generate_theme_variables_minified() {
        let mut config = UtilityConfig::default();
        config.css.minify = true;

        let gen = UtilityGenerator::new(config);
        let css = gen.generate_theme_variables();

        // Check minified format (no newlines except at end)
        assert!(css.starts_with(":root{"));
        assert!(css.contains("--color-blue-500:#3b82f6;"));
        assert!(!css.contains("\n  ")); // No indentation
    }

    #[test]
    fn test_generate_css_with_theme_mode() {
        let mut config = UtilityConfig::default();
        config.css.theme_mode = Some(true);

        let mut gen = UtilityGenerator::new(config);

        // Add a utility to generate
        gen.used_utilities.insert("p-4".to_string());

        let css = gen.generate_css();

        // Should include :root block
        assert!(css.contains(":root {"));
        assert!(css.contains("--color-blue-500:"));

        // Should also include utility class
        assert!(css.contains(".p-4"));
    }

    #[test]
    fn test_generate_css_without_theme_mode() {
        let mut config = UtilityConfig::default();
        config.css.theme_mode = None; // Default is disabled

        let mut gen = UtilityGenerator::new(config);

        // Add a utility to generate
        gen.used_utilities.insert("p-4".to_string());

        let css = gen.generate_css();

        // Should NOT include :root block
        assert!(!css.contains(":root {"));
        assert!(!css.contains("--color-blue-500:"));

        // Should still include utility class
        assert!(css.contains(".p-4"));
    }

    #[test]
    fn test_parse_dark_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let (variants, base_class) = gen.parse_variants("dark:bg-gray-900");

        assert_eq!(variants.len(), 1);
        assert!(matches!(variants[0], Variant::Dark));
        assert_eq!(base_class, "bg-gray-900");
    }

    #[test]
    fn test_generate_dark_mode_utility() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("dark:bg-gray-900").unwrap();

        // Should include .dark parent selector
        assert!(css.contains(".dark .dark\\:bg-gray-900"));
        assert!(css.contains("background-color: #111827"));
    }

    #[test]
    fn test_dark_variant_with_state() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("dark:hover:bg-blue-600").unwrap();

        // Should include both dark and hover
        assert!(css.contains(".dark .dark\\:hover\\:bg-blue-600:hover"));
        assert!(css.contains("background-color: #2563eb"));
    }

    #[test]
    fn test_dark_variant_with_responsive() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("md:dark:text-white").unwrap();

        // Should include both media query and .dark parent
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains(".dark .md\\:dark\\:text-white"));
        assert!(css.contains("color: white"));
    }

    #[test]
    fn test_dark_variant_chained() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("lg:dark:hover:opacity-80").unwrap();

        // Should include media query, dark parent, and hover state
        assert!(css.contains("@media (min-width: 1024px)"));
        assert!(css.contains(".dark .lg\\:dark\\:hover\\:opacity-80:hover"));
        assert!(css.contains("opacity: 0.8"));
    }

    #[test]
    fn test_debug_dark_mode_simple() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("dark:bg-gray-900").unwrap();
        println!("Generated CSS:\n{}", css);

        assert!(css.contains(".dark .dark\\:bg-gray-900"));
    }

    #[test]
    fn test_opacity_modifier_basic() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("bg-blue-500/50").unwrap();

        // Should generate rgba with 50% opacity
        assert!(css.contains("rgba(59, 130, 246, 0.5)"));
    }

    #[test]
    fn test_opacity_modifier_text_color() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("text-red-500/75").unwrap();

        // Should generate rgba with 75% opacity
        assert!(css.contains("rgba(239, 68, 68, 0.75)"));
    }

    #[test]
    fn test_opacity_modifier_white() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("bg-white/25").unwrap();

        // Should generate rgba for white with 25% opacity
        assert!(css.contains("rgba(255, 255, 255, 0.25)"));
    }

    #[test]
    fn test_opacity_modifier_with_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("hover:bg-blue-500/80").unwrap();

        // Should work with hover variant
        assert!(css.contains("hover\\:bg-blue-500\\/80:hover"));
        assert!(css.contains("rgba(59, 130, 246, 0.8)"));
    }

    #[test]
    fn test_opacity_100_unchanged() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("bg-blue-500/100").unwrap();

        // 100% opacity should still use rgba format
        assert!(css.contains("rgba(59, 130, 246, 1)"));
    }

    #[test]
    fn test_arbitrary_hex_color() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("bg-[#1da1f2]").unwrap();

        // Should use the exact hex color
        assert!(css.contains("background-color: #1da1f2"));
    }

    #[test]
    fn test_arbitrary_rgb_color() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("text-[rgb(255,0,0)]").unwrap();

        // Should use rgb format
        assert!(css.contains("color: rgb(255,0,0)"));
    }

    #[test]
    fn test_arbitrary_rgba_color() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("border-[rgba(0,128,255,0.5)]").unwrap();

        // Should use rgba format
        assert!(css.contains("border-color: rgba(0,128,255,0.5)"));
    }

    #[test]
    fn test_arbitrary_short_hex() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("bg-[#fff]").unwrap();

        // Should accept short hex format
        assert!(css.contains("background-color: #fff"));
    }

    #[test]
    fn test_arbitrary_with_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("hover:bg-[#ff6b6b]").unwrap();
        println!("Generated CSS:\n{}", css);

        // Should work with variants
        assert!(css.contains(":hover"));
        assert!(css.contains("background-color: #ff6b6b"));
    }

    #[test]
    fn test_arbitrary_invalid_rejected() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        // Invalid color format should return None
        let css = gen.generate_utility("bg-[invalid]");
        assert!(css.is_none());
    }

    #[test]
    fn test_print_variant_basic() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("print:hidden").unwrap();

        // Should wrap in @media print
        assert!(css.contains("@media print"));
        assert!(css.contains("display: none"));
    }

    #[test]
    fn test_print_variant_color() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("print:text-black").unwrap();

        // Should wrap in @media print
        assert!(css.contains("@media print"));
        assert!(css.contains("color: black"));
    }

    #[test]
    fn test_print_with_responsive() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("md:print:p-4").unwrap();

        // Should have both responsive and print media queries
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("@media print"));
    }

    #[test]
    fn test_print_only_utility() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("print:block").unwrap();

        // Should show element only when printing
        assert!(css.contains("@media print"));
        assert!(css.contains("display: block"));
    }

    #[test]
    fn test_sr_only_basic() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("sr-only").unwrap();

        // Should have all screen reader-only properties
        assert!(css.contains(".sr-only"));
        assert!(css.contains("position: absolute"));
        assert!(css.contains("width: 1px"));
        assert!(css.contains("height: 1px"));
        assert!(css.contains("overflow: hidden"));
        assert!(css.contains("clip: rect(0, 0, 0, 0)"));
    }

    #[test]
    fn test_not_sr_only() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("not-sr-only").unwrap();

        // Should reverse sr-only styles
        assert!(css.contains(".not-sr-only"));
        assert!(css.contains("position: static"));
        assert!(css.contains("width: auto"));
        assert!(css.contains("height: auto"));
        assert!(css.contains("overflow: visible"));
    }

    #[test]
    fn test_sr_only_with_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("md:sr-only").unwrap();

        // Should wrap in media query
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("position: absolute"));
    }

    #[test]
    fn test_ring_basic() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("ring").unwrap();

        // Should have ring styles
        assert!(css.contains(".ring"));
        assert!(css.contains("box-shadow"));
        assert!(css.contains("--tw-ring-color"));
    }

    #[test]
    fn test_ring_width() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("ring-2").unwrap();

        // Should have 2px ring
        assert!(css.contains(".ring-2"));
        assert!(css.contains("calc(2px + var(--tw-ring-offset-width))"));
    }

    #[test]
    fn test_ring_color() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("ring-blue-500").unwrap();

        // Should have blue ring color
        assert!(css.contains(".ring-blue-500"));
        assert!(css.contains("--tw-ring-color: #3b82f6"));
    }

    #[test]
    fn test_ring_offset() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("ring-offset-2").unwrap();

        // Should set offset width
        assert!(css.contains(".ring-offset-2"));
        assert!(css.contains("--tw-ring-offset-width: 2px"));
    }

    #[test]
    fn test_outline_none() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("outline-none").unwrap();

        // Should remove outline
        assert!(css.contains(".outline-none"));
        assert!(css.contains("outline: 2px solid transparent"));
    }

    #[test]
    fn test_outline_width() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("outline-2").unwrap();

        // Should have 2px outline
        assert!(css.contains(".outline-2"));
        assert!(css.contains("outline-width: 2px"));
    }

    #[test]
    fn test_focus_ring_variant() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("focus:ring-2").unwrap();

        // Should have focus pseudo-class
        assert!(css.contains(":focus"));
        assert!(css.contains("ring-2"));
    }

    #[test]
    fn test_focus_outline_none() {
        let config = UtilityConfig::default();
        let gen = UtilityGenerator::new(config);

        let css = gen.generate_utility("focus:outline-none").unwrap();

        // Should have focus pseudo-class with outline removal
        assert!(css.contains(":focus"));
        assert!(css.contains("outline"));
    }
}
