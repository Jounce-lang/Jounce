# Jounce Module System Documentation

**Status**: Phase 11 - Module System Implementation (5/11 tasks complete)
**Created**: October 24, 2025
**Last Updated**: October 24, 2025

---

## Overview

Jounce has a module system for organizing code across multiple files and importing packages. This document describes the current implementation, what works, what's missing, and the plan for multi-file support.

**Latest Update**: Tasks 6 and 9 completed! Local file imports with `./` and `../` now work. Multi-file todo app example demonstrates full functionality.

---

## Current Implementation

###

 What Works ✅

#### 1. Package Imports

**Syntax**: `use package_name::SymbolName;`

```jounce
// Import specific symbol from a package
use jounce_http::HttpClient;

// Import multiple symbols
use jounce_router::{Router, Route, Link};

// Wildcard import (all exports)
use jounce_store;
```

**How It Works**:
- `ModuleLoader` (src/module_loader.rs) resolves package names to file paths
- Package names use snake_case (`jounce_http`), directories use kebab-case (`jounce-http`)
- Searches in: `test_modules/`, `aloha-shirts/`, and custom package root
- Default entry point: `package-name/src/lib.jnc`
- All top-level functions, structs, enums, and consts are automatically exported

**File Resolution**:
```
use jounce_router           → aloha-shirts/jounce-router/src/lib.jnc
use jounce_store::store     → aloha-shirts/jounce-store/src/store/store.jnc
use jounce_http::client     → aloha-shirts/jounce-http/src/client/client.jnc
```

#### 2. Local File Imports (NEW! ✅)

**Syntax**: `use ./path` or `use ../path`

```jounce
// Import from current directory
use ./math;              → ./math.jnc

// Import from parent directory
use ../utils;            → ../utils.jnc

// Import from nested directories
use ./models/user;       → ./models/user.jnc
use ../lib/helpers;      → ../lib/helpers.jnc
```

**How It Works**:
- Parser (src/parser.rs:135) detects `.` and `..` tokens
- Parses relative path segments (`.`, `..`, identifiers)
- ModuleLoader (src/module_loader.rs:58) resolves relative to current working directory
- All top-level functions, structs, enums are automatically imported

**Implementation** (Task 6 Complete):
- Parser updated to handle Dot and DotDot tokens
- Module loader checks for relative paths (starts with `.` or `..`)
- Resolves paths using PathBuf navigation
- Falls back to package resolution for non-relative paths

**Example**:
```jounce
// math.jnc
fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

// main.jnc
use ./math;

fn main() {
    let result = add(5, 3);
    console.log(result.to_string());
}
```

**Nested Imports Work**:
```jounce
// types.jnc
struct Todo { id: i64, title: String, completed: bool }

// storage.jnc
use ./types;  // Import types

fn get_sample() -> Todo {
    return Todo { id: 1, title: "Sample", completed: false };
}

// main.jnc
use ./storage;  // Imports storage, which imports types

fn main() {
    let todo = get_sample();
    console.log(todo.title);
}
```

#### 3. UseStatement AST Node

**Definition** (src/ast.rs:34):
```rust
pub struct UseStatement {
    pub path: Vec<Identifier>,     // e.g., ["jounce_http", "HttpClient"]
    pub imports: Vec<Identifier>,  // Empty = wildcard, non-empty = selective
}
```

#### 3. Parser Support

**Function**: `parse_use_statement()` (src/parser.rs:135)

**Supported Syntax**:
```jounce
use module_name;                    // Wildcard import
use module_name::Symbol;            // Single symbol
use module_name::{A, B, C};         // Multiple symbols
use package::submodule::Symbol;     // Nested modules
```

**Parsing Logic**:
1. Expects `use` keyword
2. Parses module path (identifiers separated by `::`)
3. If `{` found, parses selective imports (comma-separated)
4. Otherwise, treats as wildcard import (empty imports vec)

#### 4. Module Loading

**Class**: `ModuleLoader` (src/module_loader.rs)

**Key Features**:
- **Caching**: Modules are parsed once and cached
- **Cycle Detection**: Detects circular dependencies
- **Export Extraction**: Automatically extracts all top-level definitions
- **Import Merging**: Injects imported definitions into program AST

**API**:
```rust
let mut loader = ModuleLoader::new("aloha-shirts");

// Load a module
let module = loader.load_module(&["jounce_http"])?;

// Get specific export
let export = loader.get_export(&["jounce_http"], "HttpClient")?;

// Merge imports into program
loader.merge_imports(&mut program)?;
```

#### 5. Automatic Exports

**Current Behavior**: All top-level items are exported automatically.

**Extracted Symbols** (src/module_loader.rs:156):
- Functions (`fn`)
- Structs (`struct`)
- Enums (`enum`)
- Constants (`const`)

**Not Yet Supported**:
- Type aliases (`type`)
- Explicit export control (`pub`, `export`)

---

## What's Missing ❌

### 1. Export Keyword

**Problem**: All top-level items are exported. No way to make items private.

**Desired Syntax**:
```jounce
// Exported (public)
export fn public_api() {
    private_helper();
}

// Private (not exported)
fn private_helper() {
    // Only visible within this module
}
```

**What Needs to Change**:
- Add `export` keyword to lexer (token.rs)
- Parse `export` modifier in parser (parser.rs)
- Track export status in AST nodes (ast.rs)
- `extract_exports()` only exports marked items (module_loader.rs)
- JavaScript emitter generates `export` statements (js_emitter.rs)

### 2. Directory Compilation

**Problem**: CLI only compiles single files, not directories.

**Desired Commands**:
```bash
jnc compile src/              # Compile all .jnc files
jnc compile .                 # Compile current directory
jnc compile src/ --entry main.jnc  # Specify entry point
```

**What Needs to Change**:
- `main.rs` CLI must detect directory paths
- Find all .jnc files in directory
- Determine entry point (main.jnc or --entry flag)
- Compile with module resolution

### 3. Cross-File Dependency Tracking

**Problem**: Cache doesn't track file dependencies.

**Issue**: Changing an imported file doesn't trigger recompilation of files that import it.

**Example**:
```
math.jnc (defines add function)
main.jnc (imports math)

Change math.jnc → main.jnc NOT recompiled (cache hit)
```

**What Needs to Change**:
- `DependencyGraph` must track file-level dependencies
- Cache invalidation triggers when any dependency changes
- Watch mode must monitor imported files

### 4. Module Re-exports

**Problem**: Cannot re-export symbols from imported modules.

**Desired Syntax**:
```jounce
// In lib.jnc
export use ./router::Router;
export use ./types::{Route, Link};

// External usage
use my_lib::Router;  // Re-exported from internal module
```

**What Needs to Change**:
- Parse `export use` statements
- Track re-exports separately
- Include re-exported symbols in module exports

---

## Implementation Plan

### Week 1: Document & Design ✅ In Progress

- [x] Document current module behavior → THIS FILE
- [ ] Design export keyword syntax
- [ ] Test multi-file scenarios
- [ ] List all compiler changes needed

### Week 2: Core Implementation

**Task 4: Implement export parsing**
- Add `Export` token to lexer
- Parse `export` modifier (export fn, export struct, etc.)
- Update AST nodes with `exported: bool` flag
- Write parser tests

**Task 5: Generate JavaScript exports**
- Update js_emitter.rs to check `exported` flag
- Generate `export function` for exported functions
- Generate `export class` for exported structs
- Test compiled JavaScript

**Task 6: Implement cross-file imports**
- Detect relative paths (`.`, `..`) in module_path
- Resolve file paths relative to importing file
- Load and parse local .jnc files
- Build dependency graph between files

**Task 7: Cache invalidation**
- Track file dependencies in DependencyGraph
- Invalidate cache when dependencies change
- Add tests for cache invalidation

### Week 3: CLI & Examples

**Task 8: Update CLI for directories**
- Detect directory paths in `jnc compile`
- Find all .jnc files recursively
- Auto-detect entry point or use --entry flag
- Compile with full module resolution

**Task 9: Build multi-file example**
- Create `examples/todo-app-multi-file/`
- Split into: main.jnc, components.jnc, api.jnc, utils.jnc
- Demonstrate import/export usage
- Document best practices

**Task 10: Write documentation**
- Update this file with final design
- Add section to GETTING_STARTED.md
- Code examples for all use cases
- Best practices guide

**Task 11: Testing**
- 20+ tests for import/export
- Test circular dependencies (should error)
- Test deep nesting (a → b → c)
- Test that package imports still work

---

## Module Resolution Rules

### Package Imports

```jounce
use jounce_http::HttpClient;
```

**Resolution Steps**:
1. Convert snake_case to kebab-case: `jounce_http` → `jounce-http`
2. Search locations (in order):
   - `test_modules/jounce-http/src/lib.jnc`
   - `aloha-shirts/jounce-http/src/lib.jnc`
   - `<package_root>/jounce-http/src/lib.jnc`
3. If path has submodules: `jounce_store::store`
   - Search: `aloha-shirts/jounce-store/src/store/store.jnc`
4. Parse file and extract exports
5. Import specified symbols (or all if wildcard)

### Local File Imports (TO BE IMPLEMENTED)

```jounce
use ./math;
use ./utils/helpers;
use ../common/types;
```

**Resolution Steps**:
1. Detect relative path (starts with `.` or `..`)
2. Resolve relative to importing file's directory:
   - `./math` → same directory, `math.jnc`
   - `./utils/helpers` → `utils/helpers.jnc`
   - `../common/types` → parent directory, `common/types.jnc`
3. Check if file exists
4. Parse file and extract exports
5. Import specified symbols

---

## Export Control Specification (Task 2 Design)

### Design Principles

1. **Explicit Over Implicit**: Items must be explicitly marked `export` to be importable
2. **JavaScript Compatibility**: Maps cleanly to ES6 `export` statements
3. **Simplicity**: One keyword (`export`), clear semantics
4. **Consistency**: Works the same for all item types

### Syntax Specification

#### Basic Export Syntax

```jounce
// Export a function
export fn function_name(params) -> ReturnType {
    // body
}

// Export a struct
export struct StructName {
    field1: Type1,
    field2: Type2,
}

// Export an enum
export enum EnumName {
    Variant1,
    Variant2(Type),
}

// Export a constant
export const CONSTANT_NAME: Type = value;

// Export a type alias (future)
export type AliasName = Type;
```

#### Private (Default) Behavior

```jounce
// Without 'export', items are private
fn internal_helper() {
    // NOT importable from other files
}

struct InternalData {
    // NOT importable
}

const INTERNAL_CONFIG: i64 = 42;  // NOT importable
```

#### Re-export Syntax (Future)

```jounce
// Re-export from another module
export use ./types::Point;
export use ./utils::{add, multiply};

// Re-export with rename
export use ./http::HttpClient as Client;
```

### Grammar Rules

#### Lexer Changes (token.rs)

Add new token:
```rust
pub enum TokenKind {
    // ... existing tokens ...
    Export,  // 'export' keyword
}
```

#### Parser Changes (parser.rs)

Update item parsing:
```rust
fn parse_statement(&mut self) -> Result<Statement, CompileError> {
    // Check for export modifier
    let exported = self.consume_if_matches(&TokenKind::Export);

    match self.current_token().kind {
        TokenKind::Fn => {
            let mut func = self.parse_function()?;
            func.exported = exported;  // Mark as exported
            Ok(Statement::Function(func))
        }
        TokenKind::Struct => {
            let mut struct_def = self.parse_struct()?;
            struct_def.exported = exported;
            Ok(Statement::Struct(struct_def))
        }
        // ... other cases ...
    }
}
```

#### AST Changes (ast.rs)

Add `exported` field to all exportable items:
```rust
pub struct FunctionDefinition {
    pub name: Identifier,
    pub exported: bool,  // NEW FIELD
    // ... other fields ...
}

pub struct StructDefinition {
    pub name: Identifier,
    pub exported: bool,  // NEW FIELD
    // ... other fields ...
}

pub struct EnumDefinition {
    pub name: Identifier,
    pub exported: bool,  // NEW FIELD
    // ... other fields ...
}

pub struct ConstDeclaration {
    pub name: Identifier,
    pub exported: bool,  // NEW FIELD
    // ... other fields ...
}
```

### Module Resolution Rules

#### Local File Import Resolution

```jounce
use ./file;              // Same directory
use ./subdir/file;       // Subdirectory
use ../parent/file;      // Parent directory
use ../../file;          // Two levels up
```

**Resolution Algorithm**:
1. Check if path starts with `.` or `..`
2. If yes, resolve relative to importing file's directory:
   ```
   Current file: /project/src/components/Button.jnc
   Import: use ./Header;
   Resolved: /project/src/components/Header.jnc

   Import: use ../utils/format;
   Resolved: /project/src/utils/format.jnc
   ```
3. If no, treat as package import (existing behavior)

#### Package Import Resolution (Existing)

```jounce
use jounce_http::HttpClient;
```

Resolves to: `aloha-shirts/jounce-http/src/lib.jnc`

### JavaScript Code Generation

#### Exported Function

**Jounce**:
```jounce
export fn add(a: i64, b: i64) -> i64 {
    return a + b;
}
```

**JavaScript**:
```javascript
export function add(a, b) {
    return a + b;
}
```

#### Private Function

**Jounce**:
```jounce
fn internal_helper() {
    console.log("Private");
}
```

**JavaScript**:
```javascript
function internal_helper() {
    console.log("Private");
}
// No 'export' keyword
```

#### Exported Struct

**Jounce**:
```jounce
export struct Point {
    x: f64,
    y: f64,
}
```

**JavaScript**:
```javascript
export class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}
```

#### Exported Enum

**Jounce**:
```jounce
export enum Status {
    Success,
    Error(String),
}
```

**JavaScript**:
```javascript
export const Status = {
    Success: { tag: "Success" },
    Error: (msg) => ({ tag: "Error", value: msg })
};
```

### Import/Export Examples

#### Example 1: Simple Module

**File**: `src/math.jnc`
```jounce
// Exported - can be imported
export fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

export fn multiply(a: i64, b: i64) -> i64 {
    return a * b;
}

// Private - cannot be imported
fn validate_input(n: i64) -> bool {
    return n >= 0;
}

export const MAX_VALUE: i64 = 1000;
```

**File**: `src/main.jnc`
```jounce
use ./math::{add, multiply, MAX_VALUE};

fn main() {
    let sum = add(5, 3);
    let product = multiply(4, 7);
    console.log("Sum: " + sum.to_string());
    console.log("Max: " + MAX_VALUE.to_string());

    // ERROR: validate_input is private
    // let valid = validate_input(10);  // Compile error
}
```

#### Example 2: Components Module

**File**: `src/components/Button.jnc`
```jounce
export struct ButtonProps {
    label: String,
    onclick: fn(),
}

export fn Button(props: ButtonProps) -> JSX {
    return <button onclick={props.onclick}>{props.label}</button>;
}

fn get_button_styles() -> String {
    return "btn btn-primary";
}
```

**File**: `src/components/Header.jnc`
```jounce
export fn Header(title: String) -> JSX {
    return <header><h1>{title}</h1></header>;
}
```

**File**: `src/components/mod.jnc` (Re-exports)
```jounce
// Re-export all components from single entry point
export use ./Button::{Button, ButtonProps};
export use ./Header::Header;
```

**File**: `src/main.jnc`
```jounce
// Import from mod.jnc (re-exports)
use ./components::{Button, Header, ButtonProps};

fn main() {
    let app = (
        <div>
            <Header title="My App" />
            <Button props={ButtonProps {
                label: "Click me!",
                onclick: || console.log("Clicked!")
            }} />
        </div>
    );
}
```

#### Example 3: Mixed Imports

**File**: `src/app.jnc`
```jounce
// Package imports
use jounce_http::HttpClient;
use jounce_router::{Router, Route};

// Local imports
use ./components::{Button, Header};
use ./api::fetch_users;
use ./utils/format::format_date;

export fn create_app() -> Router {
    let router = Router::new();
    router.add_route("/", home_page);
    router.add_route("/users", users_page);
    return router;
}

async fn users_page() -> JSX {
    let client = HttpClient::new("https://api.example.com");
    let users = fetch_users(client).await;
    return render_users(users);
}
```

### Edge Cases & Rules

#### 1. Exported Items Must Be Top-Level

**Valid**:
```jounce
export fn top_level() {
    // OK
}
```

**Invalid**:
```jounce
fn container() {
    export fn nested() {  // ERROR: export only allowed at top level
        // ...
    }
}
```

#### 2. Cannot Export Impl Blocks

**Invalid**:
```jounce
export impl Point {  // ERROR: cannot export impl
    fn distance(&self) -> f64 {
        // ...
    }
}
```

**Valid** (export the struct, not the impl):
```jounce
export struct Point {
    x: f64,
    y: f64,
}

impl Point {  // Impl is always tied to the struct
    fn distance(&self) -> f64 {
        // Methods are accessible if struct is imported
    }
}
```

#### 3. Trait Implementations Are Always Exported

If a struct is exported and implements a trait, the implementation is automatically available:

```jounce
export struct Point {
    x: f64,
    y: f64,
}

// Automatically exported with Point
impl Display for Point {
    fn to_string(&self) -> String {
        return "(" + self.x.to_string() + ", " + self.y.to_string() + ")";
    }
}
```

#### 4. Main Function Cannot Be Exported

```jounce
fn main() {  // OK (entry point)
    // ...
}

export fn main() {  // ERROR: main cannot be exported
    // ...
}
```

#### 5. Server/Client Annotations Work With Export

```jounce
@server
export fn get_users() -> Vec<User> {
    // Server-side function, exported for RPC
}

@client
export fn render_ui() -> JSX {
    // Client-side function
}
```

### Backward Compatibility

**Migration Strategy**:

**Phase 1** (Current - v0.3.0):
- All top-level items automatically exported
- No breaking changes

**Phase 2** (v0.3.1 - Transition):
- Add `export` keyword support
- Emit warnings for items without explicit export
- Still export everything for compatibility

**Phase 3** (v0.4.0 - Breaking):
- Require explicit `export` keyword
- Private by default
- Breaking change for existing code

**Migration Tool**:
```bash
jnc migrate --add-exports src/
# Automatically adds 'export' to all top-level items
```

### Error Messages

**Importing Private Item**:
```
Error: Cannot import 'internal_helper' from './math'
  --> src/main.jnc:3:5
   |
 3 | use ./math::internal_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ item is not exported
   |
   = note: add 'export' keyword to make it importable:
           export fn internal_helper() { ... }
```

**Export In Wrong Location**:
```
Error: 'export' keyword can only be used at module top level
  --> src/utils.jnc:5:9
   |
 5 |         export fn nested() {
   |         ^^^^^^ export not allowed here
   |
   = note: move this function to top level
```

**Circular Dependency**:
```
Error: Circular module dependency detected
  --> src/a.jnc:1:1
   |
 1 | use ./b;
   | ^^^^^^^^
   |
   = note: dependency chain: a.jnc → b.jnc → c.jnc → a.jnc
```

### Implementation Checklist

- [ ] Add `Export` token to lexer (token.rs)
- [ ] Add `exported: bool` field to AST nodes (ast.rs)
- [ ] Parse `export` modifier in parser (parser.rs)
- [ ] Update `extract_exports()` to check exported flag (module_loader.rs)
- [ ] Generate `export` statements in JS emitter (js_emitter.rs)
- [ ] Add error for invalid export locations
- [ ] Add error for importing private items
- [ ] Write 15+ tests for export behavior
- [ ] Update documentation with examples
- [ ] Create migration tool (optional)

### Testing Plan

**Unit Tests** (parser.rs):
```rust
#[test]
fn test_parse_exported_function() {
    // export fn add() -> i64
}

#[test]
fn test_parse_private_function() {
    // fn helper() (no export)
}

#[test]
fn test_export_only_at_top_level() {
    // Should error for nested export
}
```

**Integration Tests** (test_exports.jnc):
```jounce
// Test file for export behavior

export fn public_api() {
    return "accessible";
}

fn private_helper() {
    return "hidden";
}

// Try to import
use ./test_exports::public_api;  // Should work
use ./test_exports::private_helper;  // Should fail
```

---

**Task 2 Status**: COMPLETE - Export keyword fully designed
**Next**: Task 3 - Test multi-file scenarios
**Lines Added**: 400+ lines of specification

---

## Examples

### Example 1: Package Import (Works Today)

**File**: `examples/http-client.jnc`
```jounce
use jounce_http::HttpClient;

async fn fetch_data() {
    let client = HttpClient::new("https://api.example.com");
    let response = client.get("/users").send().await;
    console.log(response.unwrap());
}
```

**Compilation**:
```bash
jnc compile examples/http-client.jnc
# Successfully loads jounce-http package
```

### Example 2: Multi-File Project (To Be Implemented)

**File**: `src/math.jnc`
```jounce
export fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

export fn multiply(a: i64, b: i64) -> i64 {
    return a * b;
}

fn internal_log(msg: String) {
    console.log("[Math] " + msg);
}
```

**File**: `src/main.jnc`
```jounce
use ./math::{add, multiply};

fn main() {
    let sum = add(5, 3);
    let product = multiply(4, 7);
    console.log("Sum: " + sum.to_string());
    console.log("Product: " + product.to_string());
}
```

**Compilation** (after Phase 11):
```bash
jnc compile src/
# Compiles both files, resolves imports
```

### Example 3: Mixed Imports (To Be Implemented)

**File**: `src/app.jnc`
```jounce
// Package import
use jounce_router::Router;

// Local import
use ./components::{Button, Header};
use ./api::fetch_users;

fn main() {
    let router = Router::new();
    router.add_route("/", home_page);
    router.start();
}

fn home_page() -> JSX {
    return (
        <div>
            <Header title="My App" />
            <Button label="Click me!" />
        </div>
    );
}
```

---

## Known Limitations

### 1. No Namespace Control

**Issue**: Cannot control export namespace.

**Workaround**: Rename symbols on import (not yet supported).

**Future**:
```jounce
use jounce_http::{HttpClient as Client};  // Rename on import
```

### 2. No Conditional Imports

**Issue**: Cannot import based on conditions.

**Future**:
```jounce
#[cfg(feature = "json")]
use jounce_json::Parser;
```

### 3. No Glob Imports

**Issue**: Cannot import all files in a directory.

**Future**:
```jounce
use ./components/*;  // Import all modules in directory
```

---

## Testing Strategy

### Unit Tests

**Location**: `src/module_loader.rs:320`

**Existing Tests**:
- `test_module_path_resolution` - Package path resolution
- `test_snake_to_kebab_conversion` - Name conversion

**Needed Tests**:
- Relative path resolution (`.`, `..`)
- Circular dependency detection
- Export filtering (after export keyword added)
- Cache invalidation with dependencies
- Wildcard vs selective imports
- Error messages for missing modules

### Integration Tests

**To Be Created**: `tests/test_modules.jnc`

**Test Cases**:
- Import from local file
- Import from package
- Mixed imports (local + package)
- Circular dependencies (should error)
- Deep nesting (a → b → c → d)
- Re-exports
- Private vs exported items

---

## Success Criteria

Phase 11 is complete when:

- ✅ Multi-file todo app compiles and runs
- ✅ `export` keyword works correctly
- ✅ Local imports work (`use ./file`)
- ✅ CLI can compile directories (`jnc compile src/`)
- ✅ Cache invalidates when dependencies change
- ✅ 20+ tests for import/export passing
- ✅ All 638 existing tests still passing
- ✅ Documentation complete and accurate

---

## References

- **module_loader.rs**: Module resolution and loading (344 lines)
- **parser.rs:135**: UseStatement parsing
- **ast.rs:34**: UseStatement definition
- **main.rs**: CLI entry point (needs directory support)
- **cache/dependency_graph.rs**: Dependency tracking (needs file support)

---

**Status**: Task 1 Complete - Module system documented
**Next**: Task 2 - Design export keyword syntax
**Target**: v0.3.1 with multi-file support (2-3 weeks)
