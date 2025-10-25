# Jounce Examples

**Comprehensive examples demonstrating the Jounce programming language**

---

## 📂 Directory Structure

```
examples/
├── README.md (you are here)
├── apps/ - 20-Application Progressive Series
│   └── 01-hello-counter/ - Simple JSX demonstration
├── tutorials/ - Beginner to Advanced Tutorials
│   ├── 01-basics/ - Variables, functions, comments (10 examples)
│   ├── 02-control-flow/ - If/else, loops, match (10 examples)
│   ├── 03-functions/ - Recursion, closures, HOF (8 examples)
│   ├── 04-patterns/ - Option, Result, error handling (8 examples)
│   ├── 05-advanced-types/ - Generics, traits (6 examples)
│   └── 06-async/ - Async/await, concurrency (6 examples)
├── features/ - Single-Feature Demonstrations
│   ├── reactivity/ - Signal, computed, effect demos
│   ├── forms/ - Form validation examples
│   ├── styling/ - Styled components
│   └── modules/ - Multi-file imports
├── projects/ - Larger Multi-Feature Projects
│   ├── task-dashboard/ - Full task management
│   ├── devboard/ - Developer dashboard
│   ├── taskflow/ - Workflow management
│   └── bluebird-backend/ - Backend API example
└── archived/ - Old examples and legacy apps
    ├── loose-jnc-files/ - Standalone .jnc files
    └── old-apps/ - Deprecated applications
```

---

## 🚀 Quick Start

### Running an App

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile an app
cargo run -- compile examples/apps/01-hello-counter/main.jnc

# Run the Node.js server
cd dist
node server.js

# Open http://localhost:3000
```

### HMR Development Server (Auto-Reload)

```bash
# From the Jounce root directory
cargo run -- compile examples/apps/01-hello-counter/main.jnc

# Start HMR server with live reload
node scripts/hmr-server.js

# Open http://localhost:3000
# Edit files and see changes instantly!
```

**See [DEV_SERVER_GUIDE.md](../DEV_SERVER_GUIDE.md) for complete server documentation.**

---

## 📚 Learning Path

### 1. Complete Beginner (New to Programming)
1. Start with `tutorials/01-basics/` - Learn variables, functions, comments
2. Move to `tutorials/02-control-flow/` - Learn if/else, loops
3. Practice `tutorials/03-functions/` - Learn recursion, closures

### 2. Experienced Developer (Learning Jounce)
1. Skim `tutorials/01-basics/` for syntax differences
2. Focus on `tutorials/04-patterns/` - Option, Result, match patterns
3. Explore `tutorials/05-advanced-types/` - Generics and traits
4. Try `apps/01-hello-counter/` - See JSX in action

### 3. Building Real Apps
1. Study `apps/01-hello-counter/` - Simple JSX demo
2. Review `features/reactivity/` - Signal-based state management
3. Explore `features/styling/` - Styled components
4. Build `projects/task-dashboard/` - Full-featured app

---

## 🎯 20 Applications Roadmap

Progressive complexity series designed to test all 35 Jounce packages:

| Tier | Apps | Complexity | Lines | Packages | Time |
|------|------|------------|-------|----------|------|
| **Tier 1** | 1-5 | Beginner | 50-150 | 0-2 | ~5h |
| **Tier 2** | 6-10 | Intermediate | 200-400 | 3-5 | ~12h |
| **Tier 3** | 11-15 | Advanced | 500-1000 | 6-10 | ~25h |
| **Tier 4** | 16-20 | Portfolio | 1000-2500 | 10+ | ~50h |

### Current Progress

**Apps Complete**: 1/20

- ✅ **App 1: Hello Counter** - Simple JSX demonstration

**Apps Planned**:
- App 2: Color Picker (jounce-theme)
- App 3: Markdown Previewer (jounce-markdown, jounce-sanitizer)
- App 4: Todo List (jounce-store, local storage)
- App 5: Weather Widget (jounce-http, external API)
- ...and 15 more!

**See [20_APPS_PLAN.md](../20_APPS_PLAN.md) for complete roadmap.**

---

## 📖 Tutorial Series

### 01-basics/ - Absolute Fundamentals
**Difficulty**: Beginner | **Time**: 30-60 mins

1. `01_hello_world.jnc` - Your first program
2. `02_variables_let.jnc` - Variable bindings
3. `03_variables_mut.jnc` - Mutable variables
4. `04_integers.jnc` - Integer arithmetic
5. `05_strings.jnc` - String literals
6. `06_booleans.jnc` - Boolean values
7. `07_simple_function.jnc` - Functions with parameters
8. `08_function_return.jnc` - Return statements
9. `09_implicit_return.jnc` - Implicit returns
10. `10_comments.jnc` - Comment syntax

### 02-control-flow/ - Making Decisions
**Difficulty**: Beginner-Intermediate | **Time**: 1-2 hours

1. `01_simple_if.jnc` - Simple if statements
2. `02_if_else.jnc` - If/else branches
3. `03_if_else_expression.jnc` - If/else as expression
4. `04_nested_if_2_levels.jnc` - Nested if/else
5. `05_nested_if_3_levels.jnc` - Three-level nesting
6. `06_for_loop_exclusive.jnc` - For loops (exclusive range)
7. `07_for_loop_inclusive.jnc` - For loops (inclusive range)
8. `08_while_loop.jnc` - While loops
9. `09_match_simple.jnc` - Match expressions
10. `10_arrays.jnc` - Array creation and indexing

### 03-functions/ - Advanced Functions
**Difficulty**: Intermediate | **Time**: 2-3 hours

1. `01_factorial_recursion.jnc` - Factorial recursion
2. `02_fibonacci_recursion.jnc` - Fibonacci sequence
3. `03_mutual_recursion.jnc` - Even/odd mutual recursion
4. `04_higher_order_map.jnc` - Map pattern
5. `05_higher_order_filter.jnc` - Filter pattern
6. `06_closures_basic.jnc` - Basic closures
7. `07_closures_typed.jnc` - Typed closures
8. `08_function_composition.jnc` - Function composition

### 04-patterns/ - Error Handling
**Difficulty**: Intermediate | **Time**: 2-3 hours

1. `01_option_basic.jnc` - Option<T> basics
2. `02_option_pattern_match.jnc` - Pattern matching on Option
3. `03_result_basic.jnc` - Result<T,E> error handling
4. `04_result_pattern_match.jnc` - Pattern matching with Result
5. `05_try_operator.jnc` - Try operator (?)
6. `06_nested_patterns.jnc` - Nested Option<Result<T,E>>
7. `07_combining_results.jnc` - Combining Results
8. `08_real_world_errors.jnc` - Real-world error handling

### 05-advanced-types/ - Type System
**Difficulty**: Advanced | **Time**: 2-3 hours

1. `01_generic_functions.jnc` - Generic functions
2. `02_generic_structs.jnc` - Generic structs
3. `03_sized_arrays.jnc` - Sized arrays [T; N]
4. `04_generic_algorithms.jnc` - Generic algorithms
5. `05_generics_advanced.jnc` - Advanced generics
6. `06_real_world_generics.jnc` - Real-world leaderboard

### 06-async/ - Asynchronous Programming
**Difficulty**: Advanced | **Time**: 2-3 hours

1. `01_async_basic.jnc` - Async/await fundamentals
2. `02_async_functions.jnc` - Async functions with returns
3. `03_concurrent_operations.jnc` - Sequential vs concurrent
4. `04_async_error_handling.jnc` - Async with Result/Option
5. `05_async_loops.jnc` - Async in loops
6. `06_real_world_async.jnc` - Complete async pipeline

---

## 🔧 Running Examples

### Compile a Single Example

```bash
# From Jounce root
cargo run -- compile examples/tutorials/01-basics/01_hello_world.jnc

# View output in dist/
```

### Run All Examples in a Category

```bash
# Test all basics examples
for file in examples/tutorials/01-basics/*.jnc; do
    echo "Compiling: $file"
    cargo run -- compile "$file"
done
```

### Watch Mode (Auto-recompile)

```bash
cargo run -- watch examples/apps/01-hello-counter/main.jnc
```

---

## 📊 Statistics

- **Total Examples**: 48+ tutorial examples
- **Tutorial Progress**: 6/6 categories complete
- **Apps Progress**: 1/20 apps complete
- **Test Coverage**: 100% of examples compile successfully
- **Package Coverage**: Testing 35 ecosystem packages

---

## 🎓 Learning Tips

1. **Type Out Examples**: Don't just read - type them yourself
2. **Experiment**: Modify examples and see what breaks
3. **Expected Output**: Verify your output matches expectations
4. **Progression**: Complete categories in order
5. **Practice**: Write your own variations

---

## 🤝 Contributing

Want to add examples? Follow these guidelines:

1. **One Concept**: Each example demonstrates exactly ONE concept
2. **Progressive**: Build on previous examples
3. **Commented**: Explain WHY, not just WHAT
4. **Tested**: Ensure it compiles and runs
5. **Template**: Follow existing example structure

---

## 📚 Additional Resources

- [Getting Started Guide](../docs/GETTING_STARTED.md)
- [Development Server Guide](../DEV_SERVER_GUIDE.md)
- [20 Apps Roadmap](../20_APPS_PLAN.md)
- [Language Reference](../CLAUDE.md)
- [API Documentation](../docs/guides/STDLIB_API_REFERENCE.md)
- [LSP Features](../docs/guides/LSP_FEATURES.md)

---

## 📁 Archived Content

The `archived/` directory contains:
- Legacy analytics dashboard demo
- Old standalone .jnc files
- Deprecated applications
- Historical deployment guides

These are kept for reference but are not actively maintained.

---

**Last Updated**: October 24, 2025
**Jounce Version**: v0.8.0
**Total Examples**: 48 tutorials + 1 app + 20+ feature demos
