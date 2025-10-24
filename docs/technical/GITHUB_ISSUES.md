# GitHub Issues Tracker - Jounce TODOs

**Generated**: 2025-10-20 (Day 2)
**Source**: Codebase analysis from DEVELOPMENT_PLAN_3_PHASES.md
**Status**: Ready for GitHub issue creation

---

## 🔥 P0 - Critical (Must Fix for v1.0)

### Issue #1: Implement JSX Support
**Priority**: P0
**Labels**: feature, critical, compiler
**Milestone**: Phase 1 - Foundation

**Description**:
JSX/Component syntax is documented but not implemented. All component-based examples fail to compile with "ParserError: No prefix parse function for Slash".

**Affected Files**:
- src/lexer.rs - Need JSX token types
- src/parser.rs - Need JSX parsing functions
- src/ast.rs - Need JSX AST nodes
- src/codegen.rs - Need JSX code generation
- dist/client-runtime.js - Need createElement helpers

**Affected Examples** (15+ files):
- examples/todo_app.jnc
- examples/analytics_dashboard.jnc
- examples/devboard/src/main.jnc
- All other component-based examples

**Acceptance Criteria**:
- [ ] Lexer tokenizes JSX: `<div>`, `</div>`, `<img />`
- [ ] Parser constructs JSX AST nodes
- [ ] Codegen emits createElement() calls
- [ ] Simple component example compiles successfully
- [ ] All existing component examples compile

**Estimated Effort**: 2-3 weeks (per DEVELOPMENT_PLAN Phase 1.3-1.5)

**Related**:
- See PARSER_LIMITATIONS.md for implementation details
- See DEVELOPMENT_PLAN_3_PHASES.md Phase 1.3-1.5

---

### Issue #2: Implement WASM Function Tables for Closures
**Priority**: P0
**Labels**: feature, critical, wasm, closures
**Milestone**: Phase 2 - Features

**Description**:
Closures parse correctly but cannot be stored in variables or passed as arguments due to missing WASM function table implementation.

**Current Status**:
- ✅ Parser support complete
- ✅ Type system support complete
- ❌ WASM function tables not implemented
- ❌ Capture analysis not implemented

**Location**: src/codegen.rs:966, src/codegen.rs:1755

**Code Reference**:
```rust
// codegen.rs:966
// TODO: Full closure implementation should return tuple [func_index, env_ptr]
// and update lambda signatures to accept env_ptr as first parameter
```

**Test Case** (currently fails):
```raven
fn test() -> i32 {
    let add_ten = |x| x + 10;  // ❌ Cannot store in variable
    return add_ten(5);          // ❌ Cannot call variable
}
```

**Acceptance Criteria**:
- [ ] Generate WASM function table
- [ ] Add functions to table during codegen
- [ ] Implement call_indirect instruction
- [ ] Support function parameters and return values
- [ ] Test: Can call functions stored in variables
- [ ] Test: Can pass functions as arguments

**Estimated Effort**: 3-4 days (per CLOSURE_STATUS.md Phase 1)

**Related**:
- See CLOSURE_STATUS.md for implementation plan
- See DEVELOPMENT_PLAN_3_PHASES.md Phase 2.1

---

### Issue #3: Implement Closure Environment Capture
**Priority**: P0
**Labels**: feature, critical, closures
**Milestone**: Phase 2 - Features

**Description**:
Closures cannot capture variables from outer scope.

**Location**: src/codegen.rs:1755

**Code Reference**:
```rust
// codegen.rs:1755
captured_vars: Vec::new(),  // TODO: Implement capture analysis
```

**Test Case** (currently fails):
```raven
fn make_adder(n: i32) -> fn(i32) -> i32 {
    return |x| x + n;  // ❌ Cannot capture 'n'
}
```

**Acceptance Criteria**:
- [ ] Implement capture analysis
- [ ] Generate closure structs: [func_index, env_ptr]
- [ ] Allocate environment on heap
- [ ] Pass environment pointer to closure functions
- [ ] Support mutable captures
- [ ] Test: Closures can capture immutable variables
- [ ] Test: Closures can capture mutable variables (with RefCell)

**Estimated Effort**: 4-5 days (per CLOSURE_STATUS.md Phase 2)

**Related**:
- Depends on Issue #2 (function tables)
- See CLOSURE_STATUS.md for implementation plan

---

### Issue #4: Improve Forward Reference Handling
**Priority**: P1
**Labels**: bug, compiler
**Milestone**: Phase 1 - Foundation

**Description**:
Functions called before they're defined cause codegen to push dummy values.

**Location**: src/codegen.rs:848

**Code Reference**:
```rust
// codegen.rs:848
// TODO: Improve parsing/semantic analysis to catch these issues earlier
f.instruction(&Instruction::I32Const(0));  // Dummy value for forward ref
```

**Impact**: Can cause runtime errors if forward references aren't resolved

**Acceptance Criteria**:
- [ ] Implement two-pass semantic analysis
- [ ] First pass: Collect all function signatures
- [ ] Second pass: Validate all function calls
- [ ] Error on undefined functions at compile-time
- [ ] Test: Forward references work correctly
- [ ] Test: Undefined functions cause compile error

**Estimated Effort**: 2-3 days

**Related**:
- Also affects type_checker.rs:291

---

## ⚠️ P1 - High Priority (Should Fix for v1.0)

### Issue #5: Use Semantic Analyzer for Field Access
**Priority**: P1
**Labels**: improvement, type-system
**Milestone**: Phase 1 - Foundation

**Description**:
Field access currently infers struct types at codegen time instead of using semantic analyzer.

**Location**: src/codegen.rs:1079

**Code Reference**:
```rust
// codegen.rs:1079
// TODO: Use semantic analyzer type information for accurate field access
if let Ok(struct_name) = self.infer_struct_type(&field_access.object) {
```

**Impact**: Less accurate field offset calculations, potential runtime errors

**Acceptance Criteria**:
- [ ] Semantic analyzer tracks struct types
- [ ] Codegen uses semantic analyzer data
- [ ] Remove infer_struct_type() workaround
- [ ] Test: Field access works for all struct types

**Estimated Effort**: 2 days

---

### Issue #6: Track Function Signatures for call_indirect
**Priority**: P1
**Labels**: improvement, wasm
**Milestone**: Phase 2 - Features

**Description**:
Call_indirect uses placeholder type index instead of correct function signature.

**Location**: src/codegen.rs:1392

**Code Reference**:
```rust
// codegen.rs:1392
// TODO: Track function signatures to use the correct type index
let type_index = 0;  // Placeholder - should match function signature
```

**Impact**: May cause WASM validation errors with complex function types

**Acceptance Criteria**:
- [ ] Build function signature → type index mapping
- [ ] Use correct type index in call_indirect
- [ ] Test: call_indirect works with various function signatures

**Estimated Effort**: 1-2 days

**Related**:
- Needed for Issue #2 (function tables)

---

### Issue #7: Implement String/Array Method Support
**Priority**: P1
**Labels**: feature, stdlib
**Milestone**: Phase 2 - Features

**Description**:
String and array methods return placeholder values instead of actual implementations.

**Locations**:
- src/codegen.rs:1467
- src/codegen.rs:1473

**Code Reference**:
```rust
// codegen.rs:1467
// TODO: Implement actual string searching and array manipulation in WASM
f.instruction(&Instruction::I32Const(1));  // Placeholder

// codegen.rs:1473
// TODO: Add comprehensive method support
f.instruction(&Instruction::I32Const(0));  // Unknown method placeholder
```

**Missing Methods**:
- String: contains(), trim(), split(), to_lowercase(), to_uppercase()
- Array: push(), pop(), filter(), map(), reduce(), find()

**Acceptance Criteria**:
- [ ] Implement string methods in WASM
- [ ] Implement array methods in WASM
- [ ] Test: All common string methods work
- [ ] Test: All common array methods work

**Estimated Effort**: 3-4 days

---

### Issue #8: Proper Enum Tag Checking
**Priority**: P1
**Labels**: bug, pattern-matching
**Milestone**: Phase 1 - Foundation

**Description**:
Pattern matching on enum variants doesn't check tags, just executes body.

**Location**: src/codegen.rs:1656

**Code Reference**:
```rust
// codegen.rs:1656
// TODO: Implement proper enum tag checking and field extraction
// Just generate the body expression for this arm
```

**Impact**: Pattern matching doesn't work correctly for enums

**Acceptance Criteria**:
- [ ] Check enum discriminant (tag)
- [ ] Extract variant fields
- [ ] Jump to correct match arm based on tag
- [ ] Test: Result<T, E> pattern matching works
- [ ] Test: Custom enum pattern matching works

**Estimated Effort**: 2-3 days

---

## 📋 P2 - Medium Priority (Nice to Have for v1.0)

### Issue #9: Two-Pass Type Checking
**Priority**: P2
**Labels**: improvement, type-system
**Milestone**: Phase 2 - Features

**Description**:
Type checker returns Type::Any for forward references instead of proper two-pass checking.

**Location**: src/type_checker.rs:291

**Code Reference**:
```rust
// type_checker.rs:291
// TODO: Implement proper two-pass type checking to handle forward references
Ok(Type::Any)
```

**Acceptance Criteria**:
- [ ] First pass: Collect all type signatures
- [ ] Second pass: Perform type checking
- [ ] Test: Forward function references type check correctly
- [ ] Test: Mutually recursive functions work

**Estimated Effort**: 3 days

---

### Issue #10: Cleanup Unused Code
**Priority**: P2
**Labels**: cleanup, tech-debt
**Milestone**: Phase 1 - Foundation

**Description**:
Multiple unused methods, fields, and variables throughout codebase.

**Affected Areas**:
- Semantic analyzer: analyze_captures(), collect_variable_references()
- Parser: parse_macro_invocation(), refresh_peek()
- Various unused fields: id, velocity, current_line, name
- Unused enum variants: Memory, Global, Table
- Multiple unused variables in codegen.rs

**Acceptance Criteria**:
- [ ] Remove genuinely unused code
- [ ] Add #[allow(dead_code)] for intentionally unused code
- [ ] Document why code is kept if unused
- [ ] Target: 0 unused code warnings

**Estimated Effort**: 1-2 days

---

## 📝 Documentation Issues

### Issue #11: Fix All Empty Lines After Doc Comments
**Priority**: P3
**Labels**: style, clippy
**Milestone**: Phase 1 - Foundation

**Description**:
Clippy warns about 15+ empty lines after doc comments.

**Example**:
```rust
/// This is a doc comment

pub fn foo() {}  // ❌ Empty line after ///
```

**Acceptance Criteria**:
- [ ] Remove empty lines after doc comments
- [ ] Run clippy with no empty-line warnings

**Estimated Effort**: 30 minutes (trivial)

---

### Issue #12: Add Default Implementations
**Priority**: P3
**Labels**: improvement, ergonomics
**Milestone**: Phase 2 - Features

**Description**:
Clippy suggests Default implementations for several types.

**Types**:
- BorrowChecker
- SemanticAnalyzer
- RArray<T>
- RMap<K, V>
- RegistryClient
- CodeSplitter
- JSMinifier
- Compiler

**Acceptance Criteria**:
- [ ] Implement Default for all suggested types
- [ ] Test: Can create instances with Type::default()

**Estimated Effort**: 1-2 hours

---

## 🔧 Code Quality Issues

### Issue #13: Replace push_str with push for Single Chars
**Priority**: P3
**Labels**: performance, clippy
**Milestone**: Phase 2 - Features

**Description**:
11+ instances of `push_str("\n")` should use `push('\n')` for better performance.

**Files**:
- src/js_emitter.rs (primary)
- src/code_splitter.rs

**Acceptance Criteria**:
- [ ] Replace all single-char push_str with push
- [ ] Verify no performance regression

**Estimated Effort**: 15 minutes (trivial)

---

### Issue #14: Simplify Identical If Blocks
**Priority**: P3
**Labels**: code-quality, clippy
**Milestone**: Phase 2 - Features

**Description**:
Multiple if statements with identical blocks.

**Count**: 6 instances

**Acceptance Criteria**:
- [ ] Merge identical if/else branches
- [ ] Extract common logic to functions
- [ ] Run clippy with no identical-blocks warnings

**Estimated Effort**: 1 hour

---

### Issue #15: Fix Confusing Lifetime Elision
**Priority**: P3
**Labels**: code-quality, clippy
**Milestone**: Phase 2 - Features

**Description**:
Hiding a lifetime that's elided elsewhere is confusing.

**Acceptance Criteria**:
- [ ] Make lifetime parameters explicit
- [ ] Or remove redundant lifetime annotations
- [ ] Run clippy with no lifetime warnings

**Estimated Effort**: 1 hour

---

## 📊 Issue Summary

| Priority | Count | Category | Est. Total Effort |
|----------|-------|----------|-------------------|
| P0 | 4 | Critical Features | 3-4 weeks |
| P1 | 6 | High Priority | 2-3 weeks |
| P2 | 3 | Medium Priority | 4-5 days |
| P3 | 5 | Code Quality | 1-2 days |

**Total Issues**: 18
**Total Estimated Effort**: 6-8 weeks

---

## 📅 Suggested Prioritization

### Week 1-2 (Current - Phase 1)
- Issue #4: Forward reference handling
- Issue #5: Semantic analyzer for field access
- Issue #8: Enum tag checking
- Issue #10: Cleanup unused code
- Issue #11-15: All P3 code quality issues

### Week 3-4 (Phase 1.3-1.5)
- **Issue #1: JSX Support** ⭐ CRITICAL PATH

### Week 5-6 (Phase 2.1-2.2)
- Issue #2: WASM function tables
- Issue #3: Closure capture
- Issue #6: Function signatures for call_indirect

### Week 7-8 (Phase 2.3-2.5)
- Issue #7: String/array methods
- Issue #9: Two-pass type checking
- Issue #12: Default implementations

---

## 🔗 Cross-References

**Documentation**:
- DEVELOPMENT_PLAN_3_PHASES.md - Overall roadmap
- CLOSURE_STATUS.md - Closure implementation details
- PARSER_LIMITATIONS.md - JSX implementation requirements
- DAY1_PROGRESS.md - Day 1 accomplishments
- STATUS.md - Current project status

**Codebase**:
- src/codegen.rs - Most TODOs (8 total)
- src/type_checker.rs - 1 TODO
- src/semantic_analyzer.rs - 2 TODOs

---

## ✅ How to Use This Document

1. **Create GitHub Issues**: Copy each issue section into GitHub
2. **Add Labels**: Apply labels as specified
3. **Set Milestones**: Use Phase 1, Phase 2, or Phase 3
4. **Assign Priority**: Use P0, P1, P2, P3 labels
5. **Track Progress**: Update issue status in GitHub
6. **Link PRs**: Reference issue numbers in commits/PRs

**Example Commit Message**:
```
Fix forward reference handling (#4)

- Implement two-pass semantic analysis
- First pass collects function signatures
- Second pass validates all calls
- Fixes codegen.rs:848 TODO

Closes #4
```

---

**Document Status**: Ready for GitHub import
**Next Action**: Create GitHub repository and import these issues
