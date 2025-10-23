# Phase 7.5: CSS Integration - Complete Implementation Plan

**Status**: 📋 PLANNED
**Priority**: CRITICAL - Must complete before libraries
**Duration**: 2-3 weeks (3 focused sprints)
**Goal**: Native CSS integration for RavensOne components

---

## 🎯 Phase Overview

### Why CSS Integration Now?
1. **Every library needs styling** - Can't build raven-ui without CSS
2. **Examples are incomplete** - Sprint 7-8 full-stack apps need real styling
3. **Developer expectation** - Modern frameworks (Svelte, Vue, Solid) have built-in CSS
4. **Competitive necessity** - Without CSS, RavensOne feels incomplete

### What We're Building
- **CSS-in-Raven**: Native `css!` macro for component-scoped styles
- **Dynamic styles**: Use Raven variables in CSS
- **Scoped by default**: No global namespace pollution
- **Advanced features**: Nesting, pseudo-classes, media queries
- **SSR-ready**: Critical CSS extraction for server-side rendering

---

## 📅 Sprint Breakdown

```
Sprint 1 (Week 1): CSS Foundation
├── Parser: css! macro syntax
├── AST: CSS expression nodes
├── Compiler: Generate scoped CSS
├── Output: Emit .css files
└── Integration: Auto-inject into HTML

Sprint 2 (Week 2): Advanced Features
├── Nesting: & selector support
├── Dynamic: Raven variables in CSS
├── Media queries: Responsive design
├── Pseudo-classes: :hover, :focus, etc.
└── Animations: @keyframes support

Sprint 3 (Week 3): Utilities & Ecosystem
├── Utility generator: Tailwind-like system
├── CSS modules: Import .css files
├── Theme system: Design tokens
├── Dark mode: Built-in theming
└── SSR: Critical CSS extraction
```

---

## 🚀 Sprint 1: CSS Foundation (Week 1)

**Goal**: Get basic CSS-in-Raven working end-to-end

### Day 1: Parser & Syntax Design (6-8 hours)

#### Task 1.1: Design CSS Macro Syntax (1-2h)
**Goal**: Define how CSS looks in Raven code

**Syntax Option A - css! macro (RECOMMENDED)**:
```raven
let styles = css! {
    .button {
        background: blue;
        padding: 12px 24px;
        border-radius: 4px;
        color: white;
    }

    .button:hover {
        background: darkblue;
    }
};

<button class={styles.button}>Click</button>
```

**Syntax Option B - inline style attribute**:
```raven
<button style={css! {
    background: blue;
    padding: 12px 24px;
}}>Click</button>
```

**Decision Matrix**:
| Feature | css! macro | inline style |
|---------|-----------|--------------|
| Scoping | ✅ Easy | ⚠️ Complex |
| Reuse | ✅ Yes | ❌ No |
| Pseudo-classes | ✅ Yes | ❌ No |
| Media queries | ✅ Yes | ❌ No |
| **Recommendation** | **USE THIS** | Later |

**Deliverable**: Document final syntax in `docs/guides/CSS_SYNTAX.md`

---

#### Task 1.2: Lexer Changes (1-2h)
**Goal**: Tokenize CSS syntax

**File**: `src/lexer.rs`

**New Tokens Needed**:
```rust
pub enum Token {
    // Existing tokens...

    // CSS-specific tokens
    CssMacro,           // css!
    CssBlockStart,      // { after css!
    CssBlockEnd,        // }
    CssSelector,        // .button, #id, div, etc.
    CssProperty,        // background, padding, etc.
    CssValue,           // blue, 12px, etc.
    CssColon,           // :
    CssSemicolon,       // ;
    CssPseudo,          // :hover, :focus, etc.
    CssAmpersand,       // & for nesting
}
```

**Implementation Steps**:
1. Add CSS keyword recognition: `css!`
2. Create CSS lexing mode (triggered by `css!{`)
3. Parse CSS properties as `CssProperty` tokens
4. Parse CSS values (strings, numbers, colors, units)
5. Handle special CSS syntax (`:`, `;`, `&`, etc.)
6. Exit CSS mode on closing `}`

**Test Cases**:
```rust
#[test]
fn test_lex_css_macro() {
    let input = r#"
        let styles = css! {
            .button {
                background: blue;
                padding: 12px;
            }
        };
    "#;

    let tokens = lex(input);
    assert!(tokens.contains(&Token::CssMacro));
    assert!(tokens.contains(&Token::CssSelector));
    assert!(tokens.contains(&Token::CssProperty));
}
```

**Deliverable**: Lexer can tokenize CSS blocks

---

#### Task 1.3: Parser Changes (2-3h)
**Goal**: Parse CSS into AST

**File**: `src/parser.rs`

**New Parsing Methods**:
```rust
impl Parser {
    // Parse: let styles = css! { ... }
    fn parse_css_macro(&mut self) -> Result<CssExpression, ParserError> {
        self.expect(Token::CssMacro)?;
        self.expect(Token::CssBlockStart)?;

        let mut rules = Vec::new();
        while !self.check(Token::CssBlockEnd) {
            rules.push(self.parse_css_rule()?);
        }

        self.expect(Token::CssBlockEnd)?;
        Ok(CssExpression { rules })
    }

    // Parse: .button { ... }
    fn parse_css_rule(&mut self) -> Result<CssRule, ParserError> {
        let selector = self.parse_css_selector()?;
        self.expect(Token::LeftBrace)?;

        let mut declarations = Vec::new();
        while !self.check(Token::RightBrace) {
            declarations.push(self.parse_css_declaration()?);
        }

        self.expect(Token::RightBrace)?;
        Ok(CssRule { selector, declarations })
    }

    // Parse: background: blue;
    fn parse_css_declaration(&mut self) -> Result<CssDeclaration, ParserError> {
        let property = self.parse_css_property()?;
        self.expect(Token::CssColon)?;
        let value = self.parse_css_value()?;
        self.expect(Token::CssSemicolon)?;

        Ok(CssDeclaration { property, value })
    }
}
```

**Test Cases**:
```rust
#[test]
fn test_parse_css_macro() {
    let input = r#"
        let styles = css! {
            .button {
                background: blue;
            }
        };
    "#;

    let ast = parse(input).unwrap();
    assert!(matches!(ast, Statement::LetBinding {
        value: Expression::CssMacro(_), ..
    }));
}
```

**Deliverable**: Parser produces CSS AST nodes

---

#### Task 1.4: AST Additions (1h)
**Goal**: Add CSS-specific AST nodes

**File**: `src/ast.rs`

**New AST Nodes**:
```rust
// CSS macro expression: css! { ... }
#[derive(Debug, Clone)]
pub struct CssExpression {
    pub rules: Vec<CssRule>,
    pub source_location: SourceLocation,
}

// CSS rule: .button { ... }
#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: CssSelector,
    pub declarations: Vec<CssDeclaration>,
    pub nested_rules: Vec<CssRule>,  // For Sprint 2 nesting
}

// CSS selector: .button, #id, div, etc.
#[derive(Debug, Clone)]
pub enum CssSelector {
    Class(String),           // .button
    Id(String),              // #main
    Element(String),         // div, button
    Pseudo(String),          // :hover, :focus
    Nested(String),          // & for nesting
    Compound(Vec<CssSelector>), // .button:hover
}

// CSS declaration: background: blue;
#[derive(Debug, Clone)]
pub struct CssDeclaration {
    pub property: String,
    pub value: CssValue,
}

// CSS value types
#[derive(Debug, Clone)]
pub enum CssValue {
    Color(String),           // blue, #ff0000, rgb(255,0,0)
    Length(f64, String),     // 12px, 1.5rem, 50%
    String(String),          // "Arial", url(...)
    Number(f64),             // 1.5, 0.5
    Keyword(String),         // auto, none, inherit
    Function(String, Vec<CssValue>), // calc(100% - 20px)
    Dynamic(Expression),     // {props.color} - Sprint 2
}

// Add to existing Expression enum
pub enum Expression {
    // ... existing variants
    CssMacro(CssExpression),
}
```

**Deliverable**: Complete AST structure for CSS

---

### Day 2: Code Generation (6-8 hours)

#### Task 1.5: CSS Scoping Strategy (1-2h)
**Goal**: Design how CSS classes are scoped

**Strategy**: Hash-based scoping (like CSS Modules)

**Algorithm**:
```rust
fn generate_scoped_class_name(
    component_name: &str,
    class_name: &str,
    file_path: &str,
) -> String {
    // Create unique hash from component + file
    let hash = hash(&format!("{}{}", component_name, file_path));
    let short_hash = &hash[0..6];

    // Format: ComponentName_className_abc123
    format!("{}_{}_{}",
        component_name,
        class_name,
        short_hash
    )
}

// Example:
// Input:  .button in Button component
// Output: Button_button_a1b2c3
```

**Scope Map Generation**:
```rust
struct ScopeMap {
    // Maps original class names to scoped names
    // .button -> Button_button_a1b2c3
    class_mappings: HashMap<String, String>,
}

fn build_scope_map(component: &Component, css: &CssExpression) -> ScopeMap {
    let mut map = HashMap::new();

    for rule in &css.rules {
        if let CssSelector::Class(name) = &rule.selector {
            let scoped = generate_scoped_class_name(
                &component.name,
                name,
                &component.file_path,
            );
            map.insert(name.clone(), scoped);
        }
    }

    ScopeMap { class_mappings: map }
}
```

**Deliverable**: Scoping algorithm design doc

---

#### Task 1.6: CSS Code Generation (3-4h)
**Goal**: Generate CSS from AST

**File**: `src/css_emitter.rs` (NEW FILE)

**Implementation**:
```rust
pub struct CssEmitter {
    output: String,
    scope_map: ScopeMap,
    component_name: String,
}

impl CssEmitter {
    pub fn new(component_name: String, scope_map: ScopeMap) -> Self {
        Self {
            output: String::new(),
            scope_map,
            component_name,
        }
    }

    pub fn emit_css_expression(&mut self, css: &CssExpression) -> String {
        for rule in &css.rules {
            self.emit_css_rule(rule);
        }
        self.output.clone()
    }

    fn emit_css_rule(&mut self, rule: &CssRule) {
        // Emit selector with scoping
        let scoped_selector = self.scope_selector(&rule.selector);
        self.output.push_str(&format!("{} {{\n", scoped_selector));

        // Emit declarations
        for decl in &rule.declarations {
            self.emit_css_declaration(decl);
        }

        self.output.push_str("}\n\n");
    }

    fn scope_selector(&self, selector: &CssSelector) -> String {
        match selector {
            CssSelector::Class(name) => {
                // Look up scoped name
                self.scope_map.class_mappings
                    .get(name)
                    .unwrap_or(name)
                    .clone()
            }
            CssSelector::Element(name) => {
                // Elements don't get scoped (yet)
                name.clone()
            }
            CssSelector::Compound(selectors) => {
                // .button:hover -> Button_button_abc123:hover
                selectors.iter()
                    .map(|s| self.scope_selector(s))
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => todo!("Other selector types in Sprint 2"),
        }
    }

    fn emit_css_declaration(&mut self, decl: &CssDeclaration) {
        self.output.push_str(&format!(
            "  {}: {};\n",
            decl.property,
            self.emit_css_value(&decl.value)
        ));
    }

    fn emit_css_value(&self, value: &CssValue) -> String {
        match value {
            CssValue::Color(c) => c.clone(),
            CssValue::Length(num, unit) => format!("{}{}", num, unit),
            CssValue::String(s) => format!("\"{}\"", s),
            CssValue::Number(n) => n.to_string(),
            CssValue::Keyword(k) => k.clone(),
            CssValue::Function(name, args) => {
                let arg_strings = args.iter()
                    .map(|a| self.emit_css_value(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", name, arg_strings)
            }
            CssValue::Dynamic(_) => todo!("Dynamic values in Sprint 2"),
        }
    }
}
```

**Test Cases**:
```rust
#[test]
fn test_emit_scoped_css() {
    let css = parse_css("css! { .button { background: blue; } }");
    let scope_map = build_scope_map("MyComponent", &css);
    let mut emitter = CssEmitter::new("MyComponent".to_string(), scope_map);

    let output = emitter.emit_css_expression(&css);

    assert!(output.contains("MyComponent_button_"));
    assert!(output.contains("background: blue;"));
}
```

**Deliverable**: Working CSS emitter

---

#### Task 1.7: JavaScript Integration (2h)
**Goal**: Generate style object in JavaScript

**File**: `src/js_emitter.rs`

**JS Output for CSS**:
```javascript
// From: let styles = css! { .button { ... } }

// Generate styles object
const Button_styles = {
    button: "Button_button_a1b2c3",
    icon: "Button_icon_a1b2c3",
    // ... all scoped class names
};

// Usage: <button class={styles.button}>
// Becomes: <button class="Button_button_a1b2c3">
```

**Implementation**:
```rust
impl JSEmitter {
    fn emit_css_macro(&mut self, css: &CssExpression, var_name: &str) -> String {
        let scope_map = self.build_scope_map(css);

        // Emit styles object
        let mut js = format!("const {} = {{\n", var_name);

        for (original, scoped) in &scope_map.class_mappings {
            js.push_str(&format!("  {}: \"{}\",\n", original, scoped));
        }

        js.push_str("};\n");
        js
    }
}
```

**Deliverable**: JS emitter generates style objects

---

### Day 3: File Output & Integration (6-8 hours)

#### Task 1.8: CSS File Output (2-3h)
**Goal**: Write generated CSS to files

**File**: `src/codegen.rs`

**Output Structure**:
```
dist/
├── client.js       # JavaScript
├── server.js       # Server code
├── styles.css      # ← NEW: All component styles
├── app.wasm       # WASM
└── index.html     # ← UPDATE: Add <link> to styles.css
```

**Implementation**:
```rust
pub struct CssOutput {
    pub file_path: PathBuf,
    pub content: String,
    pub source_maps: Option<String>,
}

impl Compiler {
    fn compile_css(&mut self) -> Result<CssOutput, CompileError> {
        let mut combined_css = String::new();

        // Collect CSS from all components
        for component in &self.components {
            if let Some(css_expr) = &component.css {
                let scope_map = self.build_scope_map(component, css_expr);
                let mut emitter = CssEmitter::new(
                    component.name.clone(),
                    scope_map,
                );

                let css = emitter.emit_css_expression(css_expr);
                combined_css.push_str(&css);
            }
        }

        // Write to dist/styles.css
        let output_path = self.output_dir.join("styles.css");
        fs::write(&output_path, &combined_css)?;

        Ok(CssOutput {
            file_path: output_path,
            content: combined_css,
            source_maps: None,
        })
    }
}
```

**Deliverable**: CSS written to dist/styles.css

---

#### Task 1.9: HTML Integration (1-2h)
**Goal**: Auto-inject CSS into generated HTML

**File**: `src/html_generator.rs`

**Update HTML Template**:
```rust
fn generate_html(&self, css_path: Option<&Path>) -> String {
    let css_link = if let Some(path) = css_path {
        format!("<link rel=\"stylesheet\" href=\"{}\">", path.display())
    } else {
        String::new()
    };

    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RavensOne App</title>
    {css_link}
</head>
<body>
    <div id="app"></div>
    <script src="client.js"></script>
</body>
</html>
    "#, css_link = css_link)
}
```

**Deliverable**: HTML includes CSS link

---

#### Task 1.10: End-to-End Test (2-3h)
**Goal**: Complete working example

**Test File**: `test_css_basic.raven`

```raven
@client
fn Button(props: ButtonProps) -> JSX {
    let styles = css! {
        .button {
            background: #4f46e5;
            color: white;
            padding: 12px 24px;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 16px;
        }

        .button:hover {
            background: #4338ca;
        }
    };

    <button class={styles.button} onclick={props.onClick}>
        {props.children}
    </button>
}

@client
fn App() -> JSX {
    let mut count = 0;

    <div>
        <h1>CSS Test</h1>
        <Button onClick={() => count += 1}>
            Count: {count}
        </Button>
    </div>
}

fn main() {
    render(App, "#app");
}
```

**Expected Output**:

`dist/styles.css`:
```css
.Button_button_a1b2c3 {
  background: #4f46e5;
  color: white;
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
}

.Button_button_a1b2c3:hover {
  background: #4338ca;
}
```

`dist/client.js`:
```javascript
const Button_styles = {
    button: "Button_button_a1b2c3",
};

function Button(props) {
    return `<button class="${Button_styles.button}" onclick="...">
        ${props.children}
    </button>`;
}
```

**Success Criteria**:
- ✅ Compiles without errors
- ✅ CSS file generated
- ✅ Classes are scoped
- ✅ HTML includes CSS link
- ✅ Styles apply in browser

**Deliverable**: Working end-to-end CSS example

---

### Sprint 1 Deliverables Checklist

- [ ] Parser recognizes `css!` macro
- [ ] Lexer tokenizes CSS syntax
- [ ] AST has CSS nodes
- [ ] CSS emitter generates scoped CSS
- [ ] JS emitter generates style objects
- [ ] CSS written to dist/styles.css
- [ ] HTML includes CSS link
- [ ] End-to-end example works
- [ ] Unit tests pass (15+ tests)
- [ ] Documentation updated

**Sprint 1 Success = Can style a button with css! macro**

---

## 🚀 Sprint 2: Advanced CSS Features (Week 2)

**Goal**: Add nesting, dynamic styles, media queries

### Day 4: CSS Nesting (6-8 hours)

#### Task 2.1: Nesting Syntax (2-3h)
**Goal**: Support `&` selector for nesting

**Example**:
```raven
let styles = css! {
    .card {
        padding: 20px;
        background: white;

        &:hover {
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }

        & .title {
            font-size: 24px;
            margin-bottom: 10px;
        }

        & .content {
            color: #666;
        }
    }
};
```

**Output**:
```css
.Card_card_abc123 {
  padding: 20px;
  background: white;
}

.Card_card_abc123:hover {
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}

.Card_card_abc123 .title {
  font-size: 24px;
  margin-bottom: 10px;
}

.Card_card_abc123 .content {
  color: #666;
}
```

**Implementation**:
```rust
// Update CssRule to support nesting
pub struct CssRule {
    pub selector: CssSelector,
    pub declarations: Vec<CssDeclaration>,
    pub nested_rules: Vec<CssRule>,  // ← Add this
}

impl CssEmitter {
    fn emit_css_rule(&mut self, rule: &CssRule, parent_selector: Option<&str>) {
        let selector = self.build_selector(&rule.selector, parent_selector);

        // Emit declarations
        if !rule.declarations.is_empty() {
            self.output.push_str(&format!("{} {{\n", selector));
            for decl in &rule.declarations {
                self.emit_css_declaration(decl);
            }
            self.output.push_str("}\n\n");
        }

        // Emit nested rules
        for nested in &rule.nested_rules {
            self.emit_css_rule(nested, Some(&selector));
        }
    }

    fn build_selector(&self, sel: &CssSelector, parent: Option<&str>) -> String {
        match sel {
            CssSelector::Nested(inner) => {
                // & -> parent selector
                if let Some(p) = parent {
                    format!("{}{}", p, inner)
                } else {
                    inner.clone()
                }
            }
            _ => self.scope_selector(sel),
        }
    }
}
```

**Deliverable**: Nesting works with `&`

---

#### Task 2.2: Pseudo-classes & Elements (2-3h)
**Goal**: Support :hover, :focus, ::before, etc.

**Examples**:
```raven
let styles = css! {
    .button {
        /* Pseudo-classes */
        &:hover { background: blue; }
        &:focus { outline: 2px solid blue; }
        &:active { transform: scale(0.98); }
        &:disabled { opacity: 0.5; }

        /* Pseudo-elements */
        &::before { content: "→"; }
        &::after { content: "←"; }
    }
};
```

**Parser Update**:
```rust
fn parse_pseudo(&mut self) -> Result<CssPseudo, ParserError> {
    self.expect(Token::Colon)?;

    let is_element = self.check(Token::Colon);
    if is_element {
        self.advance(); // consume second :
    }

    let name = self.expect_identifier()?;

    Ok(CssPseudo {
        name,
        is_element,  // true for ::, false for :
    })
}
```

**Deliverable**: All pseudo-classes/elements work

---

#### Task 2.3: Media Queries (2h)
**Goal**: Responsive design support

**Example**:
```raven
let styles = css! {
    .container {
        width: 100%;
        padding: 20px;

        @media (min-width: 768px) {
            width: 750px;
            margin: 0 auto;
        }

        @media (min-width: 1024px) {
            width: 960px;
        }
    }
};
```

**AST Addition**:
```rust
pub struct MediaQuery {
    pub condition: String,  // "(min-width: 768px)"
    pub rules: Vec<CssRule>,
}

pub enum CssRule {
    StyleRule {
        selector: CssSelector,
        declarations: Vec<CssDeclaration>,
        nested: Vec<CssRule>,
    },
    MediaRule {
        query: MediaQuery,
    },
}
```

**Deliverable**: Media queries work

---

### Day 5: Dynamic Styles (6-8 hours)

#### Task 2.4: Raven Variables in CSS (4-5h)
**Goal**: Use props/variables in CSS values

**Example**:
```raven
@client
fn Button(props: ButtonProps) -> JSX {
    let styles = css! {
        .button {
            background: {props.color};  // Dynamic!
            padding: {props.size == "large" ? "16px 32px" : "8px 16px"};
            opacity: {props.disabled ? 0.5 : 1.0};
        }
    };

    <button class={styles.button}>{props.children}</button>
}
```

**Strategy**: Inline styles for dynamic values

**Output**:
```javascript
function Button(props) {
    // Static styles in CSS file
    const staticClass = "Button_button_abc123";

    // Dynamic styles inline
    const dynamicStyle = {
        background: props.color,
        padding: props.size === "large" ? "16px 32px" : "8px 16px",
        opacity: props.disabled ? 0.5 : 1.0,
    };

    return `<button
        class="${staticClass}"
        style="${styleToString(dynamicStyle)}"
    >${props.children}</button>`;
}
```

**Implementation**:
```rust
// Detect dynamic values in CSS
impl CssEmitter {
    fn emit_css_value(&self, value: &CssValue) -> CssOutput {
        match value {
            CssValue::Dynamic(expr) => {
                // This needs to be inline style
                CssOutput::Inline {
                    property: self.current_property.clone(),
                    value_expr: expr.clone(),
                }
            }
            _ => CssOutput::Static(self.emit_static_value(value)),
        }
    }
}

// Split CSS into static and dynamic parts
pub struct SplitCss {
    pub static_rules: Vec<CssRule>,
    pub dynamic_declarations: Vec<(String, Expression)>,
}
```

**Deliverable**: Dynamic CSS values work

---

#### Task 2.5: CSS Variables (2-3h)
**Goal**: CSS custom properties support

**Example**:
```raven
let styles = css! {
    .theme {
        --primary-color: #4f46e5;
        --text-color: #111827;
    }

    .button {
        background: var(--primary-color);
        color: var(--text-color);
    }
};
```

**Just pass through** - CSS variables work natively!

**Deliverable**: CSS variables work

---

### Day 6: Animations & Keyframes (6-8 hours)

#### Task 2.6: @keyframes Support (3-4h)
**Goal**: CSS animations

**Example**:
```raven
let styles = css! {
    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    @keyframes slideIn {
        0% { transform: translateX(-100%); }
        100% { transform: translateX(0); }
    }

    .animated {
        animation: fadeIn 0.3s ease-in;
    }

    .slide {
        animation: slideIn 0.5s ease-out;
    }
};
```

**Implementation**:
```rust
pub struct Keyframes {
    pub name: String,
    pub frames: Vec<KeyframeRule>,
}

pub struct KeyframeRule {
    pub selector: KeyframeSelector,  // "from", "to", "50%"
    pub declarations: Vec<CssDeclaration>,
}

pub enum KeyframeSelector {
    From,
    To,
    Percentage(f64),
}
```

**Scoping**: Keyframe names need scoping too!
```css
/* Input */
@keyframes fadeIn { ... }

/* Output */
@keyframes Button_fadeIn_abc123 { ... }
.Button_animated_abc123 {
    animation: Button_fadeIn_abc123 0.3s ease-in;
}
```

**Deliverable**: Animations work

---

#### Task 2.7: Transitions (1-2h)
**Goal**: CSS transitions

**Example**:
```raven
let styles = css! {
    .button {
        background: blue;
        transition: all 0.2s ease;

        &:hover {
            background: darkblue;
            transform: scale(1.05);
        }
    }
};
```

**Easy** - transitions are just properties!

**Deliverable**: Transitions work

---

#### Task 2.8: Sprint 2 Testing (2h)
**Goal**: Comprehensive test file

**Test File**: `test_css_advanced.raven`

```raven
@client
fn AnimatedCard(props: CardProps) -> JSX {
    let styles = css! {
        @keyframes slideIn {
            from { transform: translateY(-20px); opacity: 0; }
            to { transform: translateY(0); opacity: 1; }
        }

        .card {
            background: white;
            padding: 20px;
            border-radius: 8px;
            animation: slideIn 0.3s ease;
            transition: all 0.2s ease;

            &:hover {
                box-shadow: 0 10px 20px rgba(0,0,0,0.1);
                transform: translateY(-2px);
            }

            & .title {
                font-size: {props.titleSize};  // Dynamic!
                color: {props.titleColor};      // Dynamic!
                margin-bottom: 10px;
            }

            & .content {
                color: #666;

                @media (max-width: 768px) {
                    font-size: 14px;
                }
            }
        }
    };

    <div class={styles.card}>
        <h2 class={styles.title}>{props.title}</h2>
        <div class={styles.content}>{props.content}</div>
    </div>
}
```

**Success Criteria**:
- ✅ Nesting works
- ✅ Media queries work
- ✅ Animations work
- ✅ Dynamic values work
- ✅ Pseudo-classes work

**Deliverable**: All advanced features work

---

### Sprint 2 Deliverables Checklist

- [ ] CSS nesting with `&`
- [ ] Pseudo-classes (:hover, :focus)
- [ ] Pseudo-elements (::before, ::after)
- [ ] Media queries (@media)
- [ ] Dynamic CSS values
- [ ] CSS custom properties (--var)
- [ ] Keyframe animations (@keyframes)
- [ ] Transitions
- [ ] Advanced test passes
- [ ] 20+ new tests

**Sprint 2 Success = Advanced CSS features work**

---

## 🚀 Sprint 3: Utilities & Ecosystem (Week 3)

**Goal**: Utility classes, CSS modules, theming, SSR

### Day 7: Utility Class Generator (6-8 hours)

#### Task 3.1: Utility System Design (2-3h)
**Goal**: Built-in Tailwind-like utilities

**Usage**:
```raven
<div class="flex items-center justify-between p-4 bg-blue-500 text-white rounded-lg">
    <span class="text-lg font-bold">Hello</span>
    <button class="px-4 py-2 bg-white text-blue-500 rounded hover:bg-gray-100">
        Click
    </button>
</div>
```

**Config File**: `raven.config.toml`
```toml
[css]
utilities = true

[css.theme]
colors = [
    { name = "blue", values = ["#eff6ff", "#dbeafe", "#3b82f6", "#1e40af"] },
    { name = "gray", values = ["#f9fafb", "#f3f4f6", "#6b7280", "#111827"] },
]

spacing = [0, 4, 8, 12, 16, 20, 24, 32, 40, 48, 64]
border_radius = [0, 2, 4, 6, 8, 12, 16, 24]
```

**Generated CSS**:
```css
/* Layout */
.flex { display: flex; }
.items-center { align-items: center; }
.justify-between { justify-content: space-between; }

/* Spacing */
.p-4 { padding: 16px; }
.px-4 { padding-left: 16px; padding-right: 16px; }
.py-2 { padding-top: 8px; padding-bottom: 8px; }

/* Colors */
.bg-blue-500 { background-color: #3b82f6; }
.text-white { color: white; }

/* Border */
.rounded { border-radius: 4px; }
.rounded-lg { border-radius: 8px; }

/* Typography */
.text-lg { font-size: 18px; }
.font-bold { font-weight: 700; }

/* Hover states */
.hover\:bg-gray-100:hover { background-color: #f3f4f6; }
```

**Implementation**:
```rust
pub struct UtilityGenerator {
    config: UtilityConfig,
}

impl UtilityGenerator {
    pub fn generate(&self) -> String {
        let mut css = String::new();

        css.push_str(&self.generate_layout());
        css.push_str(&self.generate_spacing());
        css.push_str(&self.generate_colors());
        css.push_str(&self.generate_typography());
        css.push_str(&self.generate_borders());

        css
    }

    fn generate_spacing(&self) -> String {
        let mut css = String::new();

        for size in &self.config.spacing {
            // p-4 -> padding: 16px
            css.push_str(&format!(".p-{} {{ padding: {}px; }}\n", size, size));
            css.push_str(&format!(".px-{} {{ padding-left: {}px; padding-right: {}px; }}\n", size, size));
            // ... all variants
        }

        css
    }
}
```

**Tree-shaking**: Only include used utilities
```rust
fn analyze_used_utilities(ast: &AST) -> HashSet<String> {
    let mut used = HashSet::new();

    // Walk AST and find all class names
    walk_ast(ast, |node| {
        if let Some(class_attr) = node.get_attribute("class") {
            for class in class_attr.split_whitespace() {
                used.insert(class.to_string());
            }
        }
    });

    used
}
```

**Deliverable**: Utility generator working

---

#### Task 3.2: JIT Compilation (2-3h)
**Goal**: Generate utilities on-demand

**Strategy**: Watch for class usage and generate CSS
```rust
impl UtilityGenerator {
    pub fn generate_on_demand(&mut self, class_name: &str) -> Option<String> {
        // Parse: "bg-blue-500" -> background-color: #3b82f6
        if let Some(utility) = self.parse_utility_class(class_name) {
            return Some(self.generate_utility(&utility));
        }
        None
    }

    fn parse_utility_class(&self, class: &str) -> Option<UtilityDefinition> {
        // bg-blue-500
        if let Some(caps) = self.color_pattern.captures(class) {
            return Some(UtilityDefinition::BackgroundColor {
                color: caps.get(1)?.as_str(),
                shade: caps.get(2)?.as_str().parse().ok()?,
            });
        }

        // p-4
        if let Some(caps) = self.spacing_pattern.captures(class) {
            return Some(UtilityDefinition::Padding {
                size: caps.get(1)?.as_str().parse().ok()?,
            });
        }

        None
    }
}
```

**Deliverable**: JIT utilities work

---

#### Task 3.3: Utility Customization (1-2h)
**Goal**: User-defined utilities

**Config**:
```toml
[css.utilities.custom]
".btn-primary" = """
    background: #4f46e5;
    color: white;
    padding: 12px 24px;
    border-radius: 6px;
"""

".container" = """
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 20px;
"""
```

**Deliverable**: Custom utilities work

---

### Day 8: CSS Modules & Imports (6-8 hours)

#### Task 3.4: CSS Module Import (3-4h)
**Goal**: Import external CSS files

**Usage**:
```raven
import styles from "./button.module.css";

@client
fn Button(props: ButtonProps) -> JSX {
    <button class={styles.primary}>
        {props.children}
    </button>
}
```

**File**: `button.module.css`
```css
.primary {
    background: blue;
    color: white;
}

.secondary {
    background: gray;
    color: black;
}
```

**Compilation**:
1. Parse import statement
2. Read CSS file
3. Scope class names
4. Generate style object
5. Include in output

**Implementation**:
```rust
impl Parser {
    fn parse_css_import(&mut self) -> Result<Import, ParserError> {
        // import styles from "./file.module.css"
        self.expect(Token::Import)?;
        let name = self.expect_identifier()?;
        self.expect(Token::From)?;
        let path = self.expect_string()?;

        // Check if .module.css
        if path.ends_with(".module.css") {
            Ok(Import::CssModule { name, path })
        } else if path.ends_with(".css") {
            Ok(Import::GlobalCss { path })
        } else {
            Err(ParserError::InvalidImport)
        }
    }
}

impl Compiler {
    fn process_css_import(&mut self, import: &Import) -> Result<(), Error> {
        match import {
            Import::CssModule { name, path } => {
                let css = fs::read_to_string(path)?;
                let parsed = self.parse_css_file(&css)?;
                let scoped = self.scope_css_module(&parsed, name);

                // Add to component's style object
                self.add_style_binding(name, scoped);
            }
            Import::GlobalCss { path } => {
                // Just include CSS without scoping
                let css = fs::read_to_string(path)?;
                self.global_css.push_str(&css);
            }
        }
        Ok(())
    }
}
```

**Deliverable**: CSS modules work

---

#### Task 3.5: Global CSS Import (1-2h)
**Goal**: Import global stylesheets

**Usage**:
```raven
// In main.raven
import "./global.css";  // Global styles
import "./reset.css";   // CSS reset

fn main() {
    // ...
}
```

**Output**: Prepended to styles.css
```css
/* From reset.css */
* { margin: 0; padding: 0; box-sizing: border-box; }

/* From global.css */
body { font-family: sans-serif; }

/* Component styles */
.Button_button_abc123 { ... }
```

**Deliverable**: Global imports work

---

#### Task 3.6: PostCSS/Autoprefixer (2h)
**Goal**: Auto-add vendor prefixes

**Example Input**:
```css
.box {
    display: flex;
    user-select: none;
}
```

**Output**:
```css
.box {
    display: -webkit-box;
    display: -ms-flexbox;
    display: flex;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}
```

**Implementation**: Use `autoprefixer` crate
```rust
use autoprefixer::Autoprefixer;

impl CssEmitter {
    pub fn post_process(&mut self, css: String) -> String {
        let prefixer = Autoprefixer::new();
        prefixer.process(&css)
    }
}
```

**Deliverable**: Autoprefixer works

---

### Day 9: Theming & SSR (6-8 hours)

#### Task 3.7: Theme System (3-4h)
**Goal**: Built-in light/dark mode

**Usage**:
```raven
let styles = css! {
    .card {
        background: theme(background);
        color: theme(text);
        border: 1px solid theme(border);
    }
};

// Or with variants
let styles = css! {
    .card {
        @light {
            background: white;
            color: black;
        }

        @dark {
            background: #1a1a1a;
            color: white;
        }
    }
};
```

**Output**:
```css
/* Light theme (default) */
.Card_card_abc123 {
    background: white;
    color: black;
}

/* Dark theme */
[data-theme="dark"] .Card_card_abc123 {
    background: #1a1a1a;
    color: white;
}

/* Or with CSS variables */
:root {
    --background: white;
    --text: black;
}

[data-theme="dark"] {
    --background: #1a1a1a;
    --text: white;
}

.Card_card_abc123 {
    background: var(--background);
    color: var(--text);
}
```

**JavaScript Helper**:
```javascript
function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
}

// Auto-detect
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
setTheme(localStorage.getItem('theme') || (prefersDark ? 'dark' : 'light'));
```

**Deliverable**: Theme system works

---

#### Task 3.8: Critical CSS Extraction (2-3h)
**Goal**: Extract above-the-fold CSS for SSR

**Strategy**:
1. Analyze component tree
2. Identify "critical" components (rendered first)
3. Extract their CSS
4. Inline in `<head>`
5. Load rest async

**Implementation**:
```rust
pub struct CriticalCssExtractor {
    critical_components: HashSet<String>,
}

impl CriticalCssExtractor {
    pub fn extract(&self, css: &str) -> (String, String) {
        let mut critical = String::new();
        let mut deferred = String::new();

        for rule in parse_css(css) {
            if self.is_critical(&rule) {
                critical.push_str(&rule.to_string());
            } else {
                deferred.push_str(&rule.to_string());
            }
        }

        (critical, deferred)
    }

    fn is_critical(&self, rule: &CssRule) -> bool {
        // Check if rule belongs to a critical component
        self.critical_components.iter().any(|component| {
            rule.selector.starts_with(&format!("{}_", component))
        })
    }
}
```

**HTML Output**:
```html
<!DOCTYPE html>
<html>
<head>
    <title>App</title>
    <!-- Critical CSS inlined -->
    <style>
        .App_root_abc123 { ... }
        .Header_header_def456 { ... }
    </style>
</head>
<body>
    <div id="app"></div>
    <!-- Rest of CSS loaded async -->
    <link rel="stylesheet" href="styles.css" media="print" onload="this.media='all'">
    <script src="client.js"></script>
</body>
</html>
```

**Deliverable**: Critical CSS works

---

#### Task 3.9: Source Maps (1-2h)
**Goal**: CSS source maps for debugging

**Generate**:
```rust
pub struct CssSourceMap {
    pub version: u8,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
}

impl CssEmitter {
    fn generate_source_map(&self) -> CssSourceMap {
        let mut map = CssSourceMap::new();

        for rule in &self.rules {
            map.add_mapping(
                rule.output_line,
                rule.output_column,
                rule.source_file,
                rule.source_line,
                rule.source_column,
            );
        }

        map
    }
}
```

**Output**: `styles.css.map`
```json
{
  "version": 3,
  "sources": ["button.raven", "card.raven"],
  "names": [],
  "mappings": "AAAA,CAAC,CAAC,CAAC,CAAC,CAAC,CAAC,CAAC"
}
```

**Deliverable**: Source maps work

---

### Sprint 3 Deliverables Checklist

- [ ] Utility class generator
- [ ] JIT utility compilation
- [ ] CSS module imports
- [ ] Global CSS imports
- [ ] Autoprefixer integration
- [ ] Theme system (light/dark)
- [ ] Critical CSS extraction
- [ ] CSS source maps
- [ ] 25+ new tests
- [ ] Performance benchmarks

**Sprint 3 Success = Production-ready CSS system**

---

## 📚 Documentation Plan

### Guides to Write

1. **CSS_SYNTAX.md** - Complete syntax reference
2. **CSS_COOKBOOK.md** - Common patterns and recipes
3. **CSS_THEMING.md** - Theme system guide
4. **CSS_UTILITIES.md** - Utility class reference
5. **CSS_MIGRATION.md** - Migrating from CSS/Sass/styled-components

### Example Structure

**CSS_COOKBOOK.md** should include:
- Basic button styling
- Card components
- Navigation menus
- Forms and inputs
- Layouts (flex, grid)
- Animations and transitions
- Responsive patterns
- Dark mode
- Print styles

---

## 🧪 Testing Strategy

### Unit Tests (100+ tests total)

**Lexer Tests** (15 tests):
- Tokenize css! macro
- Parse CSS properties
- Parse CSS values
- Parse selectors
- Parse pseudo-classes

**Parser Tests** (20 tests):
- Parse simple rules
- Parse nested rules
- Parse media queries
- Parse keyframes
- Parse dynamic values

**Emitter Tests** (25 tests):
- Generate scoped classes
- Generate nested CSS
- Generate media queries
- Generate keyframes
- Generate dynamic styles

**Integration Tests** (40+ tests):
- End-to-end compilation
- Multi-component CSS
- CSS modules
- Utilities
- Theming

### Performance Benchmarks

```rust
#[bench]
fn bench_css_compilation(b: &mut Bencher) {
    let css = load_large_css_file();
    b.iter(|| compile_css(&css));
}

// Target: < 10ms for 1000 lines of CSS
```

---

## 🎯 Success Criteria

### Sprint 1 Success
- ✅ Can style a button with css! macro
- ✅ Classes are scoped
- ✅ CSS file generated
- ✅ Works in browser

### Sprint 2 Success
- ✅ Nesting works
- ✅ Dynamic values work
- ✅ Media queries work
- ✅ Animations work

### Sprint 3 Success
- ✅ Utilities work
- ✅ CSS modules work
- ✅ Theming works
- ✅ SSR-ready

### Overall Success
- ✅ 100+ tests passing
- ✅ Documentation complete
- ✅ 10+ examples
- ✅ Performance benchmarks met
- ✅ Developer experience is great

---

## 🚀 After Phase 7.5

Once CSS is complete:

1. **Use it in examples** - Retrofit Sprint 7-8 apps
2. **Build raven-ui** - Component library with great styling
3. **Build raven-forms** - Styled form components
4. **Create design system** - Official RavensOne design tokens
5. **Showcase** - Beautiful examples that wow people

---

## 📊 Time Estimates

| Sprint | Tasks | Time | Output |
|--------|-------|------|--------|
| Sprint 1 | CSS Foundation | 3 days | Basic css! macro |
| Sprint 2 | Advanced Features | 3 days | Nesting, dynamic, animations |
| Sprint 3 | Utilities & Ecosystem | 3 days | Utilities, modules, theming |
| **Total** | **9 sprints** | **2-3 weeks** | **Production CSS system** |

---

## 🎨 Example Gallery

After Phase 7.5, create these examples:

1. **Button Variants** - Primary, secondary, ghost, danger
2. **Card Components** - Product cards, profile cards, stats cards
3. **Navigation** - Headers, sidebars, breadcrumbs
4. **Forms** - Login, signup, multi-step
5. **Layouts** - Dashboard, landing page, blog
6. **Animations** - Loading, transitions, micro-interactions
7. **Dark Mode** - Full theme switching
8. **Responsive** - Mobile-first layouts

---

## 💡 Future Enhancements (Phase 8+)

- **CSS Grid helpers** - Grid utilities and helpers
- **Container queries** - New CSS feature
- **Sass compatibility** - Import .scss files
- **CSS-in-JS libraries** - styled() API
- **Design tokens** - JSON/YAML design system
- **Figma integration** - Import styles from Figma
- **Storybook** - Component documentation
- **Visual regression** - Screenshot testing

---

**Ready to start Sprint 1 tomorrow? 🚀**

**First task**: Task 1.1 - Design CSS macro syntax (1-2 hours)

Good luck! 🎨
