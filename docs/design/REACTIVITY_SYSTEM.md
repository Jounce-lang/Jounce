# Jounce Reactivity System Design Specification

**Version**: v0.4.0-draft
**Status**: Design Phase
**Author**: Claude & Jordan Hill
**Date**: October 24, 2025

---

## 1. Overview

### 1.1 Goals

Add a fine-grained reactivity system to Jounce inspired by Solid.js that enables:

- **Automatic dependency tracking** - No manual subscription management
- **Minimal re-computation** - Only update what actually changed
- **Simple mental model** - Easy to understand and reason about
- **Type-safe** - Full integration with Jounce's type system
- **Zero runtime overhead** when not used

### 1.2 Non-Goals

- Virtual DOM (Jounce doesn't use VDOM)
- React-style hooks (different programming model)
- Async reactivity (keep it synchronous for simplicity)
- Reactive collections (defer to Phase 13+)

---

## 2. Core Concepts

### 2.1 Signals

**Definition**: A signal is a reactive container for a single value that notifies subscribers when changed.

**API**:
```jounce
// Create signal
let count = signal<int>(0);

// Read value
let current = count.value;

// Write value
count.value = 5;
```

**Properties**:
- Generic over any type `T`
- `.value` property for read/write access
- Automatically tracks readers (effects/computed)
- Notifies all subscribers on write

### 2.2 Computed Values

**Definition**: A computed value is a derived signal that automatically updates when its dependencies change.

**API**:
```jounce
let count = signal<int>(0);
let doubled = computed<int>(() => count.value * 2);

console.log(doubled.value);  // 0
count.value = 5;
console.log(doubled.value);  // 10 (automatically updated)
```

**Properties**:
- Read-only (no `.value = x` setter)
- Lazy evaluation (only runs when `.value` is accessed)
- Memoized (caches result until dependencies change)
- Can depend on multiple signals/computed values
- Forms a dependency graph (DAG)

### 2.3 Effects

**Definition**: An effect is a side-effect function that re-runs when its reactive dependencies change.

**API**:
```jounce
let count = signal<int>(0);

effect(() => {
    console.log("Count is: " + count.value.to_string());
});
// Logs: "Count is: 0"

count.value = 5;
// Logs: "Count is: 5"
```

**Properties**:
- Runs immediately on creation
- Re-runs when any tracked signal/computed changes
- Synchronous execution
- No return value
- Can create/destroy DOM, make network calls, etc.

### 2.4 Batch Updates

**Definition**: Batch multiple signal updates to prevent redundant effect executions.

**API**:
```jounce
let firstName = signal<string>("John");
let lastName = signal<string>("Doe");

effect(() => {
    console.log(firstName.value + " " + lastName.value);
});
// Logs: "John Doe"

batch(() => {
    firstName.value = "Jane";
    lastName.value = "Smith";
});
// Logs: "Jane Smith" (only once, not twice)
```

**Properties**:
- Defers all effect executions until block completes
- Prevents redundant work
- Maintains consistency (effects see all updates together)

---

## 3. JavaScript Runtime Implementation

### 3.1 Runtime File Structure

Create `runtime/reactivity.js` with:

```javascript
// Global tracking context
let currentObserver = null;
let batchDepth = 0;
let batchedEffects = new Set();

// Signal class
class Signal {
    constructor(initialValue) {
        this._value = initialValue;
        this._subscribers = new Set();
    }

    get value() {
        // Track dependency
        if (currentObserver) {
            this._subscribers.add(currentObserver);
            currentObserver._dependencies.add(this);
        }
        return this._value;
    }

    set value(newValue) {
        if (this._value === newValue) return;  // No-op if unchanged
        this._value = newValue;
        this._notify();
    }

    _notify() {
        for (const subscriber of this._subscribers) {
            if (batchDepth > 0) {
                batchedEffects.add(subscriber);
            } else {
                subscriber._execute();
            }
        }
    }
}

// Computed class (read-only signal)
class Computed {
    constructor(computation) {
        this._computation = computation;
        this._value = undefined;
        this._dirty = true;
        this._subscribers = new Set();
        this._dependencies = new Set();
    }

    get value() {
        if (this._dirty) {
            this._recompute();
        }

        // Track this computed as dependency
        if (currentObserver) {
            this._subscribers.add(currentObserver);
            currentObserver._dependencies.add(this);
        }

        return this._value;
    }

    _recompute() {
        // Clear old dependencies
        for (const dep of this._dependencies) {
            dep._subscribers.delete(this);
        }
        this._dependencies.clear();

        // Run computation and track new dependencies
        const prevObserver = currentObserver;
        currentObserver = this;
        this._value = this._computation();
        currentObserver = prevObserver;

        this._dirty = false;
    }

    _execute() {
        this._dirty = true;
        this._notify();
    }

    _notify() {
        for (const subscriber of this._subscribers) {
            if (batchDepth > 0) {
                batchedEffects.add(subscriber);
            } else {
                subscriber._execute();
            }
        }
    }
}

// Effect class
class Effect {
    constructor(fn) {
        this._fn = fn;
        this._dependencies = new Set();
        this._execute();
    }

    _execute() {
        // Clear old dependencies
        for (const dep of this._dependencies) {
            dep._subscribers.delete(this);
        }
        this._dependencies.clear();

        // Run effect and track new dependencies
        const prevObserver = currentObserver;
        currentObserver = this;
        this._fn();
        currentObserver = prevObserver;
    }

    dispose() {
        for (const dep of this._dependencies) {
            dep._subscribers.delete(this);
        }
        this._dependencies.clear();
    }
}

// Batch function
function batch(fn) {
    batchDepth++;
    try {
        fn();
    } finally {
        batchDepth--;
        if (batchDepth === 0) {
            const effects = Array.from(batchedEffects);
            batchedEffects.clear();
            for (const effect of effects) {
                effect._execute();
            }
        }
    }
}

// Public API
function signal(initialValue) {
    return new Signal(initialValue);
}

function computed(computation) {
    return new Computed(computation);
}

function effect(fn) {
    return new Effect(fn);
}

// Export for Jounce runtime
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { signal, computed, effect, batch };
}
```

### 3.2 Integration with Jounce Runtime

The compiler will inject `runtime/reactivity.js` into generated code when reactive primitives are detected.

---

## 4. Jounce Language Integration

### 4.1 AST Nodes

Add new AST node types in `src/parser.rs`:

```rust
pub enum Expr {
    // ... existing variants ...

    Signal {
        type_annotation: Option<Type>,
        initial_value: Box<Expr>,
    },

    Computed {
        type_annotation: Option<Type>,
        computation: Box<Expr>,  // Must be a lambda/closure
    },

    Effect {
        callback: Box<Expr>,  // Must be a lambda/closure
    },

    Batch {
        body: Box<Expr>,  // Must be a lambda/closure
    },
}
```

### 4.2 Parsing

Recognize reactivity functions as special built-ins:

```rust
// In parse_primary_expr()
match self.current_token {
    Token::Identifier(ref name) if name == "signal" => {
        self.advance();
        // Parse signal<T>(initial_value)
        let type_annotation = self.parse_optional_type_params()?;
        self.expect(Token::LeftParen)?;
        let initial_value = self.parse_expr()?;
        self.expect(Token::RightParen)?;

        Expr::Signal {
            type_annotation,
            initial_value: Box::new(initial_value),
        }
    }

    // Similar for computed, effect, batch...
}
```

### 4.3 Type Checking

Add reactive types to type system:

```rust
pub enum Type {
    // ... existing types ...

    Signal(Box<Type>),      // signal<T>
    Computed(Box<Type>),    // computed<T> (read-only)
}

impl TypeChecker {
    fn check_signal(&mut self, initial_value: &Expr) -> Result<Type> {
        let value_type = self.infer_type(initial_value)?;
        Ok(Type::Signal(Box::new(value_type)))
    }

    fn check_computed(&mut self, computation: &Expr) -> Result<Type> {
        // computation must be a function that returns T
        let fn_type = self.infer_type(computation)?;

        match fn_type {
            Type::Function { params, return_type } => {
                if !params.is_empty() {
                    return Err("computed() takes zero-argument function".into());
                }
                Ok(Type::Computed(return_type))
            }
            _ => Err("computed() requires a function".into())
        }
    }

    fn check_property_access(&mut self, obj: &Expr, property: &str) -> Result<Type> {
        let obj_type = self.infer_type(obj)?;

        match obj_type {
            Type::Signal(inner) | Type::Computed(inner) => {
                if property == "value" {
                    Ok(*inner)
                } else {
                    Err(format!("Signal/Computed only has .value property, not .{}", property).into())
                }
            }
            // ... existing property checks ...
        }
    }

    fn check_assignment(&mut self, target: &Expr, value: &Expr) -> Result<()> {
        // Allow: signal.value = x
        // Disallow: computed.value = x

        if let Expr::PropertyAccess { object, property } = target {
            let obj_type = self.infer_type(object)?;

            if let Type::Computed(_) = obj_type {
                if property == "value" {
                    return Err("Cannot assign to computed.value (read-only)".into());
                }
            }
        }

        // ... existing assignment checks ...
    }
}
```

### 4.4 Code Generation

Generate JavaScript that uses the runtime:

```rust
impl JsEmitter {
    fn emit_signal(&mut self, type_annotation: &Option<Type>, initial_value: &Expr) -> String {
        let value_js = self.emit_expr(initial_value);
        format!("signal({})", value_js)
    }

    fn emit_computed(&mut self, computation: &Expr) -> String {
        let fn_js = self.emit_expr(computation);
        format!("computed({})", fn_js)
    }

    fn emit_effect(&mut self, callback: &Expr) -> String {
        let fn_js = self.emit_expr(callback);
        format!("effect({})", fn_js)
    }

    fn emit_batch(&mut self, body: &Expr) -> String {
        let fn_js = self.emit_expr(body);
        format!("batch({})", fn_js)
    }

    fn emit_property_access(&mut self, object: &Expr, property: &str) -> String {
        let obj_js = self.emit_expr(object);

        // Special handling for .value on signals/computed
        let obj_type = self.type_checker.infer_type(object).ok();
        if let Some(Type::Signal(_)) | Some(Type::Computed(_)) = obj_type {
            if property == "value" {
                return format!("{}.value", obj_js);
            }
        }

        // ... existing property access ...
    }
}
```

---

## 5. Type System Integration

### 5.1 Type Inference

```jounce
// Explicit type annotation
let count: Signal<int> = signal<int>(0);

// Inferred from initial value
let count = signal(0);  // inferred as Signal<int>

// Computed type inference
let doubled = computed(() => count.value * 2);  // inferred as Computed<int>
```

### 5.2 Type Constraints

- `signal<T>(value)` - `value` must be assignable to `T`
- `computed<T>(fn)` - `fn` must be `() -> T`
- `effect(fn)` - `fn` must be `() -> void`
- `batch(fn)` - `fn` must be `() -> void`

### 5.3 Generic Signals

```jounce
struct User {
    name: string,
    age: int,
}

let user = signal<User>({ name: "Alice", age: 30 });
let userName = computed<string>(() => user.value.name);
```

---

## 6. Memory Management

### 6.1 Automatic Cleanup

Effects should be disposed when no longer needed:

```jounce
// Future enhancement: return disposer function
let dispose = effect(() => {
    console.log(count.value);
});

// Later...
dispose();  // Stop tracking
```

### 6.2 Dependency Graph

Maintain a directed acyclic graph (DAG) of dependencies:

```
Signal<int>
    ↓
Computed<int> (doubled)
    ↓
Effect (console.log)
```

When signal changes:
1. Mark computed as dirty
2. Queue effect for execution
3. Execute queued effects (in batch if active)

### 6.3 Circular Dependency Detection

Detect cycles during effect execution:

```javascript
class Effect {
    _execute() {
        if (this._running) {
            throw new Error("Circular dependency detected in effect");
        }
        this._running = true;
        try {
            // ... execute ...
        } finally {
            this._running = false;
        }
    }
}
```

---

## 7. Performance Considerations

### 7.1 Lazy Evaluation

Computed values only recompute when:
1. They are marked dirty (dependency changed)
2. AND `.value` is accessed

### 7.2 Batching

Batch prevents redundant work:

```jounce
// Without batch: effect runs 3 times
count.value = 1;
count.value = 2;
count.value = 3;

// With batch: effect runs once
batch(() => {
    count.value = 1;
    count.value = 2;
    count.value = 3;
});
```

### 7.3 Equality Checks

Signals skip notification if value unchanged:

```javascript
set value(newValue) {
    if (this._value === newValue) return;  // Skip notification
    this._value = newValue;
    this._notify();
}
```

### 7.4 Set vs Array for Subscribers

Use `Set` instead of `Array` for O(1) add/remove:

```javascript
this._subscribers = new Set();  // Fast add/remove
```

---

## 8. Example Usage Patterns

### 8.1 Counter Example

```jounce
let count = signal<int>(0);
let doubled = computed<int>(() => count.value * 2);

effect(() => {
    console.log("Count: " + count.value.to_string());
    console.log("Doubled: " + doubled.value.to_string());
});

count.value = 5;
// Logs:
// Count: 5
// Doubled: 10
```

### 8.2 Form Validation

```jounce
let email = signal<string>("");
let isValidEmail = computed<bool>(() => {
    let value = email.value;
    return value.contains("@") && value.contains(".");
});

effect(() => {
    if (isValidEmail.value) {
        console.log("Email is valid!");
    } else {
        console.log("Email is invalid");
    }
});

email.value = "test@example.com";  // Logs: "Email is valid!"
```

### 8.3 Todo List

```jounce
struct Todo {
    id: int,
    text: string,
    done: bool,
}

let todos = signal<Todo[]>([]);
let completedCount = computed<int>(() => {
    let count = 0;
    for (let todo in todos.value) {
        if (todo.done) {
            count = count + 1;
        }
    }
    return count;
});

effect(() => {
    console.log("Completed: " + completedCount.value.to_string());
});

todos.value = [
    { id: 1, text: "Learn Jounce", done: true },
    { id: 2, text: "Build app", done: false },
];
// Logs: "Completed: 1"
```

### 8.4 Batched Updates

```jounce
let firstName = signal<string>("John");
let lastName = signal<string>("Doe");
let fullName = computed<string>(() => firstName.value + " " + lastName.value);

effect(() => {
    console.log("Name: " + fullName.value);
});
// Logs: "Name: John Doe"

batch(() => {
    firstName.value = "Jane";
    lastName.value = "Smith";
});
// Logs: "Name: Jane Smith" (only once)
```

---

## 9. Edge Cases & Error Handling

### 9.1 Circular Dependencies

```jounce
let a = signal<int>(0);
let b = computed<int>(() => a.value + 1);

// This would create a cycle:
// a.value = b.value;  // ERROR: Cannot assign computed to signal that it depends on

// Runtime should detect and throw:
// "Circular dependency detected in effect"
```

### 9.2 Type Mismatches

```jounce
let count = signal<int>(0);
count.value = "hello";  // TYPE ERROR: Cannot assign string to Signal<int>

let doubled = computed<string>(() => count.value * 2);
// TYPE ERROR: Cannot assign int to Computed<string>
```

### 9.3 Read-Only Computed

```jounce
let count = signal<int>(0);
let doubled = computed<int>(() => count.value * 2);

doubled.value = 10;  // TYPE ERROR: Cannot assign to read-only computed.value
```

### 9.4 Invalid Function Types

```jounce
computed(42);  // TYPE ERROR: computed() requires a function, got int

effect((x) => console.log(x));
// TYPE ERROR: effect() requires zero-argument function
```

---

## 10. Testing Strategy

### 10.1 Unit Tests

Test each component in isolation:

- **Signal tests**: Create, read, write, subscribe, notify
- **Computed tests**: Lazy eval, memoization, dependency tracking
- **Effect tests**: Auto-run, re-run on change, cleanup
- **Batch tests**: Defer updates, run once at end

### 10.2 Integration Tests

Test interactions:

- Signal → Computed → Effect chain
- Multiple signals feeding one computed
- Nested computed values
- Batch with multiple signals

### 10.3 Edge Case Tests

- Circular dependencies (should throw)
- Type errors (should fail at compile time)
- Read-only violations (should fail at compile time)
- Memory leaks (effects should clean up)

### 10.4 Performance Tests

- Large dependency graphs (1000+ signals)
- Deep computed chains (100+ levels)
- Batch efficiency (compare with/without)

---

## 11. Migration Path

### 11.1 Backward Compatibility

Reactivity is opt-in. Existing code continues to work:

```jounce
// Old code (still works)
let count = 0;
count = count + 1;

// New reactive code
let count = signal<int>(0);
count.value = count.value + 1;
```

### 11.2 Incremental Adoption

Users can adopt reactivity incrementally:

1. Start with signals for state
2. Add computed for derived values
3. Add effects for side effects
4. Optimize with batch

### 11.3 Migration Guide

Provide clear before/after examples:

```jounce
// Before: Manual updates
let count = 0;
let doubled = 0;

fn updateDoubled() {
    doubled = count * 2;
}

count = 5;
updateDoubled();  // Manual call

// After: Automatic updates
let count = signal<int>(0);
let doubled = computed<int>(() => count.value * 2);

count.value = 5;  // doubled auto-updates
```

---

## 12. Implementation Timeline

### Phase 1: Runtime (Week 1, Days 1-3)
- ✅ Research complete
- [ ] Write `runtime/reactivity.js`
- [ ] Test runtime in isolation
- [ ] Handle edge cases

### Phase 2: Language Integration (Week 2)
- [ ] Update AST nodes
- [ ] Add parsing for signal/computed/effect/batch
- [ ] Implement type checking
- [ ] Add code generation

### Phase 3: Testing & Examples (Week 3)
- [ ] Write 20+ tests
- [ ] Build counter example
- [ ] Build todo example
- [ ] Build form validation example
- [ ] Write documentation

---

## 13. Future Enhancements (Post v0.4.0)

### 13.1 Reactive Collections

```jounce
let items = signalArray<string>(["a", "b", "c"]);
items.push("d");  // Reactive
```

### 13.2 Effect Cleanup

```jounce
let dispose = effect(() => {
    // Subscribe to event
    return () => {
        // Cleanup when effect re-runs
    };
});
```

### 13.3 Untrack

```jounce
untrack(() => {
    // Read signals without tracking dependencies
    let value = count.value;  // Not tracked
});
```

### 13.4 On Mount/Cleanup

```jounce
onMount(() => {
    // Run once when component mounts
});

onCleanup(() => {
    // Run when component unmounts
});
```

---

## 14. Success Criteria

✅ Phase 12 is complete when:

- [ ] `runtime/reactivity.js` implemented and tested
- [ ] Parser recognizes signal/computed/effect/batch
- [ ] Type checker validates reactive types
- [ ] Code generator emits correct JavaScript
- [ ] 20+ tests passing
- [ ] 3 example apps working
- [ ] Documentation written
- [ ] All existing 638 tests still passing

---

## 15. References

- [Solid.js Reactivity Documentation](https://docs.solidjs.com/concepts/signals)
- [Understanding Fine-Grained Reactivity](https://hackmd.io/@solid-docs/SyKMFjMDn)
- [Build Signals from Scratch - freeCodeCamp](https://www.freecodecamp.org/news/learn-javascript-reactivity-build-signals-from-scratch/)
- [A Hands-on Introduction to Fine-Grained Reactivity - Ryan Carniato](https://dev.to/ryansolid/a-hands-on-introduction-to-fine-grained-reactivity-3ndf)

---

**Last Updated**: October 24, 2025
**Status**: Design Complete - Ready for Implementation
**Next Step**: Task 3 - Implement `runtime/reactivity.js`
