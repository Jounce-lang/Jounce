# jounce-docs

Documentation generation from source code with doc comments, markdown, and API references for Jounce packages.

## Features

- ✅ **Doc Comment Parsing** - Extract /// comments from code
- ✅ **Markdown Generation** - Generate beautiful documentation
- ✅ **API Reference** - Auto-generate API docs
- ✅ **Example Extraction** - Pull code examples from comments
- ✅ **Symbol Categorization** - Functions, structs, enums, traits
- ✅ **Tag Support** - @param, @return, @example, @throws
- ✅ **Table of Contents** - Auto-generated TOC
- ✅ **Configuration** - Flexible output options

## Installation

```bash
jnc pkg add jounce-docs
```

## Quick Start

```jounce
use jounce_docs::{DocParser, ApiReferenceGenerator};

fn main() {
    // Parse source code
    let mut parser = DocParser::new();
    parser.parse_file("src/lib.jnc", file_content);

    // Generate API reference
    let mut api_ref = ApiReferenceGenerator::new("my-package", "1.0.0");

    for symbol in parser.get_public_symbols() {
        api_ref.add_symbol(symbol);
    }

    let docs = api_ref.generate();
    println(docs);
}
```

## Usage

### Parsing Doc Comments

```jounce
use jounce_docs::{DocParser, DocComment};

let parser = DocParser::new();

let comment = "
/// Adds two numbers
///
/// This function performs addition of two integers.
///
/// @param a First number
/// @param b Second number
/// @return Sum of a and b
/// @example
/// let result = add(5, 3);
/// assert_eq!(result, 8);
";

let doc = parser.parse_doc_comment(comment);

println(doc.summary);      // "Adds two numbers"
println(doc.description);   // "This function performs addition..."
```

### Creating Doc Comments

```jounce
use jounce_docs::DocComment;

let mut doc = DocComment::new()
    .with_summary("Creates a new user")
    .with_description("This function creates a user in the database");

doc.add_tag("param", "name: string - User's name");
doc.add_tag("param", "email: string - User's email");
doc.add_tag("return", "User - The created user object");
doc.add_example("let user = create_user(\"John\", \"john@example.com\");");
```

### Generating API References

```jounce
use jounce_docs::{ApiReferenceGenerator, Symbol, SymbolType, DocComment};

let mut api_ref = ApiReferenceGenerator::new("my-lib", "1.0.0");

// Add a function
let doc = DocComment::new()
    .with_summary("Calculates the sum")
    .with_description("Adds two numbers together");

let mut symbol = Symbol::new("add", SymbolType::Function)
    .with_doc(doc)
    .with_signature("pub fn add(a: int, b: int) -> int");

symbol.visibility = "pub";

api_ref.add_symbol(symbol);

// Generate markdown
let markdown = api_ref.generate();
```

### Markdown Generation

```jounce
use jounce_docs::{MarkdownGenerator, MarkdownSection};

let mut generator = MarkdownGenerator::new("User Guide");

generator.add_section(
    MarkdownSection::new("Installation", "Run `jnc pkg add my-package`")
        .with_level(2)
);

generator.add_section(
    MarkdownSection::new("Usage", "Import and use the package")
        .with_level(2)
);

let markdown = generator.generate();
```

### Example Extraction

```jounce
use jounce_docs::{ExampleExtractor, DocComment};

let mut extractor = ExampleExtractor::new();

let mut doc = DocComment::new();
doc.add_example("
let x = 5;
let y = 10;
let sum = x + y;
");

extractor.extract_from_doc(doc, "addition");

// Generate examples file
let examples_md = extractor.generate_examples_file();
```

### Symbol Types

```jounce
use jounce_docs::{Symbol, SymbolType};

// Function
let func = Symbol::new("process", SymbolType::Function);

// Struct
let struct_sym = Symbol::new("User", SymbolType::Struct);

// Enum
let enum_sym = Symbol::new("Status", SymbolType::Enum);

// Trait
let trait_sym = Symbol::new("Validator", SymbolType::Trait);

// All types: Function, Struct, Enum, Trait, Impl, Const, Module
```

### Configuration

```jounce
use jounce_docs::DocsConfig;

let config = DocsConfig::new("my-package", "1.0.0")
    .with_output_dir("api-docs/")
    .include_private();

println("Output dir: {}", config.output_dir);
println("Include private: {}", config.include_private);
```

## Doc Comment Format

### Basic Format

```jounce
/// Short summary on first line
///
/// Longer description can span multiple lines.
/// Provides more details about the function.
///
/// @param name Description of parameter
/// @return Description of return value
/// @example
/// Example code here
pub fn my_function(name: string) -> string {
    return "Hello, " + name;
}
```

### Supported Tags

- `@param` - Parameter documentation
- `@return` - Return value documentation
- `@example` - Code example
- `@throws` - Exceptions/errors thrown

### Example with All Tags

```jounce
/// Processes user data
///
/// This function validates and processes user information,
/// storing it in the database.
///
/// @param user User object to process
/// @param validate Whether to validate data
/// @return Processed user ID
/// @throws ValidationError if validation fails
/// @example
/// let user = User { name: "John", age: 30 };
/// let id = process_user(user, true);
pub fn process_user(user: User, validate: bool) -> string {
    // ...
}
```

## API Reference

### DocComment

```jounce
DocComment::new() -> DocComment
doc.with_summary(summary: string) -> DocComment
doc.with_description(description: string) -> DocComment
doc.add_tag(tag: string, value: string)
doc.add_example(example: string)
```

### Symbol

```jounce
Symbol::new(name: string, symbol_type: SymbolType) -> Symbol
symbol.with_doc(doc: DocComment) -> Symbol
symbol.with_signature(signature: string) -> Symbol
symbol.is_public() -> bool
```

### SymbolType

```jounce
SymbolType::Function
SymbolType::Struct
SymbolType::Enum
SymbolType::Trait
SymbolType::Impl
SymbolType::Const
SymbolType::Module
```

### DocParser

```jounce
DocParser::new() -> DocParser
parser.parse_file(file_path: string, content: string)
parser.parse_doc_comment(comment: string) -> DocComment
parser.get_public_symbols() -> Array<Symbol>
parser.get_symbols_by_type(symbol_type: SymbolType) -> Array<Symbol>
```

### MarkdownGenerator

```jounce
MarkdownGenerator::new(title: string) -> MarkdownGenerator
generator.add_section(section: MarkdownSection)
generator.generate() -> string
```

### MarkdownSection

```jounce
MarkdownSection::new(title: string, content: string) -> MarkdownSection
section.with_level(level: int) -> MarkdownSection
section.to_markdown() -> string
```

### ApiReferenceGenerator

```jounce
ApiReferenceGenerator::new(package_name: string, version: string) -> ApiReferenceGenerator
generator.add_symbol(symbol: Symbol)
generator.generate() -> string
```

### ExampleExtractor

```jounce
ExampleExtractor::new() -> ExampleExtractor
extractor.extract_from_doc(doc: DocComment, symbol_name: string)
extractor.generate_examples_file() -> string
```

### DocsConfig

```jounce
DocsConfig::new(package_name: string, version: string) -> DocsConfig
config.with_output_dir(dir: string) -> DocsConfig
config.include_private() -> DocsConfig
```

## Generated Documentation Structure

```
docs/
├── README.md              # Package overview
├── API.md                 # API reference
├── EXAMPLES.md            # Code examples
└── guides/
    ├── getting-started.md
    └── advanced.md
```

## Best Practices

1. **Write Clear Summaries**: First line should be concise and descriptive
2. **Add Examples**: Include at least one example per public function
3. **Document Parameters**: Describe all parameters and return values
4. **Use Proper Tags**: Follow tag conventions consistently
5. **Keep It Updated**: Regenerate docs when code changes
6. **Include Types**: Specify types in @param and @return tags

## Examples

### Documenting a Function

```jounce
/// Validates an email address
///
/// Checks if the provided string is a valid email format
/// using regex pattern matching.
///
/// @param email Email address to validate
/// @return true if valid, false otherwise
/// @example
/// let is_valid = validate_email("user@example.com");
/// assert!(is_valid);
pub fn validate_email(email: string) -> bool {
    // Implementation
}
```

### Documenting a Struct

```jounce
/// User account information
///
/// Represents a user account with authentication details
/// and profile information.
pub struct User {
    /// User's unique identifier
    pub id: string,

    /// User's email address
    pub email: string,

    /// User's display name
    pub name: string,
}
```

### Generating Complete Documentation

```jounce
use jounce_docs::{DocParser, ApiReferenceGenerator, DocsConfig};

fn generate_docs() {
    let config = DocsConfig::new("my-package", "1.0.0");

    let mut parser = DocParser::new();
    parser.parse_file("src/lib.jnc", read_file("src/lib.jnc"));

    let mut api_ref = ApiReferenceGenerator::new(
        config.package_name,
        config.version
    );

    for symbol in parser.get_public_symbols() {
        api_ref.add_symbol(symbol);
    }

    let docs = api_ref.generate();
    write_file(config.output_dir + "API.md", docs);
}
```

## License

MIT

## Version

0.1.0
