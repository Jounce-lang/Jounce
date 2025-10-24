// Module Loader for Jounce
// Handles compile-time module resolution and import processing

use crate::ast::{Program, Statement, FunctionDefinition, StructDefinition, EnumDefinition, ConstDeclaration, Identifier, UseStatement};
use crate::errors::CompileError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents an exported symbol from a module
#[derive(Debug, Clone)]
pub enum ExportedSymbol {
    Function(FunctionDefinition),
    Struct(StructDefinition),
    Enum(EnumDefinition),
    Const(ConstDeclaration),
    Type(Identifier),
}

/// Represents a loaded module with its exports
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub file_path: PathBuf,
    pub exports: HashMap<String, ExportedSymbol>,
    pub ast: Program,
}

/// Module loader for resolving and loading Jounce modules
pub struct ModuleLoader {
    /// Root directory for package resolution (usually project root or aloha-shirts/)
    package_root: PathBuf,
    /// Cache of loaded modules to avoid re-parsing
    module_cache: HashMap<String, Module>,
    /// Set of module paths currently being loaded (for cycle detection)
    loading_stack: HashSet<String>,
    /// Current file being processed (for relative path resolution)
    current_file: Option<PathBuf>,
}

impl ModuleLoader {
    /// Create a new module loader with the given package root
    pub fn new<P: AsRef<Path>>(package_root: P) -> Self {
        Self {
            package_root: package_root.as_ref().to_path_buf(),
            module_cache: HashMap::new(),
            loading_stack: HashSet::new(),
            current_file: None,
        }
    }

    /// Set the current file being processed (for relative path resolution)
    pub fn set_current_file<P: AsRef<Path>>(&mut self, file_path: P) {
        self.current_file = Some(file_path.as_ref().to_path_buf());
    }

    /// Resolve a module path to a filesystem path
    ///
    /// Examples:
    /// - `raven_router` -> `aloha-shirts/raven-router/src/lib.jnc`
    /// - `raven_store::store` -> `aloha-shirts/raven-store/src/store/store.jnc`
    /// - `./math` -> `./math.jnc` (relative to current directory)
    /// - `../utils/helpers` -> `../utils/helpers.jnc`
    pub fn resolve_module_path(&self, module_path: &[String]) -> Result<PathBuf, CompileError> {
        if module_path.is_empty() {
            return Err(CompileError::Generic("Empty module path".to_string()));
        }

        // Check if this is a relative path (starts with . or ..)
        let is_relative = module_path[0] == "." || module_path[0] == "..";

        if is_relative {
            // Resolve relative path from the current file's directory
            let base_path = if let Some(ref current) = self.current_file {
                // Get the directory of the current file
                current.parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("."))
            } else {
                // Fallback to current working directory if no context
                PathBuf::from(".")
            };

            let mut path = base_path;

            for segment in module_path {
                if segment == "." {
                    // Current directory - no-op
                    continue;
                } else if segment == ".." {
                    // Parent directory
                    path.pop();
                } else {
                    // Regular path segment
                    path.push(segment);
                }
            }

            // Add .jnc extension
            path.set_extension("jnc");

            if path.exists() {
                return Ok(path);
            } else {
                return Err(CompileError::Generic(format!(
                    "Local module not found: {:?}",
                    path
                )));
            }
        }

        // Package import (existing logic)
        // Convert module name from snake_case to kebab-case for directory lookup
        // raven_router -> raven-router
        let package_name = module_path[0].replace('_', "-");

        // Try multiple package root locations
        let package_roots = vec![
            PathBuf::from("test_modules"),  // For testing
            PathBuf::from("aloha-shirts"),   // Default package location
            self.package_root.clone(),       // User-specified root
        ];

        for root in package_roots {
            let mut path = root.join(&package_name);

            // If there are submodules (e.g., raven_store::store::computed)
            if module_path.len() > 1 {
                path = path.join("src");
                for segment in &module_path[1..] {
                    path = path.join(segment);
                }
                path = path.with_extension("jnc");
            } else {
                // Just the package name - look for lib.jnc
                path = path.join("src").join("lib.jnc");
            }

            if path.exists() {
                return Ok(path);
            }
        }

        // If not found in any location, return error
        Err(CompileError::Generic(format!(
            "Module not found: {} (searched in test_modules, aloha-shirts, and {:?})",
            module_path.join("::"),
            self.package_root
        )))
    }

    /// Load a module from the given path
    pub fn load_module(&mut self, module_path: &[String]) -> Result<&Module, CompileError> {
        let module_key = module_path.join("::");

        // Check if already loaded
        if self.module_cache.contains_key(&module_key) {
            return Ok(self.module_cache.get(&module_key).unwrap());
        }

        // Check for circular dependencies
        if self.loading_stack.contains(&module_key) {
            return Err(CompileError::Generic(format!(
                "Circular module dependency detected: {}",
                module_key
            )));
        }

        // Mark as loading
        self.loading_stack.insert(module_key.clone());

        // Resolve the file path
        let file_path = self.resolve_module_path(module_path)?;

        // Set current file context for nested imports
        let previous_file = self.current_file.clone();
        self.current_file = Some(file_path.clone());

        // Read the file
        let source = fs::read_to_string(&file_path)
            .map_err(|e| CompileError::Generic(format!(
                "Failed to read module {}: {}",
                module_key, e
            )))?;

        // Parse the module
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        let mut ast = parser.parse_program()?;

        // Process imports in this module (recursive)
        let _nested_imports = self.merge_imports(&mut ast)?;

        // Extract exports
        let exports = self.extract_exports(&ast)?;

        // Restore previous file context
        self.current_file = previous_file;

        // Create the module
        let module = Module {
            name: module_key.clone(),
            file_path,
            exports,
            ast,
        };

        // Done loading
        self.loading_stack.remove(&module_key);

        // Cache the module
        self.module_cache.insert(module_key.clone(), module);

        Ok(self.module_cache.get(&module_key).unwrap())
    }

    /// Extract exported symbols from a module's AST
    fn extract_exports(&self, program: &Program) -> Result<HashMap<String, ExportedSymbol>, CompileError> {
        let mut exports = HashMap::new();

        for statement in &program.statements {
            match statement {
                Statement::Function(func) => {
                    // In Jounce, all top-level functions are currently exported
                    // TODO: Add explicit `pub` keyword support
                    exports.insert(func.name.value.clone(), ExportedSymbol::Function(func.clone()));
                }
                Statement::Struct(struct_def) => {
                    exports.insert(struct_def.name.value.clone(), ExportedSymbol::Struct(struct_def.clone()));
                }
                Statement::Enum(enum_def) => {
                    exports.insert(enum_def.name.value.clone(), ExportedSymbol::Enum(enum_def.clone()));
                }
                Statement::Const(const_decl) => {
                    exports.insert(const_decl.name.value.clone(), ExportedSymbol::Const(const_decl.clone()));
                }
                // TODO: Handle type aliases
                _ => {}
            }
        }

        Ok(exports)
    }

    /// Get a specific export from a module
    pub fn get_export(&mut self, module_path: &[String], symbol_name: &str) -> Result<ExportedSymbol, CompileError> {
        let module = self.load_module(module_path)?;

        module.exports.get(symbol_name)
            .cloned()
            .ok_or_else(|| CompileError::Generic(format!(
                "Symbol '{}' not found in module {}",
                symbol_name,
                module_path.join("::")
            )))
    }

    /// Get multiple exports from a module
    pub fn get_exports(&mut self, module_path: &[String], symbol_names: &[String]) -> Result<HashMap<String, ExportedSymbol>, CompileError> {
        let module = self.load_module(module_path)?;
        let mut result = HashMap::new();

        for symbol_name in symbol_names {
            if let Some(export) = module.exports.get(symbol_name) {
                result.insert(symbol_name.clone(), export.clone());
            } else {
                return Err(CompileError::Generic(format!(
                    "Symbol '{}' not found in module {}",
                    symbol_name,
                    module_path.join("::")
                )));
            }
        }

        Ok(result)
    }

    /// Get all exports from a module (for wildcard imports)
    pub fn get_all_exports(&mut self, module_path: &[String]) -> Result<HashMap<String, ExportedSymbol>, CompileError> {
        let module = self.load_module(module_path)?;
        Ok(module.exports.clone())
    }

    /// Clear the module cache (useful for testing)
    pub fn clear_cache(&mut self) {
        self.module_cache.clear();
        self.loading_stack.clear();
    }

    /// Merge imported module definitions into a program's AST
    ///
    /// This processes all `use` statements and adds the imported definitions
    /// to the program, making them available for code generation.
    ///
    /// Returns: Vec<PathBuf> - List of file paths that were imported
    pub fn merge_imports(&mut self, program: &mut Program) -> Result<Vec<PathBuf>, CompileError> {
        // Track imported files for dependency tracking
        let mut imported_files: Vec<PathBuf> = Vec::new();

        // First, collect all use statements and find insertion point
        let mut use_statements: Vec<UseStatement> = Vec::new();
        let mut last_use_index = 0;

        for (i, stmt) in program.statements.iter().enumerate() {
            if let Statement::Use(use_stmt) = stmt {
                use_statements.push(use_stmt.clone());
                last_use_index = i + 1; // Insert after the last use statement
            }
        }

        // Keep track of which symbols we've already added to avoid duplicates
        let mut imported_symbols: HashMap<String, bool> = HashMap::new();

        // Collect all imported statements first
        let mut statements_to_insert: Vec<Statement> = Vec::new();

        // For each use statement, load the module and add imported definitions
        for use_stmt in use_statements {
            let module_path: Vec<String> = use_stmt.path.iter()
                .map(|ident| ident.value.clone())
                .collect();

            // Load the module
            let module = self.load_module(&module_path)?;

            // Track this imported file
            imported_files.push(module.file_path.clone());

            // Determine which symbols to import
            let symbols_to_import: Vec<(String, ExportedSymbol)> = if use_stmt.imports.is_empty() {
                // Wildcard import - import all exports
                module.exports.clone().into_iter().collect()
            } else {
                // Selective import - only import specified symbols
                use_stmt.imports.iter()
                    .filter_map(|ident| {
                        let name = ident.value.clone();
                        module.exports.get(&name)
                            .map(|export| (name, export.clone()))
                    })
                    .collect()
            };

            // Add each imported symbol to the collection
            for (symbol_name, export) in symbols_to_import {
                // Skip if already imported
                if imported_symbols.contains_key(&symbol_name) {
                    continue;
                }

                // Add the definition to the collection based on symbol type
                let statement = match export {
                    ExportedSymbol::Function(func) => {
                        Some(Statement::Function(func))
                    }
                    ExportedSymbol::Struct(struct_def) => {
                        Some(Statement::Struct(struct_def))
                    }
                    ExportedSymbol::Enum(enum_def) => {
                        Some(Statement::Enum(enum_def))
                    }
                    ExportedSymbol::Const(const_decl) => {
                        Some(Statement::Const(const_decl))
                    }
                    ExportedSymbol::Type(_) => {
                        // Type aliases - skip for now
                        // TODO: Add type alias support
                        None
                    }
                };

                if let Some(stmt) = statement {
                    statements_to_insert.push(stmt);
                }

                imported_symbols.insert(symbol_name, true);
            }
        }

        // Insert all imported statements after the last use statement
        // This ensures they're available before any code that uses them
        for (i, stmt) in statements_to_insert.into_iter().enumerate() {
            program.statements.insert(last_use_index + i, stmt);
        }

        Ok(imported_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_path_resolution() {
        let loader = ModuleLoader::new("aloha-shirts");

        // Test simple package path
        let path = loader.resolve_module_path(&["raven_router".to_string()]);
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("raven-router"));
        assert!(path.to_string_lossy().contains("lib.jnc"));
    }

    #[test]
    fn test_snake_to_kebab_conversion() {
        let loader = ModuleLoader::new("aloha-shirts");
        let path = loader.resolve_module_path(&["raven_router".to_string()]).unwrap();
        assert!(path.to_string_lossy().contains("raven-router"));
        assert!(!path.to_string_lossy().contains("raven_router"));
    }
}
