// Cached compilation methods
// These methods leverage the compilation cache for faster incremental builds

use std::path::Path;
use std::sync::Arc;

use crate::borrow_checker::BorrowChecker;
use crate::cache::CompilationCache;
use crate::codegen::CodeGenerator;
use crate::errors::CompileError;
use crate::lexer::Lexer;
use crate::module_loader;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::type_checker::TypeChecker;
use crate::utility_config;
use crate::utility_generator;
use crate::wasm_optimizer::WasmOptimizer;
use crate::BuildTarget;

/// Compile with caching support
pub fn compile_source_cached(
    source: &str,
    file_path: &Path,
    target: BuildTarget,
    cache: &Arc<CompilationCache>,
    optimize: bool,
) -> Result<(Vec<u8>, String), CompileError> {
    println!("   - Starting cached compilation for: {:?}", file_path);

    // Try to get cached AST or parse new one
    let program_ast = cache.get_or_compile(file_path, source, |src| {
        // This closure is only called on cache miss
        let mut lexer = Lexer::new(src.to_string());
        let mut parser = Parser::new(&mut lexer);
        let initial_ast = parser.parse_program()?;

        // Check for macro expansion needs
        let mut needs_reparse = false;
        for statement in &initial_ast.statements {
            if let crate::ast::Statement::MacroInvocation(_) = statement {
                needs_reparse = true;
                break;
            }
        }

        let program_ast = if needs_reparse {
            initial_ast // Placeholder for real expansion
        } else {
            initial_ast
        };

        Ok(program_ast)
    })?;

    // Clone the AST for processing (needed because we need a mutable reference)
    let mut program_ast = program_ast;

    // Module import resolution
    let mut module_loader = module_loader::ModuleLoader::new("aloha-shirts");
    module_loader.merge_imports(&mut program_ast)?;

    // Analysis passes (these could be cached too in the future)
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(&program_ast)?;

    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&program_ast.statements)?;

    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program_ast)?;

    // Code generation
    let mut code_generator = CodeGenerator::new(target);
    let mut wasm_bytes = code_generator.generate_program(&program_ast)?;

    // Utility CSS generation
    let utility_config = utility_config::UtilityConfig::load();
    let mut utility_gen = utility_generator::UtilityGenerator::new(utility_config);
    utility_gen.scan_for_utilities(&program_ast);
    let utility_css = utility_gen.generate_css();

    // Extract CSS output
    let component_css = code_generator.get_css_output().to_string();

    // Combine CSS
    let css_output = if utility_css.is_empty() {
        component_css
    } else {
        format!("{}\n{}", utility_css, component_css)
    };

    // Optimization
    if optimize {
        let mut optimizer = WasmOptimizer::new();
        wasm_bytes = optimizer.optimize(wasm_bytes);

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

    // Print cache statistics
    let cache_stats = cache.stats();
    if cache_stats.hits + cache_stats.misses > 0 {
        println!("   - Cache stats: {} hits, {} misses ({:.1}% hit rate)",
            cache_stats.hits,
            cache_stats.misses,
            cache_stats.hit_rate() * 100.0
        );
    }

    Ok((wasm_bytes, css_output))
}
