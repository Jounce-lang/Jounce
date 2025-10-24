// Enhanced Diagnostics and Error Reporting for Jounce
// Beautiful, helpful error messages with colors, context, and suggestions

use crate::token::Token;

/// ANSI color codes for terminal output
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";

    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";

    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_YELLOW: &str = "\x1b[43m";
}

/// Severity level of a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Help,
}

impl Severity {
    pub fn color(&self) -> &'static str {
        match self {
            Severity::Error => colors::RED,
            Severity::Warning => colors::YELLOW,
            Severity::Info => colors::CYAN,
            Severity::Help => colors::GREEN,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Help => "help",
        }
    }
}

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl SourceLocation {
    pub fn from_token(token: &Token, file: &str) -> Self {
        SourceLocation {
            file: file.to_string(),
            line: token.line,
            column: token.column,
            length: token.lexeme.len(),
        }
    }

    pub fn unknown() -> Self {
        SourceLocation {
            file: "<unknown>".to_string(),
            line: 0,
            column: 0,
            length: 0,
        }
    }
}

/// A diagnostic message (error, warning, info, or help)
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub suggestions: Vec<String>,
    pub notes: Vec<String>,
    pub code: Option<String>, // Error code like E001
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Error,
            message: message.into(),
            location: None,
            suggestions: Vec::new(),
            notes: Vec::new(),
            code: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Warning,
            message: message.into(),
            location: None,
            suggestions: Vec::new(),
            notes: Vec::new(),
            code: None,
        }
    }

    pub fn at(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Get LSP-compatible range from source location
    pub fn to_lsp_range(&self) -> Option<crate::lsp::Range> {
        self.location.as_ref().map(|loc| {
            crate::lsp::Range {
                start: crate::lsp::Position {
                    line: if loc.line > 0 { loc.line - 1 } else { 0 },
                    character: if loc.column > 0 { loc.column - 1 } else { 0 },
                },
                end: crate::lsp::Position {
                    line: if loc.line > 0 { loc.line - 1 } else { 0 },
                    character: if loc.column > 0 {
                        loc.column - 1 + loc.length
                    } else {
                        loc.length
                    },
                },
            }
        })
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Format and display the diagnostic with colors
    pub fn display(&self, source_code: Option<&str>) -> String {
        let mut output = String::new();

        // Header: severity and message
        output.push_str(&format!(
            "{bold}{color}{severity}:{reset} {bold}{message}{reset}\n",
            bold = colors::BOLD,
            color = self.severity.color(),
            severity = self.severity.label(),
            reset = colors::RESET,
            message = self.message,
        ));

        // Location info
        if let Some(loc) = &self.location {
            output.push_str(&format!(
                "  {dim}-->{reset} {file}:{line}:{column}\n",
                dim = colors::DIM,
                reset = colors::RESET,
                file = loc.file,
                line = loc.line,
                column = loc.column,
            ));

            // Show source code snippet if available
            if let Some(source) = source_code {
                output.push_str(&self.format_source_snippet(source, loc));
            }
        }

        // Error code
        if let Some(code) = &self.code {
            output.push_str(&format!(
                "  {dim}[{code}]{reset}\n",
                dim = colors::DIM,
                code = code,
                reset = colors::RESET,
            ));
        }

        // Suggestions
        for suggestion in &self.suggestions {
            output.push_str(&format!(
                "  {green}{bold}help:{reset} {suggestion}\n",
                green = colors::GREEN,
                bold = colors::BOLD,
                reset = colors::RESET,
                suggestion = suggestion,
            ));
        }

        // Notes
        for note in &self.notes {
            output.push_str(&format!(
                "  {cyan}note:{reset} {note}\n",
                cyan = colors::CYAN,
                reset = colors::RESET,
                note = note,
            ));
        }

        output
    }

    /// Format a source code snippet with the error highlighted
    fn format_source_snippet(&self, source: &str, loc: &SourceLocation) -> String {
        let mut output = String::new();
        let lines: Vec<&str> = source.lines().collect();

        if loc.line == 0 || loc.line > lines.len() {
            return output;
        }

        let line_idx = loc.line - 1;
        let line_content = lines[line_idx];
        let line_num_width = loc.line.to_string().len();

        // Show previous line for context (if available)
        if line_idx > 0 {
            output.push_str(&format!(
                "   {dim}{:>width$} |{reset} {}\n",
                line_idx,
                lines[line_idx - 1],
                dim = colors::DIM,
                reset = colors::RESET,
                width = line_num_width,
            ));
        }

        // Show the error line
        output.push_str(&format!(
            "   {cyan}{:>width$} |{reset} {}\n",
            loc.line,
            line_content,
            cyan = colors::CYAN,
            reset = colors::RESET,
            width = line_num_width,
        ));

        // Show the error indicator (^^^)
        let padding = " ".repeat(line_num_width + 3 + loc.column - 1);
        let underline = "^".repeat(loc.length.max(1));
        output.push_str(&format!(
            "   {dim}{:>width$} |{reset}{padding}{color}{underline}{reset}\n",
            "",
            dim = colors::DIM,
            reset = colors::RESET,
            width = line_num_width,
            padding = padding,
            color = self.severity.color(),
            underline = underline,
        ));

        // Show next line for context (if available)
        if line_idx + 1 < lines.len() {
            output.push_str(&format!(
                "   {dim}{:>width$} |{reset} {}\n",
                line_idx + 2,
                lines[line_idx + 1],
                dim = colors::DIM,
                reset = colors::RESET,
                width = line_num_width,
            ));
        }

        output
    }
}

/// Diagnostic builder for common error patterns
pub struct DiagnosticBuilder;

impl DiagnosticBuilder {
    /// Type mismatch error
    pub fn type_mismatch(expected: &str, found: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("type mismatch: expected `{}`, found `{}`", expected, found))
            .at(location)
            .with_code("E001")
            .with_suggestion(format!("consider converting `{}` to `{}`", found, expected))
    }

    /// Undefined variable error
    pub fn undefined_variable(name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("cannot find variable `{}` in this scope", name))
            .at(location)
            .with_code("E002")
            .with_suggestion("check that the variable is declared before use")
            .with_note("variables must be declared with `let` before they can be used")
    }

    /// Undefined function error
    pub fn undefined_function(name: &str, location: SourceLocation, similar: Option<&str>) -> Diagnostic {
        let mut diag = Diagnostic::error(format!("cannot find function `{}` in this scope", name))
            .at(location)
            .with_code("E003");

        if let Some(similar_name) = similar {
            diag = diag.with_suggestion(format!("did you mean `{}`?", similar_name));
        }

        diag
    }

    /// Invalid syntax error
    pub fn syntax_error(expected: &str, found: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("expected {}, found {}", expected, found))
            .at(location)
            .with_code("E004")
    }

    /// Borrow checker error
    pub fn borrow_error(message: impl Into<String>, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(message)
            .at(location)
            .with_code("E005")
            .with_note("Jounce enforces memory safety through borrow checking")
    }

    /// Invalid JSX error
    pub fn jsx_error(message: impl Into<String>, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(message)
            .at(location)
            .with_code("E006")
            .with_note("JSX syntax must follow proper HTML-like structure")
    }

    /// Warning: unused variable
    pub fn unused_variable(name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::warning(format!("unused variable: `{}`", name))
            .at(location)
            .with_code("W001")
            .with_suggestion(format!("prefix with `_` to silence: `_{}`", name))
    }

    /// Warning: unreachable code
    pub fn unreachable_code(location: SourceLocation) -> Diagnostic {
        Diagnostic::warning("unreachable code")
            .at(location)
            .with_code("W002")
            .with_note("any code after a `return` statement will never execute")
    }

    /// Module not found error
    pub fn module_not_found(module_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("module `{}` not found", module_name))
            .at(location)
            .with_code("E007")
            .with_suggestion("check that the module name is spelled correctly")
            .with_note("modules must be installed via `raven pkg add` or exist in the workspace")
    }

    /// Import item not found error
    pub fn import_not_found(item: &str, module: &str, location: SourceLocation, similar: Option<&str>) -> Diagnostic {
        let mut diag = Diagnostic::error(format!("cannot find `{}` in module `{}`", item, module))
            .at(location)
            .with_code("E008");

        if let Some(similar_name) = similar {
            diag = diag.with_suggestion(format!("did you mean `{}`?", similar_name));
        } else {
            diag = diag.with_suggestion(format!("check the exports of `{}`", module));
        }

        diag
    }

    /// Circular dependency error
    pub fn circular_dependency(modules: &[String], location: SourceLocation) -> Diagnostic {
        let cycle = modules.join(" -> ");
        Diagnostic::error(format!("circular dependency detected: {}", cycle))
            .at(location)
            .with_code("E009")
            .with_note("circular dependencies can cause initialization issues")
            .with_suggestion("refactor code to break the circular dependency")
    }

    /// JSX: unclosed element error
    pub fn jsx_unclosed_element(tag_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("unclosed JSX element `<{}>`", tag_name))
            .at(location)
            .with_code("E010")
            .with_suggestion(format!("add closing tag `</{}>` or make it self-closing `<{} />`", tag_name, tag_name))
    }

    /// JSX: mismatched tags error
    pub fn jsx_mismatched_tags(opening: &str, closing: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("mismatched JSX tags: expected `</{}`, found `</{}>`", opening, closing))
            .at(location)
            .with_code("E011")
            .with_suggestion(format!("change closing tag to `</{}>`", opening))
    }

    /// JSX: invalid attribute error
    pub fn jsx_invalid_attribute(attr: &str, tag: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("invalid attribute `{}` for `<{}>` element", attr, tag))
            .at(location)
            .with_code("E012")
            .with_note("check HTML attribute names for this element type")
    }

    /// Async/await: cannot await non-async function
    pub fn await_non_async(function_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("cannot await non-async function `{}`", function_name))
            .at(location)
            .with_code("E013")
            .with_suggestion(format!("mark function `{}` as `async fn` or remove `await`", function_name))
    }

    /// Async/await: async function not awaited
    pub fn async_not_awaited(function_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::warning(format!("async function `{}` is not awaited", function_name))
            .at(location)
            .with_code("W003")
            .with_suggestion("add `await` before the function call or use `.then()` for Promise handling")
            .with_note("async functions return Promises that should be awaited")
    }

    /// Missing return type error
    pub fn missing_return_type(function_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("function `{}` is missing return type annotation", function_name))
            .at(location)
            .with_code("E014")
            .with_suggestion("add `-> ReturnType` after function parameters")
            .with_note("explicit return types improve code clarity and type safety")
    }

    /// Dead code warning
    pub fn dead_code(item_type: &str, item_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::warning(format!("{} `{}` is never used", item_type, item_name))
            .at(location)
            .with_code("W004")
            .with_suggestion("remove unused code or prefix with `_` to suppress warning")
    }

    /// Deprecated API warning
    pub fn deprecated_api(api_name: &str, replacement: Option<&str>, location: SourceLocation) -> Diagnostic {
        let mut diag = Diagnostic::warning(format!("use of deprecated API: `{}`", api_name))
            .at(location)
            .with_code("W005");

        if let Some(new_api) = replacement {
            diag = diag.with_suggestion(format!("use `{}` instead", new_api));
        }

        diag.with_note("deprecated APIs may be removed in future versions")
    }

    /// Type annotation needed
    pub fn type_annotation_needed(var_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("cannot infer type for variable `{}`, type annotation required", var_name))
            .at(location)
            .with_code("E015")
            .with_suggestion(format!("add type annotation: `let {}: Type = ...`", var_name))
            .with_note("type inference requires sufficient context")
    }

    /// Missing field in struct literal
    pub fn missing_struct_field(struct_name: &str, field_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("missing field `{}` in `{}` struct literal", field_name, struct_name))
            .at(location)
            .with_code("E016")
            .with_suggestion(format!("add field: `{}: value`", field_name))
    }

    /// Unknown field in struct literal
    pub fn unknown_struct_field(struct_name: &str, field_name: &str, location: SourceLocation, similar: Option<&str>) -> Diagnostic {
        let mut diag = Diagnostic::error(format!("struct `{}` has no field `{}`", struct_name, field_name))
            .at(location)
            .with_code("E017");

        if let Some(similar_name) = similar {
            diag = diag.with_suggestion(format!("did you mean `{}`?", similar_name));
        }

        diag
    }

    /// Match not exhaustive
    pub fn match_not_exhaustive(missing_patterns: &[String], location: SourceLocation) -> Diagnostic {
        let patterns = missing_patterns.join("`, `");
        Diagnostic::error(format!("match is not exhaustive, missing patterns: `{}`", patterns))
            .at(location)
            .with_code("E018")
            .with_suggestion("add missing match arms or use `_` wildcard pattern")
            .with_note("match expressions must handle all possible values")
    }
}

/// Diagnostic collector for managing multiple diagnostics
pub struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
    error_count: usize,
    warning_count: usize,
}

impl DiagnosticCollector {
    pub fn new() -> Self {
        DiagnosticCollector {
            diagnostics: Vec::new(),
            error_count: 0,
            warning_count: 0,
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        match diagnostic.severity {
            Severity::Error => self.error_count += 1,
            Severity::Warning => self.warning_count += 1,
            _ => {}
        }
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    pub fn has_warnings(&self) -> bool {
        self.warning_count > 0
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }

    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    pub fn display_all(&self, source_code: Option<&str>) -> String {
        let mut output = String::new();

        for diagnostic in &self.diagnostics {
            output.push_str(&diagnostic.display(source_code));
            output.push('\n');
        }

        // Summary
        if self.has_errors() || self.has_warnings() {
            output.push_str(&format!(
                "{bold}compilation result:{reset} ",
                bold = colors::BOLD,
                reset = colors::RESET,
            ));

            if self.has_errors() {
                output.push_str(&format!(
                    "{red}{error_count} error{s}{reset}",
                    red = colors::RED,
                    error_count = self.error_count,
                    s = if self.error_count == 1 { "" } else { "s" },
                    reset = colors::RESET,
                ));
            }

            if self.has_warnings() {
                if self.has_errors() {
                    output.push_str(", ");
                }
                output.push_str(&format!(
                    "{yellow}{warning_count} warning{s}{reset}",
                    yellow = colors::YELLOW,
                    warning_count = self.warning_count,
                    s = if self.warning_count == 1 { "" } else { "s" },
                    reset = colors::RESET,
                ));
            }

            output.push('\n');
        }

        output
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.error_count = 0;
        self.warning_count = 0;
    }
}

impl Default for DiagnosticCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Levenshtein distance for fuzzy matching (did you mean...?)
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    matrix[len1][len2]
}

/// Find the most similar string from a list (for "did you mean?" suggestions)
pub fn find_similar(target: &str, candidates: &[&str]) -> Option<String> {
    let mut best_match: Option<(String, usize)> = None;

    for candidate in candidates {
        let distance = levenshtein_distance(target, candidate);

        // Only consider matches within reasonable distance (< 3 edits)
        if distance < 3 {
            if let Some((_, best_distance)) = best_match {
                if distance < best_distance {
                    best_match = Some((candidate.to_string(), distance));
                }
            } else {
                best_match = Some((candidate.to_string(), distance));
            }
        }
    }

    best_match.map(|(name, _)| name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_display() {
        let diag = Diagnostic::error("type mismatch")
            .at(SourceLocation {
                file: "test.jnc".to_string(),
                line: 10,
                column: 5,
                length: 3,
            })
            .with_suggestion("consider converting types")
            .with_code("E001");

        let output = diag.display(None);
        assert!(output.contains("error"));
        assert!(output.contains("type mismatch"));
        assert!(output.contains("test.jnc:10:5"));
        assert!(output.contains("help"));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("test", "test"), 0);
        assert_eq!(levenshtein_distance("", "test"), 4);
    }

    #[test]
    fn test_find_similar() {
        let candidates = vec!["Signal", "Computed", "Effect", "Resource"];

        assert_eq!(find_similar("Signa", &candidates), Some("Signal".to_string()));
        assert_eq!(find_similar("Compited", &candidates), Some("Computed".to_string()));
        assert_eq!(find_similar("xyz", &candidates), None); // Too different
    }

    #[test]
    fn test_diagnostic_collector() {
        let mut collector = DiagnosticCollector::new();

        collector.add(Diagnostic::error("error 1"));
        collector.add(Diagnostic::warning("warning 1"));
        collector.add(Diagnostic::error("error 2"));

        assert_eq!(collector.error_count(), 2);
        assert_eq!(collector.warning_count(), 1);
        assert!(collector.has_errors());
        assert!(collector.has_warnings());
    }

    #[test]
    fn test_source_snippet_formatting() {
        let source = "let x = 10;\nlet y = 20;\nlet z = x + y;";
        let loc = SourceLocation {
            file: "test.jnc".to_string(),
            line: 2,
            column: 5,
            length: 1,
        };

        let diag = Diagnostic::error("undefined variable").at(loc);
        let output = diag.display(Some(source));

        assert!(output.contains("let y = 20"));
        assert!(output.contains("^"));
    }

    #[test]
    fn test_module_not_found() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::module_not_found("missing_module", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("module `missing_module` not found"));
        assert_eq!(diag.code, Some("E007".to_string()));
        assert!(!diag.suggestions.is_empty());
    }

    #[test]
    fn test_import_not_found() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::import_not_found("Item", "raven_store", loc, Some("Signal"));

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("cannot find `Item` in module"));
        assert_eq!(diag.code, Some("E008".to_string()));
        assert!(diag.suggestions[0].contains("did you mean `Signal`?"));
    }

    #[test]
    fn test_circular_dependency() {
        let loc = SourceLocation::unknown();
        let modules = vec!["A".to_string(), "B".to_string(), "A".to_string()];
        let diag = DiagnosticBuilder::circular_dependency(&modules, loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("circular dependency"));
        assert_eq!(diag.code, Some("E009".to_string()));
    }

    #[test]
    fn test_jsx_unclosed_element() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::jsx_unclosed_element("div", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("unclosed JSX element `<div>`"));
        assert_eq!(diag.code, Some("E010".to_string()));
        assert!(diag.suggestions[0].contains("closing tag"));
    }

    #[test]
    fn test_jsx_mismatched_tags() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::jsx_mismatched_tags("div", "span", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("mismatched JSX tags"));
        assert_eq!(diag.code, Some("E011".to_string()));
    }

    #[test]
    fn test_await_non_async() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::await_non_async("fetch_data", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("cannot await non-async function"));
        assert_eq!(diag.code, Some("E013".to_string()));
    }

    #[test]
    fn test_async_not_awaited() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::async_not_awaited("fetch_data", loc);

        assert_eq!(diag.severity, Severity::Warning);
        assert!(diag.message.contains("async function"));
        assert_eq!(diag.code, Some("W003".to_string()));
    }

    #[test]
    fn test_type_annotation_needed() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::type_annotation_needed("x", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("cannot infer type"));
        assert_eq!(diag.code, Some("E015".to_string()));
        assert!(diag.suggestions[0].contains("type annotation"));
    }

    #[test]
    fn test_missing_struct_field() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::missing_struct_field("Point", "z", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("missing field `z` in `Point`"));
        assert_eq!(diag.code, Some("E016".to_string()));
    }

    #[test]
    fn test_unknown_struct_field() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::unknown_struct_field("Point", "z", loc, Some("x"));

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("struct `Point` has no field `z`"));
        assert_eq!(diag.code, Some("E017".to_string()));
        assert!(diag.suggestions[0].contains("did you mean `x`?"));
    }

    #[test]
    fn test_match_not_exhaustive() {
        let loc = SourceLocation::unknown();
        let missing = vec!["Some(_)".to_string(), "None".to_string()];
        let diag = DiagnosticBuilder::match_not_exhaustive(&missing, loc);

        assert_eq!(diag.severity, Severity::Error);
        assert!(diag.message.contains("match is not exhaustive"));
        assert_eq!(diag.code, Some("E018".to_string()));
    }

    #[test]
    fn test_dead_code_warning() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::dead_code("function", "unused_func", loc);

        assert_eq!(diag.severity, Severity::Warning);
        assert!(diag.message.contains("function `unused_func` is never used"));
        assert_eq!(diag.code, Some("W004".to_string()));
    }

    #[test]
    fn test_deprecated_api() {
        let loc = SourceLocation::unknown();
        let diag = DiagnosticBuilder::deprecated_api("old_api", Some("new_api"), loc);

        assert_eq!(diag.severity, Severity::Warning);
        assert!(diag.message.contains("deprecated API"));
        assert_eq!(diag.code, Some("W005".to_string()));
        assert!(diag.suggestions[0].contains("new_api"));
    }
}
