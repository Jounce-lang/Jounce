# Jounce Module System Documentation

**Status**: Phase 11 - Module System Implementation
**Created**: October 24, 2025
**Last Updated**: October 24, 2025

---

## Overview

Jounce has a module system for organizing code across multiple files and importing packages. This document describes the current implementation, what works, what's missing, and the plan for multi-file support.

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

#### 2. UseStatement AST Node

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

### 1. Local File Imports

**Problem**: Cannot import from relative file paths.

**Desired Syntax**:
```jounce
use ./math;              // Import from ./math.jnc
use ./utils/helpers;     // Import from ./utils/helpers.jnc
use ../common/types;     // Import from parent directory
```

**Current Error**: Module loader only searches in package directories, not relative to current file.

**What Needs to Change**:
- `resolve_module_path()` must detect `.` or `..` prefix
- Resolve path relative to importing file's directory
- Load and parse the local .jnc file

### 2. Export Keyword

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

### 3. Directory Compilation

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

### 4. Cross-File Dependency Tracking

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

### 5. Module Re-exports

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

## Export Control (TO BE IMPLEMENTED)

### Current Behavior (Implicit Export)

```jounce
// In math.jnc
fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

// AUTOMATICALLY EXPORTED (can be imported)
```

### Future Behavior (Explicit Export)

```jounce
// In math.jnc

// Exported - can be imported by other files
export fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

// Private - only visible in this file
fn internal_helper() {
    // Cannot be imported
}

// Exported struct
export struct Point {
    x: f64,
    y: f64,
}

// Private enum
enum InternalState {
    Ready,
    Processing,
}
```

### JavaScript Output

```javascript
// Exported function
export function add(a, b) {
    return a + b;
}

// Private function (no export)
function internal_helper() {
    // Not exported
}

// Exported class
export class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}
```

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
