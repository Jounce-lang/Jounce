# RavensOne Utility Classes - Examples

This directory contains practical examples demonstrating the RavensOne utility class system.

## Quick Start

Compile any example:
```bash
cd /path/to/ravensone
./target/release/raven compile examples/utilities/01_buttons.raven
```

Open `dist/index.html` in your browser to see the result!

---

## Examples

### 01_buttons.raven
**Demonstrates**: Button variants, sizes, states, and groups

**What You'll Learn**:
- Creating primary, secondary, outline, and danger buttons
- Button sizing (small, medium, large)
- Hover and active states
- Button groups
- Responsive button sizing
- Icon buttons
- Disabled states

**Key Classes**: `px-*`, `py-*`, `bg-*`, `text-*`, `rounded-*`, `hover:*`, `shadow-*`, `opacity-*`, `cursor-*`

---

### 02_cards.raven
**Demonstrates**: Card components with various layouts

**What You'll Learn**:
- Simple cards with padding and shadows
- Image cards with headers
- Hover effects on cards
- Horizontal card layouts
- Pricing cards with borders
- Profile cards with centered content
- Stat cards with icons
- Responsive card sizing

**Key Classes**: `p-*`, `rounded-*`, `shadow-*`, `hover:*`, `flex`, `items-center`, `justify-*`, `grid-cols-*`, `gap-*`

---

### 03_forms.raven
**Demonstrates**: Form layouts and input styling

**What You'll Learn**:
- Login forms with focus states
- Contact forms with grid layouts
- Search forms with inline buttons
- Filter forms with selects and checkboxes
- Newsletter signup forms
- Validation states (success/error)
- Responsive form layouts

**Key Classes**: `w-full`, `px-*`, `py-*`, `border`, `border-*`, `focus:*`, `rounded`, `grid-cols-*`, `gap-*`, `flex-*`

---

### 04_responsive_layouts.raven
**Demonstrates**: Responsive design patterns

**What You'll Learn**:
- Responsive grid layouts (1 → 2 → 4 columns)
- Sidebar layouts (stacked → side-by-side)
- Responsive navigation bars
- Hero sections with scaling text
- Responsive card grids
- Responsive typography
- Conditional visibility (hide on mobile, show on desktop)
- Mobile-first design approach

**Key Classes**: `sm:*`, `md:*`, `lg:*`, `xl:*`, `2xl:*`, `hidden`, `block`, `flex-col`, `flex-row`

---

## Utility Categories

All examples use these utility categories:

### Layout
- **Display**: `block`, `flex`, `grid`, `hidden`
- **Flexbox**: `flex-row`, `flex-col`, `items-center`, `justify-between`
- **Grid**: `grid-cols-1`, `grid-cols-2`, `grid-cols-3`, `grid-cols-4`, `gap-4`

### Spacing
- **Padding**: `p-4`, `px-6`, `py-2`, `pt-4`, `pr-2`, `pb-4`, `pl-2`
- **Margin**: `m-4`, `mx-auto`, `my-8`, `mt-4`, `mb-6`

### Colors
- **Background**: `bg-blue-500`, `bg-white`, `bg-gray-100`, `bg-transparent`
- **Text**: `text-white`, `text-gray-700`, `text-blue-500`
- **Border**: `border-gray-300`, `border-blue-500`

### Typography
- **Size**: `text-sm`, `text-base`, `text-lg`, `text-xl`, `text-2xl`, `text-3xl`, `text-4xl`
- **Weight**: `font-normal`, `font-medium`, `font-semibold`, `font-bold`
- **Align**: `text-left`, `text-center`, `text-right`

### Borders
- **Width**: `border`, `border-2`, `border-4`
- **Radius**: `rounded`, `rounded-lg`, `rounded-xl`, `rounded-full`

### Sizing
- **Width**: `w-full`, `w-screen`, `w-auto`, `w-32`, `w-64`, `w-1/2`
- **Height**: `h-full`, `h-screen`, `h-32`, `h-48`
- **Max Width**: `max-w-sm`, `max-w-md`, `max-w-lg`, `max-w-2xl`, `max-w-4xl`, `max-w-6xl`

### Effects
- **Shadow**: `shadow-sm`, `shadow-md`, `shadow-lg`, `shadow-xl`
- **Opacity**: `opacity-50`, `opacity-75`, `opacity-100`
- **Cursor**: `cursor-pointer`, `cursor-not-allowed`

### Position
- **Type**: `relative`, `absolute`, `fixed`, `sticky`
- **Offsets**: `top-0`, `right-0`, `bottom-0`, `left-0`, `inset-0`

### Responsive Variants
- `sm:` - 640px and up (mobile landscape/small tablet)
- `md:` - 768px and up (tablet)
- `lg:` - 1024px and up (desktop)
- `xl:` - 1280px and up (large desktop)
- `2xl:` - 1536px and up (extra large)

### State Variants
- `hover:` - Mouse hover
- `focus:` - Keyboard/mouse focus
- `active:` - Active/pressed state
- `disabled:` - Disabled state

---

## Tips for Learning

### 1. Start Simple
Begin with `01_buttons.raven` to understand basic utility usage.

### 2. Experiment
Try modifying class names to see how styles change:
```raven
// Original
<button class="px-4 py-2 bg-blue-500">Click</button>

// Try changing:
<button class="px-8 py-4 bg-red-500">Click</button>
```

### 3. Combine Examples
Mix patterns from different examples to create your own components.

### 4. Resize Your Browser
For responsive examples, resize your browser window to see breakpoints in action.

### 5. Inspect Generated CSS
Check `dist/styles.css` after compiling to see the generated utility classes.

---

## Compile All Examples

```bash
# Compile each example
./target/release/raven compile examples/utilities/01_buttons.raven
./target/release/raven compile examples/utilities/02_cards.raven
./target/release/raven compile examples/utilities/03_forms.raven
./target/release/raven compile examples/utilities/04_responsive_layouts.raven
```

---

## Next Steps

After exploring these examples:

1. **Read the User Guide**: `docs/guides/UTILITY_CLASSES.md`
2. **Check Implementation Details**: `docs/guides/UTILITY_SYSTEM_DESIGN.md`
3. **Build Your Own Components**: Create custom components using these patterns
4. **Customize Configuration**: Create `raven.config.toml` to customize colors, spacing, etc.

---

## Need Help?

- **Full Documentation**: See `docs/guides/UTILITY_CLASSES.md`
- **System Design**: See `docs/guides/UTILITY_SYSTEM_DESIGN.md`
- **CSS Syntax**: See `docs/guides/CSS_SYNTAX.md`

---

**Happy Coding!** 🚀
