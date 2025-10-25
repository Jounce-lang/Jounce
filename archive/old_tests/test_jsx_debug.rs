extern crate ravensone_compiler;

use ravensone_compiler::lexer::Lexer;

fn main() {
    let source = "<div></div>";
    let mut lexer = Lexer::new(source.to_string());

    println!("=== Tokenizing: {} ===", source);
    println!("JSX mode: {}", lexer.is_jsx_mode());

    // Token 1: <
    let t1 = lexer.next_token();
    println!("Token 1: {:?} '{}' (JSX mode: {})", t1.kind, t1.lexeme, lexer.is_jsx_mode());

    // Token 2: div
    let t2 = lexer.next_token();
    println!("Token 2: {:?} '{}' (JSX mode: {})", t2.kind, t2.lexeme, lexer.is_jsx_mode());

    // Now the parser would call enter_jsx_mode() before consuming >
    println!("\n>>> Parser calls enter_jsx_mode() <<<\n");
    lexer.enter_jsx_mode();

    // Token 3: >
    let t3 = lexer.next_token();
    println!("Token 3: {:?} '{}' (JSX mode: {})", t3.kind, t3.lexeme, lexer.is_jsx_mode());

    // Token 4: Should be < from </div>
    let t4 = lexer.next_token();
    println!("Token 4: {:?} '{}' (JSX mode: {})", t4.kind, t4.lexeme, lexer.is_jsx_mode());

    // Token 5: Should be /
    let t5 = lexer.next_token();
    println!("Token 5: {:?} '{}' (JSX mode: {})", t5.kind, t5.lexeme, lexer.is_jsx_mode());

    // Token 6: Should be div
    let t6 = lexer.next_token();
    println!("Token 6: {:?} '{}' (JSX mode: {})", t6.kind, t6.lexeme, lexer.is_jsx_mode());

    // Token 7: Should be >
    let t7 = lexer.next_token();
    println!("Token 7: {:?} '{}' (JSX mode: {})", t7.kind, t7.lexeme, lexer.is_jsx_mode());
}
