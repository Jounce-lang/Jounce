# App 1: Hello Counter 🔢

**Complexity**: Very Simple
**Lines**: ~50
**Packages**: None (vanilla Jounce)
**Time to Build**: 30 minutes

---

## 📖 Description

A simple counter application demonstrating the core features of Jounce:
- **Reactivity**: Uses `signal` for reactive state management
- **JSX**: Clean component syntax
- **Event Handling**: Button click handlers
- **Auto-Updates**: UI updates automatically when state changes

---

## ✨ Features

- ✅ Increment counter
- ✅ Decrement counter
- ✅ Reset to zero
- ✅ Reactive display (auto-updates)
- ✅ Clean, minimal UI

---

## 🎯 What This App Tests

### Language Features
- [x] **Reactivity (signal)** - Mutable reactive state
- [x] **JSX** - Component rendering
- [x] **Event handlers** - onClick events
- [x] **Lambda expressions** - Inline arrow functions
- [x] **Type annotations** - `signal<int>`

### Jounce Primitives
- [x] `signal<T>(value)` - Create reactive state
- [x] `.value` - Access/update signal value
- [x] `onClick={() => ...}` - Event handling
- [x] `render()` - Mount component to DOM

---

## 🚀 How to Run

### Method 1: Production Server (Recommended)

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile the app
cargo run -- compile examples/apps/01-hello-counter/main.jnc

# Run the Node.js server
cd dist
node server.js

# Open browser to http://localhost:3000
```

### Method 2: HMR Dev Server (Live Reload)

```bash
# From the Jounce root directory
cargo run -- compile examples/apps/01-hello-counter/main.jnc

# Start HMR server with auto-reload
node scripts/hmr-server.js

# Open browser to http://localhost:3000
# Edit main.jnc and see changes instantly!
```

### Method 3: Static File (Quick Test)

```bash
# Just open the HTML file directly
cd dist
open index.html  # macOS
# or: xdg-open index.html  # Linux
# or: start index.html  # Windows
```

**See [DEV_SERVER_GUIDE.md](../../../DEV_SERVER_GUIDE.md) for complete server documentation.**

---

## 📸 Screenshot

```
┌─────────────────────────────────┐
│     Hello Counter! 🔢          │
├─────────────────────────────────┤
│                                 │
│            42                   │
│                                 │
├─────────────────────────────────┤
│  [− Decrease] [Reset] [+ Increase]  │
├─────────────────────────────────┤
│ Click the buttons to change     │
│ the counter value.              │
│                                 │
│ The display updates             │
│ automatically using Jounce's    │
│ reactivity system!              │
└─────────────────────────────────┘
```

---

## 💡 Key Concepts

### 1. Reactive State with `signal`

```jounce
let count = signal<int>(0);
```

Creates a reactive signal that:
- Holds a value (initially 0)
- Automatically tracks dependencies
- Updates all dependent UI when changed

### 2. Accessing and Updating Signals

```jounce
count.value           // Read the value
count.value = 42      // Update the value (triggers UI update)
```

### 3. Event Handlers

```jounce
<button onClick={() => count.value = count.value + 1}>
```

Lambda functions handle events inline, updating the signal directly.

### 4. Automatic UI Updates

When `count.value` changes, Jounce automatically:
1. Detects the change
2. Finds components using that signal
3. Re-renders only the affected parts

No manual DOM manipulation needed!

---

## 📚 Learning Outcomes

After studying this app, you should understand:

1. ✅ How to create reactive state with `signal`
2. ✅ How to read and update signal values
3. ✅ How event handlers trigger state changes
4. ✅ How Jounce automatically updates the UI
5. ✅ Basic JSX component structure

---

## 🔄 Variations to Try

**Easy**:
- Add a "Double" button that multiplies count by 2
- Add a "Halve" button that divides count by 2
- Display whether the count is even or odd

**Medium**:
- Add step size control (increment by 1, 5, or 10)
- Add min/max limits (e.g., 0-100)
- Add keyboard shortcuts (arrow keys)

**Hard**:
- Add undo/redo functionality
- Add animation when count changes
- Create a history of all values

---

## 📝 Code Walkthrough

### Line-by-Line Explanation

```jounce
// Line 4: Import the signal primitive
use std::reactive::{signal};

// Line 7: Create reactive state
let count = signal<int>(0);
// - `signal` creates a reactive container
// - `<int>` specifies the type
// - `0` is the initial value

// Line 10-12: Component function
fn App() -> JSX {
    return (...);
}
// - Functions that return JSX are components
// - Can be used like HTML tags

// Line 14-16: Display the count
<div class="counter-display">
    <p class="count">{count.value}</p>
</div>
// - `{count.value}` interpolates the signal value
// - Automatically re-renders when count changes

// Line 19: Increment handler
onClick={() => count.value = count.value + 1}
// - Lambda function (arrow function)
// - Reads current value, adds 1, updates signal
// - Triggers automatic UI update
```

---

## 🎓 Next Steps

After mastering this app, move on to:

**App 2: Color Picker** - Introduces computed values and the theme system

**App 3: Markdown Previewer** - Adds packages (jounce-markdown, jounce-sanitizer)

---

## ✅ Success Criteria

This app is complete when:

- [x] Counter starts at 0
- [x] Increment button adds 1
- [x] Decrement button subtracts 1
- [x] Reset button returns to 0
- [x] Display updates automatically
- [x] No console errors
- [x] Compiles successfully

---

**Status**: ✅ Complete
**Date**: October 24, 2025
**Jounce Version**: v0.8.0
