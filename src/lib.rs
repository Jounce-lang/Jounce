//#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod borrow_checker;
pub mod codegen;
pub mod css_generator; // CSS generation (Phase 7.5)
pub mod deployer; // Make sure deployer is a module
pub mod errors;
pub mod lexer;
pub mod macros;
pub mod parser;
pub mod semantic_analyzer;
pub mod token;
pub mod vdom;
pub mod stdlib; // Standard library
pub mod types; // Type system
pub mod type_checker; // Type checking and inference
pub mod ssr; // Server-side rendering
pub mod hydration; // Client-side hydration
pub mod reactive; // Reactive state management
pub mod router; // Client-side routing
pub mod forms; // Forms and validation
pub mod animation; // Animation system
pub mod diagnostics; // Enhanced error reporting
pub mod wasm_runtime; // WebAssembly runtime support
pub mod lsp; // Language Server Protocol
pub mod hmr; // Hot Module Replacement
pub mod package_manager; // Package Manager
pub mod module_loader; // Module loader for compile-time imports
pub mod source_map; // Source map generation for debugging
pub mod wasm_optimizer; // WASM optimization (DCE, inlining, constant folding)
pub mod doc_generator; // Documentation generator (raven doc)
pub mod profiler; // Performance profiling
pub mod code_splitter; // Code splitting for server/client separation
pub mod rpc_generator; // RPC stub generation for client/server communication
pub mod js_emitter; // JavaScript code generation for server and client bundles
pub mod js_minifier; // JavaScript minification for production builds
pub mod formatter; // Code formatter for consistent style
pub mod watcher; // File watching and auto-recompilation

use borrow_checker::BorrowChecker;
use codegen::CodeGenerator;
use errors::CompileError;
use lexer::Lexer;
use parser::Parser;
use semantic_analyzer::SemanticAnalyzer;
use type_checker::TypeChecker;
use token::{Token, TokenKind};
use wasm_optimizer::WasmOptimizer;

// This enum is now public so the deployer can use it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildTarget {
    Client,
    Server,
}

pub struct Compiler {
    pub optimize: bool,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            optimize: true,  // Enable optimizations by default
        }
    }

    /// Create a compiler with optimizations disabled
    pub fn without_optimization() -> Self {
        Compiler {
            optimize: false,
        }
    }

    // FIX: The function now takes the target as a required argument.
    pub fn compile_source(&self, source: &str, target: BuildTarget) -> Result<Vec<u8>, CompileError> {
        println!("   - Starting compilation for target: {:?}", target);

        // --- Lexing, Parsing, Macro Expansion ---
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(&mut lexer);
        let initial_ast = parser.parse_program()?;

        // This is a simplified macro expansion for now.
        let mut needs_reparse = false;
        for statement in &initial_ast.statements {
            if let ast::Statement::MacroInvocation(_) = statement {
                needs_reparse = true;
                break;
            }
        }

        // FIX: Rename the AST variable to avoid shadowing the `ast` module.
        let mut program_ast = if needs_reparse {
            // Placeholder for real expansion
            initial_ast
        } else {
            initial_ast
        };

        // --- Module Import Resolution ---
        // Merge imported module definitions into the AST
        let mut module_loader = module_loader::ModuleLoader::new("aloha-shirts");
        module_loader.merge_imports(&mut program_ast)?;

        // --- Analysis Passes ---
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze_program(&program_ast)?;

        // Type checking with inference
        let mut type_checker = TypeChecker::new();
        type_checker.check_program(&program_ast.statements)?;

        // Re-enabled temporarily for debugging
        let mut borrow_checker = BorrowChecker::new();
        borrow_checker.check_program(&program_ast)?;

        // --- Code Generation ---
        // FIX: Pass the target down to the CodeGenerator.
        let mut code_generator = CodeGenerator::new(target);
        let mut wasm_bytes = code_generator.generate_program(&program_ast)?;

        // --- Optimization ---
        if self.optimize {
            let mut optimizer = WasmOptimizer::new();
            wasm_bytes = optimizer.optimize(wasm_bytes);

            // Print optimization statistics
            let stats = optimizer.stats();
            if stats.total_optimizations() > 0 {
                println!("   - Optimizations applied: {} total", stats.total_optimizations());
                if stats.functions_removed > 0 {
                    println!("     • Dead functions removed: {}", stats.functions_removed);
                }
                if stats.constants_folded > 0 {
                    println!("     • Constants folded: {}", stats.constants_folded);
                }
                if stats.functions_inlined > 0 {
                    println!("     • Functions inlined: {}", stats.functions_inlined);
                }
                println!("     • Size reduction: {:.1}%", stats.size_reduction_percent());
            }
        }

        Ok(wasm_bytes)
    }

    /// Compile source code and return both WASM bytes and CSS output (Phase 7.5)
    pub fn compile_source_with_css(&self, source: &str, target: BuildTarget) -> Result<(Vec<u8>, String), CompileError> {
        println!("   - Starting compilation for target: {:?}", target);

        // --- Lexing, Parsing, Macro Expansion ---
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(&mut lexer);
        let initial_ast = parser.parse_program()?;

        // This is a simplified macro expansion for now.
        let mut needs_reparse = false;
        for statement in &initial_ast.statements {
            if let ast::Statement::MacroInvocation(_) = statement {
                needs_reparse = true;
                break;
            }
        }

        // FIX: Rename the AST variable to avoid shadowing the `ast` module.
        let mut program_ast = if needs_reparse {
            // Placeholder for real expansion
            initial_ast
        } else {
            initial_ast
        };

        // --- Module Import Resolution ---
        // Merge imported module definitions into the AST
        let mut module_loader = module_loader::ModuleLoader::new("aloha-shirts");
        module_loader.merge_imports(&mut program_ast)?;

        // --- Analysis Passes ---
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze_program(&program_ast)?;

        // Type checking with inference
        let mut type_checker = TypeChecker::new();
        type_checker.check_program(&program_ast.statements)?;

        // Re-enabled temporarily for debugging
        let mut borrow_checker = BorrowChecker::new();
        borrow_checker.check_program(&program_ast)?;

        // --- Code Generation ---
        // FIX: Pass the target down to the CodeGenerator.
        let mut code_generator = CodeGenerator::new(target);
        let mut wasm_bytes = code_generator.generate_program(&program_ast)?;

        // Extract CSS output (Phase 7.5)
        let css_output = code_generator.get_css_output().to_string();

        // --- Optimization ---
        if self.optimize {
            let mut optimizer = WasmOptimizer::new();
            wasm_bytes = optimizer.optimize(wasm_bytes);

            // Print optimization statistics
            let stats = optimizer.stats();
            if stats.total_optimizations() > 0 {
                println!("   - Optimizations applied: {} total", stats.total_optimizations());
                if stats.functions_removed > 0 {
                    println!("     • Dead functions removed: {}", stats.functions_removed);
                }
                if stats.constants_folded > 0 {
                    println!("     • Constants folded: {}", stats.constants_folded);
                }
                if stats.functions_inlined > 0 {
                    println!("     • Functions inlined: {}", stats.functions_inlined);
                }
                println!("     • Size reduction: {:.1}%", stats.size_reduction_percent());
            }
        }

        Ok((wasm_bytes, css_output))
    }

    /// Display a compilation error with beautiful diagnostics
    pub fn display_error(error: &CompileError, source: Option<&str>, filename: &str) -> String {
        

        let diagnostic = error.to_diagnostic(filename);
        diagnostic.display(source)
    }
}

pub trait LexerExt {
    fn collect_tokens(&mut self) -> Result<Vec<Token>, CompileError>;
}

impl LexerExt for Lexer {
    fn collect_tokens(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let kind = token.kind.clone();
            if let TokenKind::Illegal(ch) = kind {
                return Err(CompileError::LexerError(format!("Illegal character: '{}'", ch)));
            }
            tokens.push(token);
            if kind == TokenKind::Eof {
                break;
            }
        }
        Ok(tokens)
    }
}

// Integration tests - tests that compile full programs end-to-end
#[cfg(test)]
mod integration_tests;