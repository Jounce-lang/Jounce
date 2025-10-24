# JSX Lexer Usage Guide for Parser Developers

**Date**: 2025-10-21 (Day 5)
**Status**: JSX Lexer Complete and Tested

---

## Overview

The Jounce lexer has full JSX support built-in. The lexer can tokenize JSX elements, text content, expressions, and self-closing tags. **The parser controls when JSX mode is active** using the lexer's public API.

---

## Lexer JSX API

The lexer provides three public methods for JSX mode management:

```rust
// Enter JSX mode (increment depth)
pub fn enter_jsx_mode(&mut self);

// Exit JSX mode (decrement depth)
pub fn exit_jsx_mode(&mut self);

// Check if currently in JSX mode
pub fn is_jsx_mode(&self) -> bool;
```

---

## JSX Token Types

The lexer produces these JSX-specific tokens:

```rust
TokenKind::JsxText(String)       // Text content between tags
TokenKind::JsxSelfClose          // />
TokenKind::JsxOpenBrace          // { (in JSX context)
TokenKind::JsxCloseBrace         // } (in JSX context)
TokenKind::LAngle                // <
TokenKind::RAngle                // >
TokenKind::Slash                 // /
```

---

## Parser Usage Pattern

### Opening Tag: `<div>`

```
1. See < (LAngle)
2. Read identifier "div"
3. Read attributes (if any)
4. See > (RAngle)
5. **Call lexer.enter_jsx_mode()**
6. Now inside tag - can read text content or child elements
```

### Self-Closing Tag: `<img />`

```
1. See < (LAngle)
2. Read identifier "img"
3. Read attributes (if any)
4. See / (Slash)
5. See > (RAngle) forming />
6. **DO NOT enter JSX mode** (no content to read)
```

### Closing Tag: `</div>`

```
1. **In JSX mode**, see < (LAngle)
2. **Call lexer.exit_jsx_mode()** (exiting this element)
3. Read / (Slash)
4. Read identifier "div"
5. See > (RAngle)
```

### JSX Text Content

```
When in JSX mode with brace_depth == 0:
- Text is automatically read until < or { is encountered
- Returns TokenKind::JsxText(String)
- Whitespace is trimmed but preserved in multiline

Example: <div>Hello World</div>
After <div>, lexer.enter_jsx_mode()
Next token: JsxText("Hello World")
Next token: LAngle (start of </div>)
```

### JSX Expressions: `{variable}`

```
When in JSX mode:
{ → TokenKind::JsxOpenBrace (increments brace_depth)
  ...regular tokens inside expression...
} → TokenKind::JsxCloseBrace (decrements brace_depth)

Example: <div>{name}</div>
JsxText if any
JsxOpenBrace
Identifier("name")
JsxCloseBrace
JsxText if any
```

---

## Critical Rules for Parser

### Rule 1: Enter JSX Mode After Opening `>`

```rust
// ✅ CORRECT
match self.current_token().kind {
    TokenKind::RAngle => {
        self.advance(); // consume >
        self.lexer.enter_jsx_mode(); // NOW enter JSX mode
        // Read JSX content...
    }
}

// ❌ WRONG
match self.current_token().kind {
    TokenKind::LAngle => {
        self.lexer.enter_jsx_mode(); // Too early!
        // This will cause attributes to be read as JSX text
    }
}
```

### Rule 2: Exit JSX Mode When Seeing `<` in Content

```rust
// When reading JSX content and you see <
if self.lexer.is_jsx_mode() && self.current_token().kind == TokenKind::LAngle {
    self.lexer.exit_jsx_mode(); // Exit before parsing child/closing tag
    // Now parse the nested element or closing tag
}
```

### Rule 3: Never Enter JSX Mode for Self-Closing Tags

```rust
// When you see />
if self.current_token().kind == TokenKind::Slash
    && self.peek_token().kind == TokenKind::RAngle {
    self.advance(); // /
    self.advance(); // >
    // DO NOT call enter_jsx_mode() - there's no content
}
```

### Rule 4: Track Nesting Depth

```rust
// The lexer automatically tracks depth via enter/exit calls
// Multiple enter_jsx_mode() calls = nested elements
// Each exit_jsx_mode() decrements until depth reaches 0

// Example: <div><span>text</span></div>
enter_jsx_mode()  // depth=1, jsx_mode=true
  enter_jsx_mode()  // depth=2, jsx_mode=true
  exit_jsx_mode()   // depth=1, jsx_mode=true
exit_jsx_mode()   // depth=0, jsx_mode=false
```

---

## Common Patterns

### Pattern 1: Parse Opening Tag

```rust
fn parse_jsx_opening_tag(&mut self) -> Result<String, ParserError> {
    self.expect(TokenKind::LAngle)?;
    let tag_name = self.expect_identifier()?;

    // Parse attributes...
    while !matches!(self.current_token().kind,
                    TokenKind::RAngle | TokenKind::Slash) {
        self.parse_jsx_attribute()?;
    }

    // Check for self-closing
    if self.current_token().kind == TokenKind::Slash {
        self.advance(); // /
        self.expect(TokenKind::RAngle)?;
        // Self-closing - DON'T enter JSX mode
        return Ok(tag_name);
    }

    // Regular opening tag
    self.expect(TokenKind::RAngle)?;
    self.lexer.enter_jsx_mode(); // Enter NOW

    Ok(tag_name)
}
```

### Pattern 2: Parse JSX Content

```rust
fn parse_jsx_children(&mut self) -> Result<Vec<JsxChild>, ParserError> {
    let mut children = Vec::new();

    loop {
        match self.current_token().kind {
            // Text content
            TokenKind::JsxText(ref text) => {
                children.push(JsxChild::Text(text.clone()));
                self.advance();
            }

            // Expression {expr}
            TokenKind::JsxOpenBrace => {
                children.push(self.parse_jsx_expression()?);
            }

            // Child element or closing tag
            TokenKind::LAngle => {
                self.lexer.exit_jsx_mode(); // Exit before checking

                // Check if it's a closing tag
                if self.peek_token().kind == TokenKind::Slash {
                    break; // End of children
                }

                // It's a child element
                children.push(self.parse_jsx_element()?);
                self.lexer.enter_jsx_mode(); // Re-enter for more content
            }

            _ => break,
        }
    }

    Ok(children)
}
```

### Pattern 3: Parse Closing Tag

```rust
fn parse_jsx_closing_tag(&mut self, expected_tag: &str) -> Result<(), ParserError> {
    // Parser should have already exited JSX mode
    self.expect(TokenKind::LAngle)?;
    self.expect(TokenKind::Slash)?;

    let tag_name = self.expect_identifier()?;
    if tag_name != expected_tag {
        return Err(ParserError::MismatchedClosingTag {
            expected: expected_tag.to_string(),
            found: tag_name,
        });
    }

    self.expect(TokenKind::RAngle)?;
    Ok(())
}
```

---

## Test Coverage

The lexer has 13 comprehensive tests covering:

1. ✅ Simple JSX text
2. ✅ JSX text with whitespace (trimmed)
3. ✅ JSX mode entry/exit
4. ✅ Nested JSX mode (depth tracking)
5. ✅ Self-closing detection
6. ✅ JSX expression braces {}
7. ✅ Text stops at tag <
8. ✅ Text stops at expression {
9. ✅ Angle brackets in code mode (comparison)
10. ✅ Braces in code mode (blocks)
11. ✅ Nested expressions
12. ✅ Closing tag detection
13. ✅ Multiline text

All tests pass. Run them with:
```bash
cargo test --lib lexer::tests::test_jsx
```

---

## Example: Complete JSX Element Parsing

```rust
// Input: <div class="container">Hello {name}!</div>

// 1. Parse opening tag
TokenKind::LAngle         // <
TokenKind::Identifier     // div
TokenKind::Identifier     // class
TokenKind::Assign         // =
TokenKind::String         // "container"
TokenKind::RAngle         // >
lexer.enter_jsx_mode()    // NOW in JSX mode

// 2. Parse content
TokenKind::JsxText("Hello ") // Automatically read
TokenKind::JsxOpenBrace      // {
TokenKind::Identifier        // name
TokenKind::JsxCloseBrace     // }
TokenKind::JsxText("!")      // Automatically read

// 3. Parse closing tag
TokenKind::LAngle         // <
lexer.exit_jsx_mode()     // Exit JSX mode
TokenKind::Slash          // /
TokenKind::Identifier     // div
TokenKind::RAngle         // >
```

---

## Gotchas and Common Mistakes

### ❌ Mistake 1: Entering JSX Mode Too Early

```rust
// WRONG: Entering before >
self.expect(TokenKind::LAngle)?;
self.lexer.enter_jsx_mode(); // ❌ Too early!
let tag = self.expect_identifier()?;
// Attributes will be read as JSX text!
```

### ❌ Mistake 2: Forgetting to Exit Before Child Elements

```rust
// WRONG: Not exiting before parsing child
if self.current_token().kind == TokenKind::LAngle {
    // Still in JSX mode - will try to read "</div>" as text!
    self.parse_jsx_child_element()?; // ❌
}

// CORRECT:
if self.current_token().kind == TokenKind::LAngle {
    self.lexer.exit_jsx_mode(); // ✅ Exit first
    self.parse_jsx_child_element()?;
}
```

### ❌ Mistake 3: Entering JSX Mode for Self-Closing Tags

```rust
// WRONG: Self-closing tags have no content
if self.current_token().kind == TokenKind::RAngle {
    self.lexer.enter_jsx_mode(); // ❌ Always entering
    // But this might be after /> !
}

// CORRECT: Check for self-closing first
if self.current_token().kind == TokenKind::Slash {
    self.advance();
    self.expect(TokenKind::RAngle)?;
    // DON'T enter JSX mode
} else {
    self.expect(TokenKind::RAngle)?;
    self.lexer.enter_jsx_mode(); // ✅ Only for regular tags
}
```

---

## Debugging Tips

### Check JSX Mode State

```rust
println!("JSX mode: {}, Current token: {:?}",
         self.lexer.is_jsx_mode(),
         self.current_token().kind);
```

### Verify Depth Tracking

```rust
// The lexer tracks depth internally
// Each enter increments, each exit decrements
// Mode is disabled when depth reaches 0
```

### Test with Simple Cases First

```
✅ Start with: <div></div>
✅ Then: <div>text</div>
✅ Then: <div>{expr}</div>
✅ Finally: <div><span>nested</span></div>
```

---

## Next Steps for Parser Implementation

Now that the lexer is complete, the parser needs:

1. **JSX AST Nodes** (Days 6-7)
   - `JsxElement` expression variant
   - `JsxAttribute` struct
   - `JsxChild` enum (Element, Text, Expression)

2. **JSX Parsing Functions** (Days 8-11)
   - `parse_jsx_element()`
   - `parse_jsx_attributes()`
   - `parse_jsx_children()`
   - `parse_jsx_expression()`

3. **JSX Code Generation** (Days 12-14)
   - `generate_jsx_element()`
   - Emit `createElement()` calls
   - Handle event handlers
   - Generate VDOM structure

---

**Status**: ✅ Lexer JSX Support Complete
**Tests**: 13/13 passing
**Next Phase**: Parser JSX Implementation (Days 6-11)

---

_Document created: Day 5 (2025-10-21)_
_Last updated: Day 5 (2025-10-21)_
