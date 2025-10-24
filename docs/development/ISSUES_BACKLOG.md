# Issues Backlog

Issues discovered but not yet fixed, prioritized for future sprints.

**Last Updated**: 2025-10-21

---

## 🔴 Critical (Blocks Compilation)

### #B001: Option Constructors Missing
- **Discovered**: Sprint 6 (2025-10-21)
- **Status**: 🔴 OPEN
- **Impact**: All apps using Some()/None fail compilation
- **Error**: `Undefined function: 'Some'`
- **Locations**:
  - test_unwrap_or.jnc:2
  - examples/apps/ecommerce/main.jnc:126, 129, 431, 443, 447, 457, 536
  - examples/apps/social/main.jnc:72, 119, 131
- **Complexity**: HIGH
- **Estimated Effort**: 2-3 hours
- **Approach**: Add Some/None as keywords (like true/false) or built-in functions
- **Dependencies**: None
- **Assigned to**: Sprint 7

### #B002: Unicode/Emoji Lexer Support
- **Discovered**: Sprint 6 (2025-10-21)
- **Status**: 🔴 OPEN
- **Impact**: Blocks social app, taskboard app
- **Error**: `Expected LAngle, found Illegal('🔔')`
- **Locations**:
  - examples/apps/social/main.jnc:495 (🔔 bell emoji)
  - examples/apps/taskboard/main.jnc:478 (📋 clipboard emoji)
- **Complexity**: MEDIUM
- **Estimated Effort**: 1-2 hours
- **Approach**: Update lexer to handle multi-byte UTF-8 characters in string literals and JSX text
- **Dependencies**: None
- **Assigned to**: Sprint 7

### #B004: Parser Token Buffering with JSX Mode
- **Discovered**: Sprint 7 (2025-10-21)
- **Status**: 🔴 OPEN
- **Impact**: Blocks ALL JSX features including emojis, expressions
- **Error**: Various - tokens created with wrong jsx_mode state
- **Locations**:
  - All JSX code when entering/exiting JSX mode
- **Complexity**: HIGH (architectural)
- **Estimated Effort**: 4-6 hours
- **Approach**: Implement lazy tokenization OR lexer auto-detects JSX context
- **Dependencies**: None
- **Root Cause**: Parser 2-token buffer (current + peek) created before jsx_mode changes
- **Assigned to**: Future Sprint

---

## 🟡 High Priority (Degrades UX)

(None yet)

---

## 🟢 Low Priority (Nice-to-have)

(None yet)

---

## ✅ Completed Issues

- ✅ #B003: PipePipe prefix parsing for `|| { }` closures (Sprint 7)
- ✅ #000: Type casting with `as` keyword (Sprint 6)
- ✅ #-02: Division operator `/` (Sprint 1-4)
- ✅ #-01: README.md outdated limitations (Sprint 6)
