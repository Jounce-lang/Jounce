use crate::ast::*;
use crate::errors::CompileError;
use crate::module_loader::{ModuleLoader, ExportedSymbol};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

/// Represents the fully resolved type of any expression in the compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedType {
    Integer,
    Float,
    String,
    Bool,
    Array(Box<ResolvedType>),
    Tuple(Vec<ResolvedType>),  // Tuple with element types
    Struct(String), // Struct type identified by name
    Signal(Box<ResolvedType>), // Reactive signal wrapping a type
    Unit, // Represents "no value", like for a statement.
    Component, // For JSX components
    VNode, // For JSX elements (virtual DOM nodes)
    ComplexType,
    Unknown, // For types we haven't implemented yet
}

impl std::fmt::Display for ResolvedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedType::Integer => write!(f, "i32"),
            ResolvedType::Float => write!(f, "f64"),
            ResolvedType::String => write!(f, "string"),
            ResolvedType::Bool => write!(f, "bool"),
            ResolvedType::Array(inner) => write!(f, "Array<{}>", inner),
            ResolvedType::Tuple(types) => {
                write!(f, "(")?;
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", ty)?;
                }
                write!(f, ")")
            }
            ResolvedType::Struct(name) => write!(f, "{}", name),
            ResolvedType::Signal(inner) => write!(f, "Signal<{}>", inner),
            ResolvedType::Unit => write!(f, "()"),
            ResolvedType::Component => write!(f, "Component"),
            ResolvedType::VNode => write!(f, "VNode"),
            ResolvedType::ComplexType => write!(f, "complex"),
            ResolvedType::Unknown => write!(f, "unknown"),
        }
    }
}

impl ResolvedType {
    pub fn is_copy(&self) -> bool {
        matches!(self, ResolvedType::Integer | ResolvedType::Float | ResolvedType::Bool | ResolvedType::String)
    }
}

/// A symbol table that manages scopes and declared variables.
struct SymbolTable {
    scopes: Vec<HashMap<String, ResolvedType>>,
}

impl SymbolTable {
    fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    fn define(&mut self, name: String, ty: ResolvedType) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    fn lookup(&self, name: &str) -> Option<ResolvedType> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
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

/// Stores struct definitions for type checking
struct StructTable {
    structs: HashMap<String, HashMap<String, ResolvedType>>,  // struct_name -> (field_name -> field_type)
}

impl StructTable {
    fn new() -> Self {
        Self { structs: HashMap::new() }
    }

    fn define(&mut self, name: String, fields: HashMap<String, ResolvedType>) {
        self.structs.insert(name, fields);
    }

    fn get_field_type(&self, struct_name: &str, field_name: &str) -> Option<ResolvedType> {
        self.structs
            .get(struct_name)
            .and_then(|fields| fields.get(field_name))
            .cloned()
    }

    #[allow(dead_code)] // For future struct validation
    fn exists(&self, struct_name: &str) -> bool {
        self.structs.contains_key(struct_name)
    }
}

/// Tracks enum definitions for exhaustiveness checking
struct EnumTable {
    enums: HashMap<String, Vec<String>>,  // enum_name -> list of variant names
}

impl EnumTable {
    fn new() -> Self {
        Self { enums: HashMap::new() }
    }

    fn define(&mut self, name: String, variants: Vec<String>) {
        self.enums.insert(name, variants);
    }

    fn get_variants(&self, enum_name: &str) -> Option<&Vec<String>> {
        self.enums.get(enum_name)
    }

    #[allow(dead_code)] // Used in future enum validation
    fn exists(&self, enum_name: &str) -> bool {
        self.enums.contains_key(enum_name)
    }
}

/// Traverses the AST to perform type checking and symbol resolution.
pub struct SemanticAnalyzer {
    symbols: SymbolTable,
    structs: StructTable,  // Track struct definitions
    enums: EnumTable,  // Track enum definitions
    in_component: bool,  // Track if we're inside a component
    reactive_variables: HashSet<String>,  // Track reactive variable names
    module_loader: ModuleLoader,  // Module loader for imports
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self::with_package_root("aloha-shirts")
    }

    pub fn with_package_root<P: Into<PathBuf>>(package_root: P) -> Self {
        Self {
            symbols: SymbolTable::new(),
            structs: StructTable::new(),
            enums: EnumTable::new(),
            in_component: false,
            reactive_variables: HashSet::new(),
            module_loader: ModuleLoader::new(package_root.into()),
        }
    }

    pub fn analyze_program(&mut self, program: &Program) -> Result<(), CompileError> {
        // First pass: collect struct and enum definitions
        for statement in &program.statements {
            match statement {
                Statement::Struct(struct_def) => {
                    self.register_struct(struct_def)?;
                }
                Statement::Enum(enum_def) => {
                    self.register_enum(enum_def)?;
                }
                _ => {}
            }
        }

        // Second pass: analyze statements
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }
        Ok(())
    }

    fn register_struct(&mut self, struct_def: &StructDefinition) -> Result<(), CompileError> {
        let mut field_types = HashMap::new();
        for (field_name, field_type_expr) in &struct_def.fields {
            // Convert TypeExpression to ResolvedType
            let field_type = self.type_expression_to_resolved_type(field_type_expr);
            field_types.insert(field_name.value.clone(), field_type);
        }
        self.structs.define(struct_def.name.value.clone(), field_types);
        Ok(())
    }

    fn register_enum(&mut self, enum_def: &EnumDefinition) -> Result<(), CompileError> {
        let variant_names: Vec<String> = enum_def.variants.iter()
            .map(|v| v.name.value.clone())
            .collect();
        self.enums.define(enum_def.name.value.clone(), variant_names);
        Ok(())
    }

    fn type_expression_to_resolved_type(&self, type_expr: &TypeExpression) -> ResolvedType {
        match type_expr {
            TypeExpression::Named(ident) => {
                match ident.value.as_str() {
                    "i32" => ResolvedType::Integer,
                    "f64" => ResolvedType::Float,
                    "bool" => ResolvedType::Bool,
                    "string" => ResolvedType::String,
                    _ => {
                        // Check if it's a struct type
                        if self.structs.exists(&ident.value) {
                            ResolvedType::Struct(ident.value.clone())
                        } else {
                            ResolvedType::Unknown
                        }
                    }
                }
            }
            TypeExpression::Generic(ident, args) => {
                // Handle generic types like Array<T>
                if ident.value == "Array" && !args.is_empty() {
                    let inner_type = self.type_expression_to_resolved_type(&args[0]);
                    ResolvedType::Array(Box::new(inner_type))
                } else {
                    ResolvedType::Unknown
                }
            }
            TypeExpression::Tuple(types) => {
                // Convert tuple type expression to resolved tuple type
                let resolved_types: Vec<ResolvedType> = types.iter()
                    .map(|t| self.type_expression_to_resolved_type(t))
                    .collect();
                ResolvedType::Tuple(resolved_types)
            }
            TypeExpression::Reference(_inner) => {
                // For now, return Unknown for reference types
                ResolvedType::Unknown
            }
            TypeExpression::MutableReference(_inner) => {
                // For now, return Unknown for mutable reference types
                ResolvedType::Unknown
            }
            TypeExpression::Slice(inner) => {
                // Slices are array views - recursively process the inner type
                // Return ResolvedType::Array with the inner type
                let inner_type = self.type_expression_to_resolved_type(inner);
                ResolvedType::Array(Box::new(inner_type))
            }
            TypeExpression::Function(_param_types, _return_type) => {
                // For now, return Unknown for function types
                // In a full implementation, we'd track function signatures properly
                ResolvedType::Unknown
            }
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<ResolvedType, CompileError> {
        match stmt {
            Statement::Use(use_stmt) => self.analyze_use_statement(use_stmt),
            Statement::Let(let_stmt) => self.analyze_let_statement(let_stmt),
            Statement::Const(const_decl) => self.analyze_const_declaration(const_decl),
            Statement::Assignment(assign_stmt) => {
                // Check that the target is a valid assignment target
                match &assign_stmt.target {
                    Expression::Identifier(ident) => {
                        // Check that variable exists
                        if self.symbols.lookup(&ident.value).is_none() {
                            return Err(CompileError::Generic(format!(
                                "Cannot assign to undefined variable '{}'",
                                ident.value
                            )));
                        }
                    }
                    Expression::FieldAccess(_) | Expression::IndexAccess(_) => {
                        // For field access and index access, analyze the target expression
                        self.analyze_expression(&assign_stmt.target)?;
                    }
                    _ => {
                        return Err(CompileError::Generic(
                            "Invalid assignment target".to_string()
                        ));
                    }
                }

                // Analyze the value expression
                self.analyze_expression(&assign_stmt.value)?;

                Ok(ResolvedType::Unit)
            }
            Statement::Return(return_stmt) => self.analyze_return_statement(return_stmt),
            Statement::Expression(expr) => self.analyze_expression(expr),
            Statement::If(if_stmt) => self.analyze_if_statement(if_stmt),
            Statement::While(while_stmt) => self.analyze_while_statement(while_stmt),
            Statement::For(for_stmt) => self.analyze_for_statement(for_stmt),
            Statement::ForIn(for_in_stmt) => self.analyze_for_in_statement(for_in_stmt),
            Statement::MacroInvocation(_) => Ok(ResolvedType::Unit),
            Statement::Function(func_def) => {
                // Enter a new scope for the function body
                self.symbols.enter_scope();

                // Register function parameters in the new scope
                for param in &func_def.parameters {
                    let param_type = self.type_expression_to_resolved_type(&param.type_annotation);
                    self.symbols.define(param.name.value.clone(), param_type);
                }

                // Analyze the function body
                for stmt in &func_def.body.statements {
                    self.analyze_statement(stmt)?;
                }

                // Exit the function scope
                self.symbols.exit_scope();

                Ok(ResolvedType::Unit)
            }
            Statement::Component(comp) => {
                // Mark that we're inside a component
                let was_in_component = self.in_component;
                self.in_component = true;

                // Analyze component body statements
                for stmt in &comp.body.statements {
                    self.analyze_statement(stmt)?;
                }

                self.in_component = was_in_component;
                Ok(ResolvedType::Component)
            }
            Statement::ExternBlock(_) => Ok(ResolvedType::Unit),
            Statement::Struct(_) => Ok(ResolvedType::Unit),
            Statement::Enum(_) => Ok(ResolvedType::Unit),
            Statement::ImplBlock(_) => Ok(ResolvedType::Unit),
            Statement::Trait(_) => Ok(ResolvedType::Unit),
        }
    }

    fn analyze_if_statement(&mut self, stmt: &IfStatement) -> Result<ResolvedType, CompileError> {
        let cond_type = self.analyze_expression(&stmt.condition)?;
        if cond_type != ResolvedType::Bool && cond_type != ResolvedType::Integer {
            return Err(CompileError::Generic(format!(
                "If condition must be bool or integer, got '{}'",
                cond_type
            )));
        }

        // Analyze then branch
        for s in &stmt.then_branch.statements {
            self.analyze_statement(s)?;
        }

        // Analyze else branch if present
        if let Some(else_stmt) = &stmt.else_branch {
            self.analyze_statement(else_stmt)?;
        }

        Ok(ResolvedType::Unit)
    }

    fn analyze_while_statement(&mut self, stmt: &WhileStatement) -> Result<ResolvedType, CompileError> {
        let cond_type = self.analyze_expression(&stmt.condition)?;
        if cond_type != ResolvedType::Bool && cond_type != ResolvedType::Integer {
            return Err(CompileError::Generic(format!(
                "While condition must be bool or integer, got '{}'",
                cond_type
            )));
        }

        // Analyze loop body
        for s in &stmt.body.statements {
            self.analyze_statement(s)?;
        }

        Ok(ResolvedType::Unit)
    }

    fn analyze_for_statement(&mut self, stmt: &ForStatement) -> Result<ResolvedType, CompileError> {
        // Analyze init statement if present
        if let Some(init) = &stmt.init {
            self.analyze_statement(init)?;
        }

        // Analyze condition
        let cond_type = self.analyze_expression(&stmt.condition)?;
        if cond_type != ResolvedType::Bool && cond_type != ResolvedType::Integer {
            return Err(CompileError::Generic(format!(
                "For loop condition must be bool or integer, got '{}'",
                cond_type
            )));
        }

        // Analyze update statement if present
        if let Some(update) = &stmt.update {
            self.analyze_statement(update)?;
        }

        // Analyze loop body
        for s in &stmt.body.statements {
            self.analyze_statement(s)?;
        }

        Ok(ResolvedType::Unit)
    }

    fn analyze_for_in_statement(&mut self, stmt: &ForInStatement) -> Result<ResolvedType, CompileError> {
        // Analyze the iterator expression
        let iterator_type = self.analyze_expression(&stmt.iterator)?;

        // Verify that the iterator type is iterable (Array, Range, etc.)
        // For now, we accept Array types and Unknown types
        match &iterator_type {
            ResolvedType::Array(_) => {
                // Valid iterator type
            }
            ResolvedType::Unknown => {
                // Accept unknown types (may be a range or other iterable)
            }
            _ => {
                return Err(CompileError::Generic(format!(
                    "Cannot iterate over non-iterable type '{}'",
                    iterator_type
                )));
            }
        }

        // Analyze body statements
        for s in &stmt.body.statements {
            self.analyze_statement(s)?;
        }

        Ok(ResolvedType::Unit)
    }

    fn analyze_use_statement(&mut self, use_stmt: &UseStatement) -> Result<ResolvedType, CompileError> {
        // Convert Identifier vec to String vec for module path
        let module_path: Vec<String> = use_stmt.path.iter()
            .map(|ident| ident.value.clone())
            .collect();

        // If no specific imports are specified, import all exports
        if use_stmt.imports.is_empty() {
            // Wildcard import (use module::*)
            let exports = self.module_loader.get_all_exports(&module_path)
                .map_err(|e| CompileError::Generic(format!("Failed to load module: {}", e)))?;

            for (name, export) in exports {
                self.import_symbol(&name, &export)?;
            }
        } else {
            // Selective imports (use module::{A, B, C})
            let import_names: Vec<String> = use_stmt.imports.iter()
                .map(|ident| ident.value.clone())
                .collect();

            let exports = self.module_loader.get_exports(&module_path, &import_names)
                .map_err(|e| CompileError::Generic(format!("Failed to load exports: {}", e)))?;

            for (name, export) in exports {
                self.import_symbol(&name, &export)?;
            }
        }

        Ok(ResolvedType::Unit)
    }

    fn import_symbol(&mut self, name: &str, export: &ExportedSymbol) -> Result<(), CompileError> {
        match export {
            ExportedSymbol::Function(_func) => {
                // For now, register the function name as Unknown type
                // In a full implementation, we'd track function signatures
                self.symbols.define(name.to_string(), ResolvedType::Unknown);
            }
            ExportedSymbol::Struct(struct_def) => {
                // Register the struct definition
                self.register_struct(struct_def)?;
                // Also add the struct name as a type
                self.symbols.define(name.to_string(), ResolvedType::Struct(name.to_string()));
            }
            ExportedSymbol::Enum(enum_def) => {
                // Register the enum definition
                self.register_enum(enum_def)?;
                // Add enum name to symbols
                self.symbols.define(name.to_string(), ResolvedType::Unknown);
            }
            ExportedSymbol::Const(_const_decl) => {
                // Register the constant as a symbol
                // Type will be inferred from the constant's value
                self.symbols.define(name.to_string(), ResolvedType::Unknown);
            }
            ExportedSymbol::Type(_) => {
                // Type alias - for now treat as Unknown
                self.symbols.define(name.to_string(), ResolvedType::Unknown);
            }
        }
        Ok(())
    }

    fn analyze_const_declaration(&mut self, decl: &ConstDeclaration) -> Result<ResolvedType, CompileError> {
        // Get the type annotation if provided
        let annotation_type = decl
            .type_annotation
            .as_ref()
            .map(|ty| self.type_expression_to_resolved_type(ty));

        // Analyze the value expression
        let mut value_type = if let Some(expected) = annotation_type.as_ref() {
            self.analyze_expression_with_expected(&decl.value, Some(expected))?
        } else {
            self.analyze_expression(&decl.value)?
        };

        // Check type compatibility if annotation is provided
        if let Some(expected_type) = annotation_type {
            if !self.types_compatible(&expected_type, &value_type) {
                return Err(CompileError::Generic(format!(
                    "Type mismatch in const declaration '{}': expected '{}', found '{}'",
                    decl.name.value, expected_type, value_type
                )));
            }
            value_type = expected_type;
        }

        // Register the constant in the symbol table
        self.symbols.define(decl.name.value.clone(), value_type);

        Ok(ResolvedType::Unit)
    }

    fn analyze_let_statement(&mut self, stmt: &LetStatement) -> Result<ResolvedType, CompileError> {
        use crate::ast::Pattern;

        let annotation_type = stmt
            .type_annotation
            .as_ref()
            .map(|ty| self.type_expression_to_resolved_type(ty));

        let mut value_type = if let Some(expected) = annotation_type.as_ref() {
            self.analyze_expression_with_expected(&stmt.value, Some(expected))?
        } else {
            self.analyze_expression(&stmt.value)?
        };

        if let Some(expected_type) = annotation_type {
            if !self.types_compatible(&expected_type, &value_type) {
                let pattern_str = match &stmt.pattern {
                    Pattern::Identifier(id) => id.value.clone(),
                    Pattern::Tuple(_) => "(tuple)".to_string(),
                    _ => "(pattern)".to_string(),
                };
                return Err(CompileError::Generic(format!(
                    "Type mismatch in let binding '{}': expected '{}', found '{}'",
                    pattern_str, expected_type, value_type
                )));
            }
            value_type = expected_type;
        }

        // Register all bound identifiers from the pattern
        let identifiers = stmt.pattern.bound_identifiers();
        for (idx, ident) in identifiers.iter().enumerate() {
            // For tuple patterns, extract the corresponding type from the tuple
            let ident_type = match &stmt.pattern {
                Pattern::Tuple(_) => {
                    // If value is a tuple type, extract the element type
                    if let ResolvedType::Tuple(types) = &value_type {
                        types.get(idx).cloned().unwrap_or(ResolvedType::Unknown)
                    } else {
                        value_type.clone()
                    }
                }
                _ => value_type.clone(),
            };

            // Auto-wrap in Signal<T> if inside a component and type is simple
            let final_type = if self.in_component && self.should_be_reactive(&ident_type) {
                self.reactive_variables.insert(ident.value.clone());
                println!("[Reactive] Variable '{}' marked as reactive: {}", ident.value, ident_type);
                ResolvedType::Signal(Box::new(ident_type))
            } else {
                ident_type
            };

            self.symbols.define(ident.value.clone(), final_type);
        }

        Ok(ResolvedType::Unit)
    }

    fn should_be_reactive(&self, ty: &ResolvedType) -> bool {
        // Only wrap primitives in Signal<T> automatically
        matches!(ty,
            ResolvedType::Integer |
            ResolvedType::Float |
            ResolvedType::String |
            ResolvedType::Bool
        )
    }

    fn analyze_return_statement(&mut self, stmt: &ReturnStatement) -> Result<ResolvedType, CompileError> {
        self.analyze_expression(&stmt.value)?;
        Ok(ResolvedType::Unit)
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Result<ResolvedType, CompileError> {
        self.analyze_expression_with_expected(expr, None)
    }

    fn analyze_expression_with_expected(
        &mut self,
        expr: &Expression,
        expected: Option<&ResolvedType>,
    ) -> Result<ResolvedType, CompileError> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(ResolvedType::Integer),
            Expression::FloatLiteral(_) => Ok(ResolvedType::Float),
            Expression::StringLiteral(_) => Ok(ResolvedType::String),
            Expression::BoolLiteral(_) => Ok(ResolvedType::Bool),
            Expression::Identifier(ident) => {
                if let Some(ty) = self.symbols.lookup(&ident.value) {
                    Ok(ty)
                } else {
                    Ok(ResolvedType::ComplexType)
                }
            }
            Expression::Prefix(prefix_expr) => {
                self.analyze_expression_with_expected(&prefix_expr.right, None)?;
                Ok(ResolvedType::Integer)
            }
            Expression::Spread(spread_expr) => {
                // Analyze the inner expression of spread operator
                self.analyze_expression_with_expected(&spread_expr.expression, None)?;
                // Spread preserves the type of the array being spread
                Ok(ResolvedType::Unit)  // Type will be inferred from context
            }
            Expression::Infix(infix_expr) => self.analyze_infix_expression(infix_expr),
            Expression::ArrayLiteral(array_lit) => {
                let mut expected_element_type = match expected {
                    Some(ResolvedType::Array(inner)) => Some((**inner).clone()),
                    Some(other) => {
                        return Err(CompileError::Generic(format!(
                            "Expected type '{}' does not match array literal",
                            other
                        )));
                    }
                    None => None,
                };

                if array_lit.elements.is_empty() {
                    return Ok(ResolvedType::Array(Box::new(
                        expected_element_type.unwrap_or(ResolvedType::Unknown),
                    )));
                }

                for (index, elem) in array_lit.elements.iter().enumerate() {
                    let elem_type = self.analyze_expression_with_expected(
                        elem,
                        expected_element_type.as_ref(),
                    )?;
                    if let Some(expected_ty) = &expected_element_type {
                        if !self.types_compatible(expected_ty, &elem_type) {
                            return Err(CompileError::Generic(format!(
                                "Array element type mismatch at index {}: expected '{}', found '{}'",
                                index,
                                expected_ty,
                                elem_type
                            )));
                        }
                    } else {
                        expected_element_type = Some(elem_type.clone());
                    }
                }

                Ok(ResolvedType::Array(Box::new(
                    expected_element_type.unwrap_or(ResolvedType::Unknown),
                )))
            }
            Expression::TupleLiteral(tuple_lit) => {
                // Infer type for each element
                let mut element_types = Vec::new();
                for elem in &tuple_lit.elements {
                    let elem_type = self.analyze_expression_with_expected(elem, None)?;
                    element_types.push(elem_type);
                }
                Ok(ResolvedType::Tuple(element_types))
            }
            Expression::StructLiteral(struct_lit) => {
                // Check if struct exists
                if !self.structs.exists(&struct_lit.name.value) {
                    return Err(CompileError::Generic(format!(
                        "Unknown struct type: '{}'",
                        struct_lit.name.value
                    )));
                }

                // Analyze all field values and check types
                for (field_name, field_value) in &struct_lit.fields {
                    let _value_type =
                        self.analyze_expression_with_expected(field_value, None)?;

                    // Look up expected field type
                    if let Some(_expected_type) =
                        self.structs.get_field_type(&struct_lit.name.value, &field_name.value)
                    {
                        // TODO: Add type compatibility checking here
                        // For now, just accept any type
                    } else {
                        return Err(CompileError::Generic(format!(
                            "Struct '{}' has no field named '{}'",
                            struct_lit.name.value,
                            field_name.value
                        )));
                    }
                }

                // Return the struct type
                Ok(ResolvedType::Struct(struct_lit.name.value.clone()))
            }
            Expression::FieldAccess(field_access) => {
                // Analyze the object expression to get its type
                let object_type =
                    self.analyze_expression_with_expected(&field_access.object, None)?;

                // If it's a struct type, look up the field
                match object_type {
                    ResolvedType::Struct(struct_name) => {
                        if let Some(field_type) =
                            self.structs.get_field_type(&struct_name, &field_access.field.value)
                        {
                            Ok(field_type)
                        } else {
                            Err(CompileError::Generic(format!(
                                "Struct '{}' has no field named '{}'",
                                struct_name,
                                field_access.field.value
                            )))
                        }
                    }
                    _ => {
                        // For non-struct types, return Unknown for now
                        Ok(ResolvedType::Unknown)
                    }
                }
            }
            Expression::IndexAccess(index_expr) => {
                // Analyze the array expression
                let array_type =
                    self.analyze_expression_with_expected(&index_expr.array, None)?;

                // Analyze the index expression (should be integer)
                let index_type =
                    self.analyze_expression_with_expected(&index_expr.index, None)?;
                if index_type != ResolvedType::Integer && index_type != ResolvedType::Unknown {
                    return Err(CompileError::Generic(format!(
                        "Array index must be an integer, got '{}'",
                        index_type
                    )));
                }

                // Return the element type if it's an array
                match array_type {
                    ResolvedType::Array(element_type) => Ok(*element_type),
                    ResolvedType::Unknown => Ok(ResolvedType::Unknown),
                    _ => Err(CompileError::Generic(format!(
                        "Cannot index into non-array type '{}'",
                        array_type
                    ))),
                }
            }
            Expression::Match(match_expr) => {
                // Analyze the scrutinee to get its type
                let scrutinee_type =
                    self.analyze_expression_with_expected(&match_expr.scrutinee, None)?;

                // Check exhaustiveness if matching on an enum
                self.check_match_exhaustiveness(match_expr, &scrutinee_type)?;

                // Analyze all match arms and infer the result type
                if match_expr.arms.is_empty() {
                    return Ok(ResolvedType::Unit);
                }

                // Get the type of the first arm's body
                let first_arm_type =
                    self.analyze_expression_with_expected(&match_expr.arms[0].body, None)?;

                // Verify all other arms return the same type
                for arm in &match_expr.arms[1..] {
                    let arm_type = self.analyze_expression_with_expected(&arm.body, None)?;

                    // Check type compatibility
                    if !self.types_compatible(&arm_type, &first_arm_type) {
                        return Err(CompileError::Generic(format!(
                            "Match arms have incompatible types: expected '{}', found '{}'",
                            first_arm_type,
                            arm_type
                        )));
                    }
                }

                // All arms have compatible types, return the common type
                Ok(first_arm_type)
            }
            Expression::JsxElement(_) => Ok(ResolvedType::VNode),
            Expression::FunctionCall(func_call) => self.analyze_function_call(func_call),
            Expression::Lambda(_) => Ok(ResolvedType::Unknown),
            Expression::Borrow(borrow_expr) => {
                self.analyze_expression_with_expected(&borrow_expr.expression, None)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::MutableBorrow(borrow_expr) => {
                self.analyze_expression_with_expected(&borrow_expr.expression, None)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::Dereference(deref_expr) => {
                self.analyze_expression_with_expected(&deref_expr.expression, None)?;
                Ok(ResolvedType::Unknown)
            }
            Expression::Range(_range_expr) => {
                // Range expressions are placeholders for now
                // In a full implementation, we'd analyze the start and end expressions
                Ok(ResolvedType::Unknown)
            }
            Expression::TryOperator(try_expr) => {
                // Analyze the inner expression and return its type
                // In a full implementation, we would verify that the inner expression
                // returns a Result<T, E> type and extract the T type
                self.analyze_expression_with_expected(&try_expr.expression, None)
            }
            Expression::Ternary(ternary) => {
                // Analyze all three parts
                self.analyze_expression(&ternary.condition)?;
                let true_type = self.analyze_expression_with_expected(&ternary.true_expr, expected)?;
                let _false_type = self.analyze_expression_with_expected(&ternary.false_expr, expected)?;

                // In a full implementation, we would verify that both branches have compatible types
                // For now, just return the type of the true branch
                Ok(true_type.clone())
            }
            Expression::TypeCast(type_cast) => {
                // Analyze the expression being cast
                self.analyze_expression(&type_cast.expression)?;

                // Return the target type - extract from TypeExpression
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
                // Analyze the inner expression and return its type
                // In a full implementation, we would verify that the inner expression
                // returns a Future<T> type and extract the T type
                self.analyze_expression(&await_expr.expression)
            }
            Expression::IfExpression(if_expr) => {
                // Analyze the condition
                let cond_type =
                    self.analyze_expression_with_expected(&if_expr.condition, None)?;
                if cond_type != ResolvedType::Bool && cond_type != ResolvedType::Integer
                    && cond_type != ResolvedType::Unknown
                {
                    return Err(CompileError::Generic(format!(
                        "If condition must be bool or integer, got '{}'",
                        cond_type
                    )));
                }

                // Analyze then expression
                let then_type =
                    self.analyze_expression_with_expected(&if_expr.then_expr, None)?;

                // Analyze else expression if present
                if let Some(else_expr) = &if_expr.else_expr {
                    let else_type =
                        self.analyze_expression_with_expected(else_expr, None)?;
                    // Check type compatibility
                    if !self.types_compatible(&then_type, &else_type) {
                        return Err(CompileError::Generic(format!(
                            "If expression branches have incompatible types: then '{}', else '{}'",
                            then_type,
                            else_type
                        )));
                    }
                }

                Ok(then_type)
            }
            Expression::Block(block) => {
                // Analyze all statements in the block
                let mut last_type = ResolvedType::Unit;
                for stmt in &block.statements {
                    last_type = self.analyze_statement(stmt)?;
                }
                Ok(last_type)
            }
            Expression::MacroCall(macro_call) => {
                // Analyze all macro arguments
                for arg in &macro_call.arguments {
                    self.analyze_expression_with_expected(arg, None)?;
                }
                // For now, return Unknown type for macro calls
                // In a full implementation, we'd expand the macro and analyze its result
                Ok(ResolvedType::Unknown)
            }
        }
    }

    fn analyze_infix_expression(&mut self, expr: &InfixExpression) -> Result<ResolvedType, CompileError> {
        let left_type = self.analyze_expression(&expr.left)?;
        let right_type = self.analyze_expression(&expr.right)?;

        // Handle Unknown and ComplexType gracefully - allow them through
        match (&left_type, &right_type) {
            // If either side is Unknown or ComplexType, return Integer (common case)
            (ResolvedType::Unknown, _) | (_, ResolvedType::Unknown) => Ok(ResolvedType::Integer),
            (ResolvedType::ComplexType, _) | (_, ResolvedType::ComplexType) => Ok(ResolvedType::Integer),
            // Both sides are Integer - valid operation
            (ResolvedType::Integer, ResolvedType::Integer) => Ok(ResolvedType::Integer),
            // Both sides are Float - valid operation
            (ResolvedType::Float, ResolvedType::Float) => Ok(ResolvedType::Float),
            // Mixed Integer/Float - promote to Float
            (ResolvedType::Integer, ResolvedType::Float) | (ResolvedType::Float, ResolvedType::Integer) => {
                Ok(ResolvedType::Float)
            }
            // String concatenation
            (ResolvedType::String, ResolvedType::String) if expr.operator.lexeme == "+" => {
                Ok(ResolvedType::String)
            }
            // Logical operators (&&, ||) with Bool operands
            (ResolvedType::Bool, ResolvedType::Bool) if matches!(expr.operator.lexeme.as_str(), "&&" | "||") => {
                Ok(ResolvedType::Bool)
            }
            // Comparison operators return Bool
            _ if matches!(expr.operator.lexeme.as_str(), "==" | "!=" | "<" | ">" | "<=" | ">=") => {
                Ok(ResolvedType::Bool)
            }
            // Invalid operation
            _ => Err(CompileError::Generic(format!(
                "Cannot apply operator '{}' to types '{}' and '{}'",
                expr.operator.lexeme, left_type, right_type
            )))
        }
    }

    fn analyze_function_call(&mut self, func_call: &FunctionCall) -> Result<ResolvedType, CompileError> {
        // Analyze all arguments
        for arg in &func_call.arguments {
            self.analyze_expression_with_expected(arg, None)?;
        }

        // Check if this is a method call (function is a field access)
        if let Expression::FieldAccess(field_access) = &*func_call.function {
            // Get the type of the object
            let object_type = self.analyze_expression(&field_access.object)?;
            let method_name = &field_access.field.value;

            // Handle String methods
            if object_type == ResolvedType::String {
                return Ok(match method_name.as_str() {
                    // Methods that return bool
                    "contains" | "starts_with" | "ends_with" |
                    "is_empty" | "is_alphabetic" | "is_numeric" | "is_alphanumeric" => {
                        ResolvedType::Bool
                    },
                    // Methods that return String
                    "to_uppercase" | "to_lowercase" | "trim" | "trim_start" | "trim_end" |
                    "substring" | "replace" | "repeat" | "reverse" |
                    "pad_start" | "pad_end" => {
                        ResolvedType::String
                    },
                    // Methods that return i32
                    "len" | "count" => {
                        ResolvedType::Integer
                    },
                    // Methods that return Option<i32> (use Unknown for now since Option isn't in ResolvedType)
                    "find" | "rfind" | "char_at" | "pop" => {
                        ResolvedType::Unknown
                    },
                    // Methods that return Vec<String>
                    "split" | "lines" => {
                        ResolvedType::Array(Box::new(ResolvedType::String))
                    },
                    // Default: unknown method
                    _ => ResolvedType::Unknown,
                });
            }

            // TODO: Add method type inference for other types (Vec, HashMap, etc.)
        }

        // For regular function calls, return Unknown for now
        Ok(ResolvedType::Unknown)
    }

    fn types_compatible(&self, type1: &ResolvedType, type2: &ResolvedType) -> bool {
        match (type1, type2) {
            (ResolvedType::Unknown, _) | (_, ResolvedType::Unknown) => true,
            (ResolvedType::ComplexType, _) | (_, ResolvedType::ComplexType) => true,
            (ResolvedType::Array(a), ResolvedType::Array(b)) => {
                self.types_compatible(a.as_ref(), b.as_ref())
            }
            (ResolvedType::Signal(a), ResolvedType::Signal(b)) => {
                self.types_compatible(a.as_ref(), b.as_ref())
            }
            (ResolvedType::Tuple(types_a), ResolvedType::Tuple(types_b)) => {
                if types_a.len() != types_b.len() {
                    return false;
                }
                for (a, b) in types_a.iter().zip(types_b.iter()) {
                    if !self.types_compatible(a, b) {
                        return false;
                    }
                }
                true
            }
            _ => type1 == type2,
        }
    }

    fn check_match_exhaustiveness(&self, match_expr: &MatchExpression, _scrutinee_type: &ResolvedType) -> Result<(), CompileError> {
        // Collect all patterns from match arms
        let mut covered_variants: HashSet<String> = HashSet::new();
        let mut has_wildcard = false;

        for arm in &match_expr.arms {
            // Check all patterns in OR pattern: 3 | 4 | 5 => ...
            for pattern in &arm.patterns {
                match pattern {
                    Pattern::Wildcard => {
                        has_wildcard = true;
                    }
                    Pattern::Tuple(_) => {
                        // TODO: Check tuple pattern exhaustiveness
                        has_wildcard = true;  // Treat as wildcard for now
                    }
                    Pattern::EnumVariant { name, .. } => {
                        // Extract the variant name (could be "Color::Red" format)
                        let variant_name = if name.value.contains("::") {
                            // Split "Color::Red" into ["Color", "Red"]
                            let parts: Vec<&str> = name.value.split("::").collect();
                            if parts.len() == 2 {
                                parts[1].to_string()
                            } else {
                                name.value.clone()
                            }
                        } else {
                            name.value.clone()
                        };
                        covered_variants.insert(variant_name);
                    }
                    Pattern::Identifier(_) => {
                        // Identifier patterns act like wildcards
                        has_wildcard = true;
                    }
                    Pattern::Literal(_) => {
                        // Literals don't contribute to enum exhaustiveness
                    }
                }
            }
        }

        // If we have a wildcard pattern, match is exhaustive
        if has_wildcard {
            return Ok(());
        }

        // Check if we're matching on an enum
        // Try to extract enum name from first variant pattern
        for arm in &match_expr.arms {
            // Check all patterns (handles OR patterns)
            for pattern in &arm.patterns {
            if let Pattern::EnumVariant { name, .. } = pattern {
                if name.value.contains("::") {
                    let parts: Vec<&str> = name.value.split("::").collect();
                    if parts.len() == 2 {
                        let enum_name = parts[0];

                        // Check if this enum exists
                        if let Some(variants) = self.enums.get_variants(enum_name) {
                            // Check if all variants are covered
                            let uncovered: Vec<String> = variants.iter()
                                .filter(|v| !covered_variants.contains(*v))
                                .cloned()
                                .collect();

                            if !uncovered.is_empty() {
                                return Err(CompileError::Generic(format!(
                                    "Match expression is not exhaustive. Missing variants: {}",
                                    uncovered.join(", ")
                                )));
                            }
                        }
                        break;
                    }
                }
            }
            }
        }

        Ok(())
    }
}