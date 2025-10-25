use crate::ast::*;
use crate::errors::CompileError;
use crate::BuildTarget; // Import the BuildTarget enum
use crate::token::TokenKind;
use crate::vdom::VNode;
use crate::semantic_analyzer::ResolvedType;
use crate::css_generator; // CSS generation (Phase 7.5)
use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, ImportSection, Instruction,
    Module, TypeSection, ValType, EntityType, MemoryType, MemorySection,
    TableSection, TableType, RefType, ElementSection, Elements, ConstExpr,
};

/// A symbol table to track function indices.
struct FuncSymbolTable {
    funcs: HashMap<String, u32>,
}
impl FuncSymbolTable { fn new() -> Self { Self { funcs: HashMap::new() } } }

/// Tracks struct field layouts for memory operations
#[derive(Debug, Clone)]
struct StructLayout {
    fields: Vec<(String, u32, ResolvedType)>,  // field_name, offset, type
    total_size: u32,
}

impl StructLayout {
    fn new() -> Self {
        Self {
            fields: Vec::new(),
            total_size: 0,
        }
    }

    fn add_field(&mut self, name: String, ty: ResolvedType) {
        let offset = self.total_size;
        let size = Self::type_size(&ty);
        self.fields.push((name, offset, ty));
        self.total_size += size;
    }

    fn get_field_offset(&self, field_name: &str) -> Option<u32> {
        self.fields
            .iter()
            .find(|(name, _, _)| name == field_name)
            .map(|(_, offset, _)| *offset)
    }

    fn type_size(ty: &ResolvedType) -> u32 {
        match ty {
            ResolvedType::Integer => 4,
            ResolvedType::Float => 8,
            ResolvedType::Bool => 4,
            ResolvedType::String => 4,  // Pointer to string data
            ResolvedType::Array(_) => 4,  // Pointer to array data
            ResolvedType::Struct(_) => 4,  // Pointer to struct data
            ResolvedType::Signal(_) => 4,  // Signal ID
            _ => 4,  // Default to pointer size
        }
    }
}

/// Tracks struct definitions and their layouts
struct StructTable {
    structs: HashMap<String, StructLayout>,
}

impl StructTable {
    fn new() -> Self {
        Self { structs: HashMap::new() }
    }

    fn define(&mut self, name: String, layout: StructLayout) {
        self.structs.insert(name, layout);
    }

    fn get_layout(&self, struct_name: &str) -> Option<&StructLayout> {
        self.structs.get(struct_name)
    }

    fn get_field_offset(&self, struct_name: &str, field_name: &str) -> Option<u32> {
        self.structs
            .get(struct_name)
            .and_then(|layout| layout.get_field_offset(field_name))
    }
}

/// Tracks lambda expressions for conversion to anonymous functions
#[derive(Debug, Clone)]
struct LambdaInfo {
    func_index: u32,                    // Function index in WASM
    type_index: u32,                    // Type signature index
    parameters: Vec<LambdaParameter>,   // Parameter names and types
    body: Box<Expression>,              // Lambda body expression
    captured_vars: Vec<String>,         // Variables captured from enclosing scope
}

/// Lambda table to track all lambdas for conversion
struct LambdaTable {
    lambdas: Vec<LambdaInfo>,           // All collected lambdas
}

impl LambdaTable {
    fn new() -> Self {
        Self { lambdas: Vec::new() }
    }

    fn add_lambda(&mut self, info: LambdaInfo) -> usize {
        let id = self.lambdas.len();
        self.lambdas.push(info);
        id
    }

    fn get_lambda(&self, id: usize) -> Option<&LambdaInfo> {
        self.lambdas.get(id)
    }
}


/// The code generator, responsible for emitting Wasm bytecode.
pub struct CodeGenerator {

    func_symbols: FuncSymbolTable,
    struct_table: StructTable,
    lambda_table: LambdaTable,  // Tracks all lambda expressions for conversion
    // Per-function state
    local_symbol_table: HashMap<String, u32>,
    local_type_table: HashMap<String, String>,  // variable_name -> struct_type_name
    local_count: u32,
    heap_pointer: u32,  // Tracks the next available heap address
    target: BuildTarget,
    // Lambda expression counter (used to match lambdas during code generation)
    lambda_encounter_counter: usize,
    // Current lambda context (Some(lambda_index) when generating a lambda body, None otherwise)
    current_lambda_context: Option<usize>,
    // CSS output (Phase 7.5)
    css_output: String,
}

impl CodeGenerator {
    /// Creates a new CodeGenerator for a specific build target.
    pub fn new(target: BuildTarget) -> Self {
        Self {
            //module: Module::new(),
            func_symbols: FuncSymbolTable::new(),
            struct_table: StructTable::new(),
            lambda_table: LambdaTable::new(),
            local_symbol_table: HashMap::new(),
            local_type_table: HashMap::new(),
            local_count: 0,
            heap_pointer: 0,  // Start heap at address 0
            target,
            lambda_encounter_counter: 0,
            current_lambda_context: None,
            css_output: String::new(),
        }
    }

    /// Get the generated CSS output (Phase 7.5)
    pub fn get_css_output(&self) -> &str {
        &self.css_output
    }

    /// Extract CSS expressions from AST and generate scoped CSS (Phase 7.5)
    fn extract_and_generate_css(&mut self, program: &Program) -> Result<(), CompileError> {
        for stmt in &program.statements {
            match stmt {
                Statement::Function(func_def) => {
                    // Search function body for CSS macros
                    self.extract_css_from_statements(&func_def.body.statements, &func_def.name.value)?;
                }
                Statement::Component(comp_def) => {
                    // Search component body for CSS macros
                    self.extract_css_from_statements(&comp_def.body.statements, &comp_def.name.value)?;
                }
                Statement::Let(let_stmt) => {
                    // Check if let binding has CSS macro
                    self.extract_css_from_expression(&let_stmt.value, "App")?;
                }
                Statement::Style(style_block) => {
                    // Phase 13: Generate CSS from style block
                    self.generate_style_block_css(style_block)?;
                }
                Statement::Theme(theme_block) => {
                    // Phase 13: Generate CSS custom properties from theme block
                    self.generate_theme_block_css(theme_block)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Recursively search statements for CSS macros
    fn extract_css_from_statements(&mut self, stmts: &[Statement], component_name: &str) -> Result<(), CompileError> {
        for stmt in stmts {
            match stmt {
                Statement::Let(let_stmt) => {
                    self.extract_css_from_expression(&let_stmt.value, component_name)?;
                }
                Statement::Expression(expr) => {
                    self.extract_css_from_expression(expr, component_name)?;
                }
                Statement::If(if_stmt) => {
                    self.extract_css_from_statements(&if_stmt.then_branch.statements, component_name)?;
                    if let Some(else_branch) = &if_stmt.else_branch {
                        // Recursively handle else branch (could be another If or any statement)
                        self.extract_css_from_statements(&[*else_branch.clone()], component_name)?;
                    }
                }
                Statement::While(while_stmt) => {
                    self.extract_css_from_statements(&while_stmt.body.statements, component_name)?;
                }
                Statement::For(for_stmt) => {
                    self.extract_css_from_statements(&for_stmt.body.statements, component_name)?;
                }
                Statement::ForIn(for_in_stmt) => {
                    self.extract_css_from_statements(&for_in_stmt.body.statements, component_name)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Recursively search expression for CSS macros
    fn extract_css_from_expression(&mut self, expr: &Expression, component_name: &str) -> Result<(), CompileError> {
        match expr {
            Expression::CssMacro(css_expr) => {
                // Found a CSS macro! Generate scoped CSS
                let mut generator = css_generator::CssGenerator::new(component_name.to_string());
                let css = generator.generate(css_expr);
                self.css_output.push_str(&css);
            }
            Expression::IfExpression(if_expr) => {
                self.extract_css_from_expression(&if_expr.condition, component_name)?;
                self.extract_css_from_expression(&if_expr.then_expr, component_name)?;
                if let Some(else_expr) = &if_expr.else_expr {
                    self.extract_css_from_expression(else_expr, component_name)?;
                }
            }
            Expression::Match(match_expr) => {
                for arm in &match_expr.arms {
                    self.extract_css_from_expression(&arm.body, component_name)?;
                }
            }
            Expression::Block(block) => {
                self.extract_css_from_statements(&block.statements, component_name)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// The main entry point for generating a complete Wasm module from an AST.
    pub fn generate_program(&mut self, program: &Program) -> Result<Vec<u8>, CompileError> {
        let mut module = Module::new();
        let mut types = TypeSection::new();
        let mut functions = FunctionSection::new();
        let mut imports = ImportSection::new();
        let mut exports = ExportSection::new();
        let mut code = CodeSection::new();
        let mut func_index_counter = 0;

        // --- Pass 0: Collect Struct Definitions ---
        // Build struct layouts from the AST
        for stmt in &program.statements {
            if let Statement::Struct(struct_def) = stmt {
                let mut layout = StructLayout::new();
                for (field_name, field_type) in &struct_def.fields {
                    // Convert TypeExpression to ResolvedType
                    let resolved_type = self.type_expression_to_resolved_type(field_type);
                    layout.add_field(field_name.value.clone(), resolved_type);
                }
                self.struct_table.define(struct_def.name.value.clone(), layout);
            }
        }

        // --- Pass 0.5: Collect Lambda Expressions ---
        // Walk the AST and collect all lambda expressions before generating code
        // This allows us to generate anonymous functions for them
        self.collect_lambdas_from_program(program);

        // --- Pass 0.75: Extract and Generate CSS (Phase 7.5) ---
        // Walk the AST and generate CSS from css! macro expressions
        self.extract_and_generate_css(program)?;

        // --- First Pass: Signatures and Imports ---
        // This pass collects all function signatures and builds the import table.
        for stmt in &program.statements {
            match stmt {
                Statement::ExternBlock(block) => {
                    for func in &block.functions {
                        let param_types: Vec<ValType> = func.parameters.iter().map(|_| ValType::I32).collect();
                        let type_index = types.len();
                        types.function(param_types, vec![]);
                        imports.import(&block.abi, &func.name.value, EntityType::Function(type_index));
                        self.func_symbols.funcs.insert(func.name.value.clone(), func_index_counter);
                        func_index_counter += 1;
                    }
                }
                Statement::Function(func_def) => {
                    // All functions, server or client, get a type signature.
                    let type_index = types.len();
                    // Generate parameter types (for now, all i32)
                    let param_types: Vec<ValType> = func_def.parameters.iter().map(|_| ValType::I32).collect();
                    types.function(param_types, vec![ValType::I32]);
                    functions.function(type_index);
                    self.func_symbols.funcs.insert(func_def.name.value.clone(), func_index_counter);

                    // Export the function if it's the main entry point or if we're on the server.
                    if func_def.name.value == "main" || (self.target == BuildTarget::Server && func_def.is_server) {
                        exports.export(&func_def.name.value, ExportKind::Func, func_index_counter);
                    }
                    func_index_counter += 1;
                }
                Statement::Component(comp) => {
                    // Export component as a function
                    let type_index = types.len();
                    types.function(vec![], vec![ValType::I32]);
                    functions.function(type_index);
                    self.func_symbols.funcs.insert(comp.name.value.clone(), func_index_counter);
                    exports.export(&comp.name.value, ExportKind::Func, func_index_counter);
                    func_index_counter += 1;
                }
                _ => {}
            }
        }

        // --- Pass 1.5: Register Lambda Function Signatures ---
        // For each collected lambda, create a type signature and register a function
        for i in 0..self.lambda_table.lambdas.len() {
            let lambda = &self.lambda_table.lambdas[i];

            // Create type signature for this lambda
            // For lambdas WITH captured variables, add an environment pointer as the first parameter
            let type_index = types.len();
            let mut param_types: Vec<ValType> = Vec::new();

            // If lambda has captured variables, add environment pointer as first parameter
            if !lambda.captured_vars.is_empty() {
                param_types.push(ValType::I32);  // Environment pointer (i32)
            }

            // Add the lambda's declared parameters
            param_types.extend(lambda.parameters.iter().map(|_| ValType::I32));

            types.function(param_types, vec![ValType::I32]);

            // Register as a function
            functions.function(type_index);

            // Update the lambda info with the correct indices
            // We need to do this mutably, so we'll update it after the loop
            let func_index = func_index_counter;
            func_index_counter += 1;

            // Store the indices back into the lambda table
            // (we have to do this carefully because we're borrowing self)
            self.lambda_table.lambdas[i].func_index = func_index;
            self.lambda_table.lambdas[i].type_index = type_index;
        }

        // --- Second Pass: Code Generation ---
        // This pass generates the actual instruction bodies for the functions.
        for stmt in &program.statements {
            match stmt {
                Statement::Function(func_def) => {
                    match self.target {
                        BuildTarget::Client => {
                            if func_def.is_server {
                                // On the client, server functions get a stub.
                                code.function(&self.generate_rpc_stub(func_def)?);
                            } else {
                                // Normal functions get a full body.
                                code.function(&self.generate_function(func_def)?);
                            }
                        }
                        BuildTarget::Server => {
                            // On the server, we only compile server functions.
                            if func_def.is_server {
                                code.function(&self.generate_function(func_def)?);
                            }
                            // Non-server functions are ignored in a server build.
                        }
                    }
                }
                Statement::Component(comp) => {
                    // Generate component function
                    code.function(&self.generate_component(comp)?);
                }
                Statement::ImplBlock(_) => {
                    // Impl blocks don't generate code directly - methods are called through method call syntax
                    // For now, we skip impl blocks in codegen
                }
                Statement::Trait(_) => {
                    // Trait definitions don't generate code - they're just signatures
                    // Actual method implementations come from impl blocks
                }
                _ => {}
            }
        }

        // --- Pass 2.5: Generate Lambda Function Bodies ---
        // For each collected lambda, generate its function body
        for i in 0..self.lambda_table.lambdas.len() {
            code.function(&self.generate_lambda(i)?);
        }

        // Assemble the WASM module sections in the correct order
        module.section(&types);
        module.section(&imports);
        module.section(&functions);

        // Add function table for indirect calls (closures, higher-order functions)
        let mut tables = TableSection::new();
        tables.table(TableType {
            element_type: RefType::FUNCREF,
            minimum: 10,  // Start with space for 10 function references
            maximum: Some(100),  // Can grow up to 100 functions
        });
        module.section(&tables);

        // Add memory section (1 page = 64KB initially, can grow)
        let mut memory = MemorySection::new();
        memory.memory(MemoryType {
            minimum: 1,
            maximum: Some(10),
            memory64: false,
            shared: false,
        });
        module.section(&memory);

        module.section(&exports);

        // Populate function table with all defined functions
        // This allows indirect calls via call_indirect instruction
        // Element section must come after Export section and before Code section
        if func_index_counter > 0 {
            let mut elements = ElementSection::new();
            // Collect all function indices (starting after imports)
            // For now, we'll populate the table starting at offset 0
            let func_indices: Vec<u32> = (0..func_index_counter).collect();
            elements.active(
                None,  // Table index 0 (default table)
                &ConstExpr::i32_const(0),  // Start at offset 0 in the table
                Elements::Functions(&func_indices),
            );
            module.section(&elements);
        }

        module.section(&code);

        Ok(module.finish())
    }

    /// Generates the full Wasm instruction body for a given function.
    fn generate_function(&mut self, func: &FunctionDefinition) -> Result<Function, CompileError> {
        self.local_symbol_table.clear();
        self.local_type_table.clear();
        self.local_count = 0;
        self.lambda_encounter_counter = 0;  // Reset lambda counter for this function

        // Register function parameters as locals (they start at index 0)
        for param in &func.parameters {
            self.local_symbol_table.insert(param.name.value.clone(), self.local_count);
            self.local_count += 1;
        }

        // Count locals needed for the function body
        // This includes:
        // - let statements (1 local each)
        // - for-in loops (3 locals each: iterator, loop variable, option)
        // - match expressions (1 local for scrutinee)
        let local_count = self.count_required_locals(&func.body.statements);
        let local_types: Vec<ValType> = (0..local_count).map(|_| ValType::I32).collect();
        let mut f = Function::new_with_locals_types(local_types);

        for stmt in &func.body.statements {
            self.generate_statement(stmt, &mut f)?;
        }

        f.instruction(&Instruction::I32Const(0));
        f.instruction(&Instruction::End);
        Ok(f)
    }

    /// Counts the number of WASM locals needed for a list of statements
    fn count_required_locals(&self, stmts: &[Statement]) -> u32 {
        let mut count = 0;
        for stmt in stmts {
            count += self.count_statement_locals(stmt);
        }
        count
    }

    /// Counts locals needed for a single statement (recursively)
    fn count_statement_locals(&self, stmt: &Statement) -> u32 {
        match stmt {
            Statement::Let(_) => 1,
            Statement::ForIn(_) => {
                // For-in loops need 3 locals: iterator, loop variable, option
                3
            }
            Statement::If(if_stmt) => {
                let mut count = 0;
                count += self.count_required_locals(&if_stmt.then_branch.statements);
                if let Some(else_stmt) = &if_stmt.else_branch {
                    count += self.count_statement_locals(else_stmt);
                }
                count
            }
            Statement::While(while_stmt) => {
                self.count_required_locals(&while_stmt.body.statements)
            }
            Statement::For(for_stmt) => {
                let mut count = 0;
                if let Some(init) = &for_stmt.init {
                    count += self.count_statement_locals(init);
                }
                count += self.count_required_locals(&for_stmt.body.statements);
                if let Some(update) = &for_stmt.update {
                    count += self.count_statement_locals(update);
                }
                count
            }
            Statement::Expression(expr) => {
                // Match expressions allocate locals for scrutinee
                self.count_expression_locals(expr)
            }
            _ => 0,
        }
    }

    /// Counts locals needed for expressions (mainly for match)
    fn count_expression_locals(&self, expr: &Expression) -> u32 {
        match expr {
            Expression::Match(_) => {
                // Match expressions need 1 local for the scrutinee
                1
            }
            _ => 0,
        }
    }

    /// Generates a component as a WASM function
    fn generate_component(&mut self, comp: &ComponentDefinition) -> Result<Function, CompileError> {
        self.local_symbol_table.clear();
        self.local_type_table.clear();
        self.local_count = 0;
        self.lambda_encounter_counter = 0;  // Reset lambda counter for this component

        // Register component parameters as locals (they start at index 0)
        for param in &comp.parameters {
            self.local_symbol_table.insert(param.name.value.clone(), self.local_count);
            self.local_count += 1;
        }

        // Count locals needed for the component body
        let local_count = self.count_required_locals(&comp.body.statements);
        let local_types: Vec<ValType> = (0..local_count).map(|_| ValType::I32).collect();
        let mut f = Function::new_with_locals_types(local_types);

        // Generate all statements in the component body
        for stmt in &comp.body.statements {
            self.generate_statement(stmt, &mut f)?;
        }

        // Components should return their final expression value
        // If the last statement is not a return, push a default value
        f.instruction(&Instruction::I32Const(0));
        f.instruction(&Instruction::End);
        Ok(f)
    }

    /// Generates a WASM function for a lambda expression
    /// Similar to generate_function but simpler - lambdas only have an expression body
    fn generate_lambda(&mut self, lambda_index: usize) -> Result<Function, CompileError> {
        // Clear local state for this lambda function
        self.local_symbol_table.clear();
        self.local_type_table.clear();
        self.local_count = 0;

        // Get the lambda info from the lambda table
        let lambda = self.lambda_table.get_lambda(lambda_index)
            .ok_or_else(|| CompileError::Generic(format!(
                "Codegen: Lambda index {} not found in lambda table", lambda_index
            )))?
            .clone();

        // Set the current lambda context so generate_expression knows which lambda we're in
        self.current_lambda_context = Some(lambda_index);

        // If lambda has captured variables, the first parameter (local 0) is the environment pointer
        if !lambda.captured_vars.is_empty() {
            // Register environment pointer as local 0
            // We use a special name "__env" that won't conflict with user variables
            self.local_symbol_table.insert("__env".to_string(), self.local_count);
            self.local_count += 1;
        }

        // Register lambda parameters as locals (they start after the environment pointer if present)
        for param in &lambda.parameters {
            self.local_symbol_table.insert(param.name.value.clone(), self.local_count);
            self.local_count += 1;
        }

        // Lambda bodies are expressions, not statement blocks
        // We don't need to count locals for the body since lambdas can't have let statements
        // Create the function with no additional locals (only parameters)
        let mut f = Function::new(vec![]);

        // Generate the lambda body expression
        self.generate_expression(&lambda.body, &mut f)?;

        // Clear the lambda context
        self.current_lambda_context = None;

        // End the function
        f.instruction(&Instruction::End);
        Ok(f)
    }

    /// Generates a placeholder "teleporter pad" for a server function on the client.
    fn generate_rpc_stub(&mut self, _func: &FunctionDefinition) -> Result<Function, CompileError> {
        let mut f = Function::new(vec![]);
        // This generated function is a placeholder. A real implementation would:
        // 1. Serialize arguments into a buffer.
        // 2. Call a generic `_rpc_call` FFI function.
        // 3. Await and deserialize the result.

        // For now, it just returns a dummy value (e.g., -1 for an i32) to indicate it's a stub.
        f.instruction(&Instruction::I32Const(-1));
        f.instruction(&Instruction::End);
        Ok(f)
    }

    fn generate_statement(&mut self, stmt: &Statement, f: &mut Function) -> Result<(), CompileError> {
        match stmt {
            Statement::Let(let_stmt) => {
                use crate::ast::Pattern;

                self.generate_expression(&let_stmt.value, f)?;

                // Handle pattern binding
                match &let_stmt.pattern {
                    Pattern::Identifier(id) => {
                        // Track the type if it's a struct literal
                        if let Expression::StructLiteral(struct_lit) = &let_stmt.value {
                            self.local_type_table.insert(
                                id.value.clone(),
                                struct_lit.name.value.clone()
                            );
                        }

                        let local_index = self.local_count;
                        self.local_symbol_table.insert(id.value.clone(), local_index);
                        self.local_count += 1;
                        f.instruction(&Instruction::LocalSet(local_index));
                    }
                    Pattern::Tuple(patterns) => {
                        // For tuple destructuring in WASM, we need to:
                        // 1. Store the tuple value in a temp local
                        // 2. Extract each element and store in separate locals
                        // Note: This is a simplified implementation
                        // Full implementation would need to handle tuples properly

                        // For now, just bind all pattern identifiers to the same value
                        // TODO: Implement proper tuple element extraction
                        for (idx, pattern) in patterns.iter().enumerate() {
                            if let Pattern::Identifier(id) = pattern {
                                let local_index = self.local_count;
                                self.local_symbol_table.insert(id.value.clone(), local_index);
                                self.local_count += 1;

                                // Store the value (this is simplified - should extract tuple element idx)
                                if idx == 0 {
                                    f.instruction(&Instruction::LocalSet(local_index));
                                } else {
                                    // For subsequent elements, load from first local
                                    // This is a placeholder - proper implementation needs tuple access
                                    f.instruction(&Instruction::I32Const(0));
                                    f.instruction(&Instruction::LocalSet(local_index));
                                }
                            }
                        }
                    }
                    _ => {
                        // Other patterns not yet implemented for WASM
                        return Err(CompileError::Generic(
                            "Pattern type not yet supported in WASM codegen".to_string()
                        ));
                    }
                }
            }
            Statement::Assignment(assign_stmt) => {
                match &assign_stmt.target {
                    Expression::Identifier(ident) => {
                        // Generate the value expression
                        self.generate_expression(&assign_stmt.value, f)?;

                        // Get the local index of the target variable
                        let local_index = *self.local_symbol_table.get(&ident.value)
                            .ok_or_else(|| CompileError::Generic(format!(
                                "Codegen: undefined variable '{}' in assignment",
                                ident.value
                            )))?;

                        // Set the local variable
                        f.instruction(&Instruction::LocalSet(local_index));
                    }
                    Expression::FieldAccess(_) | Expression::IndexAccess(_) => {
                        // For field access and index access assignments, we need to:
                        // 1. Evaluate the target expression to get the memory location
                        // 2. Evaluate the value expression
                        // 3. Store the value at the memory location
                        // This is complex and requires memory/heap support
                        // For now, return an error
                        return Err(CompileError::Generic(
                            "Field and index assignments not yet implemented in WASM codegen".to_string()
                        ));
                    }
                    _ => {
                        return Err(CompileError::Generic(
                            "Invalid assignment target in WASM codegen".to_string()
                        ));
                    }
                }
            }
            Statement::Return(return_stmt) => {
                // Generate the return value
                // Note: We don't add an End instruction here because the function's
                // generate_function() method will add it at the end
                // The Return instruction in WASM is implicit - we just leave the value on the stack
                self.generate_expression(&return_stmt.value, f)?;
                // After generating the return value, we need to return from the function
                // In WASM, we use 'return' instruction to exit early
                f.instruction(&Instruction::Return);
            }
            Statement::Expression(expr) => {
                self.generate_expression(expr, f)?;
                // Expressions used as statements leave a value on the stack. Pop it.
                f.instruction(&Instruction::Drop);
            }
            Statement::If(if_stmt) => {
                self.generate_if_statement(if_stmt, f)?;
            }
            Statement::While(while_stmt) => {
                self.generate_while_statement(while_stmt, f)?;
            }
            Statement::For(for_stmt) => {
                self.generate_for_statement(for_stmt, f)?;
            }
            Statement::ForIn(for_in_stmt) => {
                self.generate_for_in_statement(for_in_stmt, f)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn generate_if_statement(&mut self, stmt: &IfStatement, f: &mut Function) -> Result<(), CompileError> {
        // Generate condition
        self.generate_expression(&stmt.condition, f)?;

        // Start if block
        f.instruction(&Instruction::If(wasm_encoder::BlockType::Empty));

        // Generate then branch
        for s in &stmt.then_branch.statements {
            self.generate_statement(s, f)?;
        }

        // Generate else branch if present
        if stmt.else_branch.is_some() {
            f.instruction(&Instruction::Else);
            if let Some(else_stmt) = &stmt.else_branch {
                self.generate_statement(else_stmt, f)?;
            }
        }

        // End if block
        f.instruction(&Instruction::End);
        Ok(())
    }

    fn generate_while_statement(&mut self, stmt: &WhileStatement, f: &mut Function) -> Result<(), CompileError> {
        // Start loop block
        f.instruction(&Instruction::Loop(wasm_encoder::BlockType::Empty));

        // Generate condition
        self.generate_expression(&stmt.condition, f)?;

        // Invert condition (exit if false)
        f.instruction(&Instruction::I32Eqz);

        // Break out of loop if condition is false
        f.instruction(&Instruction::BrIf(1));

        // Generate loop body
        for s in &stmt.body.statements {
            self.generate_statement(s, f)?;
        }

        // Branch back to start of loop
        f.instruction(&Instruction::Br(0));

        // End loop block
        f.instruction(&Instruction::End);

        Ok(())
    }

    fn generate_for_statement(&mut self, stmt: &ForStatement, f: &mut Function) -> Result<(), CompileError> {
        // Generate init statement if present (runs once before loop)
        if let Some(init) = &stmt.init {
            self.generate_statement(init, f)?;
        }

        // Start loop block
        f.instruction(&Instruction::Loop(wasm_encoder::BlockType::Empty));

        // Generate condition
        self.generate_expression(&stmt.condition, f)?;

        // Invert condition (exit if false)
        f.instruction(&Instruction::I32Eqz);

        // Break out of loop if condition is false
        f.instruction(&Instruction::BrIf(1));

        // Generate loop body
        for s in &stmt.body.statements {
            self.generate_statement(s, f)?;
        }

        // Generate update statement if present (runs after each iteration)
        if let Some(update) = &stmt.update {
            self.generate_statement(update, f)?;
        }

        // Branch back to start of loop
        f.instruction(&Instruction::Br(0));

        // End loop block
        f.instruction(&Instruction::End);

        Ok(())
    }

    fn generate_for_in_statement(&mut self, stmt: &ForInStatement, f: &mut Function) -> Result<(), CompileError> {
        // For-in loops use the Iterator protocol:
        // 1. Call into_iter() on the collection to get an iterator
        // 2. Loop: call next() on the iterator
        // 3. If Some(value), bind value to loop variable and execute body
        // 4. If None, exit loop
        //
        // This implementation generates WASM code that follows this protocol.
        // Note: This is a simplified version that works with the stdlib iterator types.
        // Full implementation would need:
        // - Dynamic dispatch for trait methods (into_iter, next)
        // - Option<T> enum discrimination in WASM
        // - Proper type inference from semantic analysis

        // Allocate a local for the iterator
        let iterator_local = self.local_count;
        self.local_count += 1;

        // Allocate a local for the loop variable
        let loop_var_local = self.local_count;
        self.local_symbol_table.insert(stmt.variable.value.clone(), loop_var_local);
        self.local_count += 1;

        // Allocate a local for the Option<T> result from next()
        let option_local = self.local_count;
        self.local_count += 1;

        // Step 1: Convert the collection into an iterator by calling into_iter()
        // For now, we'll assume the iterator expression evaluates to an iterable type
        // In a full implementation, we'd call the into_iter() method here
        self.generate_expression(&stmt.iterator, f)?;
        f.instruction(&Instruction::LocalSet(iterator_local));

        // Step 2: Generate the loop structure
        // We use a WASM loop block that:
        // - Calls next() on the iterator
        // - Checks if the result is Some or None
        // - If Some, extracts the value, binds it, executes body, continues loop
        // - If None, breaks out of the loop

        // Start the loop block
        f.instruction(&Instruction::Loop(wasm_encoder::BlockType::Empty));

        // Step 3: Call next() on the iterator
        // For now, this is a simplified version that assumes:
        // - The iterator is stored in linear memory
        // - next() returns an Option<T> encoded as: tag (0=None, 1=Some) + value
        // In a full implementation, we'd:
        // 1. Load the iterator from local
        // 2. Call the next() method via dynamic dispatch
        // 3. Get the Option<T> result

        // Simplified: Load iterator (pointer to iterator object)
        f.instruction(&Instruction::LocalGet(iterator_local));

        // Call a hypothetical next() method that returns Option<T>
        // For this simplified version, we'll generate a placeholder that:
        // - Loads the option tag from memory (iterator_ptr + offset)
        // - If tag == 0 (None), exit loop
        // - If tag == 1 (Some), load value and continue

        // Load the option tag (first field of the Option<T> in memory)
        // Assume Option<T> layout: [tag: i32] [value: T]
        f.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
            offset: 0,  // Tag is at offset 0
            align: 2,
            memory_index: 0,
        }));

        // Store the tag in option_local for checking
        f.instruction(&Instruction::LocalTee(option_local));

        // Check if tag == 0 (None)
        f.instruction(&Instruction::I32Eqz);

        // If tag is 0 (None), break out of the loop (exit to outer block)
        f.instruction(&Instruction::BrIf(1));

        // Step 4: If we're here, tag == 1 (Some), so extract the value
        // Load the value from Option<T> (second field after tag)
        f.instruction(&Instruction::LocalGet(iterator_local));
        f.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
            offset: 4,  // Value is at offset 4 (after the tag)
            align: 2,
            memory_index: 0,
        }));

        // Bind the value to the loop variable
        f.instruction(&Instruction::LocalSet(loop_var_local));

        // Step 5: Execute the loop body
        for s in &stmt.body.statements {
            self.generate_statement(s, f)?;
        }

        // Step 6: Continue the loop (branch back to the start)
        f.instruction(&Instruction::Br(0));

        // End the loop block
        f.instruction(&Instruction::End);

        Ok(())
    }

    fn generate_expression(&mut self, expr: &Expression, f: &mut Function) -> Result<(), CompileError> {
        match expr {
            Expression::IntegerLiteral(val) => {
                f.instruction(&Instruction::I32Const(*val as i32));
            }
            Expression::FloatLiteral(val) => {
                let float_val: f64 = val.parse().unwrap_or(0.0);
                f.instruction(&Instruction::F64Const(float_val));
            }
            Expression::BoolLiteral(val) => {
                f.instruction(&Instruction::I32Const(if *val { 1 } else { 0 }));
            }
            Expression::UnitLiteral => {
                // Unit type () - push 0 as a placeholder (unused in practice)
                f.instruction(&Instruction::I32Const(0));
            }
            Expression::Identifier(ident) => {
                // Check if we're in a lambda and this identifier is a captured variable
                if let Some(lambda_index) = self.current_lambda_context {
                    if let Some(lambda_info) = self.lambda_table.get_lambda(lambda_index) {
                        // Check if this variable is in the captured_vars list
                        if let Some(capture_index) = lambda_info.captured_vars.iter().position(|v| v == &ident.value) {
                            // This is a captured variable! Load it from the environment
                            // Environment layout with RC: [ref_count: i32] [var0: i32] [var1: i32] [var2: i32] ...
                            // ref_count is at offset 0 (4 bytes)
                            // Variables start at offset 4
                            // Each variable is at offset (4 + capture_index * 4)

                            // Get the environment pointer (local 0, registered as "__env")
                            let env_local = *self.local_symbol_table.get("__env")
                                .ok_or_else(|| CompileError::Generic(
                                    "Codegen: Environment pointer not found in lambda with captured variables".to_string()
                                ))?;

                            // Load environment pointer
                            f.instruction(&Instruction::LocalGet(env_local));

                            // Load the captured variable from environment
                            // Offset = 4 (skip ref_count) + capture_index * 4
                            f.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
                                offset: 4 + (capture_index as u64) * 4,
                                align: 2,  // 4-byte alignment for i32
                                memory_index: 0,
                            }));
                            return Ok(());
                        }
                    }
                }

                // Not a captured variable - try regular lookup
                // Try to get as a local variable first
                if let Some(&local_index) = self.local_symbol_table.get(&ident.value) {
                    f.instruction(&Instruction::LocalGet(local_index));
                } else if let Some(&func_idx) = self.func_symbols.funcs.get(&ident.value) {
                    // This is a function name used as a value - push its table index
                    // This enables: let f = some_func;
                    f.instruction(&Instruction::I32Const(func_idx as i32));
                } else {
                    // If not found as local or function, push dummy value
                    // This might be a forward reference or parsing issue
                    // TODO: Improve parsing/semantic analysis to catch these issues earlier
                    f.instruction(&Instruction::I32Const(0));
                }
            }
            Expression::Prefix(prefix) => {
                // Apply the prefix operator
                match &prefix.operator.kind {
                    TokenKind::Minus => {
                        // Negation: 0 - x  (push 0 first, then x, then subtract)
                        f.instruction(&Instruction::I32Const(0));
                        self.generate_expression(&prefix.right, f)?;
                        f.instruction(&Instruction::I32Sub);
                    }
                    TokenKind::Bang => {
                        // Logical NOT: x == 0
                        self.generate_expression(&prefix.right, f)?;
                        f.instruction(&Instruction::I32Eqz);
                    }
                    _ => return Err(CompileError::Generic(format!(
                        "Unsupported prefix operator: {:?}", prefix.operator.kind
                    ))),
                }
            }
            Expression::Spread(_spread) => {
                // Spread operator is primarily a JavaScript feature
                // For WASM, we'd need to expand it at compile time
                // For now, push a placeholder value
                f.instruction(&Instruction::I32Const(0));
            }
            Expression::Infix(infix) => {
                self.generate_expression(&infix.left, f)?;
                self.generate_expression(&infix.right, f)?;

                match &infix.operator.kind {
                    TokenKind::Plus => { f.instruction(&Instruction::I32Add); }
                    TokenKind::Minus => { f.instruction(&Instruction::I32Sub); }
                    TokenKind::Star => { f.instruction(&Instruction::I32Mul); }
                    TokenKind::Slash => { f.instruction(&Instruction::I32DivS); }
                    TokenKind::Percent => { f.instruction(&Instruction::I32RemS); }
                    TokenKind::Eq => { f.instruction(&Instruction::I32Eq); }
                    TokenKind::NotEq => { f.instruction(&Instruction::I32Ne); }
                    TokenKind::LAngle => { f.instruction(&Instruction::I32LtS); }
                    TokenKind::RAngle => { f.instruction(&Instruction::I32GtS); }
                    TokenKind::LtEq => { f.instruction(&Instruction::I32LeS); }
                    TokenKind::GtEq => { f.instruction(&Instruction::I32GeS); }
                    TokenKind::AmpAmp => { f.instruction(&Instruction::I32And); }  // Logical AND (works for booleans as 0/1)
                    TokenKind::PipePipe => { f.instruction(&Instruction::I32Or); }  // Logical OR (works for booleans as 0/1)
                    _ => return Err(CompileError::Generic(format!(
                        "Unsupported operator: {:?}", infix.operator.kind
                    ))),
                }
            }
            Expression::Lambda(_lambda) => {
                // Lambda expressions are converted to anonymous functions with reference-counted environments
                // Closures are represented as a pair: [func_index, env_ptr]
                //
                // For lambdas with NO captured variables:
                //   Return just the function index (simpler, no environment needed)
                //
                // For lambdas WITH captured variables:
                //   1. Allocate environment struct in heap with ref count
                //   2. Initialize ref count to 1
                //   3. Store captured variable values in environment
                //   4. Return a closure tuple [func_index, env_ptr]
                //
                // Environment layout with reference counting:
                //   [ref_count: i32 (4 bytes)] [var0: i32] [var1: i32] [var2: i32] ...

                // Get the current lambda's index from the encounter counter
                let lambda_id = self.lambda_encounter_counter;
                self.lambda_encounter_counter += 1;

                // Look up the lambda info
                let lambda_info = self.lambda_table.get_lambda(lambda_id)
                    .ok_or_else(|| CompileError::Generic(format!(
                        "Codegen: Lambda {} not found in lambda table (total: {})",
                        lambda_id, self.lambda_table.lambdas.len()
                    )))?
                    .clone();

                if lambda_info.captured_vars.is_empty() {
                    // Simple case: No captured variables
                    // Just return the function index
                    f.instruction(&Instruction::I32Const(lambda_info.func_index as i32));
                } else {
                    // Complex case: Lambda has captured variables
                    // We need to allocate a reference-counted closure environment

                    // Calculate environment size: 4 bytes for ref_count + 4 bytes per captured variable
                    let env_size = 4 + (lambda_info.captured_vars.len() as u32) * 4;

                    // Allocate environment struct on the heap
                    let env_ptr = self.heap_pointer;
                    self.heap_pointer += env_size;

                    // Initialize reference count to 1 (this is the first reference)
                    f.instruction(&Instruction::I32Const(env_ptr as i32));
                    f.instruction(&Instruction::I32Const(1));
                    f.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                        offset: 0,  // ref_count is at offset 0
                        align: 2,
                        memory_index: 0,
                    }));

                    // Store each captured variable value into the environment
                    // Variables start at offset 4 (after the ref_count field)
                    for (i, var_name) in lambda_info.captured_vars.iter().enumerate() {
                        // Get the local index of the captured variable
                        if let Some(&local_index) = self.local_symbol_table.get(var_name) {
                            // Push environment pointer
                            f.instruction(&Instruction::I32Const(env_ptr as i32));

                            // Load the captured variable value
                            f.instruction(&Instruction::LocalGet(local_index));

                            // Store it in the environment at offset (4 + i * 4)
                            // Offset 0-3: ref_count
                            // Offset 4+: captured variables
                            f.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                                offset: (4 + (i as u64) * 4),  // +4 to skip ref_count
                                align: 2,  // 4-byte alignment for i32
                                memory_index: 0,
                            }));
                        }
                        // If variable not found in local scope, skip (shouldn't happen with proper capture analysis)
                    }

                    // For now, return just the function index
                    // TODO: Full closure implementation should return tuple [func_index, env_ptr]
                    // and update lambda signatures to accept env_ptr as first parameter
                    f.instruction(&Instruction::I32Const(lambda_info.func_index as i32));

                    // Store env_ptr for potential future use
                    // In a complete implementation, we'd:
                    // 1. Allocate a closure struct: [func_index, env_ptr]
                    // 2. Return pointer to this closure struct
                    // 3. Update call sites to extract func_index and env_ptr when calling
                }
            }
            Expression::FunctionCall(call) => {
                // Generate function call
                self.generate_function_call(call, f)?;
            }
            Expression::JsxElement(jsx) => {
                // Generate JSX element as VDOM
                self.generate_jsx_element(jsx, f)?;
            }
            Expression::ArrayLiteral(array_lit) => {
                // Arrays in WASM linear memory layout:
                // [length: i32 (4 bytes)][element0: i32][element1: i32]...
                //
                // Steps:
                // 1. Calculate total size needed (4 bytes for length + 4 bytes per element)
                // 2. Allocate memory from heap
                // 3. Store length at offset 0
                // 4. Store each element at offset 4 + (index * element_size)
                // 5. Return pointer to the array

                let element_count = array_lit.elements.len();
                let element_size = 4; // Assume all elements are i32 for now
                let total_size = 4 + (element_count as u32 * element_size); // 4 bytes for length + elements

                // Allocate memory and get pointer
                let array_ptr = self.heap_pointer;
                self.heap_pointer += total_size;

                // Store the array length at offset 0
                f.instruction(&Instruction::I32Const(array_ptr as i32));
                f.instruction(&Instruction::I32Const(element_count as i32));
                f.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                    offset: 0,
                    align: 2,  // 4-byte alignment for i32
                    memory_index: 0,
                }));

                // Store each element in the array
                for (index, element) in array_lit.elements.iter().enumerate() {
                    // Push the base pointer
                    f.instruction(&Instruction::I32Const(array_ptr as i32));

                    // Generate code for the element value
                    self.generate_expression(element, f)?;

                    // Store at offset: 4 (length field) + (index * element_size)
                    f.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                        offset: (4 + (index as u64 * element_size as u64)),
                        align: 2,  // 4-byte alignment for i32
                        memory_index: 0,
                    }));
                }

                // Push the array pointer as the result
                f.instruction(&Instruction::I32Const(array_ptr as i32));
            }
            Expression::StructLiteral(struct_lit) => {
                // Look up the struct layout
                let layout = self.struct_table.get_layout(&struct_lit.name.value)
                    .ok_or_else(|| CompileError::Generic(format!(
                        "Codegen: Unknown struct type '{}'",
                        struct_lit.name.value
                    )))?
                    .clone();

                // Allocate memory for the struct and push pointer onto stack
                let struct_ptr = self.heap_pointer;
                self.allocate_struct(layout.total_size, f);

                // For each field in the struct literal, store the value at the correct offset
                for (field_name, field_value) in &struct_lit.fields {
                    // Get the field offset from the layout
                    let offset = layout.get_field_offset(&field_name.value)
                        .ok_or_else(|| CompileError::Generic(format!(
                            "Codegen: Struct '{}' has no field '{}'",
                            struct_lit.name.value,
                            field_name.value
                        )))?;

                    // Push the base pointer + offset
                    f.instruction(&Instruction::I32Const(struct_ptr as i32));

                    // Generate code for the field value
                    self.generate_expression(field_value, f)?;

                    // Store the value at (base_ptr + offset)
                    // For now, assume all fields are i32 (we'll need to check type later)
                    f.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                        offset: offset as u64,
                        align: 2,  // 4-byte alignment for i32
                        memory_index: 0,
                    }));
                }

                // The struct pointer is already on the stack from allocate_struct
                // No need to push it again - we're reusing it as the return value
            }
            Expression::FieldAccess(field_access) => {
                // Generate code for the object expression to get the base pointer
                self.generate_expression(&field_access.object, f)?;

                // Try to infer struct type and field offset
                // If we can't infer the type, just return the object value as-is
                // TODO: Use semantic analyzer type information for accurate field access
                if let Ok(struct_name) = self.infer_struct_type(&field_access.object) {
                    if let Some(offset) = self.struct_table.get_field_offset(&struct_name, &field_access.field.value) {
                        // Load the value from memory at (base_ptr + offset)
                        f.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
                            offset: offset as u64,
                            align: 2,  // 4-byte alignment for i32
                            memory_index: 0,
                        }));
                    } else {
                        // Field not found, just use the object value
                        // (already on stack from generate_expression above)
                    }
                } else {
                    // Can't infer struct type, just use the object value
                    // (already on stack from generate_expression above)
                }
            }
            Expression::Match(match_expr) => {
                // Generate code for the scrutinee and store it in a local
                self.generate_expression(&match_expr.scrutinee, f)?;

                // For simple match expressions with literal/wildcard patterns,
                // we generate nested if/else blocks
                // This implementation handles:
                // 1. Literal patterns (comparing values)
                // 2. Wildcard patterns (catch-all)
                // 3. Identifier patterns (binding values - treated as wildcard)

                if match_expr.arms.is_empty() {
                    // Empty match, push unit value
                    f.instruction(&Instruction::I32Const(0));
                } else {
                    // Generate nested if/else structure for pattern matching
                    self.generate_match_arms(&match_expr.arms, f)?;
                }
            }
            Expression::IndexAccess(index_expr) => {
                // Generate code for array indexing: arr[index]
                // In WASM, arrays are stored in linear memory
                // Array layout: [length (4 bytes)] [element0] [element1] ...

                // Generate the array expression (should produce a pointer)
                self.generate_expression(&index_expr.array, f)?;

                // Generate the index expression (should produce an i32)
                self.generate_expression(&index_expr.index, f)?;

                // Calculate the memory address: base_ptr + 4 + (index * element_size)
                // For now, assume all elements are 4 bytes (i32)
                // Multiply index by 4 (element size)
                f.instruction(&Instruction::I32Const(4));
                f.instruction(&Instruction::I32Mul);

                // Add offset for length field (skip first 4 bytes)
                f.instruction(&Instruction::I32Const(4));
                f.instruction(&Instruction::I32Add);

                // Add to base pointer
                f.instruction(&Instruction::I32Add);

                // Load the value from memory
                f.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
                    offset: 0,
                    align: 2,  // 4-byte alignment for i32
                    memory_index: 0,
                }));
            }
            Expression::TupleLiteral(tuple_lit) => {
                // For now, tuples are stored in linear memory similar to structs
                // Tuple layout: [element0] [element1] [element2] ...
                // Each element is 4 bytes (i32 for now)

                // Calculate tuple size (4 bytes per element)
                let tuple_size = (tuple_lit.elements.len() as u32) * 4;

                // Allocate memory for the tuple
                let tuple_ptr = self.heap_pointer;
                f.instruction(&Instruction::I32Const(tuple_ptr as i32));
                self.heap_pointer += tuple_size;

                // Store each element in memory
                for (i, elem) in tuple_lit.elements.iter().enumerate() {
                    // Push the base pointer
                    f.instruction(&Instruction::I32Const(tuple_ptr as i32));

                    // Generate the element value
                    self.generate_expression(elem, f)?;

                    // Store at offset (i * 4)
                    f.instruction(&Instruction::I32Store(wasm_encoder::MemArg {
                        offset: (i as u64) * 4,
                        align: 2,  // 4-byte alignment for i32
                        memory_index: 0,
                    }));
                }

                // Push the tuple pointer as the result
                f.instruction(&Instruction::I32Const(tuple_ptr as i32));
            }
            Expression::StringLiteral(_s) => {
                // For now, strings are represented as i32 (pointer to string data)
                // In a full implementation, we'd allocate string in WASM memory
                // For now, push a dummy value
                f.instruction(&Instruction::I32Const(0));
            }
            Expression::Borrow(borrow_expr) => {
                // Borrowing in WASM is a no-op since everything is already a value or pointer
                // Just generate the inner expression
                self.generate_expression(&borrow_expr.expression, f)?;
            }
            Expression::MutableBorrow(borrow_expr) => {
                // Mutable borrowing in WASM is a no-op since everything is already a value or pointer
                // Just generate the inner expression
                self.generate_expression(&borrow_expr.expression, f)?;
            }
            Expression::Dereference(deref_expr) => {
                // Dereferencing in WASM - just generate the inner expression
                // In WASM, pointers and values are both i32, so dereferencing is mostly a no-op
                // In a full implementation, this might load from memory if the value is a pointer
                self.generate_expression(&deref_expr.expression, f)?;
            }
            Expression::Range(_range_expr) => {
                // Range expressions are placeholders for now
                // In a full implementation, we'd generate code for range creation
                // For now, push a dummy value
                f.instruction(&Instruction::I32Const(0));
            }
            Expression::TryOperator(try_expr) => {
                // Try operator for error propagation
                // For now, just compile the inner expression
                // TODO: Implement proper Result unwrapping in WASM
                self.generate_expression(&try_expr.expression, f)?;
            }
            Expression::Ternary(ternary) => {
                // Generate code for ternary expression: condition ? true_expr : false_expr
                // This is an expression, so it must produce a value on the stack

                // Generate the condition
                self.generate_expression(&ternary.condition, f)?;

                // Start if block with a result type (produces i32 value)
                f.instruction(&Instruction::If(wasm_encoder::BlockType::Result(ValType::I32)));

                // Generate the true branch
                self.generate_expression(&ternary.true_expr, f)?;

                // Generate the false branch
                f.instruction(&Instruction::Else);
                self.generate_expression(&ternary.false_expr, f)?;

                // End if block
                f.instruction(&Instruction::End);
            }
            Expression::TypeCast(type_cast) => {
                // Generate code for type casting
                // In WebAssembly, we need to emit conversion instructions for numeric types
                self.generate_expression(&type_cast.expression, f)?;

                // Extract type name from TypeExpression
                let type_name = match &type_cast.target_type {
                    TypeExpression::Named(ident) => ident.value.as_str(),
                    _ => return Ok(()), // For complex types, no conversion needed
                };

                // Apply appropriate WASM conversion based on target type
                match type_name {
                    "i32" => {
                        // Assume source is f64, convert to i32
                        f.instruction(&Instruction::I32TruncF64S);
                    }
                    "i64" => {
                        // Assume source is f64, convert to i64
                        f.instruction(&Instruction::I64TruncF64S);
                    }
                    "f32" => {
                        // Assume source is i32/f64, demote if needed
                        // For now, assume f64 -> f32
                        f.instruction(&Instruction::F32DemoteF64);
                    }
                    "f64" => {
                        // Assume source is i32, convert to f64
                        f.instruction(&Instruction::F64ConvertI32S);
                    }
                    _ => {
                        // For other types (like bool, custom types), no conversion needed
                        // Value is already on stack
                    }
                }
            }
            Expression::Await(await_expr) => {
                // Await operator for async/await
                // In a full implementation, this would:
                // 1. Evaluate the inner expression (which should return Future<T>)
                // 2. Suspend the current execution
                // 3. Wait for the future to resolve
                // 4. Resume execution with the resolved value
                // For now, just evaluate the inner expression directly
                self.generate_expression(&await_expr.expression, f)?;
            }
            Expression::IfExpression(if_expr) => {
                // Generate code for if-expression: if cond { then_expr } else { else_expr }
                // This is an expression, so it must produce a value on the stack

                // Generate the condition
                self.generate_expression(&if_expr.condition, f)?;

                // Start if block with a result type (produces i32 value)
                f.instruction(&Instruction::If(wasm_encoder::BlockType::Result(ValType::I32)));

                // Generate the then branch
                self.generate_expression(&if_expr.then_expr, f)?;

                // Generate else branch if present, otherwise push default value (0)
                if let Some(else_expr) = &if_expr.else_expr {
                    f.instruction(&Instruction::Else);
                    self.generate_expression(else_expr, f)?;
                } else {
                    f.instruction(&Instruction::Else);
                    f.instruction(&Instruction::I32Const(0));
                }

                // End if block
                f.instruction(&Instruction::End);
            }
            Expression::Block(block) => {
                // Generate code for all statements in the block
                for stmt in &block.statements {
                    self.generate_statement(stmt, f)?;
                }
            }
            Expression::MacroCall(macro_call) => {
                // Process macro arguments recursively (similar to FunctionCall)
                for arg in &macro_call.arguments {
                    self.generate_expression(arg, f)?;
                }
                // For now, push a placeholder value
                // In a full implementation, we'd expand the macro here
                f.instruction(&Instruction::I32Const(0));
            }
            Expression::CssMacro(_) => {
                // CSS generation handled separately in Sprint 1 Task 1.6
                // Return placeholder for now
                f.instruction(&Instruction::I32Const(0));
            }
            // Reactivity primitives (Phase 12) - Not supported in WASM backend yet
            Expression::Signal(_) | Expression::Computed(_) |
            Expression::Effect(_) | Expression::Batch(_) => {
                // Reactivity is JavaScript-only for now
                // Return placeholder for WASM backend
                f.instruction(&Instruction::I32Const(0));
            }
        }
        Ok(())
    }

    fn generate_jsx_element(&mut self, jsx: &JsxElement, f: &mut Function) -> Result<(), CompileError> {
        // Convert JSX to VDOM structure and generate code to build it
        let vnode = self.jsx_to_vnode(jsx)?;

        // For now, we'll generate calls to DOM creation functions
        self.generate_vnode(&vnode, f)?;

        Ok(())
    }

    fn jsx_to_vnode(&self, jsx: &JsxElement) -> Result<VNode, CompileError> {
        let tag = jsx.opening_tag.name.value.clone();

        // Convert attributes
        let mut attrs = Vec::new();
        for attr in &jsx.opening_tag.attributes {
            // For now, we only handle string literal values
            // Event handlers and expressions need special handling
            let value = match &attr.value {
                Expression::StringLiteral(s) => s.clone(),
                Expression::Lambda(_) => {
                    // Event handler - we'll handle this specially
                    continue;
                }
                _ => "".to_string(), // Placeholder for other expressions
            };
            attrs.push((attr.name.value.clone(), value));
        }

        // Convert children
        let mut children = Vec::new();
        for child in &jsx.children {
            match child {
                JsxChild::Element(child_jsx) => {
                    children.push(self.jsx_to_vnode(child_jsx)?);
                }
                JsxChild::Text(text) => {
                    children.push(VNode::Text(text.clone()));
                }
                JsxChild::Expression(_expr) => {
                    // For now, skip expressions in children
                    // In full implementation, we'd evaluate and convert to text
                    children.push(VNode::Text("{{expr}}".to_string()));
                }
            }
        }

        Ok(VNode::Element { tag, attrs, children })
    }

    #[allow(unused_variables)] // content used in future JSX implementation
    fn generate_vnode(&mut self, vnode: &VNode, f: &mut Function) -> Result<(), CompileError> {
        match vnode {
            VNode::Element { tag: _, attrs: _, children: _ } => {
                // Call createElement(tag) -> elementId
                // For now, we'll just push a dummy element ID
                f.instruction(&Instruction::I32Const(0)); // dummy element ID

                // In a full implementation:
                // 1. Call createElement(tag_ptr, tag_len) to create element
                // 2. For each attribute, call setAttribute(elementId, name_ptr, name_len, value_ptr, value_len)
                // 3. For each child, recursively generate and call appendChild(parentId, childId)
                // 4. Return the element ID
            }
            VNode::Text(content) => {
                // Call createTextNode(content) -> nodeId
                f.instruction(&Instruction::I32Const(0)); // dummy node ID
            }
        }
        Ok(())
    }

    fn generate_function_call(&mut self, call: &FunctionCall, f: &mut Function) -> Result<(), CompileError> {
        // Check if this is a method call (function is a FieldAccess)
        if let Expression::FieldAccess(field_access) = &*call.function {
            return self.generate_method_call(field_access, &call.arguments, f);
        }

        // Generate arguments (push them onto the stack)
        for arg in &call.arguments {
            self.generate_expression(arg, f)?;
        }

        // Check if it's a built-in reactive function
        if let Expression::Identifier(ident) = &*call.function {
            match ident.value.as_str() {
                // Signal operations
                "signal_new" => {
                    // signal_new(initialValue) - returns signal ID
                    f.instruction(&Instruction::Call(self.get_import_index("signal_new")?));
                    return Ok(());
                }
                "signal_get" => {
                    // signal_get(signalId) - returns current value
                    f.instruction(&Instruction::Call(self.get_import_index("signal_get")?));
                    return Ok(());
                }
                "signal_set" => {
                    // signal_set(signalId, newValue)
                    f.instruction(&Instruction::Call(self.get_import_index("signal_set")?));
                    return Ok(());
                }
                "signal_update" => {
                    // signal_update(signalId, delta)
                    f.instruction(&Instruction::Call(self.get_import_index("signal_update")?));
                    return Ok(());
                }
                _ => {
                    // Check if this is a namespaced identifier (e.g., console::log, document::write)
                    // Treat all namespaced identifiers as external imports
                    if ident.value.contains("::") {
                        // Try to get the import index
                        // For now, we'll assume all :: functions are external and skip codegen
                        // In a full implementation, we'd register these as imports
                        return Err(CompileError::Generic(format!(
                            "External function call '{}' - requires FFI import registration", ident.value
                        )));
                    }

                    // Check if this identifier is a local variable (function stored in a variable)
                    // If so, use call_indirect instead of direct call
                    if let Some(&local_index) = self.local_symbol_table.get(&ident.value) {
                        // This is a function stored in a variable - use call_indirect
                        // The function table index is stored in the local variable
                        f.instruction(&Instruction::LocalGet(local_index));

                        // Generate call_indirect instruction
                        // For now, we assume all functions take i32 params and return i32
                        // Type index 0 is typically fn(i32) -> i32, but we need the correct type
                        // TODO: Track function signatures to use the correct type index
                        let type_index = 0;  // Placeholder - should match function signature
                        f.instruction(&Instruction::CallIndirect {
                            ty: type_index,
                            table: 0,  // Table index (we only have one table)
                        });
                        return Ok(());
                    }

                    // Look up user-defined function (direct call)
                    if let Some(&func_idx) = self.func_symbols.funcs.get(&ident.value) {
                        f.instruction(&Instruction::Call(func_idx));
                    } else if ident.value == "Some" || ident.value == "None" || ident.value == "Ok" || ident.value == "Err" {
                        // Built-in Option/Result constructors - skip WASM compilation
                        // These are handled by JavaScript runtime
                        // For WASM, we just push a placeholder value (0)
                        f.instruction(&Instruction::I32Const(0));
                    } else {
                        return Err(CompileError::Generic(format!(
                            "Undefined function: '{}'", ident.value
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    #[allow(unused_variables)] // arguments used in future method implementations
    fn generate_method_call(
        &mut self,
        field_access: &FieldAccessExpression,
        arguments: &[Expression],
        f: &mut Function,
    ) -> Result<(), CompileError> {
        let method_name = &field_access.field.value;

        // Handle array and string methods
        match method_name.as_str() {
            "len" | "length" => {
                // arr.len() or str.length() - returns the length of the array/string
                // Array layout: [length (4 bytes)] [elements...]
                // String layout: [length (4 bytes)] [characters...]
                // Generate the array/string expression to get the pointer
                self.generate_expression(&field_access.object, f)?;

                // Load the length from memory at base_ptr + 0
                f.instruction(&Instruction::I32Load(wasm_encoder::MemArg {
                    offset: 0,
                    align: 2,  // 4-byte alignment for i32
                    memory_index: 0,
                }));

                Ok(())
            }
            "push" => {
                // arr.push(value) - adds an element to the end of the array
                // This is complex because we need to:
                // 1. Read the current length
                // 2. Store the new value at base_ptr + 4 + (length * element_size)
                // 3. Increment the length
                // For now, return a placeholder error
                Err(CompileError::Generic(
                    "Codegen: arr.push() not yet fully implemented - requires mutable arrays".to_string()
                ))
            }
            "pop" => {
                // arr.pop() - removes and returns the last element
                // This would require:
                // 1. Read the current length
                // 2. Decrement the length
                // 3. Return the value at base_ptr + 4 + (new_length * element_size)
                // For now, return a placeholder error
                Err(CompileError::Generic(
                    "Codegen: arr.pop() not yet fully implemented - requires mutable arrays".to_string()
                ))
            }
            "contains" | "join" | "map" => {
                // String/array methods that return values
                // For now, return a placeholder value (1 for contains = found, 0 for not found)
                // TODO: Implement actual string searching and array manipulation in WASM
                f.instruction(&Instruction::I32Const(1));
                Ok(())
            }
            _ => {
                // Unknown method - return a placeholder value to allow compilation to continue
                // TODO: Add comprehensive method support
                f.instruction(&Instruction::I32Const(0));
                Ok(())
            }
        }
    }

    fn get_import_index(&self, name: &str) -> Result<u32, CompileError> {
        self.func_symbols.funcs.get(name).copied().ok_or_else(|| {
            CompileError::Generic(format!("Import '{}' not found", name))
        })
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
                        if self.struct_table.structs.contains_key(&ident.value) {
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
            TypeExpression::Reference(inner) => {
                // References are represented as pointers in WASM (i32)
                // For now, treat references the same as the underlying type
                // In a full implementation, we'd track reference semantics
                self.type_expression_to_resolved_type(inner)
            }
            TypeExpression::MutableReference(inner) => {
                // Mutable references are represented as pointers in WASM (i32)
                // For now, treat mutable references the same as the underlying type
                // In a full implementation, we'd track reference semantics
                self.type_expression_to_resolved_type(inner)
            }
            TypeExpression::Slice(inner) => {
                // Slices are array views - recursively process the inner type
                // Return ResolvedType::Array with the inner type
                let inner_type = self.type_expression_to_resolved_type(inner);
                ResolvedType::Array(Box::new(inner_type))
            }
            TypeExpression::SizedArray(inner, _size) => {
                // Sized arrays [T; N] - treat as arrays with the inner type
                // Size information is primarily for type checking, not runtime
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

    /// Allocates memory for a struct and returns the pointer
    fn allocate_struct(&mut self, size: u32, f: &mut Function) {
        // Push the current heap pointer (this will be the struct address)
        f.instruction(&Instruction::I32Const(self.heap_pointer as i32));

        // Update heap pointer for next allocation
        self.heap_pointer += size;
    }

    /// Attempts to infer the struct type from an expression
    /// This is a simplified version - in a full implementation, we'd use the semantic analyzer's type information
    fn infer_struct_type(&self, expr: &Expression) -> Result<String, CompileError> {
        match expr {
            Expression::Identifier(ident) => {
                // Look up the variable in our local type table
                if let Some(struct_name) = self.local_type_table.get(&ident.value) {
                    Ok(struct_name.clone())
                } else {
                    Err(CompileError::Generic(format!(
                        "Codegen: Cannot infer struct type for variable '{}' (not tracked)",
                        ident.value
                    )))
                }
            }
            Expression::StructLiteral(lit) => {
                // Easy case - we know the struct type from the literal
                Ok(lit.name.value.clone())
            }
            Expression::FieldAccess(field_access) => {
                // Recursively infer the type of nested field access
                // The type is determined by the base object
                self.infer_struct_type(&field_access.object)
            }
            _ => Err(CompileError::Generic(
                "Codegen: Cannot infer struct type for this expression".to_string()
            )),
        }
    }

    /// Generates WASM code for match arms using nested if/else blocks
    /// The scrutinee value is already on the stack when this is called
    fn generate_match_arms(&mut self, arms: &[MatchArm], f: &mut Function) -> Result<(), CompileError> {
        // We need to store the scrutinee in a local so we can compare it multiple times
        // Allocate a new local for the scrutinee value
        let scrutinee_local = self.local_count;
        self.local_count += 1;

        // Store the scrutinee value (which is already on the stack)
        f.instruction(&Instruction::LocalSet(scrutinee_local));

        // Generate nested if/else blocks for each arm
        self.generate_match_arm(arms, 0, scrutinee_local, f)?;

        Ok(())
    }

    /// Recursively generates WASM code for a single match arm
    /// This creates a nested if/else structure
    #[allow(unused_variables)] // name and fields used in future enum matching implementation
    fn generate_match_arm(
        &mut self,
        arms: &[MatchArm],
        arm_index: usize,
        scrutinee_local: u32,
        f: &mut Function,
    ) -> Result<(), CompileError> {
        if arm_index >= arms.len() {
            // This should never happen due to exhaustiveness checking,
            // but if it does, return a dummy value
            f.instruction(&Instruction::I32Const(0));
            return Ok(());
        }

        let arm = &arms[arm_index];

        // For OR patterns, use the first pattern for WASM generation
        // TODO: Implement proper OR pattern matching in WASM
        let pattern = &arm.patterns[0];

        match pattern {
            Pattern::Wildcard | Pattern::Identifier(_) => {
                // Wildcard or identifier patterns always match
                // Just generate the body expression
                self.generate_expression(&arm.body, f)?;
            }
            Pattern::Tuple(_) => {
                // TODO: Implement tuple pattern matching in WASM
                // For now, treat as wildcard
                self.generate_expression(&arm.body, f)?;
            }
            Pattern::Literal(literal_expr) => {
                // For literal patterns, we need to compare the scrutinee with the literal

                // Load the scrutinee value
                f.instruction(&Instruction::LocalGet(scrutinee_local));

                // Generate the literal value
                self.generate_expression(literal_expr, f)?;

                // Compare them for equality
                f.instruction(&Instruction::I32Eq);

                // Start an if block (ValType::I32 means the block produces an i32 value)
                f.instruction(&Instruction::If(wasm_encoder::BlockType::Result(ValType::I32)));

                // If they match, generate this arm's body
                self.generate_expression(&arm.body, f)?;

                // Otherwise, try the next arm
                f.instruction(&Instruction::Else);

                // Recursively generate the next arm
                self.generate_match_arm(arms, arm_index + 1, scrutinee_local, f)?;

                // End the if block
                f.instruction(&Instruction::End);
            }
            Pattern::EnumVariant { name, fields } => {
                // For enum variants like Result::Ok or Result::Err
                // For now, we'll generate a simplified match that just executes the body
                // TODO: Implement proper enum tag checking and field extraction

                // Just generate the body expression for this arm
                // In a full implementation, we would:
                // 1. Load the enum tag from scrutinee (first field at offset 0)
                // 2. Compare with expected variant tag (0 for Ok, 1 for Err, etc.)
                // 3. Extract variant data if fields are present
                // 4. Bind fields to local variables

                // For now, assume this arm matches and generate its body
                self.generate_expression(&arm.body, f)?;
            }
        }

        Ok(())
    }

    // --- Lambda Collection Methods ---

    /// Collects all lambda expressions from a program for conversion to anonymous functions
    fn collect_lambdas_from_program(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.collect_lambdas_from_statement(stmt);
        }
    }

    /// Recursively collects lambda expressions from a statement
    fn collect_lambdas_from_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(let_stmt) => {
                self.collect_lambdas_from_expression(&let_stmt.value);
            }
            Statement::Assignment(assign_stmt) => {
                self.collect_lambdas_from_expression(&assign_stmt.value);
            }
            Statement::Return(return_stmt) => {
                self.collect_lambdas_from_expression(&return_stmt.value);
            }
            Statement::Expression(expr) => {
                self.collect_lambdas_from_expression(expr);
            }
            Statement::If(if_stmt) => {
                self.collect_lambdas_from_expression(&if_stmt.condition);
                for s in &if_stmt.then_branch.statements {
                    self.collect_lambdas_from_statement(s);
                }
                if let Some(else_stmt) = &if_stmt.else_branch {
                    self.collect_lambdas_from_statement(else_stmt);
                }
            }
            Statement::While(while_stmt) => {
                self.collect_lambdas_from_expression(&while_stmt.condition);
                for s in &while_stmt.body.statements {
                    self.collect_lambdas_from_statement(s);
                }
            }
            Statement::For(for_stmt) => {
                if let Some(init) = &for_stmt.init {
                    self.collect_lambdas_from_statement(init);
                }
                self.collect_lambdas_from_expression(&for_stmt.condition);
                for s in &for_stmt.body.statements {
                    self.collect_lambdas_from_statement(s);
                }
                if let Some(update) = &for_stmt.update {
                    self.collect_lambdas_from_statement(update);
                }
            }
            Statement::ForIn(for_in_stmt) => {
                self.collect_lambdas_from_expression(&for_in_stmt.iterator);
                for s in &for_in_stmt.body.statements {
                    self.collect_lambdas_from_statement(s);
                }
            }
            Statement::Function(func_def) => {
                for s in &func_def.body.statements {
                    self.collect_lambdas_from_statement(s);
                }
            }
            Statement::Component(comp) => {
                for s in &comp.body.statements {
                    self.collect_lambdas_from_statement(s);
                }
            }
            _ => {}
        }
    }

    /// Recursively collects lambda expressions from an expression
    fn collect_lambdas_from_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Lambda(lambda) => {
                // Found a lambda! Add it to the lambda table
                // For now, we'll assign placeholder indices - these will be updated later
                let lambda_info = LambdaInfo {
                    func_index: 0,  // Will be set during Pass 1.5
                    type_index: 0,  // Will be set during Pass 1.5
                    parameters: lambda.parameters.clone(),
                    body: lambda.body.clone(),
                    captured_vars: Vec::new(),  // TODO: Implement capture analysis
                };
                self.lambda_table.add_lambda(lambda_info);

                // Also collect lambdas from the lambda body
                self.collect_lambdas_from_expression(&lambda.body);
            }
            Expression::Prefix(prefix) => {
                self.collect_lambdas_from_expression(&prefix.right);
            }
            Expression::Spread(spread) => {
                self.collect_lambdas_from_expression(&spread.expression);
            }
            Expression::Infix(infix) => {
                self.collect_lambdas_from_expression(&infix.left);
                self.collect_lambdas_from_expression(&infix.right);
            }
            Expression::FunctionCall(call) => {
                self.collect_lambdas_from_expression(&call.function);
                for arg in &call.arguments {
                    self.collect_lambdas_from_expression(arg);
                }
            }
            Expression::ArrayLiteral(array_lit) => {
                for elem in &array_lit.elements {
                    self.collect_lambdas_from_expression(elem);
                }
            }
            Expression::StructLiteral(struct_lit) => {
                for (_, field_value) in &struct_lit.fields {
                    self.collect_lambdas_from_expression(field_value);
                }
            }
            Expression::FieldAccess(field_access) => {
                self.collect_lambdas_from_expression(&field_access.object);
            }
            Expression::Match(match_expr) => {
                self.collect_lambdas_from_expression(&match_expr.scrutinee);
                for arm in &match_expr.arms {
                    self.collect_lambdas_from_expression(&arm.body);
                }
            }
            Expression::IndexAccess(index_expr) => {
                self.collect_lambdas_from_expression(&index_expr.array);
                self.collect_lambdas_from_expression(&index_expr.index);
            }
            Expression::TupleLiteral(tuple_lit) => {
                for elem in &tuple_lit.elements {
                    self.collect_lambdas_from_expression(elem);
                }
            }
            Expression::Borrow(borrow_expr) => {
                self.collect_lambdas_from_expression(&borrow_expr.expression);
            }
            Expression::MutableBorrow(borrow_expr) => {
                self.collect_lambdas_from_expression(&borrow_expr.expression);
            }
            Expression::Dereference(deref_expr) => {
                self.collect_lambdas_from_expression(&deref_expr.expression);
            }
            Expression::IfExpression(if_expr) => {
                self.collect_lambdas_from_expression(&if_expr.condition);
                self.collect_lambdas_from_expression(&if_expr.then_expr);
                if let Some(else_expr) = &if_expr.else_expr {
                    self.collect_lambdas_from_expression(else_expr);
                }
            }
            Expression::Ternary(ternary) => {
                self.collect_lambdas_from_expression(&ternary.condition);
                self.collect_lambdas_from_expression(&ternary.true_expr);
                self.collect_lambdas_from_expression(&ternary.false_expr);
            }
            Expression::TypeCast(type_cast) => {
                self.collect_lambdas_from_expression(&type_cast.expression);
            }
            Expression::Block(block) => {
                for stmt in &block.statements {
                    self.collect_lambdas_from_statement(stmt);
                }
            }
            Expression::JsxElement(jsx) => {
                // Collect lambdas from JSX attributes (event handlers)
                for attr in &jsx.opening_tag.attributes {
                    self.collect_lambdas_from_expression(&attr.value);
                }
                // Collect lambdas from JSX children
                for child in &jsx.children {
                    if let JsxChild::Expression(expr) = child {
                        self.collect_lambdas_from_expression(expr);
                    } else if let JsxChild::Element(child_jsx) = child {
                        self.collect_lambdas_from_expression(&Expression::JsxElement(*child_jsx.clone()));
                    }
                }
            }
            Expression::MacroCall(macro_call) => {
                // Collect lambdas from macro arguments
                for arg in &macro_call.arguments {
                    self.collect_lambdas_from_expression(arg);
                }
            }
            Expression::CssMacro(_) => {
                // CSS macro doesn't contain lambdas
            }
            // Reactivity primitives (Phase 12) - Collect lambdas from callbacks
            Expression::Signal(signal_expr) => {
                self.collect_lambdas_from_expression(&signal_expr.initial_value);
            }
            Expression::Computed(computed_expr) => {
                self.collect_lambdas_from_expression(&computed_expr.computation);
            }
            Expression::Effect(effect_expr) => {
                self.collect_lambdas_from_expression(&effect_expr.callback);
            }
            Expression::Batch(batch_expr) => {
                self.collect_lambdas_from_expression(&batch_expr.body);
            }
            // Base cases - no nested expressions to search
            Expression::IntegerLiteral(_)
            | Expression::FloatLiteral(_)
            | Expression::BoolLiteral(_)
            | Expression::UnitLiteral
            | Expression::StringLiteral(_)
            | Expression::Identifier(_)
            | Expression::Range(_)
            | Expression::TryOperator(_)
            | Expression::Await(_) => {}
        }
    }

    // --- Capture Analysis Methods ---
    // These methods are for future closure capture implementation (Issue #3)

    /// Analyzes a lambda expression to identify captured variables
    /// Returns a list of variable names that are referenced but not parameters
    #[allow(dead_code)]
    fn analyze_captures(&self, lambda: &LambdaExpression, enclosing_scope: &HashMap<String, u32>) -> Vec<String> {
        use std::collections::HashSet;

        // Collect all variable references in the lambda body
        let mut referenced_vars = HashSet::new();
        self.collect_variable_references(&lambda.body, &mut referenced_vars);

        // Build a set of parameter names for quick lookup
        let param_names: HashSet<String> = lambda.parameters
            .iter()
            .map(|p| p.name.value.clone())
            .collect();

        // Filter out parameters and function names - only keep variables from enclosing scope
        let mut captured: Vec<String> = referenced_vars
            .into_iter()
            .filter(|var_name| {
                // Not a parameter
                !param_names.contains(var_name) &&
                // Not a function name
                !self.func_symbols.funcs.contains_key(var_name) &&
                // Is in enclosing scope
                enclosing_scope.contains_key(var_name)
            })
            .collect();

        // Sort for deterministic output
        captured.sort();
        captured
    }

    /// Recursively collects all variable references (identifiers) in an expression
    #[allow(dead_code)]
    fn collect_variable_references(&self, expr: &Expression, vars: &mut std::collections::HashSet<String>) {
        match expr {
            Expression::Identifier(ident) => {
                vars.insert(ident.value.clone());
            }
            Expression::Prefix(prefix) => {
                self.collect_variable_references(&prefix.right, vars);
            }
            Expression::Spread(spread) => {
                self.collect_variable_references(&spread.expression, vars);
            }
            Expression::Infix(infix) => {
                self.collect_variable_references(&infix.left, vars);
                self.collect_variable_references(&infix.right, vars);
            }
            Expression::FunctionCall(call) => {
                self.collect_variable_references(&call.function, vars);
                for arg in &call.arguments {
                    self.collect_variable_references(arg, vars);
                }
            }
            Expression::ArrayLiteral(array_lit) => {
                for elem in &array_lit.elements {
                    self.collect_variable_references(elem, vars);
                }
            }
            Expression::StructLiteral(struct_lit) => {
                for (_, field_value) in &struct_lit.fields {
                    self.collect_variable_references(field_value, vars);
                }
            }
            Expression::FieldAccess(field_access) => {
                self.collect_variable_references(&field_access.object, vars);
            }
            Expression::Match(match_expr) => {
                self.collect_variable_references(&match_expr.scrutinee, vars);
                for arm in &match_expr.arms {
                    self.collect_variable_references(&arm.body, vars);
                }
            }
            Expression::IndexAccess(index_expr) => {
                self.collect_variable_references(&index_expr.array, vars);
                self.collect_variable_references(&index_expr.index, vars);
            }
            Expression::TupleLiteral(tuple_lit) => {
                for elem in &tuple_lit.elements {
                    self.collect_variable_references(elem, vars);
                }
            }
            Expression::Borrow(borrow_expr) => {
                self.collect_variable_references(&borrow_expr.expression, vars);
            }
            Expression::MutableBorrow(borrow_expr) => {
                self.collect_variable_references(&borrow_expr.expression, vars);
            }
            Expression::Dereference(deref_expr) => {
                self.collect_variable_references(&deref_expr.expression, vars);
            }
            Expression::IfExpression(if_expr) => {
                self.collect_variable_references(&if_expr.condition, vars);
                self.collect_variable_references(&if_expr.then_expr, vars);
                if let Some(else_expr) = &if_expr.else_expr {
                    self.collect_variable_references(else_expr, vars);
                }
            }
            Expression::Block(block) => {
                for stmt in &block.statements {
                    self.collect_variable_references_from_statement(stmt, vars);
                }
            }
            Expression::Lambda(_) => {
                // Don't traverse into nested lambdas - they have their own scope
            }
            Expression::MacroCall(macro_call) => {
                // Collect variable references from macro arguments
                for arg in &macro_call.arguments {
                    self.collect_variable_references(arg, vars);
                }
            }
            Expression::CssMacro(_) => {
                // CSS macro doesn't contain variable references
            }
            // Reactivity primitives (Phase 12) - Collect variable references
            Expression::Signal(signal_expr) => {
                self.collect_variable_references(&signal_expr.initial_value, vars);
            }
            Expression::Computed(computed_expr) => {
                self.collect_variable_references(&computed_expr.computation, vars);
            }
            Expression::Effect(effect_expr) => {
                self.collect_variable_references(&effect_expr.callback, vars);
            }
            Expression::Batch(batch_expr) => {
                self.collect_variable_references(&batch_expr.body, vars);
            }
            // Base cases - no variable references
            Expression::IntegerLiteral(_)
            | Expression::FloatLiteral(_)
            | Expression::BoolLiteral(_)
            | Expression::UnitLiteral
            | Expression::StringLiteral(_)
            | Expression::Range(_)
            | Expression::TryOperator(_)
            | Expression::Ternary(_)
            | Expression::TypeCast(_)
            | Expression::Await(_)
            | Expression::JsxElement(_) => {}
        }
    }

    /// Recursively collects variable references from a statement
    #[allow(dead_code)]
    fn collect_variable_references_from_statement(&self, stmt: &Statement, vars: &mut std::collections::HashSet<String>) {
        match stmt {
            Statement::Let(let_stmt) => {
                self.collect_variable_references(&let_stmt.value, vars);
            }
            Statement::Assignment(assign_stmt) => {
                // Collect variable references from the target
                self.collect_variable_references(&assign_stmt.target, vars);
                // Collect variable references from the value
                self.collect_variable_references(&assign_stmt.value, vars);
            }
            Statement::Return(return_stmt) => {
                self.collect_variable_references(&return_stmt.value, vars);
            }
            Statement::Expression(expr) => {
                self.collect_variable_references(expr, vars);
            }
            Statement::If(if_stmt) => {
                self.collect_variable_references(&if_stmt.condition, vars);
                for s in &if_stmt.then_branch.statements {
                    self.collect_variable_references_from_statement(s, vars);
                }
                if let Some(else_stmt) = &if_stmt.else_branch {
                    self.collect_variable_references_from_statement(else_stmt, vars);
                }
            }
            Statement::While(while_stmt) => {
                self.collect_variable_references(&while_stmt.condition, vars);
                for s in &while_stmt.body.statements {
                    self.collect_variable_references_from_statement(s, vars);
                }
            }
            Statement::For(for_stmt) => {
                if let Some(init) = &for_stmt.init {
                    self.collect_variable_references_from_statement(init, vars);
                }
                self.collect_variable_references(&for_stmt.condition, vars);
                for s in &for_stmt.body.statements {
                    self.collect_variable_references_from_statement(s, vars);
                }
                if let Some(update) = &for_stmt.update {
                    self.collect_variable_references_from_statement(update, vars);
                }
            }
            Statement::ForIn(for_in_stmt) => {
                self.collect_variable_references(&for_in_stmt.iterator, vars);
                for s in &for_in_stmt.body.statements {
                    self.collect_variable_references_from_statement(s, vars);
                }
            }
            _ => {}
        }
    }

    // ==================== PHASE 13: STYLE SYSTEM GENERATION ====================

    /// Generate CSS from a theme block (creates CSS custom properties)
    /// theme DarkMode { primary: #1a1a1a; } -> :root { --DarkMode-primary: #1a1a1a; }
    fn generate_theme_block_css(&mut self, theme: &ThemeBlock) -> Result<(), CompileError> {
        self.css_output.push_str(":root {\n");

        for prop in &theme.properties {
            let var_name = format!("--{}-{}", theme.name.value, prop.name);
            self.css_output.push_str(&format!("  {}: {};\n", var_name, prop.value));
        }

        self.css_output.push_str("}\n\n");
        Ok(())
    }

    /// Generate CSS from a style block (creates scoped classes)
    /// style Button { background: blue; } -> .Button_a1b2c3 { background: blue; }
    fn generate_style_block_css(&mut self, style: &StyleBlock) -> Result<(), CompileError> {
        // Generate scoped class name with hash
        let class_name = self.generate_scoped_class_name(&style.name.value);

        // Generate main rule
        if !style.properties.is_empty() {
            self.css_output.push_str(&format!(".{} {{\n", class_name));

            for prop in &style.properties {
                let value = self.resolve_style_value(&prop.value);
                self.css_output.push_str(&format!("  {}: {};\n", prop.name, value));
            }

            self.css_output.push_str("}\n\n");
        }

        // Generate nested selectors
        for nested in &style.nested {
            let nested_selector = match &nested.selector {
                SelectorType::PseudoClass(pseudo) => {
                    format!(".{}:{}", class_name, pseudo)
                }
                SelectorType::Class(cls) => {
                    format!(".{}.{}", class_name, cls)
                }
            };

            if !nested.properties.is_empty() {
                self.css_output.push_str(&format!("{} {{\n", nested_selector));

                for prop in &nested.properties {
                    let value = self.resolve_style_value(&prop.value);
                    self.css_output.push_str(&format!("  {}: {};\n", prop.name, value));
                }

                self.css_output.push_str("}\n\n");
            }
        }

        Ok(())
    }

    /// Generate a scoped class name with hash
    /// "Button" -> "Button_a1b2c3"
    fn generate_scoped_class_name(&self, name: &str) -> String {
        use sha2::{Sha256, Digest};

        // Create hash from name for now (could include file path for more uniqueness)
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result)[..6].to_string();

        format!("{}_{}", name, hash)
    }

    /// Resolve a style value (literal or theme reference)
    /// StyleValue::Literal("blue") -> "blue"
    /// StyleValue::ThemeRef { theme: "DarkMode", property: "primary" } -> "var(--DarkMode-primary)"
    fn resolve_style_value(&self, value: &StyleValue) -> String {
        match value {
            StyleValue::Literal(lit) => lit.clone(),
            StyleValue::ThemeRef { theme, property } => {
                format!("var(--{}-{})", theme, property)
            }
        }
    }
}