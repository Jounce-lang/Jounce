use ravensone_compiler::lexer::Lexer;
use ravensone_compiler::token::TokenKind;

fn main() {
    // Simulate parsing <span>{stock}</span>
    let input = "{stock}".to_string();
    let mut lexer = Lexer::new(input);

    // Parser would have called enter_jsx_mode() after seeing <span>
    lexer.enter_jsx_mode();

    println!("=== Lexer state ===");
    println!("jsx_mode: {}", lexer.is_jsx_mode());

    // Now try to tokenize {stock}
    let token1 = lexer.next_token();
    println!("Token 1: {:?}", token1.kind);

    let token2 = lexer.next_token();
    println!("Token 2: {:?}", token2.kind);

    let token3 = lexer.next_token();
    println!("Token 3: {:?}", token3.kind);
}
