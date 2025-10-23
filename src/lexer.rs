use crate::token::{Token, TokenKind, KEYWORDS};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
    jsx_mode: bool,           // Track if we're in JSX context
    jsx_depth: usize,         // Track nesting depth of JSX elements
    brace_depth: usize,       // Track braces in JSX expressions
    jsx_in_tag: bool,         // Track if we're inside a tag (between < and >)
    in_closing_tag: bool,     // Track if parser is currently parsing a closing tag
    jsx_baseline_brace_depths: Vec<usize>, // Stack of brace depths when entering each JSX element
    just_closed_jsx_expr: bool, // Track if we just emitted a JsxCloseBrace (allows delimiters as JSX text)
    css_mode: bool,           // Track if we're in CSS context
    css_depth: usize,         // Track brace nesting depth in CSS
    css_paren_depth: usize,   // Track parenthesis depth in CSS (for media queries)
    in_media_query: bool,     // Track if we're parsing @media condition (until we hit {)
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
            jsx_mode: false,
            jsx_depth: 0,
            brace_depth: 0,
            jsx_in_tag: false,
            in_closing_tag: false,
            jsx_baseline_brace_depths: Vec::new(),
            just_closed_jsx_expr: false,
            css_mode: false,
            css_depth: 0,
            css_paren_depth: 0,
            in_media_query: false,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        // In JSX mode, handle text content differently
        // Only read JSX text when we're not inside a tag (between < and >) AND we're actually inside a JSX element (jsx_depth > 0)
        // Also don't read JSX text if we're currently parsing a closing tag
        // Check if brace_depth is at or below the baseline for the current JSX element
        let baseline_brace_depth = self.jsx_baseline_brace_depths.last().copied().unwrap_or(0);

        // Read JSX text when at baseline brace depth (not inside expressions)
        // OR when we've just finished parsing an opening tag (even if inside nested braces)
        let at_baseline = self.brace_depth == baseline_brace_depth;

        // CRITICAL: Don't read JSX text if current character is a delimiter/operator
        // This prevents reading `)`, `}`, `]`, `,`, `;` as JSX text after self-closing tags in closures
        // EXCEPTION: After closing a JSX expression with `}`, allow delimiters as JSX text
        // Example: `Comments ({expr})` - the `)` after `}` should be read as text
        let is_delimiter = !self.just_closed_jsx_expr && matches!(self.ch, ')' | ']' | ',' | ';');

        // CRITICAL: Check if we would only read whitespace before a delimiter
        // This prevents empty JSX text tokens after self-closing tags in expression contexts
        let would_read_only_whitespace = self.ch.is_whitespace() && {
            let mut temp_pos = self.position;
            let mut temp_ch = self.ch;
            // Skip whitespace to see what's next
            while temp_ch.is_whitespace() && temp_ch != '\0' {
                temp_pos += 1;
                temp_ch = if temp_pos < self.input.len() {
                    self.input[temp_pos]
                } else {
                    '\0'
                };
            }
            // Check if next non-whitespace is a delimiter or JSX-significant character
            matches!(temp_ch, '}' | ')' | ']' | '<' | '\0')
        };

        let can_read_jsx_text = self.jsx_mode && self.jsx_depth > 0 && at_baseline && !self.jsx_in_tag && !self.in_closing_tag && !is_delimiter && !would_read_only_whitespace && self.ch != '<' && self.ch != '{' && self.ch != '}' && self.ch != '\0';

        if can_read_jsx_text {
            // Reset the flag since we're reading JSX text now
            self.just_closed_jsx_expr = false;
            return self.read_jsx_text();
        }

        // Reset the flag for non-JSX-text tokens (will be set again for JsxCloseBrace below)
        self.just_closed_jsx_expr = false;

        // CSS mode handling
        if self.css_mode {
            self.skip_whitespace();
            let start_col = self.column;

            // Handle CSS-specific tokens
            return match self.ch {
                '{' => {
                    self.css_depth += 1;
                    self.in_media_query = false; // Exit media query mode when { is found
                    self.read_char();
                    Token::new(TokenKind::LBrace, "{".to_string(), self.line, start_col)
                }
                '}' => {
                    if self.css_depth > 0 {
                        self.css_depth -= 1;
                    }
                    if self.css_depth == 0 {
                        self.css_mode = false;
                    }
                    self.read_char();
                    Token::new(TokenKind::RBrace, "}".to_string(), self.line, start_col)
                }
                ';' => {
                    self.read_char();
                    Token::new(TokenKind::Semicolon, ";".to_string(), self.line, start_col)
                }
                ':' => {
                    self.read_char();
                    Token::new(TokenKind::Colon, ":".to_string(), self.line, start_col)
                }
                '(' => {
                    self.css_paren_depth += 1;
                    self.read_char();
                    Token::new(TokenKind::LParen, "(".to_string(), self.line, start_col)
                }
                ')' => {
                    if self.css_paren_depth > 0 {
                        self.css_paren_depth -= 1;
                    }
                    self.read_char();
                    Token::new(TokenKind::RParen, ")".to_string(), self.line, start_col)
                }
                '.' | '#' | '&' => {
                    // CSS selector (including & for nesting)
                    self.read_css_selector()
                }
                '@' => {
                    // Check if this is @media or @keyframes
                    let pos = self.position;
                    self.read_char(); // consume '@'
                    let ident_token = self.read_identifier();

                    match ident_token.lexeme.as_str() {
                        "media" => {
                            self.in_media_query = true; // Enter media query mode
                            return Token::new(TokenKind::CssMedia, "@media".to_string(), self.line, start_col);
                        }
                        "keyframes" => {
                            return Token::new(TokenKind::CssKeyframes, "@keyframes".to_string(), self.line, start_col);
                        }
                        _ => {
                            // Not a recognized @-rule, reset
                            self.position = pos;
                            self.ch = '@';
                            self.read_char();
                            Token::new(TokenKind::At, "@".to_string(), self.line, start_col)
                        }
                    }
                }
                '\0' => Token::new(TokenKind::Eof, "".to_string(), self.line, start_col),
                _ => {
                    if self.ch.is_alphabetic() || self.ch == '-' {
                        // When in media query mode or inside parentheses, read as CSS property (handles hyphens like min-width, and keywords like 'and')
                        if self.css_paren_depth > 0 || self.in_media_query {
                            return self.read_css_property();
                        }

                        // Could be a property name or selector
                        // Peek ahead to determine which
                        let mut peek_pos = self.position;
                        while peek_pos < self.input.len() && (self.input[peek_pos].is_alphanumeric() || self.input[peek_pos] == '-') {
                            peek_pos += 1;
                        }
                        // Skip whitespace
                        while peek_pos < self.input.len() && self.input[peek_pos].is_whitespace() {
                            peek_pos += 1;
                        }

                        if peek_pos < self.input.len() && self.input[peek_pos] == ':' {
                            // It's a property name (followed by colon)
                            self.read_css_property()
                        } else if peek_pos < self.input.len() && self.input[peek_pos] == '{' {
                            // It's a selector (followed by brace)
                            self.read_css_selector()
                        } else {
                            // Assume it's a CSS value
                            self.read_css_value()
                        }
                    } else if self.ch == '"' {
                        // String value
                        self.read_string()
                    } else if self.ch.is_ascii_digit() {
                        // When in media query mode or inside parentheses, read as number
                        if self.css_paren_depth > 0 || self.in_media_query {
                            return self.read_number();
                        }
                        // Numeric value - read as CSS value
                        let num_token = self.read_number();
                        let mut value = num_token.lexeme.clone();

                        // Check for percentage sign or CSS units (px, rem, em, %, etc.)
                        if self.ch == '%' {
                            value.push('%');
                            self.read_char();
                        } else if self.ch.is_alphabetic() {
                            // Could be px, rem, em, vh, vw, etc.
                            while self.ch.is_alphabetic() {
                                value.push(self.ch);
                                self.read_char();
                            }
                        }

                        // Convert to CSS value
                        Token::new(TokenKind::CssValue(value.clone()), value, num_token.line, num_token.column)
                    } else {
                        // Unknown character
                        let ch = self.ch;
                        self.read_char();
                        Token::new(TokenKind::Illegal(ch), ch.to_string(), self.line, start_col)
                    }
                }
            };
        }

        self.skip_whitespace();
        let start_col = self.column;
        let token = match self.ch {
           ':' => {
                if self.peek() == ':' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::DoubleColon, "::".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Colon, ":".to_string(), self.line, start_col)
                }
           }
            '=' => {
                if self.peek() == '>' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::FatArrow, "=>".to_string(), self.line, start_col);
                } else if self.peek() == '=' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::Eq, "==".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Assign, "=".to_string(), self.line, start_col)
                }
            }
            ';' => Token::new(TokenKind::Semicolon, ";".to_string(), self.line, start_col),
            '|' => {
                if self.peek() == '|' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::PipePipe, "||".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Pipe, "|".to_string(), self.line, start_col)
                }
            }
            ',' => Token::new(TokenKind::Comma, ",".to_string(), self.line, start_col),
            '.' => {
                // Check for .., ..=, or ...
                if self.peek() == '.' {
                    self.read_char();
                    self.read_char();
                    // Check for ... (spread operator)
                    if self.ch == '.' {
                        self.read_char();
                        return Token::new(TokenKind::DotDotDot, "...".to_string(), self.line, start_col);
                    }
                    // Check for ..=
                    if self.ch == '=' {
                        self.read_char();
                        return Token::new(TokenKind::DotDotEq, "..=".to_string(), self.line, start_col);
                    }
                    // Just ..
                    return Token::new(TokenKind::DotDot, "..".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Dot, ".".to_string(), self.line, start_col)
                }
            }
            '+' => Token::new(TokenKind::Plus, "+".to_string(), self.line, start_col),
            '*' => Token::new(TokenKind::Star, "*".to_string(), self.line, start_col),
            '%' => Token::new(TokenKind::Percent, "%".to_string(), self.line, start_col),
            '&' => {
                if self.peek() == '&' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::AmpAmp, "&&".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Ampersand, "&".to_string(), self.line, start_col)
                }
            }
            '?' => Token::new(TokenKind::Question, "?".to_string(), self.line, start_col),
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::NotEq, "!=".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Bang, "!".to_string(), self.line, start_col)
                }
            }
            '(' => Token::new(TokenKind::LParen, "(".to_string(), self.line, start_col),
            ')' => Token::new(TokenKind::RParen, ")".to_string(), self.line, start_col),
            '{' => {
                // Track brace depth for JSX expressions
                if self.jsx_mode {
                    let baseline = self.jsx_baseline_brace_depths.last().copied().unwrap_or(0);
                    self.brace_depth += 1;
                    // Only use JsxOpenBrace for the first level (opening a JSX expression)
                    // Nested braces should be regular LBrace tokens (for blocks, match, etc.)
                    if self.brace_depth == baseline + 1 {
                        Token::new(TokenKind::JsxOpenBrace, "{".to_string(), self.line, start_col)
                    } else {
                        Token::new(TokenKind::LBrace, "{".to_string(), self.line, start_col)
                    }
                } else {
                    Token::new(TokenKind::LBrace, "{".to_string(), self.line, start_col)
                }
            }
            '}' => {
                // Track brace depth for JSX expressions
                if self.jsx_mode && self.brace_depth > 0 {
                    let baseline = self.jsx_baseline_brace_depths.last().copied().unwrap_or(0);
                    // Only use JsxCloseBrace for the first level (closing a JSX expression)
                    // Nested braces should be regular RBrace tokens
                    let is_jsx_close = self.brace_depth == baseline + 1;
                    let token = if is_jsx_close {
                        // Set flag to allow delimiters as JSX text after closing a JSX expression
                        self.just_closed_jsx_expr = true;
                        Token::new(TokenKind::JsxCloseBrace, "}".to_string(), self.line, start_col)
                    } else {
                        Token::new(TokenKind::RBrace, "}".to_string(), self.line, start_col)
                    };
                    self.brace_depth -= 1;
                    token
                } else {
                    Token::new(TokenKind::RBrace, "}".to_string(), self.line, start_col)
                }
            }
            '[' => Token::new(TokenKind::LBracket, "[".to_string(), self.line, start_col),
            ']' => Token::new(TokenKind::RBracket, "]".to_string(), self.line, start_col),
            '<' => {
                if self.peek() == '=' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::LtEq, "<=".to_string(), self.line, start_col);
                } else {
                    // Check if this might be JSX: < followed by an alphabetic character or uppercase
                    // This handles <div>, <Component>, etc.
                    // Always set jsx_in_tag when we see <, as the parser will enable JSX mode if needed
                    self.jsx_in_tag = true;
                    Token::new(TokenKind::LAngle, "<".to_string(), self.line, start_col)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::GtEq, ">=".to_string(), self.line, start_col);
                } else {
                    // Only mark that we're exiting a tag if we're at the baseline brace depth
                    // This prevents `>` comparison operators inside attribute expressions from incorrectly
                    // setting jsx_in_tag = false
                    let baseline = self.jsx_baseline_brace_depths.last().copied().unwrap_or(0);
                    if self.brace_depth == baseline {
                        self.jsx_in_tag = false;
                    }
                    Token::new(TokenKind::RAngle, ">".to_string(), self.line, start_col)
                }
            }
            '/' => {
                // Check for self-closing JSX tag />
                if self.peek() == '>' && self.jsx_mode {
                    self.read_char();
                    self.read_char();
                    // Don't automatically decrement jsx_depth here - let the parser manage it
                    // via exit_jsx_mode() based on whether this element entered JSX mode
                    // Mark that we're exiting a tag
                    self.jsx_in_tag = false;
                    return Token::new(TokenKind::JsxSelfClose, "/>".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Slash, "/".to_string(), self.line, start_col)
                }
            }
            '-' => {
                if self.peek() == '>' {
                    self.read_char();
                    self.read_char();
                    return Token::new(TokenKind::Arrow, "->".to_string(), self.line, start_col);
                } else {
                    Token::new(TokenKind::Minus, "-".to_string(), self.line, start_col)
                }
            }
            '@' => {
                // Check if in CSS mode and if this is @media or @keyframes
                if self.is_css_mode() {
                    let pos = self.position;
                    self.read_char(); // consume '@'
                    let ident_token = self.read_identifier();

                    match ident_token.lexeme.as_str() {
                        "media" => {
                            return Token::new(TokenKind::CssMedia, "@media".to_string(), self.line, start_col);
                        }
                        "keyframes" => {
                            return Token::new(TokenKind::CssKeyframes, "@keyframes".to_string(), self.line, start_col);
                        }
                        _ => {
                            // Not a recognized @-rule, reset
                            self.position = pos;
                            self.ch = '@';
                        }
                    }
                }
                Token::new(TokenKind::At, "@".to_string(), self.line, start_col)
            }
            '\0' => Token::new(TokenKind::Eof, "".to_string(), self.line, start_col),
            '"' => return self.read_string(),
            '\'' => {
                // Check if this is a lifetime (e.g., 'a, 'static)
                if self.peek().is_alphabetic() || self.peek() == '_' {
                    return self.read_lifetime();
                } else {
                    // For now, treat single quote without identifier as illegal
                    Token::new(TokenKind::Illegal(self.ch), self.ch.to_string(), self.line, start_col)
                }
            }
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    return self.read_identifier();
                } else if self.ch.is_ascii_digit() {
                    return self.read_number();
                } else {
                    Token::new(TokenKind::Illegal(self.ch), self.ch.to_string(), self.line, start_col)
                }
            }
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        if self.ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char();
            } else if self.ch == '/' && self.peek() == '/' {
                // Skip line comment //
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else if self.ch == '/' && self.peek() == '*' {
                // Skip block comment /* */
                self.read_char(); // consume /
                self.read_char(); // consume *
                while !(self.ch == '*' && self.peek() == '/') && self.ch != '\0' {
                    self.read_char();
                }
                if self.ch == '*' {
                    self.read_char(); // consume *
                    self.read_char(); // consume /
                }
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_pos = self.position;
        let start_col = self.column;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        let literal: String = self.input[start_pos..self.position].iter().collect();

        // Check for css! macro
        if literal == "css" && self.ch == '!' {
            self.read_char(); // consume !
            return Token::new(TokenKind::CssMacro, "css!".to_string(), self.line, start_col);
        }

        // Check for boolean literals
        let kind = match literal.as_str() {
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            _ => KEYWORDS.get(literal.as_str()).cloned().unwrap_or(TokenKind::Identifier),
        };

        Token::new(kind, literal, self.line, start_col)
    }

    fn read_number(&mut self) -> Token {
        let start_pos = self.position;
        let start_col = self.column;
        let mut is_float = false;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        // Check for decimal point
        if self.ch == '.' && self.peek().is_ascii_digit() {
            is_float = true;
            self.read_char(); // consume '.'
            while self.ch.is_ascii_digit() {
                self.read_char();
            }
        }

        let literal: String = self.input[start_pos..self.position].iter().collect();

        if is_float {
            Token::new(TokenKind::Float(literal.clone()), literal, self.line, start_col)
        } else {
            let value = literal.parse().unwrap_or(0);
            Token::new(TokenKind::Integer(value), literal, self.line, start_col)
        }
    }
    
    fn read_string(&mut self) -> Token {
        let start_col = self.column;
        self.read_char(); // Consume opening '"'

        let mut result = String::new();

        while self.ch != '"' && self.ch != '\0' {
            if self.ch == '\\' {
                // Handle escape sequences
                self.read_char(); // consume backslash
                match self.ch {
                    'n' => result.push('\n'),   // newline
                    't' => result.push('\t'),   // tab
                    'r' => result.push('\r'),   // carriage return
                    '\\' => result.push('\\'),  // backslash
                    '"' => result.push('"'),    // quote
                    '\'' => result.push('\''),  // single quote
                    '0' => result.push('\0'),   // null
                    _ => {
                        // Unknown escape sequence - include backslash and char
                        result.push('\\');
                        result.push(self.ch);
                    }
                }
                self.read_char();
            } else {
                result.push(self.ch);
                self.read_char();
            }
        }

        let token = Token::new(TokenKind::String(result.clone()), result, self.line, start_col);
        self.read_char(); // Consume closing '"'
        token
    }

    fn read_lifetime(&mut self) -> Token {
        let start_pos = self.position;
        let start_col = self.column;

        self.read_char(); // Consume the '

        // Read the lifetime name (identifier after the ')
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }

        let literal: String = self.input[start_pos..self.position].iter().collect();
        // Extract the lifetime name without the leading quote
        let lifetime_name = literal[1..].to_string();

        Token::new(TokenKind::Lifetime(lifetime_name.clone()), literal, self.line, start_col)
    }

    fn read_jsx_text(&mut self) -> Token {
        let start_col = self.column;
        let mut result = String::new();

        // Read text until we hit < (tag start), { (expression start), } (expression end), or end of input
        while self.ch != '<' && self.ch != '{' && self.ch != '}' && self.ch != '\0' {
            result.push(self.ch);
            self.read_char();
        }

        // Trim the result to remove extra whitespace (but preserve intentional spacing)
        let trimmed = result.trim().to_string();

        Token::new(TokenKind::JsxText(trimmed.clone()), trimmed, self.line, start_col)
    }

    // Public methods for parser to manage JSX mode
    pub fn enter_jsx_mode(&mut self) {
        self.jsx_mode = true;
        self.jsx_depth += 1;
        // Record the current brace depth as the baseline for this JSX element
        self.jsx_baseline_brace_depths.push(self.brace_depth);
    }

    // Enter nested JSX (already in jsx_mode, just track nesting)
    pub fn enter_nested_jsx(&mut self) {
        self.jsx_depth += 1;
        // Push current brace depth as baseline for this nested JSX element
        // This is CRITICAL for JSX inside expressions like: {cond ? (<div>...</div>) : ...}
        self.jsx_baseline_brace_depths.push(self.brace_depth);
    }

    pub fn exit_jsx_mode(&mut self) {
        if self.jsx_depth > 0 {
            self.jsx_depth -= 1;
            // Pop the baseline brace depth for this JSX element
            self.jsx_baseline_brace_depths.pop();
        }
        if self.jsx_depth == 0 {
            self.jsx_mode = false;
        }
    }

    pub fn is_jsx_mode(&self) -> bool {
        self.jsx_mode
    }

    pub fn enter_closing_tag_mode(&mut self) {
        self.in_closing_tag = true;
    }

    pub fn exit_closing_tag_mode(&mut self) {
        self.in_closing_tag = false;
    }

    pub fn increment_brace_depth(&mut self) {
        self.brace_depth += 1;
    }

    pub fn decrement_brace_depth(&mut self) {
        if self.brace_depth > 0 {
            self.brace_depth -= 1;
        }
    }

    // CSS mode management
    pub fn enter_css_mode(&mut self) {
        self.css_mode = true;
        self.css_depth = 1; // Start at depth 1 (first opening brace)
    }

    pub fn exit_css_mode(&mut self) {
        self.css_mode = false;
        self.css_depth = 0;
        self.css_paren_depth = 0;
    }

    pub fn is_css_mode(&self) -> bool {
        self.css_mode
    }

    // Read a CSS selector (.button, #id, div, .button:hover, .card .title, etc.)
    fn read_css_selector(&mut self) -> Token {
        let start_col = self.column;
        let start_pos = self.position;

        // Read selector until we hit { (which indicates start of declarations)
        // This allows for nested selectors like ".card .title"
        while self.ch != '{' && self.ch != '\0' && self.ch != '\n' {
            self.read_char();
        }

        // Trim whitespace from the end
        let mut end_pos = self.position;
        while end_pos > start_pos && self.input[end_pos - 1].is_whitespace() {
            end_pos -= 1;
        }

        let selector: String = self.input[start_pos..end_pos].iter().collect();
        Token::new(TokenKind::CssSelector(selector.clone()), selector, self.line, start_col)
    }

    // Read a CSS property name (background, padding, etc.)
    fn read_css_property(&mut self) -> Token {
        let start_col = self.column;
        let start_pos = self.position;

        // Read property name (alphanumeric and hyphens)
        while self.ch.is_alphanumeric() || self.ch == '-' {
            self.read_char();
        }

        let property: String = self.input[start_pos..self.position].iter().collect();
        Token::new(TokenKind::CssProperty(property.clone()), property, self.line, start_col)
    }

    // Read a CSS value (blue, 12px, "Arial", etc.)
    fn read_css_value(&mut self) -> Token {
        let start_col = self.column;
        let start_pos = self.position;

        // Skip leading whitespace
        while self.ch.is_whitespace() && self.ch != '\n' {
            self.read_char();
        }

        // Read until semicolon, closing brace, or newline
        while self.ch != ';' && self.ch != '}' && self.ch != '\0' {
            self.read_char();
        }

        let value: String = self.input[start_pos..self.position].iter().collect();
        let trimmed = value.trim().to_string();
        Token::new(TokenKind::CssValue(trimmed.clone()), trimmed, self.line, start_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_escape_sequences() {
        let input = r#""Hello\nWorld""#.to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "Hello\nWorld");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    #[test]
    fn test_string_tab_escape() {
        let input = r#""Tab\there""#.to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "Tab\there");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    #[test]
    fn test_string_quote_escape() {
        let input = r#""Say \"Hello\"""#.to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "Say \"Hello\"");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    #[test]
    fn test_string_backslash_escape() {
        let input = r#""Path\\to\\file""#.to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "Path\\to\\file");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    #[test]
    fn test_string_multiple_escapes() {
        let input = r#""Line1\nLine2\tTabbed\\Backslash""#.to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "Line1\nLine2\tTabbed\\Backslash");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    #[test]
    fn test_multiline_string() {
        let input = "\"Line 1\nLine 2\nLine 3\"".to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "Line 1\nLine 2\nLine 3");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    #[test]
    fn test_multiline_string_with_indentation() {
        let input = "\"  Indented line 1\n    Indented line 2\n  End\"".to_string();
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();

        if let TokenKind::String(s) = token.kind {
            assert_eq!(s, "  Indented line 1\n    Indented line 2\n  End");
        } else {
            panic!("Expected String token, got {:?}", token.kind);
        }
    }

    // JSX Lexer Tests

    #[test]
    fn test_jsx_simple_text() {
        let input = "Hello World".to_string();
        let mut lexer = Lexer::new(input);

        // Manually enter JSX mode (parser would do this)
        lexer.enter_jsx_mode();

        let token = lexer.next_token();
        assert_eq!(token.kind, TokenKind::JsxText("Hello World".to_string()));
    }

    #[test]
    fn test_jsx_text_with_whitespace() {
        let input = "  Hello World  ".to_string();
        let mut lexer = Lexer::new(input);

        lexer.enter_jsx_mode();

        let token = lexer.next_token();
        // JSX text is trimmed
        assert_eq!(token.kind, TokenKind::JsxText("Hello World".to_string()));
    }

    #[test]
    fn test_jsx_mode_entry_exit() {
        let mut lexer = Lexer::new("test".to_string());

        assert!(!lexer.is_jsx_mode());

        lexer.enter_jsx_mode();
        assert!(lexer.is_jsx_mode());

        lexer.exit_jsx_mode();
        assert!(!lexer.is_jsx_mode());
    }

    #[test]
    fn test_jsx_nested_mode() {
        let mut lexer = Lexer::new("test".to_string());

        // Enter JSX mode twice (nested elements)
        lexer.enter_jsx_mode();
        lexer.enter_jsx_mode();
        assert!(lexer.is_jsx_mode());

        // Exit once - should still be in JSX mode
        lexer.exit_jsx_mode();
        assert!(lexer.is_jsx_mode());

        // Exit again - now should be out
        lexer.exit_jsx_mode();
        assert!(!lexer.is_jsx_mode());
    }

    #[test]
    fn test_jsx_slash_gt_in_code_mode() {
        // Self-closing /> should be recognized when NOT in JSX text mode
        // Parser enters JSX mode only AFTER the opening >, not during attributes
        let input = "/>".to_string();
        let mut lexer = Lexer::new(input);

        // NOT in JSX mode - just reading regular tokens
        let token = lexer.next_token();

        // Without JSX mode, /> is just two separate tokens
        assert_eq!(token.kind, TokenKind::Slash);

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::RAngle);
    }

    #[test]
    fn test_jsx_expression_braces() {
        let input = "{ name }".to_string();
        let mut lexer = Lexer::new(input);

        lexer.enter_jsx_mode();

        // Opening brace
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::JsxOpenBrace);

        // Identifier inside expression
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::Identifier);
        assert_eq!(token2.lexeme, "name");

        // Closing brace
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, TokenKind::JsxCloseBrace);
    }

    #[test]
    fn test_jsx_text_stops_at_tag() {
        let input = "Hello<div".to_string();
        let mut lexer = Lexer::new(input);

        lexer.enter_jsx_mode();

        // Should read "Hello" and stop at <
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::JsxText("Hello".to_string()));

        // Next token should be <
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::LAngle);
    }

    #[test]
    fn test_jsx_text_stops_at_expression() {
        let input = "Hello{name".to_string();
        let mut lexer = Lexer::new(input);

        lexer.enter_jsx_mode();

        // Should read "Hello" and stop at {
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::JsxText("Hello".to_string()));

        // Next token should be {
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::JsxOpenBrace);
    }

    #[test]
    fn test_jsx_angle_brackets_in_code_mode() {
        let input = "a < b".to_string();
        let mut lexer = Lexer::new(input);

        // NOT in JSX mode - should treat < as comparison operator
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::Identifier);

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::LAngle);

        let token3 = lexer.next_token();
        assert_eq!(token3.kind, TokenKind::Identifier);
    }

    #[test]
    fn test_jsx_braces_in_code_mode() {
        let input = "{ let x = 1; }".to_string();
        let mut lexer = Lexer::new(input);

        // NOT in JSX mode - should treat { } as regular braces
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::LBrace);

        lexer.next_token(); // let
        lexer.next_token(); // x
        lexer.next_token(); // =
        lexer.next_token(); // 1
        lexer.next_token(); // ;

        let token_close = lexer.next_token();
        assert_eq!(token_close.kind, TokenKind::RBrace);
    }

    #[test]
    fn test_jsx_nested_expressions() {
        let input = "{ { nested } }".to_string();
        let mut lexer = Lexer::new(input);

        lexer.enter_jsx_mode();

        // First { - JSX expression open (baseline + 1)
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::JsxOpenBrace);

        // Inner { - regular LBrace (baseline + 2)
        // Nested braces inside JSX expressions are treated as regular braces
        // to allow blocks, match expressions, etc.
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::LBrace);

        // Identifier
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, TokenKind::Identifier);

        // Inner } - regular RBrace
        let token4 = lexer.next_token();
        assert_eq!(token4.kind, TokenKind::RBrace);

        // Outer } - JSX close brace (back to baseline)
        let token5 = lexer.next_token();
        assert_eq!(token5.kind, TokenKind::JsxCloseBrace);
    }

    #[test]
    fn test_jsx_closing_tag_detected() {
        // Simulates being inside JSX content and hitting a closing tag
        // <div> [we're here] </div>
        let input = "</div>".to_string();
        let mut lexer = Lexer::new(input);

        // Parser entered JSX mode after reading <div>
        lexer.enter_jsx_mode();

        // When JSX text reading checks ch, it sees '<' so it doesn't read text
        // Instead it returns to normal token matching
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::LAngle);

        // IMPORTANT: Parser would exit JSX mode here after seeing < in JSX content
        // because it indicates either a child element or closing tag
        lexer.exit_jsx_mode();

        // Now not in JSX mode, / is a regular token
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::Slash);

        // div is an identifier
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, TokenKind::Identifier);

        // >
        let token4 = lexer.next_token();
        assert_eq!(token4.kind, TokenKind::RAngle);
    }

    #[test]
    fn test_jsx_multiline_text() {
        let input = "Line 1\nLine 2\nLine 3".to_string();
        let mut lexer = Lexer::new(input);

        lexer.enter_jsx_mode();

        let token = lexer.next_token();
        assert_eq!(token.kind, TokenKind::JsxText("Line 1\nLine 2\nLine 3".to_string()));
    }

    #[test]
    fn test_css_macro_recognition() {
        let input = "css!".to_string();
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_eq!(token.kind, TokenKind::CssMacro);
        assert_eq!(token.lexeme, "css!");
    }

    #[test]
    fn test_css_basic_rule() {
        let input = r#"css! {
            .button {
                background: blue;
                padding: 12px;
            }
        }"#.to_string();

        let mut lexer = Lexer::new(input);

        // css!
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, TokenKind::CssMacro);

        // {
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, TokenKind::LBrace);

        // Enter CSS mode manually for testing
        lexer.enter_css_mode();

        // .button
        let token3 = lexer.next_token();
        assert!(matches!(token3.kind, TokenKind::CssSelector(_)));

        // {
        let token4 = lexer.next_token();
        assert_eq!(token4.kind, TokenKind::LBrace);

        // background
        let token5 = lexer.next_token();
        assert!(matches!(token5.kind, TokenKind::CssProperty(_)));

        // :
        let token6 = lexer.next_token();
        assert_eq!(token6.kind, TokenKind::Colon);

        // blue
        let token7 = lexer.next_token();
        assert!(matches!(token7.kind, TokenKind::CssValue(_)));

        // ;
        let token8 = lexer.next_token();
        assert_eq!(token8.kind, TokenKind::Semicolon);
    }
}