// Cached compilation methods
// These methods leverage the compilation cache for faster incremental builds

use std::path::{Path, PathBuf};
use std::sync::Arc;
use rayon::prelude::*;

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

/// Compile multiple files in parallel using cached compilation
/// Uses dependency graph to determine which files can be compiled in parallel
pub fn compile_project_parallel(
    files: Vec<(PathBuf, String)>, // Vec of (file_path, source_code)
    target: BuildTarget,
    cache: &Arc<CompilationCache>,
    optimize: bool,
) -> Result<Vec<(PathBuf, Vec<u8>, String)>, CompileError> {
    println!("   - Starting parallel compilation for {} files", files.len());

    // Build dependency graph by analyzing imports
    // For now, we'll compile files independently (no dependencies)
    // TODO: Implement proper dependency analysis from import statements

    // Get dependency levels for parallel compilation
    let dependency_levels = {
        let deps = cache.dependencies();
        deps.lock().unwrap().topological_levels()
    };

    if dependency_levels.is_empty() {
        // No dependencies registered, compile all files in parallel
        println!("   - No dependencies detected, compiling all files in parallel");

        let results: Vec<Result<(PathBuf, Vec<u8>, String), CompileError>> = files
            .par_iter()
            .map(|(path, source)| {
                compile_source_cached(source, path, target, cache, optimize)
                    .map(|(wasm, css)| (path.clone(), wasm, css))
            })
            .collect();

        // Collect results and check for errors
        let mut compiled = Vec::new();
        for result in results {
            compiled.push(result?);
        }

        Ok(compiled)
    } else {
        // Compile level by level (respecting dependencies)
        println!("   - Compiling in {} levels (respecting dependencies)", dependency_levels.len());

        let mut compiled = Vec::new();

        for (level_idx, level_files) in dependency_levels.iter().enumerate() {
            println!("   - Level {}: {} files", level_idx + 1, level_files.len());

            // Find source for files in this level
            let level_sources: Vec<_> = level_files
                .iter()
                .filter_map(|path| {
                    files.iter()
                        .find(|(p, _)| p == path)
                        .map(|(p, s)| (p.clone(), s.clone()))
                })
                .collect();

            // Compile this level in parallel
            let results: Vec<Result<(PathBuf, Vec<u8>, String), CompileError>> = level_sources
                .par_iter()
                .map(|(path, source)| {
                    compile_source_cached(source, path, target, cache, optimize)
                        .map(|(wasm, css)| (path.clone(), wasm, css))
                })
                .collect();

            // Collect results
            for result in results {
                compiled.push(result?);
            }
        }

        Ok(compiled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_compilation_simple() {
        let cache = Arc::new(CompilationCache::default());

        let files = vec![
            (
                PathBuf::from("test1.jnc"),
                "fn main() { let x = 42; }".to_string(),
            ),
            (
                PathBuf::from("test2.jnc"),
                "fn main() { let y = 10; }".to_string(),
            ),
            (
                PathBuf::from("test3.jnc"),
                "fn main() { let z = 5; }".to_string(),
            ),
        ];

        let result = compile_project_parallel(files, BuildTarget::Client, &cache, false);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok());

        let compiled = result.unwrap();
        assert_eq!(compiled.len(), 3);

        // Verify cache statistics
        let stats = cache.stats();
        assert_eq!(stats.misses, 3); // All files should be cache misses on first compile
        assert_eq!(stats.hits, 0);
    }

    #[test]
    fn test_parallel_compilation_with_cache() {
        let cache = Arc::new(CompilationCache::default());

        let files = vec![
            (
                PathBuf::from("cached1.jnc"),
                "fn main() { let a = 10; let b = 20; }".to_string(),
            ),
            (
                PathBuf::from("cached2.jnc"),
                "fn main() { let x = 5; let y = 15; }".to_string(),
            ),
        ];

        // First compilation (cold cache)
        let result1 = compile_project_parallel(files.clone(), BuildTarget::Client, &cache, false);
        if let Err(ref e) = result1 {
            eprintln!("First compilation error: {:?}", e);
        }
        assert!(result1.is_ok());

        let stats1 = cache.stats();
        eprintln!("After first compilation: misses={}, hits={}", stats1.misses, stats1.hits);
        assert_eq!(stats1.misses, 2);
        assert_eq!(stats1.hits, 0);

        // Second compilation (warm cache)
        let result2 = compile_project_parallel(files, BuildTarget::Client, &cache, false);
        assert!(result2.is_ok());

        let stats2 = cache.stats();
        eprintln!("After second compilation: misses={}, hits={}", stats2.misses, stats2.hits);
        assert_eq!(stats2.misses, 2); // Same as before
        assert_eq!(stats2.hits, 2);   // Both should hit cache
    }
}
