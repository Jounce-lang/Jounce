extern crate ravensone_compiler;

use ravensone_compiler::lexer::Lexer;

fn main() {
    let source = "<div>Hello {name}!</div>";
    let mut lexer = Lexer::new(source.to_string());

    println!("=== Tokenizing: {} ===\n", source);

    // Simulate parser behavior
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        println!("Token: {:?} '{}' (JSX mode: {})", token.kind, token.lexeme, lexer.is_jsx_mode());
        if matches!(token.kind, ravensone_compiler::token::TokenKind::Eof) {
            break;
        }
        tokens.push(token);

        // Simulate parser entering JSX mode after consuming >
        if tokens.len() == 3 { // After <, div, >
            println!("\n>>> Parser enters JSX mode after consuming > <<<\n");
            lexer.enter_jsx_mode();
        }
    }
}
