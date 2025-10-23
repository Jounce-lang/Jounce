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
**Difficulty**: Beginner-Intermediate | **Lines**: 20-40 per example

Learn how to control program flow with conditions and loops.

- If/else expressions (1-level, 2-level, 3-level nesting)
- For loops with ranges (inclusive and exclusive)
- While loops
- Match expressions
- Pattern matching basics

*(Coming in Sprint 2)*

### 03-functions/ - Advanced Functions
**Difficulty**: Intermediate | **Lines**: 30-50 per example

Master function concepts including recursion and closures.

- Recursive functions (factorial, fibonacci)
- Higher-order functions
- Closures with type annotations
- Function composition

*(Coming in Sprint 3)*

### 04-patterns/ - Error Handling
**Difficulty**: Intermediate | **Lines**: 40-60 per example

Learn RavensOne's powerful pattern matching and error handling.

- Option<T> with Some/None
- Result<T, E> with Ok/Err
- Try operator (?) for error propagation
- Nested pattern matching

*(Coming in Sprint 4)*

### 05-advanced-types/ - Type System
**Difficulty**: Advanced | **Lines**: 50-80 per example

Explore RavensOne's advanced type system features.

- Generic functions with type parameters
- Traits and trait implementations
- Trait bounds
- Sized arrays [T; N]

*(Coming in Sprint 5)*

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

- **Total Examples**: 10 (Sprint 1 complete)
- **Total Planned**: ~60 examples across 8 categories
- **Test Coverage**: 100% of examples compile successfully
- **Difficulty Levels**: 3 (Beginner, Intermediate, Advanced)

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

**Sprint Status**: Phase 6 Sprint 1 Complete (10/10 examples)
**Next Sprint**: Sprint 2 - Control Flow & Collections (10 examples)
**Last Updated**: 2025-10-22
