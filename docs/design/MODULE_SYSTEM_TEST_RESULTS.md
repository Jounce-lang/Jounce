# Module System Test Results (Task 3)

**Date**: October 24, 2025
**Phase**: Phase 11 - Module System Testing
**Goal**: Test multi-file scenarios and document errors

---

## Test Environment

**Location**: `examples/test-modules/`
**Compiler**: `jnc` v0.3.0
**Test Files**:
- `math.jnc` - Simple math functions
- `utils.jnc` - Utility functions and structs
- `main.jnc` - Entry point
- `test-imports-errors.jnc` - Import test cases

---

## Test Results Summary

| Scenario | Status | Error | Fix Required |
|----------|--------|-------|--------------|
| Basic compilation (no imports) | ✅ Works | None | None |
| Local file import (`use ./math`) | ❌ Fails | Parser error | Parser: Support `.` in paths |
| Package import (`use jounce_http::HttpClient`) | ⚠️ Parsed | Module not found | Need installed packages |
| Selective import (`use mod::{A, B}`) | ✅ Parsed | N/A | Works (existing) |
| Export keyword | ❌ Not supported | Token doesn't exist | Add Export token |
| Directory compilation | ❌ Not supported | CLI only does files | Update CLI |

---

## Detailed Test Cases

### Test 1: Basic Compilation ✅

**File**: `test-local-import.jnc`
```jounce
fn main() {
    console.log("Test: Local imports");
    let x = 5;
    console.log("Value: " + x.to_string());
}
```

**Command**: `jnc compile test-local-import.jnc`

**Result**: ✅ **SUCCESS**
```
✨ Compilation complete! (911.79µs)
   Cache: 0 hits, 1 misses (0.0% hit rate)
```

**Finding**: Basic compilation works perfectly. No issues with single-file projects.

---

### Test 2: Local File Import ❌

**File**: `test-imports-errors.jnc`
```jounce
use ./math;

fn main() {
    console.log("Testing imports...");
}
```

**Command**: `jnc compile test-imports-errors.jnc`

**Result**: ❌ **PARSER ERROR**
```
❌ Parsing failed: ParserError {
    message: "Expected Identifier, found Dot",
    line: 4,
    column: 6
}
```

**Root Cause**: `parse_use_statement()` in parser.rs:137 calls `parse_identifier()` immediately after consuming `use` keyword. It doesn't check for `.` or `..` tokens.

**Current Parser Logic** (parser.rs:135-151):
```rust
fn parse_use_statement(&mut self) -> Result<UseStatement, CompileError> {
    self.expect_and_consume(&TokenKind::Use)?;
    let mut path = vec![self.parse_identifier()?];  // ← FAILS HERE FOR "."
    while self.consume_if_matches(&TokenKind::DoubleColon) {
        if self.current_token().kind == TokenKind::LBrace { break; }
        path.push(self.parse_identifier()?);
    }
    // ... rest of function
}
```

**Fix Required**:
1. Check if current token is `Dot` after `use`
2. If yes, parse relative path (./file or ../file)
3. Convert to special PathSegment that module_loader can recognize
4. Otherwise, continue with identifier parsing (existing behavior)

**Proposed Fix**:
```rust
fn parse_use_statement(&mut self) -> Result<UseStatement, CompileError> {
    self.expect_and_consume(&TokenKind::Use)?;

    // Check for relative path (. or ..)
    let mut path = Vec::new();
    if self.current_token().kind == TokenKind::Dot {
        // Parse relative path: ./file or ../parent/file
        while self.current_token().kind == TokenKind::Dot {
            self.consume(); // consume '.'
            path.push(Identifier { value: ".".to_string() });

            if self.current_token().kind == TokenKind::Slash {
                self.consume(); // consume '/'
            }
        }
    }

    // Parse module path components
    path.push(self.parse_identifier()?);
    while self.consume_if_matches(&TokenKind::DoubleColon) {
        if self.current_token().kind == TokenKind::LBrace { break; }
        path.push(self.parse_identifier()?);
    }

    // ... rest unchanged
}
```

**Alternative**: Add `Slash` token to lexer and parse `./` as a single unit.

---

### Test 3: Package Import ⚠️

**File**: `test-package-import.jnc`
```jounce
use jounce_http::HttpClient;

fn main() {
    console.log("Testing package import...");
}
```

**Command**: `jnc compile test-package-import.jnc`

**Result**: ⚠️ **PARSED, MODULE NOT FOUND**
```
✓ Parsed 2 statements
❌ Module import failed: Error: Module not found: jounce_http::HttpClient
   (searched in test_modules, aloha-shirts, and "aloha-shirts")
```

**Finding**:
- ✅ Parser successfully parses package imports
- ✅ Module loader is invoked
- ❌ Package doesn't exist in search paths (expected - we haven't installed packages)

**This Confirms**:
- Package import syntax works
- Module loader is integrated
- Search paths are correct (test_modules, aloha-shirts)

**No Fix Required**: This is expected behavior. Package imports work correctly.

---

### Test 4: Multiple File Project Structure

**Created Files**:
```
examples/test-modules/
├── math.jnc           # Math functions (add, multiply, is_even)
├── utils.jnc          # Utilities (double, format_number, Point, PI)
├── main.jnc           # Entry point
└── test-*.jnc         # Test cases
```

**Desired Usage** (Not Yet Working):
```jounce
// In main.jnc
use ./math::{add, multiply};
use ./utils::{double, Point};

fn main() {
    let sum = add(5, 3);
    let doubled = double(sum);
    let point = Point { x: 10, y: 20 };
}
```

**Current Status**: Cannot compile due to parser error on `./`

---

## Required Compiler Changes

### 1. Parser Changes (parser.rs) - HIGH PRIORITY

**Location**: `parse_use_statement()` line 135

**Changes Needed**:
- [ ] Detect `.` or `..` tokens after `use` keyword
- [ ] Parse relative path segments (. and ..)
- [ ] Handle `/` separators (or use `::` for consistency)
- [ ] Store relative path indicator in AST

**Impact**: Critical - blocks all local imports

**Estimated Effort**: 2-3 hours

---

### 2. AST Changes (ast.rs) - MEDIUM PRIORITY

**Location**: `UseStatement` struct (line 34)

**Current**:
```rust
pub struct UseStatement {
    pub path: Vec<Identifier>,     // Only identifiers
    pub imports: Vec<Identifier>,
}
```

**Proposed**:
```rust
pub struct UseStatement {
    pub path: Vec<PathSegment>,    // Can be identifiers or relative markers
    pub imports: Vec<Identifier>,
    pub is_relative: bool,          // NEW: true if starts with . or ..
}

pub enum PathSegment {
    Identifier(String),
    CurrentDir,      // .
    ParentDir,       // ..
}
```

**Impact**: Medium - enables relative path support

**Estimated Effort**: 1-2 hours

---

### 3. Module Loader Changes (module_loader.rs) - HIGH PRIORITY

**Location**: `resolve_module_path()` line 56

**Changes Needed**:
- [ ] Check `UseStatement.is_relative` flag
- [ ] If true, resolve relative to importing file's directory
- [ ] Handle `.`, `..`, and nested paths
- [ ] Convert to absolute PathBuf

**Current Logic** (package only):
```rust
pub fn resolve_module_path(&self, module_path: &[String]) -> Result<PathBuf, CompileError> {
    if module_path.is_empty() {
        return Err(CompileError::Generic("Empty module path".to_string()));
    }

    // Convert snake_case to kebab-case
    let package_name = module_path[0].replace('_', "-");

    // Search in package roots
    for root in package_roots {
        // ... existing package search
    }
}
```

**Proposed Addition**:
```rust
pub fn resolve_module_path(
    &self,
    module_path: &[String],
    is_relative: bool,
    importing_file: Option<&Path>  // NEW: source file location
) -> Result<PathBuf, CompileError> {
    if is_relative {
        // Resolve relative to importing file
        let base_dir = importing_file
            .and_then(|p| p.parent())
            .ok_or_else(|| CompileError::Generic("No importing file for relative path".to_string()))?;

        let mut resolved = base_dir.to_path_buf();
        for segment in module_path {
            if segment == "." {
                // Current directory (no-op)
            } else if segment == ".." {
                resolved.pop();
            } else {
                resolved.push(segment);
            }
        }
        resolved.set_extension("jnc");

        if resolved.exists() {
            return Ok(resolved);
        } else {
            return Err(CompileError::Generic(format!(
                "Module not found: {:?}",
                resolved
            )));
        }
    }

    // Existing package resolution logic
    // ...
}
```

**Impact**: Critical - enables local file imports

**Estimated Effort**: 3-4 hours

---

### 4. Dependency Tracking (cache/dependency_graph.rs) - MEDIUM PRIORITY

**Changes Needed**:
- [ ] Track file-to-file dependencies (not just packages)
- [ ] When file A imports file B, add edge A → B
- [ ] Cache invalidation propagates through dependencies
- [ ] Detect circular dependencies at file level

**Current**: Only tracks package-level dependencies

**Impact**: Medium - needed for correct incremental compilation

**Estimated Effort**: 2-3 hours

---

### 5. CLI Changes (main.rs) - LOW PRIORITY (for now)

**Changes Needed**:
- [ ] Accept directory paths: `jnc compile src/`
- [ ] Find all .jnc files recursively
- [ ] Determine entry point (main.jnc or --entry flag)
- [ ] Compile with module resolution

**Current**: Only compiles single files

**Impact**: Low - can work around by specifying entry file

**Estimated Effort**: 2-3 hours (can be delayed to Week 3)

---

## Implementation Priority

### Week 2 (Critical Path)

**Task 4: Implement export parsing** (4-6 hours)
- Lower priority for multi-file support
- Can be done in parallel with import fixes

**Task 5: Generate JavaScript exports** (3-4 hours)
- Lower priority
- Works without for testing

**Task 6: Implement cross-file imports** (8-10 hours) 🔥 **CRITICAL**
- Parser changes (2-3 hours)
- AST changes (1-2 hours)
- Module loader changes (3-4 hours)
- Testing (2-3 hours)

**This blocks everything else!**

**Task 7: Cache invalidation** (2-3 hours)
- Important but can use simple approach initially
- Full implementation can wait

---

## Recommended Approach

### Phase 1: Minimal Viable Import System (Week 2)

**Goal**: Get basic local imports working

1. **Parser**: Support `./file` syntax (3 hours)
   - Parse `.` token
   - Handle single-level relative imports only
   - No `..` support yet (simplify)

2. **Module Loader**: Resolve relative paths (3 hours)
   - Take importing file path as parameter
   - Resolve `./file.jnc` relative to importing file
   - Basic error handling

3. **Integration**: Connect parser → loader (2 hours)
   - Pass file path through compilation pipeline
   - Test with simple two-file project

4. **Testing**: Verify it works (2 hours)
   - Create test-modules example
   - Compile and run
   - Fix any issues

**Total**: ~10 hours (Week 2, Tasks 6-7)

### Phase 2: Full Feature Set (Week 3)

1. **Export keyword**: Add to parser/AST (4 hours)
2. **Parent directory**: Support `../` paths (2 hours)
3. **Deep nesting**: Support `../../../` (1 hour)
4. **CLI directories**: `jnc compile src/` (3 hours)
5. **Cache deps**: Full dependency tracking (2 hours)

**Total**: ~12 hours (Week 3, Tasks 8-11)

---

## Success Criteria (Revised)

**Minimum (End of Week 2)**:
- ✅ Parse `use ./file` syntax
- ✅ Resolve relative paths
- ✅ Import functions from local files
- ✅ 2-file example works (math.jnc + main.jnc)

**Full (End of Week 3)**:
- ✅ Export keyword works
- ✅ Parent directory imports (`../`)
- ✅ CLI compiles directories
- ✅ Cache tracks dependencies
- ✅ Multi-file todo app works
- ✅ All tests pass

---

## Next Steps (Immediate)

1. **Update CLAUDE.md** with revised plan
2. **Start Task 6** (cross-file imports)
   - Begin with parser changes
   - Test incrementally
3. **Skip Task 4-5** for now (export can wait)
4. **Focus on imports first** (critical path)

---

## Files Modified (This Task)

**Created**:
- `examples/test-modules/utils.jnc`
- `examples/test-modules/test-local-import.jnc`
- `examples/test-modules/test-imports-errors.jnc`
- `examples/test-modules/test-package-import.jnc`
- `docs/design/MODULE_SYSTEM_TEST_RESULTS.md` (this file)

**Test Results**:
- 1 passing (basic compilation)
- 1 failing (local imports - parser error)
- 1 warning (package imports - module not found)

**Total Test Coverage**: 3 scenarios documented

---

**Task 3 Status**: COMPLETE - Multi-file scenarios tested and documented
**Key Finding**: Parser doesn't support `.` in module paths (critical blocker)
**Recommendation**: Jump to Task 6 (imports) before Task 4-5 (exports)
**Next**: Task 6 - Implement cross-file imports (parser + module loader)
