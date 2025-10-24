# CLAUDE.md - Jounce Compiler Guide

## 📌 Current Status

**Phase**: Phase 10 - Production Readiness & Polish
**Version**: 0.2.0 → 0.3.0
**Tests**: 564 core (100%) + 74 stdlib (65 passing = 87.8%)
**Latest Commit**: e0ad951 - Phase 9 Sprint 4 Complete

## 🎯 What Works

**Language**: JSX, async/await, generics, traits, pattern matching, closures, recursion, try (?), loops, unit type ()
**CSS**: css! macro, 150+ utilities, responsive/state/dark variants
**Dev Tools**: LSP (8 features), watch mode, formatter, package manager, test framework, source maps
**Stdlib**: JSON, DateTime, Crypto, File I/O, YAML (87% complete), HTTP client, collections
**SSR**: Server-side rendering, JSX→HTML, client hydration

## 🚀 Quick Commands

```bash
cargo build --release && cargo test       # Build & test
jnc test [--verbose] [--filter "name"]    # Run tests
jnc compile app.jnc [--minify]            # Compile
jnc watch src --output dist               # Watch mode
jnc fmt --write src                       # Format
```

## 📋 Phase 10: Production Readiness (v0.3.0)

### Sprint 1: Fix Remaining Tests (NEXT)

**Goal**: 74/74 stdlib tests passing (100%)

**9 YAML Tests Failing**:
1. test_parse_scalars - "Called unwrap on None"
2. test_parse_flow_sequence - "Called unwrap on None"
3. test_parse_flow_mapping - "Expected Result::Ok variant"
4. test_parse_nested - "Expected Result::Ok variant"
5. test_parse_mixed_sequence - Type mismatch
6. test_yaml_value_sequence_manipulation - "Cannot read properties of undefined"
7. test_yaml_value_mapping_manipulation - "Cannot read properties of undefined"
8. test_quoted_strings - "value.ends_with is not a function"
9. test_complex_nested_structure - "Expected Result::Ok variant"

**Root Causes**:
- StringBuilder vs primitive string method mismatches
- YAML parser Option/Result handling edge cases
- Complex nested structure parsing issues

**Approach**:
1. Debug each test individually to identify exact failure point
2. Fix StringBuilder method compatibility
3. Add missing string methods if needed
4. Fix YAML parser edge cases
5. Verify all 74 tests pass

**Timeline**: 1-2 days

### Sprint 2: Performance Optimization

- Activate parallel compilation (cache infrastructure ready)
- Benchmark suite
- Minification improvements

### Sprint 3: Documentation & Polish

- Complete stdlib API docs (JSON, DateTime, Crypto, File I/O already documented)
- Tutorial series
- Code cleanup (remove 20 warnings)

### Sprint 4: Production Features

- SSR dev server with HMR
- Production build optimizations
- CLI enhancements

**Target**: v0.3.0 - "Production Ready" (2-3 weeks)

## 📂 Key Files

**Core**: src/lib.rs, main.rs (1340 lines), lexer.rs, parser.rs, js_emitter.rs
**Stdlib**: src/stdlib/{json,time,crypto,fs,yaml}.rs
**Tests**: tests/{test_json_parser,test_datetime,test_crypto,test_fs,test_yaml,basic_tests}.jnc
**Docs**: docs/api/YAML_MODULE.md, docs/design/*, docs/archive/

## 🔧 Dev Patterns

**Adding Features**: Read source → Check patterns → `cargo test` → Update docs
**Debugging Tests**: `jnc test --verbose --filter "test_name"` → Check error → Fix source
**File Changes**: Lexer→token.rs, Parser→ast.rs, Codegen→js_emitter.rs

## 🎯 Next Steps (START HERE)

1. **Run failing tests individually** to understand exact errors:
   ```bash
   jnc test --verbose --filter "test_parse_scalars"
   ```

2. **Check StringBuilder compatibility** - Are all string methods available?

3. **Fix YAML parser edge cases** - Handle Option/Result properly

4. **Verify fixes** - Run full test suite after each fix

5. **Achieve 100%** - All 74 stdlib tests passing

## 📚 History

**Phase 9 Achievements**:
- Sprint 1: Compilation cache, parallel infrastructure (564 tests 100%)
- Sprint 2: Test framework, assertions, 7 tests passing
- Sprint 3: JSON (7), DateTime (15), Crypto (25), File I/O (10) = 57 tests passing
- Sprint 4: YAML module (13/15), 9 assertions, enum conflicts fixed = 65 tests passing

**Detailed History**: See `docs/archive/CLAUDE_*.md` for full Phase 1-9 details

---

**Last Updated**: 2025-10-24
**Status**: Ready for Phase 10 Sprint 1 - Fix remaining 9 YAML tests
**Next Session**: Debug and fix YAML test failures, achieve 100% pass rate
