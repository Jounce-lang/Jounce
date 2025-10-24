# JSX AST Guide for Parser Developers

**Date**: 2025-10-21 (Day 6)
**Status**: JSX AST Complete and Documented

---

## Overview

The Jounce AST has complete JSX support with well-designed node types. The parser can represent any JSX syntax using the AST nodes defined in `src/ast.rs`. This guide explains the AST structure and how to construct JSX nodes during parsing.

---

## JSX AST Node Types

### 1. `JsxElement` - Complete JSX Element

The main JSX node type representing a complete element.

```rust
pub struct JsxElement {
    pub opening_tag: JsxOpeningTag,
    pub children: Vec<JsxChild>,
    pub closing_tag: Option<Identifier>,
}
```

**Fields**:
- `opening_tag`: The opening tag (`<tag>` or `<tag/>`)
- `children`: Child nodes (elements, text, expressions)
- `closing_tag`: Closing tag name (None for self-closing)

**Examples**:
```jsx
// Regular element: <div>Hello</div>
JsxElement {
    opening_tag: JsxOpeningTag { name: "div", attributes: [], self_closing: false },
    children: [JsxChild::Text("Hello")],
    closing_tag: Some("div"),
}

// Self-closing: <img />
JsxElement {
    opening_tag: JsxOpeningTag { name: "img", attributes: [], self_closing: true },
    children: [],
    closing_tag: None,
}
```

**Helper Methods**:
```rust
// Create new element
let mut elem = JsxElement::new("div".to_string());

// Create self-closing element
let img = JsxElement::new_self_closing("img".to_string());

// Query methods
elem.is_self_closing() // -> bool
elem.tag_name()        // -> &str

// Modification methods
elem.add_child(JsxChild::Text("Hello".to_string()));
elem.add_text("World".to_string());
elem.add_attribute("class".to_string(), Expression::StringLiteral("container".to_string()));
```

---

### 2. `JsxOpeningTag` - Opening Tag Information

Represents the opening part of a JSX element.

```rust
pub struct JsxOpeningTag {
    pub name: Identifier,
    pub attributes: Vec<JsxAttribute>,
    pub self_closing: bool,
}
```

**Fields**:
- `name`: Tag name (lowercase for HTML, PascalCase for components)
- `attributes`: List of attributes/props
- `self_closing`: True if ends with `/>`

**Examples**:
```jsx
// <div class="x" id="y">
JsxOpeningTag {
    name: Identifier { value: "div" },
    attributes: [
        JsxAttribute { name: "class", value: StringLiteral("x") },
        JsxAttribute { name: "id", value: StringLiteral("y") },
    ],
    self_closing: false,
}

// <Button onClick={handler} />
JsxOpeningTag {
    name: Identifier { value: "Button" },
    attributes: [
        JsxAttribute { name: "onClick", value: Identifier("handler") },
    ],
    self_closing: true,
}
```

---

### 3. `JsxChild` - Child Nodes

Represents possible children within JSX content.

```rust
pub enum JsxChild {
    Element(Box<JsxElement>),  // Nested element
    Text(String),              // Plain text
    Expression(Box<Expression>), // {expr} interpolation
}
```

**Variants**:

#### `Element` - Nested JSX Element
```jsx
<div>
  <span>nested</span>
</div>

// AST:
JsxChild::Element(Box::new(JsxElement { ... }))
```

#### `Text` - Plain Text Content
```jsx
<div>Hello World</div>

// AST:
JsxChild::Text("Hello World".to_string())
```

**Note**: Text is automatically read by the lexer when in JSX mode. The lexer trims whitespace but preserves multiline text.

#### `Expression` - Interpolated Expression
```jsx
<div>{name}</div>
<div>{count + 1}</div>

// AST:
JsxChild::Expression(Box::new(Expression::Identifier(...)))
JsxChild::Expression(Box::new(Expression::Infix(...)))
```

**Parser Flow**:
1. See `TokenKind::JsxOpenBrace` (`{`)
2. Parse expression inside
3. See `TokenKind::JsxCloseBrace` (`}`)
4. Wrap in `JsxChild::Expression`

---

### 4. `JsxAttribute` - Attributes and Props

Represents JSX attributes (HTML attributes or React props).

```rust
pub struct JsxAttribute {
    pub name: Identifier,
    pub value: Expression,
}
```

**Attribute Types**:

#### String Literals
```jsx
class="container"

// AST:
JsxAttribute {
    name: Identifier { value: "class" },
    value: Expression::StringLiteral("container".to_string()),
}
```

#### Expressions
```jsx
value={count}

// AST:
JsxAttribute {
    name: Identifier { value: "value" },
    value: Expression::Identifier(Identifier { value: "count" }),
}
```

#### Event Handlers
```jsx
onClick={handleClick}
onClick={() => console.log("clicked")}

// AST:
JsxAttribute {
    name: Identifier { value: "onClick" },
    value: Expression::Identifier(...) // or Expression::Lambda(...)
}
```

#### Boolean Attributes
```jsx
disabled
checked

// AST:
JsxAttribute {
    name: Identifier { value: "disabled" },
    value: Expression::BoolLiteral(true),
}
```

**Helper Methods**:
```rust
// String attribute
JsxAttribute::new_string("class".to_string(), "btn".to_string())

// Expression attribute
JsxAttribute::new_expr("value".to_string(), Expression::Identifier(...))

// Boolean attribute
JsxAttribute::new_bool("disabled".to_string())

// Check if event handler
attr.is_event_handler() // -> bool (checks if name starts with "on")
```

---

## Parser Integration

### Complete Parsing Example

```jsx
<div class="container">
  Hello {name}!
  <span>nested</span>
</div>
```

**Resulting AST**:
```rust
Expression::JsxElement(JsxElement {
    opening_tag: JsxOpeningTag {
        name: Identifier { value: "div".to_string() },
        attributes: vec![
            JsxAttribute {
                name: Identifier { value: "class".to_string() },
                value: Expression::StringLiteral("container".to_string()),
            },
        ],
        self_closing: false,
    },
    children: vec![
        JsxChild::Text("Hello ".to_string()),
        JsxChild::Expression(Box::new(
            Expression::Identifier(Identifier { value: "name".to_string() })
        )),
        JsxChild::Text("!".to_string()),
        JsxChild::Element(Box::new(JsxElement {
            opening_tag: JsxOpeningTag {
                name: Identifier { value: "span".to_string() },
                attributes: vec![],
                self_closing: false,
            },
            children: vec![
                JsxChild::Text("nested".to_string()),
            ],
            closing_tag: Some(Identifier { value: "span".to_string() }),
        })),
    ],
    closing_tag: Some(Identifier { value: "div".to_string() }),
})
```

---

## Parsing Patterns

### Pattern 1: Parse Opening Tag

```rust
fn parse_jsx_opening_tag(&mut self) -> Result<JsxOpeningTag, ParserError> {
    // Expect <
    self.expect(TokenKind::LAngle)?;

    // Get tag name
    let name = self.expect_identifier()?;

    // Parse attributes
    let mut attributes = Vec::new();
    while !matches!(self.current_token().kind,
                    TokenKind::RAngle | TokenKind::Slash) {
        attributes.push(self.parse_jsx_attribute()?);
    }

    // Check for self-closing
    let self_closing = if self.current_token().kind == TokenKind::Slash {
        self.advance(); // consume /
        true
    } else {
        false
    };

    // Expect >
    self.expect(TokenKind::RAngle)?;

    Ok(JsxOpeningTag { name, attributes, self_closing })
}
```

### Pattern 2: Parse Attribute

```rust
fn parse_jsx_attribute(&mut self) -> Result<JsxAttribute, ParserError> {
    // Get attribute name
    let name = self.expect_identifier()?;

    // Check for = (value)
    if self.current_token().kind != TokenKind::Assign {
        // Boolean attribute: just the name
        return Ok(JsxAttribute::new_bool(name.value));
    }

    self.advance(); // consume =

    // Parse value
    let value = match self.current_token().kind {
        TokenKind::String(ref s) => {
            let val = Expression::StringLiteral(s.clone());
            self.advance();
            val
        }
        TokenKind::JsxOpenBrace => {
            self.advance(); // consume {
            let expr = self.parse_expression()?;
            self.expect(TokenKind::JsxCloseBrace)?;
            expr
        }
        _ => return Err(ParserError::UnexpectedToken(...)),
    };

    Ok(JsxAttribute { name, value })
}
```

### Pattern 3: Parse Children

```rust
fn parse_jsx_children(&mut self) -> Result<Vec<JsxChild>, ParserError> {
    let mut children = Vec::new();

    // Enter JSX mode so lexer reads text automatically
    self.lexer.enter_jsx_mode();

    loop {
        match self.current_token().kind {
            // Text content
            TokenKind::JsxText(ref text) => {
                if !text.is_empty() {
                    children.push(JsxChild::Text(text.clone()));
                }
                self.advance();
            }

            // Expression {expr}
            TokenKind::JsxOpenBrace => {
                self.advance(); // consume {
                let expr = self.parse_expression()?;
                self.expect(TokenKind::JsxCloseBrace)?;
                children.push(JsxChild::Expression(Box::new(expr)));
            }

            // Nested element or closing tag
            TokenKind::LAngle => {
                self.lexer.exit_jsx_mode(); // Exit before checking tag

                // Check if it's a closing tag
                if self.peek_token().kind == TokenKind::Slash {
                    break; // End of children
                }

                // Parse nested element
                let child_element = self.parse_jsx_element()?;
                children.push(JsxChild::Element(Box::new(child_element)));

                self.lexer.enter_jsx_mode(); // Re-enter for more content
            }

            _ => break,
        }
    }

    // Make sure we've exited JSX mode
    if self.lexer.is_jsx_mode() {
        self.lexer.exit_jsx_mode();
    }

    Ok(children)
}
```

### Pattern 4: Complete Element

```rust
fn parse_jsx_element(&mut self) -> Result<Expression, ParserError> {
    let opening_tag = self.parse_jsx_opening_tag()?;

    // If self-closing, we're done
    if opening_tag.self_closing {
        return Ok(Expression::JsxElement(JsxElement {
            opening_tag,
            children: vec![],
            closing_tag: None,
        }));
    }

    // Parse children
    let children = self.parse_jsx_children()?;

    // Parse closing tag
    let closing_tag = self.parse_jsx_closing_tag(&opening_tag.name.value)?;

    Ok(Expression::JsxElement(JsxElement {
        opening_tag,
        children,
        closing_tag: Some(closing_tag),
    }))
}
```

---

## Common Patterns

### Using Helper Methods

```rust
// Build JSX programmatically
let mut div = JsxElement::new("div".to_string());
div.add_attribute("class".to_string(), Expression::StringLiteral("container".to_string()));
div.add_text("Hello".to_string());

// Add a nested element
let mut span = JsxElement::new("span".to_string());
span.add_text("world".to_string());
div.add_child(JsxChild::Element(Box::new(span)));

// Result: <div class="container">Hello<span>world</span></div>
```

### Creating Attributes

```rust
// String attribute
let class_attr = JsxAttribute::new_string("class".to_string(), "btn".to_string());

// Expression attribute
let value_attr = JsxAttribute::new_expr(
    "value".to_string(),
    Expression::Identifier(Identifier { value: "count".to_string() })
);

// Boolean attribute
let disabled_attr = JsxAttribute::new_bool("disabled".to_string());

// Event handler
let click_attr = JsxAttribute::new_expr(
    "onClick".to_string(),
    Expression::Lambda(...)
);

// Check if event handler
if click_attr.is_event_handler() {
    // Special handling for events
}
```

---

## Validation Rules

### Tag Name Matching

```rust
fn validate_jsx_element(elem: &JsxElement) -> Result<(), ParserError> {
    // Self-closing elements should have no closing tag
    if elem.is_self_closing() && elem.closing_tag.is_some() {
        return Err(ParserError::InvalidJsx(
            "Self-closing element cannot have closing tag".to_string()
        ));
    }

    // Regular elements must have matching closing tag
    if !elem.is_self_closing() {
        match &elem.closing_tag {
            None => return Err(ParserError::MissingClosingTag(
                elem.tag_name().to_string()
            )),
            Some(closing) => {
                if closing.value != elem.tag_name() {
                    return Err(ParserError::MismatchedClosingTag {
                        expected: elem.tag_name().to_string(),
                        found: closing.value.clone(),
                    });
                }
            }
        }
    }

    Ok(())
}
```

### Self-Closing Tags

Self-closing elements must have:
- `opening_tag.self_closing = true`
- `children = vec![]` (empty)
- `closing_tag = None`

Regular elements must have:
- `opening_tag.self_closing = false`
- `closing_tag = Some(tag_name)`

---

## Integration with Code Generation

The codegen uses these AST nodes to generate JavaScript/WASM code:

```rust
// In src/codegen.rs
fn generate_jsx_element(&mut self, jsx: &JsxElement, f: &mut Function) -> Result<(), CompileError> {
    // Convert AST to runtime VNode
    let vnode = self.jsx_to_vnode(jsx)?;

    // Generate WASM instructions for VNode
    self.generate_vnode(&vnode, f)?;

    Ok(())
}
```

See `src/codegen.rs` lines 1264-1324 for full implementation.

---

## Examples

### Example 1: Simple Element

```jsx
<div>Hello</div>
```

**Parser Actions**:
1. See `<` → call `parse_jsx_element()`
2. Parse opening tag: `div`, no attributes
3. Enter JSX mode
4. Read text: `"Hello"`
5. See `<` → exit JSX mode
6. Parse closing tag: `/div`
7. Construct `JsxElement`

**AST**:
```rust
JsxElement {
    opening_tag: JsxOpeningTag {
        name: Identifier { value: "div" },
        attributes: vec![],
        self_closing: false,
    },
    children: vec![JsxChild::Text("Hello".to_string())],
    closing_tag: Some(Identifier { value: "div" }),
}
```

### Example 2: Self-Closing with Attributes

```jsx
<img src="photo.jpg" alt="Photo" />
```

**AST**:
```rust
JsxElement {
    opening_tag: JsxOpeningTag {
        name: Identifier { value: "img" },
        attributes: vec![
            JsxAttribute::new_string("src", "photo.jpg"),
            JsxAttribute::new_string("alt", "Photo"),
        ],
        self_closing: true,
    },
    children: vec![],
    closing_tag: None,
}
```

### Example 3: With Expressions

```jsx
<div class="user">
  Welcome {name}!
</div>
```

**AST**:
```rust
JsxElement {
    opening_tag: JsxOpeningTag {
        name: Identifier { value: "div" },
        attributes: vec![
            JsxAttribute::new_string("class", "user"),
        ],
        self_closing: false,
    },
    children: vec![
        JsxChild::Text("Welcome ".to_string()),
        JsxChild::Expression(Box::new(
            Expression::Identifier(Identifier { value: "name" })
        )),
        JsxChild::Text("!".to_string()),
    ],
    closing_tag: Some(Identifier { value: "div" }),
}
```

---

## Testing

Currently there are no unit tests for AST construction. Tests should be added in parser tests when parsing is implemented.

**Recommended Tests**:
```rust
#[test]
fn test_jsx_element_creation() {
    let elem = JsxElement::new("div".to_string());
    assert_eq!(elem.tag_name(), "div");
    assert!(!elem.is_self_closing());
}

#[test]
fn test_jsx_self_closing() {
    let elem = JsxElement::new_self_closing("img".to_string());
    assert!(elem.is_self_closing());
    assert_eq!(elem.closing_tag, None);
}

#[test]
fn test_jsx_add_children() {
    let mut elem = JsxElement::new("div".to_string());
    elem.add_text("Hello".to_string());
    assert_eq!(elem.children.len(), 1);
}

#[test]
fn test_jsx_attribute_event_handler() {
    let attr = JsxAttribute::new_expr(
        "onClick".to_string(),
        Expression::Identifier(Identifier { value: "handler" })
    );
    assert!(attr.is_event_handler());
}
```

---

## Next Steps

Now that the AST is complete and documented, the next phase is parser implementation:

**Days 7-11: Parser Functions** (see `JSX_LEXER_USAGE.md` for lexer details)
1. Implement `parse_jsx_element()`
2. Implement `parse_jsx_opening_tag()`
3. Implement `parse_jsx_attribute()`
4. Implement `parse_jsx_children()`
5. Implement `parse_jsx_closing_tag()`
6. Add validation
7. Write comprehensive tests

---

## Reference

- **AST Definitions**: `src/ast.rs` lines 301-482
- **Code Generation**: `src/codegen.rs` lines 1264-1324
- **Lexer Integration**: See `JSX_LEXER_USAGE.md`
- **VDOM Runtime**: `src/vdom.rs`

---

**Status**: ✅ JSX AST Complete and Documented
**Helper Methods**: 13 methods added
**Documentation**: Complete inline docs + this guide
**Tests**: 197/197 passing (AST doesn't break existing code)
**Next Phase**: Parser Implementation (Days 7-11)

---

_Document created: Day 6 (2025-10-21)_
_Last updated: Day 6 (2025-10-21)_
