# CLAUDE.md - Jounce Compiler Guide

## 📌 Current Status

**Phase**: Phase 9 Sprint 2 - Developer Tools (100% COMPLETE!)
**Version**: 0.2.0 | **Tests**: 564 passing (100%) | **Ext**: .jnc

**Latest**: ✅ Test framework COMPLETE! All 7 integration tests passing!

## 🎯 What Works

**Language**: JSX, async/await, generics, traits, pattern matching, closures, recursion, try (?), for/while
**CSS**: css! macro, scoped styles, 150+ utilities, responsive/state/dark variants, custom utilities
**Dev Tools**: LSP (8 features), watch mode, formatter, package manager, error diagnostics, source maps, test framework

## 🚀 Commands

```bash
# Development
cargo build --release && cargo test
jnc compile app.jnc [--minify]
jnc watch src --output dist
jnc test [--verbose] [--filter "name"] [--watch]
jnc fmt --write src

# Package Manager
jnc pkg init/add/remove/tree
```

## 📋 Phase 9 Sprint 2 - Developer Tools (100% ✅ COMPLETE!)

- ✅ Error Reporting (873 lines) - Already production-ready
- ✅ Source Maps (356 lines) - Already production-ready
- ✅ LSP Refactoring (4,480 lines) - Already production-ready
- ✅ Test Framework Design (357 lines) - NEW
- ✅ Test Framework Implementation (314 lines) - NEW
- ✅ CLI Integration (COMPLETE - all 7 tests passing!)
- ⏸️ REPL (Deferred to Sprint 3)

**Test Results**: 7/7 passing (test_addition, test_subtraction, test_multiplication, test_is_even, test_boolean_assertions, test_not_equal, test_async_operation)

## 🧪 Test Framework

**Syntax**:
```jounce
fn test_addition() {
    assert_eq(add(2, 3), 5, "2 + 3 = 5");
}

async fn test_async() {
    let result = await fetch_data();
    assert_ok(result);
}
```

**Assertions**: assert, assert_eq, assert_ne, assert_true, assert_false, assert_some, assert_none, assert_ok, assert_err, assert_contains, assert_length, assert_empty, assert_not_empty, assert_approx

## 📂 Key Files

**Core**: `src/lib.rs`, `main.rs` (1340 lines), `lexer.rs`, `parser.rs`, `codegen.rs`
**Phase 9**: `src/cache/` (Sprint 1), `src/test_framework.rs` (314 lines, Sprint 2)
**Docs**: `docs/archive/` (full history), `docs/design/TEST_FRAMEWORK_DESIGN.md`
**Examples**: `examples/testing/basic_tests.jnc`

## 🔧 Dev Patterns

**Adding Features**: Read source → Check patterns → `cargo test` → Update docs
**File Changes**: Lexer→token.rs, Parser→ast.rs, Types→type_checker.rs, CSS→lexer+parser+ast+css_generator

## 🎯 Next Steps

1. ✅ CLI integration (test commands added to main.rs)
2. ⏳ Run integration test (verify test framework end-to-end)
3. ⏳ Update docs with Sprint 2 completion
4. 🎉 Sprint 2 DONE → Sprint 3

---

**Last Updated**: 2025-10-23 | **Status**: Phase 9 Sprint 2 - Testing CLI integration
**Archives**: See `docs/archive/CLAUDE_PHASE9_SPRINT2.md` for full Sprint 2 details
