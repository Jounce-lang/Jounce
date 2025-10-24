# LSP Features Guide

## Overview

Jounce's Language Server Protocol (LSP) implementation provides intelligent IDE features for enhanced developer experience. The LSP server offers context-aware completions, hover information, and real-time diagnostics.

## Features

### 1. Context-Aware Completions

The LSP automatically detects what you're trying to complete based on cursor position and provides relevant suggestions.

#### Context Types

##### Namespace Access (`::`)
When typing after `::`, you get members of that namespace.

```raven
Math::  // Shows: abs, min, max, sqrt, pow, round, floor, ceil, random
Signal::  // Shows: new
String::  // Shows: split, trim, contains
```

**How it works**: The LSP recognizes the `::` operator and filters completions to show only members of the specified namespace.

##### Member Access (`.`)
When typing after `.`, you get methods and fields available on that object.

```raven
let count = Signal::new(0);
count.  // Shows: get, set
```

```raven
let items = vec![1, 2, 3];
items.  // Shows: iter, map, filter, len, push
```

**How it works**: The LSP detects the `.` operator and provides common methods that work on the object type.

##### JSX Tag Context
When typing after `<`, you get available JSX tags and components.

```raven
component App() {
    <  // Shows: div, button, input, h1, p, ul, li, App (and other components)
}
```

**How it works**: The LSP parses the file to find all component definitions and combines them with standard HTML elements.

##### JSX Attribute Context
When typing attributes inside a JSX tag, you get valid attributes for that tag.

```raven
<div   // Shows: class, id, style, onclick
<input   // Shows: class, id, style, value, oninput, placeholder
<button   // Shows: class, id, style, onclick
<form   // Shows: class, id, style, onsubmit
```

**How it works**: The LSP tracks which JSX tag you're inside and suggests appropriate HTML attributes and event handlers.

##### Statement Start
At the beginning of a line or after `{`, you get keywords and declarations.

```raven
  // Shows: component, fn, let, if, for, while, match, struct, enum, return
```

**How it works**: The LSP detects when you're at a statement boundary and prioritizes language keywords and declarations.

##### Function Call Context
When typing inside function parentheses, you get parameter hints and general completions.

```raven
Math::pow(  // Parameter hints for pow(base, exponent)
```

**How it works**: The LSP finds the unclosed `(` and identifies the function name to provide parameter information.

##### General Context
In expressions and other general contexts, you get all available completions.

```raven
let x =   // Shows: keywords, stdlib functions, reactive primitives, JSX snippets, local variables
```

### 2. Hover Information ✅

Hover over identifiers to see documentation, type information, function signatures, and definitions.

**Supported Items**:
- ✅ Standard library functions (Math, Signal, String, etc.)
- ✅ Reactive primitives (Signal, Computed, Effect, Resource)
- ✅ User-defined functions with full signatures
- ✅ Variables with type annotations
- ✅ Constants with type information
- ✅ Components with parameter lists
- ✅ Struct definitions
- ✅ Enum definitions

**Examples**:

**Function Hover**:
```raven
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

let result = add(5, 10);
        // Hover over 'add' shows:
        // ```raven
        // fn add(x: i32, y: i32) -> i32
        // ```
        // **Function**
```

**Variable Hover**:
```raven
let count: i32 = 0;
        // Hover over 'count' shows:
        // ```raven
        // let count: i32
        // ```
        // **Variable** with type `i32`
```

**Struct Hover**:
```raven
struct Point {
    x: f64,
    y: f64,
}
        // Hover over 'Point' shows:
        // ```raven
        // struct Point {
        //     x: f64,
        //     y: f64,
        // }
        // ```
        // **Struct** definition
```

**Stdlib Function Hover**:
```raven
let x = Math::abs(-42);
        // Hover over 'abs' shows:
        // fn abs(x: f64) -> f64
        // Returns the absolute value of a number
        // Example: let positive = Math::abs(-42.5); // 42.5
```

### 2.5. Signature Help ✅ NEW

Get real-time parameter hints while typing function calls. The LSP automatically shows:
- Function signature with parameter types
- Current parameter highlighted
- Parameter documentation

**How it works**: When you type `(` after a function name, the LSP shows the function signature and highlights the parameter you're currently typing.

**Example**:
```raven
fn calculate(x: i32, y: i32, z: i32) -> i32 {
    return x + y + z;
}

let result = calculate(1, |    // ← Cursor here
        // Shows:
        // fn calculate(x: i32, y: i32, z: i32) -> i32
        // Active parameter: y: i32 (parameter 2)
```

**Keyboard Shortcuts**:
- **VS Code/Most Editors**: Signature help appears automatically when you type `(`
- **Manual Trigger**: `Ctrl+Shift+Space` (VS Code) or `Cmd+Shift+Space` (macOS)

**Features**:
- ✅ Automatic detection of function calls
- ✅ Parameter index tracking (counts commas)
- ✅ Nested function call support
- ✅ Multi-line signature support

### 3. Real-Time Diagnostics

The LSP analyzes your code as you type and reports errors.

**Analysis Stages**:
1. **Lexical Analysis** - Detects tokenization errors
2. **Parsing** - Identifies syntax errors
3. **Semantic Analysis** - Checks for undefined variables, scope issues
4. **Type Checking** - Verifies type compatibility

**Example**:
```raven
let x: i32 = "hello";  // Error: Type mismatch
```

### 4. Intelligent Completions

#### Keywords
- `component` - Define a component
- `fn` - Define a function
- `let` - Declare a variable
- `if`, `for`, `while`, `match` - Control flow
- `struct`, `enum` - Type definitions
- `@server`, `@client` - Code splitting annotations

#### Standard Library (70+ completions)

**Math Functions**:
- `Math::abs`, `Math::min`, `Math::max`, `Math::clamp`
- `Math::pow`, `Math::sqrt`, `Math::square`
- `Math::round`, `Math::floor`, `Math::ceil`
- `Math::sin`, `Math::cos`, `Math::tan`
- `Math::random`, `Math::random_int`

**Reactive Primitives**:
- `Signal::new` - Create reactive state
- `Computed::new` - Create derived values
- `Effect::new` - Side effects
- `Resource::new` - Async data loading

**HTTP Functions**:
- `HttpRequest::get`, `HttpRequest::post`
- `get` - Convenience function

**Collections**:
- `Collections::filter`, `Collections::map`, `Collections::find`

**String Functions**:
- `String::split`, `String::trim`, `String::contains`

**Storage** (Client-side):
- `Storage::set`, `Storage::get`, `Storage::set_json`

**Forms**:
- `Forms::validate_email`, `Forms::validate_required`

**Time**:
- `Time::now`, `Time::sleep`

**JSON**:
- `JSON::parse`, `JSON::stringify`

**Auth** (Server-side):
- `Auth::hash_password`, `Auth::verify_password`

#### JSX Snippets

**HTML Elements**:
- `<div>` - Container element
- `<button>` - Clickable button
- `<input>` - Text input field
- `<h1>` - Heading
- `<p>` - Paragraph
- `<ul>`, `<li>` - Lists
- `<form>` - Form container

**Common Patterns**:
- `counter` - Full counter component with Signal
- `list-map` - Map array to JSX elements
- `conditional` - Conditional rendering

## Usage in Editors

### VS Code

1. **Install Language Extension** (coming soon)
2. **Features automatically enabled**:
   - Autocomplete: Trigger with `Ctrl+Space`
   - Hover: Hover over any identifier
   - Diagnostics: Real-time error checking

### Vim/Neovim

1. **Install LSP client** (e.g., `coc.nvim`, `vim-lsp`)
2. **Configure Jounce LSP**:
```vim
" Example with coc.nvim
{
  "languageserver": {
    "raven": {
      "command": "raven",
      "args": ["lsp"],
      "filetypes": ["raven"],
      "rootPatterns": ["jounce.toml"]
    }
  }
}
```

### Emacs

1. **Install `lsp-mode`**
2. **Add Jounce configuration**:
```elisp
(lsp-register-client
 (make-lsp-client :new-connection (lsp-stdio-connection '("raven" "lsp"))
                  :major-modes '(raven-mode)
                  :server-id 'raven-lsp))
```

## Implementation Details

### Context Detection Algorithm

The LSP uses a multi-stage approach to detect context:

1. **Line Analysis**: Extract text before cursor position
2. **Pattern Matching**: Check for specific patterns (`::", `.`, `<`, etc.)
3. **Depth Tracking**: Track parentheses and JSX tag nesting
4. **Identifier Extraction**: Extract namespace/object names from context

### Completion Filtering

Completions are filtered based on detected context:

- **Namespace context**: Only show namespace members
- **Member context**: Only show object methods/fields
- **JSX tag context**: Only show HTML tags and components
- **JSX attribute context**: Only show valid attributes for that tag
- **Statement context**: Prioritize keywords and declarations
- **General context**: Show all available completions

### Performance Considerations

- **Incremental Parsing**: LSP re-analyzes only changed portions
- **Caching**: Completion lists are cached per context type
- **Lazy Loading**: Documentation loaded on-demand
- **Fast Context Detection**: O(n) time complexity where n = line length

## Testing

The LSP includes comprehensive tests for all context types:

```bash
# Run LSP tests
cargo test --lib lsp

# Run specific context test
cargo test test_context_detection_namespace
```

**Test Coverage**:
- ✅ Statement start context
- ✅ General context
- ✅ Namespace access (`::`)
- ✅ Member access (`.`)
- ✅ JSX tag context
- ✅ JSX attribute context
- ✅ Hover information
- ✅ Document management

## Future Enhancements

### Type-Aware Member Completions (Planned)
Currently, member completions show common methods. Future versions will use type checker information to show only methods available on the actual type.

```raven
// Future: Type-aware completions
let count: Signal<i32> = Signal::new(0);
count.  // Will show: get(), set(i32), update(fn(i32) -> i32)

let text: String = "hello";
text.  // Will show: len(), chars(), split(), trim(), etc.
```

### Parameter Hints (Planned)
Enhanced parameter hints showing:
- Parameter names and types
- Current parameter position
- Optional vs required parameters
- Default values

### Signature Help (Planned)
Real-time signature information as you type function calls.

### Go to Definition ✅
Jump to the definition of functions, types, variables, components, structs, and enums.

**How to Use**:
- **Ctrl+Click** (Cmd+Click on macOS) on any symbol
- Press **F12** with cursor on a symbol
- Right-click and select "Go to Definition"

**Supported Symbols**:
- Functions (`fn myFunction()`)
- Variables (`let myVar`)
- Constants (`const MY_CONST`)
- Components (`component MyComponent()`)
- Structs (`struct MyStruct`)
- Enums (`enum MyEnum`)

**Example**:
```raven
fn calculate(x: i32) -> i32 {
    return x * 2;
}

let result = calculate(10);
//           ^^^^^^^^^ Ctrl+Click here jumps to line 1
```

### Find References ✅
Find all usages of a symbol across your codebase.

**How to Use**:
- **Shift+F12** with cursor on a symbol
- Right-click and select "Find All References"

**Features**:
- Finds all usages including the definition
- Works across the entire document
- Shows line numbers and context for each reference
- Can optionally exclude the definition

**Example**:
```raven
fn calculate(x: i32) -> i32 {
    return x * 2;
}

let result = calculate(10);
let result2 = calculate(20);
//            ^^^^^^^^^ Find References shows: Line 1 (definition), Line 5, Line 6
```

### Rename Symbol ✅
Safely rename a symbol across all its usages.

**How to Use**:
- Press **F2** with cursor on a symbol
- Right-click and select "Rename Symbol"
- Enter the new name and press Enter

**Features**:
- Renames all occurrences atomically
- Validates identifier syntax
- Works across the entire document
- Prevents invalid identifier names

**Example**:
```raven
fn oldName() {
    return 42;
}

let x = oldName();
let y = oldName();

// Press F2 on "oldName" and type "newName"
// All 3 occurrences are renamed to "newName"
```

### Document Symbols ✅
Outline view of all symbols in the current file.

**How to Use**:
- **Ctrl+Shift+O** (Cmd+Shift+O on macOS) to open symbol list
- View the "Outline" panel in your editor

**Supported Symbols**:
- Functions with their signatures
- Variables and constants
- Components
- Structs and enums
- Type aliases

**Example Document Symbols**:
```raven
fn calculate() {}          // Function
let counter = 0;           // Variable
const MAX = 100;           // Constant
struct User {}             // Struct
enum Status {}             // Enum
component App() {}         // Component (shown as Class)
```

Shows in outline as:
- 📦 calculate (Function)
- 📝 counter (Variable)
- 📝 MAX (Variable)
- 🏗️ User (Struct)
- 🔢 Status (Enum)
- 📦 App (Class)

### Code Actions & Quick Fixes ✅
Automatic code actions and quick fixes for common errors. Available via `Cmd+.` (macOS) or `Ctrl+.` (Windows/Linux) in supported editors.

#### Quick Fix 1: "Did you mean?" Typo Fixes
Automatically fixes typos and undefined identifiers.

```raven
// Before:
let count = Signa::new(0);  // Error: undefined identifier

// Quick Fix: "Change to 'Signal'"
let count = Signal::new(0);  // ✅ Fixed
```

#### Quick Fix 2: Prefix Unused Variable with `_`
Suppresses warnings for intentionally unused variables.

```raven
// Before:
let count = 0;  // Warning: unused variable 'count'

// Quick Fix: "Rename to '_count'"
let _count = 0;  // ✅ Fixed - warning suppressed
```

#### Quick Fix 3: Add Type Cast
Automatically casts numeric types when there's a type mismatch.

```raven
// Before:
let x: f64 = 42;  // Error: type mismatch (expected f64, found i32)

// Quick Fix: "Cast to f64"
let x: f64 = 42 as f64;  // ✅ Fixed
```

#### Quick Fix 4: Add Missing Semicolon
Inserts missing semicolons at statement end.

```raven
// Before:
let x = 10  // Error: expected ';'

// Quick Fix: "Add semicolon"
let x = 10;  // ✅ Fixed
```

#### Quick Fix 5: Add Type Annotation
Adds explicit type annotations when type inference fails.

```raven
// Before:
let items = vec![];  // Error: type annotation needed

// Quick Fix: "Add type annotation: Vec<i32>"
let items: Vec<i32> = vec![];  // ✅ Fixed
```

#### Quick Fix 6: Remove Unused Import
Removes unused import statements.

```raven
// Before:
use raven_store::Signal;  // Warning: unused import

// Quick Fix: "Remove unused import"
// ✅ Line removed
```

## 3. Enhanced Diagnostics & Error Reporting

Jounce provides beautiful, actionable error messages with:
- ✅ **Rich formatting** with ANSI colors
- ✅ **Source code snippets** showing the error location
- ✅ **Helpful suggestions** for fixing common mistakes
- ✅ **"Did you mean...?"** fuzzy matching for typos
- ✅ **Error codes** for documentation lookup (E001-E018, W001-W005)

### Error Message Format

```
error: type mismatch: expected `f64`, found `i32`
  --> app.jnc:42:10
   41 | let total: f64 = 100;
   42 | let result = calculate(total);
      |          ^^^^^^^^^^^^^^^^^^^
  help: consider converting `i32` to `f64`
  [E001]
```

### Error Codes Reference

#### Type Errors (E001-E006)

- **E001 - Type mismatch**: Expected one type, found another
- **E002 - Undefined variable**: Variable not declared in scope
- **E003 - Undefined function**: Function not found or not imported
- **E004 - Syntax error**: Invalid syntax or missing tokens
- **E005 - Borrow checker error**: Memory safety violation
- **E006 - Invalid JSX**: JSX syntax errors

#### Module & Import Errors (E007-E009)

- **E007 - Module not found**: Module doesn't exist or not installed
- **E008 - Import item not found**: Item doesn't exist in module
- **E009 - Circular dependency**: Modules depend on each other

#### JSX Errors (E010-E012)

- **E010 - Unclosed JSX element**: Missing closing tag
- **E011 - Mismatched JSX tags**: Opening and closing tags don't match
- **E012 - Invalid JSX attribute**: Attribute not valid for element

#### Async/Await Errors (E013-E014)

- **E013 - Await non-async function**: Cannot await synchronous function
- **E014 - Missing return type**: Function missing return type annotation

#### Type System Errors (E015-E018)

- **E015 - Type annotation needed**: Compiler cannot infer type
- **E016 - Missing struct field**: Required field not provided
- **E017 - Unknown struct field**: Field doesn't exist on struct
- **E018 - Match not exhaustive**: Match doesn't handle all cases

### Warnings (W001-W005)

- **W001 - Unused variable**: Variable declared but never used
- **W002 - Unreachable code**: Code after return statement
- **W003 - Async not awaited**: Async function call missing await
- **W004 - Dead code**: Code that's never executed
- **W005 - Deprecated API**: Using deprecated function or method

### "Did You Mean?" Suggestions

The compiler uses fuzzy matching (Levenshtein distance) to suggest corrections:

```raven
use raven_store::{Signa};
```

```
error: cannot find `Signa` in module `raven_store`
  --> app.jnc:1:20
   1 | use raven_store::{Signa};
     |                    ^^^^^
  help: did you mean `Signal`?
  [E008]
```

## Inlay Hints ✅

Inlay hints display inline type annotations and parameter names without modifying your source code, similar to Rust Analyzer. These subtle hints help you understand code at a glance.

### Type Hints

Show inferred types for variables without explicit type annotations:

**What you write**:
```raven
let count = 42;
let name = "Alice";
let active = true;
let price = 9.99;
```

**What you see** (with inlay hints):
```raven
let count: i32 = 42;
let name: String = "Alice";
let active: bool = true;
let price: f64 = 9.99;
```

**Supported Types**:
- **Integers**: `i32` for whole numbers
- **Floats**: `f64` for decimal numbers
- **Strings**: `String` for text literals
- **Booleans**: `bool` for true/false
- **Characters**: `char` for single characters
- **Collections**: `Vec`, `Array` for collections

**Note**: Type hints only appear for variables WITHOUT explicit type annotations. If you write `let count: i32 = 42;`, no hint will be shown.

### Parameter Hints

Show parameter names in function calls:

**What you write**:
```raven
calculate(10, 20, 5);
render(elem, true);
```

**What you see** (with inlay hints):
```raven
calculate(x: 10, y: 20, z: 5);
render(element: elem, visible: true);
```

This is especially helpful for functions with many parameters or non-obvious parameter names.

### Configuration (Future)

Future versions may include configuration options:
- Enable/disable type hints
- Enable/disable parameter hints
- Max hint length
- Hint appearance customization

### LSP Request

**Method**: `textDocument/inlayHint`

**Request**:
```json
{
  "textDocument": { "uri": "file:///path/to/file.jnc" },
  "range": {
    "start": { "line": 0, "character": 0 },
    "end": { "line": 100, "character": 0 }
  }
}
```

**Response**:
```json
[
  {
    "position": { "line": 1, "character": 10 },
    "label": ": i32",
    "kind": 1,
    "tooltip": "Inferred type: i32",
    "paddingLeft": false,
    "paddingRight": true
  }
]
```

**Hint Kinds**:
- **1 (Type)**: Type annotation hints
- **2 (Parameter)**: Parameter name hints

## Troubleshooting

### Completions Not Working

1. **Check document is open**: LSP requires document to be opened first
2. **Verify file extension**: Only `.jnc` files are processed
3. **Check LSP logs**: Look for connection or parsing errors

### Slow Completions

1. **Check file size**: Very large files may slow down parsing
2. **Verify syntax**: Syntax errors can slow down analysis
3. **Restart LSP**: Sometimes a restart helps clear caches

### Incorrect Completions

1. **Update to latest version**: Bug fixes are released regularly
2. **Report issues**: File a bug report with reproduction steps
3. **Check context**: Ensure cursor is in expected position

## Contributing

Want to improve the LSP? Check out:
- `src/lsp/mod.rs` - Main LSP implementation
- `src/type_checker.rs` - Type information
- `src/semantic_analyzer.rs` - Semantic analysis

**Adding New Completions**:
1. Add to appropriate completion method (e.g., `get_namespace_completions`)
2. Add documentation to `StdlibDocs`
3. Add test in `lsp::tests`
4. Run `cargo test --lib lsp`

---

**Last Updated**: 2025-10-22 (Phase 2 - Sprint 9)
**LSP Version**: 0.1.0
**Completions**: 70+
**Context Types**: 7
**Hover Information**: ✅ Full support for functions, variables, structs, enums, components
**Signature Help**: ✅ Real-time parameter hints with active parameter tracking
**Inlay Hints**: ✅ Type hints (i32, f64, String, bool, etc.) + Parameter hints
**Navigation Features**: 4 (Go to Definition, Find References, Rename Symbol, Document Symbols)
**Code Actions**: 6 quick fixes
**Error Codes**: 18 errors (E001-E018) + 5 warnings (W001-W005)
**Test Coverage**: 60 LSP tests (100% passing) - 10 hover + 6 signature help + 8 inlay hints + 18 diagnostic + 18 other
