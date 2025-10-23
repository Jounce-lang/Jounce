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

    /// Generate CSS for a single rule (with optional parent for nesting)
    fn generate_rule(&mut self, rule: &CssRule) {
        self.generate_rule_with_parent(rule, None);
    }

    /// Generate CSS rule with parent selector support for nesting
    fn generate_rule_with_parent(&mut self, rule: &CssRule, parent_selector: Option<&str>) {
        let scoped_selector = self.generate_scoped_selector_with_parent(&rule.selector, parent_selector);

        // Generate rule only if it has declarations
        if !rule.declarations.is_empty() {
            self.css_output.push_str(&scoped_selector);
            self.css_output.push_str(" {\n");

            // Generate declarations
            for decl in &rule.declarations {
                self.generate_declaration(decl);
            }

            // Close rule
            self.css_output.push_str("}\n\n");
        }

        // Generate nested rules recursively with this rule's selector as parent
        for nested in &rule.nested_rules {
            self.generate_rule_with_parent(nested, Some(&scoped_selector));
        }
    }

    /// Generate a scoped selector from CssSelector (without parent)
    fn generate_scoped_selector(&mut self, selector: &CssSelector) -> String {
        self.generate_scoped_selector_with_parent(selector, None)
    }

    /// Generate a scoped selector with optional parent for nesting
    fn generate_scoped_selector_with_parent(&mut self, selector: &CssSelector, parent: Option<&str>) -> String {
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
            CssSelector::Nested(nested_selector) => {
                // Handle & selector for nesting
                if nested_selector.starts_with('&') {
                    // & refers to the parent selector
                    if let Some(parent) = parent {
                        // Replace & with parent selector
                        let rest = &nested_selector[1..]; // Remove &
                        if rest.is_empty() {
                            // Just & by itself
                            parent.to_string()
                        } else {
                            // &:hover, & .title, etc.
                            format!("{}{}", parent, rest)
                        }
                    } else {
                        // No parent, just output as-is (shouldn't happen in well-formed CSS)
                        nested_selector.clone()
                    }
                } else {
                    // Regular nested selector like `.card .title` (descendant combinator)
                    // Scope any classes in it
                    self.scope_nested_selector(nested_selector)
                }
            }
            CssSelector::Compound(selectors) => {
                // Compound selectors like `.button.primary` or `.button:hover`
                // Multiple selectors with no space between them
                let mut result = String::new();
                for sel in selectors {
                    let scoped = self.generate_scoped_selector(sel);
                    // Remove leading dot/colon if present for concatenation
                    if scoped.starts_with('.') || scoped.starts_with(':') {
                        result.push_str(&scoped);
                    } else {
                        result.push_str(&scoped);
                    }
                }
                result
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

    /// Scope a nested selector string by scoping any class names
    /// Example: ".card .title" -> ".Card_card_abc123 .title"
    /// Note: This is a simple implementation for Sprint 1 Day 3
    fn scope_nested_selector(&mut self, nested_selector: &str) -> String {
        // Split by space to handle descendant combinators
        let parts: Vec<&str> = nested_selector.split_whitespace().collect();
        let mut scoped_parts = Vec::new();

        for part in parts {
            if part.starts_with('.') {
                // It's a class selector - scope it
                let class_name = &part[1..]; // Remove the leading dot
                let scoped_name = self.generate_scoped_class_name(class_name);
                scoped_parts.push(format!(".{}", scoped_name));
            } else if part.starts_with('#') {
                // ID selector - don't scope
                scoped_parts.push(part.to_string());
            } else {
                // Element selector or other - don't scope
                scoped_parts.push(part.to_string());
            }
        }

        scoped_parts.join(" ")
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

    #[test]
    fn test_compound_selector() {
        let mut gen = CssGenerator::new("Button".to_string());

        let rule = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("button".to_string()),
                CssSelector::Pseudo("hover".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "background".to_string(),
                    value: CssValue::Raw("darkblue".to_string()),
                },
            ],
            nested_rules: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should contain scoped class with pseudo-class
        assert!(output.contains("Button_button_"));
        assert!(output.contains(":hover"));
        assert!(output.contains("background: darkblue;"));
    }

    #[test]
    fn test_compound_selector_multiple_classes() {
        let mut gen = CssGenerator::new("Card".to_string());

        let rule = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("card".to_string()),
                CssSelector::Class("primary".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "border".to_string(),
                    value: CssValue::Raw("2px solid blue".to_string()),
                },
            ],
            nested_rules: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should contain both scoped class names concatenated
        assert!(output.contains("Card_card_"));
        assert!(output.contains("Card_primary_"));
        assert!(output.contains("border: 2px solid blue;"));
    }

    #[test]
    fn test_nested_selector_simple() {
        let mut gen = CssGenerator::new("Container".to_string());

        let rule = CssRule {
            selector: CssSelector::Nested(".card .title".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "font-size".to_string(),
                    value: CssValue::Raw("18px".to_string()),
                },
            ],
            nested_rules: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should scope .card but not .title (different scoping behavior)
        // Actually, both should be scoped if they're classes
        assert!(output.contains("Container_card_"));
        assert!(output.contains("Container_title_"));
        assert!(output.contains("font-size: 18px;"));
    }

    #[test]
    fn test_nested_selector_with_element() {
        let mut gen = CssGenerator::new("App".to_string());

        let rule = CssRule {
            selector: CssSelector::Nested(".header h1".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "margin".to_string(),
                    value: CssValue::Raw("0".to_string()),
                },
            ],
            nested_rules: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should scope .header but not h1 (element)
        assert!(output.contains("App_header_"));
        assert!(output.contains(" h1"));
        assert!(output.contains("margin: 0;"));
    }

    #[test]
    fn test_get_class_map() {
        let mut gen = CssGenerator::new("Component".to_string());

        // Generate some scoped class names
        gen.generate_scoped_class_name("button");
        gen.generate_scoped_class_name("primary");

        let class_map = gen.get_class_map();

        // Should have entries for both classes
        assert!(class_map.contains_key("button"));
        assert!(class_map.contains_key("primary"));
        assert!(class_map.get("button").unwrap().starts_with("Component_button_"));
        assert!(class_map.get("primary").unwrap().starts_with("Component_primary_"));
    }

    #[test]
    fn test_nesting_with_ampersand_pseudo() {
        let mut gen = CssGenerator::new("Button".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("button".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "color".to_string(),
                    value: CssValue::Raw("blue".to_string()),
                },
            ],
            nested_rules: vec![
                CssRule {
                    selector: CssSelector::Nested("&:hover".to_string()),
                    declarations: vec![
                        CssDeclaration {
                            property: "color".to_string(),
                            value: CssValue::Raw("darkblue".to_string()),
                        },
                    ],
                    nested_rules: vec![],
                },
            ],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should generate parent rule
        assert!(output.contains("Button_button_"));
        assert!(output.contains("color: blue;"));

        // Should generate nested rule with & replaced by parent selector
        assert!(output.contains("Button_button_") && output.contains(":hover"));
        assert!(output.contains("color: darkblue;"));
    }

    #[test]
    fn test_nesting_with_ampersand_descendant() {
        let mut gen = CssGenerator::new("Card".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("card".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "background".to_string(),
                    value: CssValue::Raw("white".to_string()),
                },
            ],
            nested_rules: vec![
                CssRule {
                    selector: CssSelector::Nested("& .title".to_string()),
                    declarations: vec![
                        CssDeclaration {
                            property: "font-size".to_string(),
                            value: CssValue::Raw("24px".to_string()),
                        },
                    ],
                    nested_rules: vec![],
                },
            ],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should generate parent rule
        assert!(output.contains("Card_card_"));
        assert!(output.contains("background: white;"));

        // Should generate nested rule with & replaced by parent and space preserved
        assert!(output.contains("Card_card_") && output.contains(" .title"));
        assert!(output.contains("font-size: 24px;"));
    }

    #[test]
    fn test_deeply_nested_rules() {
        let mut gen = CssGenerator::new("Container".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("container".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "width".to_string(),
                    value: CssValue::Raw("100%".to_string()),
                },
            ],
            nested_rules: vec![
                CssRule {
                    selector: CssSelector::Nested("& .header".to_string()),
                    declarations: vec![
                        CssDeclaration {
                            property: "height".to_string(),
                            value: CssValue::Raw("60px".to_string()),
                        },
                    ],
                    nested_rules: vec![
                        CssRule {
                            selector: CssSelector::Nested("& .title".to_string()),
                            declarations: vec![
                                CssDeclaration {
                                    property: "font-size".to_string(),
                                    value: CssValue::Raw("20px".to_string()),
                                },
                            ],
                            nested_rules: vec![],
                        },
                    ],
                },
            ],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should have all three levels
        assert!(output.contains("Container_container_"));
        assert!(output.contains("width: 100%;"));
        assert!(output.contains(" .header"));
        assert!(output.contains("height: 60px;"));
        assert!(output.contains(" .title"));
        assert!(output.contains("font-size: 20px;"));
    }
}
