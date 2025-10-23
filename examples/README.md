# RavensOne Examples

A comprehensive collection of examples that progress from absolute basics to advanced full-stack applications.

## 📚 Organization

Examples are organized by difficulty and concept, with each category building on the previous:

### 01-basics/ - Absolute Fundamentals
**Difficulty**: Beginner | **Lines**: 5-20 per example | **Time**: 30-60 mins total

Start here if you're new to RavensOne! These examples introduce one concept at a time.

1. **01_hello_world.raven** - Your first program: println! macro
2. **02_variables_let.raven** - Variable bindings with let
3. **03_variables_mut.raven** - Mutable variables with mut
4. **04_integers.raven** - Integer arithmetic (+, -, *, /)
5. **05_strings.raven** - String literals and formatting
6. **06_booleans.raven** - Boolean values and comparisons
7. **07_simple_function.raven** - Functions with parameters
8. **08_function_return.raven** - Explicit return statements
9. **09_implicit_return.raven** - Implicit returns (Rust-style)
10. **10_comments.raven** - Comment syntax and best practices

### 02-control-flow/ - Making Decisions
**Difficulty**: Beginner-Intermediate | **Lines**: 20-40 per example | **Time**: 1-2 hours

Learn how to control program flow with conditions and loops.

1. **01_simple_if.raven** - Simple if without else
2. **02_if_else.raven** - If/else with both branches
3. **03_if_else_expression.raven** - If/else as an expression
4. **04_nested_if_2_levels.raven** - Two levels of nested if/else
5. **05_nested_if_3_levels.raven** - Three levels of nested if/else
6. **06_for_loop_exclusive.raven** - For loop with exclusive range (1..10)
7. **07_for_loop_inclusive.raven** - For loop with inclusive range (1..=10)
8. **08_while_loop.raven** - While loop with counter
9. **09_match_simple.raven** - Basic match expression
10. **10_arrays.raven** - Array creation and indexing

### 03-functions/ - Advanced Functions
**Difficulty**: Intermediate | **Lines**: 30-50 per example | **Time**: 2-3 hours

Master function concepts including recursion and closures.

1. **01_factorial_recursion.raven** - Classic factorial recursion
2. **02_fibonacci_recursion.raven** - Fibonacci sequence with recursion
3. **03_mutual_recursion.raven** - Even/odd with mutual recursion
4. **04_higher_order_map.raven** - Map pattern (transforming arrays)
5. **05_higher_order_filter.raven** - Filter pattern (predicates)
6. **06_closures_basic.raven** - Basic closures and variable capture
7. **07_closures_typed.raven** - Closures with type annotations
8. **08_function_composition.raven** - Composing and chaining functions

### 04-patterns/ - Error Handling
**Difficulty**: Intermediate | **Lines**: 40-60 per example | **Time**: 2-3 hours

Learn RavensOne's powerful pattern matching and error handling.

1. **01_option_basic.raven** - Basic Option<T> with Some/None
2. **02_option_pattern_match.raven** - Pattern matching on Option
3. **03_result_basic.raven** - Result<T,E> for error handling
4. **04_result_pattern_match.raven** - Pattern matching with Result
5. **05_try_operator.raven** - Try operator (?) for error propagation
6. **06_nested_patterns.raven** - Nested Option<Result<T,E>> patterns
7. **07_combining_results.raven** - Combining multiple Result operations
8. **08_real_world_errors.raven** - Real-world error handling pipeline

### 05-advanced-types/ - Type System ✅
**Difficulty**: Advanced | **Lines**: 50-80 per example | **Time**: 2-3 hours

Explore RavensOne's advanced type system features.

1. **01_generic_functions.raven** - Generic functions with type parameters
2. **02_generic_structs.raven** - Generic structs (Box, Pair, Container)
3. **03_sized_arrays.raven** - Sized arrays [T; N] syntax
4. **04_generic_algorithms.raven** - Generic algorithms (map, reduce, find)
5. **05_generics_advanced.raven** - Advanced generics with higher-order functions
6. **06_real_world_generics.raven** - Real-world leaderboard system

### 06-async/ - Asynchronous Programming
**Difficulty**: Advanced | **Lines**: 60-100 per example

Learn async/await and concurrent programming patterns.

- Async/await basics
- Concurrent operations
- Error handling with async
- Real-world async patterns

*(Coming in Sprint 6)*

### 07-fullstack/ - Full-Stack Features
**Difficulty**: Advanced | **Lines**: 100-200 per example

Discover RavensOne's unique full-stack capabilities.

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
raven compile examples/01-basics/01_hello_world.raven

# Run the compiled output
cd dist && node server.js
```

### Run All Examples in a Category

```bash
# Test all basics examples
for file in examples/01-basics/*.raven; do
    echo "Running: $file"
    raven compile "$file"
    cd dist && node server.js
    cd ..
done
```

### Watch Mode (Auto-recompile)

```bash
# Auto-recompile on file changes
raven watch examples/01-basics/01_hello_world.raven
```

## 📖 Learning Path

### Complete Beginner (Never programmed before)
1. Start with **01-basics** (examples 01-10)
2. Move to **02-control-flow** (if/else, loops)
3. Practice **03-functions** (recursion, closures)

### Experienced Developer (Learning RavensOne)
1. Skim **01-basics** for syntax differences
2. Focus on **04-patterns** (Option, Result, match)
3. Explore **05-advanced-types** (generics, traits)
4. Try **07-fullstack** (unique RavensOne features)

### LLM Training (AI Learning RavensOne)
1. Process all examples in order (01-basics → 08-apps)
2. Each example demonstrates one concept clearly
3. Comments explain WHY not just WHAT
4. Expected output is provided for verification

## 🎯 Example Template

Each example follows a consistent structure:

```raven
// examples/category/example_name.raven
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
find examples -name "*.raven" -exec raven compile {} \;
```

## 📊 Statistics

- **Total Examples**: 42 (Sprint 1 + Sprint 2 + Sprint 3 + Sprint 4 + Sprint 5 complete)
  - 01-basics: 10 examples ✅
  - 02-control-flow: 10 examples ✅
  - 03-functions: 8 examples ✅
  - 04-patterns: 8 examples ✅
  - 05-advanced-types: 6 examples ✅
- **Total Planned**: ~60 examples across 8 categories
- **Test Coverage**: 100% of examples compile successfully
- **Difficulty Levels**: 3 (Beginner, Intermediate, Advanced)
- **Progress**: 70% complete (42/60)

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

**Sprint Status**: Phase 6 Sprint 4 Complete (36/36 examples total)
- Sprint 1 ✅ Complete: 01-basics (10 examples)
- Sprint 2 ✅ Complete: 02-control-flow (10 examples)
- Sprint 3 ✅ Complete: 03-functions (8 examples)
- Sprint 4 ✅ Complete: 04-patterns (8 examples)

**Next Sprint**: Sprint 5 - Advanced Types (6 examples: generics, traits, trait bounds, sized arrays)
**Last Updated**: 2025-10-22
