# Jounce Code Formatting Guide

## Overview

Jounce includes a built-in code formatter that enforces consistent, opinionated code style across your entire codebase. The formatter is available through both the CLI (`raven fmt`) and LSP (Language Server Protocol) for seamless editor integration.

## Table of Contents

- [Formatting Philosophy](#formatting-philosophy)
- [Core Formatting Rules](#core-formatting-rules)
- [JSX Formatting](#jsx-formatting)
- [CLI Usage](#cli-usage)
- [Editor Integration](#editor-integration)
- [CI/CD Integration](#cicd-integration)
- [Configuration](#configuration)
- [Examples](#examples)

---

## Formatting Philosophy

Jounce's formatter follows these core principles:

1. **Consistency Over Preference**: The formatter enforces a single, opinionated style to eliminate debates about code formatting
2. **Readability First**: Code should be easy to read and understand at a glance
3. **JSX-Aware**: Special formatting rules for JSX elements to match React/JSX community standards
4. **Zero Configuration**: Works out of the box with sensible defaults
5. **Fast**: Formats files in milliseconds for instant feedback

---

## Core Formatting Rules

### Indentation

- **4 spaces** per indentation level (no tabs)
- Consistent indentation for all block structures (functions, structs, enums, etc.)

```raven
fn calculate(x: i32) {
    let result = x * 2;
    return result;
}
```

### Line Length

- **Soft limit**: 100 characters
- Long lines are not automatically wrapped (future enhancement)
- Use your judgment to break long expressions

### Spacing

- **Operators**: Space around binary operators (`a + b`, `x == y`)
- **Commas**: Space after, not before (`a, b, c`)
- **Colons**: Space after in type annotations (`x: i32`)
- **Blocks**: Opening brace on same line as declaration

```raven
// Good
let x: i32 = 1 + 2;
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// The formatter will fix this
let x:i32=1+2;
fn add(a:i32,b:i32)->i32{return a+b;}
```

### Semicolons

- Required at the end of statements
- Not required for last expression in blocks (implicit return)

```raven
fn get_value() -> i32 {
    let x = 42;
    x  // No semicolon - implicit return
}
```

### Comments

- Comments are preserved in their original positions
- Inline comments maintain spacing from code

---

## JSX Formatting

### Simple Elements (Single Line)

Elements with **no children** or **only text children** and **≤3 attributes** are formatted on a single line:

```raven
// Simple elements remain inline
let button = <Button>Click Me</Button>;
let input = <Input type="text" value={name} />;
let link = <a href="/home">Home</a>;
```

### Complex Elements (Multi-Line)

Elements with **nested JSX children** or **>3 attributes** are formatted on multiple lines:

```raven
// Nested JSX elements → multi-line
let elem = <div>
    <Header />
    <Content>
        <Article title="Hello" />
    </Content>
    <Footer />
</div>;

// Many attributes (>3) → multi-line
let component = <Component
    prop1={value1}
    prop2={value2}
    prop3={value3}
    prop4={value4}
/>;
```

### JSX Indentation Rules

1. **Child elements**: Indented 4 spaces from parent
2. **Closing tags**: Aligned with opening tag
3. **Attributes** (when multi-line): Indented 4 spaces from opening `<`
4. **Expressions**: Standard indentation within `{}`

```raven
let app = <div class="container">
    <Header title="My App" />
    <Content>
        {items.map(|item| <Item key={item.id} data={item} />)}
    </Content>
    <Footer>
        <p>Copyright 2025</p>
    </Footer>
</div>;
```

### JSX Attributes

- **String literals**: No braces (`class="container"`)
- **Expressions**: Use braces (`onClick={handler}`)
- **Boolean shorthand**: No value (`disabled` instead of `disabled={true}`)

```raven
// Formatted JSX attributes
<Button
    class="primary"
    onClick={handleClick}
    disabled
    data-id={userId}
/>
```

---

## CLI Usage

### Basic Commands

```bash
# Format and print to stdout (doesn't modify file)
raven fmt file.jnc

# Format and write changes to file
raven fmt --write file.jnc

# Format multiple files
raven fmt --write src/**/*.jnc

# Check if files are formatted (useful for CI)
raven fmt --check src/
```

### Command Options

| Option | Description | Exit Code |
|--------|-------------|-----------|
| (none) | Print formatted output to stdout | 0 |
| `--write` | Modify files in place | 0 on success |
| `--check` | Check if files are formatted | 0 if formatted, 1 if not |

### Examples

```bash
# Format a single file and save
raven fmt --write app.jnc

# Format all .jnc files in a directory
raven fmt --write src/**/*.jnc

# Check formatting in CI
raven fmt --check src/
if [ $? -ne 0 ]; then
  echo "Files are not formatted! Run: raven fmt --write src/"
  exit 1
fi
```

---

## Editor Integration

### VS Code / LSP-Compatible Editors

The Jounce LSP server provides automatic formatting through the `textDocument/formatting` request.

#### Enable Format on Save

Add to your editor settings:

**VS Code** (`.vscode/settings.json`):
```json
{
  "editor.formatOnSave": true,
  "[raven]": {
    "editor.defaultFormatter": "jounce.lsp"
  }
}
```

**Neovim** (Lua config):
```lua
vim.api.nvim_create_autocmd("BufWritePre", {
  pattern = "*.jnc",
  callback = function()
    vim.lsp.buf.format({ async = false })
  end,
})
```

#### Manual Formatting

- **VS Code**: `Shift + Alt + F` (Windows/Linux) or `Shift + Option + F` (Mac)
- **Neovim**: `:lua vim.lsp.buf.format()`
- **Sublime Text**: `Ctrl + Shift + P` → "Format Document"

---

## CI/CD Integration

### GitHub Actions

Add a formatting check to your CI pipeline:

```yaml
name: CI

on: [push, pull_request]

jobs:
  format-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Jounce
        run: |
          curl -L https://jounce.dev/install.sh | sh
          echo "$HOME/.jounce/bin" >> $GITHUB_PATH

      - name: Check Formatting
        run: |
          raven fmt --check src/
          if [ $? -ne 0 ]; then
            echo "❌ Code is not formatted correctly"
            echo "Run: raven fmt --write src/"
            exit 1
          fi
          echo "✅ All files are properly formatted"
```

### Pre-commit Hook

Automatically format files before committing:

```bash
# .git/hooks/pre-commit
#!/bin/bash
FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.jnc$')

if [ -n "$FILES" ]; then
  echo "Formatting .jnc files..."
  raven fmt --write $FILES
  git add $FILES
fi
```

Make the hook executable:
```bash
chmod +x .git/hooks/pre-commit
```

---

## Configuration

### Current Defaults

```rust
indent_size: 4        // Spaces per indentation level
max_line_length: 100  // Soft line length limit (not enforced)
use_spaces: true      // Use spaces instead of tabs
trailing_comma: true  // Add trailing commas in multi-line lists
```

### Future Configuration

Configuration files (`.jncfmt.toml`) are planned for future releases to allow customization of:
- Indent size
- Line length
- JSX formatting preferences
- Custom formatting rules

---

## Examples

### Before and After: Function Formatting

**Before:**
```raven
fn calculate(x:i32,y:i32)->i32{let result=x+y;return result;}
```

**After:**
```raven
fn calculate(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}
```

### Before and After: Struct Formatting

**Before:**
```raven
struct User{name:String,email:String,age:i32}
```

**After:**
```raven
struct User {
    name: String,
    email: String,
    age: i32,
}
```

### Before and After: Match Expression

**Before:**
```raven
let result=match status{Status::Active=>"active",Status::Inactive=>"inactive",_=>"unknown"};
```

**After:**
```raven
let result = match status {
    Status::Active => "active",
    Status::Inactive => "inactive",
    _ => "unknown",
};
```

### Before and After: JSX Formatting

**Before:**
```raven
let app=<div><Header title="App"/><Content>{items.map(|item|<Item key={item.id} data={item}/>)}</Content><Footer/></div>;
```

**After:**
```raven
let app = <div>
    <Header title="App" />
    <Content>
        {items.map(|item| <Item key={item.id} data={item} />)}
    </Content>
    <Footer />
</div>;
```

### Before and After: Complex JSX with Many Attributes

**Before:**
```raven
let form=<Input type="text" placeholder="Enter name" value={name} onChange={handleChange} required disabled={!isActive}/>;
```

**After:**
```raven
let form = <Input
    type="text"
    placeholder="Enter name"
    value={name}
    onChange={handleChange}
    required
    disabled={!isActive}
/>;
```

---

## Best Practices

1. **Format Early, Format Often**: Run the formatter frequently to catch issues early
2. **Enable Format on Save**: Let your editor automatically format files
3. **Use in CI**: Enforce formatting standards in your CI pipeline
4. **Don't Fight the Formatter**: Accept the opinionated style and focus on code logic
5. **Format Before Commits**: Keep your git history clean with properly formatted code

---

## Troubleshooting

### Formatter Not Working

1. **Check file syntax**: The formatter requires valid Jounce syntax
   ```bash
   raven compile --check file.jnc
   ```

2. **Check LSP connection**: Ensure your editor is connected to the Jounce LSP
   - VS Code: Check "Output" → "Jounce Language Server"
   - Neovim: Run `:LspInfo`

3. **Update Jounce**: Ensure you're using the latest version
   ```bash
   raven --version
   raven update  # If using package manager
   ```

### Formatting Errors

If the formatter encounters an error:
- **Syntax errors**: Fix syntax issues first, then format
- **Parse errors**: Check for unclosed braces, missing semicolons
- **JSX errors**: Ensure JSX tags are properly closed

---

## Formatter Implementation Details

The Jounce formatter:

1. **Parses source code** into an Abstract Syntax Tree (AST)
2. **Traverses the AST** using a visitor pattern
3. **Generates formatted code** with consistent indentation and spacing
4. **Preserves semantics**: Formatting never changes code behavior
5. **Handles JSX intelligently**: Special rules for JSX elements

### Performance

- **Fast**: < 200ms for typical files (< 1000 lines)
- **Incremental**: Only formats changed regions (LSP range formatting)
- **Parallel**: Can format multiple files concurrently

---

## Future Enhancements

Planned formatter improvements:

- [ ] Automatic line wrapping for long lines
- [ ] Comment formatting and alignment
- [ ] Import statement sorting
- [ ] Configuration file support (`.jncfmt.toml`)
- [ ] Custom formatter plugins
- [ ] More granular JSX formatting options

---

## Resources

- **Main Documentation**: [README.md](../../README.md)
- **Getting Started**: [GETTING_STARTED.md](../../GETTING_STARTED.md)
- **LSP Features**: [LSP_FEATURES.md](./LSP_FEATURES.md)
- **Parser Limitations**: [PARSER_LIMITATIONS.md](./PARSER_LIMITATIONS.md)
- **Registry**: https://jounce-registry.fly.dev

---

**Last Updated**: 2025-10-22
**Version**: 0.1.0
**Status**: Production Ready ✅
