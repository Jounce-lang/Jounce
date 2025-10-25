# jounce-markdown

Markdown parsing and HTML rendering with GitHub Flavored Markdown (GFM) support.

## Features

- ✅ **Markdown Parsing** - Parse markdown into AST
- ✅ **HTML Rendering** - Convert markdown to HTML
- ✅ **Standard Elements** - Headings, paragraphs, lists, links, images, code blocks
- ✅ **GFM Support** - Task lists, strikethrough, tables
- ✅ **Code Blocks** - Syntax highlighting support with language tags
- ✅ **XSS Protection** - HTML sanitization for safe rendering
- ✅ **Markdown Builder** - Fluent API for creating markdown programmatically

## Installation

```bash
jnc pkg add jounce-markdown
```

## Quick Start

```jounce
use jounce_markdown::parse_markdown;

let markdown = "# Hello World\n\nThis is **bold** text.";
let html = parse_markdown(markdown);

println(html);
// Output: <h1>Hello World</h1><p>This is <strong>bold</strong> text.</p>
```

## Usage

### Parsing Markdown

```jounce
use jounce_markdown::{Markdown, parse_markdown};

// Simple parse
let html = parse_markdown("# Title");

// With custom options
let md = Markdown::new("# Title")
    .with_sanitize(true)
    .parse();

let html = md.to_html();
```

### Headings

```jounce
let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
let html = parse_markdown(md);
// Renders: <h1>H1</h1><h2>H2</h2>...
```

### Paragraphs

```jounce
let md = "This is a paragraph.\n\nThis is another paragraph.";
let html = parse_markdown(md);
// Renders: <p>This is a paragraph.</p><p>This is another paragraph.</p>
```

### Lists

```jounce
// Unordered list
let md = "- Item 1\n- Item 2\n- Item 3";
let html = parse_markdown(md);
// Renders: <ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>

// Ordered list
let md = "1. First\n2. Second\n3. Third";
let html = parse_markdown(md);
// Renders: <ol><li>First</li><li>Second</li><li>Third</li></ol>
```

### Code Blocks

```jounce
let md = "```javascript\nconst x = 5;\nconsole.log(x);\n```";
let html = parse_markdown(md);
// Renders: <pre><code class="language-javascript">const x = 5;
// console.log(x);</code></pre>

// Without language
let md = "```\ncode here\n```";
let html = parse_markdown(md);
// Renders: <pre><code>code here</code></pre>
```

### Inline Code

```jounce
let md = "Use `const` for constants";
let html = parse_markdown(md);
// Renders: <p>Use <code>const</code> for constants</p>
```

### Links

```jounce
let md = "[Google](https://google.com)";
let html = parse_markdown(md);
// Renders: <a href="https://google.com">Google</a>
```

### Images

```jounce
let md = "![Logo](logo.png)";
let html = parse_markdown(md);
// Renders: <img src="logo.png" alt="Logo"/>
```

### Blockquotes

```jounce
let md = "> This is a quote";
let html = parse_markdown(md);
// Renders: <blockquote>This is a quote</blockquote>
```

### Horizontal Rules

```jounce
let md = "---";
let html = parse_markdown(md);
// Renders: <hr/>
```

### Task Lists (GFM)

```jounce
let md = "- [ ] Todo item\n- [x] Completed item\n- [ ] Another todo";
let html = parse_markdown(md);
// Renders: <ul class="task-list">
//   <li class="task-list-item"><input type="checkbox" disabled/> Todo item</li>
//   <li class="task-list-item"><input type="checkbox" disabled checked/> Completed item</li>
//   ...
```

### Sanitization

```jounce
use jounce_markdown::{parse_markdown_safe, parse_markdown_unsafe};

// Safe (default) - escapes HTML
let html = parse_markdown_safe("<script>alert('xss')</script>");
// Renders: &lt;script&gt;alert('xss')&lt;/script&gt;

// Unsafe - allows raw HTML (use with caution)
let md = Markdown::new("<div>Raw HTML</div>")
    .with_sanitize(false)
    .parse()
    .to_html();
```

### MarkdownBuilder (Fluent API)

```jounce
use jounce_markdown::MarkdownBuilder;

let md = MarkdownBuilder::new()
    .heading(1, "My Document")
    .paragraph("This is the introduction.")
    .heading(2, "Features")
    .list_item("Fast")
    .list_item("Simple")
    .list_item("Powerful")
    .heading(2, "Code Example")
    .code_block("fn main() {\n    println(\"Hello!\");\n}", "jounce")
    .build();

let html = parse_markdown(md);
```

### Building Task Lists

```jounce
let md = MarkdownBuilder::new()
    .heading(2, "My Tasks")
    .task_item("Buy groceries", false)
    .task_item("Finish project", true)
    .task_item("Write docs", false)
    .build();

let html = parse_markdown(md);
```

### Working with AST

```jounce
use jounce_markdown::{Markdown, Node, NodeType};

let md = Markdown::new("# Title").parse();
let ast = md.get_ast();

if ast.is_some() {
    let doc = ast.unwrap();
    // Traverse AST
    for child in doc.children {
        match child.node_type {
            NodeType::Heading => {
                println("Heading level: " + child.level.to_string());
                println("Content: " + child.content);
            }
            _ => {}
        }
    }
}
```

### Complete Example

```jounce
use jounce_markdown::{MarkdownBuilder, parse_markdown};

// Build markdown programmatically
let md = MarkdownBuilder::new()
    .heading(1, "Project README")
    .paragraph("A simple markdown example.")
    .heading(2, "Installation")
    .code_block("jnc pkg add my-package", "bash")
    .heading(2, "Features")
    .list_item("Fast parsing")
    .list_item("HTML rendering")
    .list_item("GFM support")
    .heading(2, "Todo")
    .task_item("Add more tests", true)
    .task_item("Improve docs", false)
    .horizontal_rule()
    .paragraph("Made with Jounce")
    .build();

// Parse to HTML
let html = parse_markdown(md);

// Use in your app
println(html);
```

## API Reference

### parse_markdown

```jounce
parse_markdown(source: string) -> string
```

Parse markdown and render to HTML with sanitization enabled.

### parse_markdown_safe

```jounce
parse_markdown_safe(source: string) -> string
```

Parse markdown with explicit sanitization (same as `parse_markdown`).

### parse_markdown_unsafe

```jounce
parse_markdown_unsafe(source: string) -> string
```

Parse markdown without sanitization. **Warning**: Only use with trusted input.

### Markdown

```jounce
Markdown::new(source: string) -> Markdown
markdown.with_sanitize(sanitize: bool) -> Markdown
markdown.parse() -> Markdown
markdown.to_html() -> string
markdown.get_ast() -> Option<Node>
```

### MarkdownBuilder

```jounce
MarkdownBuilder::new() -> MarkdownBuilder
builder.heading(level: int, text: string) -> MarkdownBuilder
builder.paragraph(text: string) -> MarkdownBuilder
builder.code_block(code: string, language: string) -> MarkdownBuilder
builder.list_item(text: string) -> MarkdownBuilder
builder.ordered_list_item(num: int, text: string) -> MarkdownBuilder
builder.link(text: string, url: string) -> MarkdownBuilder
builder.image(alt: string, url: string) -> MarkdownBuilder
builder.blockquote(text: string) -> MarkdownBuilder
builder.horizontal_rule() -> MarkdownBuilder
builder.task_item(text: string, checked: bool) -> MarkdownBuilder
builder.build() -> string
```

### Node

```jounce
Node::new(node_type: NodeType) -> Node
node.with_content(content: string) -> Node
node.with_level(level: int) -> Node
node.with_url(url: string) -> Node
node.add_child(child: Node) -> Node
```

### NodeType

```jounce
NodeType::Document
NodeType::Heading
NodeType::Paragraph
NodeType::Text
NodeType::Strong
NodeType::Emphasis
NodeType::Code
NodeType::CodeBlock
NodeType::Link
NodeType::Image
NodeType::List
NodeType::ListItem
NodeType::Blockquote
NodeType::HorizontalRule
NodeType::Table
NodeType::TableRow
NodeType::TableCell
NodeType::Strikethrough
NodeType::TaskList
NodeType::TaskItem
```

## Supported Markdown

### Standard Markdown
- ✅ Headings (#, ##, ###, etc.)
- ✅ Paragraphs
- ✅ Bold (**text**)
- ✅ Italic (*text*)
- ✅ Inline code (`code`)
- ✅ Code blocks (```)
- ✅ Links ([text](url))
- ✅ Images (![alt](url))
- ✅ Unordered lists (-, *, +)
- ✅ Ordered lists (1., 2., 3.)
- ✅ Blockquotes (>)
- ✅ Horizontal rules (---, ***, ___)

### GitHub Flavored Markdown (GFM)
- ✅ Task lists (- [ ], - [x])
- ✅ Strikethrough (~~text~~)
- ⏸️ Tables (coming soon)
- ⏸️ Autolinks (coming soon)

## Best Practices

1. **Sanitize User Input** - Always use sanitization for user-generated content
2. **Use MarkdownBuilder** - For programmatic markdown generation
3. **Cache Parsed Results** - Parse once, render multiple times if needed
4. **Language Tags** - Always specify language in code blocks for syntax highlighting
5. **Escape HTML** - Use sanitization unless you trust the source

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
