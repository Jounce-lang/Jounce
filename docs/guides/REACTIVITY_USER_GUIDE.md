# Jounce v0.4.0 Reactive - User Guide

**Version**: v0.4.0 "Reactive"
**Release Date**: October 2025
**Status**: Stable

Welcome to Jounce v0.4.0, featuring a complete fine-grained reactivity system inspired by Solid.js. This guide will teach you how to build reactive applications with automatic state management, computed values, and effects.

---

## Table of Contents

1. [Introduction to Reactivity](#introduction-to-reactivity)
2. [Core Concepts](#core-concepts)
3. [Signals](#signals)
4. [Computed Values](#computed-values)
5. [Effects](#effects)
6. [Batching](#batching)
7. [Patterns and Best Practices](#patterns-and-best-practices)
8. [Common Use Cases](#common-use-cases)
9. [Performance Tips](#performance-tips)
10. [Debugging](#debugging)

---

## Introduction to Reactivity

Reactivity in Jounce v0.4.0 allows you to build dynamic applications where the UI automatically stays in sync with your data. When data changes, all dependent computations and UI updates happen automatically.

### Why Reactivity?

**Without Reactivity:**
```jounce
let count = 0;
let doubled = count * 2;

count = 5;
// doubled is still 0! Must manually update
doubled = count * 2;
```

**With Reactivity (v0.4.0):**
```jounce
let count = signal(0);
let doubled = computed(() => count.value * 2);

count.value = 5;
// doubled.value is automatically 10!
```

### Key Benefits

- **Automatic Updates**: No manual tracking of dependencies
- **Fine-Grained**: Only affected parts update, not entire components
- **Declarative**: Describe relationships, not procedures
- **Composable**: Build complex logic from simple reactive primitives
- **Efficient**: Minimal re-computation and updates

---

## Core Concepts

Jounce v0.4.0 reactivity is built on four primitives:

1. **Signals**: Mutable reactive state
2. **Computed**: Derived state that auto-updates
3. **Effects**: Side effects that re-run when dependencies change
4. **Batching**: Group multiple updates for efficiency

### The Reactivity Graph

Jounce automatically tracks dependencies:

```
signal(count) ──┬──> computed(doubled)
                │
                └──> effect(log count)
```

When `count` changes, both `doubled` and the effect automatically update.

---

## Signals

Signals are the foundation of reactivity. They hold mutable state and notify subscribers when changed.

### Creating Signals

```jounce
// Basic signal
let count = signal(0);

// Typed signal
let name = signal<string>("Alice");
let items = signal<array>([]);
```

### Reading Signals

Access the current value with `.value`:

```jounce
let count = signal(5);
console.log(count.value);  // Output: 5
```

### Updating Signals

Assign to `.value` to update:

```jounce
count.value = 10;
count.value = count.value + 1;
```

### Signal Examples

```jounce
// Counter
let clicks = signal(0);

// Toggle
let isOpen = signal(false);
isOpen.value = !isOpen.value;

// Array manipulation
let todos = signal([]);
todos.value.push("New task");

// Object updates
let user = signal({ name: "Alice", age: 30 });
user.value = { name: "Bob", age: 25 };
```

---

## Computed Values

Computed values are derived from other reactive values and automatically update when dependencies change.

### Creating Computed Values

```jounce
let count = signal(5);
let doubled = computed(() => count.value * 2);

console.log(doubled.value);  // Output: 10
count.value = 10;
console.log(doubled.value);  // Output: 20 (auto-updated!)
```

### Typed Computed

```jounce
let firstName = signal("John");
let lastName = signal("Doe");
let fullName = computed<string>(() =>
    firstName.value + " " + lastName.value
);
```

### Computed with Multiple Dependencies

```jounce
let a = signal(10);
let b = signal(20);
let c = signal(5);

let sum = computed(() => a.value + b.value + c.value);
// sum updates when any of a, b, or c changes
```

### Chained Computed Values

```jounce
let count = signal(2);
let doubled = computed(() => count.value * 2);
let quadrupled = computed(() => doubled.value * 2);

console.log(quadrupled.value);  // 8
count.value = 3;
console.log(quadrupled.value);  // 12
```

### Computed Best Practices

✅ **Do**: Keep computed functions pure
```jounce
let total = computed(() => items.value.length);  // Good
```

❌ **Don't**: Cause side effects in computed
```jounce
let bad = computed(() => {
    console.log("Don't do this!");  // Bad
    return value.value;
});
```

---

## Effects

Effects run side effects (like logging, DOM updates, or API calls) and automatically re-run when their dependencies change.

### Creating Effects

```jounce
let count = signal(0);

effect(() => {
    console.log("Count is: " + count.value.to_string());
});
// Logs immediately: "Count is: 0"

count.value = 5;
// Logs: "Count is: 5"
```

### Effects with Multiple Dependencies

```jounce
let firstName = signal("John");
let lastName = signal("Doe");

effect(() => {
    console.log("Full name: " + firstName.value + " " + lastName.value);
});

firstName.value = "Jane";  // Effect runs
lastName.value = "Smith";  // Effect runs again
```

### Conditional Effects

```jounce
let enabled = signal(true);
let count = signal(0);

effect(() => {
    if enabled.value {
        console.log(count.value);  // Only logs when enabled
    }
});
```

### Effects for Persistence

```jounce
let todos = signal([]);

effect(() => {
    // Auto-save to localStorage whenever todos change
    localStorage.setItem("todos", JSON.stringify(todos.value));
});
```

### Effect Best Practices

✅ **Do**: Use effects for side effects
```jounce
effect(() => {
    updateDOM(data.value);
    sendAnalytics(event.value);
});
```

✅ **Do**: Clean up if needed
```jounce
effect(() => {
    let subscription = subscribe(topic.value);
    // In real app: return cleanup function
});
```

❌ **Don't**: Create infinite loops
```jounce
let counter = signal(0);
effect(() => {
    counter.value = counter.value + 1;  // Infinite loop!
});
```

---

## Batching

Batching groups multiple updates to run effects only once, improving performance.

### Basic Batching

```jounce
let x = signal(0);
let y = signal(0);

effect(() => {
    console.log("x: " + x.value.to_string() + ", y: " + y.value.to_string());
});

// Without batching: effect runs twice
x.value = 5;
y.value = 10;

// With batching: effect runs once
batch(() => {
    x.value = 15;
    y.value = 20;
});
```

### When to Use Batching

Use `batch()` when:
- Updating multiple related signals
- Performing bulk operations
- Optimizing expensive effects
- Processing multiple events

### Batching Examples

```jounce
// Form submission
batch(() => {
    name.value = formData.name;
    email.value = formData.email;
    age.value = formData.age;
});

// Bulk array operations
batch(() => {
    for item in newItems {
        items.value.push(item);
    }
});

// Coordinated updates
batch(() => {
    x.value = mouse.x;
    y.value = mouse.y;
    velocity.value = calculateVelocity();
});
```

---

## Patterns and Best Practices

### 1. Single Source of Truth

Keep one signal as the source, derive others:

```jounce
let items = signal([]);

// Derive counts from items
let total = computed(() => items.value.length);
let completed = computed(() =>
    items.value.filter(|i| i.done).length
);
```

### 2. Computed for Filtering

```jounce
let todos = signal([]);
let filter = signal("all");

let visibleTodos = computed(() => {
    if filter.value == "active" {
        todos.value.filter(|t| !t.done)
    } else if filter.value == "completed" {
        todos.value.filter(|t| t.done)
    } else {
        todos.value
    }
});
```

### 3. Effects for I/O

```jounce
let searchQuery = signal("");

effect(() => {
    if searchQuery.value.length > 2 {
        fetchResults(searchQuery.value);
    }
});
```

### 4. Batching for Performance

```jounce
function updateBulk(updates: array) {
    batch(() => {
        for update in updates {
            applyUpdate(update);
        }
    });
}
```

### 5. Validation Patterns

```jounce
let email = signal("");
let emailValid = computed(() => {
    email.value.contains("@") && email.value.length > 3
});

effect(() => {
    if !emailValid.value && email.value.length > 0 {
        showError("Invalid email");
    } else {
        clearError();
    }
});
```

---

## Common Use Cases

### Counter

```jounce
let count = signal(0);
let doubled = computed(() => count.value * 2);

effect(() => {
    updateDisplay(count.value);
});

fn increment() {
    count.value = count.value + 1;
}
```

### Todo List

```jounce
let todos = signal([]);
let activeCount = computed(() =>
    todos.value.filter(|t| !t.done).length
);

effect(() => {
    updateBadge(activeCount.value);
});

fn addTodo(text: string) {
    todos.value.push({ text: text, done: false });
}
```

### Form with Validation

```jounce
let email = signal("");
let password = signal("");

let emailValid = computed(() => email.value.contains("@"));
let passwordValid = computed(() => password.value.length >= 8);
let formValid = computed(() => emailValid.value && passwordValid.value);

effect(() => {
    submitButton.disabled = !formValid.value;
});
```

### Search with Debouncing

```jounce
let searchQuery = signal("");
let searchResults = signal([]);

effect(() => {
    // In real app: debounce this
    if searchQuery.value.length > 0 {
        performSearch(searchQuery.value);
    }
});
```

---

## Performance Tips

### 1. Use Computed for Expensive Operations

Instead of recalculating in every effect:
```jounce
let filtered = computed(() =>
    items.value.filter(/* expensive filter */)
);

effect(() => {
    display(filtered.value);  // Only recalculates when items change
});
```

### 2. Batch Related Updates

```jounce
batch(() => {
    field1.value = data.field1;
    field2.value = data.field2;
    field3.value = data.field3;
});
```

### 3. Minimize Dependencies

Only access what you need:
```jounce
// Bad: accesses all properties
effect(() => {
    if user.value.name == "Admin" {
        // ...
    }
});

// Better: only track name
let userName = computed(() => user.value.name);
effect(() => {
    if userName.value == "Admin" {
        // ...
    }
});
```

### 4. Avoid Creating Signals in Loops

```jounce
// Bad
for i in 0..100 {
    let s = signal(i);  // Creates 100 signals
}

// Better
let items = signal([]);
for i in 0..100 {
    items.value.push(i);
}
```

---

## Debugging

### Logging Dependencies

```jounce
effect(() => {
    console.log("Dependencies:");
    console.log("  count: " + count.value.to_string());
    console.log("  name: " + name.value);
});
```

### Tracking Updates

```jounce
let counter = signal(0);

effect(() => {
    console.log("Counter updated to: " + counter.value.to_string());
});
```

### Detecting Infinite Loops

If your app freezes, check for:

```jounce
// BAD: Infinite loop
effect(() => {
    count.value = count.value + 1;  // Effect updates its own dependency
});
```

### Debugging Computed Values

```jounce
let result = computed(() => {
    let temp = expensiveCalculation(input.value);
    console.log("Computed result: " + temp.to_string());
    temp
});
```

---

## What's Next?

- **API Reference**: Complete API documentation for all reactive primitives
- **Migration Guide**: How to adopt reactivity in existing Jounce apps
- **Examples**: See `examples/` directory for complete applications
- **Advanced Patterns**: Complex reactivity patterns and architectures

---

## Version Information

**Jounce v0.4.0 "Reactive"**
Released: October 2025

**Reactivity Features**:
- ✅ Signals for mutable state
- ✅ Computed values with automatic updates
- ✅ Effects for side effects
- ✅ Batching for performance
- ✅ Automatic dependency tracking
- ✅ 29/29 runtime tests passing
- ✅ 22/22 integration tests passing

**See Also**:
- [API Reference](../api/REACTIVITY_API.md)
- [Migration Guide](REACTIVITY_MIGRATION.md)
- [Design Specification](../design/REACTIVITY_SYSTEM.md)
- [Counter Example](../../examples/counter-app/)
- [Todo Example](../../examples/todo-app-reactive/)
- [Form Validation Example](../../examples/form-validation/)
