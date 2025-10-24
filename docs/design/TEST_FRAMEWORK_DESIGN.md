# Jounce Test Framework Design

**Phase 9 Sprint 2 - Developer Tools Enhancement**

## Overview

The Jounce test framework provides built-in unit testing and integration testing capabilities with a clean, familiar syntax similar to Rust and Jest.

## Goals

1. **Simple Syntax** - Easy to write and read tests
2. **Good Error Messages** - Clear failure messages with context
3. **Fast Execution** - Efficient test runner
4. **Integrated** - Works seamlessly with `jnc` CLI
5. **Rich Assertions** - Comprehensive assertion library

## Test Syntax

### Basic Test Structure

```jounce
// Attribute-based tests (Rust-style)
#[test]
fn test_addition() {
    let result = add(2, 2);
    assert_eq(result, 4);
}

#[test]
fn test_subtraction() {
    assert_eq(subtract(5, 3), 2);
}

// Async tests
#[test]
async fn test_fetch_data() {
    let data = await fetch_user(1);
    assert(data.name == "Alice");
}

// Test with expected failure
#[test]
#[should_panic]
fn test_divide_by_zero() {
    divide(10, 0);  // Should panic
}
```

### Test Organization

```jounce
// Group related tests
#[test_mod]
mod math_tests {
    #[test]
    fn test_add() {
        assert_eq(add(1, 1), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq(multiply(3, 4), 12);
    }
}

// Nested test modules
#[test_mod]
mod calculator {
    #[test_mod]
    mod basic_ops {
        #[test]
        fn test_add() {
            assert_eq(2 + 2, 4);
        }
    }

    #[test_mod]
    mod advanced_ops {
        #[test]
        fn test_power() {
            assert_eq(pow(2, 3), 8);
        }
    }
}
```

### Setup and Teardown

```jounce
#[test_mod]
mod database_tests {
    // Run before each test
    #[before_each]
    fn setup() {
        init_test_db();
    }

    // Run after each test
    #[after_each]
    fn teardown() {
        cleanup_test_db();
    }

    // Run once before all tests
    #[before_all]
    fn setup_suite() {
        create_test_schema();
    }

    // Run once after all tests
    #[after_all]
    fn teardown_suite() {
        drop_test_schema();
    }

    #[test]
    fn test_insert() {
        // setup() runs before this
        let user = insert_user("Alice");
        assert(user.id > 0);
        // teardown() runs after this
    }
}
```

## Assertion API

### Basic Assertions

```jounce
// Boolean assertions
assert(condition);  // Panic if false
assert(x > 0);
assert(x > 0, "x must be positive");  // With custom message

// Equality assertions
assert_eq(actual, expected);
assert_eq(result, 42, "Expected 42");

assert_ne(actual, not_expected);
assert_ne(result, 0);

// Null/None checks
assert_some(option);  // Panic if None
assert_none(option);  // Panic if Some

// Error handling
assert_ok(result);   // Panic if Err
assert_err(result);  // Panic if Ok
```

### Advanced Assertions

```jounce
// Type assertions
assert_type<User>(value);

// Range assertions
assert_in_range(value, min, max);

// Collection assertions
assert_contains(array, value);
assert_length(array, expected_len);
assert_empty(collection);
assert_not_empty(collection);

// String assertions
assert_starts_with(string, prefix);
assert_ends_with(string, suffix);
assert_matches(string, regex);

// Floating point assertions (with epsilon)
assert_approx(actual, expected, epsilon);
assert_approx(3.14159, 3.14, 0.01);
```

### Custom Matchers

```jounce
// Define custom assertion helpers
fn assert_valid_email(email: string) {
    let regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    assert(regex.test(email), `"${email}" is not a valid email`);
}

#[test]
fn test_user_email() {
    let user = create_user("alice@example.com");
    assert_valid_email(user.email);
}
```

## Test Runner

### CLI Commands

```bash
# Run all tests
jnc test

# Run specific test file
jnc test tests/math.jnc

# Run tests matching pattern
jnc test --filter "calculator"

# Run in watch mode
jnc test --watch

# Show detailed output
jnc test --verbose

# Run tests in parallel (default)
jnc test

# Run tests sequentially
jnc test --single-threaded

# Generate coverage report
jnc test --coverage
```

### Test Output

```
Running 15 tests...

✓ math::test_addition (0.1ms)
✓ math::test_subtraction (0.1ms)
✓ math::test_multiplication (0.1ms)
✗ math::test_division (0.2ms)

  Assertion failed: assert_eq(divide(10, 3), 3)
  Expected: 3
  Actual: 3.3333333333333335

  --> tests/math.jnc:25:5
   25 |     assert_eq(divide(10, 3), 3);
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ assertion failed

✓ calculator::basic_ops::test_add (0.1ms)
...

Test Summary:
  Passed: 14
  Failed: 1
  Total: 15
  Duration: 45ms

Failed Tests:
  - math::test_division
```

## Implementation

### AST Nodes

```rust
// In src/ast.rs
pub enum Attribute {
    Test,
    TestMod,
    BeforeEach,
    AfterEach,
    BeforeAll,
    AfterAll,
    ShouldPanic,
    Ignore,
}

pub struct TestFunction {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub is_async: bool,
    pub body: Vec<Statement>,
}

pub struct TestModule {
    pub name: String,
    pub tests: Vec<TestFunction>,
    pub setup: Option<TestFunction>,
    pub teardown: Option<TestFunction>,
    pub before_all: Option<TestFunction>,
    pub after_all: Option<TestFunction>,
    pub submodules: Vec<TestModule>,
}
```

### Test Discovery

1. Parse `.jnc` files looking for `#[test]` attributes
2. Collect all test functions into a test registry
3. Build test hierarchy (modules, tests, hooks)
4. Generate test runner code

### Test Execution

1. Compile test code to JavaScript
2. Generate test runner harness
3. Execute tests with Node.js runtime
4. Collect results and format output
5. Report pass/fail statistics

### Assertion Implementation

Assertions are built-in functions that:
1. Evaluate the condition
2. If true, return normally
3. If false, panic with detailed error message including:
   - Expected vs actual values
   - Source location
   - Custom message if provided

## Integration

### File Structure

```
project/
├── src/
│   └── lib.jnc
├── tests/
│   ├── integration_tests.jnc
│   └── unit_tests.jnc
└── jounce.toml
```

### Configuration

```toml
[test]
# Test configuration in jounce.toml
parallel = true
timeout = 5000  # ms
coverage = false

[[test.filter]]
include = ["tests/**/*.jnc"]
exclude = ["tests/fixtures/**"]
```

## Future Enhancements

- **Snapshot Testing** - Jest-style snapshot tests
- **Property-Based Testing** - QuickCheck-style tests
- **Mocking** - Built-in mocking framework
- **Code Coverage** - Coverage reporting
- **Benchmarking** - Performance benchmarks
- **Visual Testing** - Screenshot comparison for UI tests

## Examples

See `examples/testing/` for complete examples:
- `basic_tests.jnc` - Simple unit tests
- `async_tests.jnc` - Async test examples
- `integration_tests.jnc` - Integration testing
- `mocking_example.jnc` - Mocking and stubs
