# Todo App - Multi-File Example

This is a demonstration of Jounce's cross-file import system.

## Project Structure

```
todo-app-multi-file/
├── main.jnc       # Entry point, uses all modules
├── types.jnc      # Shared types (Todo, Filter)
├── utils.jnc      # Utility functions (format_count, validate_title)
├── storage.jnc    # Storage layer (add_todo, get_all_todos, etc.)
└── README.md      # This file
```

## Module Dependencies

```
main.jnc
├── imports ./types
├── imports ./utils
└── imports ./storage
    └── imports ./types
```

## Features Demonstrated

- ✅ **Multiple file imports** (`use ./module`)
- ✅ **Struct definitions** across files
- ✅ **Enum types** shared between modules
- ✅ **Function imports** from multiple sources
- ✅ **Nested imports** (storage.jnc imports types.jnc)
- ✅ **Cross-module usage** (main uses all modules)

## How to Compile

From the Jounce project root:

```bash
cd examples/todo-app-multi-file
../../target/release/jnc compile main.jnc
```

## How to Run

```bash
cd dist
node server.js
```

## Expected Output

```
=== Todo App (Multi-file Example) ===

Creating todos...
✓ Created 3 todos

All todos:
  [ ] Learn Jounce
  [ ] Build an app
  [ ] Write documentation

Completing first todo...
✓ Todo completed

Active: 2 items

Title 'New todo' is valid: true

=== All imports working correctly! ===
```

## What This Proves

This example proves that Jounce's module system can handle:

1. **Real-world project structure** - Multiple files with clear responsibilities
2. **Nested imports** - Modules can import other modules
3. **Type sharing** - Structs and enums work across files
4. **Function imports** - All functions from a module are available
5. **Compilation** - Everything compiles to working JavaScript

## Next Steps

Try modifying the app:
- Add new todo functions
- Create a UI layer (components.jnc)
- Add API functions (@server functions)
- Implement filtering (use the Filter enum)

---

**Phase 11 Status**: Multi-file imports are fully functional! 🎉
