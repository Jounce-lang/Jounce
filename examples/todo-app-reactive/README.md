# Reactive Todo App - Jounce Reactivity Example

A todo list application demonstrating reactive list management, computed filtering, and automatic persistence with Jounce's reactivity system.

## Features Demonstrated

- **Reactive Lists**: Managing collections with signals
- **Computed Filtering**: Auto-updating filtered views
- **Statistics**: Computed counts and aggregations
- **Effects for Persistence**: Auto-sync to localStorage
- **Declarative UI Updates**: UI stays in sync with state automatically

## Code Overview

```jounce
// Reactive todo list
let todos = signal([]);
let filter = signal("all");

// Computed filtered view
let filteredTodos = computed(() => {
    if filter.value == "active" {
        todos.value.filter(|t| !t.completed)
    } else if filter.value == "completed" {
        todos.value.filter(|t| t.completed)
    } else {
        todos.value
    }
});

// Computed statistics
let activeCount = computed(() =>
    todos.value.filter(|t| !t.completed).length
);

// Auto-persist to localStorage
effect(() => {
    localStorage.setItem("todos", JSON.stringify(todos.value));
});

// Add todo - everything updates automatically
todos.value.push({ text: "New task", completed: false });
```

## Key Patterns

### Reactive Collections
```jounce
let items = signal([]);
items.value.push(newItem);      // Triggers updates
items.value = items.value.filter(/* ... */);  // Replace array
```

### Computed Filtering
```jounce
let filtered = computed(() => {
    items.value.filter(|item| item.matches_filter())
});
// filtered.value updates when items or filter changes
```

### Aggregate Computations
```jounce
let total = computed(() => items.value.length);
let sum = computed(() => items.value.reduce(/* sum logic */));
```

### Automatic Persistence
```jounce
effect(() => {
    localStorage.setItem("key", JSON.stringify(data.value));
});
// Runs whenever data changes
```

## How It Works

1. **State Management**: All todos stored in reactive `signal()`
2. **Filtering**: `computed()` creates filtered views automatically
3. **Statistics**: Counts computed from filtered data
4. **Persistence**: `effect()` syncs to localStorage on changes
5. **UI Updates**: Effects update DOM when signals change

## Benefits of Reactivity

- **No Manual Updates**: UI stays in sync automatically
- **Efficient**: Only affected parts re-render
- **Declarative**: Describe relationships, not procedures
- **Composable**: Computed values can depend on other computed values
- **Predictable**: Changes flow in one direction

## Learn More

- [Jounce Reactivity System](../../docs/design/REACTIVITY_SYSTEM.md)
- [Counter Example](../counter-app/)
- [Form Validation Example](../form-validation/)
