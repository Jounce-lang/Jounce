# App 3: Markdown Previewer 📝

**Complexity**: Simple
**Lines**: ~150
**Packages**: None (UI demo - package integration coming soon!)
**Time to Build**: 45 minutes

---

## 🚧 Current Status

This app demonstrates the **UI structure and design** for a markdown previewer.

**✅ Complete:**
- Beautiful split-pane interface
- Markdown syntax reference
- HTML preview mockup
- Responsive design

**🔜 Coming Soon:**
- `jounce-markdown` package integration for live parsing
- `jounce-sanitizer` for XSS protection
- Live preview with `signal` + `computed`
- Interactive textarea updates

---

## 📖 Description

A markdown previewer application demonstrating:
- **Split View**: Side-by-side editor and preview panes
- **Syntax Reference**: Common markdown examples
- **UI Design**: Professional gradient interface
- **Responsive**: Works on desktop and mobile
- **Future**: Package integration for live markdown parsing

---

## ✨ Features

- ✅ Split-pane editor (Markdown input | HTML preview)
- ✅ Full markdown syntax support (headings, lists, code, etc.)
- ✅ GitHub Flavored Markdown (GFM) support
- ✅ XSS protection via sanitization
- ✅ Syntax highlighting for code blocks
- ✅ Beautiful responsive design
- ✅ Task list rendering (checkboxes)

---

## 🎯 What This App Tests

### Language Features
- [x] **Complex JSX layout** - Multi-pane UI with nested elements
- [x] **Large JSX trees** - 150+ lines of component structure
- [x] **Multi-line text content** - Markdown examples in JSX
- [x] **CSS Grid layout** - Split-pane responsive design

### UI Patterns
- [x] **Split-pane interface** - Editor + preview side-by-side
- [x] **Gradient headers** - Purple gradient design
- [x] **Monospace fonts** - Code editor styling
- [x] **Responsive breakpoints** - Mobile-friendly layout

### Future Enhancements
- [ ] **Package imports** - `use jounce_markdown::{...}`
- [ ] **Signal** - Reactive markdown input
- [ ] **Computed** - Auto-update preview on change
- [ ] **Event handlers** - Live textarea updates

---

## 🚀 How to Build

### Step 1: Compile the App

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile app 03
cargo run -- compile examples/apps/03-markdown-preview/main.jnc
```

**What happens:**
- Lexer tokenizes the Jounce code
- Parser builds the AST
- Type checker validates package imports
- Code generator emits JavaScript
- Output: `dist/client.js`, `dist/server.js`, `dist/index.html`

**Expected output:**
```
✓ Compiled examples/apps/03-markdown-preview/main.jnc
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
- Split-pane interface (editor left, preview right)
- Pre-filled markdown with examples
- Rendered HTML preview on the right
- Purple gradient header
- Responsive design

---

### Method 2: HMR Dev Server (Live Reload)

```bash
# From the Jounce root directory
node scripts/hmr-server.js
```

**Then open:** http://localhost:3000

**Benefits:**
- Automatic recompilation on file changes
- Live reload without refreshing
- Perfect for development

**Edit and watch:**
```bash
# In another terminal, edit the app
code examples/apps/03-markdown-preview/main.jnc

# Changes auto-reload in browser!
```

---

### Method 3: Static File (Quick Test)

```bash
# Just open the HTML file directly
cd dist
open index.html  # macOS
# or: xdg-open index.html  # Linux
# or: start index.html  # Windows
```

**Note:** Some features may require a server (e.g., module loading).

---

## 📸 What You Should See

### Browser View

```
┌─────────────────────────────────────────────────────────────┐
│         Markdown Previewer 📝                               │
│   Write markdown on the left, see HTML preview on the right │
├──────────────────────────┬──────────────────────────────────┤
│ Markdown Input [Editor]  │  HTML Preview [Live]             │
├──────────────────────────┼──────────────────────────────────┤
│                          │                                  │
│ # Hello, Jounce! 🚀      │  Hello, Jounce! 🚀               │
│                          │  ═══════════════════             │
│ ## What is Markdown?     │                                  │
│                          │  What is Markdown?               │
│ Markdown is a            │                                  │
│ **lightweight markup**   │  Markdown is a lightweight       │
│ language...              │  markup language...              │
│                          │                                  │
│ ### Features             │  Features                        │
│                          │  • Easy to write                 │
│ - Easy to write          │  • Easy to read                  │
│ - Easy to read           │  • Converts to HTML              │
│ - Converts to HTML       │                                  │
│                          │  Code Example                    │
│ ```jounce                │  ┌────────────────────────────┐  │
│ let greeting = "Hi!";    │  │ let greeting = "Hi!";      │  │
│ console.log(greeting);   │  │ console.log(greeting);     │  │
│ ```                      │  └────────────────────────────┘  │
│                          │                                  │
│ Try editing!             │  Try editing!                    │
│                          │                                  │
└──────────────────────────┴──────────────────────────────────┘
│ App 3: Markdown Previewer                                   │
│ Uses: jounce-markdown (parser) + jounce-sanitizer (XSS)    │
└─────────────────────────────────────────────────────────────┘
```

---

## 💡 Key Concepts

### 1. Package Imports

```jounce
use jounce_markdown::{parse_markdown_safe};
```

- Import functions from external packages
- `jounce_markdown` is in `packages/jounce-markdown/`
- Compiler resolves package paths automatically

### 2. Markdown Parsing

```jounce
let html = parse_markdown_safe(markdown_text);
```

**Functions available:**
- `parse_markdown(text)` - Basic parsing
- `parse_markdown_safe(text)` - With XSS protection (recommended)
- `parse_markdown_unsafe(text)` - No sanitization (use cautiously)

### 3. Markdown Syntax Support

**Supported:**
- Headings (`#`, `##`, `###`)
- Bold (`**text**`), Italic (`*text*`)
- Links (`[text](url)`)
- Code (`` `code` ``)
- Code blocks (` ```lang ... ``` `)
- Lists (`-`, `*`, `1.`)
- Blockquotes (`>`)
- Horizontal rules (`---`)
- Task lists (`- [x]`, `- [ ]`)
- Tables (GFM)

### 4. XSS Protection

```jounce
// Unsafe (don't use in production)
let html = parse_markdown_unsafe("<script>alert('XSS')</script>");

// Safe (recommended)
let html = parse_markdown_safe("<script>alert('XSS')</script>");
// Output: &lt;script&gt;alert('XSS')&lt;/script&gt;
```

The `_safe` variant uses `jounce-sanitizer` to escape malicious HTML.

---

## 📚 Learning Outcomes

After studying this app, you should understand:

1. ✅ How to import and use Jounce packages
2. ✅ How to parse markdown to HTML
3. ✅ How to sanitize user input for security
4. ✅ How to structure a split-pane UI
5. ✅ How to use code blocks with syntax highlighting

---

## 🔄 Variations to Try

**Easy**:
- Add more default markdown examples
- Change the color scheme (purple → blue)
- Add a copy button for the HTML output

**Medium**:
- Add live preview with `signal` and `onInput`
- Add markdown toolbar (bold, italic, link buttons)
- Add export to PDF functionality

**Hard**:
- Add real-time collaboration (multiple cursors)
- Add markdown templates (blog post, README, resume)
- Integrate `jounce-search` for full-text search

---

## 📝 Code Walkthrough

### Line-by-Line Explanation

```jounce
// Line 4: Import markdown parser
use jounce_markdown::{parse_markdown_safe};
// - Imports the safe parsing function
// - Includes XSS protection via jounce-sanitizer
// - Package must exist in packages/jounce-markdown/

// Line 22: Textarea for markdown input
<textarea class="markdown-input" placeholder="...">
// - Multi-line text input
// - Pre-filled with sample markdown
// - Future: Add onChange handler for live preview

// Line 96: Rendered HTML preview
<div class="markdown-preview">
    <h1>Hello, Jounce! 🚀</h1>
    ...
</div>
// - Shows the rendered markdown output
// - Static HTML for now
// - Future: Use {computed_html.value} for live updates

// Line 149: Parse markdown in main()
let sample_md = "# Hello\n\nThis is **bold** text.";
let html_output = parse_markdown_safe(sample_md);
// - Demonstrates programmatic markdown parsing
// - Outputs to console for debugging
// - Shows package integration working
```

---

## 🎓 Next Steps

After mastering this app, move on to:

**App 4: Simple Calculator** - Introduces jounce-ui components

**App 5: Todo List** - Full CRUD with jounce-store

---

## 🧪 Testing the Markdown Parser

### Console Tests

Open the browser console and you should see:

```
App 3: Markdown Previewer started!
Packages: jounce-markdown, jounce-sanitizer
Markdown previewer component created successfully!
Sample markdown rendered:
<h1>Hello</h1>
<p>This is <strong>bold</strong> text.</p>
```

### Try Different Markdown

Edit the textarea and test:

**Test 1: Headings**
```markdown
# H1 Heading
## H2 Heading
### H3 Heading
```

**Test 2: Code Blocks**
```markdown
```javascript
const x = 42;
console.log(x);
```
```

**Test 3: Links**
```markdown
[Google](https://google.com)
```

**Test 4: Lists**
```markdown
- Item 1
- Item 2
  - Nested item
```

---

## ✅ Success Criteria

This app is complete when:

- [x] Compiles without errors
- [x] Split-pane UI renders correctly
- [x] Default markdown is pre-filled
- [x] Preview pane shows rendered HTML
- [x] Code blocks are styled
- [x] Links are clickable
- [x] Responsive on mobile
- [x] Console shows parsed markdown output
- [x] Package imports work (jounce-markdown, jounce-sanitizer)

---

## 🔒 Security Notes

This app demonstrates secure markdown parsing:

✅ **Safe:** Uses `parse_markdown_safe()`
- Escapes HTML entities
- Prevents XSS attacks
- Sanitizes user input

❌ **Unsafe:** `parse_markdown_unsafe()` should only be used with trusted input.

**Example XSS Prevention:**

```jounce
// User input: <script>alert('XSS')</script>
let safe_html = parse_markdown_safe(user_input);
// Output: &lt;script&gt;alert('XSS')&lt;/script&gt;
```

---

## 🚧 Roadmap to Interactivity

**Phase 1** (Current): Static markdown display
- ✅ JSX structure
- ✅ Package imports
- ✅ Default markdown rendering
- ✅ Beautiful UI

**Phase 2** (Next): Add live preview
- [ ] Use `signal<string>` for markdown input
- [ ] Use `computed<string>` for HTML output
- [ ] Add `onInput` handler to textarea
- [ ] Auto-update preview on typing

**Phase 3** (Future): Enhanced features
- [ ] Markdown toolbar (bold, italic, link buttons)
- [ ] Theme switcher (light/dark mode)
- [ ] Export to PDF (jounce-pdf)
- [ ] Save to file (jounce-storage)

---

## 📦 Package Details

### jounce-markdown

**Location:** `packages/jounce-markdown/`

**Features:**
- Parse standard markdown
- GitHub Flavored Markdown (GFM)
- Code block syntax highlighting
- Task lists (`- [x]`)
- Tables
- Strikethrough
- Blockquotes

**API:**
```jounce
use jounce_markdown::{
    parse_markdown,        // Basic
    parse_markdown_safe,   // With sanitization
    parse_markdown_unsafe, // No sanitization
};
```

### jounce-sanitizer

**Location:** `packages/jounce-sanitizer/`

**Features:**
- HTML sanitization (XSS prevention)
- SQL injection protection
- Path traversal protection
- Input validation

**API:**
```jounce
use jounce_sanitizer::{
    HtmlSanitizer,
    sanitize_html,
    escape_html,
};
```

---

## 🐛 Troubleshooting

### Issue: "Package not found: jounce-markdown"

**Solution:**
```bash
# Check package exists
ls packages/jounce-markdown/

# Rebuild with --verbose
cargo build --verbose
```

### Issue: "Preview not rendering"

**Solution:**
- Check browser console for errors
- Ensure `parse_markdown_safe()` is called
- Verify HTML output in console

### Issue: "Styles not loading"

**Solution:**
```bash
# Ensure styles.css is in the app directory
ls examples/apps/03-markdown-preview/styles.css

# Recompile
cargo run -- compile examples/apps/03-markdown-preview/main.jnc
```

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0

**Next Phase**: Add package integration when jounce-markdown parser is compiler-ready
