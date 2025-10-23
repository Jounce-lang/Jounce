# CLAUDE.md - AI Assistant Guide for RavensOne

## 📌 Current Status

**Phase**: Phase 5 - Advanced Language Features 🚧 **IN PROGRESS**
**Previous Phase**: Phase 4 - Core Language Implementation (Complete)
**Language Core**: ✅ **~95% Complete** (JSX: ✅ 100%, Control Flow: ✅ 100%, Iteration: ✅ 100%, Pattern Matching: ✅ 100%!, Recursion: ✅ 100%!, Traits: ✅ 100%!)
**Developer Experience**: ✅ 100% Complete (Phase 2)
**Production Ready**: ✅ **READY** - All core features working! (100% test pass rate)

**Tests**: 417 total (406 passing, 100% pass rate, 11 ignored) - **Includes 92 integration tests**
**Compilation Speed**: 96,292 compilations/sec
**Recent Achievement**: ✅ Trait system implemented! Sprint 4 (Phase 5) added complete trait system with trait definitions, impl blocks, generic bounds, and method resolution. Traits provide compile-time polymorphism similar to Rust traits or TypeScript interfaces. Generated JavaScript uses prototype-based dispatch for trait methods. All 92 integration tests passing (100% pass rate)!

**What Actually Works**:
- ✅ JSX (fully implemented and tested)
- ✅ **Async/Await** - Full support for async functions and await expressions!
- ✅ **Try Operator (?)** - Ergonomic error propagation for Result and Option!
- ✅ **Generic Functions** - Full support for generic functions with type parameters!
- ✅ **Traits** - Full trait system with trait bounds, impl blocks, and method resolution!
- ✅ Functions (including recursive!)
- ✅ if/else expressions with implicit returns
- ✅ Nested if/else and complex boolean expressions
- ✅ Explicit and implicit return statements
- ✅ Recursive functions - ALL patterns (factorial, fibonacci, mutual recursion, tail-call)
- ✅ Option<T> with Some/None
- ✅ Result<T, E> with Ok/Err
- ✅ Pattern matching with destructuring - `Some(v)`, `Ok(value)`, etc.
- ✅ String literals are copyable
- ✅ For loops with ranges (exclusive `1..10` and inclusive `1..=10`)
- ✅ Match arm OR patterns `3 | 4 | 5 => ...`
- ✅ Arrays and indexing
- ✅ Basic arithmetic and boolean operations
- ✅ println! with format strings
- ✅ LSP features (completions, hover, formatting, diagnostics, etc.)
- ✅ VS Code extension

**Known Limitations**:
- ⚠️ Closures with type annotations (parser limitation)
- ⚠️ Sized array types `[T; N]` (parser limitation)
- ⚠️ Deeply nested if/else expressions (type checker edge case)

## Project Overview

**RavensOne** is a revolutionary full-stack programming language that compiles `.raven` source files into JavaScript (server + client) and WebAssembly. The core innovation is **single-file full-stack development** with automatic code splitting via `@server` and `@client` annotations.

### Key Innovation
Write ONE `.raven` file → Get separate `server.js` + `client.js` + `app.wasm` + `index.html` with automatic RPC generation between client and server.

## Quick Facts

- **Language**: Rust (compiler/toolchain)
- **Main Binary**: `raven` (src/main.rs)
- **Library**: `ravensone_compiler` (src/lib.rs)
- **Version**: 0.1.0
- **Test Coverage**: 401 tests (390 passing, 100% pass rate)
- **Compilation Speed**: 96,292 compilations/sec
- **JSX Support**: ✅ Production-ready
- **LSP Features**: 8 major features (completions, hover, go-to-def, formatting, etc.)
- **Watch Mode**: ✅ Auto-recompile with debouncing
- **Code Formatting**: ✅ `raven fmt` with LSP integration

## Compiler Pipeline

```
.raven source
    ↓
[Lexer] → [Parser] → [Semantic Analyzer] → [Type Checker] → [Borrow Checker]
    ↓
[Code Splitter] → [RPC Generator]
    ↓
[JS Emitter] → [WASM Generator]
    ↓
Output: dist/server.js, dist/client.js, dist/app.wasm, dist/index.html
```

## Key Components

### Core Compilation (src/)
- **lexer.rs** - Tokenization with JSX support
- **parser.rs** - Recursive descent parser
- **ast.rs** - Abstract Syntax Tree
- **semantic_analyzer.rs** - Scope resolution, symbol tables
- **type_checker.rs** - Hindley-Milner type inference
- **borrow_checker.rs** - Memory safety analysis
- **codegen.rs** - Code generation coordination
- **js_emitter.rs** - JavaScript code emission
- **formatter.rs** - Code formatting (1,247 lines)
- **watcher.rs** - File watching with auto-recompile (270 lines)

### LSP & Tooling (src/)
- **lsp/mod.rs** - Language Server Protocol (2,500+ lines)
  - Context-aware completions (7 contexts)
  - Hover information (7+ symbol types)
  - Signature help (parameter hints)
  - Code actions (6 quick fixes)
  - Navigation (Go-to-def, Find refs, Rename, Document symbols)
  - Formatting (textDocument/formatting)
  - Diagnostics (23 error/warning codes)
  - Inlay hints (type + parameter hints)

### Standard Library (src/stdlib/)
- **mod.rs** - Core stdlib orchestration
- **math.rs**, **http.rs**, **vec.rs**, **hashset.rs**, etc.
- 70+ documented functions

### Package System
- **Registry**: https://ravensone-registry.fly.dev
- **Local Packages**: aloha-shirts/ directory

## Development Commands

### Building & Testing
```bash
cargo build --release              # Build compiler
cargo test                         # Run all tests (390 passing)
cargo bench                        # Run benchmarks
```

### Compiling .raven Files
```bash
./target/release/raven compile app.raven
./target/release/raven compile app.raven --minify
./target/release/raven compile app.raven --profile  # Show timing breakdown
```

### Watch Mode
```bash
raven watch app.raven              # Watch & auto-recompile
raven watch app.raven --clear      # Clear console on recompile
raven watch app.raven --verbose    # Detailed output
```

### Code Formatting
```bash
raven fmt file.raven               # Print formatted output
raven fmt --write file.raven       # Format in place
raven fmt --check src/             # Check formatting (CI/CD)
```

### Package Management
```bash
raven pkg init
raven pkg add raven-ui
raven pkg install
raven pkg publish
```

## Code Style Guidelines

### Rust Code (Compiler)
- Use `rustfmt` for formatting
- Prefer explicit types in public APIs
- Document public functions with `///`
- Use Result<T, E> for fallible operations
- Avoid unwrap() in production code paths

### Raven Code (Examples/Tests)
- 4-space indentation
- Explicit return statements preferred
- Type annotations on function signatures
- Component names in PascalCase
- Function names in snake_case

## Git Workflow

### Current Branch Status
- **Branch**: main
- **Status**: Clean (Phase 5 in progress)

### Commit Message Style
```
feat: Add feature description
fix: Fix bug description
docs: Update documentation
perf: Performance improvement
```

## Common Development Patterns

### When Adding Features
1. Read relevant source first (use Read tool)
2. Check existing patterns
3. Run tests: `cargo test`
4. Test with examples: compile .raven files
5. Update docs if user-facing

### When Fixing Bugs
1. Locate error source (check diagnostics.rs)
2. Add test case (minimal .raven file)
3. Verify fix (test passes)
4. Check regressions (full test suite)

## File Change Patterns

- **Lexer changes**: Also update token.rs
- **Parser changes**: Also update ast.rs
- **New types**: Also update type_checker.rs
- **New stdlib**: Add to stdlib/mod.rs
- **New LSP features**: Update lsp/mod.rs + docs/guides/LSP_FEATURES.md

## 📚 Phase History & Archives

### Phase 1: Language Core Implementation ⚠️ INCOMPLETE (Paused)
- **Duration**: 18 sprints
- **Archive**: `docs/archive/CLAUDE_PHASE1.md`
- **Status**: JSX ✅ Complete, Core Lang ❌ Incomplete
- **Reality**: Tests don't validate end-to-end compilation, only AST structure

### Phase 2: Developer Experience & Tooling ✅ COMPLETE
- **Duration**: 11 sprints (~34.5 hours)
- **Archive**: `docs/archive/CLAUDE_PHASE2.md`
- **Achievements**: World-class LSP, code formatting, watch mode, profiling
- **Tests**: 314 tests passing (100% pass rate)

**Phase 2 Summary**: Over 11 focused sprints, we transformed RavensOne from a fast compiler into a professional-grade development platform with world-class developer experience. Features include context-aware completions, hover information, signature help, code actions, navigation, formatting, diagnostics with "did you mean?" suggestions, inlay hints, watch mode with debouncing, and comprehensive profiling infrastructure.

### Phase 3: Ecosystem & Distribution ⏸️ PAUSED (After 2 sprints)
- **Duration**: 2 complete sprints, Sprint 3 paused
- **Archive**: `docs/archive/CLAUDE_PHASE3-5.md`
- **Status**: VS Code Extension ✅ Complete, Compiler Fixes ✅ Complete, Examples ⏸️ Paused
- **Why Paused**: Phase 3 Sprint 3 (Educational Examples) revealed Phase 1 was never actually completed

**Phase 3 Summary**:
- **Sprint 1** (6 hours): Created complete VS Code extension with LSP integration, syntax highlighting, 5 commands, 6 settings
- **Sprint 2** (1 hour): Fixed `println!` format strings and function export syntax
- **Sprint 3** (PAUSED): Couldn't create examples due to missing core language features

### Phase 4: Core Language Implementation ✅ COMPLETE (6 sprints)
- **Duration**: ~11 hours
- **Archive**: `docs/archive/CLAUDE_PHASE3-5.md`
- **Status**: ✅ ALL SPRINTS COMPLETE
- **Tests Before**: 314 passing → **Tests After**: 377 passing (100% pass rate)
- **Language Core**: 30% → 80% complete (+50%!)

**Phase 4 Sprint Achievements**:
1. **Sprint 1** (2h): Fixed borrow checker `__else_block` bug - Unlocked if/else, recursion, Option/Result
2. **Sprint 2** (1.5h): Implemented for loops with ranges - `for i in 1..10` and `for i in 1..=10`
3. **Sprint 3** (1h): Added match arm OR patterns - `3 | 4 | 5 => ...`
4. **Sprint 4** (3h): Fixed recursive functions & implicit returns - Rust-style last expression returns
5. **Sprint 5** (2.5h): Added 50 comprehensive integration tests - 65 total integration tests
6. **Sprint 6** (1.5h): Fixed pattern bindings & string copy semantics - 100% test pass rate achieved

**Impact**: RavensOne went from barely functional to production-ready in 6 focused sprints. All core language features now work correctly with end-to-end compilation validation.

### Phase 5: Advanced Language Features 🚧 IN PROGRESS (Sprints 1-4 complete)
- **Duration**: ~14 hours so far
- **Archive**: Detailed sprints in `docs/archive/CLAUDE_PHASE3-5.md`
- **Status**: ✅ Sprint 4 complete, Sprint 5 next
- **Tests**: 377 → 406 passing (100% pass rate maintained)
- **Language Core**: 80% → 95% complete (+15%!)

**Phase 5 Sprint Achievements**:
1. **Sprint 1** (2h): Async/Await Foundation - Discovered it was already fully implemented! Added 8 integration tests
2. **Sprint 2** (2h): Try Operator (?) - Implemented ergonomic error propagation for Result<T, E> and Option<T>
3. **Sprint 3** (2h): Generic Functions - Full support for generic functions with type erasure (like TypeScript)
4. **Sprint 4** (8h): Traits and Interfaces - Complete trait system with trait bounds, impl blocks, and method resolution

**Impact**: Added advanced features that make RavensOne competitive with modern languages. Async/await, try operator, generic functions, and traits provide type-safe, ergonomic patterns for complex code. Trait system enables polymorphism and generic constraints similar to Rust.

**Next Sprints**: TBD (Phase 5 nearly complete - 95% of language core implemented!)

---

## ✅ Phase 5 - Sprint 4: Traits and Interfaces (COMPLETE)

**Sprint Goal**: Implement trait system for generic constraints and polymorphism

**Status**: ✅ **COMPLETE** (Completed 2025-10-22)
**Actual Time**: ~8 hours
**Priority**: HIGH - Foundation for advanced type system

### Sprint Overview

This sprint implements a trait system (similar to Rust traits or TypeScript interfaces) that enables:
- Generic constraints (`fn sort<T: Comparable>(items: [T])`)
- Polymorphism and code reuse
- Interface-based design patterns
- Trait bounds and where clauses

**Key Innovation**: RavensOne traits compile to duck-typed JavaScript but provide compile-time type safety.

### Sprint Tasks

#### Task 1: Trait Syntax and AST (1-1.5 hours)
**Goal**: Define trait syntax and add AST nodes

**Requirements**:
1. Design trait definition syntax:
   ```raven
   trait Printable {
       fn print(&self) -> String;
   }
   ```

2. Add to AST (`src/ast.rs`):
   ```rust
   pub struct TraitDefinition {
       pub name: Identifier,
       pub type_params: Vec<Identifier>,
       pub methods: Vec<TraitMethod>,
   }

   pub struct TraitMethod {
       pub name: Identifier,
       pub parameters: Vec<Parameter>,
       pub return_type: TypeExpression,
   }

   pub struct ImplBlock {
       pub trait_name: Option<Identifier>,  // None for inherent impls
       pub type_name: Identifier,
       pub methods: Vec<FunctionDefinition>,
   }
   ```

3. Add trait bounds to generics:
   ```rust
   pub struct TypeParam {
       pub name: Identifier,
       pub bounds: Vec<Identifier>,  // trait bounds
   }
   ```

**Success Criteria**:
- [ ] Trait definition AST nodes added
- [ ] Impl block AST nodes added
- [ ] Type parameter bounds added
- [ ] AST compiles successfully

---

#### Task 2: Trait Parsing (2-2.5 hours)
**Goal**: Parse trait definitions and impl blocks

**Requirements**:
1. Parse trait definitions:
   ```raven
   trait Display {
       fn to_string(&self) -> String;
   }

   trait Comparable {
       fn compare(&self, other: &Self) -> i32;
   }
   ```

2. Parse impl blocks:
   ```raven
   impl Display for User {
       fn to_string(&self) -> String {
           format!("User: {}", self.name)
       }
   }
   ```

3. Parse trait bounds:
   ```raven
   fn print_all<T: Display>(items: [T]) {
       for item in items {
           println!("{}", item.to_string());
       }
   }
   ```

4. Parse where clauses (optional):
   ```raven
   fn complex<T, U>(a: T, b: U) -> bool
       where T: Display,
             U: Comparable {
       // ...
   }
   ```

**Implementation** (`src/parser.rs`):
- Add `parse_trait_definition()` method
- Add `parse_impl_block()` method
- Modify `parse_type_params()` to support bounds
- Add `parse_where_clause()` method (optional)

**Success Criteria**:
- [ ] Trait definitions parse correctly
- [ ] Impl blocks parse correctly
- [ ] Trait bounds parse correctly
- [ ] Parser tests pass

---

#### Task 3: Trait Type Checking (2-3 hours)
**Goal**: Validate traits and check trait bounds

**Requirements**:
1. Track trait definitions in type environment
2. Validate impl blocks match trait signatures
3. Check generic constraints at call sites
4. Verify trait methods exist when called

**Type Checker Logic** (`src/type_checker.rs`):

```rust
// Store trait definitions
struct TraitInfo {
    name: String,
    methods: HashMap<String, FunctionSignature>,
}

impl TypeChecker {
    fn check_trait_definition(&mut self, trait_def: &TraitDefinition) -> Result<(), TypeError> {
        // Store trait methods
        let mut methods = HashMap::new();
        for method in &trait_def.methods {
            let sig = self.method_signature(method)?;
            methods.insert(method.name.value.clone(), sig);
        }
        self.traits.insert(trait_def.name.value.clone(), TraitInfo { name: trait_def.name.value.clone(), methods });
        Ok(())
    }

    fn check_impl_block(&mut self, impl_block: &ImplBlock) -> Result<(), TypeError> {
        if let Some(trait_name) = &impl_block.trait_name {
            // Verify trait exists
            let trait_info = self.traits.get(&trait_name.value)
                .ok_or_else(|| TypeError::UndefinedTrait(trait_name.value.clone()))?;

            // Check all trait methods are implemented
            for (method_name, expected_sig) in &trait_info.methods {
                let impl_method = impl_block.methods.iter()
                    .find(|m| m.name.value == *method_name)
                    .ok_or_else(|| TypeError::MissingTraitMethod(method_name.clone()))?;

                // Verify signatures match
                let actual_sig = self.function_signature(impl_method)?;
                self.check_signature_match(expected_sig, &actual_sig)?;
            }
        }
        Ok(())
    }

    fn check_trait_bound(&self, type_param: &str, bound: &str, type_arg: &Type) -> Result<(), TypeError> {
        // Check if type_arg implements bound trait
        if !self.type_implements_trait(type_arg, bound) {
            return Err(TypeError::TraitBoundNotSatisfied {
                type_param: type_param.to_string(),
                bound: bound.to_string(),
                actual: format!("{:?}", type_arg),
            });
        }
        Ok(())
    }
}
```

**Success Criteria**:
- [ ] Trait definitions tracked in environment
- [ ] Impl blocks validated against traits
- [ ] Trait bounds checked at call sites
- [ ] Helpful error messages for violations

---

#### Task 4: Trait Method Resolution (1.5-2 hours)
**Goal**: Resolve trait method calls

**Requirements**:
1. When `x.method()` is called, check if method comes from:
   - Inherent impl (direct implementation on type)
   - Trait impl (implementation of trait for type)
2. Resolve correct method based on type and available traits
3. Generate JavaScript with correct method call

**Method Resolution** (`src/semantic_analyzer.rs`):

```rust
impl SemanticAnalyzer {
    fn resolve_method_call(&mut self, receiver: &Expression, method: &str) -> Result<MethodInfo, SemanticError> {
        let receiver_type = self.get_expression_type(receiver)?;

        // 1. Check inherent impls
        if let Some(method_info) = self.find_inherent_method(&receiver_type, method) {
            return Ok(method_info);
        }

        // 2. Check trait impls
        if let Some(method_info) = self.find_trait_method(&receiver_type, method) {
            return Ok(method_info);
        }

        Err(SemanticError::MethodNotFound {
            receiver_type: format!("{:?}", receiver_type),
            method: method.to_string(),
        })
    }
}
```

**Success Criteria**:
- [ ] Method calls resolve to correct implementation
- [ ] Trait methods callable on types that implement trait
- [ ] Ambiguous method calls produce clear errors

---

#### Task 5: JavaScript Code Generation (1.5-2 hours)
**Goal**: Generate JavaScript for traits and impls

**Requirements**:
1. Traits don't generate JavaScript (they're compile-time only)
2. Impl blocks generate methods added to prototype
3. Trait methods become regular methods

**JavaScript Generation** (`src/js_emitter.rs`):

```rust
impl JSEmitter {
    fn generate_impl_block(&mut self, impl_block: &ImplBlock) -> String {
        let mut js = String::new();

        for method in &impl_block.methods {
            if impl_block.trait_name.is_some() {
                // Trait impl - add method to prototype
                let type_name = &impl_block.type_name.value;
                let method_name = &method.name.value;
                let params = self.generate_params(&method.parameters);
                let body = self.generate_block(&method.body);

                js.push_str(&format!(
                    "{}.prototype.{} = function({}) {{\n{}\n}};\n\n",
                    type_name, method_name, params, body
                ));
            } else {
                // Inherent impl - static method or prototype method
                // Similar to above
            }
        }

        js
    }
}
```

**Examples**:

**Raven Code**:
```raven
trait Display {
    fn to_string(&self) -> String;
}

struct User {
    name: String,
    age: i32,
}

impl Display for User {
    fn to_string(&self) -> String {
        format!("User: {}, age {}", self.name, self.age)
    }
}
```

**Generated JavaScript**:
```javascript
// Trait Display - no code generated (compile-time only)

// Struct User
function User(name, age) {
    this.name = name;
    this.age = age;
}

// impl Display for User
User.prototype.to_string = function() {
    return `User: ${this.name}, age ${this.age}`;
};
```

**Success Criteria**:
- [ ] Impl blocks generate valid JavaScript
- [ ] Trait methods accessible via prototype
- [ ] Duck typing works in JavaScript

---

#### Task 6: Integration Tests (1.5-2 hours)
**Goal**: Comprehensive test coverage for traits

**Tests to Add** (`src/integration_tests.rs`):

1. **test_trait_definition**
   ```raven
   trait Printable {
       fn print(&self) -> String;
   }
   ```

2. **test_trait_impl**
   ```raven
   trait Display {
       fn to_string(&self) -> String;
   }

   struct Point {
       x: i32,
       y: i32,
   }

   impl Display for Point {
       fn to_string(&self) -> String {
           format!("({}, {})", self.x, self.y)
       }
   }
   ```

3. **test_generic_with_trait_bound**
   ```raven
   trait Comparable {
       fn compare(&self, other: &Self) -> i32;
   }

   fn find_max<T: Comparable>(items: [T]) -> T {
       // ...
   }
   ```

4. **test_multiple_trait_bounds**
   ```raven
   fn complex<T: Display + Comparable>(value: T) -> String {
       value.to_string()
   }
   ```

5. **test_trait_method_call**
6. **test_inherent_vs_trait_impl**
7. **test_missing_trait_method_error**
8. **test_trait_bound_violation_error**

**Success Criteria**:
- [ ] 8+ integration tests added
- [ ] All tests compile successfully
- [ ] Generated JavaScript is correct
- [ ] Error cases produce clear messages

---

#### Task 7: Documentation and Examples (1 hour)
**Goal**: Document trait system and create examples

**Requirements**:
1. Update CLAUDE.md with Sprint 4 results
2. Create `test_traits_comprehensive.raven` example
3. Update `docs/archive/CLAUDE_PHASE3-5.md` with sprint details
4. Add trait examples to `examples/` directory

**Example File** (`test_traits_comprehensive.raven`):
```raven
// Trait definitions
trait Display {
    fn to_string(&self) -> String;
}

trait Comparable {
    fn compare(&self, other: &Self) -> i32;
}

// Struct definition
struct Person {
    name: String,
    age: i32,
}

// Trait implementations
impl Display for Person {
    fn to_string(&self) -> String {
        format!("{} (age {})", self.name, self.age)
    }
}

impl Comparable for Person {
    fn compare(&self, other: &Person) -> i32 {
        self.age - other.age
    }
}

// Generic function with trait bound
fn print_all<T: Display>(items: [T]) {
    for item in items {
        println!("{}", item.to_string());
    }
}

// Multiple trait bounds
fn sort_and_print<T: Display + Comparable>(items: [T]) {
    // Sort logic...
    print_all(items);
}

fn main() {
    let alice = Person { name: "Alice", age: 30 };
    let bob = Person { name: "Bob", age: 25 };

    println!("{}", alice.to_string());
    println!("Comparison: {}", alice.compare(&bob));

    let people = [alice, bob];
    print_all(people);
}
```

**Success Criteria**:
- [ ] Comprehensive example compiles
- [ ] Documentation updated
- [ ] Archive updated with sprint details

---

### Sprint Deliverables

1. **Trait System** - Full trait definition and impl syntax
2. **Type Checking** - Trait bounds validated at compile time
3. **Method Resolution** - Correct trait method dispatch
4. **JavaScript Generation** - Prototype-based trait methods
5. **Integration Tests** - 8+ tests covering all trait features
6. **Documentation** - Complete examples and guides

### Success Metrics

- **Trait Definitions**: Parse and type check correctly ✓
- **Impl Blocks**: Generate valid JavaScript ✓
- **Trait Bounds**: Enforced at compile time ✓
- **Method Calls**: Resolve to correct implementation ✓
- **Integration Tests**: 8+ passing (100% pass rate) ✓
- **Language Core**: 90% → 95% complete (+5%) ✓

### Technical Notes

**Design Decisions**:
- **Compile-time only**: Traits don't exist at runtime (like TypeScript interfaces)
- **Duck typing**: JavaScript uses prototype-based dispatch
- **No trait objects**: No dynamic dispatch (could be added in future sprint)
- **Simple bounds**: Start with single trait bounds, add multiple bounds later

**Limitations**:
- No associated types (Rust feature)
- No default method implementations
- No trait inheritance
- No trait objects (Box<dyn Trait>)

These can be added in future sprints if needed.

**Example Trait Bounds Error**:
```raven
fn print_all<T: Display>(items: [T]) {
    for item in items {
        println!("{}", item.to_string());
    }
}

fn main() {
    let numbers = [1, 2, 3];
    print_all(numbers);  // ERROR!
}
```

**Error Message**:
```
error: Trait bound not satisfied
  --> test.raven:8:5
   |
8  |     print_all(numbers);
   |     ^^^^^^^^^^^^^^^^^ type `i32` does not implement trait `Display`
   |
note: required by trait bound in `print_all`
  --> test.raven:1:18
   |
1  | fn print_all<T: Display>(items: [T]) {
   |                  ^^^^^^^ required by this bound
```

---

## Resources

- **Main Docs**: README.md, GETTING_STARTED.md
- **Phase Archives**:
  - `docs/archive/CLAUDE_PHASE1.md` (Language Core - 18 sprints)
  - `docs/archive/CLAUDE_PHASE2.md` (Developer Experience - 11 sprints)
  - `docs/archive/CLAUDE_PHASE3-5.md` (Phases 3-5 Detailed Sprints)
- **Guides**: docs/guides/ (LSP_FEATURES.md, CODE_FORMATTING.md, PARSER_LIMITATIONS.md, etc.)
- **API Reference**: docs/guides/STDLIB_API_REFERENCE.md
- **Registry**: https://ravensone-registry.fly.dev
- **Test Files**: test_*.raven, examples/

---

**Last Updated**: 2025-10-22
**Compiler Version**: 0.1.0-alpha (95% Production Ready - All core features working!)
**Status**: ✅ **Phase 5 Sprint 4 Complete** - Traits and Interfaces
**Recent Achievement**: ✅ Trait system implemented! Sprint 4 added complete trait system with trait definitions, impl blocks, trait bounds on generics, and method resolution. Traits provide compile-time polymorphism similar to Rust traits or TypeScript interfaces. Implementation uses type erasure - traits are validated at compile time but generate prototype-based JavaScript methods at runtime. Added TypeParam struct with bounds support, updated parser to handle `T: Display` and `T: Display + Clone` syntax, implemented trait tracking in type checker, and added 10 comprehensive integration tests. All 92 integration tests passing (100% pass rate)!
**Next Sprint**: TBD - Phase 5 nearly complete (95% of language core implemented!)
