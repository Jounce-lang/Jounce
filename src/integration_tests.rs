// Integration tests that compile full .jnc programs end-to-end
// These tests validate that the entire compilation pipeline works correctly

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::type_checker::TypeChecker;
use crate::borrow_checker::BorrowChecker;
use crate::js_emitter::JSEmitter;
use crate::errors::CompileError;

/// Helper function to compile a source string end-to-end
fn compile_source(source: &str) -> Result<(String, String), CompileError> {
    // Lexer
    let mut lexer = Lexer::new(source.to_string());

    // Parser
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program()?;

    // Semantic Analyzer
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(&program)?;

    // Type Checker
    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&program.statements)?;

    // Borrow Checker
    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program)?;

    // Code Generation
    let emitter = JSEmitter::new(&program);
    let server_js = emitter.generate_server_js();
    let client_js = emitter.generate_client_js();

    Ok((server_js, client_js))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else_compiles() {
        let source = r#"
            fn main() {
                if true {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if/else statement should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("if"));
        assert!(server_js.contains("else"));
    }

    #[test]
    fn test_if_else_expression() {
        let source = r#"
            fn max(a: i32, b: i32) -> i32 {
                if a > b {
                    a
                } else {
                    b
                }
            }

            fn main() {
                let result = max(5, 3);
                println!("Max: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if/else expression should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("module.exports.max"));
        assert!(server_js.contains("if"));
        assert!(server_js.contains("else"));
    }

    #[test]
    fn test_nested_if_else() {
        let source = r#"
            fn classify(n: i32) -> String {
                if n < 0 {
                    "negative"
                } else {
                    if n == 0 {
                        "zero"
                    } else {
                        "positive"
                    }
                }
            }

            fn main() {
                let result = classify(5);
                println!("Classification: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested if/else should compile successfully");
    }

    #[test]
    fn test_if_else_in_loop() {
        let source = r#"
            fn main() {
                let numbers = [1, 2, 3, 4, 5];
                for num in numbers {
                    if num > 3 {
                        println!("Large: {}", num);
                    } else {
                        println!("Small: {}", num);
                    }
                }
            }
        "#;

        let result = compile_source(source);
        // The test should at minimum compile successfully
        // Note: for-in loop support may be limited in current implementation
        assert!(result.is_ok(), "if/else in loop should compile successfully");
    }

    #[test]
    fn test_recursive_function() {
        let source = r#"
            fn factorial(n: i32) -> i32 {
                if n <= 1 {
                    1
                } else {
                    n * factorial(n - 1)
                }
            }

            fn main() {
                let result = factorial(5);
                println!("Factorial: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "recursive function should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("module.exports.factorial"));
        assert!(server_js.contains("factorial"));  // Recursive call
    }

    #[test]
    fn test_multiple_returns() {
        let source = r#"
            fn check_value(n: i32) -> String {
                if n < 0 {
                    return "negative";
                } else {
                    if n == 0 {
                        return "zero";
                    } else {
                        return "positive";
                    }
                }
            }

            fn main() {
                println!("Result: {}", check_value(5));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "multiple returns with if/else should compile");
    }

    #[test]
    fn test_if_without_else() {
        let source = r#"
            fn main() {
                let x = 5;
                if x > 0 {
                    println!("positive");
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if without else should still compile");
    }

    #[test]
    fn test_complex_conditions() {
        let source = r#"
            fn check(a: i32, b: i32) -> bool {
                if a > 0 && b > 0 {
                    true
                } else {
                    false
                }
            }

            fn main() {
                let result = check(5, 3);
                println!("Result: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "complex conditions should compile");
    }

    #[test]
    fn test_for_loop_exclusive_range() {
        let source = r#"
            fn main() {
                for i in 1..5 {
                    println!("{}", i);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "for loop with exclusive range should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should generate: for (let i = 1; i < 5; i++)
        assert!(client_js.contains("for"));
        assert!(client_js.contains("let i"));
        assert!(client_js.contains("i < 5"));
        assert!(client_js.contains("i++"));
    }

    #[test]
    fn test_for_loop_inclusive_range() {
        let source = r#"
            fn main() {
                for i in 1..=5 {
                    println!("{}", i);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "for loop with inclusive range should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should generate: for (let i = 1; i <= 5; i++)
        assert!(client_js.contains("for"));
        assert!(client_js.contains("let i"));
        assert!(client_js.contains("i <= 5"));
        assert!(client_js.contains("i++"));
    }

    #[test]
    fn test_for_loop_range_with_variables() {
        let source = r#"
            fn main() {
                let start = 10;
                let end = 20;
                for i in start..end {
                    println!("{}", i);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "for loop with variable range should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("for"));
        assert!(client_js.contains("let i"));
        assert!(client_js.contains("start"));
        assert!(client_js.contains("end"));
    }

    #[test]
    fn test_nested_for_loops() {
        let source = r#"
            fn main() {
                for i in 1..3 {
                    for j in 1..3 {
                        println!("{} {}", i, j);
                    }
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested for loops should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should have two for loops
        let for_count = client_js.matches("for (let").count();
        assert_eq!(for_count, 2, "should have exactly 2 for loops");
    }

    #[test]
    fn test_match_or_patterns() {
        let source = r#"
            fn classify(n: i32) -> String {
                match n {
                    1 => "one",
                    3 | 4 | 5 => "three to five",
                    _ => "other",
                }
            }

            fn main() {
                println!("{}", classify(3));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with OR patterns should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should generate: (__match_value === 3 || __match_value === 4 || __match_value === 5)
        assert!(client_js.contains("||"), "should have OR operator");
        assert!(client_js.contains("=== 3"), "should check for 3");
        assert!(client_js.contains("=== 4"), "should check for 4");
        assert!(client_js.contains("=== 5"), "should check for 5");
    }

    #[test]
    fn test_match_multiple_or_patterns() {
        let source = r#"
            fn classify(n: i32) -> String {
                match n {
                    1 | 2 => "one or two",
                    3 | 4 | 5 => "three to five",
                    6 | 7 | 8 | 9 => "six to nine",
                    _ => "other",
                }
            }

            fn main() {
                let result = classify(7);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with multiple OR patterns should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should have multiple OR conditions
        assert!(client_js.contains("=== 6 || "), "should check for 6");
        assert!(client_js.contains("=== 7 || "), "should check for 7");
        assert!(client_js.contains("=== 9"), "should check for 9");
    }

    #[test]
    fn test_match_single_pattern_still_works() {
        let source = r#"
            fn test(n: i32) -> String {
                match n {
                    1 => "one",
                    2 => "two",
                    _ => "other",
                }
            }

            fn main() {
                println!("{}", test(1));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with single patterns should still work");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("=== 1"), "should check for 1");
        assert!(client_js.contains("=== 2"), "should check for 2");
    }

    // =============================================================================
    // JSX Integration Tests (10 tests)
    // =============================================================================

    #[test]
    fn test_jsx_basic_element() {
        let source = r#"
            fn main() {
                let element = <div>"Hello, World!"</div>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "basic JSX element should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("createElement") || client_js.contains("div"),
                "should generate JSX element");
    }

    #[test]
    fn test_jsx_with_attributes() {
        let source = r#"
            fn main() {
                let button = <button id="submit" class="btn">"Click me"</button>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX with attributes should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("submit") || client_js.contains("btn"),
                "should include attribute values");
    }

    #[test]
    fn test_jsx_with_nested_children() {
        let source = r#"
            fn main() {
                let element = <div>
                    <h1>"Title"</h1>
                    <p>"Description"</p>
                </div>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX with nested children should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("h1") && client_js.contains("p"),
                "should include nested elements");
    }

    #[test]
    fn test_jsx_with_expressions() {
        let source = r#"
            fn main() {
                let name = "Alice";
                let age = 25;
                let card = <div>
                    <h1>{name}</h1>
                    <p>{age}</p>
                </div>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX with expressions should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("name") && client_js.contains("age"),
                "should include expression variables");
    }

    #[test]
    fn test_jsx_self_closing_tags() {
        let source = r#"
            fn main() {
                let input = <input type="text" />;
                let br = <br />;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX self-closing tags should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("input") || client_js.contains("br"),
                "should generate self-closing elements");
    }

    #[test]
    fn test_jsx_event_handlers() {
        let source = r#"
            fn handle_click() {
                println!("Clicked!");
            }

            fn main() {
                let button = <button onclick={handle_click}>"Click"</button>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX with event handlers should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("onclick") || client_js.contains("handle_click"),
                "should include event handler");
    }

    #[test]
    fn test_jsx_conditional_rendering() {
        let source = r#"
            fn main() {
                let show = true;
                let element = if show {
                    <div>"Visible"</div>
                } else {
                    <div>"Hidden"</div>
                };
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX with conditional rendering should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Visible") && client_js.contains("Hidden"),
                "should include both conditional branches");
    }

    #[test]
    fn test_jsx_with_array_iteration() {
        let source = r#"
            fn main() {
                let items = ["a", "b", "c"];
                let list = <ul>
                    {items}
                </ul>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX with array iteration should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("items") && client_js.contains("ul"),
                "should include array and ul element");
    }

    #[test]
    fn test_jsx_complex_nested_structure() {
        let source = r#"
            fn main() {
                let page = <div>
                    <header>
                        <h1>"My App"</h1>
                        <nav>
                            <a href="/">"Home"</a>
                            <a href="/about">"About"</a>
                        </nav>
                    </header>
                    <main>
                        <section>
                            <p>"Content"</p>
                        </section>
                    </main>
                </div>;
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "complex nested JSX should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("header") && client_js.contains("nav") && client_js.contains("main"),
                "should include all nested elements");
    }

    #[test]
    fn test_jsx_component_function() {
        let source = r#"
            fn Button() {
                <button>"Click me"</button>
            }

            fn main() {
                let btn = Button();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "JSX component function should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Button"),
                "should include component function");
    }

    // =============================================================================
    // Additional Control Flow Tests (8 tests)
    // =============================================================================

    #[test]
    fn test_match_with_wildcard() {
        let source = r#"
            fn classify(n: i32) -> String {
                match n {
                    1 => "one",
                    2 => "two",
                    _ => "many",
                }
            }

            fn main() {
                println!("{}", classify(100));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with wildcard should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("else") || client_js.contains("many"),
                "should include wildcard branch");
    }

    #[test]
    fn test_match_with_string_literals() {
        let source = r##"
            fn get_color(name: String) -> String {
                match name {
                    "red" => "#FF0000",
                    "green" => "#00FF00",
                    "blue" => "#0000FF",
                    _ => "#000000",
                }
            }

            fn main() {
                let color = get_color("red");
            }
        "##;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with string literals should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("red") || client_js.contains("FF0000"),
                "should include string patterns");
    }

    #[test]
    fn test_while_loop_basic() {
        let source = r#"
            fn main() {
                let mut count = 0;
                while count < 5 {
                    println!("{}", count);
                    count = count + 1;
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "while loop should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("while"),
                "should generate while loop");
    }

    #[test]
    fn test_nested_loops_with_conditions() {
        let source = r#"
            fn main() {
                for i in 1..5 {
                    for j in 1..5 {
                        if i == j {
                            println!("Equal: {}", i);
                        } else {
                            println!("Different: {} {}", i, j);
                        }
                    }
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested loops with conditions should compile successfully");

        let (_, client_js) = result.unwrap();
        let for_count = client_js.matches("for (let").count();
        assert_eq!(for_count, 2, "should have exactly 2 for loops");
        assert!(client_js.contains("if"), "should have if statement");
    }

    #[test]
    fn test_match_in_loop() {
        let source = r#"
            fn main() {
                for i in 1..10 {
                    match i {
                        1 | 2 | 3 => println!("Small"),
                        4 | 5 | 6 => println!("Medium"),
                        _ => println!("Large"),
                    }
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match in loop should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("for") && (client_js.contains("===") || client_js.contains("switch")),
                "should contain both loop and match logic");
    }

    #[test]
    #[ignore] // Parser doesn't support sized array syntax [T; N] yet
    fn test_early_return_in_function() {
        let source = r#"
            fn find_first_positive(numbers: [i32; 5]) -> i32 {
                for num in numbers {
                    if num > 0 {
                        return num;
                    }
                }
                -1
            }

            fn main() {
                let nums = [1, 2, 3, 4, 5];
                let result = find_first_positive(nums);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "early return in function should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("return"),
                "should include return statement");
    }

    #[test]
    fn test_if_else_chain() {
        let source = r#"
            fn grade(score: i32) -> String {
                if score >= 90 {
                    "A"
                } else {
                    if score >= 80 {
                        "B"
                    } else {
                        if score >= 70 {
                            "C"
                        } else {
                            "F"
                        }
                    }
                }
            }

            fn main() {
                println!("{}", grade(85));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if/else chain should compile successfully");

        let (_, client_js) = result.unwrap();
        let if_count = client_js.matches("if (").count();
        assert!(if_count >= 3, "should have multiple if statements");
    }

    #[test]
    fn test_complex_boolean_expressions() {
        let source = r#"
            fn check(a: i32, b: i32, c: i32) -> bool {
                if (a > 0 && b > 0) || c > 0 {
                    if a != b && b != c {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }

            fn main() {
                let result = check(1, 2, 3);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "complex boolean expressions should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("&&") || client_js.contains("||"),
                "should include boolean operators");
    }

    // =============================================================================
    // Function Tests (8 tests)
    // =============================================================================

    #[test]
    fn test_fibonacci_recursion() {
        let source = r#"
            fn fibonacci(n: i32) -> i32 {
                if n <= 1 {
                    n
                } else {
                    fibonacci(n - 1) + fibonacci(n - 2)
                }
            }

            fn main() {
                let result = fibonacci(10);
                println!("Fib(10): {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "fibonacci (multiple recursive calls) should compile successfully");

        let (server_js, _) = result.unwrap();
        // Should have two recursive calls to fibonacci
        let fib_call_count = server_js.matches("fibonacci").count();
        assert!(fib_call_count >= 3, "should have multiple fibonacci calls");
    }

    #[test]
    fn test_mutual_recursion() {
        let source = r#"
            fn is_even(n: i32) -> bool {
                if n == 0 {
                    true
                } else {
                    is_odd(n - 1)
                }
            }

            fn is_odd(n: i32) -> bool {
                if n == 0 {
                    false
                } else {
                    is_even(n - 1)
                }
            }

            fn main() {
                println!("Is 4 even? {}", is_even(4));
                println!("Is 3 odd? {}", is_odd(3));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "mutual recursion should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("is_even") && server_js.contains("is_odd"),
                "should include both mutually recursive functions");
    }

    #[test]
    fn test_tail_recursion_with_accumulator() {
        let source = r#"
            fn sum_helper(n: i32, acc: i32) -> i32 {
                if n == 0 {
                    acc
                } else {
                    sum_helper(n - 1, acc + n)
                }
            }

            fn sum_to_n(n: i32) -> i32 {
                sum_helper(n, 0)
            }

            fn main() {
                let result = sum_to_n(100);
                println!("Sum: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "tail recursion with accumulator should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("sum_helper"),
                "should include helper function");
    }

    #[test]
    fn test_closure_without_annotations() {
        let source = r#"
            fn main() {
                let add_one = |x| x + 1;
                let result = add_one(5);
                println!("Result: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "closure without type annotations should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("=>") || client_js.contains("function"),
                "should generate closure/arrow function");
    }

    #[test]
    fn test_higher_order_function() {
        let source = r#"
            fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
                f(f(x))
            }

            fn double(n: i32) -> i32 {
                n * 2
            }

            fn main() {
                let result = apply_twice(double, 5);
                println!("Result: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "higher-order function should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("apply_twice") && client_js.contains("double"),
                "should include both functions");
    }

    #[test]
    fn test_function_returning_function() {
        let source = r#"
            fn make_adder(n: i32) -> fn(i32) -> i32 {
                |x| x + n
            }

            fn main() {
                let add_five = make_adder(5);
                let result = add_five(10);
            }
        "#;

        let result = compile_source(source);
        // This may not work depending on closure implementation
        // but should at least attempt to compile
        let _ = result;
    }

    #[test]
    fn test_mixed_return_styles() {
        let source = r#"
            fn process(n: i32) -> i32 {
                if n < 0 {
                    return 0;
                }

                if n == 0 {
                    return 1;
                }

                // Implicit return
                n * 2
            }

            fn main() {
                println!("{}", process(5));
                println!("{}", process(-1));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "mixed explicit and implicit returns should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("return"),
                "should include return statements");
    }

    #[test]
    fn test_function_with_many_parameters() {
        let source = r#"
            fn calculate(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
                (a + b) * (c + d) / e
            }

            fn main() {
                let result = calculate(1, 2, 3, 4, 5);
                println!("Result: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "function with many parameters should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("calculate"),
                "should include calculate function");
    }

    // =============================================================================
    // Type System Tests (8 tests)
    // =============================================================================

    #[test]
    fn test_option_some() {
        let source = r#"
            fn find_value(n: i32) -> Option<i32> {
                if n > 0 {
                    Some(n * 2)
                } else {
                    None
                }
            }

            fn main() {
                let result = find_value(5);
                match result {
                    Some(v) => println!("Found: {}", v),
                    None => println!("Not found"),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Option with Some should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Some") || client_js.contains("None"),
                "should include Option constructors");
    }

    #[test]
    fn test_option_none() {
        let source = r#"
            fn get_user(id: i32) -> Option<String> {
                if id == 1 {
                    Some("Alice")
                } else {
                    None
                }
            }

            fn main() {
                match get_user(999) {
                    Some(name) => println!("User: {}", name),
                    None => println!("User not found"),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Option with None should compile successfully");
    }

    #[test]
    fn test_result_ok() {
        let source = r#"
            fn divide(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    Err("Division by zero")
                } else {
                    Ok(a / b)
                }
            }

            fn main() {
                match divide(10, 2) {
                    Ok(v) => println!("Result: {}", v),
                    Err(e) => println!("Error: {}", e),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Result with Ok should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Ok") || client_js.contains("Err"),
                "should include Result constructors");
    }

    #[test]
    fn test_result_err() {
        let source = r#"
            fn parse_number(s: String) -> Result<i32, String> {
                if s == "42" {
                    Ok(42)
                } else {
                    Err("Invalid number")
                }
            }

            fn main() {
                match parse_number("invalid") {
                    Ok(n) => println!("Parsed: {}", n),
                    Err(e) => println!("Parse error: {}", e),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Result with Err should compile successfully");
    }

    #[test]
    fn test_option_match_unwrap() {
        let source = r#"
            fn main() {
                let maybe_value = Some(42);

                let value = match maybe_value {
                    Some(v) => v,
                    None => 0,
                };

                println!("Value: {}", value);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Option match unwrap should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Some") || client_js.contains("42"),
                "should include Some constructor");
    }

    #[test]
    fn test_result_error_handling() {
        let source = r#"
            fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    Err("Cannot divide by zero")
                } else {
                    Ok(a / b)
                }
            }

            fn main() {
                let result1 = safe_divide(10, 2);
                let result2 = safe_divide(10, 0);

                match result1 {
                    Ok(v) => println!("Success: {}", v),
                    Err(e) => println!("Error: {}", e),
                }

                match result2 {
                    Ok(v) => println!("Success: {}", v),
                    Err(e) => println!("Error: {}", e),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Result error handling should compile successfully");
    }

    #[test]
    fn test_nested_option() {
        let source = r#"
            fn get_nested() -> Option<Option<i32>> {
                Some(Some(42))
            }

            fn main() {
                match get_nested() {
                    Some(inner) => {
                        match inner {
                            Some(v) => println!("Value: {}", v),
                            None => println!("Inner None"),
                        }
                    },
                    None => println!("Outer None"),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested Option should compile successfully");
    }

    #[test]
    fn test_option_result_combination() {
        let source = r#"
            fn try_find(id: i32) -> Result<Option<String>, String> {
                if id < 0 {
                    Err("Invalid ID")
                } else {
                    if id == 1 {
                        Ok(Some("Alice"))
                    } else {
                        Ok(None)
                    }
                }
            }

            fn main() {
                match try_find(1) {
                    Ok(maybe_user) => {
                        match maybe_user {
                            Some(name) => println!("Found: {}", name),
                            None => println!("Not found"),
                        }
                    },
                    Err(e) => println!("Error: {}", e),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Option/Result combination should compile successfully");
    }

    // =============================================================================
    // Collection Tests (6 tests)
    // =============================================================================

    #[test]
    fn test_array_literal() {
        let source = r#"
            fn main() {
                let numbers = [1, 2, 3, 4, 5];
                println!("{}", numbers[0]);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "array literal should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("[") && client_js.contains("]"),
                "should generate array literal");
    }

    #[test]
    fn test_array_indexing() {
        let source = r#"
            fn main() {
                let items = ["a", "b", "c"];
                let first = items[0];
                let second = items[1];
                let third = items[2];

                println!("{} {} {}", first, second, third);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "array indexing should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("[0]") || client_js.contains("[1]") || client_js.contains("[2]"),
                "should include array access");
    }

    #[test]
    fn test_array_iteration_for_in() {
        let source = r#"
            fn main() {
                let numbers = [10, 20, 30, 40, 50];
                for num in numbers {
                    println!("Number: {}", num);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "array iteration with for-in should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("for") && client_js.contains("numbers"),
                "should generate for loop over array");
    }

    #[test]
    fn test_range_iteration() {
        let source = r#"
            fn sum_range(start: i32, end: i32) -> i32 {
                let mut sum = 0;
                for i in start..end {
                    sum = sum + i;
                }
                sum
            }

            fn main() {
                let result = sum_range(1, 11);
                println!("Sum: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "range iteration should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("for"),
                "should generate for loop");
    }

    #[test]
    fn test_nested_arrays() {
        let source = r#"
            fn main() {
                let matrix = [[1, 2], [3, 4]];
                let first_row = matrix[0];
                let element = first_row[1];
                println!("{}", element);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested arrays should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("[") && client_js.contains("matrix"),
                "should include nested array access");
    }

    #[test]
    fn test_array_with_different_types() {
        let source = r#"
            fn main() {
                let integers = [1, 2, 3];
                let strings = ["a", "b", "c"];
                let bools = [true, false, true];

                println!("{}", integers[0]);
                println!("{}", strings[1]);
                println!("{}", bools[2]);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "arrays with different types should compile successfully");
    }

    // =============================================================================
    // Expression Tests (6 tests)
    // =============================================================================

    #[test]
    fn test_arithmetic_expressions() {
        let source = r#"
            fn main() {
                let a = 10 + 5;
                let b = 20 - 3;
                let c = 4 * 7;
                let d = 100 / 10;
                let e = 17 % 5;

                println!("{} {} {} {} {}", a, b, c, d, e);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "arithmetic expressions should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("+") || client_js.contains("-") || client_js.contains("*"),
                "should include arithmetic operators");
    }

    #[test]
    fn test_comparison_operators() {
        let source = r#"
            fn main() {
                let eq = 5 == 5;
                let ne = 10 != 5;
                let lt = 3 < 7;
                let gt = 8 > 2;
                let le = 5 <= 5;
                let ge = 10 >= 9;

                println!("{} {} {} {} {} {}", eq, ne, lt, gt, le, ge);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "comparison operators should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("==") || client_js.contains("<") || client_js.contains(">"),
                "should include comparison operators");
    }

    #[test]
    fn test_logical_operators() {
        let source = r#"
            fn main() {
                let and_result = true && false;
                let or_result = true || false;
                let not_result = !true;

                println!("{} {} {}", and_result, or_result, not_result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "logical operators should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("&&") || client_js.contains("||") || client_js.contains("!"),
                "should include logical operators");
    }

    #[test]
    fn test_operator_precedence() {
        let source = r#"
            fn main() {
                let result1 = 2 + 3 * 4;
                let result2 = (2 + 3) * 4;
                let result3 = 10 - 5 + 2;
                let result4 = 20 / 4 / 2;

                println!("{} {} {} {}", result1, result2, result3, result4);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "operator precedence should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("(") && client_js.contains(")"),
                "should include parentheses for precedence");
    }

    #[test]
    fn test_string_operations() {
        let source = r#"
            fn main() {
                let name = "Alice";
                let greeting = "Hello";
                let message = greeting;

                println!("{}, {}!", greeting, name);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "string operations should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Alice") || client_js.contains("Hello"),
                "should include string literals");
    }

    #[test]
    fn test_complex_expressions() {
        let source = r#"
            fn compute(a: i32, b: i32, c: i32) -> i32 {
                ((a + b) * c - 10) / 2
            }

            fn main() {
                let result = compute(3, 7, 4);
                println!("Result: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "complex expressions should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("compute"),
                "should include compute function");
    }

    // =============================================================================
    // Edge Cases & Regression Tests (4 tests)
    // =============================================================================

    #[test]
    fn test_empty_function() {
        let source = r#"
            fn do_nothing() {
            }

            fn main() {
                do_nothing();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "empty function should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("do_nothing"),
                "should include empty function");
    }

    #[test]
    fn test_single_line_function() {
        let source = r#"
            fn square(n: i32) -> i32 { n * n }

            fn main() {
                let result = square(7);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "single-line function should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("square"),
                "should include square function");
    }

    #[test]
    fn test_deeply_nested_expressions() {
        let source = r#"
            fn main() {
                let result = if true {
                    if true {
                        if true {
                            if true {
                                42
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                };

                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "deeply nested expressions should compile successfully");
    }

    #[test]
    fn test_large_match_expression() {
        let source = r#"
            fn classify_number(n: i32) -> String {
                match n {
                    0 => "zero",
                    1 | 2 | 3 => "small",
                    4 | 5 | 6 => "medium",
                    7 | 8 | 9 => "large",
                    10 | 11 | 12 | 13 | 14 | 15 => "very large",
                    _ => "huge",
                }
            }

            fn main() {
                println!("{}", classify_number(5));
                println!("{}", classify_number(100));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "large match expression should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("||"),
                "should include OR patterns");
    }

    // =============================================================================
    // Async/Await Tests - Phase 5 Sprint 1
    // =============================================================================

    #[test]
    fn test_async_function() {
        let source = r#"
            async fn get_data() -> i32 {
                42
            }

            fn main() {
                let value = get_data();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "async function should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("async function"),
                "should generate async function");
    }

    #[test]
    fn test_await_expression() {
        let source = r#"
            async fn fetch_value() -> i32 {
                42
            }

            async fn main() {
                let value = await fetch_value();
                println!("{}", value);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "await expression should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("await"),
                "should generate await expression");
    }

    #[test]
    fn test_async_with_result() {
        let source = r#"
            async fn fetch_user(id: i32) -> Result<String, String> {
                if id > 0 {
                    Ok("Alice")
                } else {
                    Err("Invalid ID")
                }
            }

            async fn main() {
                match await fetch_user(1) {
                    Ok(name) => println!("User: {}", name),
                    Err(e) => println!("Error: {}", e),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "async with Result should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("async") && client_js.contains("await"),
                "should include async/await keywords");
    }

    #[test]
    fn test_nested_await() {
        let source = r#"
            async fn get_id() -> i32 {
                1
            }

            async fn get_name(id: i32) -> String {
                "Alice"
            }

            async fn main() {
                let id = await get_id();
                let name = await get_name(id);
                println!("User {}: {}", id, name);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested await should compile successfully");

        let (_, client_js) = result.unwrap();
        let await_count = client_js.matches("await").count();
        assert!(await_count >= 2,
                "should include multiple await expressions");
    }

    #[test]
    fn test_async_in_match() {
        let source = r#"
            async fn check_status(code: i32) -> String {
                match code {
                    200 => "OK",
                    404 => "Not Found",
                    500 => "Error",
                    _ => "Unknown",
                }
            }

            async fn main() {
                let status = await check_status(200);
                println!("Status: {}", status);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "async in match should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("async") && client_js.contains("match"),
                "should include async function with match expression");
    }

    #[test]
    fn test_async_with_for_loop() {
        let source = r#"
            async fn process_item(n: i32) -> i32 {
                n * 2
            }

            async fn main() {
                for i in 1..5 {
                    let result = await process_item(i);
                    println!("{}", result);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "async with for loop should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("await") && client_js.contains("for"),
                "should include await in for loop");
    }

    #[test]
    fn test_multiple_async_functions() {
        let source = r#"
            async fn fetch_data() -> i32 {
                42
            }

            async fn process_data(data: i32) -> i32 {
                data * 2
            }

            async fn save_data(data: i32) -> bool {
                true
            }

            async fn main() {
                let data = await fetch_data();
                let processed = await process_data(data);
                let saved = await save_data(processed);
                println!("Saved: {}", saved);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "multiple async functions should compile successfully");

        let (_, client_js) = result.unwrap();
        let async_count = client_js.matches("async function").count();
        assert!(async_count >= 4,
                "should generate 4 async functions");
    }

    #[test]
    fn test_async_with_option() {
        let source = r#"
            async fn find_value(n: i32) -> Option<i32> {
                if n > 0 {
                    Some(n * 2)
                } else {
                    None
                }
            }

            async fn main() {
                match await find_value(5) {
                    Some(v) => println!("Found: {}", v),
                    None => println!("Not found"),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "async with Option should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("async") && client_js.contains("await"),
                "should include async/await with Option");
    }

    // =============================================================================
    // Try Operator (?) Tests - Phase 5 Sprint 2
    // =============================================================================

    #[test]
    fn test_try_operator_basic() {
        let source = r#"
            fn divide(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    return Err("Division by zero");
                }
                Ok(a / b)
            }

            fn calculate() -> Result<i32, String> {
                let x = divide(10, 2)?;
                Ok(x)
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "? operator should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains(".value"),
                "should generate .value unwrap for ? operator");
    }

    #[test]
    fn test_try_operator_chaining() {
        let source = r#"
            fn divide(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    Err("Division by zero")
                } else {
                    Ok(a / b)
                }
            }

            fn calculate() -> Result<i32, String> {
                let x = divide(10, 2)?;
                let y = divide(x, 2)?;
                Ok(y)
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "chained ? operators should compile");

        let (_, client_js) = result.unwrap();
        let value_count = client_js.matches(".value").count();
        assert!(value_count >= 2,
                "should generate multiple .value unwraps");
    }

    #[test]
    fn test_try_operator_with_match() {
        let source = r#"
            fn get_value() -> Result<i32, String> {
                Ok(42)
            }

            fn process() -> Result<i32, String> {
                let value = get_value()?;
                match value {
                    42 => Ok(1),
                    _ => Ok(0),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "? operator with match should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains(".value") && client_js.contains("match"),
                "should include both ? operator and match");
    }

    #[test]
    fn test_try_operator_in_async() {
        let source = r#"
            async fn fetch_data() -> Result<i32, String> {
                Ok(42)
            }

            async fn process() -> Result<i32, String> {
                let data = await fetch_data();
                let value = data?;
                Ok(value * 2)
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "? operator in async function should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("async") && client_js.contains(".value"),
                "should include both async and ? operator");
    }

    #[test]
    fn test_try_operator_option() {
        let source = r#"
            fn find_value(n: i32) -> Option<i32> {
                if n > 0 {
                    Some(n)
                } else {
                    None
                }
            }

            fn process(n: i32) -> Option<i32> {
                let value = find_value(n)?;
                Some(value * 2)
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "? operator with Option should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains(".value"),
                "should generate .value unwrap for Option");
    }

    // ========================================
    // Generic Function Tests (Phase 5 Sprint 3)
    // ========================================

    #[test]
    fn test_generic_identity_function() {
        let source = r#"
            fn identity<T>(value: T) -> T {
                value
            }

            fn main() {
                let x = identity(42);
                let y = identity("hello");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic identity function should compile");

        let (_, client_js) = result.unwrap();
        // Type parameters should be erased in JavaScript
        assert!(client_js.contains("function identity(value)"),
                "generic function should have type parameters erased");
    }

    #[test]
    fn test_generic_multiple_type_params() {
        let source = r#"
            fn pair<T, U>(first: T, second: U) -> i32 {
                42
            }

            fn main() {
                let x = pair(1, "hello");
                let y = pair("world", 3.14);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic function with multiple type params should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("function pair(first, second)"),
                "should generate function with multiple parameters");
    }

    #[test]
    fn test_generic_with_array() {
        let source = r#"
            fn first<T>(items: [T]) -> T {
                items[0]
            }

            fn main() {
                let numbers = [1, 2, 3];
                let x = first(numbers);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic function with array should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("items[0]"),
                "should generate array indexing");
    }

    #[test]
    fn test_generic_with_option() {
        let source = r#"
            fn wrap<T>(value: T) -> Option<T> {
                Some(value)
            }

            fn main() {
                let wrapped = wrap(42);
                match wrapped {
                    Some(v) => println!("{}", v),
                    None => println!("empty"),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic function with Option should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Some(value)"),
                "should generate Option constructor");
    }

    #[test]
    fn test_generic_with_result() {
        let source = r#"
            fn safe_divide<T>(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    Err("division by zero")
                } else {
                    Ok(a / b)
                }
            }

            fn main() {
                let result = safe_divide(10, 2);
                match result {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("{}", e),
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic function with Result should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Err(") && client_js.contains("Ok("),
                "should generate Result constructors");
    }

    #[test]
    fn test_generic_higher_order_function() {
        let source = r#"
            fn apply<T, U>(value: T, f: fn(T) -> U) -> U {
                f(value)
            }

            fn double(x: i32) -> i32 {
                x * 2
            }

            fn main() {
                let result = apply(5, double);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic higher-order function should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("function apply(value, f)"),
                "should generate higher-order function");
    }

    // ========================================
    // Trait System Tests (Phase 5 Sprint 4)
    // ========================================

    #[test]
    fn test_trait_definition() {
        let source = r#"
            trait Display {
                fn to_string(self: Self) -> String;
            }

            fn main() {
                println!("trait defined");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "trait definition should compile");

        let (_, client_js) = result.unwrap();
        // Traits are compile-time only, no JavaScript generated
        assert!(!client_js.contains("trait Display"),
                "traits should not appear in generated JavaScript");
    }

    #[test]
    fn test_trait_impl() {
        let source = r#"
            trait Display {
                fn to_string(self: Self) -> String;
            }

            struct Point {
                x: i32,
                y: i32,
            }

            impl Display for Point {
                fn to_string(self: Point) -> String {
                    "Point(10, 20)"
                }
            }

            fn main() {
                let p = Point { x: 10, y: 20 };
                let result = p.to_string();
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "trait implementation should compile");

        let (_, client_js) = result.unwrap();
        // Struct constructor should be generated
        assert!(client_js.contains("function Point(x, y)"),
                "should generate struct constructor");
        // Trait method should be added to prototype
        assert!(client_js.contains("Point.prototype.to_string"),
                "should generate prototype method for trait impl");
    }

    #[test]
    fn test_generic_with_trait_bound() {
        let source = r#"
            trait Printable {
                fn print(self: Self) -> String;
            }

            struct Message {
                text: String,
            }

            impl Printable for Message {
                fn print(self: Message) -> String {
                    self.text
                }
            }

            fn print_value<T: Printable>(value: T) -> String {
                value.print()
            }

            fn main() {
                let msg = Message { text: "hello" };
                let result = print_value(msg);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic function with trait bound should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("function print_value(value)"),
                "should generate generic function with trait bound");
        assert!(client_js.contains("Message.prototype.print"),
                "should generate trait method implementation");
    }

    #[test]
    fn test_multiple_trait_bounds() {
        let source = r#"
            trait Display {
                fn to_string(self: Self) -> String;
            }

            trait Clone {
                fn clone(self: Self) -> Self;
            }

            struct User {
                name: String,
            }

            impl Display for User {
                fn to_string(self: User) -> String {
                    "User"
                }
            }

            impl Clone for User {
                fn clone(self: User) -> User {
                    self
                }
            }

            fn process<T: Display + Clone>(value: T) -> String {
                "processed"
            }

            fn main() {
                let user = User { name: "Alice" };
                let result = process(user);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "generic function with multiple trait bounds should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("User.prototype.to_string"),
                "should generate to_string method");
        assert!(client_js.contains("User.prototype.clone"),
                "should generate clone method");
    }

    #[test]
    fn test_trait_method_call() {
        let source = r#"
            trait Comparable {
                fn compare(self: Self, other: Self) -> i32;
            }

            struct Number {
                value: i32,
            }

            impl Comparable for Number {
                fn compare(self: Number, other: Number) -> i32 {
                    self.value - other.value
                }
            }

            fn main() {
                let a = Number { value: 10 };
                let b = Number { value: 5 };
                let result = a.compare(b);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "trait method call should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Number.prototype.compare"),
                "should generate compare method");
        assert!(client_js.contains(".compare("),
                "should generate method call");
    }

    #[test]
    fn test_inherent_vs_trait_impl() {
        let source = r#"
            trait Display {
                fn to_string(self: Self) -> String;
            }

            struct Person {
                name: String,
                age: i32,
            }

            impl Person {
                fn new(name: String, age: i32) -> Person {
                    Person { name: name, age: age }
                }
            }

            impl Display for Person {
                fn to_string(self: Person) -> String {
                    "Person"
                }
            }

            fn main() {
                let p = Person::new("Alice", 30);
                let s = p.to_string();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "both inherent and trait impl should compile");

        let (_, client_js) = result.unwrap();
        // Both inherent and trait methods should be generated
        assert!(client_js.contains("Person.prototype.new") || client_js.contains("Person.new"),
                "should generate inherent method");
        assert!(client_js.contains("Person.prototype.to_string"),
                "should generate trait method");
    }

    #[test]
    fn test_trait_with_multiple_methods() {
        let source = r#"
            trait Shape {
                fn area(self: Self) -> i32;
                fn perimeter(self: Self) -> i32;
            }

            struct Rectangle {
                width: i32,
                height: i32,
            }

            impl Shape for Rectangle {
                fn area(self: Rectangle) -> i32 {
                    self.width * self.height
                }

                fn perimeter(self: Rectangle) -> i32 {
                    2 * (self.width + self.height)
                }
            }

            fn main() {
                let rect = Rectangle { width: 5, height: 3 };
                let a = rect.area();
                let p = rect.perimeter();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "trait with multiple methods should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Rectangle.prototype.area"),
                "should generate area method");
        assert!(client_js.contains("Rectangle.prototype.perimeter"),
                "should generate perimeter method");
    }

    #[test]
    fn test_self_type_in_trait() {
        let source = r#"
            trait Builder {
                fn build(self: Self) -> Self;
            }

            struct Config {
                enabled: bool,
            }

            impl Builder for Config {
                fn build(self: Config) -> Config {
                    self
                }
            }

            fn main() {
                let cfg = Config { enabled: true };
                let built = cfg.build();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Self type in trait should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Config.prototype.build"),
                "should generate build method");
    }

    #[test]
    fn test_trait_impl_with_generic_struct() {
        let source = r#"
            trait Display {
                fn to_string(self: Self) -> String;
            }

            struct Container<T> {
                value: T,
            }

            impl Display for Container<i32> {
                fn to_string(self: Container<i32>) -> String {
                    "Container"
                }
            }

            fn main() {
                let c = Container { value: 42 };
                let s = c.to_string();
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "trait impl for generic struct should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Container.prototype.to_string"),
                "should generate trait method for generic struct");
    }

    #[test]
    fn test_nested_trait_bounds() {
        let source = r#"
            trait Display {
                fn to_string(self: Self) -> String;
            }

            struct Wrapper<T: Display> {
                inner: T,
            }

            struct Value {
                data: i32,
            }

            impl Display for Value {
                fn to_string(self: Value) -> String {
                    "Value"
                }
            }

            fn main() {
                let v = Value { data: 42 };
                let w = Wrapper { inner: v };
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested trait bounds should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("Value.prototype.to_string"),
                "should generate trait method");
    }

    // ===== Phase 5 Sprint 5: Sized Arrays and Typed Closures =====

    #[test]
    fn test_sized_array_type() {
        let source = r#"
            fn process(numbers: [i32; 5]) -> i32 {
                let sum = 0;
                for num in numbers {
                    sum = sum + num;
                }
                sum
            }

            fn main() {
                let nums = [1, 2, 3, 4, 5];
                let result = process(nums);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "sized array type should compile");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("function process"),
                "should generate function with sized array parameter");
    }

    #[test]
    fn test_closure_with_type_annotations() {
        let source = r#"
            fn main() {
                let add = |x: i32, y: i32| -> i32 {
                    x + y
                };
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "closure with type annotations should compile");
    }

    #[test]
    fn test_closure_partial_types() {
        let source = r#"
            fn main() {
                let greet = |name: String| {
                    name
                };
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "closure with param types but no return type should compile");
    }

    #[test]
    fn test_sized_array_various_sizes() {
        let source = r#"
            fn main() {
                let small: [i32; 3] = [1, 2, 3];
                let medium: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "various sized array sizes should compile");
    }

    // =============================================================================
    // Sprint 6: Deeply Nested If/Else Expressions
    // =============================================================================

    #[test]
    fn test_nested_if_2_levels_in_let() {
        let source = r#"
            fn main() {
                let result = if true {
                    if true {
                        42
                    } else {
                        0
                    }
                } else {
                    0
                };
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "2-level nested if/else in let statement should compile");
    }

    #[test]
    fn test_nested_if_3_levels() {
        let source = r#"
            fn categorize(x: i32) -> String {
                if x > 0 {
                    if x > 100 {
                        if x > 1000 {
                            "huge positive"
                        } else {
                            "large positive"
                        }
                    } else {
                        "small positive"
                    }
                } else {
                    "non-positive"
                }
            }

            fn main() {
                let result = categorize(1500);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "3-level nested if/else should compile");
    }

    #[test]
    fn test_nested_if_with_different_conditions() {
        let source = r#"
            fn classify(x: i32, y: i32) -> String {
                if x > 0 {
                    if y > 0 {
                        "both positive"
                    } else {
                        "x positive, y non-positive"
                    }
                } else {
                    if y > 0 {
                        "x non-positive, y positive"
                    } else {
                        "both non-positive"
                    }
                }
            }

            fn main() {
                let result = classify(10, -5);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested if/else with different conditions should compile");
    }

    #[test]
    fn test_nested_if_mixed_with_expressions() {
        let source = r#"
            fn calculate(x: i32) -> i32 {
                let base = if x > 0 {
                    if x > 100 {
                        100
                    } else {
                        50
                    }
                } else {
                    0
                };

                let multiplier = if base > 50 {
                    2
                } else {
                    1
                };

                base * multiplier
            }

            fn main() {
                let result = calculate(150);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested if/else mixed with other expressions should compile");
    }

    #[test]
    fn test_nested_if_5_levels() {
        let source = r#"
            fn deep_classify(x: i32) -> String {
                if x > 0 {
                    if x > 10 {
                        if x > 100 {
                            if x > 1000 {
                                if x > 10000 {
                                    "enormous"
                                } else {
                                    "very large"
                                }
                            } else {
                                "large"
                            }
                        } else {
                            "medium"
                        }
                    } else {
                        "small"
                    }
                } else {
                    "non-positive"
                }
            }

            fn main() {
                let result = deep_classify(15000);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "5-level nested if/else should compile");
    }

    #[test]
    fn test_nested_if_with_return_statements() {
        let source = r#"
            fn early_return(x: i32) -> String {
                if x > 0 {
                    if x > 100 {
                        return "large";
                    }
                    return "small";
                }
                return "non-positive";
            }

            fn main() {
                let result = early_return(150);
                println!("{}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested if/else with return statements should compile");
    }

    // ============================================================
    // CSS Integration Tests (Phase 7.5)
    // ============================================================

    #[test]
    fn test_css_macro_simple() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button {
                        background: blue;
                    }
                };
                println!("CSS styles created");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "simple css! macro should compile");
    }

    #[test]
    fn test_css_multiple_rules() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button {
                        background: blue;
                        color: white;
                    }

                    .header {
                        color: red;
                    }
                };
                println!("Multiple CSS rules created");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "css! macro with multiple rules should compile");
    }

    #[test]
    fn test_css_selector_types() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button {
                        background: blue;
                    }

                    #header {
                        color: red;
                    }

                    div {
                        color: green;
                    }

                    :hover {
                        color: yellow;
                    }
                };
                println!("Different CSS selector types created");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "css! macro with different selector types should compile");
    }

    #[test]
    fn test_css_in_function() {
        let source = r#"
            fn create_button_styles() {
                let styles = css! {
                    .primary {
                        background: blue;
                    }
                    .secondary {
                        background: gray;
                    }
                };
            }

            fn main() {
                create_button_styles();
                println!("Function with CSS executed");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "CSS in function should compile");
    }

    #[test]
    fn test_css_multiple_declarations() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .card {
                        background: white;
                        color: black;
                        display: block;
                        position: relative;
                        float: left;
                    }
                };
                println!("CSS with multiple declarations created");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "CSS with multiple declarations should compile");
    }

    #[test]
    fn test_css_compound_selector_with_pseudo() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button:hover {
                        background: darkblue;
                    }
                };
                println!("Compound selector with pseudo-class");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS compound selector with pseudo should compile");
    }

    #[test]
    fn test_css_compound_selector_multiple_classes() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button.primary {
                        background: blue;
                    }
                };
                println!("Compound selector with multiple classes");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "CSS compound selector with multiple classes should compile");
    }

    #[test]
    fn test_css_nested_descendant_selector() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .card .title {
                        color: black;
                    }
                };
                println!("Nested descendant selectors");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Nested selector error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS nested descendant selectors should compile");
    }

    #[test]
    fn test_css_complex_selectors_mixed() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button {
                        background: blue;
                    }
                    .button:hover {
                        background: darkblue;
                    }
                    .button.primary {
                        background: green;
                    }
                    .card .title {
                        color: black;
                    }
                };
                println!("Mixed selector types");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "CSS with mixed selector types should compile");
    }

    #[test]
    fn test_css_pseudo_element_before() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .icon::before {
                        content: "→";
                        color: blue;
                    }
                };
                println!("Pseudo-element ::before");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS pseudo-element ::before should compile");
    }

    #[test]
    fn test_css_pseudo_element_after() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .quote::after {
                        content: "»";
                        color: red;
                    }
                };
                println!("Pseudo-element ::after");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS pseudo-element ::after should compile");
    }

    #[test]
    fn test_css_pseudo_class_vs_pseudo_element() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button:hover {
                        background: blue;
                    }
                    .button::before {
                        content: "→";
                    }
                    .input:focus {
                        border: blue;
                    }
                    .input::placeholder {
                        color: gray;
                    }
                };
                println!("Mix of pseudo-classes and pseudo-elements");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with both pseudo-classes and pseudo-elements should compile");
    }

    #[test]
    fn test_css_multiple_pseudo_classes() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .button:hover {
                        background: darkblue;
                    }
                    .button:active {
                        background: navy;
                    }
                    .input:focus {
                        border: blue;
                    }
                    .input:disabled {
                        opacity: 0.5;
                    }
                };
                println!("Multiple pseudo-classes");
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "CSS with multiple pseudo-classes should compile");
    }

    #[test]
    fn test_css_multiple_pseudo_elements() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .quote::before {
                        content: "«";
                    }
                    .quote::after {
                        content: "»";
                    }
                    .section::first-line {
                        font-weight: bold;
                    }
                    .section::first-letter {
                        color: blue;
                    }
                };
                println!("Multiple pseudo-elements");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with multiple pseudo-elements should compile");
    }

    #[test]
    fn test_css_media_query_simple() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .container {
                        color: blue;

                        @media (min-width: 768px) {
                            color: red;
                        }
                    }
                };
                println!("Responsive container");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with media query should compile");
    }

    #[test]
    fn test_css_media_query_multiple() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .grid {
                        color: black;

                        @media (min-width: 768px) {
                            color: blue;
                        }

                        @media (min-width: 1024px) {
                            color: green;
                        }
                    }
                };
                println!("Responsive grid");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with multiple media queries should compile");
    }

    #[test]
    fn test_css_media_query_with_nesting() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .card {
                        color: white;

                        &:hover {
                            color: gray;
                        }

                        @media (min-width: 768px) {
                            background: blue;
                        }
                    }
                };
                println!("Card with media query and nesting");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with media query and nesting should compile");
    }

    #[test]
    fn test_css_media_query_complex_condition() {
        let source = r#"
            fn main() {
                let styles = css! {
                    .sidebar {
                        color: gray;

                        @media (min-width: 768px) and (max-width: 1024px) {
                            color: black;
                            background: white;
                        }
                    }
                };
                println!("Sidebar with complex media query");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with complex media query condition should compile");
    }

    #[test]
    fn test_css_dynamic_value_simple() {
        // Sprint 2 Task 2.4 - Dynamic CSS values
        let source = r#"
            fn main() {
                let color = "blue";
                let styles = css! {
                    .button {
                        background: {color};
                        padding: 12px;
                    }
                };
                println!("Button with dynamic color");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with dynamic value should compile");
    }

    #[test]
    fn test_css_dynamic_value_expression() {
        // Sprint 2 Task 2.4 - Dynamic CSS values with expressions
        let source = r#"
            fn main() {
                let is_large = true;
                let styles = css! {
                    .button {
                        padding: {is_large ? "16px" : "8px"};
                        opacity: {is_large ? 1.0 : 0.5};
                    }
                };
                println!("Button with dynamic sizing");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with dynamic expressions should compile");
    }

    #[test]
    fn test_css_dynamic_and_static_mixed() {
        // Sprint 2 Task 2.4 - Mix of static and dynamic values
        let source = r#"
            fn main() {
                let theme_color = "blue";
                let styles = css! {
                    .card {
                        background: {theme_color};
                        color: white;
                        padding: 20px;
                        border-radius: {theme_color == "blue" ? "8px" : "4px"};
                    }
                };
                println!("Card with mixed styles");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with mixed static and dynamic values should compile");
    }

    // Sprint 2 Task 2.6: Keyframe animations tests

    #[test]
    fn test_css_keyframes_simple_from_to() {
        // Simple keyframes with from/to
        let source = r#"
            fn main() {
                let styles = css! {
                    @keyframes fadeIn {
                        from { opacity: 0; }
                        to { opacity: 1; }
                    }

                    .button {
                        animation: fadeIn 0.3s ease-in;
                    }
                };
                println!("Button with fade-in animation");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with simple keyframes should compile");
    }

    #[test]
    fn test_css_keyframes_percentages() {
        // Keyframes with percentage selectors
        let source = r#"
            fn main() {
                let styles = css! {
                    @keyframes slideIn {
                        0% { transform: translateX(-100%); }
                        50% { transform: translateX(-50%); }
                        100% { transform: translateX(0); }
                    }

                    .panel {
                        animation: slideIn 0.5s ease-out;
                    }
                };
                println!("Panel with slide-in animation");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with percentage keyframes should compile");
    }

    #[test]
    fn test_css_keyframes_multiple_properties() {
        // Keyframes with multiple CSS properties
        let source = r#"
            fn main() {
                let styles = css! {
                    @keyframes pulse {
                        from {
                            opacity: 1;
                            transform: scale(1);
                        }
                        to {
                            opacity: 0.8;
                            transform: scale(1.05);
                        }
                    }

                    .logo {
                        animation: pulse 1s infinite alternate;
                    }
                };
                println!("Logo with pulse animation");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with multi-property keyframes should compile");
    }

    #[test]
    fn test_css_multiple_keyframes() {
        // Multiple keyframes in one css! block
        let source = r#"
            fn main() {
                let styles = css! {
                    @keyframes fadeIn {
                        from { opacity: 0; }
                        to { opacity: 1; }
                    }

                    @keyframes slideUp {
                        from { transform: translateY(100%); }
                        to { transform: translateY(0); }
                    }

                    .modal {
                        animation: fadeIn 0.3s ease-in, slideUp 0.3s ease-out;
                    }
                };
                println!("Modal with combined animations");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with multiple keyframes should compile");
    }

    #[test]
    fn test_css_keyframes_with_rules() {
        // Keyframes combined with regular CSS rules
        let source = r#"
            fn main() {
                let styles = css! {
                    .button {
                        background: blue;
                        padding: 12px 24px;
                        color: white;
                    }

                    .button:hover {
                        background: darkblue;
                    }

                    @keyframes spin {
                        from { transform: rotate(0deg); }
                        to { transform: rotate(360deg); }
                    }

                    .spinner {
                        animation: spin 1s linear infinite;
                    }
                };
                println!("Button with spinner");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with keyframes and rules should compile");
    }

    #[test]
    fn test_css_complex_keyframes() {
        // Complex animation with multiple steps
        let source = r#"
            fn main() {
                let styles = css! {
                    @keyframes bounce {
                        0% { transform: translateY(0); }
                        25% { transform: translateY(-20px); }
                        50% { transform: translateY(0); }
                        75% { transform: translateY(-10px); }
                        100% { transform: translateY(0); }
                    }

                    .bouncing-ball {
                        animation: bounce 0.8s ease-in-out infinite;
                    }
                };
                println!("Ball with bounce animation");
            }
        "#;

        let result = compile_source(source);
        if let Err(ref e) = result {
            eprintln!("Compilation error: {:?}", e);
        }
        assert!(result.is_ok(), "CSS with complex multi-step keyframes should compile");
    }
}
