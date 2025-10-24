// Test Framework for Jounce
// Provides test discovery, runner generation, and execution

use std::path::{Path, PathBuf};
use std::fs;
use crate::ast::Program;
use crate::errors::CompileError;
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Represents a single test function
#[derive(Debug, Clone)]
pub struct TestFunction {
    pub name: String,
    pub file_path: PathBuf,
    pub line: usize,
    pub is_async: bool,
}

/// Represents a test suite (collection of tests)
#[derive(Debug)]
pub struct TestSuite {
    pub tests: Vec<TestFunction>,
    pub total_files: usize,
}

/// Test discovery - finds test functions in source files
pub struct TestDiscovery {
    test_pattern: String,
}

impl TestDiscovery {
    /// Create a new test discovery with default pattern (test_*)
    pub fn new() -> Self {
        TestDiscovery {
            test_pattern: "test_".to_string(),
        }
    }

    /// Discover all tests in a directory
    pub fn discover_tests(&self, dir: &Path) -> Result<TestSuite, std::io::Error> {
        let mut tests = Vec::new();
        let mut total_files = 0;

        self.discover_in_directory(dir, &mut tests, &mut total_files)?;

        Ok(TestSuite {
            tests,
            total_files,
        })
    }

    /// Recursively discover tests in directory
    fn discover_in_directory(
        &self,
        dir: &Path,
        tests: &mut Vec<TestFunction>,
        total_files: &mut usize,
    ) -> Result<(), std::io::Error> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.discover_in_directory(&path, tests, total_files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("jnc") {
                *total_files += 1;
                if let Ok(file_tests) = self.discover_in_file(&path) {
                    tests.extend(file_tests);
                }
            }
        }

        Ok(())
    }

    /// Discover tests in a single file
    fn discover_in_file(&self, file_path: &Path) -> Result<Vec<TestFunction>, CompileError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| CompileError::Generic(format!("Failed to read file: {}", e)))?;

        let mut lexer = Lexer::new(content.clone());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program()?;

        let mut tests = Vec::new();

        // Find all functions that start with "test_"
        for statement in &program.statements {
            if let crate::ast::Statement::Function(func) = statement {
                if func.name.value.starts_with(&self.test_pattern) {
                    tests.push(TestFunction {
                        name: func.name.value.clone(),
                        file_path: file_path.to_path_buf(),
                        line: 0, // TODO: Get actual line number from token
                        is_async: func.is_async,
                    });
                }
            }
        }

        Ok(tests)
    }
}

impl Default for TestDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Test runner - generates and executes tests
pub struct TestRunner {
    pub suite: TestSuite,
    pub verbose: bool,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new(suite: TestSuite) -> Self {
        TestRunner {
            suite,
            verbose: false,
        }
    }

    /// Generate test runner code (JavaScript)
    pub fn generate_runner_code_js(&self) -> String {
        let mut code = String::new();

        code.push_str("// Auto-generated test runner\n\n");
        code.push_str("(async () => {\n"); // Wrap in async IIFE for top-level await
        code.push_str("let passed = 0;\n");
        code.push_str("let failed = 0;\n\n");

        for test in &self.suite.tests {
            let test_name = &test.name;
            code.push_str(&format!("// Running test: {}\n", test_name));

            if test.is_async {
                // Wrap async tests in an async IIFE
                code.push_str("await (async () => {\n");
                code.push_str("    const start_time = Date.now();\n");
                code.push_str("    let result = 'passed';\n");
                code.push_str("    try {\n");
                code.push_str(&format!("        await {}();\n", test_name));
                code.push_str("    } catch (error) {\n");
                code.push_str("        result = error.message;\n");
                code.push_str("    }\n");
                code.push_str("    const duration = Date.now() - start_time;\n\n");
                code.push_str("    if (result === 'passed') {\n");
                code.push_str("        passed++;\n");
                code.push_str(&format!("        console.log(`  ✓ {} (${{duration}}ms)`);\n", test_name));
                code.push_str("    } else {\n");
                code.push_str("        failed++;\n");
                code.push_str(&format!("        console.log(`  ✗ {} (${{duration}}ms)`);\n", test_name));
                code.push_str("        console.log(`    Error: ${result}`);\n");
                code.push_str("    }\n");
                code.push_str("})();\n\n");
            } else {
                // Regular sync tests
                code.push_str("{\n");
                code.push_str("    const start_time = Date.now();\n");
                code.push_str("    let result = 'passed';\n");
                code.push_str("    try {\n");
                code.push_str(&format!("        {}();\n", test_name));
                code.push_str("    } catch (error) {\n");
                code.push_str("        result = error.message;\n");
                code.push_str("    }\n");
                code.push_str("    const duration = Date.now() - start_time;\n\n");
                code.push_str("    if (result === 'passed') {\n");
                code.push_str("        passed++;\n");
                code.push_str(&format!("        console.log(`  ✓ {} (${{duration}}ms)`);\n", test_name));
                code.push_str("    } else {\n");
                code.push_str("        failed++;\n");
                code.push_str(&format!("        console.log(`  ✗ {} (${{duration}}ms)`);\n", test_name));
                code.push_str("        console.log(`    Error: ${result}`);\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            }
        }

        code.push_str("console.log('');\n");
        code.push_str("console.log('Test Results:');\n");
        code.push_str("console.log(`  Passed: ${passed}`);\n");
        code.push_str("console.log(`  Failed: ${failed}`);\n");
        code.push_str("console.log(`  Total: ${passed + failed}`);\n\n");

        code.push_str("if (failed > 0) {\n");
        code.push_str("    process.exit(1);\n");
        code.push_str("}\n");

        code.push_str("})(); // End async IIFE\n"); // Close the async IIFE

        code
    }

    /// Generate test runner code (for backwards compatibility)
    pub fn generate_runner_code(&self) -> String {
        self.generate_runner_code_js()
    }

    /// Print test summary
    pub fn print_summary(&self) {
        println!("\n📋 Test Discovery Summary");
        println!("  Files scanned: {}", self.suite.total_files);
        println!("  Tests found: {}", self.suite.tests.len());

        if self.verbose {
            println!("\nDiscovered tests:");
            for test in &self.suite.tests {
                let async_marker = if test.is_async { " (async)" } else { "" };
                println!("  • {}{}", test.name, async_marker);
            }
        }
    }
}

/// Built-in assertion functions (JavaScript)
/// Note: Simplified version using only currently supported features
pub fn generate_assertion_library() -> String {
    r#"
// Jounce Test Assertions (JavaScript)
// Built-in assertion functions for testing

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || "Assertion failed");
    }
}

function assert_eq(actual, expected, message) {
    if (actual !== expected) {
        throw new Error(message || "Assertion failed: values not equal");
    }
}

function assert_ne(actual, expected, message) {
    if (actual === expected) {
        throw new Error(message || "Assertion failed: values should be different");
    }
}

function assert_true(condition, message) {
    if (condition !== true) {
        throw new Error(message || "Expected true");
    }
}

function assert_false(condition, message) {
    if (condition !== false) {
        throw new Error(message || "Expected false");
    }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_creation() {
        let discovery = TestDiscovery::new();
        assert_eq!(discovery.test_pattern, "test_");
    }

    #[test]
    fn test_assertion_library_generation() {
        let lib = generate_assertion_library();
        assert!(lib.contains("fn assert("));
        assert!(lib.contains("fn assert_eq"));
        assert!(lib.contains("fn assert_contains"));
    }
}
