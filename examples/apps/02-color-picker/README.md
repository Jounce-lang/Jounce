# App 2: Color Picker 🎨

**Complexity**: Very Simple
**Lines**: ~75
**Packages**: None (vanilla Jounce)
**Time to Build**: 45 minutes

---

## 📖 Description

A color picker application demonstrating RGB color selection with live preview:
- **RGB Sliders**: Three sliders for Red, Green, and Blue values
- **Live Preview**: Visual color preview box
- **Hex Display**: Shows hex color code
- **RGB Display**: Shows RGB values

---

## ✨ Features

- ✅ RGB color sliders (Red, Green, Blue)
- ✅ Large color preview box
- ✅ Hex color code display (#8040C0)
- ✅ RGB value display (rgb(128, 64, 192))
- ✅ Beautiful gradient UI
- ✅ Responsive design

---

## 🎯 What This App Tests

### Language Features
- [x] **JSX** - Component rendering
- [x] **HTML attributes** - Type, min, max, value
- [x] **Inline styles** - Dynamic background-color
- [x] **Multiple elements** - Complex layout structure

### Future Enhancements
- [ ] **Signal** - Reactive RGB values
- [ ] **Computed** - Calculated hex color
- [ ] **Event handlers** - Slider onChange
- [ ] **jounce-theme** - Color utilities

---

## 🚀 How to Run

### Method 1: Production Server (Recommended)

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile the app
cargo run -- compile examples/apps/02-color-picker/main.jnc

# Run the Node.js server
cd dist
node server.js

# Open browser to http://localhost:3000
```

### Method 2: HMR Dev Server (Live Reload)

```bash
# From the Jounce root directory
cargo run -- compile examples/apps/02-color-picker/main.jnc

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
│     Color Picker 🎨             │
├─────────────────────────────────┤
│                                 │
│   ┌─────────────────────────┐   │
│   │                         │   │
│   │     [Purple Box]        │   │
│   │                         │   │
│   └─────────────────────────┘   │
│                                 │
│   HEX: #8040C0   RGB: 128,64,192│
│                                 │
│   Red     [======●===] 128      │
│   Green   [==●========] 64       │
│   Blue    [=========●=] 192      │
│                                 │
│ Adjust sliders to change color  │
│ (Interactive version coming!)   │
└─────────────────────────────────┘
```

---

## 💡 Key Concepts

### 1. RGB Color Model

The RGB color model uses three channels:
- **Red** (0-255)
- **Green** (0-255)
- **Blue** (0-255)

Combined, they create any color:
- `rgb(255, 0, 0)` = Pure red
- `rgb(0, 255, 0)` = Pure green
- `rgb(0, 0, 255)` = Pure blue
- `rgb(128, 64, 192)` = Purple (our default)

### 2. Hex Color Codes

Hex codes represent RGB in hexadecimal:
- Format: `#RRGGBB`
- Example: `#8040C0`
  - `80` = 128 red
  - `40` = 64 green
  - `C0` = 192 blue

### 3. Range Input Sliders

HTML range inputs provide slider UI:
```html
<input
    type="range"
    min="0"
    max="255"
    value="128"
/>
```

### 4. Inline CSS Styles

JSX supports inline style attributes:
```jsx
<div style="background-color: rgb(128, 64, 192);"></div>
```

---

## 📚 Learning Outcomes

After studying this app, you should understand:

1. ✅ How RGB color values work (0-255 per channel)
2. ✅ How to convert RGB to hex codes
3. ✅ How to use HTML range input sliders
4. ✅ How to apply inline CSS styles in JSX
5. ✅ How to structure a multi-section UI layout

---

## 🔄 Variations to Try

**Easy**:
- Change the default color to a different RGB value
- Add more color presets (buttons for common colors)
- Show RGB values as percentages (0-100%)

**Medium**:
- Add HSL color model sliders
- Add opacity/alpha slider (RGBA)
- Copy hex code to clipboard button

**Hard**:
- Add color harmony suggestions (complementary, triadic)
- Save favorite colors to a palette
- Generate color scheme (monochromatic, analogous)

---

## 📝 Code Walkthrough

### Line-by-Line Explanation

```jounce
// Lines 6-9: Main heading
<h1>Color Picker 🎨</h1>
// - Simple heading with emoji
// - Centered at top of card

// Lines 11-13: Color preview
<div class="preview" style="background-color: rgb(128, 64, 192);"></div>
// - Inline style sets background color
// - RGB values: Red=128, Green=64, Blue=192
// - Creates a purple color

// Lines 16-18: Hex display
<span class="value">#8040C0</span>
// - Shows hex color code
// - Calculated from RGB: 128=80, 64=40, 192=C0

// Lines 28-36: Red slider
<input
    type="range"
    class="slider red-slider"
    min="0"
    max="255"
    value="128"
/>
// - Range input creates slider
// - Min/max define range (0-255)
// - Value sets initial position
// - Class applies red gradient styling
```

---

## 🎓 Next Steps

After mastering this app, move on to:

**App 3: Markdown Previewer** - Introduces jounce-markdown and jounce-sanitizer packages

**App 4: Simple Calculator** - Uses jounce-ui components

---

## ✅ Success Criteria

This app is complete when:

- [x] Color preview displays purple (128, 64, 192)
- [x] Hex code shows #8040C0
- [x] RGB values display correctly
- [x] Three sliders render with correct values
- [x] UI is responsive on mobile
- [x] No console errors
- [x] Compiles successfully

---

## 🚧 Roadmap to Interactivity

**Phase 1** (Current): Static UI demonstration
- ✅ JSX structure
- ✅ Visual design
- ✅ Default color values

**Phase 2** (Next): Add reactivity
- [ ] Use `signal<int>` for R, G, B values
- [ ] Use `computed<string>` for hex calculation
- [ ] Add `onChange` handlers to sliders
- [ ] Live color updates

**Phase 3** (Future): Enhanced features
- [ ] Integrate jounce-theme package
- [ ] CSS variable support
- [ ] Theme switching
- [ ] Color palettes

---

**Status**: ✅ Complete (Static UI)
**Date**: October 24, 2025
**Jounce Version**: v0.8.0

**Next**: Add reactivity when signal/computed fully support inline handlers
