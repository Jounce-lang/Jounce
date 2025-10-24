# CLAUDE.md - Jounce Compiler Guide

## 📌 Current Status

**Phase**: Phase 10 - Production Readiness & Polish
**Version**: 0.2.0 → 0.3.0
**Tests**: 564 core (100%) + 74 stdlib (74 passing = 100%) 🎉
**Latest Commit**: 26f4f0f - Phase 10 Sprint 1 Complete - All tests passing!

## 🎯 What Works

**Language**: JSX, async/await, generics, traits, pattern matching, closures, recursion, try (?), loops, unit type ()
**CSS**: css! macro, 150+ utilities, responsive/state/dark variants
**Dev Tools**: LSP (8 features), watch mode, formatter, package manager, test framework, source maps
**Stdlib**: JSON, DateTime, Crypto, File I/O, YAML (100% complete), HTTP client, collections
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

### Sprint 1: Fix Remaining Tests ✅ COMPLETE

**Goal**: 74/74 stdlib tests passing (100%) - **ACHIEVED!**

**Fixes Applied**:
1. **parse_float() NaN handling** - Changed from Option matching to `num == num` check
2. **Colon parsing** - Added `:` to stop characters in parse_scalar()
3. **Missing return statements** - Added explicit returns in YAML methods
4. **String.prototype.ends_with** - Added polyfill to JS emitter

**Results**:
- ✅ All 9 YAML tests fixed
- ✅ 74/74 stdlib tests passing (100%)
- ✅ 564/564 core tests passing (100%)

**Timeline**: Completed in 1 session (2025-10-24)

### Sprint 2: Performance Optimization (NEXT)

**Goals**:
- Activate parallel compilation (cache infrastructure ready)
- Create comprehensive benchmark suite
- Measure performance improvements (target: 10x faster incremental builds)
- Optimize minification

**Prerequisites**: ✅ All tests passing (Sprint 1 complete)

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

**Phase 10 Sprint 2 - Performance Optimization**

1. **Review cache infrastructure** from Phase 9 Sprint 1:
   - Modules: src/cache/{mod.rs, ast_cache.rs, dependency_graph.rs, disk_cache.rs}
   - Dependencies: xxhash-rust, rmp-serde, rayon, dashmap

2. **Integrate cache into compilation pipeline**:
   - Modify compile_source() to use AST cache
   - Implement dependency tracking
   - Add cache invalidation logic

3. **Enable parallel compilation**:
   - Use Rayon for parallel file compilation
   - Leverage topological sorting from dependency graph

4. **Create benchmark suite**:
   - Time cold builds vs warm builds
   - Measure cache hit rates
   - Test with various project sizes

5. **Measure and optimize**:
   - Target: 10x faster incremental builds
   - Document performance improvements

## 📚 History

**Phase 9 Achievements**:
- Sprint 1: Compilation cache, parallel infrastructure (564 tests 100%)
- Sprint 2: Test framework, assertions, 7 tests passing
- Sprint 3: JSON (7), DateTime (15), Crypto (25), File I/O (10) = 57 tests passing
- Sprint 4: YAML module (13/15), 9 assertions, enum conflicts fixed = 65 tests passing

**Phase 10 Achievements**:
- Sprint 1 ✅: Fixed all 9 YAML tests, 100% stdlib pass rate (74/74)

**Detailed History**: See `docs/archive/CLAUDE_*.md` for full Phase 1-9 details

---

**Last Updated**: 2025-10-24
**Status**: Phase 10 Sprint 1 COMPLETE - All tests passing (638/638 = 100%)
**Next Session**: Sprint 2 - Performance Optimization (activate parallel compilation)
