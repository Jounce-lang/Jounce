# Repository Cleanup Plan
**Date**: 2025-10-21
**Purpose**: Consolidate documentation and clean up temporary files

---

## 📊 Current State

### Documentation Files (27 total)
- **12,026 total lines** of markdown documentation
- Many files are progress reports or status updates
- Some redundancy between files
- No organizational structure (all in root)

### Test Files (30+)
- Multiple temporary JSX test files
- Compiled WASM artifacts (.wasm files)
- Test scripts
- Debug files

---

## 🎯 Cleanup Strategy

### Phase 1: Create Documentation Structure

**Create `docs/` directory** with subdirectories:
```
docs/
├── guides/           # User-facing guides
├── development/      # Development progress reports
├── technical/        # Technical implementation details
└── archive/          # Outdated or superseded docs
```

### Phase 2: Consolidate Documentation

#### Keep in Root (5 files)
1. **README.md** - Main entry point
2. **CLAUDE.md** - AI assistant guide
3. **GETTING_STARTED.md** - Quick start tutorial
4. **PRODUCTION_READY_SUMMARY.md** - Overall status
5. **CHANGELOG.md** - (to be created)

#### Move to `docs/guides/` (6 files)
1. FULLSTACK_GUIDE.md
2. JSX_LEXER_USAGE.md
3. JSX_AST_GUIDE.md
4. PARSER_LIMITATIONS.md
5. CLOSURE_IMPLEMENTATION_SUMMARY.md
6. DOUBLE_COLON_OPERATOR.md

#### Move to `docs/development/` (7 files)
1. DAY1_PROGRESS.md
2. DAY2_PROGRESS.md
3. DAY3_PROGRESS.md
4. DAY4_PROGRESS.md
5. DAY5_PROGRESS.md
6. DAY6_PROGRESS.md
7. DAY7_PROGRESS.md

#### Move to `docs/technical/` (5 files)
1. CODEGEN_PROGRESS_SUMMARY.md
2. STDLIB_COMPLETION_SUMMARY.md
3. DEPLOYMENT_SUMMARY.md
4. CURRENT_ANNOTATION_STATUS.md
5. GITHUB_ISSUES.md

#### Archive or Delete (4 files)
1. **STATUS.md** → Archive (outdated, superseded by PRODUCTION_READY_SUMMARY.md)
2. **CLOSURE_STATUS.md** → Archive (superseded by CLOSURE_IMPLEMENTATION_SUMMARY.md)
3. **REMAINING_WARNINGS.md** → Delete (outdated - only 3 warnings now)
4. **ACCOMPLISHMENTS.md** → Merge into PRODUCTION_READY_SUMMARY.md
5. **DEVELOPMENT_PLAN_3_PHASES.md** → Archive (plan complete)

---

## 🧹 File Cleanup

### Temporary Test Files to Keep in Root
**Production test files** (keep for CI/CD):
- test_minimal.jnc
- test_simple_func.jnc
- test_closure.jnc
- test_closure_complex.jnc

### Create `test/` Directory
Move manual test files:
```
test/
├── jsx/              # JSX-specific tests
│   ├── test_jsx_simple.jnc
│   ├── test_jsx_text.jnc
│   ├── test_jsx_attrs.jnc
│   ├── test_jsx_nested.jnc
│   ├── test_jsx_expr.jnc
│   ├── test_jsx_one_attr.jnc
│   └── test_jsx_self_close_attr.jnc
│
├── features/         # Feature-specific tests
│   ├── test_indirect_call.jnc
│   ├── test_indirect_simple.jnc
│   ├── test_error.jnc
│   └── test_nofunction.jnc
│
└── legacy/          # Old test files
    ├── test-*.jnc (with hyphens)
    └── test_*.wasm
```

### Files to Delete
- **.wasm files** - Compilation artifacts (should be gitignored)
- **test_lexer_jsx.rs** - Temporary debug file
- **test_registry.sh** - If no longer needed
- **test-* files** - Legacy naming convention

---

## 📝 Updates Needed

### 1. Update .gitignore
Add:
```gitignore
# Build artifacts
/target
/dist
*.wasm

# Test outputs
test-*.wasm
test_*.wasm

# Editor
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Temporary
*.tmp
*.bak
```

### 2. Create CHANGELOG.md
Document version history:
```markdown
# Changelog

## [Unreleased]

### Added (Oct 21, 2025)
- Full JSX support (lexer, parser, AST, codegen)
- 24 JSX tests (13 lexer + 11 parser)
- JSX documentation (JSX_LEXER_USAGE.md, JSX_AST_GUIDE.md)
- Helper methods for JSX AST construction

### Fixed
- Critical parser bug in JSX attribute parsing
- parse_expression() → parse_prefix() fix

## [0.1.0] - 2025-10-20

### Added
- Complete compiler pipeline
- 211 tests passing
- Full standard library (9 modules)
- Package manager and registry
```

### 3. Update STATUS.md → Delete and Reference PRODUCTION_READY_SUMMARY.md

### 4. Create docs/README.md
Index of all documentation:
```markdown
# Documentation Index

## User Guides
- [Getting Started](../GETTING_STARTED.md)
- [Full-Stack Development](guides/FULLSTACK_GUIDE.md)
- [JSX Usage](guides/JSX_LEXER_USAGE.md)

## Technical Documentation
- [Parser Limitations](guides/PARSER_LIMITATIONS.md)
- [Closure Implementation](guides/CLOSURE_IMPLEMENTATION_SUMMARY.md)
...
```

---

## 🎯 Consolidation Recommendations

### Merge These Documents

1. **ACCOMPLISHMENTS.md** → Into PRODUCTION_READY_SUMMARY.md
   - Reason: Duplicate content, both track achievements

2. **STATUS.md** → Delete
   - Reason: Outdated (Oct 20), PRODUCTION_READY_SUMMARY is canonical

3. **CLOSURE_STATUS.md** → Delete
   - Reason: Superseded by CLOSURE_IMPLEMENTATION_SUMMARY.md

4. **REMAINING_WARNINGS.md** → Delete
   - Reason: Outdated (down to 3 warnings now)

### Create These New Documents

1. **CHANGELOG.md** - Version history
2. **docs/README.md** - Documentation index
3. **CONTRIBUTING.md** - Contribution guidelines

---

## 📁 Final Structure

```
jounce/
├── README.md                           # Main entry point
├── CLAUDE.md                           # AI guide
├── GETTING_STARTED.md                  # Quick start
├── PRODUCTION_READY_SUMMARY.md         # Overall status
├── CHANGELOG.md                        # Version history (NEW)
├── CONTRIBUTING.md                     # Contribution guide (NEW)
├── .gitignore                          # Updated
│
├── docs/
│   ├── README.md                       # Documentation index (NEW)
│   ├── guides/                         # User guides
│   │   ├── FULLSTACK_GUIDE.md
│   │   ├── JSX_LEXER_USAGE.md
│   │   ├── JSX_AST_GUIDE.md
│   │   ├── PARSER_LIMITATIONS.md
│   │   ├── CLOSURE_IMPLEMENTATION_SUMMARY.md
│   │   └── DOUBLE_COLON_OPERATOR.md
│   │
│   ├── development/                    # Progress reports
│   │   ├── DAY1_PROGRESS.md
│   │   ├── DAY2_PROGRESS.md
│   │   ├── DAY3_PROGRESS.md
│   │   ├── DAY4_PROGRESS.md
│   │   ├── DAY5_PROGRESS.md
│   │   ├── DAY6_PROGRESS.md
│   │   └── DAY7_PROGRESS.md
│   │
│   ├── technical/                      # Technical details
│   │   ├── CODEGEN_PROGRESS_SUMMARY.md
│   │   ├── STDLIB_COMPLETION_SUMMARY.md
│   │   ├── DEPLOYMENT_SUMMARY.md
│   │   ├── CURRENT_ANNOTATION_STATUS.md
│   │   └── GITHUB_ISSUES.md
│   │
│   └── archive/                        # Outdated docs
│       ├── STATUS.md
│       ├── CLOSURE_STATUS.md
│       └── DEVELOPMENT_PLAN_3_PHASES.md
│
├── test/
│   ├── jsx/                            # JSX tests
│   ├── features/                       # Feature tests
│   └── legacy/                         # Old tests
│
├── test_minimal.jnc                  # Core tests (keep in root)
├── test_simple_func.jnc
├── test_closure.jnc
└── test_closure_complex.jnc
```

---

## ✅ Execution Plan

### Step 1: Create Structure (5 min)
```bash
mkdir -p docs/{guides,development,technical,archive}
mkdir -p test/{jsx,features,legacy}
```

### Step 2: Move Documentation (10 min)
```bash
# Move to guides
mv FULLSTACK_GUIDE.md docs/guides/
mv JSX_LEXER_USAGE.md docs/guides/
mv JSX_AST_GUIDE.md docs/guides/
mv PARSER_LIMITATIONS.md docs/guides/
mv CLOSURE_IMPLEMENTATION_SUMMARY.md docs/guides/
mv DOUBLE_COLON_OPERATOR.md docs/guides/

# Move to development
mv DAY*_PROGRESS.md docs/development/

# Move to technical
mv CODEGEN_PROGRESS_SUMMARY.md docs/technical/
mv STDLIB_COMPLETION_SUMMARY.md docs/technical/
mv DEPLOYMENT_SUMMARY.md docs/technical/
mv CURRENT_ANNOTATION_STATUS.md docs/technical/
mv GITHUB_ISSUES.md docs/technical/

# Archive outdated
mv STATUS.md docs/archive/
mv CLOSURE_STATUS.md docs/archive/
mv DEVELOPMENT_PLAN_3_PHASES.md docs/archive/
```

### Step 3: Move Test Files (5 min)
```bash
# JSX tests
mv test_jsx_*.jnc test/jsx/

# Feature tests
mv test_indirect*.jnc test/features/
mv test_error.jnc test/features/
mv test_nofunction.jnc test/features/

# Legacy
mv test-*.jnc test/legacy/
mv test-*.wasm test/legacy/
```

### Step 4: Delete Files (2 min)
```bash
rm REMAINING_WARNINGS.md
rm test_lexer_jsx.rs
rm *.wasm  # Compilation artifacts
```

### Step 5: Create New Files (10 min)
- Create CHANGELOG.md
- Create docs/README.md
- Create CONTRIBUTING.md
- Update .gitignore

### Step 6: Update References (5 min)
- Update README.md links
- Update CLAUDE.md links
- Update PRODUCTION_READY_SUMMARY.md

---

## 📊 Impact

### Before Cleanup
- 27 markdown files in root
- 30+ test files scattered
- 12,026 lines of docs (no structure)
- No .gitignore for artifacts

### After Cleanup
- 6 markdown files in root (essentials)
- Organized docs/ with 3 subdirectories
- Organized test/ with categorization
- Clean .gitignore

### Benefits
- ✅ Easier navigation
- ✅ Clear separation of concerns
- ✅ Better for new contributors
- ✅ Cleaner git status
- ✅ Professional appearance

---

## ⚠️ Cautions

1. **Git History**: Moving files loses git blame history (use `git mv` to preserve)
2. **External Links**: Any external links to docs will break
3. **Local Scripts**: Check if any scripts reference moved files
4. **README Updates**: Must update all documentation links

---

## 🎯 Success Criteria

- [ ] All docs organized into logical folders
- [ ] Root directory has < 10 markdown files
- [ ] Test files categorized by purpose
- [ ] .gitignore prevents artifacts
- [ ] CHANGELOG.md tracks versions
- [ ] docs/README.md provides index
- [ ] All internal links updated
- [ ] Git history preserved with `git mv`

---

**Estimated Time**: 30-40 minutes
**Priority**: Medium (improves maintainability)
**Risk**: Low (mostly organizational)

**Recommendation**: Execute this cleanup before major release or when onboarding contributors.
