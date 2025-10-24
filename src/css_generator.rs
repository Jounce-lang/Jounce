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
    /// Map of original keyframe names to scoped keyframe names
    /// e.g., "fadeIn" -> "Button_fadeIn_a3f5c9"
    /// Sprint 2 Task 2.6
    keyframes_map: HashMap<String, String>,
    /// Dynamic CSS declarations that need to become inline styles
    /// Maps class name to vec of (property, expression) pairs
    /// e.g., "button" -> [("background", props.color), ("opacity", props.disabled ? 0.5 : 1.0)]
    /// Sprint 2 Task 2.4
    dynamic_declarations: HashMap<String, Vec<(String, Expression)>>,
}

impl CssGenerator {
    /// Create a new CSS generator for a component
    pub fn new(component_name: String) -> Self {
        Self {
            component_name,
            css_output: String::new(),
            class_map: HashMap::new(),
            keyframes_map: HashMap::new(),
            dynamic_declarations: HashMap::new(),
        }
    }

    /// Generate CSS from a CssExpression
    pub fn generate(&mut self, css_expr: &CssExpression) -> String {
        // Generate CSS rules
        for rule in &css_expr.rules {
            self.generate_rule(rule);
        }

        // Generate keyframes (Sprint 2 Task 2.6)
        for keyframes in &css_expr.keyframes {
            self.generate_keyframes(keyframes);
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
            // Extract class name if selector is a class (for dynamic declarations)
            let class_name = match &rule.selector {
                CssSelector::Class(name) => Some(name.clone()),
                _ => None,
            };

            for decl in &rule.declarations {
                self.generate_declaration(decl, class_name.as_deref());
            }

            // Close rule
            self.css_output.push_str("}\n\n");
        }

        // Generate nested rules recursively with this rule's selector as parent
        for nested in &rule.nested_rules {
            self.generate_rule_with_parent(nested, Some(&scoped_selector));
        }

        // Generate media queries for this rule
        for media_query in &rule.media_queries {
            self.generate_media_query(media_query, &scoped_selector);
        }

        // Generate container queries for this rule (Phase 8)
        for container_query in &rule.container_queries {
            self.generate_container_query(container_query, &scoped_selector);
        }
    }

    /// Generate CSS for a media query
    fn generate_media_query(&mut self, media_query: &CssMediaQuery, selector: &str) {
        // Output @media condition
        self.css_output.push_str("@media ");
        self.css_output.push_str(&media_query.condition);
        self.css_output.push_str(" {\n");

        // Output selector block with media query declarations
        self.css_output.push_str("  ");
        self.css_output.push_str(selector);
        self.css_output.push_str(" {\n");

        // Generate declarations with extra indent
        for decl in &media_query.declarations {
            self.css_output.push_str("    "); // Extra indent for media query
            self.css_output.push_str(&decl.property);
            self.css_output.push_str(": ");
            self.css_output.push_str(&self.generate_value(&decl.value));
            self.css_output.push_str(";\n");
        }

        // Close selector block
        self.css_output.push_str("  }\n");

        // Close media query
        self.css_output.push_str("}\n\n");
    }

    /// Generate CSS for a container query (Phase 8 Sprint 1 Task 1.4)
    fn generate_container_query(&mut self, container_query: &CssContainerQuery, selector: &str) {
        

        // Output @container condition
        self.css_output.push_str("@container ");
        self.css_output.push_str(&container_query.condition);
        self.css_output.push_str(" {\n");

        // Output selector block with container query declarations
        self.css_output.push_str("  ");
        self.css_output.push_str(selector);
        self.css_output.push_str(" {\n");

        // Generate declarations with extra indent
        for decl in &container_query.declarations {
            self.css_output.push_str("    "); // Extra indent for container query
            self.css_output.push_str(&decl.property);
            self.css_output.push_str(": ");
            self.css_output.push_str(&self.generate_value(&decl.value));
            self.css_output.push_str(";\n");
        }

        // Close selector block
        self.css_output.push_str("  }\n");

        // Close container query
        self.css_output.push_str("}\n\n");
    }

    /// Generate CSS for keyframes animation (Sprint 2 Task 2.6)
    /// Example output:
    /// @keyframes Button_fadeIn_abc123 {
    ///   from { opacity: 0; }
    ///   to { opacity: 1; }
    /// }
    fn generate_keyframes(&mut self, keyframes: &CssKeyframes) {
        // Generate scoped keyframe name
        let scoped_name = self.generate_scoped_keyframe_name(&keyframes.name);

        // Output @keyframes declaration
        self.css_output.push_str("@keyframes ");
        self.css_output.push_str(&scoped_name);
        self.css_output.push_str(" {\n");

        // Generate each keyframe rule
        for frame in &keyframes.frames {
            // Output keyframe selector (from, to, or percentage)
            self.css_output.push_str("  ");
            self.css_output.push_str(&self.generate_keyframe_selector(&frame.selector));
            self.css_output.push_str(" {\n");

            // Generate declarations
            for decl in &frame.declarations {
                self.css_output.push_str("    ");
                self.css_output.push_str(&decl.property);
                self.css_output.push_str(": ");
                self.css_output.push_str(&self.generate_value(&decl.value));
                self.css_output.push_str(";\n");
            }

            // Close keyframe
            self.css_output.push_str("  }\n");
        }

        // Close @keyframes
        self.css_output.push_str("}\n\n");
    }

    /// Generate keyframe selector (from, to, or percentage)
    fn generate_keyframe_selector(&self, selector: &CssKeyframeSelector) -> String {
        match selector {
            CssKeyframeSelector::From => "from".to_string(),
            CssKeyframeSelector::To => "to".to_string(),
            CssKeyframeSelector::Percentage(p) => {
                // Format percentage with proper precision
                if p.fract() == 0.0 {
                    format!("{}%", *p as i32)
                } else {
                    format!("{}%", p)
                }
            }
        }
    }

    /// Generate a scoped keyframe name using hash-based approach
    /// Format: {ComponentName}_{keyframeName}_{hash}
    /// Example: "fadeIn" -> "Button_fadeIn_a3f5c9"
    fn generate_scoped_keyframe_name(&mut self, keyframe_name: &str) -> String {
        // Check if we've already generated this keyframe name
        if let Some(scoped) = self.keyframes_map.get(keyframe_name) {
            return scoped.clone();
        }

        // Generate hash from component name + keyframe name
        let hash = self.generate_hash(&format!("{}{}", self.component_name, keyframe_name));
        let scoped_name = format!("{}_{}_{}",
            self.component_name,
            keyframe_name,
            &hash[0..6]  // Use first 6 chars of hash
        );

        // Store in map for reuse
        self.keyframes_map.insert(keyframe_name.to_string(), scoped_name.clone());

        scoped_name
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
            CssSelector::PseudoClass(pseudo_class) => {
                // Pseudo-classes are not scoped (:hover, :focus, etc.)
                // Note: These should typically be combined with other selectors
                format!(":{}", pseudo_class)
            }
            CssSelector::PseudoElement(pseudo_element) => {
                // Pseudo-elements are not scoped (::before, ::after, etc.)
                // Note: These should typically be combined with other selectors
                format!("::{}", pseudo_element)
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
    fn generate_declaration(&mut self, decl: &CssDeclaration, class_name: Option<&str>) {
        // Check if value is dynamic (Sprint 2 Task 2.4)
        if let CssValue::Dynamic(ref expr) = decl.value {
            // Store dynamic declaration for JS emitter
            if let Some(class) = class_name {
                self.dynamic_declarations
                    .entry(class.to_string())
                    .or_insert_with(Vec::new)
                    .push((decl.property.clone(), *expr.clone()));
            }
            // Skip outputting dynamic declarations to static CSS
            return;
        }

        // Generate static CSS declaration
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
            CssValue::Dynamic(_expr) => {
                // Dynamic values are handled separately by the JS emitter
                // They should not appear in the static CSS output
                "/* dynamic */".to_string()
            }
        }
    }

    /// Get the class name mapping (for JavaScript code generation)
    pub fn get_class_map(&self) -> &HashMap<String, String> {
        &self.class_map
    }

    /// Get the keyframe name mapping (for JavaScript code generation)
    /// Sprint 2 Task 2.6
    pub fn get_keyframes_map(&self) -> &HashMap<String, String> {
        &self.keyframes_map
    }

    /// Get the dynamic declarations that need to become inline styles
    /// Sprint 2 Task 2.4
    pub fn get_dynamic_declarations(&self) -> &HashMap<String, Vec<(String, Expression)>> {
        &self.dynamic_declarations
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
            media_queries: vec![],
            container_queries: vec![],
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
            media_queries: vec![],
            container_queries: vec![],
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
            media_queries: vec![],
            container_queries: vec![],
                },
            ],
            keyframes: vec![],
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
                CssSelector::PseudoClass("hover".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "background".to_string(),
                    value: CssValue::Raw("darkblue".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
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
            media_queries: vec![],
            container_queries: vec![],
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
            media_queries: vec![],
            container_queries: vec![],
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
            media_queries: vec![],
            container_queries: vec![],
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
                    media_queries: vec![],
            container_queries: vec![],
                },
            ],
            media_queries: vec![],
            container_queries: vec![],
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
                    media_queries: vec![],
            container_queries: vec![],
                },
            ],
            media_queries: vec![],
            container_queries: vec![],
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
                            media_queries: vec![],
            container_queries: vec![],
                        },
                    ],
                    media_queries: vec![],
            container_queries: vec![],
                },
            ],
            media_queries: vec![],
            container_queries: vec![],
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

    #[test]
    fn test_pseudo_class() {
        let mut gen = CssGenerator::new("Button".to_string());

        let rule = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("button".to_string()),
                CssSelector::PseudoClass("hover".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "background".to_string(),
                    value: CssValue::Raw("blue".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should output scoped class + :hover (single colon)
        assert!(output.contains("Button_button_"));
        assert!(output.contains(":hover"));
        assert!(!output.contains("::hover")); // Should NOT be double colon
        assert!(output.contains("background: blue;"));
    }

    #[test]
    fn test_pseudo_element() {
        let mut gen = CssGenerator::new("Icon".to_string());

        let rule = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("icon".to_string()),
                CssSelector::PseudoElement("before".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "content".to_string(),
                    value: CssValue::String("→".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should output scoped class + ::before (double colon)
        assert!(output.contains("Icon_icon_"));
        assert!(output.contains("::before"));
        assert!(!output.contains(":::before")); // Should NOT be triple colon
        assert!(output.contains("content: \"→\";"));
    }

    #[test]
    fn test_multiple_pseudo_classes() {
        let mut gen1 = CssGenerator::new("Input".to_string());
        let mut gen2 = CssGenerator::new("Input".to_string());

        // Test :focus
        let rule1 = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("input".to_string()),
                CssSelector::PseudoClass("focus".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "outline".to_string(),
                    value: CssValue::Raw("2px solid blue".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
        };

        // Test :disabled
        let rule2 = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("input".to_string()),
                CssSelector::PseudoClass("disabled".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "opacity".to_string(),
                    value: CssValue::Raw("0.5".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
        };

        gen1.generate_rule(&rule1);
        gen2.generate_rule(&rule2);

        let output1 = gen1.css_output;
        let output2 = gen2.css_output;

        assert!(output1.contains(":focus"));
        assert!(output1.contains("outline: 2px solid blue;"));

        assert!(output2.contains(":disabled"));
        assert!(output2.contains("opacity: 0.5;"));
    }

    #[test]
    fn test_multiple_pseudo_elements() {
        let mut gen1 = CssGenerator::new("Quote".to_string());
        let mut gen2 = CssGenerator::new("Quote".to_string());

        // Test ::before
        let rule1 = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("quote".to_string()),
                CssSelector::PseudoElement("before".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "content".to_string(),
                    value: CssValue::String("«".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
        };

        // Test ::after
        let rule2 = CssRule {
            selector: CssSelector::Compound(vec![
                CssSelector::Class("quote".to_string()),
                CssSelector::PseudoElement("after".to_string()),
            ]),
            declarations: vec![
                CssDeclaration {
                    property: "content".to_string(),
                    value: CssValue::String("»".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![],
            container_queries: vec![],
        };

        gen1.generate_rule(&rule1);
        gen2.generate_rule(&rule2);

        let output1 = gen1.css_output;
        let output2 = gen2.css_output;

        assert!(output1.contains("::before"));
        assert!(output1.contains("content: \"«\";"));

        assert!(output2.contains("::after"));
        assert!(output2.contains("content: \"»\";"));
    }

    #[test]
    fn test_media_query_simple() {
        let mut gen = CssGenerator::new("Container".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("container".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "width".to_string(),
                    value: CssValue::Raw("100%".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![
                CssMediaQuery {
                    condition: "(min-width: 768px)".to_string(),
                    declarations: vec![
                        CssDeclaration {
                            property: "width".to_string(),
                            value: CssValue::Raw("750px".to_string()),
                        },
                    ],
                },
            ],
            container_queries: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should contain main rule
        assert!(output.contains("Container_container_"));
        assert!(output.contains("width: 100%;"));

        // Should contain media query
        assert!(output.contains("@media (min-width: 768px)"));
        assert!(output.contains("width: 750px;"));
    }

    #[test]
    fn test_media_query_multiple() {
        let mut gen = CssGenerator::new("Grid".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("grid".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "display".to_string(),
                    value: CssValue::Raw("grid".to_string()),
                },
            ],
            nested_rules: vec![],
            media_queries: vec![
                CssMediaQuery {
                    condition: "(min-width: 768px)".to_string(),
                    declarations: vec![
                        CssDeclaration {
                            property: "grid-template-columns".to_string(),
                            value: CssValue::Raw("repeat(2, 1fr)".to_string()),
                        },
                    ],
                },
                CssMediaQuery {
                    condition: "(min-width: 1024px)".to_string(),
                    declarations: vec![
                        CssDeclaration {
                            property: "grid-template-columns".to_string(),
                            value: CssValue::Raw("repeat(3, 1fr)".to_string()),
                        },
                    ],
                },
            ],
            container_queries: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should contain main rule
        assert!(output.contains("display: grid;"));

        // Should contain both media queries
        assert!(output.contains("@media (min-width: 768px)"));
        assert!(output.contains("repeat(2, 1fr)"));
        assert!(output.contains("@media (min-width: 1024px)"));
        assert!(output.contains("repeat(3, 1fr)"));
    }

    #[test]
    fn test_media_query_with_nesting() {
        let mut gen = CssGenerator::new("Card".to_string());

        let rule = CssRule {
            selector: CssSelector::Class("card".to_string()),
            declarations: vec![
                CssDeclaration {
                    property: "padding".to_string(),
                    value: CssValue::Raw("16px".to_string()),
                },
            ],
            nested_rules: vec![
                CssRule {
                    selector: CssSelector::Nested("&:hover".to_string()),
                    declarations: vec![
                        CssDeclaration {
                            property: "box-shadow".to_string(),
                            value: CssValue::Raw("0 4px 8px rgba(0,0,0,0.1)".to_string()),
                        },
                    ],
                    nested_rules: vec![],
                    media_queries: vec![],
            container_queries: vec![],
                },
            ],
            media_queries: vec![
                CssMediaQuery {
                    condition: "(min-width: 768px)".to_string(),
                    declarations: vec![
                        CssDeclaration {
                            property: "padding".to_string(),
                            value: CssValue::Raw("24px".to_string()),
                        },
                    ],
                },
            ],
            container_queries: vec![],
        };

        gen.generate_rule(&rule);
        let output = gen.css_output;

        // Should contain main rule
        assert!(output.contains("padding: 16px;"));

        // Should contain nested hover rule
        assert!(output.contains(":hover"));
        assert!(output.contains("box-shadow"));

        // Should contain media query
        assert!(output.contains("@media (min-width: 768px)"));
        assert!(output.contains("padding: 24px;"));
    }

    // Sprint 2 Task 2.6: Keyframe animations tests

    #[test]
    fn test_keyframes_simple_from_to() {
        let mut gen = CssGenerator::new("Button".to_string());

        let keyframes = CssKeyframes {
            name: "fadeIn".to_string(),
            frames: vec![
                CssKeyframeRule {
                    selector: CssKeyframeSelector::From,
                    declarations: vec![
                        CssDeclaration {
                            property: "opacity".to_string(),
                            value: CssValue::Raw("0".to_string()),
                        },
                    ],
                },
                CssKeyframeRule {
                    selector: CssKeyframeSelector::To,
                    declarations: vec![
                        CssDeclaration {
                            property: "opacity".to_string(),
                            value: CssValue::Raw("1".to_string()),
                        },
                    ],
                },
            ],
        };

        gen.generate_keyframes(&keyframes);
        let output = gen.css_output;

        // Should contain scoped keyframe name
        assert!(output.contains("@keyframes Button_fadeIn_"));
        // Should contain from and to
        assert!(output.contains("from {"));
        assert!(output.contains("to {"));
        // Should contain declarations
        assert!(output.contains("opacity: 0;"));
        assert!(output.contains("opacity: 1;"));
    }

    #[test]
    fn test_keyframes_percentages() {
        let mut gen = CssGenerator::new("Slider".to_string());

        let keyframes = CssKeyframes {
            name: "slideIn".to_string(),
            frames: vec![
                CssKeyframeRule {
                    selector: CssKeyframeSelector::Percentage(0.0),
                    declarations: vec![
                        CssDeclaration {
                            property: "transform".to_string(),
                            value: CssValue::Raw("translateX(-100%)".to_string()),
                        },
                    ],
                },
                CssKeyframeRule {
                    selector: CssKeyframeSelector::Percentage(50.0),
                    declarations: vec![
                        CssDeclaration {
                            property: "transform".to_string(),
                            value: CssValue::Raw("translateX(-50%)".to_string()),
                        },
                    ],
                },
                CssKeyframeRule {
                    selector: CssKeyframeSelector::Percentage(100.0),
                    declarations: vec![
                        CssDeclaration {
                            property: "transform".to_string(),
                            value: CssValue::Raw("translateX(0)".to_string()),
                        },
                    ],
                },
            ],
        };

        gen.generate_keyframes(&keyframes);
        let output = gen.css_output;

        // Should contain scoped keyframe name
        assert!(output.contains("@keyframes Slider_slideIn_"));
        // Should contain percentages
        assert!(output.contains("0% {"));
        assert!(output.contains("50% {"));
        assert!(output.contains("100% {"));
        // Should contain transforms
        assert!(output.contains("translateX(-100%)"));
        assert!(output.contains("translateX(-50%)"));
        assert!(output.contains("translateX(0)"));
    }

    #[test]
    fn test_keyframes_scoped_name_consistency() {
        let mut gen = CssGenerator::new("Component".to_string());

        let scoped1 = gen.generate_scoped_keyframe_name("bounce");
        let scoped2 = gen.generate_scoped_keyframe_name("bounce");

        // Should return same scoped name for same keyframe
        assert_eq!(scoped1, scoped2);
        assert!(scoped1.starts_with("Component_bounce_"));
    }

    #[test]
    fn test_get_keyframes_map() {
        let mut gen = CssGenerator::new("App".to_string());

        // Generate some scoped keyframe names
        gen.generate_scoped_keyframe_name("fadeIn");
        gen.generate_scoped_keyframe_name("slideOut");

        let keyframes_map = gen.get_keyframes_map();

        // Should have entries for both keyframes
        assert!(keyframes_map.contains_key("fadeIn"));
        assert!(keyframes_map.contains_key("slideOut"));
        assert!(keyframes_map.get("fadeIn").unwrap().starts_with("App_fadeIn_"));
        assert!(keyframes_map.get("slideOut").unwrap().starts_with("App_slideOut_"));
    }

    #[test]
    fn test_keyframes_with_multiple_declarations() {
        let mut gen = CssGenerator::new("Card".to_string());

        let keyframes = CssKeyframes {
            name: "pulse".to_string(),
            frames: vec![
                CssKeyframeRule {
                    selector: CssKeyframeSelector::From,
                    declarations: vec![
                        CssDeclaration {
                            property: "opacity".to_string(),
                            value: CssValue::Raw("1".to_string()),
                        },
                        CssDeclaration {
                            property: "transform".to_string(),
                            value: CssValue::Raw("scale(1)".to_string()),
                        },
                    ],
                },
                CssKeyframeRule {
                    selector: CssKeyframeSelector::To,
                    declarations: vec![
                        CssDeclaration {
                            property: "opacity".to_string(),
                            value: CssValue::Raw("0.8".to_string()),
                        },
                        CssDeclaration {
                            property: "transform".to_string(),
                            value: CssValue::Raw("scale(1.05)".to_string()),
                        },
                    ],
                },
            ],
        };

        gen.generate_keyframes(&keyframes);
        let output = gen.css_output;

        // Should contain all declarations
        assert!(output.contains("opacity: 1;"));
        assert!(output.contains("transform: scale(1);"));
        assert!(output.contains("opacity: 0.8;"));
        assert!(output.contains("transform: scale(1.05);"));
    }
}
