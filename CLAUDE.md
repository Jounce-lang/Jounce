# CLAUDE.md - Jounce Compiler Guide

## 📌 Current Status

**Phase**: Phase 9 Sprint 3 - Standard Library Expansion (IN PROGRESS)
**Version**: 0.2.0 | **Tests**: 564 core + 49 stdlib discovered | **Ext**: .jnc

**Latest**: ✅ Compiler enhancements! Fixed 11 major issues - stdlib tests now executing!

## 🎯 What Works

**Language**: JSX, async/await, generics, traits, pattern matching, closures, recursion, try (?), for/while/loop, break/continue, unit type (), hex literals (0x), bitwise ops (|&^), bit shifts (<<>>), dereference (*)
**CSS**: css! macro, scoped styles, 150+ utilities, responsive/state/dark variants, custom utilities
**Dev Tools**: LSP (8 features), watch mode, formatter, package manager, error diagnostics, source maps, test framework
**Stdlib**: JSON (parse/stringify), DateTime (formatting, timers), Crypto (hashing, random, UUID, base64), HTTP client, collections (RArray, RMap)

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

## 📋 Phase 9 Sprint 3 - Standard Library Expansion (IN PROGRESS)

**Goal**: Implement comprehensive stdlib modules for JSON, DateTime, Crypto, File I/O, and YAML parsing.

**Progress**:
- ✅ **Survey stdlib** (17 modules found: collections, http, db, auth, json, time, fs, etc.)
- ✅ **Architecture analysis** (Pattern 1: Rust impl, Pattern 2: Jounce source strings)
- ✅ **JSON Parser** (605 lines) - Full implementation with parsing, serialization, escape handling
  - parse(), stringify(), stringify_pretty()
  - JsonValue enum (Null, Bool, Number, String, Array, Object)
  - Type-safe manipulation (is_*, as_*, get, set, push, remove)
  - 7 comprehensive tests
- ✅ **DateTime Module** (670 lines) - Complete date/time support
  - DateTime: now(), parse(), format(), to_iso_string(), from_components()
  - Duration: from_seconds/minutes/hours/days, arithmetic, conversion
  - ZonedDateTime: UTC/Local timezone support
  - Timer & Stopwatch for performance measurement
  - parse_duration() helper ("5s", "2m", "1h", "3d")
  - 15 comprehensive tests
- ✅ **Crypto Module** (550+ lines) - Complete security primitives
  - Hashing: sha256(), sha1(), md5(), hmac_sha256()
  - Random: random_bytes/int/float/string, random_alphanumeric/hex
  - UUID: uuid_v4() with RFC 4122 format
  - Encoding: base64_encode/decode, hex_encode/decode
  - Password: hash_password_auto(), PBKDF2 with 100k iterations
  - 25 comprehensive tests
- 🔄 **Compiler Enhancements** (11 major fixes) - Running stdlib tests
  - Added: Unit type (), hex literals (0x), bitwise ops (|&^), bit shifts (<<>>)
  - Added: loop/break/continue statements, dereference/borrow operators
  - Fixed: String escaping, struct literal lookahead, return as expression
  - Fixed: Match arm blocks, namespace handling
  - Progress: 49 tests discovered, executing stdlib code (line 318 → 818)
- ⏸️ File I/O (skeleton exists, needs implementation)
- ⏸️ YAML parsing (not yet started)
- ⏸️ Documentation (pending)

**Test Files**:
- `tests/test_json_parser.jnc` (7 tests)
- `tests/test_datetime.jnc` (15 tests)
- `tests/test_crypto.jnc` (25 tests)

**Total**: 3 stdlib modules, ~1,825 lines of code, 47 tests

---

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

**Sprint 3 Progress**: 6/8 complete (75%)
- ✅ Survey stdlib implementation
- ✅ JSON parser implementation (605 lines, 7 tests)
- ✅ DateTime implementation (670 lines, 15 tests)
- ✅ Crypto module (550+ lines, 25 tests)
- ✅ Test file creation (47 tests total)
- 🔄 **Compiler fixes for stdlib execution** (11 major enhancements)

**Current Focus**: Fixing codegen edge cases to get all 49 stdlib tests passing

**Remaining Tasks**:
1. 🔄 **Fix remaining codegen issues** - Continue fixing JS generation bugs (in progress)
2. **File I/O completion** - Implement actual file operations (skeleton exists)
3. **YAML parsing** - Add YAML support (build on JSON)
4. **Documentation** - Document JSON, DateTime, Crypto modules
5. **Phase 10** - Move to next phase (Performance optimization, production readiness, etc.)

**Next**: Continue fixing codegen issues, then validate all 49 tests pass

---

**Last Updated**: 2025-10-23 | **Status**: Phase 9 Sprint 3 - Compiler enhancements! 11 major fixes, stdlib tests executing!
**Archives**: See `docs/archive/` for full Sprint 1-2 details
