# Jounce Parser Limitations

This document outlines the current state of the Jounce parser, what works, what doesn't, and what would be needed to implement full JSX support.

## Current State: What Works ✅

The Jounce compiler successfully parses and compiles **non-JSX Jounce code**. This includes:

- Basic data structures (structs, enums)
- Functions and methods
- Pattern matching
- Variable declarations
- Basic expressions
- Control flow (if/else, loops)

### Example: Compiles Successfully

```rust
// examples/enum_simple.jnc
enum Result {
    Ok(i32),
    Err(String),
}

fn process(value: i32) -> Result {
    if value > 0 {
        return Result::Ok(value);
    } else {
        return Result::Err("Invalid value");
    }
}

fn main() {
    let result = process(42);
    match result {
        Result::Ok(n) => println!("Success: {}", n),
        Result::Err(e) => println!("Error: {}", e),
    }
}
```

**Compilation**: `./target/release/raven compile examples/enum_simple.jnc` → ✅ SUCCESS

## What Doesn't Work ❌

### 1. JSX/Component Syntax (IMPLEMENTED ✅ - With Known Limitations)

**Status Update (2025-10-21)**: JSX parsing is now fully implemented with 11/11 parser tests passing!

**What Works**:
- ✅ Empty elements: `<div></div>`
- ✅ Text content: `<div>Hello World</div>`
- ✅ Attributes: `<div class="container" id="app"></div>`
- ✅ Self-closing tags: `<img src="photo.jpg" />`
- ✅ Nested elements: `<div><span>nested</span></div>`
- ✅ Expression interpolation: `<div>Hello {name}!</div>`
- ✅ Multiple children: `<div><span>A</span><span>B</span></div>`
- ✅ Deeply nested structures

**Known Limitations**:
- ⚠️ Full compiler JSX parsing in function return statements has edge cases
- ⚠️ Multi-line JSX formatting may have issues
- ✅ Single-line JSX expressions work correctly in tests

**Test Coverage**: 11 dedicated JSX parser tests, all passing

**Files that may still fail**:
- `examples/analytics_dashboard.jnc` - Complex multi-line JSX
- `examples/todo_app.jnc` - Component-based architecture
- `examples/counter_app.jnc` - Multi-line JSX in return
- `examples/devboard/src/main.jnc` - Full application

### 2. Lexer Limitations

#### a. Hash Symbol (`#`)
**Error**: `Lexer Error: Illegal character: '#'`

```rust
// Does NOT work
#[server]
fn get_data() -> Vec<String> { }

// Use instead
@server
fn get_data() -> Vec<String> { }
```

#### b. Emoji Characters
**Error**: `Lexer Error: Illegal character: '⭐'`

```rust
// Does NOT work
let icon = "⭐";

// Use instead
let icon = "star";
```

#### c. Template Literals (Backticks)
**Error**: `Lexer Error: Illegal character: '\`'`

```rust
// Does NOT work
let html = `<div>${name}</div>`;

// Use instead
let html = "<div>" + name + "</div>";
```

### 3. Method Chaining ✅ IMPLEMENTED (Sprint 5)

**Status**: Method chaining is now fully supported!

```rust
// ✅ This now works!
if message.name.trim().is_empty() {
    return Err("Name required");
}

// ✅ String literal chaining works
let result = "test".to_uppercase().trim();

// ✅ Any expression can be chained
arr.filter(x => x > 0).map(x => x * 2).sum();
```

**Implemented in**: Sprint 5 - Ecommerce Parser Fixes (Parser Fix #3)

### 4. String Methods
**Error**: Parser fails on method calls like `.contains()`, `.to_string()`

```rust
// Does NOT work
if email.contains("@") {
    // valid email
}

// String literals don't need .to_string()
let name = "John".to_string(); // Unnecessary

// Use instead
let name = "John"; // Already a String
```

### 5. JSX Comments
**Error**: `ParserError: No prefix parse function for Slash`

```rust
// Does NOT work
<div>
    {/* This is a comment */}
    <h1>Hello</h1>
</div>

// Use instead - remove JSX comments entirely
<div>
    <h1>Hello</h1>
</div>
```

## Technical Requirements for JSX Support

To enable JSX/component syntax in Jounce, the following compiler work is needed:

### 1. Lexer Updates

**File**: `src/lexer.rs`

Add support for:
- JSX opening tags: `<div`
- JSX closing tags: `</div>`
- JSX self-closing tags: `<img />`
- JSX attributes: `class="foo" onclick={handler}`
- JSX children: nested elements and text
- JSX expressions: `{variable}`, `{expr + 1}`
- JSX comments: `{/* comment */}`

**New Token Types Needed**:
```rust
pub enum Token {
    // ... existing tokens

    // JSX tokens
    LessThan,           // <
    GreaterThan,        // >
    Slash,              // /
    JsxText(String),    // Text content between tags
    JsxOpen,            // < (in JSX context)
    JsxClose,           // >
    JsxSelfClose,       // />
}
```

### 2. Parser Updates

**File**: `src/parser.rs`

Implement JSX parsing functions:

```rust
// Parse JSX element
fn parse_jsx_element(&mut self) -> Result<Expression, ParserError> {
    // <TagName attributes>children</TagName>
}

// Parse JSX attributes
fn parse_jsx_attributes(&mut self) -> Result<Vec<JsxAttribute>, ParserError> {
    // class="value" onclick={handler}
}

// Parse JSX children
fn parse_jsx_children(&mut self) -> Result<Vec<JsxChild>, ParserError> {
    // Mix of elements, text, and expressions
}

// Parse JSX expressions
fn parse_jsx_expression(&mut self) -> Result<Expression, ParserError> {
    // {variable} or {expr + 1}
}
```

**New AST Node Types Needed**:
```rust
pub enum Expression {
    // ... existing variants

    JsxElement {
        tag: String,
        attributes: Vec<JsxAttribute>,
        children: Vec<JsxChild>,
    },
    JsxFragment {
        children: Vec<JsxChild>,
    },
}

pub struct JsxAttribute {
    name: String,
    value: JsxAttributeValue,
}

pub enum JsxAttributeValue {
    String(String),
    Expression(Box<Expression>),
}

pub enum JsxChild {
    Element(Box<Expression>),
    Text(String),
    Expression(Box<Expression>),
}
```

### 3. Code Generation Updates

**File**: `src/codegen.rs`

Add JSX-to-JavaScript/WASM code generation:

```rust
fn generate_jsx_element(&mut self, element: &JsxElement) -> String {
    // Generate DOM creation code
    // createElement(), setAttribute(), appendChild(), etc.
}

fn generate_jsx_to_dom(&mut self, jsx: &Expression) -> String {
    match jsx {
        Expression::JsxElement { tag, attributes, children } => {
            // Generate: document.createElement(tag)
            // For each attribute: element.setAttribute(name, value)
            // For each child: element.appendChild(child)
        }
        _ => // handle other cases
    }
}
```

### 4. Runtime Library Updates

**File**: `dist/client-runtime.js`

Add helper functions for JSX:

```javascript
export function createElement(tag, props, ...children) {
    const element = document.createElement(tag);

    // Set attributes
    if (props) {
        for (const [key, value] of Object.entries(props)) {
            if (key.startsWith('on')) {
                // Event handler
                element.addEventListener(key.slice(2).toLowerCase(), value);
            } else {
                element.setAttribute(key, value);
            }
        }
    }

    // Append children
    for (const child of children) {
        if (typeof child === 'string') {
            element.appendChild(document.createTextNode(child));
        } else {
            element.appendChild(child);
        }
    }

    return element;
}
```

## Development Roadmap

### Phase 1: Lexer Support (2-3 days)
- [ ] Add JSX token types
- [ ] Implement JSX tokenization
- [ ] Handle context switching (code vs JSX)
- [ ] Add tests for JSX lexing

### Phase 2: Parser Support (3-5 days)
- [ ] Create JSX AST node types
- [ ] Implement JSX element parsing
- [ ] Implement JSX attribute parsing
- [ ] Implement JSX children parsing
- [ ] Implement JSX expression parsing
- [ ] Add tests for JSX parsing

### Phase 3: Code Generation (2-3 days)
- [ ] Implement JSX-to-DOM generation
- [ ] Handle event handlers
- [ ] Handle reactive expressions
- [ ] Add tests for code generation

### Phase 4: Runtime Support (1-2 days)
- [ ] Create JSX runtime helpers
- [ ] Implement component mounting
- [ ] Implement reactive updates
- [ ] Add tests for runtime

### Phase 5: Integration Testing (2-3 days)
- [ ] Test with DevBoard example
- [ ] Test with Analytics Dashboard
- [ ] Test with Todo App
- [ ] Performance optimization
- [ ] Documentation

**Total Estimated Time**: 10-16 days of development

## Workarounds Until JSX is Implemented

Until JSX support is added, use these patterns:

### 1. DOM Builder Functions

```rust
fn create_button(label: String, onclick: fn()) -> Element {
    let button = document.createElement("button");
    button.setAttribute("class", "btn");
    button.textContent = label;
    button.addEventListener("click", onclick);
    return button;
}
```

### 2. Template Strings with String Concatenation

```rust
fn render_card(title: String, content: String) -> String {
    let html = "<div class=\"card\">" +
        "<h3>" + title + "</h3>" +
        "<p>" + content + "</p>" +
        "</div>";
    return html;
}
```

### 3. Direct DOM Manipulation

```rust
fn main() {
    let app = document.getElementById("app");
    let heading = document.createElement("h1");
    heading.textContent = "Welcome to Jounce";
    app.appendChild(heading);
}
```

## References

- **Working Example**: `examples/enum_simple.jnc` - Basic Jounce syntax without JSX
- **Aspirational Examples**: All files in `examples/` with JSX syntax
- **DevBoard App**: `examples/devboard/src/main.jnc` - Full application showing intended syntax

## Testing JSX Implementation

When JSX support is added, test with this minimal example:

```rust
component Hello(name: String) {
    return <div>
        <h1>Hello, {name}!</h1>
        <p>Welcome to Jounce</p>
    </div>;
}

fn main() {
    let app = Hello("World");
    mount(app, "#app");
}
```

**Expected Output**: DOM tree with `<div><h1>Hello, World!</h1><p>Welcome to Jounce</p></div>`

---

**Document Status**: Updated 2025-10-21 - JSX Parser Implemented
**Jounce Version**: Sprint 7 Complete - commit f9df4db
**Test Status**: 221/230 passing (96%), 11/11 JSX parser tests ✅
