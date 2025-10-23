use crate::ast::*;
use crate::errors::CompileError;
use crate::semantic_analyzer::ResolvedType;
use std::collections::HashMap;

/// Represents the current state of a variable's ownership.
#[derive(Debug, Clone)]
enum OwnershipState {
    Owned, // The variable is valid and owns its data.
    Moved, // The variable's data has been moved to another owner.
}

/// A symbol table that tracks ownership state in addition to types.
struct BorrowSymbolTable {
    scopes: Vec<HashMap<String, (OwnershipState, ResolvedType)>>,
}

impl BorrowSymbolTable {
    fn new() -> Self { Self { scopes: vec![HashMap::new()] } }

    fn define(&mut self, name: String, ty: ResolvedType) {
        self.scopes.last_mut().unwrap().insert(name, (OwnershipState::Owned, ty));
    }

    fn update_state(&mut self, name: &str, new_state: OwnershipState) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(name) {
                symbol.0 = new_state;
                return;
            }
        }
    }

    fn lookup(&self, name: &str) -> Option<(OwnershipState, ResolvedType)> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol.clone());
            }
        }
        None
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
}

/// Traverses a type-checked AST to enforce ownership rules.
pub struct BorrowChecker {
    symbols: BorrowSymbolTable,
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            symbols: BorrowSymbolTable::new(),
        };

        // Add built-in Option constructors to global scope
        // These are always available without imports
        // Use ComplexType since the actual type is polymorphic
        checker.symbols.define(
            "Some".to_string(),
            ResolvedType::ComplexType // Function that returns Option<T>
        );
        checker.symbols.define(
            "None".to_string(),
            ResolvedType::ComplexType // Option<T> value
        );

        // Add built-in Result constructors to global scope
        checker.symbols.define(
            "Ok".to_string(),
            ResolvedType::ComplexType // Function that returns Result<T, E>
        );
        checker.symbols.define(
            "Err".to_string(),
            ResolvedType::ComplexType // Function that returns Result<T, E>
        );

        checker
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), CompileError> {
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), CompileError> {
        match stmt {
            Statement::Use(_) => Ok(()),
            Statement::Let(let_stmt) => self.check_let_statement(let_stmt),
            Statement::Const(const_decl) => {
                // Check the value expression
                self.check_expression(&const_decl.value)?;
                // Register the constant in the symbol table
                self.symbols.define(const_decl.name.value.clone(), crate::semantic_analyzer::ResolvedType::Unknown);
                Ok(())
            }
            Statement::Assignment(assign_stmt) => {
                // Check the target is valid
                match &assign_stmt.target {
                    Expression::Identifier(ident) => {
                        // Check that the target variable exists
                        if self.symbols.lookup(&ident.value).is_none() {
                            return Err(CompileError::Generic(format!(
                                "Cannot assign to undefined variable '{}'",
                                ident.value
                            )));
                        }
                    }
                    Expression::FieldAccess(_) | Expression::IndexAccess(_) => {
                        // Check the target expression
                        self.check_expression(&assign_stmt.target)?;
                    }
                    _ => {
                        return Err(CompileError::Generic(
                            "Invalid assignment target".to_string()
                        ));
                    }
                }

                // Check the value expression
                self.check_expression(&assign_stmt.value)?;

                // If the value is moved from a variable, update its state
                if let Expression::Identifier(ident) = &assign_stmt.value {
                    if let Some((_, ty)) = self.symbols.lookup(&ident.value) {
                        if !ty.is_copy() {
                            self.symbols.update_state(&ident.value, OwnershipState::Moved);
                        }
                    }
                }

                Ok(())
            }
            Statement::Return(return_stmt) => self.check_expression(&return_stmt.value).map(|_| ()),
            Statement::Expression(expr) => self.check_expression(expr).map(|_| ()),
            Statement::If(if_stmt) => self.check_if_statement(if_stmt),
            Statement::While(while_stmt) => self.check_while_statement(while_stmt),
            Statement::For(for_stmt) => self.check_for_statement(for_stmt),
            Statement::ForIn(for_in_stmt) => self.check_for_in_statement(for_in_stmt),
            Statement::MacroInvocation(_) => Ok(()),
            Statement::Function(func_def) => {
                // Enter new scope for function body
                self.symbols.enter_scope();

                // Define function parameters in the new scope
                for param in &func_def.parameters {
                    // Parameters get Unknown type since borrow checker doesn't do full type inference
                    self.symbols.define(param.name.value.clone(), ResolvedType::Unknown);
                }

                // Check function body
                for stmt in &func_def.body.statements {
                    self.check_statement(stmt)?;
                }

                // Exit function scope
                self.symbols.exit_scope();

                Ok(())
            }
            Statement::Component(_) => Ok(()),
            Statement::ExternBlock(_) => Ok(()),
            Statement::Struct(_) => Ok(()),
            Statement::Enum(_) => Ok(()),
            Statement::ImplBlock(_) => Ok(()),
            Statement::Trait(_) => Ok(()),
        }
    }

    fn check_if_statement(&mut self, stmt: &IfStatement) -> Result<(), CompileError> {
        self.check_expression(&stmt.condition)?;
        for s in &stmt.then_branch.statements {
            self.check_statement(s)?;
        }
        if let Some(else_stmt) = &stmt.else_branch {
            self.check_statement(else_stmt)?;
        }
        Ok(())
    }

    fn check_while_statement(&mut self, stmt: &WhileStatement) -> Result<(), CompileError> {
        self.check_expression(&stmt.condition)?;
        for s in &stmt.body.statements {
            self.check_statement(s)?;
        }
        Ok(())
    }

    fn check_for_statement(&mut self, stmt: &ForStatement) -> Result<(), CompileError> {
        // Check init statement if present
        if let Some(init) = &stmt.init {
            self.check_statement(init)?;
        }

        // Check condition
        self.check_expression(&stmt.condition)?;

        // Check update statement if present
        if let Some(update) = &stmt.update {
            self.check_statement(update)?;
        }

        // Check body
        for s in &stmt.body.statements {
            self.check_statement(s)?;
        }

        Ok(())
    }

    fn check_for_in_statement(&mut self, stmt: &ForInStatement) -> Result<(), CompileError> {
        // Check the iterator expression
        self.check_expression(&stmt.iterator)?;

        // Enter a new scope for the loop body
        self.symbols.enter_scope();

        // Register the loop variable in the symbol table
        self.symbols.define(stmt.variable.value.clone(), ResolvedType::Unknown);

        // Check body statements
        for s in &stmt.body.statements {
            self.check_statement(s)?;
        }

        // Exit the loop scope
        self.symbols.exit_scope();

        Ok(())
    }

    fn check_let_statement(&mut self, stmt: &LetStatement) -> Result<(), CompileError> {
        let value_type = self.check_expression(&stmt.value)?;
        if let Expression::Identifier(ident) = &stmt.value {
            if !value_type.is_copy() {
                self.symbols.update_state(&ident.value, OwnershipState::Moved);
            }
        }
        // Register all identifiers from the pattern
        for ident in stmt.pattern.bound_identifiers() {
            self.symbols.define(ident.value.clone(), value_type.clone());
        }
        Ok(())
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<ResolvedType, CompileError> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(ResolvedType::Integer),
            Expression::FloatLiteral(_) => Ok(ResolvedType::Float),
            Expression::BoolLiteral(_) => Ok(ResolvedType::Bool),
            Expression::StringLiteral(_) => Ok(ResolvedType::String),
            Expression::Identifier(ident) => {
                // Check if this is a namespaced identifier (e.g., Math::PI, console::log)
                // Treat all namespaced identifiers as external/built-in and skip ownership checking
                if ident.value.contains("::") {
                    return Ok(ResolvedType::Unknown);
                }

                let (state, ty) = self.symbols.lookup(&ident.value)
                    .ok_or_else(|| CompileError::Generic(format!("Borrow checker: undefined variable '{}'", ident.value)))?;

                if let OwnershipState::Moved = state {
                    return Err(CompileError::BorrowError(format!("Use of moved value: '{}'", ident.value)));
                }
                Ok(ty)
            }
            Expression::Prefix(prefix_expr) => {
                self.check_expression(&prefix_expr.right)?;
                Ok(ResolvedType::Integer)
            }
            Expression::Spread(spread_expr) => {
                self.check_expression(&spread_expr.expression)?;
                Ok(ResolvedType::Unit)
            }
            Expression::Infix(infix_expr) => {
                self.check_expression(&infix_expr.left)?;
                self.check_expression(&infix_expr.right)?;
                Ok(ResolvedType::Integer)
            }
            Expression::ArrayLiteral(array_lit) => {
                // Check all elements
                if array_lit.elements.is_empty() {
                    Ok(ResolvedType::Array(Box::new(ResolvedType::Unknown)))
                } else {
                    let first_type = self.check_expression(&array_lit.elements[0])?;
                    for elem in &array_lit.elements[1..] {
                        self.check_expression(elem)?;
                    }
                    Ok(ResolvedType::Array(Box::new(first_type)))
                }
            }
            Expression::TupleLiteral(tuple_lit) => {
                // Check all tuple elements
                let mut element_types = Vec::new();
                for elem in &tuple_lit.elements {
                    let elem_type = self.check_expression(elem)?;
                    element_types.push(elem_type);
                }
                Ok(ResolvedType::Tuple(element_types))
            }
            Expression::StructLiteral(struct_lit) => {
                // Check all field values
                for (_field_name, field_value) in &struct_lit.fields {
                    self.check_expression(field_value)?;
                }
                // For now, return Unknown type
                Ok(ResolvedType::Unknown)
            }
            Expression::FieldAccess(field_access) => {
                // Check the object expression
                self.check_expression(&field_access.object)?;
                // For now, return Unknown type
                Ok(ResolvedType::Unknown)
            }
            Expression::Match(match_expr) => {
                // Check the scrutinee
                self.check_expression(&match_expr.scrutinee)?;

                // Check all match arms
                for arm in &match_expr.arms {
                    // Enter a new scope for this match arm
                    self.symbols.enter_scope();

                    // Register all variables bound by the patterns in this arm
                    for pattern in &arm.patterns {
                        for ident in pattern.bound_identifiers() {
                            self.symbols.define(ident.value.clone(), ResolvedType::Unknown);
                        }
                    }

                    // Check the arm body with the pattern variables in scope
                    self.check_expression(&arm.body)?;

                    // Exit the match arm scope
                    self.symbols.exit_scope();
                }

                // For now, return Unknown type
                Ok(ResolvedType::Unknown)
            }
            Expression::IndexAccess(index_expr) => {
                // Check the array expression
                let array_type = self.check_expression(&index_expr.array)?;

                // Check the index expression
                self.check_expression(&index_expr.index)?;

                // Return the element type if it's an array
                match array_type {
                    ResolvedType::Array(element_type) => Ok(*element_type),
                    _ => Ok(ResolvedType::Unknown),
                }
            }
            Expression::JsxElement(_) => Ok(ResolvedType::VNode),
            Expression::FunctionCall(_) => Ok(ResolvedType::Unknown),
            Expression::Lambda(_) => Ok(ResolvedType::Unknown),
            Expression::Borrow(borrow_expr) => {
                self.check_expression(&borrow_expr.expression)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::MutableBorrow(borrow_expr) => {
                self.check_expression(&borrow_expr.expression)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::Dereference(deref_expr) => {
                self.check_expression(&deref_expr.expression)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::Range(_range_expr) => {
                // Range expressions are placeholders for now
                // In a full implementation, we'd check the start and end expressions
                Ok(ResolvedType::Unknown)
            }
            Expression::TryOperator(try_expr) => {
                self.check_expression(&try_expr.expression)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::Ternary(ternary) => {
                self.check_expression(&ternary.condition)?;
                let true_type = self.check_expression(&ternary.true_expr)?;
                self.check_expression(&ternary.false_expr)?;
                // In a full implementation, we'd check that both branches have the same type
                Ok(true_type)
            }
            Expression::TypeCast(type_cast) => {
                self.check_expression(&type_cast.expression)?;
                // The result type is the target type
                // Extract the type name from TypeExpression
                match &type_cast.target_type {
                    TypeExpression::Named(ident) => {
                        match ident.value.as_str() {
                            "i32" | "i64" | "isize" | "u32" | "u64" | "usize" => Ok(ResolvedType::Integer),
                            "f32" | "f64" => Ok(ResolvedType::Float),
                            "bool" => Ok(ResolvedType::Bool),
                            "String" => Ok(ResolvedType::String),
                            _ => Ok(ResolvedType::Unknown),
                        }
                    }
                    _ => Ok(ResolvedType::Unknown),
                }
            }
            Expression::Await(await_expr) => {
                self.check_expression(&await_expr.expression)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::IfExpression(if_expr) => {
                // Check condition
                self.check_expression(&if_expr.condition)?;
                // Check then expression
                let then_type = self.check_expression(&if_expr.then_expr)?;
                // Check else expression if present
                if let Some(else_expr) = &if_expr.else_expr {
                    self.check_expression(else_expr)?;
                }
                Ok(then_type)
            }
            Expression::Block(block) => {
                // Check all statements in the block
                for stmt in&block.statements {
                    self.check_statement(stmt)?;
                }
                Ok(ResolvedType::Unknown)
            }
            Expression::MacroCall(macro_call) => {
                // Check all arguments recursively (similar to FunctionCall)
                for arg in &macro_call.arguments {
                    self.check_expression(arg)?;
                }
                Ok(ResolvedType::Unknown)
            }
            Expression::CssMacro(_) => {
                // CSS is checked separately in Sprint 1 Task 1.6
                Ok(ResolvedType::Unknown)
            }
        }
    }
}