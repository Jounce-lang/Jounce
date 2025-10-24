/// Jounce Code Formatter
///
/// Provides automatic code formatting for .jnc files with consistent, opinionated style.
/// Formats all language features including JSX, pattern matching, and async/await.

use crate::ast::*;
use crate::token::Token;

/// Formatting configuration
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// Number of spaces per indentation level (default: 4)
    pub indent_size: usize,
    /// Maximum line length as a soft limit (default: 100)
    pub max_line_length: usize,
    /// Use spaces instead of tabs (default: true)
    pub use_spaces: bool,
    /// Add trailing commas in multiline lists (default: true)
    pub trailing_comma: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig {
            indent_size: 4,
            max_line_length: 100,
            use_spaces: true,
            trailing_comma: true,
        }
    }
}

/// Main formatter that traverses AST and generates formatted code
pub struct Formatter {
    config: FormatterConfig,
    indent_level: usize,
    output: String,
}

impl Formatter {
    /// Create a new formatter with default configuration
    pub fn new() -> Self {
        Self::with_config(FormatterConfig::default())
    }

    /// Create a new formatter with custom configuration
    pub fn with_config(config: FormatterConfig) -> Self {
        Formatter {
            config,
            indent_level: 0,
            output: String::new(),
        }
    }

    /// Format a complete program
    pub fn format_program(&mut self, program: &Program) -> String {
        for statement in &program.statements {
            self.format_statement(statement);
            self.newline();
        }

        // Remove trailing whitespace and ensure single trailing newline
        self.output.trim_end().to_string() + "\n"
    }

    /// Format a single statement
    pub fn format_statement(&mut self, statement: &Statement) {
        self.write_indent();

        match statement {
            Statement::Use(use_stmt) => self.format_use_statement(use_stmt),
            Statement::Let(let_stmt) => self.format_let_statement(let_stmt),
            Statement::Const(const_decl) => self.format_const_declaration(const_decl),
            Statement::Assignment(assign) => self.format_assignment_statement(assign),
            Statement::Return(ret) => self.format_return_statement(ret),
            Statement::Expression(expr) => {
                self.format_expression(expr);
                self.write(";");
            }
            Statement::If(if_stmt) => self.format_if_statement(if_stmt),
            Statement::While(while_stmt) => self.format_while_statement(while_stmt),
            Statement::For(for_stmt) => self.format_for_statement(for_stmt),
            Statement::ForIn(for_in) => self.format_for_in_statement(for_in),
            Statement::Loop(loop_stmt) => {
                self.write("loop ");
                self.format_block_inline(&loop_stmt.body);
            }
            Statement::Break => self.write("break;\n"),
            Statement::Continue => self.write("continue;\n"),
            Statement::MacroInvocation(macro_inv) => self.format_macro_invocation(macro_inv),
            Statement::Struct(struct_def) => self.format_struct_definition(struct_def),
            Statement::Enum(enum_def) => self.format_enum_definition(enum_def),
            Statement::Function(fn_def) => self.format_function_definition(fn_def),
            Statement::Component(comp_def) => self.format_component_definition(comp_def),
            Statement::ExternBlock(extern_block) => self.format_extern_block(extern_block),
            Statement::ImplBlock(impl_block) => self.format_impl_block(impl_block),
            Statement::Trait(trait_def) => self.format_trait_definition(trait_def),
        }
    }

    // ==================== STATEMENT FORMATTERS ====================

    fn format_use_statement(&mut self, use_stmt: &UseStatement) {
        self.write("use ");

        // Format path (e.g., raven_store)
        let path_str: Vec<String> = use_stmt.path.iter()
            .map(|id| id.value.clone())
            .collect();
        self.write(&path_str.join("::"));

        // Format imports if any
        if !use_stmt.imports.is_empty() {
            self.write("::{");
            for (i, import) in use_stmt.imports.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&import.value);
            }
            self.write("}");
        }

        self.write(";");
    }

    fn format_let_statement(&mut self, let_stmt: &LetStatement) {
        self.write("let ");

        if let_stmt.mutable {
            self.write("mut ");
        }

        self.format_pattern(&let_stmt.pattern);

        if let Some(type_annotation) = &let_stmt.type_annotation {
            self.write(": ");
            self.format_type_expression(type_annotation);
        }

        self.write(" = ");
        self.format_expression(&let_stmt.value);
        self.write(";");
    }

    fn format_const_declaration(&mut self, const_decl: &ConstDeclaration) {
        self.write("const ");
        self.write(&const_decl.name.value);

        if let Some(type_annotation) = &const_decl.type_annotation {
            self.write(": ");
            self.format_type_expression(type_annotation);
        }

        self.write(" = ");
        self.format_expression(&const_decl.value);
        self.write(";");
    }

    fn format_assignment_statement(&mut self, assign: &AssignmentStatement) {
        self.format_expression(&assign.target);
        self.write(" = ");
        self.format_expression(&assign.value);
        self.write(";");
    }

    fn format_return_statement(&mut self, ret: &ReturnStatement) {
        self.write("return ");
        self.format_expression(&ret.value);
        self.write(";");
    }

    fn format_if_statement(&mut self, if_stmt: &IfStatement) {
        self.write("if ");
        self.format_expression(&if_stmt.condition);
        self.write(" ");
        self.format_block_inline(&if_stmt.then_branch);

        if let Some(else_branch) = &if_stmt.else_branch {
            self.write(" else ");

            // Check if else branch is another if statement (else if)
            match else_branch.as_ref() {
                Statement::If(else_if) => {
                    // Remove trailing whitespace before "if" for else if formatting
                    let trimmed = self.output.trim_end().to_string();
                    self.output = trimmed;
                    self.format_if_statement(else_if);
                }
                _ => {
                    self.format_statement(else_branch);
                }
            }
        }
    }

    fn format_while_statement(&mut self, while_stmt: &WhileStatement) {
        self.write("while ");
        self.format_expression(&while_stmt.condition);
        self.write(" ");
        self.format_block_inline(&while_stmt.body);
    }

    fn format_for_statement(&mut self, for_stmt: &ForStatement) {
        self.write("for (");

        if let Some(init) = &for_stmt.init {
            // Format init without semicolon at the end
            let saved_output_len = self.output.len();
            let saved_indent = self.indent_level;
            self.indent_level = 0; // No indent inside for parens
            self.format_statement(init);
            self.indent_level = saved_indent;

            // Remove indent and trailing whitespace
            let formatted = self.output[saved_output_len..].trim().to_string();
            self.output.truncate(saved_output_len);
            self.write(&formatted);
        } else {
            self.write(";");
        }

        self.write(" ");
        self.format_expression(&for_stmt.condition);
        self.write(";");

        if let Some(update) = &for_stmt.update {
            self.write(" ");
            let saved_output_len = self.output.len();
            let saved_indent = self.indent_level;
            self.indent_level = 0;
            self.format_statement(update);
            self.indent_level = saved_indent;

            // Remove indent, semicolon, and trailing whitespace
            let mut formatted = self.output[saved_output_len..].trim().to_string();
            if formatted.ends_with(';') {
                formatted.pop();
            }
            self.output.truncate(saved_output_len);
            self.write(&formatted);
        }

        self.write(") ");
        self.format_block_inline(&for_stmt.body);
    }

    fn format_for_in_statement(&mut self, for_in: &ForInStatement) {
        self.write("for ");
        self.write(&for_in.variable.value);
        self.write(" in ");
        self.format_expression(&for_in.iterator);
        self.write(" ");
        self.format_block_inline(&for_in.body);
    }

    fn format_macro_invocation(&mut self, macro_inv: &MacroInvocation) {
        self.write(&macro_inv.name.value);
        self.write("!(");
        // Format macro tokens (simplified - just concatenate for now)
        for token in &macro_inv.input_tokens {
            self.write(&format!("{:?}", token)); // TODO: Better token formatting
        }
        self.write(");");
    }

    fn format_struct_definition(&mut self, struct_def: &StructDefinition) {
        // Derive macros
        if !struct_def.derives.is_empty() {
            self.write("#[derive(");
            for (i, derive) in struct_def.derives.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(derive);
            }
            self.write(")]");
            self.newline();
            self.write_indent();
        }

        self.write("struct ");
        self.write(&struct_def.name.value);

        // Generic parameters
        if !struct_def.lifetime_params.is_empty() || !struct_def.type_params.is_empty() {
            self.write("<");
            let mut first = true;
            for lifetime in &struct_def.lifetime_params {
                if !first {
                    self.write(", ");
                }
                self.write("'");
                self.write(&lifetime.name);
                first = false;
            }
            for type_param in &struct_def.type_params {
                if !first {
                    self.write(", ");
                }
                self.write(&type_param.name.value);
                if !type_param.bounds.is_empty() {
                    self.write(": ");
                    for (i, bound) in type_param.bounds.iter().enumerate() {
                        if i > 0 {
                            self.write(" + ");
                        }
                        self.write(&bound.value);
                    }
                }
                first = false;
            }
            self.write(">");
        }

        self.write(" {");
        self.newline();

        self.indent_level += 1;
        for (name, ty) in &struct_def.fields {
            self.write_indent();
            self.write(&name.value);
            self.write(": ");
            self.format_type_expression(ty);
            self.write(",");
            self.newline();
        }
        self.indent_level -= 1;

        self.write_indent();
        self.write("}");
    }

    fn format_enum_definition(&mut self, enum_def: &EnumDefinition) {
        // Derive macros
        if !enum_def.derives.is_empty() {
            self.write("#[derive(");
            for (i, derive) in enum_def.derives.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(derive);
            }
            self.write(")]");
            self.newline();
            self.write_indent();
        }

        self.write("enum ");
        self.write(&enum_def.name.value);

        // Generic parameters
        if !enum_def.lifetime_params.is_empty() || !enum_def.type_params.is_empty() {
            self.write("<");
            let mut first = true;
            for lifetime in &enum_def.lifetime_params {
                if !first {
                    self.write(", ");
                }
                self.write("'");
                self.write(&lifetime.name);
                first = false;
            }
            for type_param in &enum_def.type_params {
                if !first {
                    self.write(", ");
                }
                self.write(&type_param.name.value);
                if !type_param.bounds.is_empty() {
                    self.write(": ");
                    for (i, bound) in type_param.bounds.iter().enumerate() {
                        if i > 0 {
                            self.write(" + ");
                        }
                        self.write(&bound.value);
                    }
                }
                first = false;
            }
            self.write(">");
        }

        self.write(" {");
        self.newline();

        self.indent_level += 1;
        for variant in &enum_def.variants {
            self.write_indent();
            self.write(&variant.name.value);

            if let Some(fields) = &variant.fields {
                self.write(" {");
                self.newline();
                self.indent_level += 1;

                for (name, ty) in fields {
                    self.write_indent();
                    self.write(&name.value);
                    self.write(": ");
                    self.format_type_expression(ty);
                    self.write(",");
                    self.newline();
                }

                self.indent_level -= 1;
                self.write_indent();
                self.write("}");
            }

            self.write(",");
            self.newline();
        }
        self.indent_level -= 1;

        self.write_indent();
        self.write("}");
    }

    fn format_function_definition(&mut self, fn_def: &FunctionDefinition) {
        // Annotations
        if fn_def.is_server {
            self.write("@server");
            self.newline();
            self.write_indent();
        }
        if fn_def.is_client {
            self.write("@client");
            self.newline();
            self.write_indent();
        }

        if fn_def.is_async {
            self.write("async ");
        }

        self.write("fn ");
        self.write(&fn_def.name.value);

        // Generic parameters
        if !fn_def.lifetime_params.is_empty() || !fn_def.type_params.is_empty() {
            self.write("<");
            let mut first = true;
            for lifetime in &fn_def.lifetime_params {
                if !first {
                    self.write(", ");
                }
                self.write("'");
                self.write(&lifetime.name);
                first = false;
            }
            for type_param in &fn_def.type_params {
                if !first {
                    self.write(", ");
                }
                self.write(&type_param.name.value);
                if !type_param.bounds.is_empty() {
                    self.write(": ");
                    for (i, bound) in type_param.bounds.iter().enumerate() {
                        if i > 0 {
                            self.write(" + ");
                        }
                        self.write(&bound.value);
                    }
                }
                first = false;
            }
            self.write(">");
        }

        self.write("(");
        for (i, param) in fn_def.parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write(&param.name.value);
            self.write(": ");
            self.format_type_expression(&param.type_annotation);
        }
        self.write(") ");

        self.format_block_inline(&fn_def.body);
    }

    fn format_component_definition(&mut self, comp_def: &ComponentDefinition) {
        if comp_def.is_client {
            self.write("@client");
            self.newline();
            self.write_indent();
        }

        self.write("component ");
        self.write(&comp_def.name.value);
        self.write("(");

        for (i, param) in comp_def.parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write(&param.name.value);
            self.write(": ");
            self.format_type_expression(&param.type_annotation);
        }

        self.write(") ");
        self.format_block_inline(&comp_def.body);
    }

    fn format_extern_block(&mut self, extern_block: &ExternBlock) {
        self.write("extern \"");
        self.write(&extern_block.abi);
        self.write("\" {");
        self.newline();

        self.indent_level += 1;
        for func_decl in &extern_block.functions {
            self.write_indent();
            self.write("fn ");
            self.write(&func_decl.name.value);
            self.write("(");

            for (i, param) in func_decl.parameters.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&param.name.value);
                self.write(": ");
                self.format_type_expression(&param.type_annotation);
            }

            self.write(")");

            if let Some(return_type) = &func_decl.return_type {
                self.write(" -> ");
                self.format_type_expression(return_type);
            }

            self.write(";");
            self.newline();
        }
        self.indent_level -= 1;

        self.write_indent();
        self.write("}");
    }

    fn format_impl_block(&mut self, impl_block: &ImplBlock) {
        self.write("impl ");

        // Generic parameters (lifetimes + types)
        if !impl_block.lifetime_params.is_empty() || !impl_block.type_params.is_empty() {
            self.write("<");
            let mut first = true;
            for lifetime in &impl_block.lifetime_params {
                if !first {
                    self.write(", ");
                }
                self.write("'");
                self.write(&lifetime.name);
                first = false;
            }
            for type_param in &impl_block.type_params {
                if !first {
                    self.write(", ");
                }
                self.write(&type_param.name.value);
                if !type_param.bounds.is_empty() {
                    self.write(": ");
                    for (i, bound) in type_param.bounds.iter().enumerate() {
                        if i > 0 {
                            self.write(" + ");
                        }
                        self.write(&bound.value);
                    }
                }
                first = false;
            }
            self.write("> ");
        }

        // Trait name if any
        if let Some(trait_name) = &impl_block.trait_name {
            self.write(&trait_name.value);
            self.write(" for ");
        }

        self.write(&impl_block.type_name.value);

        self.write(" {");
        self.newline();

        self.indent_level += 1;
        for method in &impl_block.methods {
            self.write_indent();
            self.write("fn ");
            self.write(&method.name.value);
            self.write("(");

            for (i, param) in method.parameters.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&param.name.value);
                self.write(": ");
                self.format_type_expression(&param.type_annotation);
            }

            self.write(")");

            if let Some(return_type) = &method.return_type {
                self.write(" -> ");
                self.format_type_expression(return_type);
            }

            self.write(" ");
            self.format_block_inline(&method.body);
            self.newline();
        }
        self.indent_level -= 1;

        self.write_indent();
        self.write("}");
    }

    fn format_trait_definition(&mut self, trait_def: &TraitDefinition) {
        self.write("trait ");
        self.write(&trait_def.name.value);

        // Generic parameters (lifetimes + types)
        if !trait_def.lifetime_params.is_empty() || !trait_def.type_params.is_empty() {
            self.write("<");
            let mut first = true;
            for lifetime in &trait_def.lifetime_params {
                if !first {
                    self.write(", ");
                }
                self.write("'");
                self.write(&lifetime.name);
                first = false;
            }
            for type_param in &trait_def.type_params {
                if !first {
                    self.write(", ");
                }
                self.write(&type_param.name.value);
                if !type_param.bounds.is_empty() {
                    self.write(": ");
                    for (i, bound) in type_param.bounds.iter().enumerate() {
                        if i > 0 {
                            self.write(" + ");
                        }
                        self.write(&bound.value);
                    }
                }
                first = false;
            }
            self.write(">");
        }

        self.write(" {");
        self.newline();

        self.indent_level += 1;
        for method in &trait_def.methods {
            self.write_indent();
            self.write("fn ");
            self.write(&method.name.value);
            self.write("(");

            for (i, param) in method.parameters.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&param.name.value);
                self.write(": ");
                self.format_type_expression(&param.type_annotation);
            }

            self.write(")");

            if let Some(return_type) = &method.return_type {
                self.write(" -> ");
                self.format_type_expression(return_type);
            }

            self.write(";");
            self.newline();
        }
        self.indent_level -= 1;

        self.write_indent();
        self.write("}");
    }

    // ==================== EXPRESSION FORMATTERS ====================

    fn format_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(id) => self.write(&id.value),
            Expression::IntegerLiteral(n) => self.write(&n.to_string()),
            Expression::FloatLiteral(f) => self.write(f),
            Expression::StringLiteral(s) => {
                self.write("\"");
                self.write(&s.replace('"', "\\\""));
                self.write("\"");
            }
            Expression::BoolLiteral(b) => self.write(if *b { "true" } else { "false" }),
            Expression::UnitLiteral => self.write("()"),
            Expression::ArrayLiteral(arr) => self.format_array_literal(arr),
            Expression::TupleLiteral(tuple) => self.format_tuple_literal(tuple),
            Expression::StructLiteral(struct_lit) => self.format_struct_literal(struct_lit),
            Expression::Prefix(prefix) => self.format_prefix_expression(prefix),
            Expression::Spread(spread) => self.format_spread_expression(spread),
            Expression::Infix(infix) => self.format_infix_expression(infix),
            Expression::FieldAccess(field) => self.format_field_access(field),
            Expression::IndexAccess(index) => self.format_index_access(index),
            Expression::Match(match_expr) => self.format_match_expression(match_expr),
            Expression::IfExpression(if_expr) => self.format_if_expression(if_expr),
            Expression::JsxElement(jsx) => self.format_jsx_element(jsx),
            Expression::FunctionCall(call) => self.format_function_call(call),
            Expression::MacroCall(macro_call) => self.format_macro_call(macro_call),
            Expression::Lambda(lambda) => self.format_lambda_expression(lambda),
            Expression::Borrow(borrow) => self.format_borrow_expression(borrow),
            Expression::MutableBorrow(mut_borrow) => self.format_mutable_borrow_expression(mut_borrow),
            Expression::Dereference(deref) => self.format_dereference_expression(deref),
            Expression::Range(range) => self.format_range_expression(range),
            Expression::TryOperator(try_op) => self.format_try_operator(try_op),
            Expression::Ternary(ternary) => self.format_ternary_expression(ternary),
            Expression::TypeCast(cast) => self.format_type_cast(cast),
            Expression::Await(await_expr) => self.format_await_expression(await_expr),
            Expression::Block(block) => self.format_block_expression(block),
            Expression::CssMacro(_) => {
                // CSS formatting will be implemented in Sprint 1 Task 1.6
                self.write("css! { /* formatting not yet implemented */ }")
            }
            // Reactivity primitives (Phase 12)
            Expression::Signal(signal_expr) => {
                self.write("signal");
                if let Some(_type_ann) = &signal_expr.type_annotation {
                    self.write("<T>");  // Simplified for now
                }
                self.write("(");
                self.format_expression(&signal_expr.initial_value);
                self.write(")");
            }
            Expression::Computed(computed_expr) => {
                self.write("computed");
                if let Some(_type_ann) = &computed_expr.type_annotation {
                    self.write("<T>");  // Simplified for now
                }
                self.write("(");
                self.format_expression(&computed_expr.computation);
                self.write(")");
            }
            Expression::Effect(effect_expr) => {
                self.write("effect(");
                self.format_expression(&effect_expr.callback);
                self.write(")");
            }
            Expression::Batch(batch_expr) => {
                self.write("batch(");
                self.format_expression(&batch_expr.body);
                self.write(")");
            }
        }
    }

    fn format_array_literal(&mut self, arr: &ArrayLiteral) {
        self.write("[");
        for (i, elem) in arr.elements.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.format_expression(elem);
        }
        self.write("]");
    }

    fn format_tuple_literal(&mut self, tuple: &TupleLiteral) {
        self.write("(");
        for (i, elem) in tuple.elements.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.format_expression(elem);
        }
        self.write(")");
    }

    fn format_struct_literal(&mut self, struct_lit: &StructLiteral) {
        self.write(&struct_lit.name.value);
        self.write(" {");

        if !struct_lit.fields.is_empty() {
            self.write(" ");
            for (i, (name, value)) in struct_lit.fields.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&name.value);
                self.write(": ");
                self.format_expression(value);
            }
            self.write(" ");
        }

        self.write("}");
    }

    fn format_prefix_expression(&mut self, prefix: &PrefixExpression) {
        self.write(&token_to_string(&prefix.operator));
        self.format_expression(&prefix.right);
    }

    fn format_spread_expression(&mut self, spread: &SpreadExpression) {
        self.write("...");
        self.format_expression(&spread.expression);
    }

    fn format_infix_expression(&mut self, infix: &InfixExpression) {
        self.format_expression(&infix.left);
        self.write(" ");
        self.write(&token_to_string(&infix.operator));
        self.write(" ");
        self.format_expression(&infix.right);
    }

    fn format_field_access(&mut self, field: &FieldAccessExpression) {
        self.format_expression(&field.object);
        self.write(".");
        self.write(&field.field.value);
    }

    fn format_index_access(&mut self, index: &IndexExpression) {
        self.format_expression(&index.array);
        self.write("[");
        self.format_expression(&index.index);
        self.write("]");
    }

    fn format_match_expression(&mut self, match_expr: &MatchExpression) {
        self.write("match ");
        self.format_expression(&match_expr.scrutinee);
        self.write(" {");
        self.newline();

        self.indent_level += 1;
        for arm in &match_expr.arms {
            self.write_indent();
            // Format OR patterns: 3 | 4 | 5 => ...
            for (i, pattern) in arm.patterns.iter().enumerate() {
                if i > 0 {
                    self.write(" | ");
                }
                self.format_pattern(pattern);
            }
            self.write(" => ");
            self.format_expression(&arm.body);
            self.write(",");
            self.newline();
        }
        self.indent_level -= 1;

        self.write_indent();
        self.write("}");
    }

    fn format_if_expression(&mut self, if_expr: &IfExpression) {
        self.write("if ");
        self.format_expression(&if_expr.condition);
        self.write(" { ");
        self.format_expression(&if_expr.then_expr);
        self.write(" }");

        if let Some(else_expr) = &if_expr.else_expr {
            self.write(" else { ");
            self.format_expression(else_expr);
            self.write(" }");
        }
    }

    fn format_jsx_element(&mut self, jsx: &JsxElement) {
        // Determine if this should be multi-line
        let should_be_multiline = self.should_jsx_be_multiline(jsx);

        if should_be_multiline {
            self.format_jsx_element_multiline(jsx);
        } else {
            self.format_jsx_element_inline(jsx);
        }
    }

    /// Determine if JSX element should be formatted on multiple lines
    fn should_jsx_be_multiline(&self, jsx: &JsxElement) -> bool {
        // Multi-line if:
        // 1. Has more than 3 attributes
        // 2. Has JSX element children

        if jsx.opening_tag.attributes.len() > 3 {
            return true;
        }

        if !jsx.children.is_empty() {
            // Check if any child is a JSX element
            for child in &jsx.children {
                if matches!(child, JsxChild::Element(_)) {
                    return true;
                }
            }
        }

        false
    }

    /// Format JSX element on a single line (for simple elements)
    fn format_jsx_element_inline(&mut self, jsx: &JsxElement) {
        self.write("<");
        self.write(&jsx.opening_tag.name.value);

        // Attributes
        for attr in &jsx.opening_tag.attributes {
            self.write(" ");
            self.write(&attr.name.value);

            if let Expression::BoolLiteral(true) = attr.value {
                continue;
            }

            self.write("=");

            match &attr.value {
                Expression::StringLiteral(s) => {
                    self.write("\"");
                    self.write(s);
                    self.write("\"");
                }
                _ => {
                    self.write("{");
                    self.format_expression(&attr.value);
                    self.write("}");
                }
            }
        }

        if jsx.opening_tag.self_closing {
            self.write(" />");
            return;
        }

        self.write(">");

        // Children inline
        for child in &jsx.children {
            match child {
                JsxChild::Element(elem) => self.format_jsx_element(elem),
                JsxChild::Text(text) => self.write(text),
                JsxChild::Expression(expr) => {
                    self.write("{");
                    self.format_expression(expr);
                    self.write("}");
                }
            }
        }

        // Closing tag
        if let Some(closing_tag) = &jsx.closing_tag {
            self.write("</");
            self.write(&closing_tag.value);
            self.write(">");
        }
    }

    /// Format JSX element on multiple lines (for complex elements)
    fn format_jsx_element_multiline(&mut self, jsx: &JsxElement) {
        self.write("<");
        self.write(&jsx.opening_tag.name.value);

        // Attributes - multi-line if >3
        if jsx.opening_tag.attributes.len() > 3 {
            self.indent_level += 1;
            for attr in &jsx.opening_tag.attributes {
                self.newline();
                self.write_indent();
                self.write(&attr.name.value);

                if let Expression::BoolLiteral(true) = attr.value {
                    continue;
                }

                self.write("=");

                match &attr.value {
                    Expression::StringLiteral(s) => {
                        self.write("\"");
                        self.write(s);
                        self.write("\"");
                    }
                    _ => {
                        self.write("{");
                        self.format_expression(&attr.value);
                        self.write("}");
                    }
                }
            }
            self.indent_level -= 1;
            self.newline();
            self.write_indent();
        } else {
            // Inline attributes if <=3
            for attr in &jsx.opening_tag.attributes {
                self.write(" ");
                self.write(&attr.name.value);

                if let Expression::BoolLiteral(true) = attr.value {
                    continue;
                }

                self.write("=");

                match &attr.value {
                    Expression::StringLiteral(s) => {
                        self.write("\"");
                        self.write(s);
                        self.write("\"");
                    }
                    _ => {
                        self.write("{");
                        self.format_expression(&attr.value);
                        self.write("}");
                    }
                }
            }
        }

        if jsx.opening_tag.self_closing {
            if jsx.opening_tag.attributes.len() > 3 {
                self.write("/>");
            } else {
                self.write(" />");
            }
            return;
        }

        self.write(">");

        // Children - each on new line if element children exist
        let has_element_children = jsx.children.iter().any(|c| matches!(c, JsxChild::Element(_)));

        if has_element_children {
            self.indent_level += 1;
            for child in &jsx.children {
                match child {
                    JsxChild::Element(elem) => {
                        self.newline();
                        self.write_indent();
                        self.format_jsx_element(elem);
                    }
                    JsxChild::Text(text) => {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            self.newline();
                            self.write_indent();
                            self.write(trimmed);
                        }
                    }
                    JsxChild::Expression(expr) => {
                        self.newline();
                        self.write_indent();
                        self.write("{");
                        self.format_expression(expr);
                        self.write("}");
                    }
                }
            }
            self.indent_level -= 1;
            self.newline();
            self.write_indent();
        } else {
            // Inline children if no element children
            for child in &jsx.children {
                match child {
                    JsxChild::Element(elem) => self.format_jsx_element(elem),
                    JsxChild::Text(text) => self.write(text),
                    JsxChild::Expression(expr) => {
                        self.write("{");
                        self.format_expression(expr);
                        self.write("}");
                    }
                }
            }
        }

        // Closing tag
        if let Some(closing_tag) = &jsx.closing_tag {
            self.write("</");
            self.write(&closing_tag.value);
            self.write(">");
        }
    }

    fn format_function_call(&mut self, call: &FunctionCall) {
        self.format_expression(&call.function);

        // Turbofish (generic arguments)
        if let Some(type_params) = &call.type_params {
            self.write("::<");
            for (i, type_arg) in type_params.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.format_type_expression(type_arg);
            }
            self.write(">");
        }

        self.write("(");
        for (i, arg) in call.arguments.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.format_expression(arg);
        }
        self.write(")");
    }

    fn format_macro_call(&mut self, macro_call: &MacroCall) {
        self.write(&macro_call.name.value);
        self.write("!(");

        // Format macro arguments
        for (i, arg) in macro_call.arguments.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.format_expression(arg);
        }

        self.write(")");
    }

    fn format_lambda_expression(&mut self, lambda: &LambdaExpression) {
        self.write("|");

        for (i, param) in lambda.parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write(&param.name.value);
            // Format type annotation if present
            if let Some(ref type_expr) = param.type_annotation {
                self.write(": ");
                self.format_type_expression(type_expr);
            }
        }

        self.write("|");

        // Format return type if present
        if let Some(ref return_type) = lambda.return_type {
            self.write(" -> ");
            self.format_type_expression(return_type);
        }

        self.write(" ");
        self.format_expression(&lambda.body);
    }

    fn format_borrow_expression(&mut self, borrow: &BorrowExpression) {
        self.write("&");
        self.format_expression(&borrow.expression);
    }

    fn format_mutable_borrow_expression(&mut self, mut_borrow: &MutableBorrowExpression) {
        self.write("&mut ");
        self.format_expression(&mut_borrow.expression);
    }

    fn format_dereference_expression(&mut self, deref: &DereferenceExpression) {
        self.write("*");
        self.format_expression(&deref.expression);
    }

    fn format_range_expression(&mut self, range: &RangeExpression) {
        if let Some(start) = &range.start {
            self.format_expression(start);
        }

        if range.inclusive {
            self.write("..=");
        } else {
            self.write("..");
        }

        if let Some(end) = &range.end {
            self.format_expression(end);
        }
    }

    fn format_try_operator(&mut self, try_op: &TryOperatorExpression) {
        self.format_expression(&try_op.expression);
        self.write("?");
    }

    fn format_ternary_expression(&mut self, ternary: &TernaryExpression) {
        self.format_expression(&ternary.condition);
        self.write(" ? ");
        self.format_expression(&ternary.true_expr);
        self.write(" : ");
        self.format_expression(&ternary.false_expr);
    }

    fn format_type_cast(&mut self, cast: &TypeCastExpression) {
        self.format_expression(&cast.expression);
        self.write(" as ");
        self.format_type_expression(&cast.target_type);
    }

    fn format_await_expression(&mut self, await_expr: &AwaitExpression) {
        self.write("await ");
        self.format_expression(&await_expr.expression);
    }

    fn format_block_expression(&mut self, block: &BlockStatement) {
        self.write("{");

        if !block.statements.is_empty() {
            self.newline();
            self.indent_level += 1;

            for stmt in &block.statements {
                self.format_statement(stmt);
                self.newline();
            }

            self.indent_level -= 1;
            self.write_indent();
        }

        self.write("}");
    }

    // ==================== HELPER METHODS ====================

    fn format_block_inline(&mut self, block: &BlockStatement) {
        self.write("{");

        if !block.statements.is_empty() {
            self.newline();
            self.indent_level += 1;

            for stmt in &block.statements {
                self.format_statement(stmt);
                self.newline();
            }

            self.indent_level -= 1;
            self.write_indent();
        }

        self.write("}");
    }

    fn format_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Identifier(id) => self.write(&id.value),
            Pattern::Tuple(patterns) => {
                self.write("(");
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_pattern(p);
                }
                self.write(")");
            }
            Pattern::Literal(expr) => self.format_expression(expr),
            Pattern::Wildcard => self.write("_"),
            Pattern::EnumVariant { name, fields } => {
                self.write(&name.value);
                if let Some(field_patterns) = fields {
                    self.write("(");
                    for (i, p) in field_patterns.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.format_pattern(p);
                    }
                    self.write(")");
                }
            }
        }
    }

    fn format_type_expression(&mut self, ty: &TypeExpression) {
        match ty {
            TypeExpression::Named(id) => self.write(&id.value),
            TypeExpression::Generic(id, type_args) => {
                self.write(&id.value);
                self.write("<");
                for (i, arg) in type_args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_type_expression(arg);
                }
                self.write(">");
            }
            TypeExpression::Tuple(types) => {
                self.write("(");
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_type_expression(t);
                }
                self.write(")");
            }
            TypeExpression::Reference(ty) => {
                self.write("&");
                self.format_type_expression(ty);
            }
            TypeExpression::MutableReference(ty) => {
                self.write("&mut ");
                self.format_type_expression(ty);
            }
            TypeExpression::Slice(ty) => {
                self.write("[");
                self.format_type_expression(ty);
                self.write("]");
            }
            TypeExpression::SizedArray(ty, size) => {
                self.write("[");
                self.format_type_expression(ty);
                self.write("; ");
                self.write(&size.to_string());
                self.write("]");
            }
            TypeExpression::Function(params, ret) => {
                self.write("fn(");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_type_expression(param);
                }
                self.write(") -> ");
                self.format_type_expression(ret);
            }
        }
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn write_indent(&mut self) {
        if self.config.use_spaces {
            let indent = " ".repeat(self.indent_level * self.config.indent_size);
            self.output.push_str(&indent);
        } else {
            let indent = "\t".repeat(self.indent_level);
            self.output.push_str(&indent);
        }
    }

    fn newline(&mut self) {
        self.output.push('\n');
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== HELPER FUNCTIONS ====================

/// Convert a token to its string representation for formatting
fn token_to_string(token: &Token) -> String {
    use crate::token::TokenKind;

    match &token.kind {
        TokenKind::Plus => "+",
        TokenKind::Minus => "-",
        TokenKind::Star => "*",
        TokenKind::Slash => "/",
        TokenKind::Percent => "%",
        TokenKind::Eq => "==",
        TokenKind::NotEq => "!=",
        TokenKind::LAngle => "<",
        TokenKind::LtEq => "<=",
        TokenKind::RAngle => ">",
        TokenKind::GtEq => ">=",
        TokenKind::AmpAmp => "&&",
        TokenKind::PipePipe => "||",
        TokenKind::Bang => "!",
        TokenKind::Ampersand => "&",
        TokenKind::Pipe => "|",
        _ => &token.lexeme,
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn test_format_simple_let() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "x".to_string(),
                }),
                mutable: false,
                type_annotation: Some(TypeExpression::Named(Identifier {
                    value: "i32".to_string(),
                })),
                value: Expression::IntegerLiteral(42),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);
        assert_eq!(formatted, "let x: i32 = 42;\n");
    }

    #[test]
    fn test_format_function() {
        let program = Program {
            statements: vec![Statement::Function(FunctionDefinition {
                name: Identifier {
                    value: "add".to_string(),
                },
                lifetime_params: vec![],
                type_params: vec![],
                parameters: vec![
                    FunctionParameter {
                        name: Identifier {
                            value: "a".to_string(),
                        },
                        type_annotation: TypeExpression::Named(Identifier {
                            value: "i32".to_string(),
                        }),
                    },
                    FunctionParameter {
                        name: Identifier {
                            value: "b".to_string(),
                        },
                        type_annotation: TypeExpression::Named(Identifier {
                            value: "i32".to_string(),
                        }),
                    },
                ],
                is_server: false,
                is_client: false,
                is_async: false,
                body: BlockStatement {
                    statements: vec![Statement::Return(ReturnStatement {
                        value: Expression::Infix(InfixExpression {
                            left: Box::new(Expression::Identifier(Identifier {
                                value: "a".to_string(),
                            })),
                            operator: Token::new(TokenKind::Plus, "+".to_string(), 1, 1),
                            right: Box::new(Expression::Identifier(Identifier {
                                value: "b".to_string(),
                            })),
                        }),
                    })],
                },
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("fn add(a: i32, b: i32)"));
        assert!(formatted.contains("return a + b;"));
    }

    #[test]
    fn test_format_struct() {
        let program = Program {
            statements: vec![Statement::Struct(StructDefinition {
                name: Identifier {
                    value: "Point".to_string(),
                },
                lifetime_params: vec![],
                type_params: vec![],
                fields: vec![
                    (
                        Identifier {
                            value: "x".to_string(),
                        },
                        TypeExpression::Named(Identifier {
                            value: "i32".to_string(),
                        }),
                    ),
                    (
                        Identifier {
                            value: "y".to_string(),
                        },
                        TypeExpression::Named(Identifier {
                            value: "i32".to_string(),
                        }),
                    ),
                ],
                derives: vec![],
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("struct Point {"));
        assert!(formatted.contains("x: i32,"));
        assert!(formatted.contains("y: i32,"));
    }

    #[test]
    fn test_format_enum() {
        let program = Program {
            statements: vec![Statement::Enum(EnumDefinition {
                name: Identifier {
                    value: "Option".to_string(),
                },
                lifetime_params: vec![],
                type_params: vec![],
                variants: vec![
                    EnumVariant {
                        name: Identifier {
                            value: "Some".to_string(),
                        },
                        fields: None,
                    },
                    EnumVariant {
                        name: Identifier {
                            value: "None".to_string(),
                        },
                        fields: None,
                    },
                ],
                derives: vec![],
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("enum Option {"));
        assert!(formatted.contains("Some,"));
        assert!(formatted.contains("None,"));
    }

    #[test]
    fn test_format_match_expression() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "result".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::Match(MatchExpression {
                    scrutinee: Box::new(Expression::Identifier(Identifier {
                        value: "x".to_string(),
                    })),
                    arms: vec![
                        MatchArm {
                            patterns: vec![Pattern::Literal(Expression::IntegerLiteral(1))],
                            body: Box::new(Expression::StringLiteral("one".to_string())),
                        },
                        MatchArm {
                            patterns: vec![Pattern::Wildcard],
                            body: Box::new(Expression::StringLiteral("other".to_string())),
                        },
                    ],
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("match x {"));
        assert!(formatted.contains("1 => \"one\","));
        assert!(formatted.contains("_ => \"other\","));
    }

    #[test]
    fn test_format_jsx_simple_inline() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "elem".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::JsxElement(JsxElement {
                    opening_tag: JsxOpeningTag {
                        name: Identifier {
                            value: "Button".to_string(),
                        },
                        attributes: vec![],
                        self_closing: false,
                    },
                    children: vec![JsxChild::Text("Click".to_string())],
                    closing_tag: Some(Identifier {
                        value: "Button".to_string(),
                    }),
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("let elem = <Button>Click</Button>;"));
    }

    #[test]
    fn test_format_jsx_multiline_nested() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "elem".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::JsxElement(JsxElement {
                    opening_tag: JsxOpeningTag {
                        name: Identifier {
                            value: "div".to_string(),
                        },
                        attributes: vec![],
                        self_closing: false,
                    },
                    children: vec![
                        JsxChild::Element(Box::new(JsxElement {
                            opening_tag: JsxOpeningTag {
                                name: Identifier {
                                    value: "Header".to_string(),
                                },
                                attributes: vec![],
                                self_closing: true,
                            },
                            children: vec![],
                            closing_tag: None,
                        })),
                        JsxChild::Element(Box::new(JsxElement {
                            opening_tag: JsxOpeningTag {
                                name: Identifier {
                                    value: "Content".to_string(),
                                },
                                attributes: vec![],
                                self_closing: true,
                            },
                            children: vec![],
                            closing_tag: None,
                        })),
                    ],
                    closing_tag: Some(Identifier {
                        value: "div".to_string(),
                    }),
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        // Should be multi-line because it has nested JSX elements
        assert!(formatted.contains("<div>"));
        assert!(formatted.contains("<Header />"));
        assert!(formatted.contains("<Content />"));
        assert!(formatted.contains("</div>"));
    }

    #[test]
    fn test_format_jsx_many_attributes() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "elem".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::JsxElement(JsxElement {
                    opening_tag: JsxOpeningTag {
                        name: Identifier {
                            value: "Component".to_string(),
                        },
                        attributes: vec![
                            JsxAttribute {
                                name: Identifier {
                                    value: "prop1".to_string(),
                                },
                                value: Expression::IntegerLiteral(1),
                            },
                            JsxAttribute {
                                name: Identifier {
                                    value: "prop2".to_string(),
                                },
                                value: Expression::IntegerLiteral(2),
                            },
                            JsxAttribute {
                                name: Identifier {
                                    value: "prop3".to_string(),
                                },
                                value: Expression::IntegerLiteral(3),
                            },
                            JsxAttribute {
                                name: Identifier {
                                    value: "prop4".to_string(),
                                },
                                value: Expression::IntegerLiteral(4),
                            },
                        ],
                        self_closing: true,
                    },
                    children: vec![],
                    closing_tag: None,
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        // Should be multi-line because it has >3 attributes
        assert!(formatted.contains("<Component"));
        assert!(formatted.contains("prop1={1}"));
        assert!(formatted.contains("prop2={2}"));
        assert!(formatted.contains("prop3={3}"));
        assert!(formatted.contains("prop4={4}"));
        assert!(formatted.contains("/>"));
    }

    #[test]
    fn test_format_lambda() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "add".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::Lambda(LambdaExpression {
                    parameters: vec![
                        LambdaParameter {
                            name: Identifier {
                                value: "x".to_string(),
                            },
                            type_annotation: None,
                        },
                        LambdaParameter {
                            name: Identifier {
                                value: "y".to_string(),
                            },
                            type_annotation: None,
                        },
                    ],
                    return_type: None,
                    body: Box::new(Expression::Infix(InfixExpression {
                        left: Box::new(Expression::Identifier(Identifier {
                            value: "x".to_string(),
                        })),
                        operator: Token::new(TokenKind::Plus, "+".to_string(), 1, 1),
                        right: Box::new(Expression::Identifier(Identifier {
                            value: "y".to_string(),
                        })),
                    })),
                    captures: vec![],
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("|x, y| x + y"));
    }

    #[test]
    fn test_format_ternary() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "result".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::Ternary(TernaryExpression {
                    condition: Box::new(Expression::BoolLiteral(true)),
                    true_expr: Box::new(Expression::IntegerLiteral(1)),
                    false_expr: Box::new(Expression::IntegerLiteral(0)),
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("true ? 1 : 0"));
    }

    #[test]
    fn test_format_array_literal() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "arr".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::ArrayLiteral(ArrayLiteral {
                    elements: vec![
                        Expression::IntegerLiteral(1),
                        Expression::IntegerLiteral(2),
                        Expression::IntegerLiteral(3),
                    ],
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_format_if_expression() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "result".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::IfExpression(IfExpression {
                    condition: Box::new(Expression::BoolLiteral(true)),
                    then_expr: Box::new(Expression::IntegerLiteral(1)),
                    else_expr: Some(Box::new(Expression::IntegerLiteral(0))),
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("if true { 1 } else { 0 }"));
    }

    #[test]
    fn test_format_for_loop() {
        let program = Program {
            statements: vec![Statement::ForIn(ForInStatement {
                variable: Identifier {
                    value: "i".to_string(),
                },
                iterator: Expression::Identifier(Identifier {
                    value: "items".to_string(),
                }),
                body: BlockStatement {
                    statements: vec![Statement::Expression(Expression::Identifier(Identifier {
                        value: "i".to_string(),
                    }))],
                },
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("for i in items {"));
    }

    #[test]
    fn test_format_while_loop() {
        let program = Program {
            statements: vec![Statement::While(WhileStatement {
                condition: Expression::BoolLiteral(true),
                body: BlockStatement {
                    statements: vec![Statement::Expression(Expression::Identifier(Identifier {
                        value: "x".to_string(),
                    }))],
                },
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("while true {"));
    }

    #[test]
    fn test_format_const() {
        let program = Program {
            statements: vec![Statement::Const(ConstDeclaration {
                name: Identifier {
                    value: "PI".to_string(),
                },
                type_annotation: Some(TypeExpression::Named(Identifier {
                    value: "f64".to_string(),
                })),
                value: Expression::FloatLiteral("3.14".to_string()),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("const PI: f64 = 3.14;"));
    }

    #[test]
    fn test_format_use_statement() {
        let program = Program {
            statements: vec![Statement::Use(UseStatement {
                path: vec![Identifier {
                    value: "raven_store".to_string(),
                }],
                imports: vec![
                    Identifier {
                        value: "Signal".to_string(),
                    },
                    Identifier {
                        value: "Computed".to_string(),
                    },
                ],
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("use raven_store::{Signal, Computed};"));
    }

    #[test]
    fn test_format_spread_operator() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "arr".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::ArrayLiteral(ArrayLiteral {
                    elements: vec![
                        Expression::IntegerLiteral(1),
                        Expression::Spread(SpreadExpression {
                            expression: Box::new(Expression::Identifier(Identifier {
                                value: "rest".to_string(),
                            })),
                        }),
                    ],
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("[1, ...rest]"));
    }

    #[test]
    fn test_format_type_cast() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "x".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::TypeCast(TypeCastExpression {
                    expression: Box::new(Expression::IntegerLiteral(42)),
                    target_type: TypeExpression::Named(Identifier {
                        value: "f64".to_string(),
                    }),
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("42 as f64"));
    }

    #[test]
    fn test_format_struct_literal() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "point".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::StructLiteral(StructLiteral {
                    name: Identifier {
                        value: "Point".to_string(),
                    },
                    fields: vec![
                        (
                            Identifier {
                                value: "x".to_string(),
                            },
                            Expression::IntegerLiteral(10),
                        ),
                        (
                            Identifier {
                                value: "y".to_string(),
                            },
                            Expression::IntegerLiteral(20),
                        ),
                    ],
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("Point { x: 10, y: 20 }"));
    }

    #[test]
    fn test_format_async_function() {
        let program = Program {
            statements: vec![Statement::Function(FunctionDefinition {
                name: Identifier {
                    value: "fetch_data".to_string(),
                },
                lifetime_params: vec![],
                type_params: vec![],
                parameters: vec![],
                is_server: false,
                is_client: false,
                is_async: true,
                body: BlockStatement {
                    statements: vec![Statement::Return(ReturnStatement {
                        value: Expression::IntegerLiteral(42),
                    })],
                },
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("async fn fetch_data()"));
    }

    #[test]
    fn test_format_tuple_literal() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                pattern: Pattern::Identifier(Identifier {
                    value: "tuple".to_string(),
                }),
                mutable: false,
                type_annotation: None,
                value: Expression::TupleLiteral(TupleLiteral {
                    elements: vec![
                        Expression::IntegerLiteral(1),
                        Expression::StringLiteral("hello".to_string()),
                        Expression::BoolLiteral(true),
                    ],
                }),
            })],
        };

        let mut formatter = Formatter::new();
        let formatted = formatter.format_program(&program);

        assert!(formatted.contains("(1, \"hello\", true)"));
    }
}
