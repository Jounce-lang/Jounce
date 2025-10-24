# Migrating to Jounce v0.4.0 Reactive

**Version**: v0.4.0 "Reactive"
**Migrating From**: v0.3.x
**Difficulty**: Easy to Moderate

This guide helps you adopt reactivity in existing Jounce v0.3.x applications.

---

## Table of Contents

1. [What's New in v0.4.0](#whats-new-in-v040)
2. [Breaking Changes](#breaking-changes)
3. [Migration Strategy](#migration-strategy)
4. [Step-by-Step Migration](#step-by-step-migration)
5. [Before and After Examples](#before-and-after-examples)
6. [Common Patterns](#common-patterns)
7. [Troubleshooting](#troubleshooting)

---

## What's New in v0.4.0

### New Features

✅ **Reactive Primitives**:
- `signal<T>(value)` - Mutable reactive state
- `computed<T>(() => expr)` - Derived values
- `effect(() => {})` - Side effects
- `batch(() => {})` - Update optimization

✅ **Automatic Dependency Tracking**:
- No manual subscription management
- No manual update propagation
- Automatic re-computation

✅ **Fine-Grained Reactivity**:
- Only affected values update
- Minimal recomputation
- Efficient effect execution

### What Stays the Same

- ✅ All existing Jounce v0.3.x code works unchanged
- ✅ No breaking changes to existing APIs
- ✅ Opt-in adoption (use reactivity where it helps)
- ✅ Full backward compatibility

---

## Breaking Changes

**None!** Jounce v0.4.0 is fully backward compatible with v0.3.x.

Reactivity is an **additive feature**. Your existing code continues to work without modifications.

---

## Migration Strategy

### Adoption Approaches

**1. Gradual Migration** (Recommended)
- Keep existing code working
- Add reactivity to new features first
- Migrate high-value components
- No rush to migrate everything

**2. Hybrid Approach**
- Mix reactive and non-reactive code
- Use reactivity where it adds value
- Keep simple code simple

**3. Full Migration**
- Convert all state to signals
- Use computed for derived values
- Replace manual updates with effects

### When to Use Reactivity

✅ **Good candidates for reactivity**:
- Complex state dependencies
- Frequent value changes
- Multiple UI elements depending on same data
- Form validation
- Real-time data updates
- Filters and search

❌ **Keep it simple without reactivity**:
- One-time values
- Static configuration
- Simple event handlers with no state dependencies
- Values that never change

---

## Step-by-Step Migration

### Step 1: Identify State

Find mutable state in your app:

**Before (v0.3.x)**:
```jounce
let count = 0;
let message = "Hello";
let items = [];
```

**After (v0.4.0)**:
```jounce
let count = signal(0);
let message = signal("Hello");
let items = signal([]);
```

### Step 2: Convert Derived Values

Find values calculated from other values:

**Before (v0.3.x)**:
```jounce
let count = 0;
let doubled = count * 2;  // Must manually update

fn updateCount(n: i32) {
    count = n;
    doubled = count * 2;  // Manual update
}
```

**After (v0.4.0)**:
```jounce
let count = signal(0);
let doubled = computed(() => count.value * 2);  // Auto-updates!

fn updateCount(n: i32) {
    count.value = n;  // doubled updates automatically
}
```

### Step 3: Replace Manual Updates

Find manual update patterns:

**Before (v0.3.x)**:
```jounce
fn updateUI(data: Data) {
    updateDisplay(data);
    updateCounter(data.count);
    updateStatus(data.status);
}

// Call manually after every change
data = newData;
updateUI(data);
```

**After (v0.4.0)**:
```jounce
let data = signal(initialData);

// Runs automatically when data changes
effect(() => {
    updateDisplay(data.value);
    updateCounter(data.value.count);
    updateStatus(data.value.status);
});

// Just update signal
data.value = newData;  // UI updates automatically
```

### Step 4: Optimize with Batching

Find places with multiple related updates:

**Before (v0.3.x)**:
```jounce
name = "Alice";
updateNameDisplay();

age = 30;
updateAgeDisplay();

email = "alice@example.com";
updateEmailDisplay();

refreshUI();  // Manual refresh
```

**After (v0.4.0)**:
```jounce
let name = signal("");
let age = signal(0);
let email = signal("");

effect(() => {
    updateDisplay(name.value, age.value, email.value);
});

// Batch for efficiency
batch(() => {
    name.value = "Alice";
    age.value = 30;
    email.value = "alice@example.com";
});
// Effect runs once
```

---

## Before and After Examples

### Example 1: Simple Counter

**Before (v0.3.x)**:
```jounce
let count = 0;

fn increment() {
    count = count + 1;
    updateDisplay(count);
}

fn decrement() {
    count = count - 1;
    updateDisplay(count);
}

fn reset() {
    count = 0;
    updateDisplay(count);
}
```

**After (v0.4.0)**:
```jounce
let count = signal(0);

// Auto-updates display
effect(() => {
    updateDisplay(count.value);
});

fn increment() {
    count.value = count.value + 1;
}

fn decrement() {
    count.value = count.value - 1;
}

fn reset() {
    count.value = 0;
}
```

### Example 2: Todo List

**Before (v0.3.x)**:
```jounce
let todos = [];
let filter = "all";

fn addTodo(text: string) {
    todos.push({ text: text, done: false });
    updateTodoList();
    updateCount();
}

fn toggleTodo(index: i32) {
    todos[index].done = !todos[index].done;
    updateTodoList();
    updateCount();
}

fn setFilter(f: string) {
    filter = f;
    updateTodoList();  // Manual filtering
}

fn updateCount() {
    let active = todos.filter(|t| !t.done).length;
    displayCount(active);
}
```

**After (v0.4.0)**:
```jounce
let todos = signal([]);
let filter = signal("all");

// Auto-computed active count
let activeCount = computed(() =>
    todos.value.filter(|t| !t.done).length
);

// Auto-filtered view
let filteredTodos = computed(() => {
    if filter.value == "active" {
        todos.value.filter(|t| !t.done)
    } else if filter.value == "completed" {
        todos.value.filter(|t| t.done)
    } else {
        todos.value
    }
});

// Auto-update display
effect(() => {
    displayTodos(filteredTodos.value);
    displayCount(activeCount.value);
});

fn addTodo(text: string) {
    todos.value.push({ text: text, done: false });
    // Everything updates automatically!
}

fn toggleTodo(index: i32) {
    todos.value[index].done = !todos.value[index].done;
}

fn setFilter(f: string) {
    filter.value = f;
}
```

### Example 3: Form Validation

**Before (v0.3.x)**:
```jounce
let email = "";
let password = "";
let confirmPassword = "";

fn validateEmail(): bool {
    email.contains("@") && email.length > 3
}

fn validatePassword(): bool {
    password.length >= 8
}

fn validateForm(): bool {
    let emailOk = validateEmail();
    let passwordOk = validatePassword();
    let match = password == confirmPassword;

    // Manual UI updates
    updateEmailError(!emailOk);
    updatePasswordError(!passwordOk);
    updateMatchError(!match);
    updateSubmitButton(emailOk && passwordOk && match);

    emailOk && passwordOk && match
}

fn onEmailChange(value: string) {
    email = value;
    validateForm();  // Manual validation
}

fn onPasswordChange(value: string) {
    password = value;
    validateForm();  // Manual validation
}
```

**After (v0.4.0)**:
```jounce
let email = signal("");
let password = signal("");
let confirmPassword = signal("");

// Auto-computed validation
let emailValid = computed(() =>
    email.value.contains("@") && email.value.length > 3
);

let passwordValid = computed(() =>
    password.value.length >= 8
);

let passwordsMatch = computed(() =>
    password.value == confirmPassword.value
);

let formValid = computed(() =>
    emailValid.value && passwordValid.value && passwordsMatch.value
);

// Auto-update UI
effect(() => {
    updateEmailError(!emailValid.value);
    updatePasswordError(!passwordValid.value);
    updateMatchError(!passwordsMatch.value);
    updateSubmitButton(formValid.value);
});

fn onEmailChange(value: string) {
    email.value = value;
    // Validation happens automatically!
}

fn onPasswordChange(value: string) {
    password.value = value;
}
```

---

## Common Patterns

### Pattern 1: Replace Global State

**Before**:
```jounce
let globalState = {
    user: null,
    theme: "light",
    language: "en"
};

fn updateTheme(theme: string) {
    globalState.theme = theme;
    refreshUI();
}
```

**After**:
```jounce
let user = signal(null);
let theme = signal("light");
let language = signal("en");

effect(() => {
    applyTheme(theme.value);
});

fn updateTheme(t: string) {
    theme.value = t;
}
```

### Pattern 2: Replace Event Listeners

**Before**:
```jounce
let data = null;

fn onDataChange() {
    updateView1(data);
    updateView2(data);
    updateView3(data);
}

fn loadData() {
    data = fetchData();
    onDataChange();  // Manual notification
}
```

**After**:
```jounce
let data = signal(null);

effect(() => updateView1(data.value));
effect(() => updateView2(data.value));
effect(() => updateView3(data.value));

fn loadData() {
    data.value = fetchData();  // Auto-notifies
}
```

### Pattern 3: Replace Polling

**Before**:
```jounce
let pollInterval = 1000;

fn startPolling() {
    setInterval(() => {
        let newData = fetchData();
        if newData != currentData {
            currentData = newData;
            updateUI();
        }
    }, pollInterval);
}
```

**After**:
```jounce
let currentData = signal(null);

effect(() => {
    updateUI(currentData.value);  // Auto-updates
});

fn startPolling() {
    setInterval(() => {
        currentData.value = fetchData();
    }, 1000);
}
```

---

## Troubleshooting

### Issue: "Too many updates"

**Problem**: Effect running constantly

```jounce
// BAD: Infinite loop
effect(() => {
    counter.value = counter.value + 1;
});
```

**Solution**: Don't update signals in effects that depend on them

```jounce
// GOOD: Update from external events
fn onClick() {
    counter.value = counter.value + 1;
}
```

### Issue: "Effects not running"

**Problem**: Forgetting to access `.value`

```jounce
// BAD: Not tracked
effect(() => {
    console.log(signal);  // Wrong!
});
```

**Solution**: Always access `.value`

```jounce
// GOOD: Properly tracked
effect(() => {
    console.log(signal.value);  // Correct!
});
```

### Issue: "Stale values in computed"

**Problem**: Not accessing dependencies

```jounce
// BAD: Dependencies not tracked
let value = someSignal.value;  // Tracked here
let result = computed(() => value * 2);  // But not here
```

**Solution**: Access signals inside computed

```jounce
// GOOD: Dependency tracked
let result = computed(() => someSignal.value * 2);
```

### Issue: "Performance problems"

**Problem**: Too many effects running

```jounce
// BAD: Effect runs for each update
x.value = 1;
y.value = 2;
z.value = 3;
```

**Solution**: Use batching

```jounce
// GOOD: Effect runs once
batch(() => {
    x.value = 1;
    y.value = 2;
    z.value = 3;
});
```

---

## Migration Checklist

### Before You Start
- [ ] Read the [User Guide](REACTIVITY_USER_GUIDE.md)
- [ ] Review [API Reference](../api/REACTIVITY_API.md)
- [ ] Check [example apps](../../examples/)
- [ ] Understand your app's state flow

### During Migration
- [ ] Identify mutable state
- [ ] Convert to signals
- [ ] Replace derived values with computed
- [ ] Replace manual updates with effects
- [ ] Add batching where needed
- [ ] Test thoroughly

### After Migration
- [ ] Remove manual update calls
- [ ] Remove manual validation calls
- [ ] Simplify event handlers
- [ ] Profile performance
- [ ] Document reactive patterns used

---

## Next Steps

- **Learn More**: [User Guide](REACTIVITY_USER_GUIDE.md)
- **API Details**: [API Reference](../api/REACTIVITY_API.md)
- **Examples**: See `examples/` directory
- **Get Help**: Check documentation or file an issue

---

## Version Information

**Jounce v0.4.0 "Reactive"**
Released: October 2025

**Migration Summary**:
- ✅ Fully backward compatible
- ✅ No breaking changes
- ✅ Opt-in adoption
- ✅ Gradual migration supported
- ✅ Hybrid approach encouraged

**See Also**:
- [v0.4.0 Release Notes](../../RELEASE_NOTES.md)
- [User Guide](REACTIVITY_USER_GUIDE.md)
- [API Reference](../api/REACTIVITY_API.md)
