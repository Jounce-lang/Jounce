# Sprint 3 Critical Findings

## Discovery Summary

While attempting to create educational examples for Jounce, we discovered that **many "core" language features don't actually work**, despite CLAUDE.md claiming "100% complete" and "production ready" status.

## Root Cause Analysis

### Test Coverage Gap

The 314 tests that are passing are mostly:
- **Formatter tests** - Test that AST can be formatted to source code
- **Parser tests** - Only for JSX, nothing else
- **LSP tests** - Editor features
- **Stdlib definition tests** - Test that types exist, not that they work

**Missing**: End-to-end compilation tests that parse source → compile → run

### The Tests Don't Test Actual Compilation

Example from `src/formatter.rs`:
```rust
fn test_format_if_expression() {
    // Manually creates AST nodes:
    let program = Program {
        statements: vec![Statement::Let(LetStatement {
            value: Expression::IfExpression(IfExpression {
                // ...
            }),
        })],
    };

    // Only tests formatting, not parsing or compiling!
    let formatted = formatter.format_program(&program);
}
```

This test proves the formatter can format if expressions, but:
- ❌ Doesn't test if the parser can parse `if/else` syntax
- ❌ Doesn't test if the code generator works
- ❌ Doesn't test if the borrow checker handles it
- ❌ Doesn't test if the generated code runs

## What Actually Works vs What's Claimed

### ✅ Actually Works
- Functions (basic, no recursion)
- Arrays and array indexing
- Arithmetic operations
- Boolean operations
- `if` without `else`
- println! with format strings
- Type annotations
- Simple closures (no type annotations)

### ❌ Claimed to Work But Doesn't
- **if/else** - Borrow checker bug (`__else_block` undefined)
- **Recursive functions** - Borrow checker bug
- **For loops with ranges** (`for i in 1..10`) - Parser doesn't support
- **Match arms with OR** (`3 | 4 | 5`) - Parser doesn't support
- **Option/Result** - Requires if/else which is broken
- **Closures with types** (`|x: i32| -> i32`) - Parser doesn't support
- **Vec operations** - Partially implemented
- **Most control flow** - Limited to very basic patterns

## Impact on "100% Complete" Claim

The claim that Phase 1 is "100% complete" is **misleading**. The language supports:
- ~30% of expected Rust-like features
- ~50% if you count partially working features
- 100% of JSX (actually works!)

## Why This Happened

1. **Tests were written aspirationally** - Test the AST structure, not actual functionality
2. **No integration tests** - No tests that compile .jnc files end-to-end
3. **Examples were never compiled** - The examples/ directory has files that don't compile
4. **Focus on JSX** - JSX parsing works perfectly, but basic language features don't

## Recommended Next Steps

### Option 1: Fix Critical Bugs First (Recommended)
1. Fix borrow checker `__else_block` bug
2. Implement for loop range syntax
3. Add proper integration tests
4. Re-evaluate "100% complete" claim

### Option 2: Update Documentation to Match Reality
1. Change CLAUDE.md to reflect actual status
2. Mark unimplemented features clearly
3. Create "Known Limitations" guide
4. Set realistic expectations

### Option 3: Abandon Examples Sprint
1. Pause Sprint 3
2. Fix compiler bugs first
3. Resume examples when language actually works

## Test Coverage Recommendations

Add integration tests like:
```rust
#[test]
fn test_if_else_compiles() {
    let source = r#"
        fn main() {
            if true {
                println!("yes");
            } else {
                println!("no");
            }
        }
    "#;

    let result = compile_source(source);
    assert!(result.is_ok());
}
```

## Status Update for CLAUDE.md

**Current (Incorrect)**:
- Phase 1: ✅ 100% Complete
- Production Ready: ✅ YES

**Should Be**:
- Phase 1: ⚠️ ~30-50% Complete (JSX ✅, Core Lang ❌)
- Production Ready: ❌ NO (critical bugs in borrow checker)

---

**Created**: 2025-10-22
**Sprint**: Phase 3, Sprint 3
**Impact**: CRITICAL - Blocks all example creation
