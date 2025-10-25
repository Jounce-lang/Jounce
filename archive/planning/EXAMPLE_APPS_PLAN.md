# Jounce Example Applications Plan

## 🎯 Goal
Create a progressive series of small example applications that:
1. Teach Jounce features incrementally
2. Fit entirely in an LLM's context window (~30-40k tokens total)
3. Leave room for LLM to write full applications
4. Serve as learning material for both humans and AI

## 📊 Token Budget Analysis

**Target Context Window**: 200k tokens (Claude, GPT-4)
**Reserved for Examples**: ~30-40k tokens (~15-20k lines of code)
**Reserved for Compiler/Tooling Context**: ~50k tokens
**Available for LLM to Write Apps**: ~110k tokens

**Per Example Target**: 100-300 lines (200-600 tokens)

## 🎓 Example Curriculum (12 Progressive Apps)

### **Tier 1: Basics (3 examples, ~600 lines total)**

#### 1. Hello World (50 lines)
**Features**: Basic syntax, functions, console output
```raven
fn main() {
    println!("Hello, Jounce!");
}
```
**Teaches**:
- Function declaration
- Standard library (println!)
- Entry point

---

#### 2. Variables & Types (100 lines)
**Features**: let, const, mut, primitive types, type inference
**Teaches**:
- Variable declarations
- Type annotations
- Mutability
- Primitive types (i32, f64, String, bool)
- Type inference

---

#### 3. Control Flow (150 lines)
**Features**: if/else, match, for, while loops
**Teaches**:
- Conditional statements
- Pattern matching basics
- Iteration
- Ranges (0..10)

---

### **Tier 2: Core Features (3 examples, ~900 lines total)**

#### 4. Functions & Closures (200 lines)
**Features**: Function parameters, return types, closures, higher-order functions
**Teaches**:
- Function signatures
- Return values
- Closures (|x| x + 1)
- map, filter, reduce
- Function composition

---

#### 5. Structs & Enums (250 lines)
**Features**: Struct definitions, enum variants, impl blocks, methods
**Teaches**:
- Data structures
- Methods and associated functions
- Enum pattern matching
- Option<T> and Result<T, E>

---

#### 6. Collections (250 lines)
**Features**: Vec, HashMap, HashSet, iteration
**Teaches**:
- Working with collections
- Iterator methods
- Ownership and borrowing basics
- Collection manipulation

---

### **Tier 3: Reactive & JSX (2 examples, ~600 lines total)**

#### 7. Reactive State (200 lines)
**Features**: Signal, reactive updates, derived state
**Teaches**:
- Signal::new()
- .get() and .update()
- Reactive programming
- State management

---

#### 8. JSX Basics (400 lines)
**Features**: JSX elements, attributes, event handlers, basic components
**Teaches**:
- JSX syntax
- Components
- Event handlers
- Props
- Conditional rendering

---

### **Tier 4: Full-Stack (2 examples, ~1000 lines total)**

#### 9. Simple Counter App (400 lines)
**Features**: @client, @server separation, Signal, JSX, event handlers
**Teaches**:
- Full-stack single file
- Client/server annotations
- RPC basics
- Complete reactive UI

**Example Structure**:
```raven
@server
fn increment_on_server(count: i32) -> i32 {
    count + 1
}

@client
fn Counter() -> JSX {
    let count = Signal::new(0);

    <div>
        <h1>Count: {count.get()}</h1>
        <button onClick={|| count.update(|c| c + 1)}>
            Increment
        </button>
    </div>
}
```

---

#### 10. Todo List (600 lines)
**Features**: Full CRUD, local storage, form handling
**Teaches**:
- State management
- Form handling
- List rendering
- Local persistence
- Complex event handlers

---

### **Tier 5: Advanced (2 examples, ~1400 lines total)**

#### 11. API Integration (500 lines)
**Features**: HTTP requests, async/await, error handling, loading states
**Teaches**:
- Async functions
- HTTP client (stdlib)
- Error handling with Result
- Loading states
- Data fetching patterns

---

#### 12. Mini E-commerce (900 lines)
**Features**: Complex state, routing, forms, server API
**Teaches**:
- Multi-component architecture
- Complex state management
- Form validation
- Server-side logic
- Database patterns (simplified)
- Complete application structure

---

## 📁 File Organization

```
examples/
├── 01-basics/
│   ├── 01-hello-world.jnc          (50 lines)
│   ├── 02-variables-types.jnc      (100 lines)
│   └── 03-control-flow.jnc         (150 lines)
├── 02-core/
│   ├── 04-functions-closures.jnc   (200 lines)
│   ├── 05-structs-enums.jnc        (250 lines)
│   └── 06-collections.jnc          (250 lines)
├── 03-reactive/
│   ├── 07-reactive-state.jnc       (200 lines)
│   └── 08-jsx-basics.jnc           (400 lines)
├── 04-fullstack/
│   ├── 09-counter-app.jnc          (400 lines)
│   └── 10-todo-list.jnc            (600 lines)
└── 05-advanced/
    ├── 11-api-integration.jnc      (500 lines)
    └── 12-mini-ecommerce.jnc       (900 lines)
```

**Total**: ~4,500 lines (~9,000 tokens)

---

## 🎯 Token Budget Breakdown

| Component | Lines | Tokens (est.) | % of 200k |
|-----------|-------|---------------|-----------|
| Examples (12 apps) | 4,500 | ~9,000 | 4.5% |
| CLAUDE.md | 550 | ~1,100 | 0.6% |
| README.md | 200 | ~400 | 0.2% |
| LSP_FEATURES.md | 400 | ~800 | 0.4% |
| Key source files | 5,000 | ~10,000 | 5% |
| **Total Context** | ~10,650 | ~21,300 | **10.7%** |
| **Available for LLM to write** | - | ~178,700 | **89.3%** |

---

## 📝 Each Example Should Include

1. **Header Comment**:
   ```raven
   // Example 01: Hello World
   // Demonstrates: Basic syntax, functions, console output
   // Features: fn, println!, entry point
   ```

2. **Runnable Code**: Complete, working example

3. **Inline Comments**: Explain key concepts

4. **Output Examples**: Show what it produces

5. **Progressive Complexity**: Build on previous examples

---

## 🎓 Learning Path

**For Humans**:
1. Start with Tier 1 (Basics)
2. Progress through Tier 2 (Core Features)
3. Learn reactive programming (Tier 3)
4. Build full-stack apps (Tier 4)
5. Master advanced patterns (Tier 5)

**For LLMs**:
- All examples in context window
- Can reference any example when writing new code
- Learns patterns progressively
- Understands full feature set
- Can generate similar applications

---

## 🚀 Implementation Plan (Sprint 2)

### Phase 1: Create Tier 1-2 (Basics + Core)
**Time**: 2-3 hours
**Output**: 6 examples, ~1,500 lines

### Phase 2: Create Tier 3 (Reactive + JSX)
**Time**: 2-3 hours
**Output**: 2 examples, ~600 lines

### Phase 3: Create Tier 4-5 (Full-Stack + Advanced)
**Time**: 4-5 hours
**Output**: 4 examples, ~2,400 lines

**Total Sprint Time**: 8-11 hours

---

## ✅ Success Criteria

- [ ] All 12 examples compile successfully
- [ ] Total token count < 10k for examples
- [ ] Each example is self-contained and runnable
- [ ] Progressive difficulty (no sudden jumps)
- [ ] All major language features covered
- [ ] Examples are well-commented
- [ ] Each example < 1000 lines
- [ ] Combined examples fit in LLM context with room to spare

---

## 🎯 Next Steps

1. **Start with Tier 1** (Hello World, Variables, Control Flow)
2. **Test compilation** of each example
3. **Verify token counts** stay within budget
4. **Document patterns** in each example
5. **Create index/catalog** of examples

---

**Ready to start building?** 🚀

Let's begin with Tier 1: The Basics!
