# Jounce v0.4.0 Reactive - API Reference

**Version**: v0.4.0 "Reactive"
**Module**: Core Reactivity
**Status**: Stable

Complete API reference for Jounce's reactivity system.

---

## Table of Contents

1. [signal()](#signal)
2. [computed()](#computed)
3. [effect()](#effect)
4. [batch()](#batch)
5. [Signal Type](#signal-type)
6. [Computed Type](#computed-type)
7. [Type Signatures](#type-signatures)
8. [Runtime Behavior](#runtime-behavior)

---

## signal()

Creates a reactive signal that holds mutable state.

### Signature

```jounce
fn signal<T>(initialValue: T) -> Signal<T>
```

### Parameters

- `initialValue`: The initial value of the signal (any type)

### Returns

A `Signal<T>` object with a `.value` property for reading and writing.

### Examples

```jounce
// Number signal
let count = signal(0);
let count = signal<i32>(0);  // With type annotation

// String signal
let name = signal("Alice");
let name = signal<string>("Alice");

// Boolean signal
let isActive = signal(true);
let isActive = signal<bool>(false);

// Array signal
let items = signal([]);
let items = signal<array>([1, 2, 3]);

// Object signal
let user = signal({ name: "Alice", age: 30 });
```

### Usage

```jounce
let count = signal(0);

// Read value
console.log(count.value);  // 0

// Write value
count.value = 5;
console.log(count.value);  // 5

// Update based on current value
count.value = count.value + 1;
```

### Notes

- Signals are the only mutable reactive primitive
- Reading `.value` automatically tracks the dependency
- Writing to `.value` notifies all subscribers
- Signals work with any type

---

## computed()

Creates a computed value that automatically updates when dependencies change.

### Signature

```jounce
fn computed<T>(computation: () => T) -> Computed<T>
```

### Parameters

- `computation`: A function that returns the computed value
  - Must be pure (no side effects)
  - Dependencies are tracked automatically
  - Should not modify signals

### Returns

A `Computed<T>` object with a read-only `.value` property.

### Examples

```jounce
// Simple computation
let count = signal(5);
let doubled = computed(() => count.value * 2);

// Multiple dependencies
let a = signal(10);
let b = signal(20);
let sum = computed(() => a.value + b.value);

// String concatenation
let first = signal("John");
let last = signal("Doe");
let full = computed(() => first.value + " " + last.value);

// Array filtering
let items = signal([1, 2, 3, 4, 5]);
let evens = computed(() => items.value.filter(|x| x % 2 == 0));

// Conditional computation
let enabled = signal(true);
let value = signal(10);
let result = computed(() => {
    if enabled.value {
        value.value * 2
    } else {
        0
    }
});
```

### Chaining

```jounce
let x = signal(2);
let doubled = computed(() => x.value * 2);
let quadrupled = computed(() => doubled.value * 2);

// All update automatically
x.value = 5;
// doubled.value = 10
// quadrupled.value = 20
```

### Notes

- Computed values are lazy (only evaluated when accessed)
- Results are memoized (cached until dependencies change)
- The computation function must be pure
- Reading `.value` returns the current computed value
- Writing to `.value` is not allowed (read-only)

---

## effect()

Creates an effect that runs side effects when dependencies change.

### Signature

```jounce
fn effect(callback: () => void) -> Effect
```

### Parameters

- `callback`: A function to execute
  - Runs immediately on creation
  - Re-runs when dependencies change
  - Can access signals and computed values
  - Should contain side effects (logging, DOM updates, etc.)

### Returns

An `Effect` object (currently opaque, used internally for tracking).

### Examples

```jounce
// Simple logging
let count = signal(0);
effect(() => {
    console.log("Count: " + count.value.to_string());
});
// Logs immediately: "Count: 0"

count.value = 5;
// Logs: "Count: 5"

// Multiple dependencies
let x = signal(0);
let y = signal(0);
effect(() => {
    console.log("x: " + x.value.to_string() + ", y: " + y.value.to_string());
});

// Conditional tracking
let enabled = signal(true);
let data = signal("");
effect(() => {
    if enabled.value {
        console.log(data.value);  // Only tracked when enabled is true
    }
});

// DOM updates
let message = signal("Hello");
effect(() => {
    updateDOM(message.value);
});

// API calls
let query = signal("");
effect(() => {
    if query.value.length > 0 {
        fetchResults(query.value);
    }
});
```

### Execution

Effects run:
1. **Immediately** when created
2. **Automatically** when any tracked dependency changes
3. **Synchronously** (unless in a batch)
4. **Once per change** (if multiple dependencies change)

### Notes

- Effects run immediately upon creation
- Effects automatically track all signals/computed values accessed
- Effects should contain side effects (not pure computations)
- Avoid infinite loops (effect updating its own dependencies)
- Effects outside batches run synchronously

---

## batch()

Groups multiple signal updates to run effects only once.

### Signature

```jounce
fn batch(updates: () => void) -> void
```

### Parameters

- `updates`: A function containing signal updates
  - All signal updates are queued
  - Effects run only after the batch completes
  - Can be nested

### Returns

Nothing (void).

### Examples

```jounce
// Basic batching
let x = signal(0);
let y = signal(0);

effect(() => {
    console.log("x: " + x.value.to_string() + ", y: " + y.value.to_string());
});

// Without batch: effect runs twice
x.value = 5;
y.value = 10;

// With batch: effect runs once
batch(() => {
    x.value = 15;
    y.value = 20;
});

// Batch with multiple signals
batch(() => {
    name.value = "Alice";
    age.value = 30;
    email.value = "alice@example.com";
});

// Nested batching
batch(() => {
    x.value = 1;
    batch(() => {
        y.value = 2;
        z.value = 3;
    });
    w.value = 4;
});
// Effects run once after all updates
```

### Use Cases

```jounce
// Form submission
batch(() => {
    formData.name.value = input.name;
    formData.email.value = input.email;
    formData.age.value = input.age;
});

// Bulk operations
batch(() => {
    for item in newItems {
        items.value.push(item);
    }
});

// Animation frames
batch(() => {
    x.value = newX;
    y.value = newY;
    velocity.value = calculateVelocity();
});
```

### Notes

- Batching improves performance by reducing effect executions
- Use when updating multiple related signals
- Effects still see final values (not intermediate states)
- Batches can be nested
- All effects run after the outermost batch completes

---

## Signal Type

The type returned by `signal()`.

### Properties

| Property | Type | Access | Description |
|----------|------|--------|-------------|
| `.value` | `T` | Read/Write | The current value of the signal |

### Methods

Signals currently have no methods. All operations are done through `.value`.

### Behavior

```jounce
let s = signal(5);

// Reading tracks dependencies
let doubled = computed(() => s.value * 2);

// Writing notifies subscribers
s.value = 10;  // All computed values and effects update

// Can store any type
let numbers = signal<array>([1, 2, 3]);
let user = signal<User>({ name: "Alice" });
```

---

## Computed Type

The type returned by `computed()`.

### Properties

| Property | Type | Access | Description |
|----------|------|--------|-------------|
| `.value` | `T` | Read-only | The current computed value |

### Behavior

```jounce
let c = computed(() => expensive_calculation());

// Lazy evaluation
// Not computed until first access
let result = c.value;

// Memoization
// Cached until dependencies change
let again = c.value;  // Same result, no recomputation

// Auto-update
signal_dependency.value = new_value;
let updated = c.value;  // Recomputed automatically
```

---

## Type Signatures

### Generic Types

All reactive primitives support generic types:

```jounce
signal<T>(initialValue: T) -> Signal<T>
computed<T>(computation: () => T) -> Computed<T>
effect(callback: () => void) -> Effect
batch(updates: () => void) -> void
```

### Common Types

```jounce
// Primitives
signal<i32>(0)
signal<f64>(3.14)
signal<string>("text")
signal<bool>(true)

// Collections
signal<array>([1, 2, 3])
signal<Vec<T>>(vec![])

// Custom types
signal<User>({ name: "Alice", age: 30 })
signal<Result<T, E>>(Ok(value))
signal<Option<T>>(Some(value))
```

---

## Runtime Behavior

### Dependency Tracking

Jounce automatically tracks dependencies using a global context:

```jounce
let a = signal(1);
let b = signal(2);
let c = computed(() => a.value + b.value);

// When accessing c.value:
// 1. Sets current observer to c
// 2. Runs computation
// 3. Tracks a and b as dependencies
// 4. Returns result
```

### Update Propagation

When a signal changes:

```jounce
signal.value = newValue;

// 1. Signal updates internal value
// 2. Collects all subscribers (computed + effects)
// 3. If not in batch: notifies subscribers immediately
// 4. If in batch: queues subscribers for later
// 5. Each subscriber updates/re-runs
```

### Execution Order

```jounce
let a = signal(1);
let b = computed(() => a.value * 2);
let c = computed(() => b.value + 1);

effect(() => console.log(c.value));

a.value = 5;

// Execution order:
// 1. a updates to 5
// 2. b recalculates (lazily, when accessed)
// 3. c recalculates (lazily, when accessed)
// 4. Effect runs, logs 11
```

### Batching Behavior

```jounce
batch(() => {
    a.value = 1;  // Queued
    b.value = 2;  // Queued
    c.value = 3;  // Queued
});
// All effects run once here
```

---

## Error Handling

### Invalid Operations

```jounce
// ❌ Cannot write to computed
let c = computed(() => 5);
c.value = 10;  // Compile error

// ❌ Don't create infinite loops
effect(() => {
    s.value = s.value + 1;  // Runtime: infinite loop
});

// ✅ Conditional updates are safe
effect(() => {
    if someCondition {
        s.value = newValue;
    }
});
```

### Type Safety

```jounce
let count = signal<i32>(0);

count.value = 5;     // ✅ OK
count.value = "hi";  // ❌ Type error
```

---

## Performance Characteristics

### Time Complexity

| Operation | Complexity |
|-----------|------------|
| Read signal | O(1) |
| Write signal | O(n) where n = number of subscribers |
| Read computed | O(1) if cached, O(m) if dirty (m = computation time) |
| Create effect | O(1) |
| Batch updates | O(k) where k = number of effects to run |

### Memory

- Each signal: ~48 bytes + value size
- Each computed: ~64 bytes + cached value
- Each effect: ~32 bytes + closure size
- Dependency tracking: O(edges in dependency graph)

### Optimization Tips

1. **Use Computed for Expensive Operations**
   - Caches results until dependencies change

2. **Batch Related Updates**
   - Reduces effect executions

3. **Minimize Dependencies**
   - Only access signals you need

4. **Avoid Deep Nesting**
   - Keep computed functions simple

---

## Version History

### v0.4.0 "Reactive" (October 2025)

**Initial Release**:
- ✅ `signal<T>(value)` - Mutable reactive state
- ✅ `computed<T>(() => expr)` - Derived state
- ✅ `effect(() => {})` - Side effects
- ✅ `batch(() => {})` - Update batching
- ✅ Automatic dependency tracking
- ✅ 29/29 runtime tests passing
- ✅ 22/22 integration tests passing

**See Also**:
- [User Guide](../guides/REACTIVITY_USER_GUIDE.md)
- [Migration Guide](../guides/REACTIVITY_MIGRATION.md)
- [Design Specification](../design/REACTIVITY_SYSTEM.md)
