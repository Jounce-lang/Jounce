# Jounce Development Plan - 3 Phases

**Created**: 2025-10-20
**Current Status**: Phase 7 - Building Examples (85% overall complete)
**Goal**: Production-ready v1.0 launch with full feature completeness

---

## Executive Summary

Jounce is 85% complete with a solid compiler foundation, complete standard library, and deployed package ecosystem. However, research reveals critical gaps that must be addressed before v1.0 launch:

### Critical Issues Discovered
1. ❌ **Test Suite Failing** - Cargo tests show 6 compilation errors despite STATUS.md claiming 178 passing tests
2. ❌ **JSX Not Implemented** - Component syntax exists in examples but doesn't compile
3. ⚠️ **Incomplete Closure Support** - Parser exists but no WASM function table implementation
4. ⚠️ **Code Quality** - 37 compiler warnings, unused code, type mismatches

### Strategic Priorities
This 3-phase plan focuses on:
- **Phase 1 (Foundation)**: Fix broken tests, stabilize codebase, complete core features
- **Phase 2 (Features)**: Implement JSX, complete closures, polish developer experience
- **Phase 3 (Launch)**: Production examples, documentation, community launch

---

## 📊 Current State Assessment

### ✅ What's Working Well
- **Core Compiler Pipeline**: Lexer → Parser → Type Checker → Borrow Checker → Codegen
- **Standard Library**: 100% complete (9 modules: option, result, vec, json, time, hashmap, string, fs, iterator)
- **Annotation System**: @server/@client code splitting functional
- **RPC Generation**: Automatic client/server communication working
- **Package Ecosystem**: Registry deployed at jounce-registry.fly.dev with 4 seed packages
- **Developer Tools**: HMR, package manager CLI, VSCode extension, LSP basics

### ❌ Critical Gaps
1. **JSX Support**:
   - Status: NOT IMPLEMENTED (per PARSER_LIMITATIONS.md)
   - Impact: All component-based examples fail to compile
   - Files affected: 15+ examples including todo_app, analytics_dashboard, devboard
   - Error: "ParserError: No prefix parse function for Slash"

2. **Test Suite Stability**:
   - Current: 6 compilation errors, 37 warnings
   - Claimed: 178 tests passing (STATUS.md outdated?)
   - Issues: Type mismatches in parser.rs, unused variables, method compilation errors

3. **Closure Implementation**:
   - Status: PARTIAL (per CLOSURE_STATUS.md)
   - Parser: ✅ Complete
   - Type system: ✅ Complete
   - Code generation: ❌ Missing WASM function tables
   - Capture analysis: ❌ Not implemented
   - Impact: Cannot store lambdas in variables or pass as arguments

4. **Code Quality**:
   - 37 compiler warnings
   - Multiple TODOs in critical paths (codegen.rs has 8 TODOs)
   - Forward reference handling incomplete
   - Method call support partial

### 📈 Metrics
- **Total Code**: ~16,000+ lines (compiler + stdlib + packages)
- **Examples**: 70+ .jnc files (many non-functional due to JSX)
- **Test Coverage**: Unknown (tests currently broken)
- **Compilation Speed**: 15.2µs average (excellent)
- **Documentation**: Comprehensive (README, GETTING_STARTED, FULLSTACK_GUIDE)

---

## 🎯 Phase 1: Foundation & Stabilization
**Duration**: 2-3 weeks
**Goal**: Fix broken tests, stabilize codebase, complete critical features
**Success Criteria**: All tests passing, JSX working, no compilation warnings

### Week 1: Emergency Stabilization

#### 1.1 Fix Broken Tests (Days 1-2) 🔥 CRITICAL
**Priority**: P0 - Blocking everything else

**Issues to Fix**:
```
src/parser.rs:39 - Type mismatch: expected &mut Lexer, found Vec<Token>
src/wasm_optimizer.rs:441 - Unused mut variable
src/js_emitter.rs:50 - Unused mut variable
```

**Tasks**:
- [ ] Fix parser initialization type mismatch
- [ ] Remove unused mut declarations
- [ ] Run `cargo test` and ensure all tests pass
- [ ] Update STATUS.md with actual test count
- [ ] Add CI/CD to prevent future breakage

**Deliverable**: `cargo test` passes with 0 errors, 0 warnings

#### 1.2 Code Quality Sweep (Days 3-4)
**Priority**: P0

**Tasks**:
- [ ] Fix all 37 compiler warnings
- [ ] Remove all unused variables/imports
- [ ] Run `cargo clippy` and fix all issues
- [ ] Add `#[allow(dead_code)]` only where justified
- [ ] Document all public APIs with ///
- [ ] Run `cargo fmt` across entire codebase

**Deliverable**: Clean `cargo clippy` and `cargo build --release` output

#### 1.3 Implement JSX Lexer (Days 5-7)
**Priority**: P0 - Required for component examples

**Reference**: PARSER_LIMITATIONS.md Phase 1

**Tasks**:
- [ ] Add JSX token types to src/token.rs:
  - `LessThan`, `GreaterThan`, `Slash`, `JsxText`, `JsxSelfClose`
- [ ] Implement JSX tokenization in src/lexer.rs:
  - Detect `<` in JSX context vs less-than operator
  - Handle JSX opening tags: `<div`
  - Handle JSX closing tags: `</div>`
  - Handle self-closing tags: `<img />`
  - Handle JSX text content
  - Handle JSX expressions: `{variable}`
  - Handle JSX comments: `{/* comment */}`
- [ ] Add context switching (code mode vs JSX mode)
- [ ] Write comprehensive lexer tests

**Test Cases**:
```raven
// Test 1: Simple element
<div>Hello</div>

// Test 2: Self-closing
<img src="foo.png" />

// Test 3: Nested
<div><span>text</span></div>

// Test 4: With expressions
<h1>{title}</h1>

// Test 5: With attributes
<button class="btn" onclick={handler}>Click</button>
```

**Deliverable**: JSX tokenization working with 20+ passing tests

### Week 2: JSX Parser & Code Generation

#### 1.4 Implement JSX Parser (Days 8-11)
**Priority**: P0

**Reference**: PARSER_LIMITATIONS.md Phase 2

**Tasks**:
- [ ] Add JSX AST nodes to src/ast.rs:
  ```rust
  Expression::JsxElement {
      tag: String,
      attributes: Vec<JsxAttribute>,
      children: Vec<JsxChild>,
  }
  ```
- [ ] Implement `parse_jsx_element()` in src/parser.rs
- [ ] Implement `parse_jsx_attributes()`
- [ ] Implement `parse_jsx_children()`
- [ ] Implement `parse_jsx_expression()`
- [ ] Handle nested JSX elements
- [ ] Handle JSX fragments: `<>...</>`
- [ ] Write comprehensive parser tests

**Test Cases**: Compile all examples from lexer phase

**Deliverable**: JSX AST construction working with 30+ passing tests

#### 1.5 Implement JSX Code Generation (Days 12-14)
**Priority**: P0

**Reference**: PARSER_LIMITATIONS.md Phase 3

**Tasks**:
- [ ] Add JSX-to-JavaScript generation in src/js_emitter.rs:
  ```javascript
  // <div class="foo">text</div>
  // Becomes:
  createElement("div", {class: "foo"}, "text")
  ```
- [ ] Add JSX-to-DOM helpers in dist/client-runtime.js:
  - `createElement(tag, props, ...children)`
  - Event handler attachment
  - Attribute setting
  - Child appending
- [ ] Handle JSX event handlers (`onClick`, `onInput`, etc.)
- [ ] Handle reactive expressions: `{signal.get()}`
- [ ] Test with real component examples

**Test Example**:
```raven
component Button(label: String) {
    <button class="btn">{label}</button>
}
```

**Deliverable**: Component compilation working, examples/counter_app.jnc compiles

### Week 3: Complete Core Features

#### 1.6 Fix Critical TODOs (Days 15-17)
**Priority**: P1

**Files to Address**:

**src/codegen.rs** (8 TODOs):
- [ ] Line 848: Improve forward reference handling
- [ ] Line 966: Implement full closure environment
- [ ] Line 1079: Use semantic analyzer for field access
- [ ] Line 1392: Track function signatures for call_indirect
- [ ] Line 1467: Implement string/array method support
- [ ] Line 1473: Add comprehensive method support
- [ ] Line 1656: Proper enum tag checking
- [ ] Line 1755: Implement capture analysis

**src/type_checker.rs** (1 TODO):
- [ ] Line 291: Two-pass type checking for forward references

**src/semantic_analyzer.rs** (2 TODOs):
- [ ] Review and address semantic analysis TODOs

**Deliverable**: All P0 TODOs resolved, P1 TODOs documented with workarounds

#### 1.7 Integration Testing (Days 18-21)
**Priority**: P0

**Tasks**:
- [ ] Compile all 70+ examples and track success rate
- [ ] Fix examples that should work but don't
- [ ] Mark aspirational examples as "v1.1+"
- [ ] Create test matrix:
  - Basic functions ✅
  - Closures (limited) ⚠️
  - Components ✅ (after JSX)
  - Server/client splitting ✅
  - RPC generation ✅
- [ ] Write end-to-end tests for full compilation pipeline
- [ ] Test package manager with real packages
- [ ] Test HMR with live server

**Success Metric**: 80%+ of examples compile successfully

**Deliverable**: Comprehensive test report, updated STATUS.md

---

## 🚀 Phase 2: Feature Completion & Polish
**Duration**: 3-4 weeks
**Goal**: Complete all missing features, enhance developer experience
**Success Criteria**: Full closure support, excellent error messages, production-ready tooling

### Week 4: Advanced Closures

#### 2.1 WASM Function Tables (Days 22-24)
**Priority**: P1

**Reference**: CLOSURE_STATUS.md Phase 1

**Tasks**:
- [ ] Generate WASM function table in codegen.rs
- [ ] Add functions to table during codegen
- [ ] Implement `call_indirect` instruction
- [ ] Support function parameters and return values
- [ ] Add type signatures for all function table entries
- [ ] Test simple higher-order functions:
  ```raven
  fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
      return f(x);
  }
  ```

**Deliverable**: Function pointers working, can call functions stored in variables

#### 2.2 Closure Environments (Days 25-28)
**Priority**: P1

**Reference**: CLOSURE_STATUS.md Phase 2

**Tasks**:
- [ ] Implement capture analysis for closures
- [ ] Generate closure structs: [func_index, env_ptr]
- [ ] Allocate environment on heap (WASM linear memory)
- [ ] Pass environment pointer to closure functions
- [ ] Support mutable captures with RefCell
- [ ] Test closure capture:
  ```raven
  fn make_adder(n: i32) -> fn(i32) -> i32 {
      return |x| x + n;  // Captures n
  }
  ```

**Deliverable**: Full closure support with capture semantics

### Week 5: Developer Experience

#### 2.3 Enhanced Error Messages (Days 29-31)
**Priority**: P1

**Tasks**:
- [ ] Improve diagnostic messages in src/diagnostics.rs
- [ ] Add "did you mean?" suggestions for typos
- [ ] Show code snippets with error highlighting
- [ ] Add helpful hints for common mistakes:
  - "Missing @client annotation for component"
  - "Cannot call server function from server code"
  - "Closure captures variable that doesn't live long enough"
- [ ] Test error messages with beginners
- [ ] Add error codes (E001, E002, etc.)
- [ ] Create error documentation page

**Example**:
```
error[E042]: Server function called from server context
  --> src/main.jnc:15:20
   |
15 |     let data = get_users();  // @server function
   |                ^^^^^^^^^^^^ Cannot call @server from @server
   |
   = note: Server functions can only be called from @client code
   = help: Remove @server annotation or call from client
```

**Deliverable**: User-friendly error messages for all common scenarios

#### 2.4 LSP Enhancements (Days 32-35)
**Priority**: P2

**Tasks**:
- [ ] Add hover information with type signatures
- [ ] Add go-to-definition for functions/types
- [ ] Add code actions:
  - "Add @client annotation"
  - "Import missing module"
  - "Add type annotation"
- [ ] Add semantic highlighting
- [ ] Add inline type hints
- [ ] Test VSCode extension with real projects
- [ ] Add autocomplete for stdlib functions
- [ ] Add snippet support

**Deliverable**: Production-quality IDE experience

### Week 6-7: Production Tooling

#### 2.5 Advanced Compilation Features (Days 36-42)
**Priority**: P2

**Tasks**:
- [ ] Implement tree shaking (remove unused code)
- [ ] Improve WASM optimization:
  - Dead code elimination
  - Constant folding
  - Inline small functions
- [ ] Add compilation cache for faster rebuilds
- [ ] Add incremental compilation
- [ ] Add source maps for debugging
- [ ] Add profiling output:
  ```bash
  raven compile --profile
  # Shows: Lexing: 2ms, Parsing: 8ms, Codegen: 15ms
  ```
- [ ] Add build size reporting:
  ```bash
  # Bundle sizes:
  #   server.js: 45 KB (12 KB gzipped)
  #   client.js: 32 KB (9 KB gzipped)
  #   app.wasm: 28 KB
  ```

**Deliverable**: Production-grade compilation with optimization

#### 2.6 Testing Framework (Days 43-49)
**Priority**: P2

**Tasks**:
- [ ] Expand raven-test package
- [ ] Add test runner: `raven test`
- [ ] Support test annotations:
  ```raven
  #[test]
  fn test_addition() {
      assert_eq!(add(2, 3), 5);
  }
  ```
- [ ] Add assertion macros:
  - `assert_eq!`, `assert_ne!`, `assert!`
  - `expect().to_equal()`, `expect().to_be_truthy()`
- [ ] Add test coverage reporting
- [ ] Add snapshot testing
- [ ] Add async test support
- [ ] Create test documentation

**Deliverable**: Complete testing framework with 50+ tests for stdlib

---

## 🎉 Phase 3: Production Launch
**Duration**: 3-4 weeks
**Goal**: Production examples, comprehensive docs, v1.0 launch
**Success Criteria**: 5+ production examples, complete docs, 1000+ GitHub stars

### Week 8-9: Production Examples

#### 3.1 Todo App with Authentication (Days 50-56)
**Priority**: P0 - Flagship example

**Features**:
- User registration and login (JWT)
- CRUD operations for todos
- Real-time updates with WebSockets
- Filtering and search
- Database persistence
- Deployment to Fly.io

**Tech Stack**:
```raven
@server
// PostgreSQL + JWT authentication
fn create_user(email: String, password: String) -> Result<User, Error>
fn login(email: String, password: String) -> Result<Token, Error>
fn get_todos(user_id: i32) -> Vec<Todo>

@client
// React-like components with raven-ui
component TodoApp() { ... }
component TodoList(todos: Vec<Todo>) { ... }
component TodoItem(todo: Todo) { ... }
component LoginForm() { ... }
```

**Deliverable**:
- Full working app deployed at todos.jounce.dev
- Complete tutorial walkthrough
- Video demo (5-10 minutes)

#### 3.2 Blog Platform (Days 57-63)
**Priority**: P1

**Features**:
- Markdown editor with live preview
- Image upload to S3/Cloudinary
- Comments system
- User profiles
- SEO optimization
- RSS feed generation
- Syntax highlighting for code blocks

**Deliverable**:
- Working blog deployed at blog.jounce.dev
- Tutorial documentation

#### 3.3 E-commerce Store (Days 64-70)
**Priority**: P1

**Features**:
- Product catalog with search/filters
- Shopping cart with persistence
- Checkout flow
- Stripe payment integration
- Order management
- Admin dashboard
- Email notifications

**Deliverable**:
- Demo store deployed at shop.jounce.dev
- Integration guide for Stripe

### Week 10-11: Documentation & Content

#### 3.4 Complete Documentation (Days 71-77)
**Priority**: P0

**Tasks**:
- [ ] Update README.md with latest features
- [ ] Update GETTING_STARTED.md with JSX examples
- [ ] Create LANGUAGE_REFERENCE.md:
  - Complete syntax guide
  - All language features
  - Type system details
  - Memory model
  - Closure semantics
- [ ] Create API_REFERENCE.md:
  - All stdlib modules
  - All stdlib functions
  - Usage examples
- [ ] Create DEPLOYMENT_GUIDE.md:
  - Deploy to Vercel
  - Deploy to Netlify
  - Deploy to Fly.io
  - Deploy to AWS Lambda
  - Docker deployment
- [ ] Create MIGRATION_GUIDE.md:
  - From React
  - From Vue
  - From Svelte
  - From Next.js
- [ ] Add code examples to all docs (100+ examples)

**Deliverable**: Comprehensive documentation site at docs.jounce.dev

#### 3.5 Tutorial Content (Days 78-84)
**Priority**: P0

**Tasks**:
- [ ] Write "Build a Todo App in 10 Minutes" tutorial
- [ ] Write "Understanding @server/@client" guide
- [ ] Write "Full-Stack Forms Made Easy" guide
- [ ] Write "Real-time Apps with WebSockets" guide
- [ ] Create video tutorial series (5-7 videos):
  1. Introduction to Jounce (10 min)
  2. Your First Component (15 min)
  3. Server Functions and RPC (20 min)
  4. Building a Todo App (30 min)
  5. State Management (15 min)
  6. Forms and Validation (20 min)
  7. Deployment to Production (15 min)
- [ ] Create interactive playground at play.jounce.dev
- [ ] Write blog post series:
  - "Introducing Jounce: Full-Stack in One File"
  - "Why We Built a New Language for AI-Assisted Development"
  - "Compiling to WebAssembly: Lessons Learned"
  - "The Future of Full-Stack Development"

**Deliverable**: Complete learning path from beginner to advanced

### Week 12: Launch Preparation

#### 3.6 Performance Audit (Days 85-88)
**Priority**: P1

**Tasks**:
- [ ] Benchmark compilation speed (target: <20µs)
- [ ] Benchmark bundle sizes (target: <50KB gzipped)
- [ ] Benchmark runtime performance:
  - First paint (<100ms)
  - Time to interactive (<200ms)
  - Lighthouse score (>90)
- [ ] Profile memory usage
- [ ] Test with large applications (10,000+ LOC)
- [ ] Optimize hot paths
- [ ] Add performance regression tests

**Deliverable**: Performance report showing excellent metrics

#### 3.7 Security Audit (Days 89-91)
**Priority**: P0

**Tasks**:
- [ ] Review RPC security:
  - Input validation
  - SQL injection prevention
  - XSS prevention
  - CSRF protection
- [ ] Review authentication:
  - JWT best practices
  - Secure token storage
  - Session management
- [ ] Add security guide to docs
- [ ] Run security scanner on generated code
- [ ] Test with OWASP Top 10 scenarios
- [ ] Add Content Security Policy support
- [ ] Document security best practices

**Deliverable**: Security audit report and hardening guide

#### 3.8 v1.0 Launch (Days 92-98)
**Priority**: P0

**Launch Checklist**:
- [ ] All Phase 1 & 2 features complete
- [ ] 5+ production examples deployed
- [ ] Comprehensive documentation
- [ ] 10+ tutorial videos
- [ ] Package registry fully operational
- [ ] VSCode extension published
- [ ] Website deployed at jounce.dev:
  - Landing page
  - Documentation
  - Examples
  - Playground
  - Blog
- [ ] Social media accounts setup:
  - Twitter: @jounce_lang
  - Discord: discord.gg/jounce
  - Reddit: r/jounce
- [ ] Launch blog post written
- [ ] Press kit prepared
- [ ] Submit to:
  - Hacker News
  - Reddit (r/programming, r/rust, r/webdev)
  - Dev.to
  - Lobsters
  - Product Hunt
- [ ] Conference talk proposals submitted
- [ ] Email campaign to early adopters

**Launch Targets**:
- 🎯 1,000 GitHub stars in first week
- 🎯 100 VSCode extension installs
- 🎯 50 packages published to registry
- 🎯 10 community contributions
- 🎯 Hacker News front page

**Deliverable**: Official v1.0 release

---

## 📈 Success Metrics

### Technical Metrics
- ✅ 100% test pass rate (200+ tests)
- ✅ 0 compiler warnings
- ✅ <20µs compilation time
- ✅ <50KB gzipped bundle size
- ✅ >90 Lighthouse score
- ✅ 100% documentation coverage

### Adoption Metrics
- 🎯 1,000+ GitHub stars
- 🎯 100+ VSCode extension installs
- 🎯 50+ packages in registry
- 🎯 10+ community contributors
- 🎯 5+ production applications deployed

### Content Metrics
- 🎯 10+ tutorial videos
- 🎯 20+ blog posts
- 🎯 100+ code examples
- 🎯 5+ production examples
- 🎯 Complete API documentation

---

## 🚨 Risk Management

### High-Risk Items

#### 1. JSX Implementation Complexity
**Risk**: JSX parsing more complex than estimated
**Mitigation**:
- Start with minimal JSX support (no fragments, no spread)
- Iterate based on real-world usage
- Fallback: DOM builder API if JSX takes too long

#### 2. Closure Performance
**Risk**: WASM function tables slow down execution
**Mitigation**:
- Benchmark early and often
- Inline closures when possible
- Provide escape hatch for performance-critical code

#### 3. Community Adoption
**Risk**: Low initial interest
**Mitigation**:
- Focus on AI-assisted development angle
- Create viral demo videos
- Partner with influencers
- Emphasize unique value proposition

#### 4. Scope Creep
**Risk**: Adding too many features delays v1.0
**Mitigation**:
- Strict adherence to this 3-phase plan
- Move non-critical features to v1.1
- Weekly progress reviews
- Hard deadline: v1.0 by end of December 2025

### Medium-Risk Items
- Browser compatibility issues → Test in all major browsers
- Package registry downtime → Add monitoring and backups
- Breaking changes needed → Provide migration tools
- Documentation outdated → Automate doc generation from code

---

## 📅 Timeline Summary

| Phase | Duration | Dates | Deliverables |
|-------|----------|-------|--------------|
| **Phase 1: Foundation** | 3 weeks | Nov 1-21 | Tests passing, JSX working, clean code |
| **Phase 2: Features** | 4 weeks | Nov 22 - Dec 19 | Closures complete, great DX, production tooling |
| **Phase 3: Launch** | 4 weeks | Dec 20 - Jan 16 | Examples, docs, v1.0 release |

**Total**: 11 weeks
**v1.0 Launch Date**: January 16, 2026

---

## 🎯 Next Actions (Starting Tomorrow)

### Day 1 (November 1, 2025)
1. Fix parser type mismatch in src/parser.rs:39
2. Run `cargo test` and document all failures
3. Create GitHub issue tracker for all TODOs
4. Set up CI/CD pipeline

### Week 1 Priorities
1. Get tests passing (P0)
2. Fix all compiler warnings (P0)
3. Start JSX lexer implementation (P0)
4. Update STATUS.md with accurate metrics (P1)

### Month 1 Goal
Complete Phase 1 - Foundation & Stabilization

---

## 📞 Questions to Answer Before Starting

### Technical Questions
- [ ] Should we support JSX fragments (`<>...</>`) in v1.0?
- [ ] What's the minimum closure support for v1.0?
- [ ] Do we need async/await in v1.0 or can it wait for v1.1?
- [ ] Should we support mobile compilation in v1.0?

### Strategic Questions
- [ ] What's the killer demo for v1.0 launch?
- [ ] Who is the primary target audience?
- [ ] What's our competitive advantage vs Next.js/Remix?
- [ ] How do we market the AI-assisted development angle?

### Resource Questions
- [ ] Is this a solo project or do we need contributors?
- [ ] What's the budget for hosting/infrastructure?
- [ ] Do we need paid marketing or rely on organic growth?

---

## ✅ Definition of Done

### Phase 1 Complete When:
- [ ] `cargo test` shows 200+ tests passing, 0 failures
- [ ] `cargo clippy` shows 0 warnings
- [ ] All examples compile (80%+ success rate)
- [ ] JSX components work end-to-end
- [ ] STATUS.md updated with accurate metrics

### Phase 2 Complete When:
- [ ] Closures fully functional with captures
- [ ] LSP provides excellent autocomplete
- [ ] Error messages are beginner-friendly
- [ ] Compilation speed <20µs average
- [ ] Testing framework published and documented

### Phase 3 Complete When:
- [ ] 5 production examples deployed and accessible
- [ ] Documentation complete (100% coverage)
- [ ] Tutorial videos published (5-7 videos)
- [ ] v1.0 released and announced
- [ ] 1,000+ GitHub stars achieved

---

**Document Status**: Strategic plan created 2025-10-20
**Next Review**: After Phase 1 completion
**Owner**: Jounce Core Team
**Priority**: P0 - Execute immediately
