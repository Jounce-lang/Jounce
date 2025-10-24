# Jounce Examples

A comprehensive collection of examples that progress from absolute basics to advanced full-stack applications.

## 📚 Organization

Examples are organized by difficulty and concept, with each category building on the previous:

### 01-basics/ - Absolute Fundamentals
**Difficulty**: Beginner | **Lines**: 5-20 per example | **Time**: 30-60 mins total

Start here if you're new to Jounce! These examples introduce one concept at a time.

1. **01_hello_world.jnc** - Your first program: println! macro
2. **02_variables_let.jnc** - Variable bindings with let
3. **03_variables_mut.jnc** - Mutable variables with mut
4. **04_integers.jnc** - Integer arithmetic (+, -, *, /)
5. **05_strings.jnc** - String literals and formatting
6. **06_booleans.jnc** - Boolean values and comparisons
7. **07_simple_function.jnc** - Functions with parameters
8. **08_function_return.jnc** - Explicit return statements
9. **09_implicit_return.jnc** - Implicit returns (Rust-style)
10. **10_comments.jnc** - Comment syntax and best practices

### 02-control-flow/ - Making Decisions
**Difficulty**: Beginner-Intermediate | **Lines**: 20-40 per example | **Time**: 1-2 hours

Learn how to control program flow with conditions and loops.

1. **01_simple_if.jnc** - Simple if without else
2. **02_if_else.jnc** - If/else with both branches
3. **03_if_else_expression.jnc** - If/else as an expression
4. **04_nested_if_2_levels.jnc** - Two levels of nested if/else
5. **05_nested_if_3_levels.jnc** - Three levels of nested if/else
6. **06_for_loop_exclusive.jnc** - For loop with exclusive range (1..10)
7. **07_for_loop_inclusive.jnc** - For loop with inclusive range (1..=10)
8. **08_while_loop.jnc** - While loop with counter
9. **09_match_simple.jnc** - Basic match expression
10. **10_arrays.jnc** - Array creation and indexing

### 03-functions/ - Advanced Functions
**Difficulty**: Intermediate | **Lines**: 30-50 per example | **Time**: 2-3 hours

Master function concepts including recursion and closures.

1. **01_factorial_recursion.jnc** - Classic factorial recursion
2. **02_fibonacci_recursion.jnc** - Fibonacci sequence with recursion
3. **03_mutual_recursion.jnc** - Even/odd with mutual recursion
4. **04_higher_order_map.jnc** - Map pattern (transforming arrays)
5. **05_higher_order_filter.jnc** - Filter pattern (predicates)
6. **06_closures_basic.jnc** - Basic closures and variable capture
7. **07_closures_typed.jnc** - Closures with type annotations
8. **08_function_composition.jnc** - Composing and chaining functions

### 04-patterns/ - Error Handling
**Difficulty**: Intermediate | **Lines**: 40-60 per example | **Time**: 2-3 hours

Learn Jounce's powerful pattern matching and error handling.

1. **01_option_basic.jnc** - Basic Option<T> with Some/None
2. **02_option_pattern_match.jnc** - Pattern matching on Option
3. **03_result_basic.jnc** - Result<T,E> for error handling
4. **04_result_pattern_match.jnc** - Pattern matching with Result
5. **05_try_operator.jnc** - Try operator (?) for error propagation
6. **06_nested_patterns.jnc** - Nested Option<Result<T,E>> patterns
7. **07_combining_results.jnc** - Combining multiple Result operations
8. **08_real_world_errors.jnc** - Real-world error handling pipeline

### 05-advanced-types/ - Type System ✅
**Difficulty**: Advanced | **Lines**: 50-80 per example | **Time**: 2-3 hours

Explore Jounce's advanced type system features.

1. **01_generic_functions.jnc** - Generic functions with type parameters
2. **02_generic_structs.jnc** - Generic structs (Box, Pair, Container)
3. **03_sized_arrays.jnc** - Sized arrays [T; N] syntax
4. **04_generic_algorithms.jnc** - Generic algorithms (map, reduce, find)
5. **05_generics_advanced.jnc** - Advanced generics with higher-order functions
6. **06_real_world_generics.jnc** - Real-world leaderboard system

### 06-async/ - Asynchronous Programming ✅
**Difficulty**: Advanced | **Lines**: 60-150 per example | **Time**: 2-3 hours

Learn async/await and concurrent programming patterns.

1. **01_async_basic.jnc** - Async/await fundamentals (basic syntax)
2. **02_async_functions.jnc** - Async functions with return values
3. **03_concurrent_operations.jnc** - Sequential vs concurrent patterns
4. **04_async_error_handling.jnc** - Async with Result<T,E> and Option<T>
5. **05_async_loops.jnc** - Using async operations in loops
6. **06_real_world_async.jnc** - Complete async data pipeline

### 07-fullstack/ - Full-Stack Features
**Difficulty**: Advanced | **Lines**: 100-200 per example

Discover Jounce's unique full-stack capabilities.

- @server and @client annotations
- Automatic RPC generation
- JSX components
- State management

*(Coming in Sprint 7)*

### 08-apps/ - Real-World Applications
**Difficulty**: Advanced | **Lines**: 200-400 per example

Complete applications demonstrating best practices.

- Todo App (full-stack)
- Blog Engine
- E-commerce Cart
- User Authentication

*(Coming in Sprint 8)*

## 🚀 How to Run Examples

### Quick Start (Any Example)

```bash
# Compile an example
raven compile examples/01-basics/01_hello_world.jnc

# Run the compiled output
cd dist && node server.js
```

### Run All Examples in a Category

```bash
# Test all basics examples
for file in examples/01-basics/*.jnc; do
    echo "Running: $file"
    raven compile "$file"
    cd dist && node server.js
    cd ..
done
```

### Watch Mode (Auto-recompile)

```bash
# Auto-recompile on file changes
raven watch examples/01-basics/01_hello_world.jnc
```

## 📖 Learning Path

### Complete Beginner (Never programmed before)
1. Start with **01-basics** (examples 01-10)
2. Move to **02-control-flow** (if/else, loops)
3. Practice **03-functions** (recursion, closures)

### Experienced Developer (Learning Jounce)
1. Skim **01-basics** for syntax differences
2. Focus on **04-patterns** (Option, Result, match)
3. Explore **05-advanced-types** (generics, traits)
4. Try **07-fullstack** (unique Jounce features)

### LLM Training (AI Learning Jounce)
1. Process all examples in order (01-basics → 08-apps)
2. Each example demonstrates one concept clearly
3. Comments explain WHY not just WHAT
4. Expected output is provided for verification

## 🎯 Example Template

Each example follows a consistent structure:

```raven
// examples/category/example_name.jnc
//
// CONCEPT: What this example teaches
// DIFFICULTY: Beginner/Intermediate/Advanced
// FEATURES: Specific language features used
//
// Detailed explanation of the concept
// Multiple lines explaining WHY and HOW

fn main() {
    // Clear, commented code
}

// EXPECTED OUTPUT:
// What you should see when running this

// TRY IT:
// Command to compile and run

// BEST PRACTICES: (optional)
// Tips and recommendations
```

## 🧪 Testing Examples

All examples serve as compiler tests. To verify all examples compile:

```bash
# Test all examples compile
./scripts/test_examples.sh

# Or manually:
find examples -name "*.jnc" -exec raven compile {} \;
```

## 📊 Statistics

- **Total Examples**: 48 (Sprint 1 + Sprint 2 + Sprint 3 + Sprint 4 + Sprint 5 + Sprint 6 complete)
  - 01-basics: 10 examples ✅
  - 02-control-flow: 10 examples ✅
  - 03-functions: 8 examples ✅
  - 04-patterns: 8 examples ✅
  - 05-advanced-types: 6 examples ✅
  - 06-async: 6 examples ✅
- **Total Planned**: ~60 examples across 8 categories
- **Test Coverage**: 100% of examples compile successfully
- **Difficulty Levels**: 3 (Beginner, Intermediate, Advanced)
- **Progress**: 80% complete (48/60)

## 🤝 Contributing

Want to add examples? Follow these guidelines:

1. **One Concept**: Each example demonstrates exactly ONE concept
2. **Progressive**: Build on previous examples
3. **Commented**: Explain WHY, not just WHAT
4. **Tested**: Ensure it compiles and runs
5. **Template**: Follow the example template above

## 📚 Additional Resources

- [Getting Started Guide](../docs/GETTING_STARTED.md)
- [Language Reference](../CLAUDE.md)
- [API Documentation](../docs/guides/STDLIB_API_REFERENCE.md)
- [LSP Features](../docs/guides/LSP_FEATURES.md)

## 🎓 Learning Tips

1. **Type Out Examples**: Don't just read - type them yourself
2. **Experiment**: Modify examples and see what happens
3. **Expected Output**: Verify your output matches the expected
4. **Progression**: Complete categories in order
5. **Practice**: Write your own variations

---

**Sprint Status**: Phase 6 Sprint 6 Complete (48/48 examples total)
- Sprint 1 ✅ Complete: 01-basics (10 examples)
- Sprint 2 ✅ Complete: 02-control-flow (10 examples)
- Sprint 3 ✅ Complete: 03-functions (8 examples)
- Sprint 4 ✅ Complete: 04-patterns (8 examples)
- Sprint 5 ✅ Complete: 05-advanced-types (6 examples)
- Sprint 6 ✅ Complete: 06-async (6 examples)

**Next Sprint**: Sprint 7 - Full-Stack Features (5 examples: @server/@client, RPC, JSX, state)
**Last Updated**: 2025-10-22
