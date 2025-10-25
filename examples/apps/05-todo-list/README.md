# App 5: Todo List ✅

**Complexity**: Simple
**Lines**: ~90
**Packages**: None (UI demo - state management coming soon!)
**Time to Build**: 45 minutes

---

## 📖 Description

A modern todo list application demonstrating:
- **Task Management UI**: Add, complete, and delete tasks
- **Filter System**: All, Active, Completed views
- **Checkbox Interactions**: Mark tasks as complete
- **Clean Design**: Modern, responsive interface
- **Future**: Dynamic state with signal and arrays

---

## ✨ Features

- ✅ Task input field with placeholder
- ✅ Add task button
- ✅ Filter buttons (All, Active, Completed)
- ✅ 5 sample tasks (2 completed, 3 active)
- ✅ Checkboxes for task completion
- ✅ Delete buttons (visible on hover)
- ✅ Task counter (shows remaining tasks)
- ✅ Clear completed button
- ✅ Strikethrough for completed tasks
- ✅ Smooth hover animations
- ✅ Responsive mobile design

---

## 🎯 What This App Tests

### Language Features
- [x] **JSX list rendering** - Multiple task items
- [x] **Conditional classes** - `.completed` class on items
- [x] **Form inputs** - Text input and checkboxes
- [x] **Button groups** - Filter and action buttons
- [x] **Complex layout** - Header, input, list, footer

### UI Patterns
- [x] **List items** - Repeated task structure
- [x] **Flexbox layouts** - Task item alignment
- [x] **Hover states** - Show/hide delete buttons
- [x] **Strikethrough styling** - Completed tasks
- [x] **Filter buttons** - Active state indicator
- [x] **Gradient backgrounds** - Purple header

### Future Enhancements
- [ ] **Signal<Array>** - Dynamic task list
- [ ] **Event handlers** - Add, delete, toggle tasks
- [ ] **Computed** - Filter tasks by status
- [ ] **Local storage** - Persist tasks
- [ ] **jounce-store** - State management package

---

## 🚀 How to Build

### Step 1: Compile the App

```bash
# From the Jounce root directory
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce

# Compile app 05
cargo run -- compile examples/apps/05-todo-list/main.jnc
```

**Expected output:**
```
✓ Compiled examples/apps/05-todo-list/main.jnc
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
- White card on purple gradient background
- Purple gradient header "My Tasks"
- Input field and "Add Task" button
- Three filter buttons (All, Active, Completed)
- List of 5 tasks (2 with checkmarks, 3 unchecked)
- Task counter: "3 tasks remaining"
- "Clear completed" button
- Hover over tasks to see delete buttons

---

### Method 2: HMR Dev Server (Live Reload)

```bash
# From the Jounce root directory
node scripts/hmr-server.js
```

**Then open:** http://localhost:3000

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
┌─────────────────────────────────────────────┐
│           My Tasks                          │
│    Stay organized, get things done          │
├─────────────────────────────────────────────┤
│                                             │
│  [What needs to be done?        ] [Add Task]│
│                                             │
├─────────────────────────────────────────────┤
│  [All]  [Active]  [Completed]               │
├─────────────────────────────────────────────┤
│  ☑ Complete Jounce tutorial       [Delete] │
│  ☐ Build example applications     [Delete] │
│  ☐ Deploy to production           [Delete] │
│  ☐ Write documentation            [Delete] │
│  ☑ Set up development environment [Delete] │
├─────────────────────────────────────────────┤
│  3 tasks remaining      [Clear completed]   │
├─────────────────────────────────────────────┤
│  App 5: Todo List                           │
│  Task management with add/complete/delete   │
└─────────────────────────────────────────────┘
```

**Visual Features:**
- ✅ Purple gradient header
- ✅ White card with rounded corners
- ✅ Blue "Add Task" button
- ✅ Three filter buttons (All is active)
- ✅ Completed tasks have strikethrough
- ✅ Delete buttons appear on hover
- ✅ Task count in footer

---

## 💡 Key Concepts

### 1. List Rendering Pattern

```jounce
<div class="task-list">
    <div class="task-item">...</div>
    <div class="task-item">...</div>
    <div class="task-item">...</div>
</div>
```

Multiple similar items with consistent structure (future: render from array).

### 2. Completed State

```jounce
<div class="task-item completed">
    <input type="checkbox" checked />
    <span class="task-text">Complete Jounce tutorial</span>
</div>
```

The `.completed` class adds strikethrough styling.

### 3. Hover-Visible Buttons

```css
.btn-delete {
    opacity: 0;
}

.task-item:hover .btn-delete {
    opacity: 1;
}
```

Delete buttons only appear when hovering over a task.

### 4. Active Filter Button

```jounce
<button class="filter-btn active">All</button>
```

The `.active` class highlights the current filter.

### 5. Flexbox Layout

```css
.task-item {
    display: flex;
    align-items: center;
    gap: 15px;
}
```

Aligns checkbox, text, and delete button in a row.

---

## 📚 Learning Outcomes

After studying this app, you should understand:

1. ✅ How to render lists of similar items
2. ✅ How to style completed vs active states
3. ✅ How to create hover-visible elements
4. ✅ How to use checkboxes in task lists
5. ✅ How to build filter button groups
6. ✅ How to structure CRUD (Create, Read, Update, Delete) UIs

---

## 🔄 Variations to Try

**Easy**:
- Add more sample tasks
- Change the color scheme (green instead of purple)
- Add task priority labels (High, Medium, Low)

**Medium**:
- Add task categories/tags
- Add due dates to tasks
- Add task descriptions (expand on click)
- Sort tasks by date or priority

**Hard**:
- Implement drag-and-drop reordering
- Add subtasks (nested todos)
- Add collaborative features (assign to users)
- Integrate with jounce-db for persistence

---

## 📝 Code Walkthrough

### Line-by-Line Explanation

```jounce
// Lines 14-21: Add task section
<div class="add-task">
    <input type="text" class="task-input" placeholder="What needs to be done?" />
    <button class="btn-add">Add Task</button>
</div>
// - Text input for new tasks
// - Add button to create task (future: onClick handler)
// - Flexbox layout with gap

// Lines 23-27: Filter buttons
<div class="filters">
    <button class="filter-btn active">All</button>
    <button class="filter-btn">Active</button>
    <button class="filter-btn">Completed</button>
</div>
// - Three filter options
// - "All" is currently active
// - Future: onClick to switch filters

// Lines 30-35: Task item
<div class="task-item completed">
    <input type="checkbox" class="task-checkbox" checked />
    <span class="task-text">Complete Jounce tutorial</span>
    <button class="btn-delete">Delete</button>
</div>
// - Checkbox (checked for completed)
// - Task text (strikethrough if completed)
// - Delete button (visible on hover)

// Lines 64-67: Footer
<footer class="task-footer">
    <span class="task-count">3 tasks remaining</span>
    <button class="btn-clear">Clear completed</button>
</footer>
// - Shows count of uncompleted tasks
// - Button to remove all completed tasks
```

---

## 🎓 Next Steps

After mastering this app, move on to:

**App 6: Weather App** - API calls, async data (future)

**App 7: Image Gallery** - File upload, thumbnails (future)

---

## 🧪 Testing the Todo List

### Console Output

Open browser console to see:

```
App 5: Todo List started!
Features: Task list, checkboxes, add/delete actions
Coming soon: Dynamic task management with signal!
Todo list component created successfully!
```

### Visual Testing

Check that:
- ✅ 5 tasks render correctly
- ✅ 2 tasks have checkmarks and strikethrough
- ✅ 3 tasks are unchecked
- ✅ Task counter shows "3 tasks remaining"
- ✅ "All" filter button is highlighted
- ✅ Delete buttons appear on hover
- ✅ Input placeholder visible
- ✅ Responsive on mobile

---

## ✅ Success Criteria

This app is complete when:

- [x] Compiles without errors
- [x] All 5 tasks render
- [x] Completed tasks have strikethrough
- [x] Checkboxes render correctly
- [x] Delete buttons visible on hover
- [x] Task counter displays
- [x] Filter buttons render
- [x] Add task UI renders
- [x] Responsive on mobile
- [x] No console errors

---

## 🎨 Design Notes

### TodoMVC Inspiration

This design is inspired by TodoMVC with modern enhancements:
- **Clean layout**: Simple, focused interface
- **Purple accents**: Modern gradient theme
- **Hover interactions**: Delete buttons on hover
- **Smooth animations**: Transitions and slide-ins
- **Mobile-first**: Responsive design

### Color Palette

```
Primary: #667eea (purple-blue)
Secondary: #764ba2 (purple)
Background: white (#ffffff)
Text: #1f2937 (dark gray)
Muted: #6b7280 (medium gray)
Success: #10b981 (green)
Danger: #ef4444 (red)
Border: #e5e7eb (light gray)
```

---

## 🚧 Roadmap to Interactivity

**Phase 1** (Current): Static UI demonstration
- ✅ Task list layout
- ✅ Filter buttons
- ✅ Add task UI
- ✅ Completed state styling

**Phase 2** (Next): Add basic interactivity
- [ ] Use `signal<Array<Task>>` for task list
- [ ] Add `onClick` handlers
- [ ] Add new tasks
- [ ] Toggle task completion
- [ ] Delete tasks

**Phase 3** (Future): Advanced features
- [ ] Filter by status (all/active/completed)
- [ ] Clear completed tasks
- [ ] Task counter (computed)
- [ ] Local storage persistence
- [ ] Edit task text
- [ ] Drag-and-drop reordering

**Phase 4** (Future): Backend integration
- [ ] Use `jounce-db` for persistence
- [ ] Use `jounce-store` for state management
- [ ] API sync
- [ ] Multi-user support

---

## 🐛 Troubleshooting

### Issue: Tasks not rendering

**Cause**: JSX syntax error
**Solution**: Check for proper closing tags in task-item divs

### Issue: Completed styling not working

**Cause**: CSS class not applied
**Solution**: Verify `.completed` class on task-item div

### Issue: Delete buttons not appearing on hover

**Cause**: CSS opacity issue
**Solution**: Check `.task-item:hover .btn-delete { opacity: 1; }`

---

## 📚 Resources

**Todo App References:**
- TodoMVC - Standard todo app implementation
- Microsoft To Do - Modern task manager
- Google Tasks - Simple task lists

**State Management Patterns:**
- CRUD operations (Create, Read, Update, Delete)
- Array manipulation (push, filter, map)
- Computed values (derived state)
- Local storage API

---

## 🔍 Implementation Details

### Task Data Structure (Future)

```jounce
struct Task {
    id: int,
    text: string,
    completed: bool,
    created_at: DateTime,
}

let tasks = signal<Array<Task>>([]);
```

### Add Task Logic (Future)

```jounce
fn addTask(text: string) {
    let new_task = Task {
        id: tasks.value.length + 1,
        text: text,
        completed: false,
        created_at: DateTime::now(),
    };
    tasks.value.push(new_task);
}
```

### Filter Logic (Future)

```jounce
let active_tasks = computed(() => {
    tasks.value.filter(t => !t.completed)
});

let completed_tasks = computed(() => {
    tasks.value.filter(t => t.completed)
});
```

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0

**Next Phase**: Add signal + array state for dynamic task management
