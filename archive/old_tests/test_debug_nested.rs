use ravensone_compiler::lexer::Lexer;
use ravensone_compiler::parser::Parser;

fn main() {
    // Test nested JSX: <div>{<span>{x}</span>}</div>
    let input = r#"
fn test() {
    let stock = 5;
    return <div>{<span>{stock}</span>}</div>;
}
    "#.to_string();

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);

    match parser.parse_program() {
        Ok(_) => println!("✓ Parsed successfully"),
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
}
