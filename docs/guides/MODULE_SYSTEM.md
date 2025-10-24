# Module System Guide

Jounce supports multi-file projects with a simple and intuitive module system. You can split your code into multiple files and import functions, structs, enums, and other definitions across files.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Import Syntax](#import-syntax)
- [Module Resolution](#module-resolution)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Basic Usage

### Creating a Module

Any `.jnc` file is automatically a module. All top-level definitions (functions, structs, enums, constants) are available for import by other files.

**math.jnc**:
```jounce
fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn multiply(a: i64, b: i64) -> i64 {
    return a * b;
}
```

### Importing a Module

Use the `use` statement to import another file:

**main.jnc**:
```jounce
use ./math;

fn main() {
    let result = add(5, 3);
    console.log(result.to_string());
}
```

## Import Syntax

### Relative Path Imports

Jounce supports relative path imports using `./` (current directory) and `../` (parent directory):

```jounce
// Import from current directory
use ./math;

// Import from parent directory
use ../utils;

// Import from nested directories
use ./models/user;
use ../lib/helpers;
```

### Package Imports

You can also import from installed packages:

```jounce
use jounce_http::HttpClient;
use jounce_router::Router;
```

### Selective Imports

Import specific items from a module (coming soon):

```jounce
// This syntax is planned but not yet implemented
use ./math::{add, multiply};
```

## Module Resolution

### How Jounce Finds Modules

When you write `use ./math`, Jounce looks for:
1. `./math.jnc` in the current directory
2. If not found, returns an error

When you write `use jounce_http::HttpClient`, Jounce searches:
1. Local `test_modules/` directory
2. Installed packages
3. Custom module paths (if configured)

### File Extensions

Always use `.jnc` extensions for Jounce files, but omit the extension in import statements:

```jounce
// ✓ Correct
use ./math;

// ✗ Incorrect
use ./math.jnc;
```

## Best Practices

### 1. Organize by Feature

Structure your project around features or domains:

```
my-app/
├── main.jnc
├── models/
│   ├── user.jnc
│   ├── post.jnc
│   └── comment.jnc
├── services/
│   ├── auth.jnc
│   └── api.jnc
└── utils/
    ├── validation.jnc
    └── formatting.jnc
```

### 2. Shared Types File

Create a `types.jnc` file for shared structs and enums:

**types.jnc**:
```jounce
struct User {
    id: i64,
    name: String,
    email: String,
}

enum Role {
    Admin,
    User,
    Guest,
}
```

**auth.jnc**:
```jounce
use ./types;

fn create_user(name: String, email: String) -> User {
    return User {
        id: generate_id(),
        name: name,
        email: email,
    };
}
```

### 3. Utility Functions

Group related utility functions in separate files:

**utils/validation.jnc**:
```jounce
fn is_valid_email(email: String) -> bool {
    return email.contains("@");
}

fn is_valid_password(password: String) -> bool {
    return password.length() >= 8;
}
```

**utils/formatting.jnc**:
```jounce
fn format_date(timestamp: i64) -> String {
    // Implementation
}

fn format_currency(amount: f64) -> String {
    // Implementation
}
```

### 4. Avoid Circular Dependencies

Don't create circular imports:

```jounce
// ✗ AVOID THIS

// a.jnc
use ./b;

// b.jnc
use ./a;  // Circular dependency!
```

Instead, extract shared code to a third file:

```jounce
// ✓ BETTER

// shared.jnc
struct SharedData { /* ... */ }

// a.jnc
use ./shared;

// b.jnc
use ./shared;
```

## Examples

### Example 1: Simple Calculator

**calculator/math.jnc**:
```jounce
fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn subtract(a: i64, b: i64) -> i64 {
    return a - b;
}

fn multiply(a: i64, b: i64) -> i64 {
    return a * b;
}

fn divide(a: i64, b: i64) -> i64 {
    return a / b;
}
```

**calculator/main.jnc**:
```jounce
use ./math;

fn main() {
    console.log("10 + 5 = " + add(10, 5).to_string());
    console.log("10 - 5 = " + subtract(10, 5).to_string());
    console.log("10 * 5 = " + multiply(10, 5).to_string());
    console.log("10 / 5 = " + divide(10, 5).to_string());
}
```

### Example 2: Todo App (Multi-File)

See the complete example in `examples/todo-app-multi-file/`.

**Project Structure**:
```
todo-app-multi-file/
├── main.jnc       # Entry point
├── types.jnc      # Shared types (Todo, Filter)
├── utils.jnc      # Utility functions
└── storage.jnc    # Storage layer
```

**types.jnc**:
```jounce
struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

enum Filter {
    All,
    Active,
    Completed,
}

fn create_todo(id: i64, title: String) -> Todo {
    return Todo {
        id: id,
        title: title,
        completed: false,
    };
}
```

**utils.jnc**:
```jounce
fn format_count(count: i64) -> String {
    if count == 1 {
        return "1 item";
    }
    return count.to_string() + " items";
}

fn validate_title(title: String) -> bool {
    return title.length() > 0;
}
```

**storage.jnc**:
```jounce
use ./types;

fn get_sample_todo() -> Todo {
    return create_todo(1, "Sample task");
}

fn format_todo(todo: Todo) -> String {
    let status = if todo.completed { "[x]" } else { "[ ]" };
    return status + " " + todo.title;
}

fn mark_completed(todo: Todo) -> Todo {
    return Todo {
        id: todo.id,
        title: todo.title,
        completed: true,
    };
}
```

**main.jnc**:
```jounce
use ./types;
use ./utils;
use ./storage;

fn main() {
    console.log("=== Todo App ===");

    let todo = create_todo(1, "Learn Jounce");
    console.log(format_todo(todo));

    let completed = mark_completed(todo);
    console.log(format_todo(completed));

    console.log(format_count(2));
}
```

### Example 3: Web App with Services

**models/user.jnc**:
```jounce
struct User {
    id: i64,
    name: String,
    email: String,
}

fn new_user(id: i64, name: String, email: String) -> User {
    return User {
        id: id,
        name: name,
        email: email,
    };
}
```

**services/auth.jnc**:
```jounce
use ../models/user;

fn register(name: String, email: String) -> User {
    let id = generate_id();
    return new_user(id, name, email);
}

fn generate_id() -> i64 {
    return 1; // Simplified
}
```

**main.jnc**:
```jounce
use ./models/user;
use ./services/auth;

fn main() {
    let user = register("Alice", "alice@example.com");
    console.log("Registered: " + user.name);
}
```

## Troubleshooting

### Error: "Module not found"

**Problem**: Jounce can't find the module you're trying to import.

**Solutions**:
1. Check the file exists at the path you specified
2. Verify the file has a `.jnc` extension
3. Make sure you're using the correct relative path (`./` or `../`)
4. Check for typos in the module name

### Error: "Expected Identifier, found Dot"

**Problem**: Old version of Jounce without relative path support.

**Solution**: Update to Jounce v0.3.1 or later.

### Error: "Function not found"

**Problem**: The function you're calling isn't exported from the module.

**Solutions**:
1. Check the function is defined in the imported module
2. Verify the function name spelling
3. Make sure the function is not private (currently all functions are public)

### Circular Dependencies

**Problem**: Two modules import each other.

**Solution**: Extract shared code to a third module that both can import.

## Current Limitations

### No Explicit Exports (Yet)

Currently, all top-level definitions in a file are automatically exported. Explicit export control with the `export` keyword is planned for a future release:

```jounce
// Planned syntax (not yet implemented)
export fn public_function() { }

fn private_function() { }  // Not exported
```

### No Selective Imports (Yet)

You must import the entire module. Selective imports are planned:

```jounce
// Planned syntax (not yet implemented)
use ./math::{add, multiply};
```

### No Re-exports (Yet)

You cannot re-export imported items:

```jounce
// Planned syntax (not yet implemented)
export use ./math::{add};
```

## Next Steps

- Read the [Todo App Example](../../examples/todo-app-multi-file/README.md)
- Learn about [Project Structure Best Practices](./PROJECT_STRUCTURE.md) (coming soon)
- Explore [Package Management](./PACKAGE_MANAGER.md) (coming soon)

---

**Last Updated**: Phase 11 - Module System Implementation
**Status**: Local file imports fully functional
**Version**: Jounce v0.3.1 (in development)
