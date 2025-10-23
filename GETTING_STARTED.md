# Getting Started with RavensOne

> ✅ **Production Ready**: RavensOne language core is 100% complete! All core features working with 417 passing tests. See README.md for full status.

Welcome to RavensOne! This guide will help you get up and running with the full-stack reactive programming language that compiles to WebAssembly.

**What works**: Everything! JSX, deeply nested if/else, functions (including recursive), async/await, generics, traits, pattern matching, for loops, and more. Zero known limitations.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Basic Syntax](#basic-syntax)
- [Core Concepts](#core-concepts)
- [Building Your First App](#building-your-first-app)
- [Development Workflow](#development-workflow)
- [Deployment](#deployment)
- [Next Steps](#next-steps)

## Installation

### Prerequisites

- Rust 1.70+ (for building the compiler)
- Node.js 18+ (for development server)
- A modern web browser

### Install from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/ravensone.git
cd ravensone

# Build the compiler
cargo build --release

# Add to PATH (optional)
export PATH="$PATH:$(pwd)/target/release"
```

### Verify Installation

```bash
raven --version
# RavensOne Compiler v3.0
```

## Quick Start

Let's create your first RavensOne application in 5 minutes!

### 1. Create a New Project

```bash
# Initialize a new project
raven pkg init my-app
cd my-app
```

This creates:
```
my-app/
├── raven.toml       # Package manifest
├── src/
│   └── main.raven   # Entry point
└── dist/            # Output directory
```

### 2. Write Your First Component

Edit `src/main.raven`:

```rust
import { Signal } from "raven-reactive"

component Counter() {
    let count = Signal::new(0);

    <div class="counter">
        <h1>"Counter: " {count.get()}</h1>
        <button onclick={() => count.set(count.get() + 1)}>
            "Increment"
        </button>
        <button onclick={() => count.set(count.get() - 1)}>
            "Decrement"
        </button>
    </div>
}

fn main() {
    mount(Counter(), "#app");
}
```

### 3. Compile and Run

```bash
# Compile to WebAssembly
raven compile src/main.raven --output dist/app.wasm

# Start development server with Hot Module Replacement
raven dev

# Open http://localhost:3000
```

Your counter app is now running! Try changing the code - the browser updates automatically thanks to HMR.

## Basic Syntax

### Variables and Types

RavensOne has full Hindley-Milner type inference:

```rust
// Type inference
let x = 42;              // i32
let name = "Alice";      // String
let items = [1, 2, 3];   // Vec<i32>

// Explicit types
let age: i32 = 30;
let balance: f64 = 100.50;

// Mutable variables
let mut counter = 0;
counter = counter + 1;
```

### Functions

```rust
// Simple function
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// Type inference in parameters
fn greet(name: String) -> String {
    return "Hello, " + name + "!";
}

// Multiple return values
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero");
    }
    return Ok(a / b);
}
```

### Closures

```rust
// Closure syntax
let add_one = |x| x + 1;
let result = add_one(5);  // 6

// Multi-line closures
let process = |items| {
    let filtered = items.filter(|x| x > 0);
    return filtered.map(|x| x * 2);
};
```

### Structs and Enums

```rust
// Struct definition
struct User {
    name: String,
    age: i32,
    email: String,
}

// Creating instances
let user = User {
    name: "Alice",
    age: 30,
    email: "alice@example.com",
};

// Enum with variants
enum Status {
    Active,
    Inactive,
    Pending { since: String },
}

// Pattern matching
fn check_status(status: Status) -> String {
    match status {
        Status::Active => "User is active",
        Status::Inactive => "User is inactive",
        Status::Pending { since } => "Pending since " + since,
    }
}
```

## Core Concepts

### 1. Reactive State Management

RavensOne's reactivity system is built on Signals:

```rust
import { Signal, Computed, Effect } from "raven-reactive"

// Signal: reactive primitive
let count = Signal::new(0);
count.get();           // Read value
count.set(10);         // Update value
count.update(|n| n + 1);  // Transform value

// Computed: derived state
let doubled = Computed::new(|| count.get() * 2);

// Effect: side effects
Effect::new(|| {
    println!("Count is now: {}", count.get());
});
```

### 2. Components

Components are the building blocks of your UI:

```rust
// Props via function parameters
component Button(label: String, onclick: fn()) {
    <button onclick={onclick} class="btn">
        {label}
    </button>
}

// Using components
component App() {
    let count = Signal::new(0);

    <div>
        <Button
            label="Click me"
            onclick={() => count.set(count.get() + 1)}
        />
        <p>"Clicked: " {count.get()} " times"</p>
    </div>
}
```

### 3. Event Handling

```rust
component EventDemo() {
    let text = Signal::new("");

    <div>
        <input
            value={text.get()}
            oninput={(e) => text.set(e.target.value)}
        />
        <button onclick={() => text.set("")}>
            "Clear"
        </button>
        <p>"You typed: " {text.get()}</p>
    </div>
}
```

### 4. Server Functions

Write backend code alongside your frontend:

```rust
// Server-only function
#[server]
fn fetch_users() -> Vec<User> {
    // This runs on the server
    db::query("SELECT * FROM users").fetch_all()
}

// Call from client
component UserList() {
    let users = Resource::new(fetch_users);

    <div>
        {users.map(|user| {
            <div>{user.name}</div>
        })}
    </div>
}
```

### 5. Forms and Validation

```rust
import { Form, Field, Validators } from "raven-forms"

component LoginForm() {
    let form = Form::new();

    let email = Field::new("")
        .add_validator(Validators::required())
        .add_validator(Validators::email());

    let password = Field::new("")
        .add_validator(Validators::required())
        .add_validator(Validators::min_length(8));

    form.add_field("email", email);
    form.add_field("password", password);

    <form onsubmit={() => handle_submit(form)}>
        <input
            value={email.value.get()}
            oninput={(e) => email.set_value(e.target.value)}
        />
        {email.error.get().map(|err| <span class="error">{err}</span>)}

        <input
            type="password"
            value={password.value.get()}
            oninput={(e) => password.set_value(e.target.value)}
        />

        <button disabled={!form.is_valid()}>
            "Login"
        </button>
    </form>
}
```

## Building Your First App

Let's build a complete Todo application!

### Step 1: Define Data Structures

```rust
// src/models.raven
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

enum Filter {
    All,
    Active,
    Completed,
}
```

### Step 2: Create the Todo Component

```rust
// src/components/todo_item.raven
import { Signal } from "raven-reactive"

component TodoItem(todo: Todo, on_toggle: fn(i32), on_delete: fn(i32)) {
    <div class={if todo.completed { "todo completed" } else { "todo" }}>
        <input
            type="checkbox"
            checked={todo.completed}
            onchange={() => on_toggle(todo.id)}
        />
        <span>{todo.text}</span>
        <button onclick={() => on_delete(todo.id)}>
            "Delete"
        </button>
    </div>
}
```

### Step 3: Create the Main App

```rust
// src/main.raven
import { Signal, Computed } from "raven-reactive"
import { TodoItem } from "./components/todo_item"

component TodoApp() {
    let todos = Signal::new(Vec<Todo>::new());
    let filter = Signal::new(Filter::All);
    let input = Signal::new("");
    let next_id = Signal::new(1);

    // Computed filtered todos
    let filtered_todos = Computed::new(|| {
        let all = todos.get();
        match filter.get() {
            Filter::All => all,
            Filter::Active => all.filter(|t| !t.completed),
            Filter::Completed => all.filter(|t| t.completed),
        }
    });

    // Add todo
    let add_todo = || {
        if input.get().trim() != "" {
            let new_todo = Todo {
                id: next_id.get(),
                text: input.get(),
                completed: false,
            };
            todos.update(|list| list.push(new_todo));
            next_id.update(|id| id + 1);
            input.set("");
        }
    };

    // Toggle completion
    let toggle = |id| {
        todos.update(|list| {
            for todo in list.iter_mut() {
                if todo.id == id {
                    todo.completed = !todo.completed;
                }
            }
        });
    };

    // Delete todo
    let delete = |id| {
        todos.update(|list| list.retain(|t| t.id != id));
    };

    <div class="todo-app">
        <h1>"RavensOne Todo"</h1>

        <div class="input-section">
            <input
                value={input.get()}
                oninput={(e) => input.set(e.target.value)}
                onkeypress={(e) => if e.key == "Enter" { add_todo() }}
                placeholder="What needs to be done?"
            />
            <button onclick={add_todo}>"Add"</button>
        </div>

        <div class="filters">
            <button onclick={() => filter.set(Filter::All)}>
                "All"
            </button>
            <button onclick={() => filter.set(Filter::Active)}>
                "Active"
            </button>
            <button onclick={() => filter.set(Filter::Completed)}>
                "Completed"
            </button>
        </div>

        <div class="todo-list">
            {filtered_todos.get().map(|todo| {
                <TodoItem
                    todo={todo}
                    on_toggle={toggle}
                    on_delete={delete}
                />
            })}
        </div>

        <div class="footer">
            {todos.get().filter(|t| !t.completed).len()} " items left"
        </div>
    </div>
}

fn main() {
    mount(TodoApp(), "#app");
}
```

### Step 4: Add Styling

Create `public/style.css`:

```css
.todo-app {
    max-width: 600px;
    margin: 50px auto;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

.input-section {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
}

.input-section input {
    flex: 1;
    padding: 12px;
    font-size: 16px;
    border: 2px solid #e0e0e0;
    border-radius: 4px;
}

.input-section button {
    padding: 12px 24px;
    background: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.filters {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
}

.filters button {
    padding: 8px 16px;
    background: #f0f0f0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.todo {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px;
    border-bottom: 1px solid #e0e0e0;
}

.todo.completed span {
    text-decoration: line-through;
    color: #888;
}

.footer {
    margin-top: 20px;
    color: #888;
    font-size: 14px;
}
```

### Step 5: Compile and Run

```bash
# Compile with optimizations
raven compile src/main.raven --output dist/app.wasm --optimize

# Start dev server
raven dev

# Open http://localhost:3000
```

## Development Workflow

### Hot Module Replacement (HMR)

RavensOne includes built-in HMR for instant feedback:

```bash
# Start HMR server (default: localhost:3000, WS: 3001)
raven dev

# Customize ports
raven dev --port 8080 --ws-port 8081
```

Changes to `.raven` files automatically reload in the browser without losing state!

### Package Management

```bash
# Initialize package
raven pkg init

# Add dependencies
raven pkg add raven-ui
raven pkg add raven-router

# Install all dependencies
raven pkg install

# Update dependencies
raven pkg update

# Remove dependency
raven pkg remove raven-ui

# Show dependency tree
raven pkg tree

# Check for outdated packages
raven pkg outdated
```

### Using Community Packages

```toml
# raven.toml
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
raven-ui = "0.1.0"
raven-router = "0.1.0"
raven-forms = "0.1.0"
raven-http = "0.1.0"
```

```rust
// Import from packages
import { Button, Card, Modal } from "raven-ui"
import { Router, Route } from "raven-router"
import { Form, Field } from "raven-forms"
import { HttpClient } from "raven-http"
```

### Testing

```bash
# Run tests
cargo test

# Run specific test
cargo test test_counter

# Run with output
cargo test -- --nocapture
```

Write tests alongside your code:

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_signal_updates() {
    let count = Signal::new(0);
    count.set(10);
    assert_eq!(count.get(), 10);
}
```

### Performance Profiling

```bash
# Run with profiling enabled
raven compile src/main.raven --profile

# View profiler output
cat profiler_output.json
```

### Generate Documentation

```bash
# Generate HTML docs
raven doc src/main.raven --output docs/

# Open documentation
open docs/index.html
```

## Deployment

### Build for Production

```bash
# Full optimization build
raven compile src/main.raven \
    --output dist/app.wasm \
    --optimize \
    --target client

# Server build
raven compile src/main.raven \
    --output dist/server.js \
    --optimize \
    --target server
```

### Deploy to Vercel

1. Create `vercel.json`:

```json
{
  "version": 2,
  "builds": [
    {
      "src": "dist/**",
      "use": "@vercel/static"
    }
  ],
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/index.html"
    }
  ]
}
```

2. Deploy:

```bash
npm install -g vercel
vercel deploy
```

### Deploy to Netlify

1. Create `netlify.toml`:

```toml
[build]
  command = "raven compile src/main.raven --output dist/app.wasm --optimize"
  publish = "dist"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

2. Deploy:

```bash
npm install -g netlify-cli
netlify deploy --prod
```

### Deploy Server with Docker

Create `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM node:18-slim

WORKDIR /app
COPY --from=builder /app/target/release/raven /usr/local/bin/
COPY . .

RUN raven compile src/main.raven --output dist/server.js --target server

EXPOSE 3000
CMD ["node", "dist/server.js"]
```

Build and run:

```bash
docker build -t my-raven-app .
docker run -p 3000:3000 my-raven-app
```

## New in Phase 1 (v0.2.0)

RavensOne v0.2.0 brings 100% language completeness with many powerful features:

### Constants
```rust
const PI: f64 = 3.14159;
const MAX_USERS: i32 = 100;

fn calculate_area(radius: f64) -> f64 {
    return PI * radius * radius;
}
```

### Module Imports
```rust
// Named imports
use math::{PI, E, sin, cos};

// Wildcard imports
use collections::*;

// Namespaced access
use math;
let area = math::PI * radius * radius;
```

### Array Spread & Slicing
```rust
// Spread operator
let arr1 = vec![1, 2, 3];
let arr2 = vec![...arr1, 4, 5, 6];

// Slice syntax
let numbers = vec![1, 2, 3, 4, 5];
let subset = numbers[1..3];      // [2, 3]
let inclusive = numbers[1..=3];  // [2, 3, 4]
```

### Ternary Operator
```rust
let status = is_active ? "Active" : "Inactive";
let value = condition ? { let x = 5; x + 1 } : 10;
```

### Advanced Type Features
```rust
// Type casting
let x: f64 = 3.14;
let y = x as i32;  // 3

// Turbofish (explicit type parameters)
let num = "42".parse::<i32>();

// Function types
fn accepts_callback(callback: fn(i32) -> i32) {
    callback(42);
}
```

### Method Chaining
```rust
let result = "  hello world  "
    .trim()
    .to_uppercase()
    .replace(" ", "_");  // "HELLO_WORLD"
```

For complete details, see:
- **[Phase 1 Complete Summary](docs/PHASE_1_COMPLETE.md)** - All 15 sprints documented
- **[Stdlib API Reference](docs/guides/STDLIB_API_REFERENCE.md)** - 200+ functions
- **[CHANGELOG.md](CHANGELOG.md)** - Version history

---

## Current Limitations (Phase 4 - In Progress)

**RavensOne is in active development**. The following features are currently broken and being fixed:

### ❌ Not Working (Being Fixed)

- **if/else expressions** - Borrow checker bug blocks else clauses
  - ✅ Works: `if condition { code }`
  - ❌ Broken: `if condition { code } else { code }`

- **For loops with ranges** - Parser doesn't support range syntax
  - ❌ Broken: `for i in 1..10 { }`
  - ✅ Workaround: Use array iteration

- **Recursive functions** - Borrow checker bug
  - ❌ Broken: `fn factorial(n) { ... factorial(n-1) ... }`

- **Option and Result** - Depend on if/else (broken)
  - ❌ Broken: `Option<T>`, `Result<T, E>`, `Some`, `None`, `Ok`, `Err`

- **Match OR patterns** - Not implemented
  - ❌ Broken: `match x { 1 | 2 | 3 => ... }`

### ✅ What Works

- Functions with parameters and return types
- Arrays and array indexing
- Arithmetic operations (+, -, *, /, %)
- Boolean operations (&&, ||, ==, !=, <, >)
- Simple if statements (without else)
- println! with format strings
- JSX (fully working!)
- LSP features (editor support)

### 🔧 Current Focus

**Phase 4 Sprint 1** is fixing the critical borrow checker bug to enable:
- if/else expressions
- Recursive functions
- Option and Result types
- Error handling patterns

See `CLAUDE.md` for detailed Phase 4 roadmap and `SPRINT3_FINDINGS.md` for technical analysis.

---

## Next Steps

### Learn More

- **[Language Reference](LANGUAGE_REFERENCE.md)** - Complete syntax guide
- **[API Documentation](https://ravensone.dev/docs)** - Standard library docs
- **[Examples](examples/)** - Sample applications
- **[Roadmap](ROADMAP_Q1_2026.md)** - Upcoming features

### Explore the Ecosystem

- **raven-ui** - Production-ready UI components
- **raven-router** - Client-side routing
- **raven-forms** - Form handling and validation
- **raven-http** - HTTP client and server
- **raven-store** - Global state management
- **raven-i18n** - Internationalization

### Join the Community

- GitHub: https://github.com/yourusername/ravensone
- Discord: https://discord.gg/ravensone
- Twitter: @ravensone_lang

### Build Something Amazing

Now that you know the basics, start building! Here are some project ideas:

1. **Personal Blog** - Static site with markdown support
2. **E-commerce Store** - Full-stack shop with cart and checkout
3. **Real-time Chat** - WebSocket-based messaging app
4. **Dashboard** - Data visualization with charts
5. **Game** - Simple 2D game using Canvas API

Happy coding with RavensOne! 🦅
