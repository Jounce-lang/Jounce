# Counter App - Jounce Reactivity Example

A simple counter application demonstrating Jounce's reactive primitives: signals, computed values, effects, and batching.

## Features Demonstrated

- **Signals**: Reactive state with `signal(initialValue)`
- **Computed Values**: Auto-updating derived state with `computed(() => expression)`
- **Effects**: Side effects that re-run when dependencies change with `effect(() => { })`
- **Batching**: Optimize multiple updates with `batch(() => { })`

## Code Overview

```jounce
// Create reactive state
let count = signal(0);

// Create computed values
let doubled = computed(() => count.value * 2);
let isEven = computed(() => count.value % 2 == 0);

// Create effect that logs changes
effect(() => {
    console.log("Count: " + count.value.to_string());
});

// Update state
count.value = 5;  // Effect runs

// Batch multiple updates
batch(() => {
    count.value = 10;
    count.value = 20;  // Effect runs only once after batch
});
```

## How to Run

```bash
# From the counter-app directory
jnc compile main.jnc

# Run the generated code
cd dist && node server.js
```

## Expected Output

```
=== Jounce Counter App ===

Count changed: 0
  - Doubled: 0
  - Tripled: 0
  - Count is EVEN

Incrementing count to 1...
Count changed: 1
  - Doubled: 2
  - Tripled: 3
  - Count is ODD

Incrementing count to 2...
Count changed: 2
  - Doubled: 4
  - Tripled: 6
  - Count is EVEN

Batching updates (5 → 10)...
Count changed: 10
  - Doubled: 20
  - Tripled: 30
  - Count is EVEN

Setting count to 42...
Count changed: 42
  - Doubled: 84
  - Tripled: 126
  - Count is EVEN

=== Counter App Complete ===
```

## Key Concepts

### Signals
Signals are the foundation of reactivity. They hold mutable state and notify subscribers when changed:
```jounce
let count = signal(0);
count.value = 5;  // Update the signal
```

### Computed Values
Computed values automatically recalculate when their dependencies change:
```jounce
let doubled = computed(() => count.value * 2);
// doubled.value automatically updates when count changes
```

### Effects
Effects run side effects (like logging or DOM updates) when their dependencies change:
```jounce
effect(() => {
    console.log(count.value);  // Runs whenever count changes
});
```

### Batching
Batching defers effect execution until all updates are complete, preventing redundant work:
```jounce
batch(() => {
    count.value = 1;
    count.value = 2;
    count.value = 3;
});
// Effects run only once with final value (3)
```

## Learn More

- [Jounce Reactivity System Documentation](../../docs/design/REACTIVITY_SYSTEM.md)
- [API Reference](../../docs/api/REACTIVITY.md)
- [More Examples](../)
