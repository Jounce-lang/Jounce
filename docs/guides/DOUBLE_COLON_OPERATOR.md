# Double Colon (`::`) Operator Support

The `::` operator provides namespace resolution in Jounce, allowing you to call methods on objects using Rust-style syntax.

## Syntax

```raven
namespace::method(args)
```

Examples:
```raven
console::log("Hello World")
document::getElementById("app")
Math::floor(3.14)
localStorage::getItem("key")
```

## How It Works

### Parser
The parser recognizes `::` as a namespace separator and builds a single identifier:
- `console::log` → `Identifier("console::log")`
- `document::write` → `Identifier("document::write")`

### JavaScript Emission
The JavaScript emitter converts `::` to `.` (dot notation):
```raven
console::log("test")  →  console.log("test")
```

### Type Checking
All namespaced identifiers are treated as external functions with `Type::Any`:
```raven
// Type checker accepts these as external APIs
console::log(x)        // ✅ Accepted
document::write(html)  // ✅ Accepted
```

## Current Support

### ✅ Supported Contexts

**1. `@server` Functions (Pure JavaScript)**
```raven
@server
fn init() {
    console::log("Server started");
    process::exit(0);
}
```

**2. `@client` Functions (Pure JavaScript)**
```raven
@client
fn render() {
    console::log("Rendering");
    document::getElementById("app");
}
```

**3. JavaScript Output**
All `::` operators in JavaScript-targeted code work perfectly.

### ⚠️ Current Limitations

**1. Shared Functions (WASM-targeted)**
```raven
// ❌ Not yet supported in shared functions
fn shared() {
    std::println("test");  // Error: requires FFI import
}
```

Shared functions compile to WASM, which requires FFI imports to be registered. This is planned for future implementation.

**2. WASM Compilation**
External APIs like `console::log` cannot be compiled to WASM without FFI import registration.

## Implementation Details

### Files Modified

1. **src/parser.rs** (lines 705-723)
   - Added `TokenKind::DoubleColon` case in postfix operations loop
   - Concatenates identifiers: `base::next` → `"base::next"`

2. **src/type_checker.rs** (lines 264-268)
   - Checks if identifier contains `::`
   - Returns `Type::Any` for namespaced identifiers

3. **src/codegen.rs** (lines 1042-1051)
   - Detects namespaced identifiers in WASM codegen
   - Provides clear error message for external functions

### Lexer Support
The lexer already tokenizes `::` as `TokenKind::DoubleColon` (src/lexer.rs:30-37).

## Common Use Cases

### Browser APIs
```raven
@client
fn setupApp() {
    let app = document::getElementById("app");
    let data = localStorage::getItem("userData");
    console::log("App initialized");
}
```

### Node.js APIs
```raven
@server
fn readConfig() {
    let fs = require("fs");
    console::log("Config loaded");
    process::env::NODE_ENV;
}
```

### Math Operations
```raven
@client
fn calculate() {
    let x = Math::floor(3.7);
    let y = Math::random();
    return x + y;
}
```

## Future Enhancements

### Planned Features

1. **FFI Import Registration**
   - Automatically register common browser/Node.js APIs
   - Allow WASM to call external functions via imports

2. **Standard Library Namespacing**
   - `std::println()`
   - `std::vec::Vec`
   - `std::collections::HashMap`

3. **Custom Namespace Support**
   - User-defined modules with `::` syntax
   - Import resolution

## Examples

### Full-Stack Todo App
```raven
@server
fn init() {
    console::log("Server initialized");
}

@client
fn render() {
    console::log("Rendering UI");
    let app = document::getElementById("app");
    document::write("<h1>Todo App</h1>");
}
```

### Compiles To
**client.js:**
```javascript
export function render() {
    console.log("Rendering UI");
    let app = document.getElementById("app");
    document.write("<h1>Todo App</h1>");
}
```

**server.js:**
```javascript
export function init() {
    console.log("Server initialized");
}
```

## Error Messages

### WASM Compilation Error
```
❌ WASM compilation failed: Error: External function call 'console::log' - requires FFI import registration
```

This error occurs when trying to use `::` in shared functions that compile to WASM.

**Solution:** Use `::` only in `@server` or `@client` functions (JavaScript-targeted).

## Best Practices

### ✅ Do
- Use `::` in `@server` and `@client` functions
- Call browser/Node.js APIs with `::` syntax
- Use for external library methods

### ❌ Don't
- Use `::` in shared functions (WASM-targeted) yet
- Expect WASM compilation for external APIs
- Mix `.` and `::` for the same API

## Testing

### Test Files
- `examples/test_double_colon.jnc` - Full example with annotations
- `examples/test_simple_doublecolon.jnc` - Simple parsing test

### Run Tests
```bash
./target/release/raven compile examples/test_double_colon.jnc
```

## Technical Notes

### Parser Implementation
The `::` operator is handled in the postfix operations loop alongside:
- Function calls `()`
- Field access `.`
- Array indexing `[]`
- Try operator `?`

### Precedence
`::` has the same precedence as `.` (field access) - highest precedence.

```raven
console::log("test")()  // ✅ Parses as (console::log)("test")
obj.field::method()     // ✅ Parses as (obj.field)::method()
```

## Migration Guide

### From `.` to `::`

**Before:**
```raven
console.log("test")
document.getElementById("app")
```

**After:**
```raven
console::log("test")
document::getElementById("app")
```

Both syntaxes work, but `::` is preferred for consistency with Rust.

---

**Status:** ✅ Implemented (JavaScript targets only)
**Version:** 0.1.0
**Last Updated:** October 2025
