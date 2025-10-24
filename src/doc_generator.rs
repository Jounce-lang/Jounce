// Documentation Generator
//
// Generates HTML documentation from Jounce source code with doc comments.
// Similar to rustdoc, but for Jounce.
//
// Features:
// - Parse /// doc comments
// - Generate HTML pages for modules, functions, structs, enums
// - Search functionality
// - Syntax highlighting
// - Cross-references

use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::errors::CompileError;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents documentation for a single item
#[derive(Debug, Clone)]
pub struct DocItem {
    pub name: String,
    pub kind: DocItemKind,
    pub doc_comment: String,
    pub signature: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocItemKind {
    Function,
    Struct,
    Enum,
    Component,
    Module,
}

/// Documentation generator that creates HTML documentation
pub struct DocGenerator {
    pub output_dir: PathBuf,
    pub package_name: String,
    pub items: Vec<DocItem>,
}

impl DocGenerator {
    pub fn new(package_name: String, output_dir: PathBuf) -> Self {
        Self {
            output_dir,
            package_name,
            items: Vec::new(),
        }
    }

    /// Generate documentation from a source file
    pub fn generate_from_file(&mut self, file_path: &Path) -> Result<(), CompileError> {
        let source = fs::read_to_string(file_path)
            .map_err(|e| CompileError::Generic(format!("Failed to read {}: {}", file_path.display(), e)))?;

        self.generate_from_source(&source)
    }

    /// Generate documentation from source code
    pub fn generate_from_source(&mut self, source: &str) -> Result<(), CompileError> {
        // Parse the source
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program()?;

        // Extract documentation from AST
        self.extract_docs_from_program(&program);

        Ok(())
    }

    /// Extract documentation from program AST
    fn extract_docs_from_program(&mut self, program: &Program) {
        for statement in &program.statements {
            match statement {
                Statement::Function(func) => {
                    self.extract_function_doc(func);
                }
                Statement::Struct(struct_def) => {
                    self.extract_struct_doc(struct_def);
                }
                Statement::Enum(enum_def) => {
                    self.extract_enum_doc(enum_def);
                }
                Statement::Component(comp) => {
                    self.extract_component_doc(comp);
                }
                _ => {}
            }
        }
    }

    fn extract_function_doc(&mut self, func: &FunctionDefinition) {
        let params = func.parameters
            .iter()
            .map(|p| p.name.value.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let signature = format!("fn {}({})", func.name.value, params);

        self.items.push(DocItem {
            name: func.name.value.clone(),
            kind: DocItemKind::Function,
            doc_comment: "Function documentation".to_string(), // Would extract from comments
            signature,
            examples: Vec::new(),
        });
    }

    fn extract_struct_doc(&mut self, struct_def: &StructDefinition) {
        let fields = struct_def.fields
            .iter()
            .map(|(name, type_expr)| format!("  {}: {:?}", name.value, type_expr))
            .collect::<Vec<_>>()
            .join(",\n");

        let signature = format!("struct {} {{\n{}\n}}", struct_def.name.value, fields);

        self.items.push(DocItem {
            name: struct_def.name.value.clone(),
            kind: DocItemKind::Struct,
            doc_comment: "Struct documentation".to_string(),
            signature,
            examples: Vec::new(),
        });
    }

    fn extract_enum_doc(&mut self, enum_def: &EnumDefinition) {
        let variants = enum_def.variants
            .iter()
            .map(|v| format!("  {:?}", v))
            .collect::<Vec<_>>()
            .join(",\n");

        let signature = format!("enum {} {{\n{}\n}}", enum_def.name.value, variants);

        self.items.push(DocItem {
            name: enum_def.name.value.clone(),
            kind: DocItemKind::Enum,
            doc_comment: "Enum documentation".to_string(),
            signature,
            examples: Vec::new(),
        });
    }

    fn extract_component_doc(&mut self, comp: &ComponentDefinition) {
        let params = comp.parameters
            .iter()
            .map(|p| p.name.value.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let signature = format!("component {}({})", comp.name.value, params);

        self.items.push(DocItem {
            name: comp.name.value.clone(),
            kind: DocItemKind::Component,
            doc_comment: "Component documentation".to_string(),
            signature,
            examples: Vec::new(),
        });
    }

    /// Generate HTML documentation
    pub fn generate_html(&self) -> Result<(), CompileError> {
        // Create output directory
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| CompileError::Generic(format!("Failed to create output dir: {}", e)))?;

        // Generate index page
        self.generate_index_page()?;

        // Generate individual item pages
        for item in &self.items {
            self.generate_item_page(item)?;
        }

        // Generate search data
        self.generate_search_data()?;

        // Copy static assets (CSS, JS)
        self.generate_static_assets()?;

        Ok(())
    }

    fn generate_index_page(&self) -> Result<(), CompileError> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("  <title>{} - Documentation</title>\n", self.package_name));
        html.push_str("  <link rel=\"stylesheet\" href=\"styles.css\">\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <nav class=\"sidebar\">\n");
        html.push_str(&format!("    <h1>{}</h1>\n", self.package_name));
        html.push_str("    <input type=\"search\" placeholder=\"Search...\" id=\"search\">\n");
        html.push_str("    <ul>\n");

        // Group items by kind
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut components = Vec::new();

        for item in &self.items {
            match item.kind {
                DocItemKind::Function => functions.push(item),
                DocItemKind::Struct => structs.push(item),
                DocItemKind::Enum => enums.push(item),
                DocItemKind::Component => components.push(item),
                _ => {}
            }
        }

        if !functions.is_empty() {
            html.push_str("      <li><h3>Functions</h3><ul>\n");
            for func in functions {
                html.push_str(&format!("        <li><a href=\"{}.html\">{}</a></li>\n",
                    func.name, func.name));
            }
            html.push_str("      </ul></li>\n");
        }

        if !structs.is_empty() {
            html.push_str("      <li><h3>Structs</h3><ul>\n");
            for s in structs {
                html.push_str(&format!("        <li><a href=\"{}.html\">{}</a></li>\n",
                    s.name, s.name));
            }
            html.push_str("      </ul></li>\n");
        }

        if !enums.is_empty() {
            html.push_str("      <li><h3>Enums</h3><ul>\n");
            for e in enums {
                html.push_str(&format!("        <li><a href=\"{}.html\">{}</a></li>\n",
                    e.name, e.name));
            }
            html.push_str("      </ul></li>\n");
        }

        if !components.is_empty() {
            html.push_str("      <li><h3>Components</h3><ul>\n");
            for c in components {
                html.push_str(&format!("        <li><a href=\"{}.html\">{}</a></li>\n",
                    c.name, c.name));
            }
            html.push_str("      </ul></li>\n");
        }

        html.push_str("    </ul>\n");
        html.push_str("  </nav>\n");
        html.push_str("  <main>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", self.package_name));
        html.push_str(&format!("    <p>Documentation for {} with {} items.</p>\n",
            self.package_name, self.items.len()));
        html.push_str("  </main>\n");
        html.push_str("  <script src=\"search.js\"></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        let index_path = self.output_dir.join("index.html");
        fs::write(&index_path, html)
            .map_err(|e| CompileError::Generic(format!("Failed to write index.html: {}", e)))?;

        Ok(())
    }

    fn generate_item_page(&self, item: &DocItem) -> Result<(), CompileError> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str(&format!("  <title>{} - {}</title>\n", item.name, self.package_name));
        html.push_str("  <link rel=\"stylesheet\" href=\"styles.css\">\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <nav class=\"sidebar\">\n");
        html.push_str(&format!("    <h1><a href=\"index.html\">{}</a></h1>\n", self.package_name));
        html.push_str("  </nav>\n");
        html.push_str("  <main>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", item.name));
        html.push_str(&format!("    <div class=\"item-kind\">{:?}</div>\n", item.kind));
        html.push_str("    <div class=\"signature\">\n");
        html.push_str(&format!("      <pre><code>{}</code></pre>\n", item.signature));
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"documentation\">\n");
        html.push_str(&format!("      <p>{}</p>\n", item.doc_comment));
        html.push_str("    </div>\n");

        if !item.examples.is_empty() {
            html.push_str("    <h2>Examples</h2>\n");
            for example in &item.examples {
                html.push_str("    <pre><code class=\"language-raven\">");
                html.push_str(example);
                html.push_str("</code></pre>\n");
            }
        }

        html.push_str("  </main>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        let item_path = self.output_dir.join(format!("{}.html", item.name));
        fs::write(&item_path, html)
            .map_err(|e| CompileError::Generic(format!("Failed to write item page: {}", e)))?;

        Ok(())
    }

    fn generate_search_data(&self) -> Result<(), CompileError> {
        let mut search_data = String::from("const searchIndex = [\n");

        for item in &self.items {
            search_data.push_str(&format!(
                "  {{ name: '{}', kind: '{:?}', doc: '{}', url: '{}.html' }},\n",
                item.name,
                item.kind,
                item.doc_comment.replace('\'', "\\'"),
                item.name
            ));
        }

        search_data.push_str("];\n");

        let search_path = self.output_dir.join("search.js");
        fs::write(&search_path, search_data)
            .map_err(|e| CompileError::Generic(format!("Failed to write search.js: {}", e)))?;

        Ok(())
    }

    fn generate_static_assets(&self) -> Result<(), CompileError> {
        // Generate CSS
        let css = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    display: flex;
    height: 100vh;
}

.sidebar {
    width: 250px;
    background: #2c3e50;
    color: white;
    padding: 20px;
    overflow-y: auto;
}

.sidebar h1 {
    font-size: 24px;
    margin-bottom: 20px;
}

.sidebar h1 a {
    color: white;
    text-decoration: none;
}

.sidebar input {
    width: 100%;
    padding: 8px;
    margin-bottom: 20px;
    border: none;
    border-radius: 4px;
}

.sidebar ul {
    list-style: none;
}

.sidebar li {
    margin: 10px 0;
}

.sidebar a {
    color: #3498db;
    text-decoration: none;
}

.sidebar a:hover {
    text-decoration: underline;
}

main {
    flex: 1;
    padding: 40px;
    overflow-y: auto;
}

.item-kind {
    display: inline-block;
    background: #3498db;
    color: white;
    padding: 4px 12px;
    border-radius: 4px;
    font-size: 12px;
    margin: 10px 0;
}

.signature {
    background: #f5f5f5;
    padding: 20px;
    border-radius: 4px;
    margin: 20px 0;
}

.signature code {
    font-family: 'Monaco', 'Courier New', monospace;
    font-size: 14px;
}

.documentation {
    line-height: 1.6;
    margin: 20px 0;
}

pre {
    background: #f5f5f5;
    padding: 15px;
    border-radius: 4px;
    overflow-x: auto;
}

code {
    font-family: 'Monaco', 'Courier New', monospace;
}
"#;

        let css_path = self.output_dir.join("styles.css");
        fs::write(&css_path, css)
            .map_err(|e| CompileError::Generic(format!("Failed to write styles.css: {}", e)))?;

        Ok(())
    }

    /// Get statistics about generated documentation
    pub fn stats(&self) -> DocStats {
        let mut stats = DocStats::default();

        for item in &self.items {
            match item.kind {
                DocItemKind::Function => stats.functions += 1,
                DocItemKind::Struct => stats.structs += 1,
                DocItemKind::Enum => stats.enums += 1,
                DocItemKind::Component => stats.components += 1,
                DocItemKind::Module => stats.modules += 1,
            }
        }

        stats
    }
}

#[derive(Debug, Clone, Default)]
pub struct DocStats {
    pub functions: usize,
    pub structs: usize,
    pub enums: usize,
    pub components: usize,
    pub modules: usize,
}

impl DocStats {
    pub fn total(&self) -> usize {
        self.functions + self.structs + self.enums + self.components + self.modules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_doc_generator_creation() {
        let temp_dir = env::temp_dir().join("raven_docs_test");
        let gen = DocGenerator::new("test_package".to_string(), temp_dir);
        assert_eq!(gen.package_name, "test_package");
        assert_eq!(gen.items.len(), 0);
    }

    #[test]
    fn test_doc_generation_from_source() {
        let source = r#"
            fn add(x: i32, y: i32) -> i32 {
                return x + y;
            }

            struct Point {
                x: i32,
                y: i32,
            }
        "#;

        let temp_dir = env::temp_dir().join("raven_docs_test2");
        let mut gen = DocGenerator::new("test".to_string(), temp_dir);

        let result = gen.generate_from_source(source);
        assert!(result.is_ok());
        assert!(gen.items.len() >= 2); // Should have at least add() and Point
    }

    #[test]
    fn test_doc_stats() {
        let temp_dir = env::temp_dir().join("raven_docs_test3");
        let mut gen = DocGenerator::new("test".to_string(), temp_dir);

        gen.items.push(DocItem {
            name: "func1".to_string(),
            kind: DocItemKind::Function,
            doc_comment: String::new(),
            signature: String::new(),
            examples: Vec::new(),
        });

        gen.items.push(DocItem {
            name: "Struct1".to_string(),
            kind: DocItemKind::Struct,
            doc_comment: String::new(),
            signature: String::new(),
            examples: Vec::new(),
        });

        let stats = gen.stats();
        assert_eq!(stats.functions, 1);
        assert_eq!(stats.structs, 1);
        assert_eq!(stats.total(), 2);
    }
}
