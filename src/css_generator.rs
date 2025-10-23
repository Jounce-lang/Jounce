// CSS Generator - Phase 7.5 Sprint 1 Day 2
// Generates scoped CSS from CssExpression AST nodes

use crate::ast::*;
use std::collections::HashMap;

/// CSS Generator - converts CSS AST to scoped CSS strings
pub struct CssGenerator {
    /// Component name for scoping (e.g., "Button", "App")
    component_name: String,
    /// Generated CSS output
    css_output: String,
    /// Map of original class names to scoped class names
    /// e.g., "button" -> "Button_button_a3f5c9"
    class_map: HashMap<String, String>,
}

impl CssGenerator {
    /// Create a new CSS generator for a component
    pub fn new(component_name: String) -> Self {
        Self {
            component_name,
            css_output: String::new(),
            class_map: HashMap::new(),
        }
    }

    /// Generate CSS from a CssExpression
    pub fn generate(&mut self, css_expr: &CssExpression) -> String {
        for rule in &css_expr.rules {
            self.generate_rule(rule);
        }
        self.css_output.clone()
    }

    /// Generate CSS for a single rule
    fn generate_rule(&mut self, rule: &CssRule) {
        let scoped_selector = self.generate_scoped_selector(&rule.selector);

        // Start rule
        self.css_output.push_str(&scoped_selector);
        self.css_output.push_str(" {\n");

        // Generate declarations
        for decl in &rule.declarations {
            self.generate_declaration(decl);
        }

        // Close rule
        self.css_output.push_str("}\n\n");
    }

    /// Generate a scoped selector from CssSelector
    fn generate_scoped_selector(&mut self, selector: &CssSelector) -> String {
        match selector {
            CssSelector::Class(class_name) => {
                // Generate scoped class name: .button -> .Button_button_a3f5c9
                let scoped_name = self.generate_scoped_class_name(class_name);
                format!(".{}", scoped_name)
            }
            CssSelector::Id(id_name) => {
                // IDs are not scoped (unique by definition)
                format!("#{}", id_name)
            }
            CssSelector::Element(element) => {
                // Element selectors are not scoped
                element.clone()
            }
            CssSelector::Pseudo(pseudo) => {
                // Pseudo-selectors are not scoped
                // Note: These should typically be combined with other selectors
                // For now, just output as-is
                format!(":{}", pseudo)
            }
            CssSelector::Nested(_nested) => {
                // Nested selectors (Sprint 2)
                // For now, just return a placeholder
                String::from("/* nested selector - Sprint 2 */")
            }
            CssSelector::Compound(_selectors) => {
                // Compound selectors (Sprint 2)
                // For now, just return a placeholder
                String::from("/* compound selector - Sprint 2 */")
            }
        }
    }

    /// Generate a scoped class name using hash-based approach (like CSS Modules)
    /// Format: {ComponentName}_{className}_{hash}
    /// Example: "button" -> "Button_button_a3f5c9"
    fn generate_scoped_class_name(&mut self, class_name: &str) -> String {
        // Check if we've already generated this class name
        if let Some(scoped) = self.class_map.get(class_name) {
            return scoped.clone();
        }

        // Generate hash from component name + class name
        let hash = self.generate_hash(&format!("{}{}", self.component_name, class_name));
        let scoped_name = format!("{}_{}_{}",
            self.component_name,
            class_name,
            &hash[0..6]  // Use first 6 chars of hash
        );

        // Store in map for reuse
        self.class_map.insert(class_name.to_string(), scoped_name.clone());

        scoped_name
    }

    /// Generate a simple hash from a string
    /// Uses a basic hash algorithm for Sprint 1
    /// Can be improved in Sprint 2 with proper crypto hash
    fn generate_hash(&self, input: &str) -> String {
        // Simple hash: sum of byte values mod some large prime
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:x}", hash)
    }

    /// Generate a CSS declaration (property: value;)
    fn generate_declaration(&mut self, decl: &CssDeclaration) {
        self.css_output.push_str("  "); // Indent
        self.css_output.push_str(&decl.property);
        self.css_output.push_str(": ");
        self.css_output.push_str(&self.generate_value(&decl.value));
        self.css_output.push_str(";\n");
    }

    /// Generate CSS value from CssValue enum
    fn generate_value(&self, value: &CssValue) -> String {
        match value {
            CssValue::Color(color) => color.clone(),
            CssValue::Length(num, unit) => format!("{}{}", num, unit),
            CssValue::String(s) => format!("\"{}\"", s),
            CssValue::Number(n) => n.to_string(),
            CssValue::Keyword(kw) => kw.clone(),
            CssValue::Function(name, args) => {
                let arg_strs: Vec<String> = args.iter()
                    .map(|arg| self.generate_value(arg))
                    .collect();
                format!("{}({})", name, arg_strs.join(", "))
            }
            CssValue::Raw(raw) => raw.clone(),
        }
    }

    /// Get the class name mapping (for JavaScript code generation)
    pub fn get_class_map(&self) -> &HashMap<String, String> {
        &self.class_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_scoped_class_name() {
        let mut gen = CssGenerator::new("Button".to_string());
        let scoped = gen.generate_scoped_class_name("primary");

        // Should be in format: Button_primary_{hash}
        assert!(scoped.starts_with("Button_primary_"));
        assert_eq!(scoped.split('_').count(), 3);
    }

    #[test]
    fn test_generate_scoped_class_name_consistency() {
        let mut gen = CssGenerator::new("Button".to_string());
        let scoped1 = gen.generate_scoped_class_name("primary");
        let scoped2 = gen.generate_scoped_class_name("primary");

        // Should return same scoped name for same class
        assert_eq!(scoped1, scoped2);
    }

    #[test]
    fn test_generate_simple_rule() {
        let mut gen = CssGenerator::new("Button".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("button".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "color".to_string(),
                    value: CssValue::Raw("blue".to_string()),
                },
            ],
            nested_rules: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should contain scoped class name
        assert!(output.contains("Button_button_"));
        // Should contain declaration
        assert!(output.contains("color: blue;"));
    }

    #[test]
    fn test_generate_multiple_rules() {
        let mut gen = CssGenerator::new("App".to_string());

        let css_expr = CssExpression {
            rules: vec![
                CssRule {
                    selector: CssSelector::Class("header".to_string()),
                    declarations: vec![
                        CssDeclaration {
                            property: "font-size".to_string(),
                            value: CssValue::Raw("24px".to_string()),
                        },
                    ],
                    nested_rules: vec![],
                },
                CssRule {
                    selector: CssSelector::Class("footer".to_string()),
                    declarations: vec![
                        CssDeclaration {
                            property: "color".to_string(),
                            value: CssValue::Raw("gray".to_string()),
                        },
                    ],
                    nested_rules: vec![],
                },
            ],
        };

        let output = gen.generate(&css_expr);

        // Should contain both rules
        assert!(output.contains("App_header_"));
        assert!(output.contains("App_footer_"));
        assert!(output.contains("font-size: 24px;"));
        assert!(output.contains("color: gray;"));
    }
}
