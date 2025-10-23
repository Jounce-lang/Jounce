use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Use(UseStatement),
    Let(LetStatement),
    Const(ConstDeclaration),
    Assignment(AssignmentStatement),
    Return(ReturnStatement),
    Expression(Expression),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    ForIn(ForInStatement),
    MacroInvocation(MacroInvocation),
    Struct(StructDefinition),
    Enum(EnumDefinition),
    Function(FunctionDefinition),
    Component(ComponentDefinition),
    ExternBlock(ExternBlock),
    ImplBlock(ImplBlock),
    Trait(TraitDefinition),
}

#[derive(Debug, Clone)]
pub struct UseStatement {
    pub path: Vec<Identifier>,
    pub imports: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub pattern: Pattern,  // Changed from 'name' to support destructuring
    pub mutable: bool,
    pub type_annotation: Option<TypeExpression>,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ConstDeclaration {
    pub name: Identifier,
    pub type_annotation: Option<TypeExpression>,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub target: Expression,  // Can be Identifier, FieldAccess, IndexAccess, etc.
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: BlockStatement,
    pub else_branch: Option<Box<Statement>>,  // Can be another if or block
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,  // Optional initialization (let i = 0)
    pub condition: Expression,          // Loop condition
    pub update: Option<Box<Statement>>, // Optional update (i = i + 1)
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct ForInStatement {
    pub variable: Identifier,          // Loop variable (e.g., "item" in "for item in collection")
    pub iterator: Expression,          // The expression to iterate over
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct MacroInvocation {
    pub name: Identifier,
    pub input_tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: Identifier,
    pub lifetime_params: Vec<Lifetime>,  // Lifetime parameters like <'a, 'b>
    pub type_params: Vec<TypeParam>,  // Generic type parameters like <T>, <T: Display>
    pub fields: Vec<(Identifier, TypeExpression)>,
    pub derives: Vec<String>,  // Derive macros: #[derive(Debug, Clone, etc.)]
}

#[derive(Debug, Clone)]
pub struct EnumDefinition {
    pub name: Identifier,
    pub lifetime_params: Vec<Lifetime>,  // Lifetime parameters like <'a, 'b>
    pub type_params: Vec<TypeParam>,  // Generic type parameters like <T>, <T: Display>
    pub variants: Vec<EnumVariant>,
    pub derives: Vec<String>,  // Derive macros: #[derive(Debug, Clone, etc.)]
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: Identifier,
    pub fields: Option<Vec<(Identifier, TypeExpression)>>,  // For tuple/struct variants
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub lifetime_params: Vec<Lifetime>,  // Lifetime parameters like <'a, 'b>
    pub type_params: Vec<TypeParam>,  // Generic type parameters like <T>, <T: Display>
    pub parameters: Vec<FunctionParameter>,
    pub is_server: bool,
    pub is_client: bool,
    pub is_async: bool,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct ExternBlock {
    pub abi: String,
    pub functions: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeExpression>,
}

// CSS-related AST nodes

// CSS macro expression: css! { ... }
#[derive(Debug, Clone)]
pub struct CssExpression {
    pub rules: Vec<CssRule>,
}

// CSS rule: .button { ... }
#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: CssSelector,
    pub declarations: Vec<CssDeclaration>,
    pub nested_rules: Vec<CssRule>,  // For Sprint 2 nesting (not used yet)
}

// CSS selector: .button, #id, div, etc.
#[derive(Debug, Clone)]
pub enum CssSelector {
    Class(String),           // .button
    Id(String),              // #main
    Element(String),         // div, button
    Pseudo(String),          // :hover, :focus
    Nested(String),          // & for nesting (Sprint 2)
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
    Raw(String),             // Unparsed value (for now - Sprint 1)
    // Dynamic(Expression),     // {props.color} - Sprint 2 (commented out for now)
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    FloatLiteral(String),
    StringLiteral(String),
    BoolLiteral(bool),
    ArrayLiteral(ArrayLiteral),
    TupleLiteral(TupleLiteral),
    StructLiteral(StructLiteral),
    Prefix(PrefixExpression),
    Spread(SpreadExpression),  // ...expr (spread operator in arrays)
    Infix(InfixExpression),
    FieldAccess(FieldAccessExpression),
    IndexAccess(IndexExpression),
    Match(MatchExpression),
    IfExpression(IfExpression),  // if cond { then } else { else } as an expression
    JsxElement(JsxElement),
    FunctionCall(FunctionCall),
    MacroCall(MacroCall),  // macro!(...) as an expression (vec!, println!, etc.)
    Lambda(LambdaExpression),
    Borrow(BorrowExpression),      // &x (create reference)
    MutableBorrow(MutableBorrowExpression),  // &mut x (create mutable reference)
    Dereference(DereferenceExpression),  // *x (dereference)
    Range(RangeExpression),  // start..end or start..=end
    TryOperator(TryOperatorExpression),  // expr? (error propagation)
    Ternary(TernaryExpression),  // condition ? true_expr : false_expr
    TypeCast(TypeCastExpression),  // expr as Type (type casting)
    Await(AwaitExpression),  // await expr (async/await)
    Block(BlockStatement),  // { statements... } as an expression (for match arms, etc.)
    CssMacro(CssExpression),  // css! { ... } macro for styles
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub value: String,
}

// Lifetime annotation like 'a, 'b, 'static
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime {
    pub name: String,  // e.g., "a" for 'a, "static" for 'static
}

// Type parameter with optional trait bounds
// Examples: T, T: Display, T: Display + Clone
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParam {
    pub name: Identifier,
    pub bounds: Vec<Identifier>,  // trait bounds (e.g., ["Display", "Clone"])
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct SpreadExpression {
    pub expression: Box<Expression>,  // The expression being spread (e.g., arr in ...arr)
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct BorrowExpression {
    pub expression: Box<Expression>,  // The expression being borrowed
}

#[derive(Debug, Clone)]
pub struct MutableBorrowExpression {
    pub expression: Box<Expression>,  // The expression being mutably borrowed
}

#[derive(Debug, Clone)]
pub struct DereferenceExpression {
    pub expression: Box<Expression>,  // The expression being dereferenced
}

#[derive(Debug, Clone)]
pub struct RangeExpression {
    pub start: Option<Box<Expression>>,  // Start of range (None for ..end)
    pub end: Option<Box<Expression>>,    // End of range (None for start..)
    pub inclusive: bool,                  // true for ..=, false for ..
}

#[derive(Debug, Clone)]
pub struct TryOperatorExpression {
    pub expression: Box<Expression>,  // The expression being tried (must return Result<T, E>)
}

#[derive(Debug, Clone)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,     // The condition to test
    pub true_expr: Box<Expression>,     // Expression if condition is true
    pub false_expr: Box<Expression>,    // Expression if condition is false
}

#[derive(Debug, Clone)]
pub struct TypeCastExpression {
    pub expression: Box<Expression>,  // The expression being cast
    pub target_type: TypeExpression,  // The type to cast to
}

#[derive(Debug, Clone)]
pub struct AwaitExpression {
    pub expression: Box<Expression>,  // The expression being awaited (must return Future<T>)
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct TupleLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct StructLiteral {
    pub name: Identifier,
    pub fields: Vec<(Identifier, Expression)>,
}

#[derive(Debug, Clone)]
pub struct FieldAccessExpression {
    pub object: Box<Expression>,
    pub field: Identifier,
}

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct MatchExpression {
    pub scrutinee: Box<Expression>,  // The value being matched
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub patterns: Vec<Pattern>,  // Support OR patterns: 3 | 4 | 5 => ...
    pub body: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Identifier(Identifier),           // x (binds to variable)
    Tuple(Vec<Pattern>),              // (a, b, c) (destructure tuple)
    Literal(Expression),              // 42, "hello", true
    Wildcard,                         // _ (matches anything)
    EnumVariant {
        name: Identifier,             // Color::Red or Option::Some
        fields: Option<Vec<Pattern>>, // For destructuring fields
    },
}

impl Pattern {
    /// Extract all identifiers bound by this pattern
    pub fn bound_identifiers(&self) -> Vec<Identifier> {
        match self {
            Pattern::Identifier(id) => vec![id.clone()],
            Pattern::Tuple(patterns) => {
                patterns.iter().flat_map(|p| p.bound_identifiers()).collect()
            }
            Pattern::Wildcard => vec![],
            Pattern::Literal(_) => vec![],
            Pattern::EnumVariant { fields, .. } => {
                fields.as_ref().map(|f| f.iter().flat_map(|p| p.bound_identifiers()).collect()).unwrap_or_else(Vec::new)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub type_annotation: TypeExpression,
}

// This is the single, correct definition for TypeExpression
#[derive(Debug, Clone)]
pub enum TypeExpression {
    Named(Identifier),
    Generic(Identifier, Vec<TypeExpression>),
    Tuple(Vec<TypeExpression>),  // (i32, String, bool)
    Reference(Box<TypeExpression>),  // &T (immutable reference)
    MutableReference(Box<TypeExpression>),  // &mut T (mutable reference)
    Slice(Box<TypeExpression>),  // [T] (slice type)
    SizedArray(Box<TypeExpression>, usize),  // [T; N] (fixed-size array)
    Function(Vec<TypeExpression>, Box<TypeExpression>),  // fn(T1, T2) -> R (function type)
}

// --- JSX AST Nodes ---

/// Represents a complete JSX element: `<tag attr="value">children</tag>` or `<tag />`
///
/// # Examples
/// - Regular element: `<div class="container">Hello</div>`
/// - Self-closing: `<img src="photo.jpg" />`
/// - Nested: `<div><span>nested</span></div>`
/// - With expressions: `<div>{variable}</div>`
///
/// # Fields
/// - `opening_tag`: The opening tag with name, attributes, and self-closing flag
/// - `children`: Child elements, text nodes, or expressions
/// - `closing_tag`: The closing tag name (None for self-closing tags)
///
/// # Parser Notes
/// The parser should ensure closing_tag matches opening_tag.name for regular elements.
/// For self-closing elements (opening_tag.self_closing = true), closing_tag should be None.
#[derive(Debug, Clone)]
pub struct JsxElement {
    pub opening_tag: JsxOpeningTag,
    pub children: Vec<JsxChild>,
    pub closing_tag: Option<Identifier>,
}

/// Represents the opening tag of a JSX element
///
/// # Examples
/// - `<div>` → name="div", attributes=[], self_closing=false
/// - `<img src="x.jpg" />` → name="img", attributes=[...], self_closing=true
/// - `<Button onClick={handler}>` → name="Button", attributes=[...], self_closing=false
///
/// # Fields
/// - `name`: Tag name (lowercase for HTML elements, PascalCase for components)
/// - `attributes`: List of attributes/props
/// - `self_closing`: True if tag ends with `/>`
#[derive(Debug, Clone)]
pub struct JsxOpeningTag {
    pub name: Identifier,
    pub attributes: Vec<JsxAttribute>,
    pub self_closing: bool,
}

/// Represents a child node within JSX content
///
/// JSX children can be:
/// - **Element**: A nested JSX element `<child>`
/// - **Text**: Plain text content between tags
/// - **Expression**: An interpolated expression `{variable}` or `{expr + 1}`
///
/// # Examples
/// ```jsx
/// <div>
///   Hello          // Text("Hello")
///   <span>world</span>  // Element(...)
///   {name}         // Expression(Identifier)
/// </div>
/// ```
///
/// # Parser Notes
/// Text is read automatically by the lexer when in JSX mode.
/// Expressions are enclosed in {} and can be any valid expression.
#[derive(Debug, Clone)]
pub enum JsxChild {
    /// A nested JSX element
    Element(Box<JsxElement>),
    /// Plain text content (automatically read by lexer in JSX mode)
    Text(String),
    /// An expression interpolation: {expr}
    Expression(Box<Expression>),
}

/// Represents a JSX attribute (HTML attribute or React prop)
///
/// # Examples
/// - String attribute: `class="container"` → name="class", value=StringLiteral("container")
/// - Expression attribute: `value={count}` → name="value", value=Identifier(count)
/// - Event handler: `onClick={handleClick}` → name="onClick", value=Identifier(handleClick)
/// - Boolean shorthand: `disabled` → name="disabled", value=BoolLiteral(true)
///
/// # Attribute Types
/// - String literals: `attr="value"`
/// - Expressions: `attr={expr}`
/// - Event handlers: `onClick={handler}` (Expression containing function/lambda)
/// - Boolean: `disabled` → implicitly true
///
/// # Parser Notes
/// - For `attr="value"`, value is StringLiteral
/// - For `attr={expr}`, value is the expression inside {}
/// - For `attr` alone (no =), value should be BoolLiteral(true)
#[derive(Debug, Clone)]
pub struct JsxAttribute {
    pub name: Identifier,
    pub value: Expression,
}

impl JsxElement {
    /// Creates a new JSX element with the given tag name and empty children
    pub fn new(tag_name: String) -> Self {
        JsxElement {
            opening_tag: JsxOpeningTag {
                name: Identifier { value: tag_name.clone() },
                attributes: Vec::new(),
                self_closing: false,
            },
            children: Vec::new(),
            closing_tag: Some(Identifier { value: tag_name }),
        }
    }

    /// Creates a new self-closing JSX element
    pub fn new_self_closing(tag_name: String) -> Self {
        JsxElement {
            opening_tag: JsxOpeningTag {
                name: Identifier { value: tag_name },
                attributes: Vec::new(),
                self_closing: true,
            },
            children: Vec::new(),
            closing_tag: None,
        }
    }

    /// Returns true if this is a self-closing element
    pub fn is_self_closing(&self) -> bool {
        self.opening_tag.self_closing
    }

    /// Returns the tag name
    pub fn tag_name(&self) -> &str {
        &self.opening_tag.name.value
    }

    /// Adds a child to this element
    pub fn add_child(&mut self, child: JsxChild) {
        self.children.push(child);
    }

    /// Adds a text child to this element
    pub fn add_text(&mut self, text: String) {
        self.children.push(JsxChild::Text(text));
    }

    /// Adds an attribute to this element
    pub fn add_attribute(&mut self, name: String, value: Expression) {
        self.opening_tag.attributes.push(JsxAttribute {
            name: Identifier { value: name },
            value,
        });
    }
}

impl JsxAttribute {
    /// Creates a new JSX attribute with a string literal value
    pub fn new_string(name: String, value: String) -> Self {
        JsxAttribute {
            name: Identifier { value: name },
            value: Expression::StringLiteral(value),
        }
    }

    /// Creates a new JSX attribute with an expression value
    pub fn new_expr(name: String, value: Expression) -> Self {
        JsxAttribute {
            name: Identifier { value: name },
            value,
        }
    }

    /// Creates a new boolean attribute (just the name, implicitly true)
    pub fn new_bool(name: String) -> Self {
        JsxAttribute {
            name: Identifier { value: name },
            value: Expression::BoolLiteral(true),
        }
    }

    /// Returns true if this is an event handler attribute (starts with "on")
    pub fn is_event_handler(&self) -> bool {
        self.name.value.starts_with("on")
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub type_params: Option<Vec<TypeExpression>>,  // For turbofish syntax: func::<T>()
}

#[derive(Debug, Clone)]
pub struct MacroCall {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum CaptureMode {
    ByReference,       // Capture by immutable reference (&)
    ByMutableReference, // Capture by mutable reference (&mut)
    ByValue,           // Capture by value (move)
}

#[derive(Debug, Clone)]
pub struct CapturedVariable {
    pub name: Identifier,
    pub mode: CaptureMode,
}

#[derive(Debug, Clone)]
pub struct LambdaParameter {
    pub name: Identifier,
    pub type_annotation: Option<TypeExpression>,
}

#[derive(Debug, Clone)]
pub struct LambdaExpression {
    pub parameters: Vec<LambdaParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Box<Expression>,
    pub captures: Vec<CapturedVariable>,  // Variables captured from environment
}

#[derive(Debug, Clone)]
pub struct ComponentDefinition {
    pub name: Identifier,
    pub parameters: Vec<FunctionParameter>,
    pub is_client: bool,  // Components are client-side by default
    pub body: BlockStatement,  // Component body contains statements
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub trait_name: Option<Identifier>,  // None for inherent impl, Some for trait impl
    pub lifetime_params: Vec<Lifetime>,  // Lifetime parameters like <'a, 'b>
    pub type_params: Vec<TypeParam>,  // Generic type parameters like <T>, <T: Display>
    pub type_name: Identifier,  // The type being implemented (e.g., "Point")
    pub methods: Vec<ImplMethod>,
}

#[derive(Debug, Clone)]
pub struct ImplMethod {
    pub name: Identifier,
    pub parameters: Vec<FunctionParameter>,  // First parameter is usually &self or self
    pub return_type: Option<TypeExpression>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct TraitDefinition {
    pub name: Identifier,
    pub lifetime_params: Vec<Lifetime>,  // Lifetime parameters like <'a, 'b>
    pub type_params: Vec<TypeParam>,  // Generic type parameters like <T>, <T: Display>
    pub methods: Vec<TraitMethod>,
}

#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub name: Identifier,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeExpression>,
}