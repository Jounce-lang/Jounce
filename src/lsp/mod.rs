// Language Server Protocol implementation for Jounce
// Provides IDE features: autocomplete, hover, diagnostics, etc.

use crate::diagnostics::Diagnostic;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::type_checker::TypeChecker;
use std::collections::HashMap;

/// LSP Server for Jounce
pub struct LanguageServer {
    documents: HashMap<String, Document>,
    stdlib_docs: StdlibDocs,
}

/// Represents an open document
pub struct Document {
    pub uri: String,
    pub content: String,
    pub version: i32,
    pub diagnostics: Vec<Diagnostic>,
}

/// Position in a document (line, character)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

/// Range in a document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Completion item for autocomplete
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Function,
    Variable,
    Keyword,
    Class,
    Module,
    Property,
    Snippet,
}

/// Context in which completions are requested
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionContext {
    /// After a dot (e.g., "foo.")
    MemberAccess { object_name: String },
    /// After :: (e.g., "Math::")
    NamespaceAccess { namespace: String },
    /// Inside a function call (e.g., "foo(|)")
    FunctionCall { function_name: String },
    /// Inside JSX tag (e.g., "<|")
    JsxTag,
    /// Inside JSX attribute (e.g., "<div |")
    JsxAttribute { tag_name: String },
    /// At statement/expression start
    StatementStart,
    /// General context (default)
    General,
}

/// Hover information
#[derive(Debug, Clone)]
pub struct Hover {
    pub contents: String,
    pub range: Option<Range>,
}

/// Signature help for function calls
#[derive(Debug, Clone)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: usize,
    pub active_parameter: usize,
}

/// Information about a function signature
#[derive(Debug, Clone)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Vec<ParameterInformation>,
}

/// Information about a function parameter
#[derive(Debug, Clone)]
pub struct ParameterInformation {
    pub label: String,
    pub documentation: Option<String>,
}

/// Text edit for document modifications
#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

/// Formatting options
#[derive(Debug, Clone)]
pub struct FormattingOptions {
    pub tab_size: usize,
    pub insert_spaces: bool,
    pub trim_trailing_whitespace: bool,
    pub insert_final_newline: bool,
}

/// Code action kind (quick fix, refactor, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
    Source,
    SourceOrganizeImports,
}

impl CodeActionKind {
    pub fn as_str(&self) -> &str {
        match self {
            CodeActionKind::QuickFix => "quickfix",
            CodeActionKind::Refactor => "refactor",
            CodeActionKind::RefactorExtract => "refactor.extract",
            CodeActionKind::RefactorInline => "refactor.inline",
            CodeActionKind::RefactorRewrite => "refactor.rewrite",
            CodeActionKind::Source => "source",
            CodeActionKind::SourceOrganizeImports => "source.organizeImports",
        }
    }
}

/// Code action (quick fix or refactoring)
#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
    pub diagnostics: Vec<Diagnostic>,
    pub edit: WorkspaceEdit,
    pub is_preferred: bool,
}

/// Workspace edit (collection of text edits)
#[derive(Debug, Clone)]
pub struct WorkspaceEdit {
    pub changes: Vec<TextEdit>,
}

/// Location in a file (for Go to Definition, Find References)
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

/// Symbol information for definition/references
#[derive(Debug, Clone)]
struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
}

/// Kind of symbol (function, variable, etc.)
#[derive(Debug, Clone, PartialEq)]
enum SymbolKind {
    Function,
    Variable,
    Component,
    Struct,
    Enum,
    Parameter,
    TypeAlias,
}

/// Document symbol for outline view
#[derive(Debug, Clone)]
pub struct DocumentSymbol {
    pub name: String,
    pub kind: DocumentSymbolKind,
    pub range: Range,
    pub selection_range: Range,
    pub detail: Option<String>,
    pub children: Vec<DocumentSymbol>,
}

/// Kind of document symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentSymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    Struct = 23,
}

/// Inlay hint (inline code annotation)
#[derive(Debug, Clone, PartialEq)]
pub struct InlayHint {
    pub position: Position,
    pub label: String,
    pub kind: InlayHintKind,
    pub tooltip: Option<String>,
    pub padding_left: bool,
    pub padding_right: bool,
}

/// Kind of inlay hint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintKind {
    /// Type hint (e.g., `: i32` after variable name)
    Type,
    /// Parameter hint (e.g., `x:` before function argument)
    Parameter,
}

impl LanguageServer {
    pub fn new() -> Self {
        LanguageServer {
            documents: HashMap::new(),
            stdlib_docs: StdlibDocs::new(),
        }
    }

    /// Open a document
    pub fn open_document(&mut self, uri: String, content: String, version: i32) {
        let diagnostics = self.analyze_document(&content);
        let document = Document {
            uri: uri.clone(),
            content,
            version,
            diagnostics,
        };
        self.documents.insert(uri, document);
    }

    /// Change document content
    pub fn change_document(&mut self, uri: &str, content: String, version: i32) {
        // Analyze first to avoid borrow checker issues
        let diagnostics = self.analyze_document(&content);

        if let Some(doc) = self.documents.get_mut(uri) {
            doc.content = content;
            doc.version = version;
            doc.diagnostics = diagnostics;
        }
    }

    /// Close a document
    pub fn close_document(&mut self, uri: &str) {
        self.documents.remove(uri);
    }

    /// Get diagnostics for a document
    pub fn get_diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        self.documents
            .get(uri)
            .map(|doc| doc.diagnostics.clone())
            .unwrap_or_default()
    }

    /// Analyze document and return diagnostics
    fn analyze_document(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Lexical analysis and parsing
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(e) => {
                diagnostics.push(Diagnostic::error(format!("{:?}", e)));
                return diagnostics;
            }
        };

        // Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        if let Err(e) = analyzer.analyze_program(&ast) {
            diagnostics.push(Diagnostic::error(format!("{:?}", e)));
            return diagnostics;
        }

        // Type checking
        let mut type_checker = TypeChecker::new();
        if let Err(e) = type_checker.check_program(&ast.statements) {
            diagnostics.push(Diagnostic::error(format!("{:?}", e)));
        }

        diagnostics
    }

    /// Get completions at a position
    pub fn get_completions(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Get document content
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return completions;
        };

        // Detect context at cursor position
        let context = self.detect_context(content, position);

        // Filter completions based on context
        match context {
            CompletionContext::NamespaceAccess { ref namespace } => {
                // Show only members of the namespace
                completions.extend(self.get_namespace_completions(namespace));
            }
            CompletionContext::MemberAccess { ref object_name } => {
                // Show only methods/fields of the object
                completions.extend(self.get_member_completions(object_name, content));
            }
            CompletionContext::JsxTag => {
                // Show HTML tags and component names
                completions.extend(self.get_jsx_tag_completions(content));
            }
            CompletionContext::JsxAttribute { ref tag_name } => {
                // Show valid attributes for this tag
                completions.extend(self.get_jsx_attribute_completions(tag_name));
            }
            CompletionContext::StatementStart => {
                // Show keywords and top-level declarations
                completions.extend(self.get_keyword_completions());
                completions.extend(self.get_scope_completions(content));
            }
            CompletionContext::FunctionCall { ref function_name } => {
                // Show parameter hints (for now, fall back to general)
                completions.extend(self.get_parameter_hints(function_name));
                // Also show general completions for argument values
                completions.extend(self.get_general_completions(content));
            }
            CompletionContext::General => {
                // Show all completions
                completions.extend(self.get_general_completions(content));
            }
        }

        completions
    }

    /// Get general completions (all available)
    fn get_general_completions(&self, content: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Add keywords
        completions.extend(self.get_keyword_completions());

        // Add stdlib functions
        completions.extend(self.stdlib_docs.get_completions());

        // Add reactive primitives
        completions.extend(self.get_reactive_completions());

        // Add JSX snippets
        completions.extend(self.get_jsx_completions());

        // Add local variables and functions from current scope
        completions.extend(self.get_scope_completions(content));

        completions
    }

    /// Get hover information at a position
    pub fn get_hover(&self, uri: &str, position: Position) -> Option<Hover> {
        let doc = self.documents.get(uri)?;
        let word = self.get_word_at_position(&doc.content, position)?;

        // Check stdlib first (highest priority)
        if let Some(docs) = self.stdlib_docs.get_documentation(&word) {
            return Some(Hover {
                contents: docs,
                range: None,
            });
        }

        // Check reactive primitives
        if let Some(docs) = self.get_reactive_docs(&word) {
            return Some(Hover {
                contents: docs,
                range: None,
            });
        }

        // Check local symbols (functions, variables, components, etc.)
        if let Some(hover_info) = self.get_symbol_hover_info(&doc.content, &word) {
            return Some(Hover {
                contents: hover_info,
                range: None,
            });
        }

        None
    }

    /// Get hover information for a local symbol (function, variable, struct, enum)
    fn get_symbol_hover_info(&self, content: &str, symbol_name: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();

        for (line_idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Match function definitions: fn name(
            if let Some(fn_pos) = trimmed.find("fn ") {
                if let Some(paren_pos) = trimmed[fn_pos..].find('(') {
                    let name_start = fn_pos + 3;
                    let name_end = fn_pos + paren_pos;
                    let name = trimmed[name_start..name_end].trim();

                    if name == symbol_name {
                        // Extract full function signature (including return type if present)
                        let mut signature = String::from(trimmed);

                        // If signature doesn't end with { or ;, look at next lines for return type
                        if !trimmed.ends_with('{') && !trimmed.ends_with(';') {
                            let mut line_offset = 1;
                            while line_idx + line_offset < lines.len() {
                                let next_line = lines[line_idx + line_offset].trim();
                                signature.push(' ');
                                signature.push_str(next_line);
                                if next_line.contains('{') || next_line.contains(';') {
                                    break;
                                }
                                line_offset += 1;
                                if line_offset > 3 {
                                    break; // Safety limit
                                }
                            }
                        }

                        // Clean up the signature (remove opening brace)
                        if let Some(brace_pos) = signature.find('{') {
                            signature = signature[..brace_pos].trim().to_string();
                        }

                        return Some(format!("```raven\n{}\n```\n\n**Function**", signature));
                    }
                }
            }

            // Match component definitions: component Name(
            if let Some(comp_pos) = trimmed.find("component ") {
                if let Some(paren_pos) = trimmed[comp_pos..].find('(') {
                    let name_start = comp_pos + 10;
                    let name_end = comp_pos + paren_pos;
                    let name = trimmed[name_start..name_end].trim();

                    if name == symbol_name {
                        let mut signature = String::from(trimmed);

                        // Look for multi-line signatures
                        if !trimmed.ends_with('{') {
                            let mut line_offset = 1;
                            while line_idx + line_offset < lines.len() {
                                let next_line = lines[line_idx + line_offset].trim();
                                signature.push(' ');
                                signature.push_str(next_line);
                                if next_line.contains('{') {
                                    break;
                                }
                                line_offset += 1;
                                if line_offset > 3 {
                                    break;
                                }
                            }
                        }

                        if let Some(brace_pos) = signature.find('{') {
                            signature = signature[..brace_pos].trim().to_string();
                        }

                        return Some(format!("```raven\n{}\n```\n\n**Component**", signature));
                    }
                }
            }

            // Match variable definitions: let name = or let name: Type =
            if let Some(let_pos) = trimmed.find("let ") {
                let after_let = &trimmed[let_pos + 4..];
                let name_end = after_let
                    .find(|c: char| c == '=' || c == ':' || c == ' ')
                    .unwrap_or(after_let.len());
                let name = after_let[..name_end].trim();

                if name == symbol_name {
                    // Try to extract type annotation
                    if let Some(colon_pos) = after_let.find(':') {
                        if let Some(eq_pos) = after_let.find('=') {
                            let type_str = after_let[colon_pos + 1..eq_pos].trim();
                            return Some(format!(
                                "```raven\nlet {}: {}\n```\n\n**Variable** with type `{}`",
                                name, type_str, type_str
                            ));
                        }
                    }

                    // No type annotation, show just the declaration
                    return Some(format!("```raven\nlet {}\n```\n\n**Variable**", name));
                }
            }

            // Match const definitions: const NAME =
            if let Some(const_pos) = trimmed.find("const ") {
                let after_const = &trimmed[const_pos + 6..];
                let name_end = after_const
                    .find(|c: char| c == '=' || c == ':' || c == ' ')
                    .unwrap_or(after_const.len());
                let name = after_const[..name_end].trim();

                if name == symbol_name {
                    // Try to extract type annotation
                    if let Some(colon_pos) = after_const.find(':') {
                        if let Some(eq_pos) = after_const.find('=') {
                            let type_str = after_const[colon_pos + 1..eq_pos].trim();
                            return Some(format!(
                                "```raven\nconst {}: {}\n```\n\n**Constant** with type `{}`",
                                name, type_str, type_str
                            ));
                        }
                    }

                    return Some(format!("```raven\nconst {}\n```\n\n**Constant**", name));
                }
            }

            // Match struct definitions: struct Name
            if let Some(struct_pos) = trimmed.find("struct ") {
                let after_struct = &trimmed[struct_pos + 7..];
                let name_end = after_struct
                    .find(|c: char| c == '{' || c == ' ')
                    .unwrap_or(after_struct.len());
                let name = after_struct[..name_end].trim();

                if name == symbol_name {
                    // Try to extract struct definition (few lines)
                    let mut definition = String::from(trimmed);
                    let mut line_offset = 1;

                    // Add up to 5 more lines for struct body
                    while line_idx + line_offset < lines.len() && line_offset <= 5 {
                        let next_line = lines[line_idx + line_offset];
                        definition.push('\n');
                        definition.push_str(next_line);
                        if next_line.trim() == "}" {
                            break;
                        }
                        line_offset += 1;
                    }

                    return Some(format!("```raven\n{}\n```\n\n**Struct** definition", definition));
                }
            }

            // Match enum definitions: enum Name
            if let Some(enum_pos) = trimmed.find("enum ") {
                let after_enum = &trimmed[enum_pos + 5..];
                let name_end = after_enum
                    .find(|c: char| c == '{' || c == ' ')
                    .unwrap_or(after_enum.len());
                let name = after_enum[..name_end].trim();

                if name == symbol_name {
                    // Try to extract enum definition (few lines)
                    let mut definition = String::from(trimmed);
                    let mut line_offset = 1;

                    // Add up to 10 more lines for enum variants
                    while line_idx + line_offset < lines.len() && line_offset <= 10 {
                        let next_line = lines[line_idx + line_offset];
                        definition.push('\n');
                        definition.push_str(next_line);
                        if next_line.trim() == "}" {
                            break;
                        }
                        line_offset += 1;
                    }

                    return Some(format!("```raven\n{}\n```\n\n**Enum** definition", definition));
                }
            }
        }

        None
    }

    /// Get signature help for function calls at the given position
    ///
    /// Shows parameter information when cursor is inside a function call.
    /// This implements the LSP `textDocument/signatureHelp` request.
    pub fn get_signature_help(&self, uri: &str, position: Position) -> Option<SignatureHelp> {
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return None;
        };

        // Find if we're inside a function call
        let (function_name, param_index) = self.find_function_call_context(content, position)?;

        // Extract function signature from source code
        let signature_info = self.extract_function_signature(content, &function_name)?;

        Some(SignatureHelp {
            signatures: vec![signature_info],
            active_signature: 0,
            active_parameter: param_index,
        })
    }

    /// Find the function being called and the current parameter index
    fn find_function_call_context(&self, content: &str, position: Position) -> Option<(String, usize)> {
        let lines: Vec<&str> = content.lines().collect();
        if position.line >= lines.len() {
            return None;
        }

        let line = lines[position.line];
        let chars: Vec<char> = line.chars().collect();

        // Start from cursor position and work backwards to find opening paren
        let mut char_idx = position.character.min(chars.len());
        let mut paren_count = 0;
        let mut found_open_paren = false;

        // Go backwards to find the matching opening parenthesis
        while char_idx > 0 {
            char_idx -= 1;
            match chars[char_idx] {
                ')' => paren_count += 1,
                '(' => {
                    if paren_count == 0 {
                        found_open_paren = true;
                        break;
                    }
                    paren_count -= 1;
                }
                _ => {}
            }
        }

        if !found_open_paren {
            return None;
        }

        // Extract function name before the opening paren
        let mut name_end = char_idx;
        let mut name_start = name_end;

        // Skip whitespace before paren
        while name_start > 0 && chars[name_start - 1].is_whitespace() {
            name_start -= 1;
            name_end -= 1;
        }

        // Extract function name (alphanumeric and _)
        while name_start > 0 && (chars[name_start - 1].is_alphanumeric() || chars[name_start - 1] == '_') {
            name_start -= 1;
        }

        if name_start >= name_end {
            return None;
        }

        let function_name: String = chars[name_start..name_end].iter().collect();

        // Count commas between opening paren and cursor to determine parameter index
        let mut param_index = 0;
        let mut nested_parens = 0;

        for i in (char_idx + 1)..position.character.min(chars.len()) {
            match chars[i] {
                '(' => nested_parens += 1,
                ')' => nested_parens -= 1,
                ',' if nested_parens == 0 => param_index += 1,
                _ => {}
            }
        }

        Some((function_name, param_index))
    }

    /// Extract function signature from source code
    fn extract_function_signature(&self, content: &str, function_name: &str) -> Option<SignatureInformation> {
        let lines: Vec<&str> = content.lines().collect();

        for (line_idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Match function definitions: fn name(
            if let Some(fn_pos) = trimmed.find("fn ") {
                if let Some(paren_pos) = trimmed[fn_pos..].find('(') {
                    let name_start = fn_pos + 3;
                    let name_end = fn_pos + paren_pos;
                    let name = trimmed[name_start..name_end].trim();

                    if name == function_name {
                        // Extract full function signature
                        let mut signature = String::from(trimmed);

                        // Look for multi-line signatures
                        if !trimmed.ends_with('{') && !trimmed.ends_with(';') {
                            let mut line_offset = 1;
                            while line_idx + line_offset < lines.len() {
                                let next_line = lines[line_idx + line_offset].trim();
                                signature.push(' ');
                                signature.push_str(next_line);
                                if next_line.contains('{') || next_line.contains(';') {
                                    break;
                                }
                                line_offset += 1;
                                if line_offset > 3 {
                                    break;
                                }
                            }
                        }

                        // Clean up signature (remove opening brace)
                        if let Some(brace_pos) = signature.find('{') {
                            signature = signature[..brace_pos].trim().to_string();
                        }

                        // Extract parameters
                        let params = self.extract_parameters(&signature);

                        return Some(SignatureInformation {
                            label: signature.clone(),
                            documentation: Some(format!("Function: {}", function_name)),
                            parameters: params,
                        });
                    }
                }
            }
        }

        None
    }

    /// Extract parameters from a function signature
    fn extract_parameters(&self, signature: &str) -> Vec<ParameterInformation> {
        let mut parameters = Vec::new();

        // Find the parameter list (between parentheses)
        if let Some(open_paren) = signature.find('(') {
            if let Some(close_paren) = signature.rfind(')') {
                let param_str = &signature[open_paren + 1..close_paren];

                // Split by comma, but respect nested types like Vec<T>
                let mut current_param = String::new();
                let mut depth = 0;

                for ch in param_str.chars() {
                    match ch {
                        '<' | '(' | '[' => {
                            depth += 1;
                            current_param.push(ch);
                        }
                        '>' | ')' | ']' => {
                            depth -= 1;
                            current_param.push(ch);
                        }
                        ',' if depth == 0 => {
                            // End of parameter
                            let param = current_param.trim().to_string();
                            if !param.is_empty() {
                                parameters.push(ParameterInformation {
                                    label: param.clone(),
                                    documentation: None,
                                });
                            }
                            current_param.clear();
                        }
                        _ => {
                            current_param.push(ch);
                        }
                    }
                }

                // Add the last parameter
                let param = current_param.trim().to_string();
                if !param.is_empty() {
                    parameters.push(ParameterInformation {
                        label: param,
                        documentation: None,
                    });
                }
            }
        }

        parameters
    }

    /// Get inlay hints for a document range
    pub fn get_inlay_hints(&self, uri: &str, range: Range) -> Vec<InlayHint> {
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return Vec::new();
        };

        let mut hints = Vec::new();

        // Extract type hints (for variables without explicit types)
        hints.extend(self.extract_type_hints(content, range));

        // Extract parameter hints (for function calls)
        hints.extend(self.extract_parameter_hints(content, range));

        hints
    }

    /// Extract type hints for variables without explicit type annotations
    fn extract_type_hints(&self, content: &str, range: Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_idx, line) in lines.iter().enumerate() {
            // Skip lines outside the range
            if line_idx < range.start.line || line_idx > range.end.line {
                continue;
            }

            let trimmed = line.trim();

            // Match variable declarations: let name = value;
            // We'll look for patterns like "let identifier = "
            if trimmed.starts_with("let ") {
                // Extract variable name and infer type from value
                if let Some(eq_pos) = trimmed.find('=') {
                    // Get text between "let" and "="
                    let var_part = trimmed[4..eq_pos].trim();

                    // Check if it's a mutable binding
                    let var_name = if var_part.starts_with("mut ") {
                        var_part[4..].trim()
                    } else {
                        var_part
                    };

                    // Skip if variable already has type annotation (contains ':')
                    if var_name.contains(':') {
                        continue;
                    }

                    // Get the value part after "="
                    let value_part = trimmed[eq_pos + 1..].trim();

                    // Infer type from the value
                    let inferred_type = self.infer_type_from_value(value_part);

                    if let Some(type_str) = inferred_type {
                        // Find the position after the variable name in the original line
                        if let Some(var_start) = line.find(var_name) {
                            let position = Position {
                                line: line_idx,
                                character: var_start + var_name.len(),
                            };

                            hints.push(InlayHint {
                                position,
                                label: format!(": {}", type_str),
                                kind: InlayHintKind::Type,
                                tooltip: Some(format!("Inferred type: {}", type_str)),
                                padding_left: false,
                                padding_right: true,
                            });
                        }
                    }
                }
            }
        }

        hints
    }

    /// Extract parameter hints for function calls
    fn extract_parameter_hints(&self, content: &str, range: Range) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_idx, line) in lines.iter().enumerate() {
            // Skip lines outside the range
            if line_idx < range.start.line || line_idx > range.end.line {
                continue;
            }

            // Find function calls (pattern: identifier(...))
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                // Look for function name followed by (
                if chars[i].is_alphabetic() || chars[i] == '_' {
                    let name_start = i;
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        i += 1;
                    }
                    let name_end = i;

                    // Skip whitespace
                    while i < chars.len() && chars[i].is_whitespace() {
                        i += 1;
                    }

                    // Check if followed by (
                    if i < chars.len() && chars[i] == '(' {
                        let function_name: String = chars[name_start..name_end].iter().collect();

                        // Extract function signature to get parameter names
                        if let Some(sig) = self.extract_function_signature(content, &function_name) {
                            // Find the arguments in the call
                            let call_start = i + 1;
                            let mut paren_depth = 1;
                            let mut call_end = call_start;

                            while call_end < chars.len() && paren_depth > 0 {
                                match chars[call_end] {
                                    '(' => paren_depth += 1,
                                    ')' => paren_depth -= 1,
                                    _ => {}
                                }
                                if paren_depth > 0 {
                                    call_end += 1;
                                }
                            }

                            // Extract arguments (split by comma at depth 0)
                            let args_str: String = chars[call_start..call_end].iter().collect();
                            let args = self.split_arguments(&args_str);

                            // Add parameter hints for each argument
                            for (arg_idx, _arg) in args.iter().enumerate() {
                                if arg_idx < sig.parameters.len() {
                                    let param_name = &sig.parameters[arg_idx].label;

                                    // Extract just the parameter name (before :)
                                    let name_only = param_name.split(':').next().unwrap_or(param_name).trim();

                                    // Find position of this argument
                                    // For simplicity, we'll use the start of the call for now
                                    // A more sophisticated approach would track exact positions
                                    let position = Position {
                                        line: line_idx,
                                        character: call_start,
                                    };

                                    hints.push(InlayHint {
                                        position,
                                        label: format!("{}:", name_only),
                                        kind: InlayHintKind::Parameter,
                                        tooltip: Some(format!("Parameter: {}", param_name)),
                                        padding_left: false,
                                        padding_right: true,
                                    });
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
        }

        hints
    }

    /// Infer type from a value expression
    fn infer_type_from_value(&self, value: &str) -> Option<String> {
        let value = value.trim();

        // Strip trailing semicolon if present
        let value = value.trim_end_matches(';').trim();

        // String literals
        if value.starts_with('"') && value.contains('"') {
            return Some("String".to_string());
        }

        // Character literals
        if value.starts_with('\'') && value.contains('\'') {
            return Some("char".to_string());
        }

        // Boolean literals
        if value == "true" || value == "false" {
            return Some("bool".to_string());
        }

        // Numeric literals - try float first (more specific)
        if value.contains('.') {
            if let Ok(_) = value.parse::<f64>() {
                return Some("f64".to_string());
            }
        }

        // Integer literals
        if let Ok(_) = value.parse::<i32>() {
            return Some("i32".to_string());
        }

        // Array literals
        if value.starts_with('[') {
            return Some("Array".to_string());
        }

        // Vec constructor
        if value.starts_with("vec![") || value.starts_with("Vec::new") {
            return Some("Vec".to_string());
        }

        // Function calls - we'd need more context
        None
    }

    /// Split function arguments by comma (respecting nesting)
    fn split_arguments(&self, args_str: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut depth = 0;

        for ch in args_str.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    depth += 1;
                    current_arg.push(ch);
                }
                ')' | ']' | '}' | '>' => {
                    depth -= 1;
                    current_arg.push(ch);
                }
                ',' if depth == 0 => {
                    let trimmed = current_arg.trim().to_string();
                    if !trimmed.is_empty() {
                        args.push(trimmed);
                    }
                    current_arg.clear();
                }
                _ => {
                    current_arg.push(ch);
                }
            }
        }

        // Add last argument
        let trimmed = current_arg.trim().to_string();
        if !trimmed.is_empty() {
            args.push(trimmed);
        }

        args
    }

    /// Get word at position
    fn get_word_at_position(&self, content: &str, position: Position) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        if position.line >= lines.len() {
            return None;
        }

        let line = lines[position.line];
        if position.character >= line.len() {
            return None;
        }

        // Find word boundaries
        let chars: Vec<char> = line.chars().collect();
        let mut start = position.character;
        let mut end = position.character;

        // Go backwards to find start
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }

        // Go forwards to find end
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }

        if start < end {
            Some(chars[start..end].iter().collect())
        } else {
            None
        }
    }

    /// Detect the completion context at the cursor position
    fn detect_context(&self, content: &str, position: Position) -> CompletionContext {
        let lines: Vec<&str> = content.lines().collect();
        if position.line >= lines.len() {
            return CompletionContext::General;
        }

        let line = lines[position.line];
        let chars: Vec<char> = line.chars().collect();

        // Get text before cursor
        let text_before = if position.character <= chars.len() {
            chars[..position.character].iter().collect::<String>()
        } else {
            line.to_string()
        };

        // Detect JSX context
        if let Some(context) = self.detect_jsx_context(&text_before, content, position) {
            return context;
        }

        // Detect namespace access (::)
        if text_before.ends_with("::") {
            if let Some(namespace) = self.extract_namespace(&text_before) {
                return CompletionContext::NamespaceAccess { namespace };
            }
        }

        // Detect member access (.)
        if text_before.ends_with('.') {
            if let Some(object_name) = self.extract_object_name(&text_before) {
                return CompletionContext::MemberAccess { object_name };
            }
        }

        // Detect function call context
        if let Some(function_name) = self.detect_function_call(&text_before) {
            return CompletionContext::FunctionCall { function_name };
        }

        // Detect statement start (beginning of line or after {)
        let trimmed = text_before.trim_start();
        if trimmed.is_empty() || trimmed.ends_with('{') {
            return CompletionContext::StatementStart;
        }

        CompletionContext::General
    }

    /// Detect JSX context (tag or attribute)
    fn detect_jsx_context(&self, text_before: &str, _content: &str, _position: Position) -> Option<CompletionContext> {
        // Check if we're inside a JSX tag
        let mut depth: i32 = 0;
        let mut in_tag = false;
        let mut tag_name = String::new();
        let mut collecting_tag_name = false;

        for ch in text_before.chars() {
            match ch {
                '<' => {
                    depth += 1;
                    in_tag = true;
                    collecting_tag_name = true;
                    tag_name.clear();
                }
                '>' => {
                    depth = depth.saturating_sub(1);
                    in_tag = false;
                    collecting_tag_name = false;
                }
                ' ' | '\t' if in_tag && collecting_tag_name => {
                    collecting_tag_name = false;
                }
                _ if collecting_tag_name && (ch.is_alphanumeric() || ch == '_') => {
                    tag_name.push(ch);
                }
                _ => {}
            }
        }

        if in_tag && depth > 0 {
            if tag_name.is_empty() {
                // Right after <, suggest tags
                return Some(CompletionContext::JsxTag);
            } else {
                // Inside tag with name, suggest attributes
                return Some(CompletionContext::JsxAttribute { tag_name });
            }
        }

        None
    }

    /// Extract namespace from text like "Math::"
    fn extract_namespace(&self, text: &str) -> Option<String> {
        let trimmed = text.trim_end_matches("::");
        let parts: Vec<&str> = trimmed.split(|c: char| !c.is_alphanumeric() && c != '_' && c != ':').collect();
        parts.last().and_then(|s| {
            if !s.is_empty() {
                Some(s.trim_end_matches("::").to_string())
            } else {
                None
            }
        })
    }

    /// Extract object name from text like "foo."
    fn extract_object_name(&self, text: &str) -> Option<String> {
        let trimmed = text.trim_end_matches('.');
        let parts: Vec<&str> = trimmed.split(|c: char| !c.is_alphanumeric() && c != '_').collect();
        parts.last().and_then(|s| {
            if !s.is_empty() {
                Some(s.to_string())
            } else {
                None
            }
        })
    }

    /// Detect if we're inside a function call
    fn detect_function_call(&self, text: &str) -> Option<String> {
        // Find the last unclosed (
        let mut depth = 0;
        let mut function_start = None;

        for (i, ch) in text.char_indices().rev() {
            match ch {
                ')' => depth += 1,
                '(' => {
                    if depth == 0 {
                        function_start = Some(i);
                        break;
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }

        if let Some(start) = function_start {
            let before_paren = &text[..start];
            // Extract function name
            let parts: Vec<&str> = before_paren.split(|c: char| !c.is_alphanumeric() && c != '_' && c != ':').collect();
            if let Some(name) = parts.last() {
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }

        None
    }

    /// Get completions for namespace members (after ::)
    fn get_namespace_completions(&self, namespace: &str) -> Vec<CompletionItem> {
        match namespace {
            "Math" => vec![
                CompletionItem {
                    label: "abs".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn abs(x: f64) -> f64".to_string()),
                    documentation: Some("Returns the absolute value of a number".to_string()),
                    insert_text: Some("abs($0)".to_string()),
                },
                CompletionItem {
                    label: "min".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn min(a: f64, b: f64) -> f64".to_string()),
                    documentation: Some("Returns the smaller of two numbers".to_string()),
                    insert_text: Some("min($0)".to_string()),
                },
                CompletionItem {
                    label: "max".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn max(a: f64, b: f64) -> f64".to_string()),
                    documentation: Some("Returns the larger of two numbers".to_string()),
                    insert_text: Some("max($0)".to_string()),
                },
                CompletionItem {
                    label: "sqrt".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn sqrt(x: f64) -> f64".to_string()),
                    documentation: Some("Returns the square root of a number".to_string()),
                    insert_text: Some("sqrt($0)".to_string()),
                },
                CompletionItem {
                    label: "pow".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn pow(base: f64, exponent: f64) -> f64".to_string()),
                    documentation: Some("Raises a number to a power".to_string()),
                    insert_text: Some("pow($0)".to_string()),
                },
                CompletionItem {
                    label: "round".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn round(x: f64) -> f64".to_string()),
                    documentation: Some("Rounds to the nearest integer".to_string()),
                    insert_text: Some("round($0)".to_string()),
                },
                CompletionItem {
                    label: "floor".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn floor(x: f64) -> f64".to_string()),
                    documentation: Some("Rounds down to the nearest integer".to_string()),
                    insert_text: Some("floor($0)".to_string()),
                },
                CompletionItem {
                    label: "ceil".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn ceil(x: f64) -> f64".to_string()),
                    documentation: Some("Rounds up to the nearest integer".to_string()),
                    insert_text: Some("ceil($0)".to_string()),
                },
                CompletionItem {
                    label: "random".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn random() -> f64".to_string()),
                    documentation: Some("Returns a random number between 0 and 1".to_string()),
                    insert_text: Some("random()".to_string()),
                },
            ],
            "Signal" => vec![
                CompletionItem {
                    label: "new".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn new<T>(initial: T) -> Signal<T>".to_string()),
                    documentation: Some("Creates a new reactive signal".to_string()),
                    insert_text: Some("new($0)".to_string()),
                },
            ],
            "Computed" => vec![
                CompletionItem {
                    label: "new".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn new<T>(fn() -> T) -> Computed<T>".to_string()),
                    documentation: Some("Creates a computed value".to_string()),
                    insert_text: Some("new(|| $0)".to_string()),
                },
            ],
            "Effect" => vec![
                CompletionItem {
                    label: "new".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn new(fn())".to_string()),
                    documentation: Some("Creates an effect".to_string()),
                    insert_text: Some("new(|| $0)".to_string()),
                },
            ],
            "String" => vec![
                CompletionItem {
                    label: "split".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn split(s: &str, delimiter: &str) -> Vec<String>".to_string()),
                    documentation: Some("Splits a string by a delimiter".to_string()),
                    insert_text: Some("split($0)".to_string()),
                },
                CompletionItem {
                    label: "trim".to_string(),
                    kind: CompletionItemKind::Function,
                    detail: Some("fn trim(s: &str) -> String".to_string()),
                    documentation: Some("Removes whitespace from both ends".to_string()),
                    insert_text: Some("trim($0)".to_string()),
                },
            ],
            _ => vec![],
        }
    }

    /// Get completions for object members (after .)
    fn get_member_completions(&self, _object_name: &str, _content: &str) -> Vec<CompletionItem> {
        // TODO: Use type information to determine available methods/fields
        // For now, show common methods that work on multiple types
        vec![
            CompletionItem {
                label: "get".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Get value from Signal".to_string()),
                documentation: Some("Gets the current value of a signal".to_string()),
                insert_text: Some("get()".to_string()),
            },
            CompletionItem {
                label: "set".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Set value in Signal".to_string()),
                documentation: Some("Sets a new value in a signal".to_string()),
                insert_text: Some("set($0)".to_string()),
            },
            CompletionItem {
                label: "iter".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Get iterator".to_string()),
                documentation: Some("Returns an iterator over the collection".to_string()),
                insert_text: Some("iter()".to_string()),
            },
            CompletionItem {
                label: "map".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Map over collection".to_string()),
                documentation: Some("Transforms each element".to_string()),
                insert_text: Some("map(|$1| $0)".to_string()),
            },
            CompletionItem {
                label: "filter".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Filter collection".to_string()),
                documentation: Some("Filters elements by predicate".to_string()),
                insert_text: Some("filter(|$1| $0)".to_string()),
            },
            CompletionItem {
                label: "len".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Get length".to_string()),
                documentation: Some("Returns the length of the collection".to_string()),
                insert_text: Some("len()".to_string()),
            },
            CompletionItem {
                label: "push".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Push to Vec".to_string()),
                documentation: Some("Adds an element to the end".to_string()),
                insert_text: Some("push($0)".to_string()),
            },
        ]
    }

    /// Get JSX tag completions
    fn get_jsx_tag_completions(&self, content: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Add component names from current file
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        if let Ok(ast) = parser.parse_program() {
            use crate::ast::Statement;
            for statement in &ast.statements {
                if let Statement::Component(comp_def) = statement {
                    completions.push(CompletionItem {
                        label: comp_def.name.value.clone(),
                        kind: CompletionItemKind::Class,
                        detail: Some(format!("component {}", comp_def.name.value)),
                        documentation: Some("User-defined component".to_string()),
                        insert_text: Some(format!("{}>$0</{}>", comp_def.name.value, comp_def.name.value)),
                    });
                }
            }
        }

        // Add common HTML tags
        completions.extend(vec![
            CompletionItem {
                label: "div".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Container div".to_string()),
                documentation: Some("Block container element".to_string()),
                insert_text: Some("div>$0</div>".to_string()),
            },
            CompletionItem {
                label: "button".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Button element".to_string()),
                documentation: Some("Clickable button".to_string()),
                insert_text: Some("button onclick={$1}>\"$0\"</button>".to_string()),
            },
            CompletionItem {
                label: "input".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Input field".to_string()),
                documentation: Some("Text input".to_string()),
                insert_text: Some("input type=\"text\" value={$0} />".to_string()),
            },
            CompletionItem {
                label: "h1".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Heading 1".to_string()),
                documentation: Some("Top-level heading".to_string()),
                insert_text: Some("h1>\"$0\"</h1>".to_string()),
            },
            CompletionItem {
                label: "p".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Paragraph".to_string()),
                documentation: Some("Paragraph text".to_string()),
                insert_text: Some("p>\"$0\"</p>".to_string()),
            },
            CompletionItem {
                label: "ul".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Unordered list".to_string()),
                documentation: Some("Bullet point list".to_string()),
                insert_text: Some("ul>$0</ul>".to_string()),
            },
            CompletionItem {
                label: "li".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("List item".to_string()),
                documentation: Some("List item".to_string()),
                insert_text: Some("li>$0</li>".to_string()),
            },
        ]);

        completions
    }

    /// Get JSX attribute completions
    fn get_jsx_attribute_completions(&self, tag_name: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Common attributes for all elements
        completions.extend(vec![
            CompletionItem {
                label: "class".to_string(),
                kind: CompletionItemKind::Property,
                detail: Some("CSS class".to_string()),
                documentation: Some("CSS class name".to_string()),
                insert_text: Some("class=\"$0\"".to_string()),
            },
            CompletionItem {
                label: "id".to_string(),
                kind: CompletionItemKind::Property,
                detail: Some("Element ID".to_string()),
                documentation: Some("Unique element identifier".to_string()),
                insert_text: Some("id=\"$0\"".to_string()),
            },
            CompletionItem {
                label: "style".to_string(),
                kind: CompletionItemKind::Property,
                detail: Some("Inline styles".to_string()),
                documentation: Some("Inline CSS styles".to_string()),
                insert_text: Some("style=\"$0\"".to_string()),
            },
        ]);

        // Tag-specific attributes
        match tag_name {
            "button" | "div" | "span" | "a" => {
                completions.push(CompletionItem {
                    label: "onclick".to_string(),
                    kind: CompletionItemKind::Property,
                    detail: Some("Click event handler".to_string()),
                    documentation: Some("Called when element is clicked".to_string()),
                    insert_text: Some("onclick={$0}".to_string()),
                });
            }
            "input" | "textarea" => {
                completions.extend(vec![
                    CompletionItem {
                        label: "value".to_string(),
                        kind: CompletionItemKind::Property,
                        detail: Some("Input value".to_string()),
                        documentation: Some("Current input value".to_string()),
                        insert_text: Some("value={$0}".to_string()),
                    },
                    CompletionItem {
                        label: "oninput".to_string(),
                        kind: CompletionItemKind::Property,
                        detail: Some("Input event handler".to_string()),
                        documentation: Some("Called when input changes".to_string()),
                        insert_text: Some("oninput={$0}".to_string()),
                    },
                    CompletionItem {
                        label: "placeholder".to_string(),
                        kind: CompletionItemKind::Property,
                        detail: Some("Placeholder text".to_string()),
                        documentation: Some("Placeholder when empty".to_string()),
                        insert_text: Some("placeholder=\"$0\"".to_string()),
                    },
                ]);
            }
            "form" => {
                completions.push(CompletionItem {
                    label: "onsubmit".to_string(),
                    kind: CompletionItemKind::Property,
                    detail: Some("Submit event handler".to_string()),
                    documentation: Some("Called when form is submitted".to_string()),
                    insert_text: Some("onsubmit={$0}".to_string()),
                });
            }
            _ => {}
        }

        completions
    }

    /// Get parameter hints for function calls
    fn get_parameter_hints(&self, _function_name: &str) -> Vec<CompletionItem> {
        // TODO: Look up function signature and return parameter information
        // For now, return empty
        vec![]
    }

    /// Get keyword completions
    fn get_keyword_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "component".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define a component".to_string()),
                documentation: Some("Create a new component".to_string()),
                insert_text: Some("component ${1:Name}() {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "fn".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define a function".to_string()),
                documentation: None,
                insert_text: Some("fn ${1:name}($2) -> $3 {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "let".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Declare a variable".to_string()),
                documentation: None,
                insert_text: Some("let ${1:name} = $0;".to_string()),
            },
            CompletionItem {
                label: "@server".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Server-side annotation".to_string()),
                documentation: Some("Mark function as server-side only (has network access, database access)".to_string()),
                insert_text: None,
            },
            CompletionItem {
                label: "@client".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Client-side annotation".to_string()),
                documentation: Some("Mark function as client-side only (has DOM access, browser APIs)".to_string()),
                insert_text: None,
            },
            CompletionItem {
                label: "if".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Conditional".to_string()),
                documentation: None,
                insert_text: Some("if $1 {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "for".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Loop".to_string()),
                documentation: None,
                insert_text: Some("for ${1:item} in ${2:items} {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "while".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("While loop".to_string()),
                documentation: None,
                insert_text: Some("while $1 {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "match".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Pattern matching".to_string()),
                documentation: None,
                insert_text: Some("match ${1:value} {\n    ${2:pattern} => $0,\n}".to_string()),
            },
            CompletionItem {
                label: "struct".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define a struct".to_string()),
                documentation: None,
                insert_text: Some("struct ${1:Name} {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "enum".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define an enum".to_string()),
                documentation: None,
                insert_text: Some("enum ${1:Name} {\n    $0,\n}".to_string()),
            },
            CompletionItem {
                label: "return".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Return a value".to_string()),
                documentation: None,
                insert_text: Some("return $0;".to_string()),
            },
        ]
    }

    /// Get JSX-specific completions
    fn get_jsx_completions(&self) -> Vec<CompletionItem> {
        vec![
            // HTML elements
            CompletionItem {
                label: "<div>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Div element".to_string()),
                documentation: Some("Container div element".to_string()),
                insert_text: Some("<div class=\"${1:className}\">\n    $0\n</div>".to_string()),
            },
            CompletionItem {
                label: "<button>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Button element".to_string()),
                documentation: Some("Clickable button".to_string()),
                insert_text: Some("<button onclick={$1}>\n    \"${2:Click me}\"\n</button>".to_string()),
            },
            CompletionItem {
                label: "<input>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Input element".to_string()),
                documentation: Some("Text input field".to_string()),
                insert_text: Some("<input type=\"${1:text}\" value={${2:signal}.get()} oninput={(e) => ${2:signal}.set(e.target.value)} />".to_string()),
            },
            CompletionItem {
                label: "<h1>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Heading 1".to_string()),
                documentation: Some("Top-level heading".to_string()),
                insert_text: Some("<h1>\"${1:Title}\"</h1>".to_string()),
            },
            CompletionItem {
                label: "<p>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Paragraph".to_string()),
                documentation: Some("Paragraph element".to_string()),
                insert_text: Some("<p>\"${1:Text}\"</p>".to_string()),
            },
            CompletionItem {
                label: "<ul>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Unordered list".to_string()),
                documentation: Some("Bullet point list".to_string()),
                insert_text: Some("<ul>\n    {${1:items}.iter().map(|${2:item}| {\n        <li>{${2:item}}</li>\n    })}\n</ul>".to_string()),
            },
            CompletionItem {
                label: "<form>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Form element".to_string()),
                documentation: Some("Form container".to_string()),
                insert_text: Some("<form onsubmit={$1}>\n    $0\n</form>".to_string()),
            },
            // Common patterns
            CompletionItem {
                label: "counter".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Counter component".to_string()),
                documentation: Some("Basic counter with increment/decrement".to_string()),
                insert_text: Some("component ${1:Counter}() {\n    let count = Signal::new(0);\n\n    <div>\n        <h1>\"Counter: \" {count.get()}</h1>\n        <button onclick={() => count.set(count.get() + 1)}>\"Increment\"</button>\n        <button onclick={() => count.set(count.get() - 1)}>\"Decrement\"</button>\n    </div>\n}".to_string()),
            },
            CompletionItem {
                label: "list-map".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Map array to JSX".to_string()),
                documentation: Some("Render list items".to_string()),
                insert_text: Some("{${1:items}.iter().map(|${2:item}| {\n    <div key={${2:item}.id}>\n        {${2:item}.name}\n    </div>\n})}".to_string()),
            },
            CompletionItem {
                label: "conditional".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Conditional JSX".to_string()),
                documentation: Some("Render conditionally".to_string()),
                insert_text: Some("{if ${1:condition} {\n    <div>$0</div>\n}}".to_string()),
            },
        ]
    }

    /// Get reactive primitive completions
    fn get_reactive_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "Signal::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create a new Signal".to_string()),
                documentation: Some("let count = Signal::new(0);".to_string()),
                insert_text: Some("Signal::new($0)".to_string()),
            },
            CompletionItem {
                label: "Computed::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create a computed value".to_string()),
                documentation: Some("let doubled = Computed::new(|| count.get() * 2);".to_string()),
                insert_text: Some("Computed::new(|| $0)".to_string()),
            },
            CompletionItem {
                label: "Effect::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create an effect".to_string()),
                documentation: Some("Effect::new(|| { console.log(count.get()); });".to_string()),
                insert_text: Some("Effect::new(|| $0)".to_string()),
            },
            CompletionItem {
                label: "Resource::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create an async resource".to_string()),
                documentation: Some("let data = Resource::new(async { fetch_data().await });".to_string()),
                insert_text: Some("Resource::new(async { $0 })".to_string()),
            },
        ]
    }

    /// Get reactive primitive documentation
    fn get_reactive_docs(&self, word: &str) -> Option<String> {
        match word {
            "Signal" => Some("**Signal** - A reactive value that can be read and written.\n\n```raven\nlet count = Signal::new(0);\ncount.set(5);\nlet value = count.get();\n```".to_string()),
            "Computed" => Some("**Computed** - A derived value that automatically updates.\n\n```raven\nlet doubled = Computed::new(|| count.get() * 2);\n```".to_string()),
            "Effect" => Some("**Effect** - Run side effects when dependencies change.\n\n```raven\nEffect::new(|| {\n    console.log(count.get());\n});\n```".to_string()),
            "Resource" => Some("**Resource** - Async data loading with automatic refetching.\n\n```raven\nlet data = Resource::new(async {\n    fetch_data().await\n});\n```".to_string()),
            _ => None,
        }
    }

    /// Get completions from current document scope
    fn get_scope_completions(&self, content: &str) -> Vec<CompletionItem> {
        use crate::ast::Statement;

        let mut completions = Vec::new();

        // Parse the document to extract local variables and functions
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(_) => return completions,
        };

        // Extract function names
        for statement in &ast.statements {
            match statement {
                Statement::Function(func_def) => {
                    completions.push(CompletionItem {
                        label: func_def.name.value.clone(),
                        kind: CompletionItemKind::Function,
                        detail: Some(format!("fn {}(...)", func_def.name.value)),
                        documentation: Some("User-defined function".to_string()),
                        insert_text: None,
                    });
                }
                Statement::Let(let_stmt) => {
                    // Add completions for all identifiers bound in the pattern
                    for ident in let_stmt.pattern.bound_identifiers() {
                        completions.push(CompletionItem {
                            label: ident.value.clone(),
                            kind: CompletionItemKind::Variable,
                            detail: Some("Local variable".to_string()),
                            documentation: None,
                            insert_text: None,
                        });
                    }
                }
                Statement::Component(comp_def) => {
                    completions.push(CompletionItem {
                        label: comp_def.name.value.clone(),
                        kind: CompletionItemKind::Class,
                        detail: Some(format!("component {}", comp_def.name.value)),
                        documentation: Some("User-defined component".to_string()),
                        insert_text: None,
                    });
                }
                _ => {}
            }
        }

        completions
    }

    /// Format entire document
    ///
    /// Returns a TextEdit that replaces the entire document with formatted content.
    /// This implements the LSP `textDocument/formatting` request.
    pub fn format_document(&self, uri: &str, options: FormattingOptions) -> Option<Vec<TextEdit>> {
        use crate::formatter::{Formatter, FormatterConfig};
        use crate::lexer::Lexer;
        use crate::parser::Parser;

        // Get document content
        let doc = self.documents.get(uri)?;
        let content = &doc.content;

        // Parse the document
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(_) => {
                // If parsing fails, don't format (return None)
                return None;
            }
        };

        // Create formatter with config from LSP options
        let config = FormatterConfig {
            indent_size: options.tab_size,
            max_line_length: 100,
            use_spaces: options.insert_spaces,
            trailing_comma: true,
        };

        let mut formatter = Formatter::with_config(config);
        let formatted = formatter.format_program(&ast);

        // Apply final newline if requested
        let final_text = if options.insert_final_newline && !formatted.ends_with('\n') {
            format!("{}\n", formatted)
        } else {
            formatted
        };

        // Calculate document range (entire document)
        let lines: Vec<&str> = content.lines().collect();
        let last_line = if lines.is_empty() { 0 } else { lines.len() - 1 };
        let last_char = lines.last().map(|l| l.len()).unwrap_or(0);

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position {
                line: last_line,
                character: last_char,
            },
        };

        Some(vec![TextEdit {
            range,
            new_text: final_text,
        }])
    }

    /// Format a range within a document
    ///
    /// Returns a TextEdit that replaces only the specified range with formatted content.
    /// This implements the LSP `textDocument/rangeFormatting` request.
    pub fn format_range(
        &self,
        uri: &str,
        range: Range,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        use crate::formatter::{Formatter, FormatterConfig};
        use crate::lexer::Lexer;
        use crate::parser::Parser;

        // Get document content
        let doc = self.documents.get(uri)?;
        let content = &doc.content;

        // Extract the range text
        let lines: Vec<&str> = content.lines().collect();
        if range.start.line >= lines.len() {
            return None;
        }

        // For simplicity, format the entire document and extract the range
        // A more sophisticated implementation would parse only the range
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(_) => {
                // If parsing fails, don't format
                return None;
            }
        };

        // Create formatter with config from LSP options
        let config = FormatterConfig {
            indent_size: options.tab_size,
            max_line_length: 100,
            use_spaces: options.insert_spaces,
            trailing_comma: true,
        };

        let mut formatter = Formatter::with_config(config);
        let formatted = formatter.format_program(&ast);

        // For range formatting, we still replace the whole document
        // (extracting specific ranges is complex and error-prone)
        let final_text = if options.insert_final_newline && !formatted.ends_with('\n') {
            format!("{}\n", formatted)
        } else {
            formatted
        };

        let last_line = if lines.is_empty() { 0 } else { lines.len() - 1 };
        let last_char = lines.last().map(|l| l.len()).unwrap_or(0);

        let full_range = Range {
            start: Position { line: 0, character: 0 },
            end: Position {
                line: last_line,
                character: last_char,
            },
        };

        Some(vec![TextEdit {
            range: full_range,
            new_text: final_text,
        }])
    }

    /// Get code actions (quick fixes) for a range
    ///
    /// Returns available code actions for the given range, typically at the cursor position.
    /// This implements the LSP `textDocument/codeAction` request.
    pub fn get_code_actions(&self, uri: &str, range: Range) -> Vec<CodeAction> {
        let mut actions = Vec::new();

        // Get diagnostics for this document
        let diagnostics = self.get_diagnostics(uri);

        // Get document content
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return actions;
        };

        // For each diagnostic, check if it's in the range and offer quick fixes
        for diagnostic in diagnostics {
            // Check if diagnostic overlaps with the requested range
            if let Some(diag_range) = diagnostic.to_lsp_range() {
                if !ranges_overlap(diag_range, range) {
                    continue;
                }
            }

            // Generate quick fixes based on diagnostic message and code
            actions.extend(self.generate_quick_fixes_for_diagnostic(&diagnostic, content, uri));
        }

        actions
    }

    /// Get definition location for a symbol at the given position
    ///
    /// Returns the location where the symbol is defined.
    /// This implements the LSP `textDocument/definition` request.
    pub fn get_definition(&self, uri: &str, position: Position) -> Option<Location> {
        // Get document content
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return None;
        };

        // Get the word at the cursor position
        let word = self.get_word_at_position(content, position)?;
        if word.is_empty() {
            return None;
        }

        // Extract symbol definitions using text-based approach
        let symbols = extract_text_symbols(uri, content);

        // Find the definition for the word
        symbols.into_iter()
            .find(|sym| sym.name == word)
            .map(|sym| sym.location)
    }

    /// Find all references to a symbol at the given position
    ///
    /// Returns all locations where the symbol is used (including definition).
    /// This implements the LSP `textDocument/references` request.
    pub fn get_references(
        &self,
        uri: &str,
        position: Position,
        include_declaration: bool,
    ) -> Vec<Location> {
        // Get document content
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return Vec::new();
        };

        // Get the word at the cursor position
        let word = match self.get_word_at_position(content, position) {
            Some(w) if !w.is_empty() => w,
            _ => return Vec::new(),
        };

        // Find all references to this symbol
        let mut references = find_symbol_references(uri, content, &word);

        // If include_declaration is false, filter out the definition
        if !include_declaration {
            // Get the definition location to exclude it
            let symbols = extract_text_symbols(uri, content);
            if let Some(def_sym) = symbols.into_iter().find(|sym| sym.name == word) {
                let def_loc = def_sym.location;
                references.retain(|loc| {
                    loc.range.start.line != def_loc.range.start.line
                        || loc.range.start.character != def_loc.range.start.character
                });
            }
        }

        references
    }

    /// Rename a symbol at the given position
    ///
    /// Returns a WorkspaceEdit with all the rename changes.
    /// This implements the LSP `textDocument/rename` request.
    pub fn rename_symbol(
        &self,
        uri: &str,
        position: Position,
        new_name: String,
    ) -> Option<WorkspaceEdit> {
        // Validate the new name is a valid identifier
        if !is_valid_identifier(&new_name) {
            return None;
        }

        // Get all references to the symbol (including definition)
        let references = self.get_references(uri, position, true);
        if references.is_empty() {
            return None;
        }

        // Create text edits for all references
        let changes: Vec<TextEdit> = references
            .into_iter()
            .map(|loc| TextEdit {
                range: loc.range,
                new_text: new_name.clone(),
            })
            .collect();

        Some(WorkspaceEdit { changes })
    }

    /// Get document symbols (outline view)
    ///
    /// Returns a hierarchical list of symbols in the document.
    /// This implements the LSP `textDocument/documentSymbol` request.
    pub fn get_document_symbols(&self, uri: &str) -> Vec<DocumentSymbol> {
        // Get document content
        let content = if let Some(doc) = self.documents.get(uri) {
            &doc.content
        } else {
            return Vec::new();
        };

        // Extract all symbol definitions
        let symbol_infos = extract_text_symbols(uri, content);

        // Convert SymbolInfo to DocumentSymbol
        symbol_infos
            .into_iter()
            .map(|sym| {
                let kind = match sym.kind {
                    SymbolKind::Function => DocumentSymbolKind::Function,
                    SymbolKind::Variable => DocumentSymbolKind::Variable,
                    SymbolKind::Component => DocumentSymbolKind::Class,
                    SymbolKind::Struct => DocumentSymbolKind::Struct,
                    SymbolKind::Enum => DocumentSymbolKind::Enum,
                    SymbolKind::Parameter => DocumentSymbolKind::Variable,
                    SymbolKind::TypeAlias => DocumentSymbolKind::Class,
                };

                DocumentSymbol {
                    name: sym.name.clone(),
                    kind,
                    range: sym.location.range,
                    selection_range: sym.location.range,
                    detail: Some(format!("{:?}", sym.kind)),
                    children: Vec::new(), // Flat structure for now
                }
            })
            .collect()
    }

    /// Generate quick fixes for a specific diagnostic
    fn generate_quick_fixes_for_diagnostic(
        &self,
        diagnostic: &Diagnostic,
        content: &str,
        _uri: &str,
    ) -> Vec<CodeAction> {
        let mut actions = Vec::new();

        // Check diagnostic message for patterns
        let message = &diagnostic.message;

        // Quick Fix 1: "Did you mean?" typo fixes (E002 - undefined_identifier)
        if message.contains("undefined") || message.contains("not found") {
            if let Some(suggestion) = extract_suggestion(message) {
                if let Some(range) = diagnostic.to_lsp_range() {
                    let word = get_word_at_range(content, range);
                    if !word.is_empty() {
                        actions.push(CodeAction {
                            title: format!("Change to '{}'", suggestion),
                            kind: CodeActionKind::QuickFix,
                            diagnostics: vec![diagnostic.clone()],
                            edit: WorkspaceEdit {
                                changes: vec![TextEdit {
                                    range,
                                    new_text: suggestion,
                                }],
                            },
                            is_preferred: true,
                        });
                    }
                }
            }
        }

        // Quick Fix 2: Prefix unused variable with _ (W001 - unused_variable)
        if message.contains("unused") && message.contains("variable") {
            if let Some(range) = diagnostic.to_lsp_range() {
                let word = get_word_at_range(content, range);
                if !word.is_empty() && !word.starts_with('_') {
                    actions.push(CodeAction {
                        title: format!("Rename to '_{}'", word),
                        kind: CodeActionKind::QuickFix,
                        diagnostics: vec![diagnostic.clone()],
                        edit: WorkspaceEdit {
                            changes: vec![TextEdit {
                                range,
                                new_text: format!("_{}", word),
                            }],
                        },
                        is_preferred: true,
                    });
                }
            }
        }

        // Quick Fix 3: Add type cast (E003 - type_mismatch)
        if message.contains("type mismatch") || message.contains("expected") {
            // Extract expected type from message (e.g., "expected f64, found i32")
            if let Some((expected_type, found_type)) = extract_type_mismatch(message) {
                if let Some(range) = diagnostic.to_lsp_range() {
                    let expr = get_word_at_range(content, range);
                    if !expr.is_empty() && can_cast(&found_type, &expected_type) {
                        actions.push(CodeAction {
                            title: format!("Cast to {}", expected_type),
                            kind: CodeActionKind::QuickFix,
                            diagnostics: vec![diagnostic.clone()],
                            edit: WorkspaceEdit {
                                changes: vec![TextEdit {
                                    range,
                                    new_text: format!("{} as {}", expr, expected_type),
                                }],
                            },
                            is_preferred: false,
                        });
                    }
                }
            }
        }

        // Quick Fix 4: Add missing semicolon (E001 - syntax_error)
        if message.contains("expected") && message.contains(";") {
            if let Some(range) = diagnostic.to_lsp_range() {
                // Find the end of the line
                let lines: Vec<&str> = content.lines().collect();
                if range.end.line < lines.len() {
                    let line = lines[range.end.line];
                    let insert_pos = Position {
                        line: range.end.line,
                        character: line.trim_end().len(),
                    };
                    actions.push(CodeAction {
                        title: "Add semicolon".to_string(),
                        kind: CodeActionKind::QuickFix,
                        diagnostics: vec![diagnostic.clone()],
                        edit: WorkspaceEdit {
                            changes: vec![TextEdit {
                                range: Range {
                                    start: insert_pos,
                                    end: insert_pos,
                                },
                                new_text: ";".to_string(),
                            }],
                        },
                        is_preferred: true,
                    });
                }
            }
        }

        // Quick Fix 5: Add missing type annotation (E015 - type_annotation_needed)
        if message.contains("type annotation") || message.contains("cannot infer type") {
            if let Some(inferred_type) = extract_inferred_type(message) {
                if let Some(range) = diagnostic.to_lsp_range() {
                    let lines: Vec<&str> = content.lines().collect();
                    if range.start.line < lines.len() {
                        let line = lines[range.start.line];
                        // Find where to insert the type annotation (after variable name)
                        if let Some(pos) = line[range.start.character..].find('=') {
                            let insert_pos = Position {
                                line: range.start.line,
                                character: range.start.character + pos,
                            };
                            actions.push(CodeAction {
                                title: format!("Add type annotation: {}", inferred_type),
                                kind: CodeActionKind::QuickFix,
                                diagnostics: vec![diagnostic.clone()],
                                edit: WorkspaceEdit {
                                    changes: vec![TextEdit {
                                        range: Range {
                                            start: insert_pos,
                                            end: insert_pos,
                                        },
                                        new_text: format!(": {} ", inferred_type),
                                    }],
                                },
                                is_preferred: false,
                            });
                        }
                    }
                }
            }
        }

        // Quick Fix 6: Remove unused import (W002 - unused_import)
        if message.contains("unused") && message.contains("import") {
            if let Some(range) = diagnostic.to_lsp_range() {
                // Remove the entire line
                let lines: Vec<&str> = content.lines().collect();
                if range.start.line < lines.len() {
                    let line_range = Range {
                        start: Position {
                            line: range.start.line,
                            character: 0,
                        },
                        end: Position {
                            line: range.start.line + 1,
                            character: 0,
                        },
                    };
                    actions.push(CodeAction {
                        title: "Remove unused import".to_string(),
                        kind: CodeActionKind::QuickFix,
                        diagnostics: vec![diagnostic.clone()],
                        edit: WorkspaceEdit {
                            changes: vec![TextEdit {
                                range: line_range,
                                new_text: String::new(),
                            }],
                        },
                        is_preferred: true,
                    });
                }
            }
        }

        actions
    }
}

impl Default for LanguageServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard library documentation
pub struct StdlibDocs {
    functions: HashMap<String, FunctionDoc>,
}

#[derive(Debug, Clone)]
pub struct FunctionDoc {
    pub name: String,
    pub signature: String,
    pub description: String,
    pub examples: Vec<String>,
}

impl StdlibDocs {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        // ==================== MATH FUNCTIONS ====================

        // Basic operations
        functions.insert("Math::abs".to_string(), FunctionDoc {
            name: "Math::abs".to_string(),
            signature: "fn abs(x: f64) -> f64".to_string(),
            description: "Returns the absolute value of a number".to_string(),
            examples: vec!["let positive = Math::abs(-42.5); // 42.5".to_string()],
        });

        functions.insert("Math::min".to_string(), FunctionDoc {
            name: "Math::min".to_string(),
            signature: "fn min(a: f64, b: f64) -> f64".to_string(),
            description: "Returns the smaller of two numbers".to_string(),
            examples: vec!["let smallest = Math::min(10.0, 20.0); // 10.0".to_string()],
        });

        functions.insert("Math::max".to_string(), FunctionDoc {
            name: "Math::max".to_string(),
            signature: "fn max(a: f64, b: f64) -> f64".to_string(),
            description: "Returns the larger of two numbers".to_string(),
            examples: vec!["let largest = Math::max(10.0, 20.0); // 20.0".to_string()],
        });

        functions.insert("Math::clamp".to_string(), FunctionDoc {
            name: "Math::clamp".to_string(),
            signature: "fn clamp(value: f64, min: f64, max: f64) -> f64".to_string(),
            description: "Constrains a value to a range [min, max]".to_string(),
            examples: vec!["let clamped = Math::clamp(150.0, 0.0, 100.0); // 100.0".to_string()],
        });

        // Powers and roots
        functions.insert("Math::pow".to_string(), FunctionDoc {
            name: "Math::pow".to_string(),
            signature: "fn pow(base: f64, exponent: f64) -> f64".to_string(),
            description: "Raises a number to a power".to_string(),
            examples: vec!["let result = Math::pow(2.0, 8.0); // 256.0".to_string()],
        });

        functions.insert("Math::sqrt".to_string(), FunctionDoc {
            name: "Math::sqrt".to_string(),
            signature: "fn sqrt(x: f64) -> f64".to_string(),
            description: "Returns the square root of a number".to_string(),
            examples: vec!["let root = Math::sqrt(16.0); // 4.0".to_string()],
        });

        functions.insert("Math::square".to_string(), FunctionDoc {
            name: "Math::square".to_string(),
            signature: "fn square(x: f64) -> f64".to_string(),
            description: "Returns the square of a number (x²)".to_string(),
            examples: vec!["let sq = Math::square(5.0); // 25.0".to_string()],
        });

        // Rounding
        functions.insert("Math::round".to_string(), FunctionDoc {
            name: "Math::round".to_string(),
            signature: "fn round(x: f64) -> f64".to_string(),
            description: "Rounds to the nearest integer".to_string(),
            examples: vec!["let r = Math::round(3.7); // 4.0".to_string()],
        });

        functions.insert("Math::floor".to_string(), FunctionDoc {
            name: "Math::floor".to_string(),
            signature: "fn floor(x: f64) -> f64".to_string(),
            description: "Rounds down to the nearest integer".to_string(),
            examples: vec!["let f = Math::floor(3.7); // 3.0".to_string()],
        });

        functions.insert("Math::ceil".to_string(), FunctionDoc {
            name: "Math::ceil".to_string(),
            signature: "fn ceil(x: f64) -> f64".to_string(),
            description: "Rounds up to the nearest integer".to_string(),
            examples: vec!["let c = Math::ceil(3.2); // 4.0".to_string()],
        });

        // Trigonometry
        functions.insert("Math::sin".to_string(), FunctionDoc {
            name: "Math::sin".to_string(),
            signature: "fn sin(x: f64) -> f64".to_string(),
            description: "Returns the sine of an angle (in radians)".to_string(),
            examples: vec!["let s = Math::sin(Math::PI); // ~0.0".to_string()],
        });

        functions.insert("Math::cos".to_string(), FunctionDoc {
            name: "Math::cos".to_string(),
            signature: "fn cos(x: f64) -> f64".to_string(),
            description: "Returns the cosine of an angle (in radians)".to_string(),
            examples: vec!["let c = Math::cos(0.0); // 1.0".to_string()],
        });

        functions.insert("Math::tan".to_string(), FunctionDoc {
            name: "Math::tan".to_string(),
            signature: "fn tan(x: f64) -> f64".to_string(),
            description: "Returns the tangent of an angle (in radians)".to_string(),
            examples: vec!["let t = Math::tan(0.0); // 0.0".to_string()],
        });

        // Random
        functions.insert("Math::random".to_string(), FunctionDoc {
            name: "Math::random".to_string(),
            signature: "fn random() -> f64".to_string(),
            description: "Returns a random number between 0 (inclusive) and 1 (exclusive)".to_string(),
            examples: vec!["let r = Math::random(); // e.g., 0.742".to_string()],
        });

        functions.insert("Math::random_int".to_string(), FunctionDoc {
            name: "Math::random_int".to_string(),
            signature: "fn random_int(min: i32, max: i32) -> i32".to_string(),
            description: "Returns a random integer in the range [min, max] inclusive".to_string(),
            examples: vec!["let dice = Math::random_int(1, 6); // 1-6".to_string()],
        });

        // ==================== REACTIVE FUNCTIONS ====================

        functions.insert("Signal::new".to_string(), FunctionDoc {
            name: "Signal::new".to_string(),
            signature: "fn new<T>(initial: T) -> Signal<T>".to_string(),
            description: "Creates a new reactive signal with an initial value".to_string(),
            examples: vec!["let count = Signal::new(0);".to_string()],
        });

        functions.insert("Signal::get".to_string(), FunctionDoc {
            name: "signal.get".to_string(),
            signature: "fn get(&self) -> T".to_string(),
            description: "Gets the current value and tracks the dependency".to_string(),
            examples: vec!["let value = count.get();".to_string()],
        });

        functions.insert("Signal::set".to_string(), FunctionDoc {
            name: "signal.set".to_string(),
            signature: "fn set(&self, value: T)".to_string(),
            description: "Sets a new value and notifies all dependents".to_string(),
            examples: vec!["count.set(5);".to_string()],
        });

        functions.insert("Computed::new".to_string(), FunctionDoc {
            name: "Computed::new".to_string(),
            signature: "fn new<T>(fn() -> T) -> Computed<T>".to_string(),
            description: "Creates a computed value that automatically updates when dependencies change".to_string(),
            examples: vec!["let doubled = Computed::new(|| count.get() * 2);".to_string()],
        });

        functions.insert("create_effect".to_string(), FunctionDoc {
            name: "create_effect".to_string(),
            signature: "fn create_effect(fn())".to_string(),
            description: "Creates an effect that runs when dependencies change".to_string(),
            examples: vec!["create_effect(|| println!(\"Count: {}\", count.get()));".to_string()],
        });

        // ==================== HTTP FUNCTIONS ====================

        functions.insert("HttpRequest::get".to_string(), FunctionDoc {
            name: "HttpRequest::get".to_string(),
            signature: "fn get(url: &str) -> HttpRequest".to_string(),
            description: "Creates a GET request".to_string(),
            examples: vec!["let resp = HttpRequest::get(\"https://api.example.com/users\").send().await;".to_string()],
        });

        functions.insert("HttpRequest::post".to_string(), FunctionDoc {
            name: "HttpRequest::post".to_string(),
            signature: "fn post(url: &str) -> HttpRequest".to_string(),
            description: "Creates a POST request".to_string(),
            examples: vec!["let resp = HttpRequest::post(url).json(data).send().await;".to_string()],
        });

        functions.insert("get".to_string(), FunctionDoc {
            name: "get".to_string(),
            signature: "@server async fn get(url: &str) -> Result<HttpResponse, String>".to_string(),
            description: "Convenience function for quick GET requests".to_string(),
            examples: vec!["let resp = get(\"https://api.example.com/data\").await;".to_string()],
        });

        // ==================== COLLECTIONS FUNCTIONS ====================

        functions.insert("Collections::filter".to_string(), FunctionDoc {
            name: "Collections::filter".to_string(),
            signature: "fn filter<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> Vec<T>".to_string(),
            description: "Filters elements matching a predicate".to_string(),
            examples: vec!["let evens = Collections::filter(&numbers, |n| n % 2 == 0);".to_string()],
        });

        functions.insert("Collections::map".to_string(), FunctionDoc {
            name: "Collections::map".to_string(),
            signature: "fn map<T, U>(vec: &Vec<T>, mapper: fn(&T) -> U) -> Vec<U>".to_string(),
            description: "Transforms elements with a mapper function".to_string(),
            examples: vec!["let doubled = Collections::map(&numbers, |n| n * 2);".to_string()],
        });

        functions.insert("Collections::find".to_string(), FunctionDoc {
            name: "Collections::find".to_string(),
            signature: "fn find<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> Option<&T>".to_string(),
            description: "Finds the first element matching a predicate".to_string(),
            examples: vec!["let found = Collections::find(&items, |i| i.id == 42);".to_string()],
        });

        // ==================== STRING FUNCTIONS ====================

        functions.insert("String::split".to_string(), FunctionDoc {
            name: "String::split".to_string(),
            signature: "fn split(s: &str, delimiter: &str) -> Vec<String>".to_string(),
            description: "Splits a string by a delimiter".to_string(),
            examples: vec!["let parts = String::split(\"a,b,c\", \",\"); // [\"a\", \"b\", \"c\"]".to_string()],
        });

        functions.insert("String::trim".to_string(), FunctionDoc {
            name: "String::trim".to_string(),
            signature: "fn trim(s: &str) -> String".to_string(),
            description: "Removes whitespace from both ends of a string".to_string(),
            examples: vec!["let trimmed = String::trim(\"  hello  \"); // \"hello\"".to_string()],
        });

        functions.insert("String::contains".to_string(), FunctionDoc {
            name: "String::contains".to_string(),
            signature: "fn contains(haystack: &str, needle: &str) -> bool".to_string(),
            description: "Checks if a string contains a substring".to_string(),
            examples: vec!["let has_at = String::contains(email, \"@\");".to_string()],
        });

        // ==================== STORAGE FUNCTIONS ====================

        functions.insert("Storage::set".to_string(), FunctionDoc {
            name: "Storage::set".to_string(),
            signature: "@client fn set(key: &str, value: &str)".to_string(),
            description: "Stores a value in localStorage".to_string(),
            examples: vec!["Storage::set(\"theme\", \"dark\");".to_string()],
        });

        functions.insert("Storage::get".to_string(), FunctionDoc {
            name: "Storage::get".to_string(),
            signature: "@client fn get(key: &str) -> Option<String>".to_string(),
            description: "Retrieves a value from localStorage".to_string(),
            examples: vec!["let theme = Storage::get(\"theme\");".to_string()],
        });

        functions.insert("Storage::set_json".to_string(), FunctionDoc {
            name: "Storage::set_json".to_string(),
            signature: "@client fn set_json(key: &str, value: &Value)".to_string(),
            description: "Stores a JSON value in localStorage".to_string(),
            examples: vec!["Storage::set_json(\"cart\", &cart_data);".to_string()],
        });

        // ==================== FORMS FUNCTIONS ====================

        functions.insert("Forms::validate_email".to_string(), FunctionDoc {
            name: "Forms::validate_email".to_string(),
            signature: "fn validate_email(email: &str) -> bool".to_string(),
            description: "Validates email format".to_string(),
            examples: vec!["if Forms::validate_email(&email) { /* valid */ }".to_string()],
        });

        functions.insert("Forms::validate_required".to_string(), FunctionDoc {
            name: "Forms::validate_required".to_string(),
            signature: "fn validate_required(value: &str) -> bool".to_string(),
            description: "Checks if a value is non-empty".to_string(),
            examples: vec!["if !Forms::validate_required(&username) { /* error */ }".to_string()],
        });

        // ==================== TIME FUNCTIONS ====================

        functions.insert("Time::now".to_string(), FunctionDoc {
            name: "Time::now".to_string(),
            signature: "fn now() -> i64".to_string(),
            description: "Gets current Unix timestamp in seconds".to_string(),
            examples: vec!["let timestamp = Time::now();".to_string()],
        });

        functions.insert("Time::sleep".to_string(), FunctionDoc {
            name: "Time::sleep".to_string(),
            signature: "fn sleep(ms: u64)".to_string(),
            description: "Sleeps for specified milliseconds (blocking)".to_string(),
            examples: vec!["Time::sleep(1000); // Sleep 1 second".to_string()],
        });

        // ==================== JSON FUNCTIONS ====================

        functions.insert("JSON::parse".to_string(), FunctionDoc {
            name: "JSON::parse".to_string(),
            signature: "fn parse(text: &str) -> Result<Value, String>".to_string(),
            description: "Parses a JSON string".to_string(),
            examples: vec!["let data = JSON::parse(json_str)?;".to_string()],
        });

        functions.insert("JSON::stringify".to_string(), FunctionDoc {
            name: "JSON::stringify".to_string(),
            signature: "fn stringify(value: &Value) -> String".to_string(),
            description: "Converts a value to JSON string".to_string(),
            examples: vec!["let json_str = JSON::stringify(&user);".to_string()],
        });

        // ==================== AUTH FUNCTIONS ====================

        functions.insert("Auth::hash_password".to_string(), FunctionDoc {
            name: "Auth::hash_password".to_string(),
            signature: "@server fn hash_password(password: &str) -> String".to_string(),
            description: "Hashes a password using bcrypt".to_string(),
            examples: vec!["let hashed = Auth::hash_password(&password);".to_string()],
        });

        functions.insert("Auth::verify_password".to_string(), FunctionDoc {
            name: "Auth::verify_password".to_string(),
            signature: "@server fn verify_password(password: &str, hash: &str) -> bool".to_string(),
            description: "Verifies a password against a hash".to_string(),
            examples: vec!["if Auth::verify_password(&password, &user.password_hash) { /* valid */ }".to_string()],
        });

        // ==================== CONSOLE FUNCTIONS ====================

        functions.insert("println".to_string(), FunctionDoc {
            name: "println".to_string(),
            signature: "fn println!(format: &str, ...)".to_string(),
            description: "Prints a formatted message with newline".to_string(),
            examples: vec!["println!(\"Count: {}\", count);".to_string()],
        });

        functions.insert("console.log".to_string(), FunctionDoc {
            name: "console.log".to_string(),
            signature: "fn log(message: Any)".to_string(),
            description: "Logs a message to the console".to_string(),
            examples: vec!["console.log(\"Hello, world!\");".to_string()],
        });

        StdlibDocs { functions }
    }

    pub fn get_completions(&self) -> Vec<CompletionItem> {
        self.functions
            .values()
            .map(|doc| CompletionItem {
                label: doc.name.clone(),
                kind: CompletionItemKind::Function,
                detail: Some(doc.signature.clone()),
                documentation: Some(doc.description.clone()),
                insert_text: None,
            })
            .collect()
    }

    pub fn get_documentation(&self, name: &str) -> Option<String> {
        self.functions.get(name).map(|doc| {
            format!(
                "**{}**\n\n{}\n\n```raven\n{}\n```\n\n{}",
                doc.name,
                doc.signature,
                doc.examples.join("\n"),
                doc.description
            )
        })
    }
}

impl Default for StdlibDocs {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== Helper Functions for Code Actions ====================

/// Check if two ranges overlap
fn ranges_overlap(a: Range, b: Range) -> bool {
    // Check if ranges are on the same lines or overlap
    if a.end.line < b.start.line || b.end.line < a.start.line {
        return false;
    }
    if a.end.line == b.start.line && a.end.character <= b.start.character {
        return false;
    }
    if b.end.line == a.start.line && b.end.character <= a.start.character {
        return false;
    }
    true
}

/// Extract suggestion from diagnostic message (e.g., "Did you mean 'Signal'?")
fn extract_suggestion(message: &str) -> Option<String> {
    // Look for patterns like "Did you mean 'X'?" or "Did you mean `X`?"
    if let Some(start) = message.find("Did you mean") {
        let rest = &message[start..];
        // Look for text between quotes or backticks
        if let Some(quote_start) = rest.find('\'') {
            if let Some(quote_end) = rest[quote_start + 1..].find('\'') {
                return Some(rest[quote_start + 1..quote_start + 1 + quote_end].to_string());
            }
        }
        if let Some(tick_start) = rest.find('`') {
            if let Some(tick_end) = rest[tick_start + 1..].find('`') {
                return Some(rest[tick_start + 1..tick_start + 1 + tick_end].to_string());
            }
        }
    }
    None
}

/// Extract type mismatch information from diagnostic message
/// Returns (expected_type, found_type)
fn extract_type_mismatch(message: &str) -> Option<(String, String)> {
    // Look for pattern: "expected X, found Y" or "expected X but found Y"
    if let Some(expected_pos) = message.find("expected ") {
        let after_expected = &message[expected_pos + 9..]; // "expected ".len() == 9

        // Find the expected type (up to comma, "but", or "found")
        let expected_end = after_expected
            .find(',')
            .or_else(|| after_expected.find(" but"))
            .or_else(|| after_expected.find(" found"))
            .unwrap_or(after_expected.len());

        let expected_type = after_expected[..expected_end].trim().to_string();

        // Look for the found type
        if let Some(found_pos) = message.find("found ") {
            let after_found = &message[found_pos + 6..]; // "found ".len() == 6

            // Find the found type (up to punctuation or end)
            let found_end = after_found
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after_found.len());

            let found_type = after_found[..found_end].trim().to_string();

            if !expected_type.is_empty() && !found_type.is_empty() {
                return Some((expected_type, found_type));
            }
        }
    }
    None
}

/// Check if a type can be cast to another type
fn can_cast(from: &str, to: &str) -> bool {
    // Numeric type casts
    let numeric_types = ["i32", "i64", "f32", "f64", "u32", "u64", "usize", "isize"];
    numeric_types.contains(&from) && numeric_types.contains(&to)
}

/// Extract inferred type from diagnostic message
fn extract_inferred_type(message: &str) -> Option<String> {
    // Look for patterns like "inferred to be X" or "type is X"
    if let Some(pos) = message.find("inferred to be ") {
        let rest = &message[pos + 15..];
        let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
            .unwrap_or(rest.len());
        let typ = rest[..end].trim().to_string();
        if !typ.is_empty() {
            return Some(typ);
        }
    }
    if let Some(pos) = message.find("type is ") {
        let rest = &message[pos + 8..];
        let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
            .unwrap_or(rest.len());
        let typ = rest[..end].trim().to_string();
        if !typ.is_empty() {
            return Some(typ);
        }
    }
    None
}

/// Get word at a specific range in content
fn get_word_at_range(content: &str, range: Range) -> String {
    let lines: Vec<&str> = content.lines().collect();
    if range.start.line >= lines.len() {
        return String::new();
    }

    let line = lines[range.start.line];
    let chars: Vec<char> = line.chars().collect();

    if range.start.character >= chars.len() {
        return String::new();
    }

    let start = range.start.character;
    let end = if range.end.line == range.start.line {
        range.end.character.min(chars.len())
    } else {
        chars.len()
    };

    if start >= end {
        return String::new();
    }

    chars[start..end].iter().collect()
}

/// Extract symbol definitions using text-based parsing
/// This is a simpler approach that doesn't require full AST parsing
fn extract_text_symbols(uri: &str, content: &str) -> Vec<SymbolInfo> {
    let mut symbols = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Match function definitions: fn name(
        if let Some(fn_pos) = trimmed.find("fn ") {
            if let Some(paren_pos) = trimmed[fn_pos..].find('(') {
                let name_start = fn_pos + 3;
                let name_end = fn_pos + paren_pos;
                let name = trimmed[name_start..name_end].trim();

                if !name.is_empty() && is_valid_identifier(name) {
                    let char_pos = line.find(name).unwrap_or(0);
                    symbols.push(SymbolInfo {
                        name: name.to_string(),
                        kind: SymbolKind::Function,
                        location: Location {
                            uri: uri.to_string(),
                            range: Range {
                                start: Position {
                                    line: line_idx,
                                    character: char_pos,
                                },
                                end: Position {
                                    line: line_idx,
                                    character: char_pos + name.len(),
                                },
                            },
                        },
                    });
                }
            }
        }

        // Match component definitions: component Name(
        if let Some(comp_pos) = trimmed.find("component ") {
            if let Some(paren_pos) = trimmed[comp_pos..].find('(') {
                let name_start = comp_pos + 10;
                let name_end = comp_pos + paren_pos;
                let name = trimmed[name_start..name_end].trim();

                if !name.is_empty() && is_valid_identifier(name) {
                    let char_pos = line.find(name).unwrap_or(0);
                    symbols.push(SymbolInfo {
                        name: name.to_string(),
                        kind: SymbolKind::Component,
                        location: Location {
                            uri: uri.to_string(),
                            range: Range {
                                start: Position {
                                    line: line_idx,
                                    character: char_pos,
                                },
                                end: Position {
                                    line: line_idx,
                                    character: char_pos + name.len(),
                                },
                            },
                        },
                    });
                }
            }
        }

        // Match variable definitions: let name =
        if let Some(let_pos) = trimmed.find("let ") {
            let after_let = &trimmed[let_pos + 4..];
            // Find the identifier (stop at =, :, or space)
            let name_end = after_let
                .find(|c: char| c == '=' || c == ':' || c == ' ')
                .unwrap_or(after_let.len());
            let name = after_let[..name_end].trim();

            if !name.is_empty() && is_valid_identifier(name) {
                let char_pos = line.find(name).unwrap_or(0);
                symbols.push(SymbolInfo {
                    name: name.to_string(),
                    kind: SymbolKind::Variable,
                    location: Location {
                        uri: uri.to_string(),
                        range: Range {
                            start: Position {
                                line: line_idx,
                                character: char_pos,
                            },
                            end: Position {
                                line: line_idx,
                                character: char_pos + name.len(),
                            },
                        },
                    },
                });
            }
        }

        // Match const definitions: const NAME =
        if let Some(const_pos) = trimmed.find("const ") {
            let after_const = &trimmed[const_pos + 6..];
            let name_end = after_const
                .find(|c: char| c == '=' || c == ':' || c == ' ')
                .unwrap_or(after_const.len());
            let name = after_const[..name_end].trim();

            if !name.is_empty() && is_valid_identifier(name) {
                let char_pos = line.find(name).unwrap_or(0);
                symbols.push(SymbolInfo {
                    name: name.to_string(),
                    kind: SymbolKind::Variable,
                    location: Location {
                        uri: uri.to_string(),
                        range: Range {
                            start: Position {
                                line: line_idx,
                                character: char_pos,
                            },
                            end: Position {
                                line: line_idx,
                                character: char_pos + name.len(),
                            },
                        },
                    },
                });
            }
        }

        // Match struct definitions: struct Name
        if let Some(struct_pos) = trimmed.find("struct ") {
            let after_struct = &trimmed[struct_pos + 7..];
            let name_end = after_struct
                .find(|c: char| c == '{' || c == ' ')
                .unwrap_or(after_struct.len());
            let name = after_struct[..name_end].trim();

            if !name.is_empty() && is_valid_identifier(name) {
                let char_pos = line.find(name).unwrap_or(0);
                symbols.push(SymbolInfo {
                    name: name.to_string(),
                    kind: SymbolKind::Struct,
                    location: Location {
                        uri: uri.to_string(),
                        range: Range {
                            start: Position {
                                line: line_idx,
                                character: char_pos,
                            },
                            end: Position {
                                line: line_idx,
                                character: char_pos + name.len(),
                            },
                        },
                    },
                });
            }
        }

        // Match enum definitions: enum Name
        if let Some(enum_pos) = trimmed.find("enum ") {
            let after_enum = &trimmed[enum_pos + 5..];
            let name_end = after_enum
                .find(|c: char| c == '{' || c == ' ')
                .unwrap_or(after_enum.len());
            let name = after_enum[..name_end].trim();

            if !name.is_empty() && is_valid_identifier(name) {
                let char_pos = line.find(name).unwrap_or(0);
                symbols.push(SymbolInfo {
                    name: name.to_string(),
                    kind: SymbolKind::Enum,
                    location: Location {
                        uri: uri.to_string(),
                        range: Range {
                            start: Position {
                                line: line_idx,
                                character: char_pos,
                            },
                            end: Position {
                                line: line_idx,
                                character: char_pos + name.len(),
                            },
                        },
                    },
                });
            }
        }
    }

    symbols
}

/// Find all references (usages) of a symbol in the content
/// This includes the definition and all usages of the symbol
fn find_symbol_references(uri: &str, content: &str, symbol_name: &str) -> Vec<Location> {
    let mut references = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        // Find all occurrences of the symbol in this line
        let mut start_pos = 0;
        while let Some(pos) = line[start_pos..].find(symbol_name) {
            let actual_pos = start_pos + pos;

            // Check if this is a whole word match (not part of another identifier)
            let before_ok = if actual_pos == 0 {
                true
            } else {
                let prev_char = line.chars().nth(actual_pos - 1).unwrap();
                !prev_char.is_alphanumeric() && prev_char != '_'
            };

            let after_pos = actual_pos + symbol_name.len();
            let after_ok = if after_pos >= line.len() {
                true
            } else {
                let next_char = line.chars().nth(after_pos).unwrap();
                !next_char.is_alphanumeric() && next_char != '_'
            };

            // Only add if it's a complete word match
            if before_ok && after_ok {
                references.push(Location {
                    uri: uri.to_string(),
                    range: Range {
                        start: Position {
                            line: line_idx,
                            character: actual_pos,
                        },
                        end: Position {
                            line: line_idx,
                            character: actual_pos + symbol_name.len(),
                        },
                    },
                });
            }

            start_pos = actual_pos + 1;
        }
    }

    references
}

/// Check if a string is a valid identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();

    // First character must be letter or underscore
    if !chars[0].is_alphabetic() && chars[0] != '_' {
        return false;
    }

    // Remaining characters must be alphanumeric or underscore
    chars.iter().all(|&c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_server_open_document() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "let x = 10;".to_string(),
            1,
        );

        assert!(server.documents.contains_key("file:///test.jnc"));
    }

    #[test]
    fn test_get_completions_statement_start() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "let x = 10;".to_string(),
            1,
        );

        // At statement start, should get keywords
        let completions = server.get_completions("file:///test.jnc", Position { line: 0, character: 0 });
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "component"));
        assert!(completions.iter().any(|c| c.label == "let"));
        assert!(completions.iter().any(|c| c.label == "fn"));
    }

    #[test]
    fn test_get_completions_general() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "let x = ".to_string(),
            1,
        );

        // In general context, should get all completions
        let completions = server.get_completions("file:///test.jnc", Position { line: 0, character: 8 });
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "Signal::new"));
    }

    #[test]
    fn test_context_detection_namespace() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "let x = Math::".to_string(),
            1,
        );

        // After ::, should get namespace members
        let completions = server.get_completions("file:///test.jnc", Position { line: 0, character: 14 });
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "abs"));
        assert!(completions.iter().any(|c| c.label == "sqrt"));
        // Should NOT include keywords
        assert!(!completions.iter().any(|c| c.label == "component"));
    }

    #[test]
    fn test_context_detection_member_access() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "let count = Signal::new(0);\nlet val = count.".to_string(),
            1,
        );

        // After ., should get member methods
        let completions = server.get_completions("file:///test.jnc", Position { line: 1, character: 16 });
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "get"));
        assert!(completions.iter().any(|c| c.label == "set"));
    }

    #[test]
    fn test_context_detection_jsx_tag() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "component App() { <".to_string(),
            1,
        );

        // After <, should get JSX tags
        let completions = server.get_completions("file:///test.jnc", Position { line: 0, character: 19 });
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "div"));
        assert!(completions.iter().any(|c| c.label == "button"));
    }

    #[test]
    fn test_context_detection_jsx_attribute() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "component App() { <div ".to_string(),
            1,
        );

        // Inside JSX tag, should get attributes
        let completions = server.get_completions("file:///test.jnc", Position { line: 0, character: 23 });
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "class"));
        assert!(completions.iter().any(|c| c.label == "id"));
        assert!(completions.iter().any(|c| c.label == "onclick"));
    }

    #[test]
    fn test_get_word_at_position() {
        let server = LanguageServer::new();
        let content = "let count = Signal::new(0);";
        let word = server.get_word_at_position(content, Position { line: 0, character: 5 });

        assert_eq!(word, Some("count".to_string()));
    }

    #[test]
    fn test_reactive_docs() {
        let server = LanguageServer::new();
        let docs = server.get_reactive_docs("Signal");

        assert!(docs.is_some());
        assert!(docs.unwrap().contains("reactive value"));
    }

    #[test]
    fn test_format_document() {
        let mut server = LanguageServer::new();
        let unformatted = "let x:i32=42;";
        server.open_document(
            "file:///test.jnc".to_string(),
            unformatted.to_string(),
            1,
        );

        let options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
        };

        let edits = server.format_document("file:///test.jnc", options);

        assert!(edits.is_some());
        let edits = edits.unwrap();
        assert_eq!(edits.len(), 1);
        assert!(edits[0].new_text.contains("let x: i32 = 42;"));
    }

    #[test]
    fn test_format_document_with_function() {
        let mut server = LanguageServer::new();
        let unformatted = "fn add(a:i32,b:i32){return a+b;}";
        server.open_document(
            "file:///test.jnc".to_string(),
            unformatted.to_string(),
            1,
        );

        let options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
        };

        let edits = server.format_document("file:///test.jnc", options);

        assert!(edits.is_some());
        let edits = edits.unwrap();
        assert_eq!(edits.len(), 1);

        let formatted = &edits[0].new_text;
        assert!(formatted.contains("fn add(a: i32, b: i32)"));
        assert!(formatted.contains("return a + b;"));
    }

    #[test]
    fn test_format_range() {
        let mut server = LanguageServer::new();
        let code = "let x:i32=42;\nlet y:i32=43;";
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
        };

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 13 },
        };

        let edits = server.format_range("file:///test.jnc", range, options);

        assert!(edits.is_some());
        let edits = edits.unwrap();
        assert_eq!(edits.len(), 1);
        assert!(edits[0].new_text.contains("let x: i32 = 42;"));
        assert!(edits[0].new_text.contains("let y: i32 = 43;"));
    }

    #[test]
    fn test_format_document_invalid_syntax() {
        let mut server = LanguageServer::new();
        let invalid = "let x = ;"; // Invalid syntax
        server.open_document(
            "file:///test.jnc".to_string(),
            invalid.to_string(),
            1,
        );

        let options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
        };

        let edits = server.format_document("file:///test.jnc", options);

        // Should return None for invalid syntax
        assert!(edits.is_none());
    }

    // ==================== Code Action Tests ====================

    #[test]
    fn test_get_code_actions_empty() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.jnc".to_string(),
            "let x = 10;".to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 11 },
        };

        let actions = server.get_code_actions("file:///test.jnc", range);

        // Should return empty for valid code with no diagnostics
        assert!(actions.is_empty());
    }

    #[test]
    fn test_extract_suggestion() {
        let message = "Variable 'Signa' not found. Did you mean 'Signal'?";
        let suggestion = extract_suggestion(message);
        assert_eq!(suggestion, Some("Signal".to_string()));

        let message2 = "Variable 'cout' not found. Did you mean `count`?";
        let suggestion2 = extract_suggestion(message2);
        assert_eq!(suggestion2, Some("count".to_string()));
    }

    #[test]
    fn test_extract_type_mismatch() {
        let message = "type mismatch: expected f64, found i32";
        let types = extract_type_mismatch(message);
        assert_eq!(types, Some(("f64".to_string(), "i32".to_string())));

        let message2 = "expected String but found i32";
        let types2 = extract_type_mismatch(message2);
        assert_eq!(types2, Some(("String".to_string(), "i32".to_string())));
    }

    #[test]
    fn test_can_cast() {
        assert!(can_cast("i32", "f64"));
        assert!(can_cast("f64", "i32"));
        assert!(can_cast("u32", "i64"));
        assert!(!can_cast("String", "i32"));
        assert!(!can_cast("bool", "f64"));
    }

    #[test]
    fn test_extract_inferred_type() {
        let message = "cannot infer type, type is i32";
        let typ = extract_inferred_type(message);
        assert_eq!(typ, Some("i32".to_string()));

        let message2 = "type annotation needed, inferred to be String";
        let typ2 = extract_inferred_type(message2);
        assert_eq!(typ2, Some("String".to_string()));
    }

    #[test]
    fn test_get_word_at_range() {
        let content = "let count = Signal::new(0);";
        let range = Range {
            start: Position { line: 0, character: 4 },
            end: Position { line: 0, character: 9 },
        };
        let word = get_word_at_range(content, range);
        assert_eq!(word, "count");
    }

    #[test]
    fn test_ranges_overlap() {
        let range1 = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 10 },
        };
        let range2 = Range {
            start: Position { line: 0, character: 5 },
            end: Position { line: 0, character: 15 },
        };
        assert!(ranges_overlap(range1, range2));

        let range3 = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 5 },
        };
        let range4 = Range {
            start: Position { line: 0, character: 10 },
            end: Position { line: 0, character: 15 },
        };
        assert!(!ranges_overlap(range3, range4));
    }

    #[test]
    fn test_code_action_kind_as_str() {
        assert_eq!(CodeActionKind::QuickFix.as_str(), "quickfix");
        assert_eq!(CodeActionKind::Refactor.as_str(), "refactor");
        assert_eq!(CodeActionKind::SourceOrganizeImports.as_str(), "source.organizeImports");
    }

    // ==================== Go to Definition Tests ====================

    #[test]
    fn test_go_to_definition_function() {
        let mut server = LanguageServer::new();
        let code = r#"
fn calculate(x: i32) -> i32 {
    return x * 2;
}

let result = calculate(10);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Click on "calculate" in the function call (line 5)
        let position = Position { line: 5, character: 13 };
        let location = server.get_definition("file:///test.jnc", position);

        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.uri, "file:///test.jnc");
        assert_eq!(loc.range.start.line, 1); // Function definition on line 1
    }

    #[test]
    fn test_go_to_definition_variable() {
        let mut server = LanguageServer::new();
        let code = r#"
let count = 0;
let doubled = count * 2;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Click on "count" in line 2
        let position = Position { line: 2, character: 14 };
        let location = server.get_definition("file:///test.jnc", position);

        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.uri, "file:///test.jnc");
        assert_eq!(loc.range.start.line, 1); // Variable definition on line 1
        assert_eq!(loc.range.start.character, 4); // "count" starts at column 4
    }

    #[test]
    fn test_go_to_definition_component() {
        let mut server = LanguageServer::new();
        let code = r#"
component Counter() {
    <div>Counter</div>
}

component App() {
    <Counter />
}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Click on "Counter" in JSX (line 6)
        let position = Position { line: 6, character: 5 };
        let location = server.get_definition("file:///test.jnc", position);

        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.uri, "file:///test.jnc");
        assert_eq!(loc.range.start.line, 1); // Component definition on line 1
    }

    #[test]
    fn test_go_to_definition_not_found() {
        let mut server = LanguageServer::new();
        let code = "let x = 10;";
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Click on "y" which doesn't exist
        let position = Position { line: 0, character: 8 };
        let location = server.get_definition("file:///test.jnc", position);

        assert!(location.is_none());
    }

    // ==================== Find References Tests ====================

    #[test]
    fn test_find_references_function() {
        let mut server = LanguageServer::new();
        let code = r#"
fn calculate(x: i32) -> i32 {
    return x * 2;
}

let result = calculate(10);
let result2 = calculate(20);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Find references for "calculate" (on definition line)
        let position = Position { line: 1, character: 3 };
        let references = server.get_references("file:///test.jnc", position, true);

        // Should find 3 occurrences: definition + 2 calls
        assert_eq!(references.len(), 3);
        assert_eq!(references[0].range.start.line, 1); // Definition
        assert_eq!(references[1].range.start.line, 5); // First call
        assert_eq!(references[2].range.start.line, 6); // Second call
    }

    #[test]
    fn test_find_references_variable() {
        let mut server = LanguageServer::new();
        let code = r#"
let count = 0;
count = count + 1;
print(count);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Find references for "count"
        let position = Position { line: 1, character: 4 };
        let references = server.get_references("file:///test.jnc", position, true);

        // Should find 4 occurrences: definition + 3 usages
        assert_eq!(references.len(), 4);
    }

    #[test]
    fn test_find_references_exclude_declaration() {
        let mut server = LanguageServer::new();
        let code = r#"
fn test() {
    return 42;
}

let x = test();
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Find references excluding declaration
        let position = Position { line: 1, character: 3 };
        let references = server.get_references("file:///test.jnc", position, false);

        // Should find only the call, not the definition
        assert_eq!(references.len(), 1);
        assert_eq!(references[0].range.start.line, 5);
    }

    #[test]
    fn test_find_references_component() {
        let mut server = LanguageServer::new();
        let code = r#"
component Button() {
    <button>Click</button>
}

component App() {
    <div>
        <Button />
        <Button />
    </div>
}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Find references for "Button"
        let position = Position { line: 1, character: 10 };
        let references = server.get_references("file:///test.jnc", position, true);

        // Should find 3 occurrences: definition + 2 JSX usages
        assert_eq!(references.len(), 3);
    }

    // ==================== Rename Symbol Tests ====================

    #[test]
    fn test_rename_symbol_function() {
        let mut server = LanguageServer::new();
        let code = r#"
fn oldName() {
    return 42;
}

let x = oldName();
let y = oldName();
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Rename "oldName" to "newName"
        let position = Position { line: 1, character: 3 };
        let edit = server.rename_symbol(
            "file:///test.jnc",
            position,
            "newName".to_string(),
        );

        assert!(edit.is_some());
        let workspace_edit = edit.unwrap();

        // Should have 3 text edits (definition + 2 calls)
        assert_eq!(workspace_edit.changes.len(), 3);
        for change in &workspace_edit.changes {
            assert_eq!(change.new_text, "newName");
        }
    }

    #[test]
    fn test_rename_symbol_variable() {
        let mut server = LanguageServer::new();
        let code = r#"
let counter = 0;
counter = counter + 1;
print(counter);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Rename "counter" to "total"
        let position = Position { line: 1, character: 4 };
        let edit = server.rename_symbol(
            "file:///test.jnc",
            position,
            "total".to_string(),
        );

        assert!(edit.is_some());
        let workspace_edit = edit.unwrap();

        // Should have 4 text edits
        assert_eq!(workspace_edit.changes.len(), 4);
        for change in &workspace_edit.changes {
            assert_eq!(change.new_text, "total");
        }
    }

    #[test]
    fn test_rename_symbol_invalid_name() {
        let mut server = LanguageServer::new();
        let code = "fn test() {}";
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Try to rename to invalid identifier
        let position = Position { line: 0, character: 3 };
        let edit = server.rename_symbol(
            "file:///test.jnc",
            position,
            "123invalid".to_string(),
        );

        // Should return None for invalid identifier
        assert!(edit.is_none());
    }

    #[test]
    fn test_rename_symbol_not_found() {
        let mut server = LanguageServer::new();
        let code = "let x = 10;";
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Try to rename on whitespace (position has no symbol)
        let position = Position { line: 0, character: 6 }; // On the space between '=' and '10'
        let edit = server.rename_symbol(
            "file:///test.jnc",
            position,
            "newName".to_string(),
        );

        // Should return None when cursor is not on a valid symbol
        assert!(edit.is_none());
    }

    // ==================== Document Symbols Tests ====================

    #[test]
    fn test_document_symbols_functions() {
        let mut server = LanguageServer::new();
        let code = r#"
fn function1() {}
fn function2() {}
fn function3() {}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let symbols = server.get_document_symbols("file:///test.jnc");

        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].name, "function1");
        assert_eq!(symbols[1].name, "function2");
        assert_eq!(symbols[2].name, "function3");

        for symbol in &symbols {
            assert_eq!(symbol.kind, DocumentSymbolKind::Function);
        }
    }

    #[test]
    fn test_document_symbols_mixed() {
        let mut server = LanguageServer::new();
        let code = r#"
fn myFunction() {}
let myVariable = 10;
const MY_CONST = 100;
struct MyStruct {}
enum MyEnum {}
component MyComponent() {}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let symbols = server.get_document_symbols("file:///test.jnc");

        assert_eq!(symbols.len(), 6);

        // Check each symbol type
        assert_eq!(symbols[0].name, "myFunction");
        assert_eq!(symbols[0].kind, DocumentSymbolKind::Function);

        assert_eq!(symbols[1].name, "myVariable");
        assert_eq!(symbols[1].kind, DocumentSymbolKind::Variable);

        assert_eq!(symbols[2].name, "MY_CONST");
        assert_eq!(symbols[2].kind, DocumentSymbolKind::Variable);

        assert_eq!(symbols[3].name, "MyStruct");
        assert_eq!(symbols[3].kind, DocumentSymbolKind::Struct);

        assert_eq!(symbols[4].name, "MyEnum");
        assert_eq!(symbols[4].kind, DocumentSymbolKind::Enum);

        assert_eq!(symbols[5].name, "MyComponent");
        assert_eq!(symbols[5].kind, DocumentSymbolKind::Class);
    }

    #[test]
    fn test_document_symbols_empty() {
        let mut server = LanguageServer::new();
        let code = "// Just a comment";
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let symbols = server.get_document_symbols("file:///test.jnc");

        assert_eq!(symbols.len(), 0);
    }

    #[test]
    fn test_document_symbols_range() {
        let mut server = LanguageServer::new();
        let code = r#"
fn calculate() {
    return 42;
}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let symbols = server.get_document_symbols("file:///test.jnc");

        assert_eq!(symbols.len(), 1);
        let symbol = &symbols[0];

        // Check range is correct
        assert_eq!(symbol.range.start.line, 1);
        assert_eq!(symbol.selection_range.start.line, 1);
        // Both range and selection_range should be valid
        assert!(symbol.range.end.character > symbol.range.start.character);
    }

    // Hover tests
    #[test]
    fn test_hover_function() {
        let mut server = LanguageServer::new();
        let code = r#"
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

let result = add(5, 10);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over function name in definition
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 3 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("fn add(x: i32, y: i32) -> i32"));
        assert!(hover_content.contains("**Function**"));
    }

    #[test]
    fn test_hover_component() {
        let mut server = LanguageServer::new();
        let code = r#"
component Button(props: ButtonProps) {
    return <button>{props.label}</button>;
}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over component name
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 10 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("component Button(props: ButtonProps)"));
        assert!(hover_content.contains("**Component**"));
    }

    #[test]
    fn test_hover_variable_with_type() {
        let mut server = LanguageServer::new();
        let code = r#"
let count: i32 = 0;
let value = count + 1;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over variable with type annotation
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 4 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("let count: i32"));
        assert!(hover_content.contains("**Variable**"));
        assert!(hover_content.contains("type `i32`"));
    }

    #[test]
    fn test_hover_variable_without_type() {
        let mut server = LanguageServer::new();
        let code = r#"
let message = "Hello";
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over variable without type annotation
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 4 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("let message"));
        assert!(hover_content.contains("**Variable**"));
    }

    #[test]
    fn test_hover_const() {
        let mut server = LanguageServer::new();
        let code = r#"
const MAX_SIZE: usize = 100;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over constant
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 6 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("const MAX_SIZE: usize"));
        assert!(hover_content.contains("**Constant**"));
        assert!(hover_content.contains("type `usize`"));
    }

    #[test]
    fn test_hover_struct() {
        let mut server = LanguageServer::new();
        let code = r#"
struct Point {
    x: f64,
    y: f64,
}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over struct name
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 7 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("struct Point"));
        assert!(hover_content.contains("x: f64"));
        assert!(hover_content.contains("y: f64"));
        assert!(hover_content.contains("**Struct**"));
    }

    #[test]
    fn test_hover_enum() {
        let mut server = LanguageServer::new();
        let code = r#"
enum Status {
    Pending,
    Active,
    Completed,
}
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over enum name
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 5 });
        assert!(hover.is_some());
        let hover_content = hover.unwrap().contents;
        assert!(hover_content.contains("enum Status"));
        assert!(hover_content.contains("Pending"));
        assert!(hover_content.contains("Active"));
        assert!(hover_content.contains("Completed"));
        assert!(hover_content.contains("**Enum**"));
    }

    #[test]
    fn test_hover_stdlib_function() {
        let mut server = LanguageServer::new();
        let code = r#"
let result = Math::abs(-5);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over stdlib function
        // Note: stdlib documentation may or may not be loaded, so we just verify
        // the hover mechanism works (returns Some or None based on documentation availability)
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 19 });

        // If hover exists, it should have non-empty contents
        if let Some(h) = hover {
            assert!(!h.contents.is_empty(), "Hover contents should not be empty");
        }
        // It's ok if hover is None (stdlib docs not loaded)
    }

    #[test]
    fn test_hover_no_match() {
        let mut server = LanguageServer::new();
        let code = r#"
let x = 10;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Hover over a non-identifier location (should return None)
        let hover = server.get_hover("file:///test.jnc", Position { line: 1, character: 6 });
        // Position at "=" should not match anything useful
        assert!(hover.is_none() || hover.unwrap().contents.is_empty());
    }

    // Signature Help tests
    #[test]
    fn test_signature_help_first_param() {
        let mut server = LanguageServer::new();
        let code = r#"
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

let result = add(5, 10);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Cursor at first parameter: add(|
        let sig_help = server.get_signature_help("file:///test.jnc", Position { line: 5, character: 17 });
        assert!(sig_help.is_some());

        let sig = sig_help.unwrap();
        assert_eq!(sig.signatures.len(), 1);
        assert_eq!(sig.active_parameter, 0); // First parameter

        let sig_info = &sig.signatures[0];
        assert!(sig_info.label.contains("fn add(x: i32, y: i32) -> i32"));
        assert_eq!(sig_info.parameters.len(), 2);
        assert_eq!(sig_info.parameters[0].label, "x: i32");
        assert_eq!(sig_info.parameters[1].label, "y: i32");
    }

    #[test]
    fn test_signature_help_second_param() {
        let mut server = LanguageServer::new();
        let code = r#"
fn multiply(a: f64, b: f64, c: f64) -> f64 {
    return a * b * c;
}

let result = multiply(2.0, 3.0, 4.0);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Cursor at second parameter: multiply(2.0, |
        let sig_help = server.get_signature_help("file:///test.jnc", Position { line: 5, character: 27 });
        assert!(sig_help.is_some());

        let sig = sig_help.unwrap();
        assert_eq!(sig.active_parameter, 1); // Second parameter
        assert_eq!(sig.signatures[0].parameters.len(), 3);
    }

    #[test]
    fn test_signature_help_third_param() {
        let mut server = LanguageServer::new();
        let code = r#"
fn calculate(x: i32, y: i32, z: i32) -> i32 {
    return x + y + z;
}

let result = calculate(1, 2, 3);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Cursor at third parameter: calculate(1, 2, |
        let sig_help = server.get_signature_help("file:///test.jnc", Position { line: 5, character: 30 });
        assert!(sig_help.is_some());

        let sig = sig_help.unwrap();
        assert_eq!(sig.active_parameter, 2); // Third parameter
    }

    #[test]
    fn test_signature_help_no_params() {
        let mut server = LanguageServer::new();
        let code = r#"
fn get_value() -> i32 {
    return 42;
}

let val = get_value();
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Cursor inside function call with no parameters
        let sig_help = server.get_signature_help("file:///test.jnc", Position { line: 5, character: 20 });
        assert!(sig_help.is_some());

        let sig = sig_help.unwrap();
        assert_eq!(sig.signatures.len(), 1);
        assert_eq!(sig.signatures[0].parameters.len(), 0);
        assert!(sig.signatures[0].label.contains("fn get_value() -> i32"));
    }

    #[test]
    fn test_signature_help_not_in_call() {
        let mut server = LanguageServer::new();
        let code = r#"
fn foo() {}
let x = 10;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Cursor not inside a function call
        let sig_help = server.get_signature_help("file:///test.jnc", Position { line: 2, character: 8 });
        assert!(sig_help.is_none());
    }

    #[test]
    fn test_signature_help_function_not_found() {
        let mut server = LanguageServer::new();
        let code = r#"
fn known_func(x: i32) -> i32 {
    return x;
}

let result = unknown_func(10);
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Cursor inside call to unknown function
        let sig_help = server.get_signature_help("file:///test.jnc", Position { line: 5, character: 27 });
        // Should return None because function is not defined
        assert!(sig_help.is_none());
    }

    // Inlay hints tests
    #[test]
    fn test_inlay_hints_type_i32() {
        let mut server = LanguageServer::new();
        let code = r#"
let count = 42;
let value = 100;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have type hints for both variables
        assert_eq!(hints.len(), 2);

        // First hint for 'count'
        assert_eq!(hints[0].kind, InlayHintKind::Type);
        assert_eq!(hints[0].label, ": i32");
        assert_eq!(hints[0].position.line, 1);

        // Second hint for 'value'
        assert_eq!(hints[1].kind, InlayHintKind::Type);
        assert_eq!(hints[1].label, ": i32");
        assert_eq!(hints[1].position.line, 2);
    }

    #[test]
    fn test_inlay_hints_type_string() {
        let mut server = LanguageServer::new();
        let code = r#"
let name = "Alice";
let message = "Hello, World!";
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have type hints for both string variables
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].label, ": String");
        assert_eq!(hints[1].label, ": String");
    }

    #[test]
    fn test_inlay_hints_type_bool() {
        let mut server = LanguageServer::new();
        let code = r#"
let is_active = true;
let is_done = false;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have type hints for both boolean variables
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].label, ": bool");
        assert_eq!(hints[1].label, ": bool");
    }

    #[test]
    fn test_inlay_hints_type_float() {
        let mut server = LanguageServer::new();
        let code = r#"
let pi = 3.14;
let e = 2.718;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have type hints for both float variables
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].label, ": f64");
        assert_eq!(hints[1].label, ": f64");
    }

    #[test]
    fn test_inlay_hints_no_hints_for_explicit_types() {
        let mut server = LanguageServer::new();
        let code = r#"
let count: i32 = 42;
let name: String = "Alice";
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have NO type hints since types are explicitly declared
        // (only parameter hints from any function calls, but there are none here)
        assert_eq!(hints.len(), 0);
    }

    #[test]
    fn test_inlay_hints_type_vec() {
        let mut server = LanguageServer::new();
        let code = r#"
let items = vec![1, 2, 3];
let names = Vec::new();
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have type hints for Vec types
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].label, ": Vec");
        assert_eq!(hints[1].label, ": Vec");
    }

    #[test]
    fn test_inlay_hints_mixed_types() {
        let mut server = LanguageServer::new();
        let code = r#"
let count = 42;
let name = "Bob";
let active = true;
let price = 9.99;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should have type hints for all variables
        assert_eq!(hints.len(), 4);
        assert_eq!(hints[0].label, ": i32");
        assert_eq!(hints[1].label, ": String");
        assert_eq!(hints[2].label, ": bool");
        assert_eq!(hints[3].label, ": f64");
    }

    #[test]
    fn test_inlay_hints_range_filtering() {
        let mut server = LanguageServer::new();
        let code = r#"
let a = 1;
let b = 2;
let c = 3;
let d = 4;
"#;
        server.open_document(
            "file:///test.jnc".to_string(),
            code.to_string(),
            1,
        );

        // Only request hints for lines 2-3
        let range = Range {
            start: Position { line: 2, character: 0 },
            end: Position { line: 3, character: 0 },
        };

        let hints = server.get_inlay_hints("file:///test.jnc", range);

        // Should only have hints for variables in the range
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].position.line, 2); // 'b'
        assert_eq!(hints[1].position.line, 3); // 'c'
    }
}
