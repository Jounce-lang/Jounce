

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Self { kind, lexeme, line, column }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Keywords
    Let, Const, Fn, Struct, Enum, Impl, Trait, Component, Extern, Return, Server, Client, Async, Await, Use, True, False, If, Else, While, For, In, Match, Mut, As,

    // Identifiers & Literals
    Identifier,
    Lifetime(String),  // Lifetime like 'a, 'b, 'static
    Integer(i64),
    Float(String), // Store as string to preserve precision during parsing
    String(String),
    Bool(bool),

    // Symbols & Punctuation
    At,          // @
    Assign,      // =
    Semicolon,   // ;
    Colon,       // :
    Comma,       // ,
    Dot,         // .
    DotDot,      // .. (range)
    DotDotEq,    // ..= (inclusive range)
    DotDotDot,   // ... (spread operator)
    Plus,        // +
    Minus,       // -
    Star,        // *
    Percent,     // %
    Bang,        // !
    Question,    // ?
    Ampersand,   // &
    AmpAmp,      // && (logical AND)
    Pipe,        // | NEW: For closures
    PipePipe,    // || (logical OR)
    Arrow,       // ->
    FatArrow,    // => NEW: For lambda expressions
    DoubleColon, // :: NEW

    // Comparison operators
    Eq,          // ==
    NotEq,       // !=
    LtEq,        // <=
    GtEq,        // >=

    // Grouping
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]

    // JSX & Comparison
    LAngle,      // <
    RAngle,      // >
    Slash,       // /

    // JSX-specific tokens
    JsxText(String),       // Text content between JSX tags
    JsxSelfClose,          // />
    JsxOpenBrace,          // { in JSX context (for expressions)
    JsxCloseBrace,         // } in JSX context

    // CSS-specific tokens
    CssMacro,              // css!
    CssSelector(String),   // .button, #id, div, .button:hover, etc.
    CssProperty(String),   // background, padding, color, etc.
    CssValue(String),      // blue, 12px, "Arial", etc.
    CssMedia,              // @media

    // Meta
    Eof,
    Illegal(char),
}

lazy_static::lazy_static! {
    pub static ref KEYWORDS: std::collections::HashMap<&'static str, TokenKind> = {
        let mut map = std::collections::HashMap::new();
        map.insert("let", TokenKind::Let);
        map.insert("const", TokenKind::Const);
        map.insert("fn", TokenKind::Fn);
        map.insert("struct", TokenKind::Struct);
        map.insert("enum", TokenKind::Enum);
        map.insert("impl", TokenKind::Impl);
        map.insert("trait", TokenKind::Trait);
        map.insert("component", TokenKind::Component);
        map.insert("extern", TokenKind::Extern);
        map.insert("return", TokenKind::Return);
        map.insert("server", TokenKind::Server);
        map.insert("client", TokenKind::Client);
        map.insert("async", TokenKind::Async);
        map.insert("await", TokenKind::Await);
        map.insert("use", TokenKind::Use);
        map.insert("true", TokenKind::True);
        map.insert("false", TokenKind::False);
        map.insert("if", TokenKind::If);
        map.insert("else", TokenKind::Else);
        map.insert("while", TokenKind::While);
        map.insert("for", TokenKind::For);
        map.insert("in", TokenKind::In);
        map.insert("match", TokenKind::Match);
        map.insert("mut", TokenKind::Mut);
        map.insert("as", TokenKind::As);
        map
    };
}