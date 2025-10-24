// Type Checker with Hindley-Milner Type Inference

use crate::ast::{Expression, Statement, InfixExpression, PrefixExpression, TypeExpression, TraitDefinition, ImplBlock};
use crate::errors::CompileError;
use crate::types::{Substitution, Type, TypeEnv};
use std::collections::{HashSet, HashMap};

// Information about a trait's methods
#[derive(Debug, Clone)]
pub struct TraitInfo {
    pub name: String,
    pub methods: HashMap<String, FunctionSignature>,
}

// Function signature for trait method validation
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub param_types: Vec<Type>,
    pub return_type: Type,
}

pub struct TypeChecker {
    env: TypeEnv,
    constraints: Vec<(Type, Type)>,
    traits: HashMap<String, TraitInfo>,  // Track trait definitions
    impls: HashMap<String, Vec<String>>,  // Track which traits are implemented for each type
    methods: HashMap<String, HashMap<String, FunctionSignature>>,  // type_name -> (method_name -> signature)
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = TypeEnv::new();

        // Add built-in types and functions
        env.bind("console".to_string(), Type::Any);
        env.bind("Math".to_string(), Type::Any);

        TypeChecker {
            env,
            constraints: Vec::new(),
            traits: HashMap::new(),
            impls: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    /// Convert TypeExpression from AST to Type
    fn type_expr_to_type(&self, type_expr: &TypeExpression) -> Type {
        match type_expr {
            TypeExpression::Named(ident) => {
                match ident.value.as_str() {
                    "i32" | "i64" | "i8" | "i16" | "isize" => Type::Int,
                    "f32" | "f64" => Type::Float,
                    "bool" => Type::Bool,
                    "str" | "String" => Type::String,
                    _ => {
                        // Check if this is a generic type parameter in scope
                        // If so, return Type::Any (type erasure)
                        if let Some(ty) = self.env.lookup(&ident.value) {
                            if ty == Type::Any {
                                // This is a generic type parameter
                                return Type::Any;
                            }
                        }
                        Type::Named(ident.value.clone())
                    }
                }
            }
            TypeExpression::Generic(ident, args) => {
                match ident.value.as_str() {
                    "Array" | "Vec" if args.len() == 1 => {
                        Type::Array(Box::new(self.type_expr_to_type(&args[0])))
                    }
                    "Option" if args.len() == 1 => {
                        Type::Option(Box::new(self.type_expr_to_type(&args[0])))
                    }
                    _ => Type::Named(ident.value.clone()),
                }
            }
            TypeExpression::Tuple(types) => {
                // Convert tuple type expression to Type::Tuple
                let converted_types: Vec<Type> = types.iter()
                    .map(|t| self.type_expr_to_type(t))
                    .collect();
                Type::Tuple(converted_types)
            }
            TypeExpression::Reference(_inner) => {
                // For now, return Any for reference types
                Type::Any
            }
            TypeExpression::MutableReference(_inner) => {
                // For now, return Any for mutable reference types
                Type::Any
            }
            TypeExpression::Slice(inner) => {
                // Slices are array views - recursively process the inner type
                // Return Type::Array with the inner type
                let inner_type = self.type_expr_to_type(inner);
                Type::Array(Box::new(inner_type))
            }
            TypeExpression::SizedArray(inner, _size) => {
                // Sized arrays [T; N] - treat as arrays with the inner type
                let inner_type = self.type_expr_to_type(inner);
                Type::Array(Box::new(inner_type))
            }
            TypeExpression::Function(param_types, return_type) => {
                // Convert function type to Type::Function
                let params: Vec<Type> = param_types.iter()
                    .map(|t| self.type_expr_to_type(t))
                    .collect();
                let ret = Box::new(self.type_expr_to_type(return_type));
                Type::Function { params, return_type: ret }
            }
        }
    }

    /// Type check a program (list of statements)
    pub fn check_program(&mut self, statements: &[Statement]) -> Result<(), CompileError> {
        for stmt in statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    /// Infer the type of a statement
    pub fn check_statement(&mut self, stmt: &Statement) -> Result<Type, CompileError> {
        match stmt {
            Statement::Let(let_stmt) => {
                let value_type = self.infer_expression(&let_stmt.value)?;
                // Register all identifiers from the pattern
                for ident in let_stmt.pattern.bound_identifiers() {
                    self.env.bind(ident.value.clone(), value_type.clone());
                }
                Ok(value_type)
            }

            Statement::Assignment(assign_stmt) => {
                // Infer the type of the target expression
                let target_type = match &assign_stmt.target {
                    Expression::Identifier(ident) => {
                        // Check that variable exists and clone the type
                        self.env.lookup(&ident.value)
                            .ok_or_else(|| CompileError::Generic(format!(
                                "Cannot assign to undefined variable '{}'",
                                ident.value
                            )))?.clone()
                    }
                    Expression::FieldAccess(_) | Expression::IndexAccess(_) => {
                        // Infer the type of field access or index access
                        self.infer_expression(&assign_stmt.target)?
                    }
                    _ => {
                        return Err(CompileError::Generic(
                            "Invalid assignment target".to_string()
                        ));
                    }
                };

                // Check that value type matches target type
                let value_type = self.infer_expression(&assign_stmt.value)?;
                if let Err(e) = self.unify(&value_type, &target_type) {
                    let target_desc = match &assign_stmt.target {
                        Expression::Identifier(ident) => format!("'{}'", ident.value),
                        Expression::FieldAccess(_) => "field".to_string(),
                        Expression::IndexAccess(_) => "index".to_string(),
                        _ => "target".to_string(),
                    };
                    return Err(CompileError::Generic(format!(
                        "Type mismatch in assignment to {}: expected {}, got {}. {}",
                        target_desc, target_type, value_type, e
                    )));
                }

                Ok(Type::Void)
            }

            Statement::Function(func_def) => {
                self.env.push_scope();

                // Bind generic type parameters as Type::Any
                // This allows them to unify with any concrete type during type checking
                // (Type erasure approach - generics are erased at runtime like TypeScript)
                for type_param in &func_def.type_params {
                    self.env.bind(type_param.name.value.clone(), Type::Any);
                }

                // Bind parameters to scope with their actual types
                let mut param_types = Vec::new();
                for param in &func_def.parameters {
                    let param_type = self.type_expr_to_type(&param.type_annotation);
                    self.env.bind(param.name.value.clone(), param_type.clone());
                    param_types.push(param_type);
                }

                // Check body
                let mut body_type = Type::Void;
                for stmt in &func_def.body.statements {
                    body_type = self.check_statement(stmt)?;
                }

                self.env.pop_scope();

                let func_type = Type::function(param_types, body_type);
                self.env.bind(func_def.name.value.clone(), func_type.clone());
                Ok(func_type)
            }

            Statement::Component(comp_def) => {
                self.env.push_scope();

                // Bind parameters
                for param in &comp_def.parameters {
                    self.env.bind(param.name.value.clone(), Type::Any);
                }

                // Check body statements
                for stmt in &comp_def.body.statements {
                    self.check_statement(stmt)?;
                }

                self.env.pop_scope();

                let component_type = Type::Component(vec![]);
                self.env.bind(comp_def.name.value.clone(), component_type.clone());
                Ok(component_type)
            }

            Statement::Return(ret_stmt) => {
                self.infer_expression(&ret_stmt.value)
            }

            Statement::Expression(expr) => self.infer_expression(expr),

            Statement::If(if_stmt) => {
                let cond_type = self.infer_expression(&if_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(CompileError::Generic(format!(
                        "If condition must be bool, got {}",
                        cond_type
                    )));
                }

                // Check then branch and get the type of the last statement
                let mut then_type = Type::Void;
                for stmt in &if_stmt.then_branch.statements {
                    then_type = self.check_statement(stmt)?;
                }

                // Check else branch if present
                if let Some(else_branch) = &if_stmt.else_branch {
                    let else_type = self.check_statement(else_branch)?;

                    // If both branches return a value, unify the types
                    let subst = self.unify(&then_type, &else_type)?;
                    let unified_type = subst.apply(&then_type);
                    return Ok(unified_type);
                }

                // No else branch - can only return Void
                Ok(Type::Void)
            }

            Statement::While(while_stmt) => {
                let cond_type = self.infer_expression(&while_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(CompileError::Generic(format!(
                        "While condition must be bool, got {}",
                        cond_type
                    )));
                }

                for stmt in &while_stmt.body.statements {
                    self.check_statement(stmt)?;
                }

                Ok(Type::Void)
            }

            Statement::For(for_stmt) => {
                // Check init statement if present
                if let Some(init) = &for_stmt.init {
                    self.check_statement(init)?;
                }

                // Check condition
                let cond_type = self.infer_expression(&for_stmt.condition)?;
                if cond_type != Type::Bool {
                    return Err(CompileError::Generic(format!(
                        "For loop condition must be bool, got {}",
                        cond_type
                    )));
                }

                // Check update statement if present
                if let Some(update) = &for_stmt.update {
                    self.check_statement(update)?;
                }

                // Check body
                for stmt in &for_stmt.body.statements {
                    self.check_statement(stmt)?;
                }

                Ok(Type::Void)
            }

            Statement::ForIn(for_in_stmt) => {
                // Infer the type of the iterator expression
                let iterator_type = self.infer_expression(&for_in_stmt.iterator)?;

                // Verify that the iterator is an iterable type (Array, Range, etc.)
                match &iterator_type {
                    Type::Array(_) => {
                        // Valid array iterator
                    }
                    Type::Any => {
                        // Accept Any type (may be a range or other iterable)
                    }
                    _ => {
                        return Err(CompileError::Generic(format!(
                            "Cannot iterate over non-iterable type: {}",
                            iterator_type
                        )));
                    }
                }

                // Check body statements
                for stmt in &for_in_stmt.body.statements {
                    self.check_statement(stmt)?;
                }

                Ok(Type::Void)
            }

            Statement::Trait(trait_def) => {
                self.check_trait_definition(trait_def)?;
                Ok(Type::Void)
            }

            Statement::ImplBlock(impl_block) => {
                self.check_impl_block(impl_block)?;
                Ok(Type::Void)
            }

            _ => Ok(Type::Void),
        }
    }

    /// Infer the type of an expression using Hindley-Milner algorithm
    pub fn infer_expression(&mut self, expr: &Expression) -> Result<Type, CompileError> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(Type::Int),
            Expression::FloatLiteral(_) => Ok(Type::Float),
            Expression::StringLiteral(_) => Ok(Type::String),
            Expression::BoolLiteral(_) => Ok(Type::Bool),
            Expression::UnitLiteral => Ok(Type::Void),  // Unit type () maps to Void

            Expression::Identifier(ident) => {
                // Check if this is a namespaced identifier (e.g., console::log, document::write)
                // Treat all namespaced identifiers as external/built-in functions
                if ident.value.contains("::") {
                    return Ok(Type::Any);
                }

                // Special handling for Result enum constructors and built-in macros
                if ident.value == "Ok" || ident.value == "Err" || ident.value == "vec" {
                    // These are Result enum constructors or built-in macros, treat as functions
                    return Ok(Type::Any);
                }

                if let Some(ty) = self.env.lookup(&ident.value) {
                    Ok(ty.clone())
                } else {
                    // If not in environment, assume it's a forward reference or async function
                    // Return Type::Any to allow compilation to proceed
                    // TODO: Implement proper two-pass type checking to handle forward references
                    Ok(Type::Any)
                }
            }

            Expression::Prefix(prefix) => {
                self.check_prefix_expression(prefix)
            }

            Expression::Spread(spread) => {
                // Type check the inner expression - must be an array
                self.infer_expression(&spread.expression)
            }

            Expression::Infix(infix) => {
                self.check_infix_expression(infix)
            }

            Expression::FunctionCall(call) => {
                // Infer function type
                let func_type = self.infer_expression(&call.function)?;

                // Check if it's actually a function
                match &func_type {
                    Type::Function { params, return_type } => {
                        // Check argument count
                        if call.arguments.len() != params.len() {
                            return Err(CompileError::Generic(format!(
                                "Function expects {} arguments, got {}",
                                params.len(),
                                call.arguments.len()
                            )));
                        }

                        // Check argument types
                        for (i, (arg, expected_type)) in call.arguments.iter().zip(params.iter()).enumerate() {
                            let arg_type = self.infer_expression(arg)?;

                            // Try to unify the argument type with expected type
                            if let Err(e) = self.unify(&arg_type, expected_type) {
                                return Err(CompileError::Generic(format!(
                                    "Argument {} type mismatch: expected {}, got {}. {}",
                                    i + 1, expected_type, arg_type, e
                                )));
                            }
                        }

                        // Return the return type
                        Ok((**return_type).clone())
                    }
                    Type::Any => {
                        // If function type is Any (e.g., from external functions), skip checking
                        Ok(Type::Any)
                    }
                    _ => {
                        // Not a function type
                        Err(CompileError::Generic(format!(
                            "Cannot call non-function type: {}",
                            func_type
                        )))
                    }
                }
            }

            Expression::JsxElement(_) => {
                // JSX elements return component instances
                Ok(Type::Named("ReactElement".to_string()))
            }

            Expression::Lambda(lambda) => {
                self.env.push_scope();

                // Bind parameters
                for param in &lambda.parameters {
                    let param_type = if let Some(ref type_expr) = param.type_annotation {
                        self.type_expr_to_type(type_expr)
                    } else {
                        Type::Any // Type inference for untyped parameters
                    };
                    self.env.bind(param.name.value.clone(), param_type);
                }

                let body_type = self.infer_expression(&lambda.body)?;

                self.env.pop_scope();

                Ok(Type::function(vec![Type::Any; lambda.parameters.len()], body_type))
            }

            Expression::ArrayLiteral(array_lit) => {
                if array_lit.elements.is_empty() {
                    // Empty array - unknown element type
                    Ok(Type::Array(Box::new(Type::Any)))
                } else {
                    // Infer type from first element
                    let first_type = self.infer_expression(&array_lit.elements[0])?;

                    // Check all elements have compatible types
                    for elem in &array_lit.elements[1..] {
                        let elem_type = self.infer_expression(elem)?;
                        self.unify(&elem_type, &first_type)?;
                    }

                    Ok(Type::Array(Box::new(first_type)))
                }
            }

            Expression::TupleLiteral(tuple_lit) => {
                // Infer type for each element
                let mut element_types = Vec::new();
                for elem in &tuple_lit.elements {
                    let elem_type = self.infer_expression(elem)?;
                    element_types.push(elem_type);
                }
                Ok(Type::Tuple(element_types))
            }

            Expression::StructLiteral(_) => {
                // For now, return Any for struct literals
                Ok(Type::Any)
            }

            Expression::FieldAccess(field_access) => {
                // Infer object type
                let object_type = self.infer_expression(&field_access.object)?;
                let field_name = &field_access.field.value;

                // For String methods, return function type with proper signature
                if object_type == Type::String {
                    return Ok(match field_name.as_str() {
                        // Methods that return bool (with string argument)
                        "contains" | "starts_with" | "ends_with" => {
                            Type::Function {
                                params: vec![Type::String],
                                return_type: Box::new(Type::Bool),
                            }
                        },
                        // Methods that return bool (no arguments)
                        "is_empty" | "is_alphabetic" | "is_numeric" | "is_alphanumeric" => {
                            Type::Function {
                                params: vec![],
                                return_type: Box::new(Type::Bool),
                            }
                        },
                        // Methods that return String
                        "to_uppercase" | "to_lowercase" | "trim" | "trim_start" | "trim_end" |
                        "substring" | "replace" | "repeat" | "reverse" |
                        "pad_start" | "pad_end" => {
                            Type::Function {
                                params: vec![],
                                return_type: Box::new(Type::String),
                            }
                        },
                        // Methods that return i32
                        "len" | "count" => {
                            Type::Function {
                                params: vec![],
                                return_type: Box::new(Type::Int),
                            }
                        },
                        // Methods that return arrays
                        "split" | "lines" => {
                            Type::Function {
                                params: vec![Type::String],
                                return_type: Box::new(Type::Array(Box::new(Type::String))),
                            }
                        },
                        // Default: return Any
                        _ => Type::Any,
                    });
                }

                // Check if this is a method call on a user-defined type with impl blocks
                if let Type::Named(type_name) = &object_type {
                    if let Some(type_methods) = self.methods.get(type_name) {
                        if let Some(method_sig) = type_methods.get(field_name) {
                            // Return the method as a function type
                            return Ok(Type::Function {
                                params: method_sig.param_types.clone(),
                                return_type: Box::new(method_sig.return_type.clone()),
                            });
                        }
                    }
                }

                // For other types, return Any for now
                Ok(Type::Any)
            }

            Expression::IndexAccess(index_expr) => {
                // Process array and index expressions
                let array_type = self.infer_expression(&index_expr.array)?;
                let index_type = self.infer_expression(&index_expr.index)?;

                // Index must be an integer
                if index_type != Type::Int && index_type != Type::Any {
                    return Err(CompileError::Generic(format!(
                        "Array index must be an integer, got {}",
                        index_type
                    )));
                }

                // If array type is Array<T>, return T
                match array_type {
                    Type::Array(elem_type) => Ok(*elem_type),
                    Type::Any => Ok(Type::Any),
                    _ => Err(CompileError::Generic(format!(
                        "Cannot index into non-array type: {}",
                        array_type
                    ))),
                }
            }

            Expression::Match(_) => {
                // For now, return Any for match expressions
                Ok(Type::Any)
            }

            Expression::Borrow(borrow_expr) => {
                // Process the inner expression
                self.infer_expression(&borrow_expr.expression)?;
                // For now, return Any for borrow expressions
                Ok(Type::Any)
            }

            Expression::MutableBorrow(borrow_expr) => {
                // Process the inner expression
                self.infer_expression(&borrow_expr.expression)?;
                // For now, return Any for mutable borrow expressions
                Ok(Type::Any)
            }

            Expression::Dereference(deref_expr) => {
                // Process the inner expression
                self.infer_expression(&deref_expr.expression)?;
                // For now, return Any for dereference expressions
                Ok(Type::Any)
            }

            Expression::Range(_) => {
                // For now, return Any for range expressions
                Ok(Type::Any)
            }

            Expression::TryOperator(try_expr) => {
                // Infer the type of the inner expression
                let inner_type = self.infer_expression(&try_expr.expression)?;

                // If the inner type is Option<T>, extract T
                if let Type::Option(inner) = inner_type {
                    return Ok(*inner);
                }

                // For Result<T, E> and other cases, return Any
                // This is consistent with how Ok/Err are treated (line 304-307)
                // TODO: Add proper Result<T, E> type support to the type system
                Ok(Type::Any)
            }

            Expression::Ternary(ternary) => {
                // Infer types of all three parts
                let _condition_type = self.infer_expression(&ternary.condition)?;
                let true_type = self.infer_expression(&ternary.true_expr)?;
                let _false_type = self.infer_expression(&ternary.false_expr)?;

                // In a full implementation, we'd verify both branches have compatible types
                // For now, return the true branch type
                Ok(true_type)
            }
            Expression::TypeCast(type_cast) => {
                // Infer the type of the expression being cast
                let _expr_type = self.infer_expression(&type_cast.expression)?;

                // Return the target type specified in the cast - extract from TypeExpression
                match &type_cast.target_type {
                    TypeExpression::Named(ident) => {
                        Ok(Type::Named(ident.value.clone()))
                    }
                    _ => Ok(Type::Void), // Use Void for unknown complex types
                }
            }

            Expression::Await(await_expr) => {
                // Process the inner expression recursively
                // In a full implementation, we would verify that the inner expression
                // returns a Future<T> type and extract the T type
                self.infer_expression(&await_expr.expression)
            }

            Expression::IfExpression(if_expr) => {
                // Check the condition type
                let cond_type = self.infer_expression(&if_expr.condition)?;
                if cond_type != Type::Bool && cond_type != Type::Any {
                    return Err(CompileError::Generic(format!(
                        "If condition must be bool, got {}",
                        cond_type
                    )));
                }

                // Infer types of both branches
                let then_type = self.infer_expression(&if_expr.then_expr)?;
                if let Some(else_expr) = &if_expr.else_expr {
                    let else_type = self.infer_expression(else_expr)?;
                    // Try to unify both branch types
                    self.unify(&then_type, &else_type)?;
                }

                Ok(then_type)
            }

            Expression::Block(block) => {
                // Type-check all statements in the block and return the type of the last statement
                let mut last_type = Type::Void;
                for stmt in &block.statements {
                    last_type = self.check_statement(stmt)?;
                }
                Ok(last_type)
            }

            Expression::MacroCall(macro_call) => {
                // Type-check all macro arguments
                for arg in &macro_call.arguments {
                    self.infer_expression(arg)?;
                }
                // For now, return Any type for macro calls
                // In a full implementation, we'd expand the macro and infer its result type
                Ok(Type::Any)
            }

            Expression::CssMacro(_) => {
                // CSS macro returns a styles object (handled in Sprint 1 Task 1.6)
                Ok(Type::Any)
            }
        }
    }

    fn check_prefix_expression(&mut self, prefix: &PrefixExpression) -> Result<Type, CompileError> {
        let right_type = self.infer_expression(&prefix.right)?;
        let op = &prefix.operator.lexeme;

        match op.as_str() {
            "-" => {
                // Negation operator
                if !right_type.is_numeric() {
                    return Err(CompileError::Generic(format!(
                        "Cannot negate non-numeric type: {}",
                        right_type
                    )));
                }
                Ok(right_type)
            }
            "!" => {
                // Logical NOT operator
                // Allow Bool, Any, and Array types (arrays are truthy/falsy based on length)
                match &right_type {
                    Type::Bool | Type::Any => Ok(Type::Bool),
                    Type::Array(_) => Ok(Type::Bool), // Arrays can be used in boolean context (empty = false, non-empty = true)
                    _ => Err(CompileError::Generic(format!(
                        "Logical NOT expects bool, got {}",
                        right_type
                    )))
                }
            }
            _ => Err(CompileError::Generic(format!(
                "Unknown prefix operator: {}",
                op
            ))),
        }
    }

    fn check_infix_expression(&mut self, infix: &InfixExpression) -> Result<Type, CompileError> {
        let left_type = self.infer_expression(&infix.left)?;
        let right_type = self.infer_expression(&infix.right)?;

        let op = &infix.operator.lexeme;

        match op.as_str() {
            "+" | "-" | "*" | "/" | "%" => {
                // Arithmetic operations
                if !left_type.is_numeric() {
                    return Err(CompileError::Generic(format!(
                        "Expected numeric type, got {}",
                        left_type
                    )));
                }
                if !right_type.is_numeric() {
                    return Err(CompileError::Generic(format!(
                        "Expected numeric type, got {}",
                        right_type
                    )));
                }

                // Result is Float if either operand is Float
                if left_type == Type::Float || right_type == Type::Float {
                    Ok(Type::Float)
                } else {
                    Ok(Type::Int)
                }
            }

            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                // Comparison operations
                if !left_type.is_compatible_with(&right_type) {
                    return Err(CompileError::Generic(format!(
                        "Cannot compare {} with {}",
                        left_type, right_type
                    )));
                }
                Ok(Type::Bool)
            }

            "&&" | "||" => {
                // Logical operations
                if left_type != Type::Bool {
                    return Err(CompileError::Generic(format!(
                        "Expected bool, got {}",
                        left_type
                    )));
                }
                if right_type != Type::Bool {
                    return Err(CompileError::Generic(format!(
                        "Expected bool, got {}",
                        right_type
                    )));
                }
                Ok(Type::Bool)
            }

            _ => Ok(Type::Any),
        }
    }

    /// Unify two types and generate substitutions
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<Substitution, CompileError> {
        match (t1, t2) {
            // Same types unify trivially
            (a, b) if a == b => Ok(Substitution::new()),

            // Any unifies with everything
            (Type::Any, _) | (_, Type::Any) => Ok(Substitution::new()),

            // Type variables
            (Type::Var(id), ty) | (ty, Type::Var(id)) => {
                if self.occurs_check(*id, ty) {
                    Err(CompileError::Generic("Infinite type detected".to_string()))
                } else {
                    let mut subst = Substitution::new();
                    subst.insert(*id, ty.clone());
                    Ok(subst)
                }
            }

            // Functions
            (
                Type::Function {
                    params: p1,
                    return_type: r1,
                },
                Type::Function {
                    params: p2,
                    return_type: r2,
                },
            ) => {
                if p1.len() != p2.len() {
                    return Err(CompileError::Generic("Function arity mismatch".to_string()));
                }

                let mut subst = Substitution::new();

                // Unify parameters
                for (param1, param2) in p1.iter().zip(p2.iter()) {
                    let s = self.unify(&subst.apply(param1), &subst.apply(param2))?;
                    subst = subst.compose(&s);
                }

                // Unify return types
                let s = self.unify(&subst.apply(r1), &subst.apply(r2))?;
                Ok(subst.compose(&s))
            }

            // Arrays
            (Type::Array(t1), Type::Array(t2)) => self.unify(t1, t2),

            // Options
            (Type::Option(t1), Type::Option(t2)) => self.unify(t1, t2),

            // Tuples
            (Type::Tuple(t1), Type::Tuple(t2)) => {
                if t1.len() != t2.len() {
                    return Err(CompileError::Generic("Tuple length mismatch".to_string()));
                }

                let mut subst = Substitution::new();
                for (ty1, ty2) in t1.iter().zip(t2.iter()) {
                    let s = self.unify(&subst.apply(ty1), &subst.apply(ty2))?;
                    subst = subst.compose(&s);
                }
                Ok(subst)
            }

            // Numeric compatibility
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => Ok(Substitution::new()),

            _ => Err(CompileError::Generic(format!(
                "Cannot unify types {} and {}",
                t1, t2
            ))),
        }
    }

    /// Occurs check - prevent infinite types
    fn occurs_check(&self, var: usize, ty: &Type) -> bool {
        match ty {
            Type::Var(id) => *id == var,
            Type::Array(inner) => self.occurs_check(var, inner),
            Type::Option(inner) => self.occurs_check(var, inner),
            Type::Function { params, return_type } => {
                params.iter().any(|p| self.occurs_check(var, p))
                    || self.occurs_check(var, return_type)
            }
            Type::Tuple(types) => types.iter().any(|t| self.occurs_check(var, t)),
            Type::Union(types) => types.iter().any(|t| self.occurs_check(var, t)),
            _ => false,
        }
    }

    /// Solve all constraints and return final substitution
    pub fn solve_constraints(&mut self) -> Result<Substitution, CompileError> {
        let mut subst = Substitution::new();

        for (t1, t2) in &self.constraints.clone() {
            let s = self.unify(&subst.apply(t1), &subst.apply(t2))?;
            subst = subst.compose(&s);
        }

        Ok(subst)
    }

    /// Get free type variables in a type
    #[allow(dead_code)]
    fn free_vars(&self, ty: &Type) -> HashSet<usize> {
        match ty {
            Type::Var(id) => {
                let mut set = HashSet::new();
                set.insert(*id);
                set
            }
            Type::Array(inner) => self.free_vars(inner),
            Type::Option(inner) => self.free_vars(inner),
            Type::Function { params, return_type } => {
                let mut set = HashSet::new();
                for param in params {
                    set.extend(self.free_vars(param));
                }
                set.extend(self.free_vars(return_type));
                set
            }
            Type::Tuple(types) => {
                let mut set = HashSet::new();
                for ty in types {
                    set.extend(self.free_vars(ty));
                }
                set
            }
            Type::Union(types) => {
                let mut set = HashSet::new();
                for ty in types {
                    set.extend(self.free_vars(ty));
                }
                set
            }
            _ => HashSet::new(),
        }
    }

    /// Check a trait definition and store it for later validation
    fn check_trait_definition(&mut self, trait_def: &TraitDefinition) -> Result<(), CompileError> {
        let trait_name = trait_def.name.value.clone();
        let mut methods = HashMap::new();

        // Process each trait method signature
        for method in &trait_def.methods {
            let param_types: Vec<Type> = method.parameters.iter()
                .map(|p| self.type_expr_to_type(&p.type_annotation))
                .collect();

            let return_type = method.return_type.as_ref()
                .map(|rt| self.type_expr_to_type(rt))
                .unwrap_or(Type::Void);

            let signature = FunctionSignature {
                param_types,
                return_type,
            };

            methods.insert(method.name.value.clone(), signature);
        }

        // Store the trait information
        self.traits.insert(trait_name.clone(), TraitInfo {
            name: trait_name,
            methods,
        });

        Ok(())
    }

    /// Check an impl block and validate it against the trait (if any)
    fn check_impl_block(&mut self, impl_block: &ImplBlock) -> Result<(), CompileError> {
        let type_name = impl_block.type_name.value.clone();

        if let Some(trait_name_ident) = &impl_block.trait_name {
            let trait_name = trait_name_ident.value.clone();

            // Verify the trait exists and clone it to avoid borrow issues
            let trait_info = self.traits.get(&trait_name)
                .ok_or_else(|| CompileError::Generic(format!(
                    "Undefined trait: {}", trait_name
                )))?
                .clone();

            // Check that all trait methods are implemented
            for (method_name, expected_sig) in &trait_info.methods {
                let impl_method = impl_block.methods.iter()
                    .find(|m| m.name.value == *method_name)
                    .ok_or_else(|| CompileError::Generic(format!(
                        "Missing trait method '{}' in impl of '{}' for '{}'",
                        method_name, trait_name, type_name
                    )))?;

                // Build the actual signature from the impl method
                // Note: In the impl, Self resolves to the type being implemented
                let actual_param_types: Vec<Type> = impl_method.parameters.iter()
                    .map(|p| {
                        let ty = self.type_expr_to_type(&p.type_annotation);
                        // Replace Named(type_name) with the actual type when checking against Self
                        if ty == Type::Named(type_name.clone()) {
                            ty
                        } else {
                            ty
                        }
                    })
                    .collect();

                let actual_return_type = impl_method.return_type.as_ref()
                    .map(|rt| self.type_expr_to_type(rt))
                    .unwrap_or(Type::Void);

                let actual_sig = FunctionSignature {
                    param_types: actual_param_types,
                    return_type: actual_return_type,
                };

                // Create expected signature with Self replaced by the implementing type
                let expected_sig_resolved = FunctionSignature {
                    param_types: expected_sig.param_types.iter().map(|ty| {
                        if ty == &Type::Named("Self".to_string()) {
                            Type::Named(type_name.clone())
                        } else {
                            ty.clone()
                        }
                    }).collect(),
                    return_type: if expected_sig.return_type == Type::Named("Self".to_string()) {
                        Type::Named(type_name.clone())
                    } else {
                        expected_sig.return_type.clone()
                    },
                };

                // Verify signatures match
                if actual_sig != expected_sig_resolved {
                    return Err(CompileError::Generic(format!(
                        "Method '{}' signature mismatch in impl of '{}' for '{}'. Expected {:?}, found {:?}",
                        method_name, trait_name, type_name, expected_sig, actual_sig
                    )));
                }

                // Type check the method body
                self.env.push_scope();

                // Bind parameters
                for param in &impl_method.parameters {
                    let param_type = self.type_expr_to_type(&param.type_annotation);
                    self.env.bind(param.name.value.clone(), param_type);
                }

                // Check method body
                for stmt in &impl_method.body.statements {
                    self.check_statement(stmt)?;
                }

                self.env.pop_scope();
            }

            // Record that this type implements this trait
            self.impls.entry(type_name.clone())
                .or_insert_with(Vec::new)
                .push(trait_name.clone());

            // Store trait methods for this type
            for (method_name, sig) in &trait_info.methods {
                self.methods.entry(type_name.clone())
                    .or_insert_with(HashMap::new)
                    .insert(method_name.clone(), sig.clone());
            }
        } else {
            // Inherent impl - just type check the methods and store them
            for method in &impl_block.methods {
                self.env.push_scope();

                // Bind parameters
                for param in &method.parameters {
                    let param_type = self.type_expr_to_type(&param.type_annotation);
                    self.env.bind(param.name.value.clone(), param_type);
                }

                // Check method body
                for stmt in &method.body.statements {
                    self.check_statement(stmt)?;
                }

                self.env.pop_scope();

                // Store the method signature
                let param_types: Vec<Type> = method.parameters.iter()
                    .map(|p| self.type_expr_to_type(&p.type_annotation))
                    .collect();
                let return_type = method.return_type.as_ref()
                    .map(|rt| self.type_expr_to_type(rt))
                    .unwrap_or(Type::Void);

                let signature = FunctionSignature {
                    param_types,
                    return_type,
                };

                self.methods.entry(type_name.clone())
                    .or_insert_with(HashMap::new)
                    .insert(method.name.value.clone(), signature);
            }
        }

        Ok(())
    }

    /// Check if a type implements a given trait
    pub fn type_implements_trait(&self, type_name: &str, trait_name: &str) -> bool {
        self.impls.get(type_name)
            .map(|traits| traits.contains(&trait_name.to_string()))
            .unwrap_or(false)
    }

    /// Check trait bounds for a generic function call
    pub fn check_trait_bounds(&self, type_param_name: &str, bounds: &[String], actual_type: &Type) -> Result<(), CompileError> {
        for bound in bounds {
            // Extract the actual type name
            let type_name = match actual_type {
                Type::Named(name) => name.clone(),
                _ => {
                    // For non-named types, we can't check trait bounds yet
                    // In a full implementation, we'd need more sophisticated type matching
                    continue;
                }
            };

            if !self.type_implements_trait(&type_name, bound) {
                return Err(CompileError::Generic(format!(
                    "Trait bound not satisfied: type '{}' does not implement trait '{}' (required by type parameter '{}')",
                    type_name, bound, type_param_name
                )));
            }
        }
        Ok(())
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_primitives() {
        let mut checker = TypeChecker::new();
        assert!(checker.unify(&Type::Int, &Type::Int).is_ok());
        assert!(checker.unify(&Type::Int, &Type::Float).is_ok());
        assert!(checker.unify(&Type::String, &Type::Bool).is_err());
    }

    #[test]
    fn test_unify_type_variables() {
        let mut checker = TypeChecker::new();
        let result = checker.unify(&Type::Var(0), &Type::Int);
        assert!(result.is_ok());

        let subst = result.unwrap();
        assert_eq!(subst.apply(&Type::Var(0)), Type::Int);
    }

    #[test]
    fn test_occurs_check() {
        let checker = TypeChecker::new();
        let recursive_type = Type::Array(Box::new(Type::Var(0)));
        assert!(checker.occurs_check(0, &recursive_type));
    }
}
