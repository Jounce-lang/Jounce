# jounce-testing

**Comprehensive testing utilities for Jounce applications**

A complete testing toolkit with assertions, mocking, spying, fixtures, and benchmarking for building robust test suites.

## Features

- **Extended Assertions** - 15+ assertion helpers beyond basic assert
- **Mocking** - Full mock objects with call tracking and verification
- **Spying** - Spy on function calls while preserving original behavior
- **Fixtures** - Reusable test data with setup/teardown hooks
- **Test Suites** - Organize tests with before/after hooks
- **Benchmarking** - Measure performance with detailed metrics
- **Stubs** - Simple test doubles for controlled responses
- **Test Runner** - Run multiple test suites with aggregated results

## Installation

```bash
jnc install jounce-testing
```

## Quick Start

```jounce
use jounce_testing::{
    assert_equals, assert_contains, assert_greater_than,
    Mock, Spy, Benchmark
};

#[test]
fn test_user_service() {
    // Extended assertions
    assert_equals(user.age, 25, "Age should be 25");
    assert_contains(user.email, "@example.com", "Should be valid email");
    assert_greater_than(user.score, 100, "Score should exceed 100");

    // Mocking
    let mut mock = Mock::new("UserAPI");
    mock = mock.when("getUser", "John Doe");

    let result = mock.call("getUser", vec!["123"]);
    assert_equals(result, "John Doe", "Should return mocked value");
    assert!(mock.was_called("getUser"), "Should track calls");
}
```

## API Reference

### Assertions

```jounce
// Equality
assert_equals(actual, expected, message)
assert_not_equals(actual, expected, message)

// Boolean
assert_true(value, message)
assert_false(value, message)

// Nullability
assert_null(value, message)
assert_not_null(value, message)

// String operations
assert_contains(haystack, needle, message)
assert_not_contains(haystack, needle, message)
assert_starts_with(text, prefix, message)
assert_ends_with(text, suffix, message)

// Numeric comparisons
assert_greater_than(actual, expected, message)
assert_less_than(actual, expected, message)
assert_in_range(value, min, max, message)

// Array operations
assert_array_length(array, expected_length, message)
assert_array_contains(array, item, message)
```

### Mocking

```jounce
use jounce_testing::Mock;

let mut mock = Mock::new("API");

// Set return values
mock = mock.when("getUser", "user_data");
mock = mock.when("deleteUser", "success");

// Call methods
let result = mock.call("getUser", vec!["123"]);

// Verify calls
assert!(mock.was_called("getUser"));
assert_eq!(mock.call_count("getUser"), 1);
assert!(mock.verify_called_once("getUser"));
assert!(mock.verify_called_with("getUser", vec!["123"]));

// Get call history
let calls = mock.get_calls("getUser");

// Reset
mock = mock.reset();
```

### Spying

```jounce
use jounce_testing::Spy;

let mut spy = Spy::new("logger");

// Track calls
spy.call(vec!["info", "message"]);
spy.call(vec!["warn", "warning"]);

// Verify
assert!(spy.was_called());
assert_eq!(spy.call_count(), 2);

// Reset
spy = spy.reset();
```

### Fixtures

```jounce
use jounce_testing::Fixture;

let fixture = Fixture::new("user", default_user())
    .with_setup(|| create_test_user())
    .with_teardown(|user| cleanup_user(user));

// Before each test
let user = fixture.before_each();

// After each test
fixture.after_each(user);
```

### Test Suites

```jounce
use jounce_testing::{TestSuite, TestCase};

let mut suite = TestSuite::new("Math Tests")
    .with_before_all(|| setup_environment())
    .with_after_all(|| cleanup_environment())
    .with_before_each(|| reset_state())
    .with_after_each(|| verify_invariants());

let test1 = TestCase::new("test_addition").pass(10);
let test2 = TestCase::new("test_subtraction").pass(15);

suite = suite.add_test(test1).add_test(test2);

let result = suite.run();
assert!(result.all_passed());
assert_eq!(result.success_rate(), 1.0);
```

### Benchmarking

```jounce
use jounce_testing::Benchmark;

let bench = Benchmark::new("sort_array", 1000)
    .run(|| sort_large_array());

println(bench.report());
// Output:
// Benchmark: sort_array
// Iterations: 1000
// Average: 5.2ms
// Min: 4.8ms
// Max: 6.1ms
// Ops/sec: 192.3

let avg_time = bench.average_time();
let ops_per_sec = bench.ops_per_second();
```

### Test Runner

```jounce
use jounce_testing::{TestRunner, TestSuite};

let mut runner = TestRunner::new()
    .with_verbose(true);

runner = runner
    .add_suite(math_suite)
    .add_suite(string_suite)
    .add_suite(api_suite);

let results = runner.run_all();

let total_passed = runner.total_passed();
let total_failed = runner.total_failed();
```

### Stubs

```jounce
use jounce_testing::Stub;

let stub = Stub::new("getConfig")
    .returns("test_config");

let result = stub.call();
assert_eq!(result, "test_config");
```

## Examples

### Complete Test Suite

```jounce
use jounce_testing::{
    assert_equals, assert_true,
    Mock, TestSuite, TestCase, Benchmark
};

fn run_user_tests() {
    let mut suite = TestSuite::new("User Tests");

    // Test 1: User creation
    let test1 = TestCase::new("test_create_user").pass(5);

    // Test 2: User validation
    let test2 = TestCase::new("test_validate_user").pass(3);

    suite = suite.add_test(test1).add_test(test2);

    let result = suite.run();
    assert!(result.all_passed());
}

fn benchmark_performance() {
    let bench = Benchmark::new("user_lookup", 10000)
        .run(|| find_user_by_id(12345));

    println(bench.report());
}
```

### Mocking External APIs

```jounce
use jounce_testing::Mock;

fn test_api_client() {
    let mut api = Mock::new("ExternalAPI");

    api = api
        .when("get", "{\"status\":\"success\"}")
        .when("post", "{\"id\":123}")
        .when("delete", "{\"deleted\":true}");

    // Test GET
    let get_result = api.call("get", vec!["/users"]);
    assert!(get_result.contains("success"));

    // Test POST
    let post_result = api.call("post", vec!["/users", "data"]);
    assert!(post_result.contains("123"));

    // Verify all calls
    assert_eq!(api.call_count("get"), 1);
    assert_eq!(api.call_count("post"), 1);
}
```

## Best Practices

1. **Use Descriptive Messages** - Make assertion failures clear
2. **Mock External Dependencies** - Isolate unit tests
3. **Benchmark Critical Paths** - Measure performance-sensitive code
4. **Organize with Suites** - Group related tests together
5. **Clean Up with Fixtures** - Use teardown to prevent test pollution
6. **Verify Mock Behavior** - Check both return values and call patterns

## Testing jounce-testing

Run the test suite:

```bash
jnc test
```

## License

MIT

## Version

0.1.0
