# App 4: Simple Calculator 🔢

**Complexity**: Simple
**Lines**: ~60
**Packages**: None (UI demo)
**Time to Build**: 30 minutes

---

## 📖 Description

A classic calculator application demonstrating:
- **Button Grid Layout**: CSS Grid for calculator buttons
- **iOS-Inspired Design**: Dark theme with orange accents
- **Responsive Layout**: Works on all screen sizes
- **Clean UI**: Professional calculator interface
- **Future**: Interactive calculations with signal

---

## ✨ Features

- ✅ iOS-style calculator design (dark theme)
- ✅ 4×5 button grid layout
- ✅ Large display area
- ✅ Number pad (0-9)
- ✅ Basic operators (+, -, ×, ÷)
- ✅ Function buttons (AC, +/-, %)
- ✅ Equals button
- ✅ Decimal point support
- ✅ Responsive design (mobile-friendly)

---

## 🎯 What This App Tests

### Language Features
- [x] **JSX button grid** - 19 button elements
- [x] **CSS Grid layout** - 4-column responsive grid
- [x] **Class names** - Multiple button styles
- [x] **Component structure** - Calculator as single component

### UI Patterns
- [x] **Button grid** - 4×5 calculator layout
- [x] **Dark theme** - Black background, white text
- [x] **Color coding** - Different colors for number/operator/function buttons
- [x] **Circular buttons** - border-radius: 50%
- [x] **Large display** - 4em font size
- [x] **Hover effects** - Scale and brightness changes

### Future Enhancements
- [ ] **Signal** - Reactive display value
- [ ] **Event handlers** - onClick for each button
- [ ] **Computed** - Calculate results
- [ ] **State management** - Track current operation

---

## 🚀 How to Build

### Step 1: Compile the App

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile app 04
cargo run -- compile examples/apps/04-calculator/main.jnc
```

**Expected output:**
```
✓ Compiled examples/apps/04-calculator/main.jnc
✓ Generated dist/client.js
✓ Generated dist/server.js
✓ Generated dist/index.html
```

---

## 🚢 How to Deploy

### Method 1: Production Server (Recommended)

```bash
# Start the Node.js server
cd dist
node server.js
```

**Then open:** http://localhost:3000

**What you should see:**
- Dark calculator with iOS-style design
- Blue gradient header "Calculator 🔢"
- Large display showing "0"
- 4×5 grid of buttons
- Orange operator buttons on the right
- Gray function buttons on top row
- Dark gray number buttons

---

### Method 2: HMR Dev Server (Live Reload)

```bash
# From the Jounce root directory
node scripts/hmr-server.js
```

**Then open:** http://localhost:3000

**Benefits:**
- Auto-reload on file changes
- Perfect for styling tweaks

---

### Method 3: Static File (Quick Test)

```bash
cd dist
open index.html  # macOS
```

---

## 📸 What You Should See

### Browser View

```
┌────────────────────────────────────┐
│      Calculator 🔢                 │
├────────────────────────────────────┤
│                                    │
│                           0        │
│                                    │
├────────────────────────────────────┤
│  (AC)  (+/-)  (%)   [÷]            │
│                                    │
│   7     8     9    [×]             │
│                                    │
│   4     5     6    [-]             │
│                                    │
│   1     2     3    [+]             │
│                                    │
│      0       .     [=]             │
│                                    │
└────────────────────────────────────┘
│ App 4: Simple Calculator           │
│ Interactive functionality coming!  │
└────────────────────────────────────┘
```

**Color Key:**
- Numbers (0-9, .): Dark gray (#333)
- Operators (+, -, ×, ÷): Orange (#ff9f0a)
- Functions (AC, +/-, %): Light gray (#a5a5a5)
- Equals (=): Orange (#ff9f0a)
- Display: White text on black
- Background: Blue gradient

---

## 💡 Key Concepts

### 1. CSS Grid Layout

```css
.buttons {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
}
```

Creates a 4-column grid with equal-width columns and 12px gaps.

### 2. Spanning Multiple Columns

```css
.btn-zero {
    grid-column: span 2;
}
```

The "0" button spans 2 columns (takes up double width).

### 3. Circular Buttons

```css
.btn {
    border-radius: 50%;
    height: 80px;
}
```

Creates circular buttons when width equals height.

### 4. Button Color Coding

Different button types use different colors:
- **Numbers**: Dark gray for easy identification
- **Operators**: Orange for visual prominence
- **Functions**: Light gray to differentiate from numbers

### 5. Hover Effects

```css
.btn:hover {
    filter: brightness(1.2);
    transform: scale(1.05);
}
```

Buttons brighten and scale up on hover for feedback.

---

## 📚 Learning Outcomes

After studying this app, you should understand:

1. ✅ How to create button grid layouts with CSS Grid
2. ✅ How to style buttons with different color schemes
3. ✅ How to make circular buttons with border-radius
4. ✅ How to span grid items across multiple columns
5. ✅ How to create hover effects with transforms
6. ✅ How to build iOS-style dark theme interfaces

---

## 🔄 Variations to Try

**Easy**:
- Change color scheme (blue/green instead of orange)
- Add more spacing between buttons
- Increase button size

**Medium**:
- Add scientific calculator functions (sin, cos, log)
- Add memory buttons (M+, M-, MR, MC)
- Add history display (show previous calculation)

**Hard**:
- Implement full calculator logic with signal
- Add keyboard support (type numbers and operators)
- Add calculation history sidebar
- Add themes (light/dark toggle)

---

## 📝 Code Walkthrough

### Line-by-Line Explanation

```jounce
// Line 10-13: Display area
<div class="display">
    <div class="display-value">0</div>
</div>
// - Large display showing current value
// - Right-aligned text
// - 4em font size for readability

// Line 16: Button grid container
<div class="buttons">
// - CSS Grid with 4 columns
// - 12px gap between buttons
// - Contains all 19 calculator buttons

// Line 17-20: Top row (functions + divide)
<button class="btn btn-function">AC</button>
<button class="btn btn-function">+/-</button>
<button class="btn btn-function">%</button>
<button class="btn btn-operator">÷</button>
// - AC clears all (future: clear display and state)
// - +/- toggles sign (future: negate current number)
// - % calculates percentage (future: divide by 100)
// - ÷ division operator

// Line 22-25: Number row 7-9 + multiply
<button class="btn btn-number">7</button>
<button class="btn btn-number">8</button>
<button class="btn btn-number">9</button>
<button class="btn btn-operator">×</button>
// - Number buttons: Dark gray background
// - Operator button: Orange background

// Line 43-45: Bottom row (zero, decimal, equals)
<button class="btn btn-number btn-zero">0</button>
<button class="btn btn-number">.</button>
<button class="btn btn-equals">=</button>
// - Zero button spans 2 columns (wider)
// - Decimal point for float numbers
// - Equals calculates result (future: evaluate expression)
```

---

## 🎓 Next Steps

After mastering this app, move on to:

**App 5: Todo List** - CRUD operations, list management, state

**App 6: Weather App** - API calls, async data fetching (future)

---

## 🧪 Testing the Calculator

### Console Output

Open browser console to see:

```
App 4: Simple Calculator started!
Features: Button grid layout, calculator display
Coming soon: Interactive calculations with signal!
Calculator component created successfully!
```

### Visual Testing

Check that:
- ✅ All 19 buttons render correctly
- ✅ Display shows "0" by default
- ✅ Zero button is wider (spans 2 columns)
- ✅ Operator buttons are orange
- ✅ Function buttons are light gray
- ✅ Number buttons are dark gray
- ✅ Buttons are circular
- ✅ Hover effects work (brightness + scale)

---

## ✅ Success Criteria

This app is complete when:

- [x] Compiles without errors
- [x] All 19 buttons render in 4×5 grid
- [x] Display shows "0"
- [x] Buttons have correct colors
- [x] Zero button spans 2 columns
- [x] Circular button shape
- [x] Hover effects work
- [x] Responsive on mobile
- [x] Dark theme looks professional
- [x] No console errors

---

## 🎨 Design Notes

### iOS Calculator Inspiration

This design is inspired by the iOS Calculator app:
- **Dark theme**: Black background (#000000)
- **Number buttons**: Dark gray (#333333)
- **Function buttons**: Light gray (#a5a5a5)
- **Operator buttons**: Orange (#ff9f0a)
- **Large display**: White text, right-aligned
- **Circular buttons**: border-radius: 50%
- **Grid layout**: 4 columns, consistent spacing

### Color Palette

```
Background: #000000 (black)
Numbers: #333333 (dark gray)
Functions: #a5a5a5 (light gray)
Operators: #ff9f0a (orange)
Display Text: #ffffff (white)
Info Text: #999999 (medium gray)
```

---

## 🚧 Roadmap to Interactivity

**Phase 1** (Current): Static UI demonstration
- ✅ Button grid layout
- ✅ Display area
- ✅ Visual design
- ✅ Hover effects

**Phase 2** (Next): Add basic interactivity
- [ ] Use `signal<string>` for display value
- [ ] Add `onClick` handlers to buttons
- [ ] Update display when numbers clicked
- [ ] Clear display with AC button

**Phase 3** (Future): Full calculator logic
- [ ] Track current operation (add, subtract, etc.)
- [ ] Track previous value
- [ ] Implement equals button logic
- [ ] Handle decimal points
- [ ] Handle multiple operations
- [ ] Add keyboard support

---

## 🐛 Troubleshooting

### Issue: Buttons not circular

**Cause**: Width and height must be equal
**Solution**: Check that `.btn { height: 80px; }` and grid columns are equal

### Issue: Zero button not wider

**Cause**: CSS Grid span not working
**Solution**: Verify `.btn-zero { grid-column: span 2; }`

### Issue: Colors wrong

**Cause**: Class names might be incorrect
**Solution**: Check button classes (btn-number, btn-operator, btn-function, btn-equals)

---

## 📚 Resources

**Calculator Logic References:**
- Basic calculator: Add, subtract, multiply, divide
- Order of operations: Multiply/divide before add/subtract
- Decimal handling: Limit decimal places
- Edge cases: Division by zero, overflow

**Design References:**
- iOS Calculator app
- Google Calculator
- Windows Calculator

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0

**Next Phase**: Add signal and onClick handlers for interactivity
