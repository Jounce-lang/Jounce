# Jounce Standard Library Tutorial

**Welcome to the Jounce Standard Library Tutorial!**

This hands-on guide will teach you how to use Jounce's powerful standard library to build real applications. We'll start with simple examples and gradually build more complex features.

---

## Prerequisites

- Basic understanding of programming (variables, functions, loops)
- Jounce compiler installed (`./target/release/raven`)
- Text editor

**No prior Jounce experience needed!** We'll learn together.

---

## Table of Contents

1. [Getting Started: Your First Program](#lesson-1-getting-started)
2. [Math: Calculations and Numbers](#lesson-2-math-calculations)
3. [Reactive Programming: Making UIs Interactive](#lesson-3-reactive-programming)
4. [Building a Counter App](#lesson-4-building-a-counter)
5. [HTTP: Fetching Data from APIs](#lesson-5-http-requests)
6. [Forms and Validation](#lesson-6-forms-and-validation)
7. [Building a Todo List](#lesson-7-todo-list-app)
8. [Real-World Patterns](#lesson-8-real-world-patterns)

---

## Lesson 1: Getting Started

### Your First Jounce Program

Let's start with the classic "Hello World" and learn basic concepts.

**Create a file**: `tutorial_01_hello.jnc`

```raven
fn main() {
    println!("Hello, Jounce!");
}
```

**Compile and run**:
```bash
./target/release/raven compile tutorial_01_hello.jnc
cd dist && node server.js
```

**Output**: `Hello, Jounce!`

---

### Understanding Functions

Functions are blocks of reusable code. Let's create some:

```raven
fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    greet("Alice");
    greet("Bob");

    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
}
```

**Key Concepts**:
- `fn` defines a function
- Parameters have types (`name: String`, `a: i32`)
- `-> i32` means the function returns an integer
- `return` gives back a value

**Exercise**: Create a function `multiply(a: f64, b: f64) -> f64` that multiplies two numbers.

<details>
<summary>Solution</summary>

```raven
fn multiply(a: f64, b: f64) -> f64 {
    return a * b;
}

fn main() {
    let result = multiply(4.5, 2.0);
    println!("4.5 * 2.0 = {}", result);
}
```
</details>

---

## Lesson 2: Math Calculations

Now let's use the Math library to do more interesting calculations.

### Basic Math Operations

**Create**: `tutorial_02_math.jnc`

```raven
fn main() {
    println!("=== Math Tutorial ===\n");

    // Absolute value
    let negative = -42.5;
    let positive = Math::abs(negative);
    println!("abs(-42.5) = {}", positive);

    // Min and max
    let smaller = Math::min(10.0, 20.0);
    let larger = Math::max(10.0, 20.0);
    println!("min(10, 20) = {}", smaller);
    println!("max(10, 20) = {}", larger);

    // Power and square root
    let squared = Math::pow(5.0, 2.0);
    let root = Math::sqrt(25.0);
    println!("5^2 = {}", squared);
    println!("√25 = {}", root);

    // Rounding
    let rounded = Math::round(3.7);
    let floored = Math::floor(3.7);
    let ceiled = Math::ceil(3.7);
    println!("round(3.7) = {}", rounded);
    println!("floor(3.7) = {}", floored);
    println!("ceil(3.7) = {}", ceiled);
}
```

**Output**:
```
=== Math Tutorial ===

abs(-42.5) = 42.5
min(10, 20) = 10
max(10, 20) = 20
5^2 = 25
√25 = 5
round(3.7) = 4
floor(3.7) = 3
ceil(3.7) = 4
```

---

### Practical Example: Circle Calculator

Let's build something useful - a circle calculator.

```raven
fn calculate_circle_area(radius: f64) -> f64 {
    return Math::PI * Math::square(radius);
}

fn calculate_circle_circumference(radius: f64) -> f64 {
    return 2.0 * Math::PI * radius;
}

fn main() {
    println!("=== Circle Calculator ===\n");

    let radius = 5.0;

    let area = calculate_circle_area(radius);
    let circumference = calculate_circle_circumference(radius);

    println!("Radius: {}", radius);
    println!("Area: {:.2}", area);
    println!("Circumference: {:.2}", circumference);
}
```

**Key Concepts**:
- `Math::PI` is a constant (π ≈ 3.14159)
- `Math::square(x)` is shorthand for x²
- `{:.2}` formats to 2 decimal places

**Exercise**: Add a function `calculate_sphere_volume(radius: f64)` that calculates sphere volume (formula: 4/3 × π × r³).

<details>
<summary>Solution</summary>

```raven
fn calculate_sphere_volume(radius: f64) -> f64 {
    return (4.0 / 3.0) * Math::PI * Math::cube(radius);
}

fn main() {
    let radius = 5.0;
    let volume = calculate_sphere_volume(radius);
    println!("Sphere volume: {:.2}", volume);
}
```
</details>

---

### Random Numbers

Random numbers are useful for games, simulations, and testing.

```raven
fn main() {
    println!("=== Random Numbers ===\n");

    // Random 0 to 1
    let random = Math::random();
    println!("Random 0-1: {}", random);

    // Random in range
    let temp = Math::random_range(20.0, 30.0);
    println!("Random temperature (20-30°C): {:.1}", temp);

    // Random integer (dice roll)
    let dice = Math::random_int(1, 6);
    println!("Dice roll: {}", dice);

    // Coin flip
    let coin = if Math::random() < 0.5 { "Heads" } else { "Tails" };
    println!("Coin flip: {}", coin);
}
```

**Exercise**: Create a function `roll_two_dice() -> i32` that simulates rolling two dice and returns their sum.

<details>
<summary>Solution</summary>

```raven
fn roll_two_dice() -> i32 {
    let die1 = Math::random_int(1, 6);
    let die2 = Math::random_int(1, 6);
    return die1 + die2;
}

fn main() {
    let total = roll_two_dice();
    println!("You rolled: {}", total);
}
```
</details>

---

## Lesson 3: Reactive Programming

**Reactive programming** automatically updates your UI when data changes. This is the foundation of modern web apps.

### Signals: Reactive State

A **Signal** is a value that notifies listeners when it changes.

**Create**: `tutorial_03_reactive.jnc`

```raven
fn main() {
    println!("=== Signals ===\n");

    // Create a signal with initial value
    let count = Signal::new(0);

    // Get the current value
    println!("Initial count: {}", count.get());

    // Set a new value
    count.set(5);
    println!("After set(5): {}", count.get());

    // Increment
    count.set(count.get() + 1);
    println!("After increment: {}", count.get());
}
```

**Output**:
```
=== Signals ===

Initial count: 0
After set(5): 5
After increment: 6
```

**Key Concepts**:
- `Signal::new(value)` creates a signal
- `.get()` reads the current value
- `.set(value)` updates the value

---

### Computed: Derived Values

A **Computed** value automatically updates when its dependencies change.

```raven
fn main() {
    println!("=== Computed Values ===\n");

    let count = Signal::new(10);

    // Computed values derive from signals
    let doubled = Computed::new(|| {
        return count.get() * 2;
    });

    let tripled = Computed::new(|| {
        return count.get() * 3;
    });

    println!("count = {}", count.get());
    println!("doubled = {}", doubled.get());
    println!("tripled = {}", tripled.get());

    // Change the signal
    count.set(20);

    println!("\nAfter count.set(20):");
    println!("count = {}", count.get());
    println!("doubled = {}", doubled.get());  // Automatically updated!
    println!("tripled = {}", tripled.get());  // Automatically updated!
}
```

**Output**:
```
=== Computed Values ===

count = 10
doubled = 20
tripled = 30

After count.set(20):
count = 20
doubled = 40
tripled = 60
```

**Key Concept**: Computed values **automatically recalculate** when their dependencies change.

---

### Effects: Side Effects

An **Effect** runs code when signals or computed values change.

```raven
fn main() {
    println!("=== Effects ===\n");

    let name = Signal::new("Alice");

    // Effect runs immediately and when 'name' changes
    create_effect(|| {
        println!("[Effect] Hello, {}!", name.get());
    });

    // Change the signal
    name.set("Bob");
    name.set("Charlie");
}
```

**Output**:
```
=== Effects ===

[Effect] Hello, Alice!
[Effect] Hello, Bob!
[Effect] Hello, Charlie!
```

**Key Concepts**:
- Effects run **immediately** when created
- Effects run **again** whenever dependencies change
- Use effects for logging, DOM updates, analytics

---

### Putting It Together: Reactive Counter

Let's combine signals, computed, and effects:

```raven
fn main() {
    println!("=== Reactive Counter ===\n");

    // State
    let count = Signal::new(0);

    // Derived state
    let is_even = Computed::new(|| {
        return count.get() % 2 == 0;
    });

    let is_positive = Computed::new(|| {
        return count.get() > 0;
    });

    // Side effect - logging
    create_effect(|| {
        let value = count.get();
        let even_str = if is_even.get() { "even" } else { "odd" };
        let sign_str = if is_positive.get() { "positive" } else { "non-positive" };

        println!("Count: {} ({}, {})", value, even_str, sign_str);
    });

    // Simulate user interactions
    count.set(1);   // Triggers effect
    count.set(2);   // Triggers effect
    count.set(-1);  // Triggers effect
    count.set(0);   // Triggers effect
}
```

**Output**:
```
=== Reactive Counter ===

Count: 0 (even, non-positive)
Count: 1 (odd, positive)
Count: 2 (even, positive)
Count: -1 (odd, non-positive)
Count: 0 (even, non-positive)
```

**Exercise**: Add a computed value `absolute_value` that shows the absolute value of the count.

<details>
<summary>Solution</summary>

```raven
let absolute_value = Computed::new(|| {
    return Math::abs(count.get() as f64) as i32;
});

create_effect(|| {
    println!("Absolute: {}", absolute_value.get());
});
```
</details>

---

## Lesson 4: Building a Counter

Let's build a real interactive counter component!

**Create**: `tutorial_04_counter.jnc`

```raven
component Counter() {
    let count = Signal::new(0);

    let doubled = Computed::new(|| {
        return count.get() * 2;
    });

    <div class="counter">
        <h1>"Counter App"</h1>

        <div class="display">
            <p>"Count: " {count.get()}</p>
            <p>"Doubled: " {doubled.get()}</p>
        </div>

        <div class="buttons">
            <button onclick={() => count.set(count.get() + 1)}>
                "Increment"
            </button>

            <button onclick={() => count.set(count.get() - 1)}>
                "Decrement"
            </button>

            <button onclick={() => count.set(0)}>
                "Reset"
            </button>
        </div>
    </div>
}

fn main() {
    <Counter />
}
```

**Compile and view**:
```bash
./target/release/raven compile tutorial_04_counter.jnc
cd dist && node server.js
# Open http://localhost:3000 in browser
```

**Key Concepts**:
- `component` defines a reusable UI component
- JSX syntax: `<div>`, `<button>`, etc.
- `onclick={() => ...}` handles click events
- `{count.get()}` embeds reactive values in JSX
- When signal changes, UI **automatically updates**

**Exercise**: Add a button that increments by 5.

<details>
<summary>Solution</summary>

```raven
<button onclick={() => count.set(count.get() + 5)}>
    "+5"
</button>
```
</details>

---

### Adding More Features

Let's make the counter more interesting:

```raven
component FancyCounter() {
    let count = Signal::new(0);

    let is_even = Computed::new(|| count.get() % 2 == 0);
    let is_positive = Computed::new(|| count.get() > 0);
    let squared = Computed::new(|| count.get() * count.get());

    <div class="counter">
        <h1>"Fancy Counter"</h1>

        <div class="stats">
            <p>"Count: " {count.get()}</p>
            <p>"Squared: " {squared.get()}</p>
            <p>"Even? " {if is_even.get() { "Yes" } else { "No" }}</p>
            <p>"Positive? " {if is_positive.get() { "Yes" } else { "No" }}</p>
        </div>

        <div class="buttons">
            <button onclick={() => count.set(count.get() + 1)}>"+"</button>
            <button onclick={() => count.set(count.get() - 1)}>"-"</button>
            <button onclick={() => count.set(count.get() * 2)}>"×2"</button>
            <button onclick={() => count.set(0)}>"Reset"</button>
        </div>
    </div>
}
```

**Exercise**: Add a "÷2" button that divides the count by 2 (integer division).

<details>
<summary>Solution</summary>

```raven
<button onclick={() => count.set(count.get() / 2)}>"÷2"</button>
```
</details>

---

## Lesson 5: HTTP Requests

Let's fetch data from the internet!

**Important**: HTTP functions only work on the **server**, so we use `@server`.

**Create**: `tutorial_05_http.jnc`

```raven
@server
async fn fetch_user_data() {
    println!("=== Fetching User Data ===\n");

    let response = HttpRequest::get("https://jsonplaceholder.typicode.com/users/1")
        .send()
        .await;

    match response {
        Ok(resp) => {
            println!("Status: {}", resp.status);

            if resp.is_ok() {
                match resp.json() {
                    Ok(user) => {
                        let name = user["name"].as_str().unwrap();
                        let email = user["email"].as_str().unwrap();

                        println!("Name: {}", name);
                        println!("Email: {}", email);
                    }
                    Err(err) => {
                        println!("JSON parse error: {}", err);
                    }
                }
            } else {
                println!("Request failed with status {}", resp.status);
            }
        }
        Err(error) => {
            println!("Network error: {}", error);
        }
    }
}

@server
async fn main() {
    fetch_user_data().await;
}
```

**Key Concepts**:
- `@server` means code runs on server (has network access)
- `async fn` means function is asynchronous
- `.await` waits for async operation to complete
- `match` handles success (Ok) and error (Err) cases
- `.json()` parses response as JSON

---

### POST Request: Creating Data

Let's send data to a server:

```raven
@server
async fn create_post() {
    println!("=== Creating Post ===\n");

    let post_data = json!({
        "title": "My First Post",
        "body": "This is the content of my post.",
        "userId": 1
    });

    let response = HttpRequest::post("https://jsonplaceholder.typicode.com/posts")
        .json(post_data)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.is_ok() {
                println!("Post created! Status: {}", resp.status);

                match resp.json() {
                    Ok(created) => {
                        let id = created["id"].as_i64().unwrap();
                        println!("New post ID: {}", id);
                    }
                    Err(_) => println!("Could not parse response"),
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

@server
async fn main() {
    create_post().await;
}
```

**Key Concepts**:
- `json!({ ... })` creates JSON data
- `HttpRequest::post(url)` creates POST request
- `.json(data)` sets request body

---

### Combining Client and Server

Jounce's superpower: **automatic RPC** between client and server!

```raven
@server
async fn fetch_random_user() -> String {
    let user_id = Math::random_int(1, 10);
    let url = format!("https://jsonplaceholder.typicode.com/users/{}", user_id);

    let response = HttpRequest::get(&url).send().await;

    match response {
        Ok(resp) => {
            if resp.is_ok() {
                match resp.json() {
                    Ok(user) => {
                        let name = user["name"].as_str().unwrap();
                        return name.to_string();
                    }
                    Err(_) => return "Error parsing user".to_string(),
                }
            } else {
                return "Error fetching user".to_string();
            }
        }
        Err(_) => return "Network error".to_string(),
    }
}

@client
component UserFetcher() {
    let user_name = Signal::new("(not loaded)");
    let is_loading = Signal::new(false);

    let load_user = || {
        is_loading.set(true);

        // Call server function from client!
        fetch_random_user().then(|name| {
            user_name.set(name);
            is_loading.set(false);
        });
    };

    <div>
        <h1>"Random User Fetcher"</h1>

        <p>"User: " {user_name.get()}</p>

        <button onclick={load_user}>
            {if is_loading.get() { "Loading..." } else { "Fetch User" }}
        </button>
    </div>
}

fn main() {
    <UserFetcher />
}
```

**Key Concepts**:
- `@server` function can be called from `@client` code
- Jounce **automatically generates RPC**
- `.then(|result| ...)` handles async result
- No manual API endpoints needed!

---

## Lesson 6: Forms and Validation

Let's build a login form with validation.

**Create**: `tutorial_06_forms.jnc`

```raven
@client
component LoginForm() {
    let email = Signal::new("");
    let password = Signal::new("");
    let errors = Signal::new(vec![]);

    let is_valid = Computed::new(|| {
        return Forms::validate_email(&email.get()) &&
               Forms::validate_min_length(&password.get(), 8);
    });

    let validate_and_submit = || {
        let mut error_list = vec![];

        // Validate email
        if !Forms::validate_required(&email.get()) {
            error_list.push("Email is required");
        } else if !Forms::validate_email(&email.get()) {
            error_list.push("Invalid email format");
        }

        // Validate password
        if !Forms::validate_required(&password.get()) {
            error_list.push("Password is required");
        } else if !Forms::validate_min_length(&password.get(), 8) {
            error_list.push("Password must be at least 8 characters");
        }

        if error_list.is_empty() {
            println!("Form is valid! Submitting...");
            // Call login server function here
        } else {
            errors.set(error_list);
        }
    };

    <div class="login-form">
        <h1>"Login"</h1>

        <div class="field">
            <label>"Email"</label>
            <input
                type="email"
                value={email.get()}
                oninput={(e) => email.set(e.target.value)}
            />
        </div>

        <div class="field">
            <label>"Password"</label>
            <input
                type="password"
                value={password.get()}
                oninput={(e) => password.set(e.target.value)}
            />
        </div>

        {if !errors.get().is_empty() {
            <div class="errors">
                {errors.get().iter().map(|err| {
                    <p class="error">{err}</p>
                })}
            </div>
        }}

        <button
            onclick={validate_and_submit}
            disabled={!is_valid.get()}
        >
            "Login"
        </button>
    </div>
}

fn main() {
    <LoginForm />
}
```

**Key Concepts**:
- `Forms::validate_email()` validates email format
- `Forms::validate_min_length()` checks minimum length
- `oninput` updates signal on user typing
- `disabled={!is_valid.get()}` disables button when form invalid
- Error messages shown reactively

**Exercise**: Add a "Confirm Password" field that must match the password field.

<details>
<summary>Solution</summary>

```raven
let password_confirm = Signal::new("");

// In validation:
if password.get() != password_confirm.get() {
    error_list.push("Passwords do not match");
}

// In JSX:
<div class="field">
    <label>"Confirm Password"</label>
    <input
        type="password"
        value={password_confirm.get()}
        oninput={(e) => password_confirm.set(e.target.value)}
    />
</div>
```
</details>

---

## Lesson 7: Todo List App

Let's build a complete todo list application combining everything we've learned!

**Create**: `tutorial_07_todo.jnc`

```raven
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

@client
component TodoApp() {
    let todos = ReactiveVec::new();
    let input = Signal::new("");
    let next_id = Signal::new(1);

    // Computed: counts
    let total_count = Computed::new(|| {
        return todos.len();
    });

    let completed_count = Computed::new(|| {
        let mut count = 0;
        for todo in todos.iter() {
            if todo.completed {
                count = count + 1;
            }
        }
        return count;
    });

    let active_count = Computed::new(|| {
        return total_count.get() - completed_count.get();
    });

    // Actions
    let add_todo = || {
        let text = input.get().trim();
        if text.is_empty() {
            return;
        }

        let new_todo = Todo {
            id: next_id.get(),
            text: text.to_string(),
            completed: false,
        };

        todos.push(new_todo);
        next_id.set(next_id.get() + 1);
        input.set("");
    };

    let toggle_todo = |id: i32| {
        for i in 0..todos.len() {
            if todos[i].id == id {
                todos[i].completed = !todos[i].completed;
                todos.notify();  // Trigger reactivity
                break;
            }
        }
    };

    let delete_todo = |id: i32| {
        for i in 0..todos.len() {
            if todos[i].id == id {
                todos.remove(i);
                break;
            }
        }
    };

    <div class="todo-app">
        <h1>"Todo List"</h1>

        <div class="stats">
            <p>"Total: " {total_count.get()}</p>
            <p>"Active: " {active_count.get()}</p>
            <p>"Completed: " {completed_count.get()}</p>
        </div>

        <div class="add-todo">
            <input
                type="text"
                placeholder="What needs to be done?"
                value={input.get()}
                oninput={(e) => input.set(e.target.value)}
                onkeypress={(e) => {
                    if e.key == "Enter" {
                        add_todo();
                    }
                }}
            />
            <button onclick={add_todo}>"Add"</button>
        </div>

        <div class="todo-list">
            {todos.iter().map(|todo| {
                <div class={if todo.completed { "todo completed" } else { "todo" }}>
                    <input
                        type="checkbox"
                        checked={todo.completed}
                        onchange={() => toggle_todo(todo.id)}
                    />

                    <span class="text">{todo.text}</span>

                    <button
                        class="delete"
                        onclick={() => delete_todo(todo.id)}
                    >
                        "×"
                    </button>
                </div>
            })}
        </div>
    </div>
}

fn main() {
    <TodoApp />
}
```

**Key Concepts**:
- `ReactiveVec` is a reactive array
- `struct` defines custom data types
- `.iter().map()` renders list of items
- State management with multiple signals
- Computed values for derived stats

**Exercise**: Add a "Clear Completed" button that removes all completed todos.

<details>
<summary>Solution</summary>

```raven
let clear_completed = || {
    let mut i = 0;
    while i < todos.len() {
        if todos[i].completed {
            todos.remove(i);
        } else {
            i = i + 1;
        }
    }
};

// In JSX:
<button onclick={clear_completed}>"Clear Completed"</button>
```
</details>

---

### Adding Persistence

Let's save todos to localStorage so they persist across page reloads:

```raven
@client
component PersistentTodoApp() {
    let todos = ReactiveVec::new();
    let input = Signal::new("");
    let next_id = Signal::new(1);

    // Load from storage on init
    let init = || {
        match Storage::get_json("todos") {
            Some(data) => {
                // Parse and populate todos
                let saved_todos = data["todos"].as_array().unwrap();
                for todo_data in saved_todos {
                    let todo = Todo {
                        id: todo_data["id"].as_i64().unwrap() as i32,
                        text: todo_data["text"].as_str().unwrap().to_string(),
                        completed: todo_data["completed"].as_bool().unwrap(),
                    };
                    todos.push(todo);
                }

                let max_id = data["next_id"].as_i64().unwrap() as i32;
                next_id.set(max_id);
            }
            None => {
                println!("No saved todos");
            }
        }
    };

    // Save to storage when todos change
    create_effect(|| {
        let todos_data = todos.iter().map(|t| {
            json!({
                "id": t.id,
                "text": t.text,
                "completed": t.completed
            })
        }).collect();

        let data = json!({
            "todos": todos_data,
            "next_id": next_id.get()
        });

        Storage::set_json("todos", &data);
    });

    init();  // Load on startup

    // ... rest of component (same as before)
}
```

**Key Concepts**:
- `Storage::get_json()` loads from localStorage
- `Storage::set_json()` saves to localStorage
- Effect automatically saves when todos change
- Data persists across page reloads

---

## Lesson 8: Real-World Patterns

### Pattern 1: API Client Wrapper

Create a reusable API client:

```raven
@server
struct ApiClient {
    base_url: String,
    api_key: String,
}

impl ApiClient {
    @server
    fn new(base_url: String, api_key: String) -> ApiClient {
        return ApiClient {
            base_url: base_url,
            api_key: api_key,
        };
    }

    @server
    async fn get(&self, path: String) -> Result<Value, String> {
        let url = format!("{}{}", self.base_url, path);

        let response = HttpRequest::get(&url)
            .header("X-API-Key", &self.api_key)
            .header("Accept", "application/json")
            .timeout(10000)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if !resp.is_ok() {
                    return Err(format!("HTTP {}", resp.status));
                }
                return resp.json();
            }
            Err(err) => return Err(err),
        }
    }

    @server
    async fn post(&self, path: String, data: Value) -> Result<Value, String> {
        let url = format!("{}{}", self.base_url, path);

        let response = HttpRequest::post(&url)
            .header("X-API-Key", &self.api_key)
            .json(data)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if !resp.is_ok() {
                    return Err(format!("HTTP {}", resp.status));
                }
                return resp.json();
            }
            Err(err) => return Err(err),
        }
    }
}

@server
async fn example_usage() {
    let api = ApiClient::new(
        "https://api.example.com".to_string(),
        "my-api-key".to_string()
    );

    // GET request
    match api.get("/users".to_string()).await {
        Ok(users) => println!("Users: {:?}", users),
        Err(err) => println!("Error: {}", err),
    }

    // POST request
    let new_user = json!({
        "name": "Alice",
        "email": "alice@example.com"
    });

    match api.post("/users".to_string(), new_user).await {
        Ok(created) => println!("Created: {:?}", created),
        Err(err) => println!("Error: {}", err),
    }
}
```

---

### Pattern 2: Loading States

Handle async operations with loading/error states:

```raven
@client
component DataFetcher() {
    let data = Signal::new(None);
    let is_loading = Signal::new(false);
    let error = Signal::new(None);

    let fetch_data = || {
        is_loading.set(true);
        error.set(None);

        fetch_users().then(|result| {
            is_loading.set(false);

            match result {
                Ok(users) => {
                    data.set(Some(users));
                }
                Err(err) => {
                    error.set(Some(err));
                }
            }
        });
    };

    <div>
        <h1>"User List"</h1>

        {if is_loading.get() {
            <div class="loading">"Loading..."</div>
        } else if error.get().is_some() {
            <div class="error">
                "Error: " {error.get().unwrap()}
            </div>
        } else if data.get().is_some() {
            <div class="users">
                {data.get().unwrap().iter().map(|user| {
                    <div class="user">{user.name}</div>
                })}
            </div>
        } else {
            <button onclick={fetch_data}>"Load Users"</button>
        }}
    </div>
}
```

---

### Pattern 3: Form with Server Validation

Combine client and server validation:

```raven
@server
async fn validate_username(username: String) -> Result<(), String> {
    // Server-side validation
    if username.len() < 3 {
        return Err("Username too short".to_string());
    }

    // Check database (simulated)
    let exists = db_check_username_exists(&username);
    if exists {
        return Err("Username already taken".to_string());
    }

    return Ok(());
}

@client
component RegistrationForm() {
    let username = Signal::new("");
    let username_error = Signal::new(None);
    let is_checking = Signal::new(false);

    let check_username = || {
        let name = username.get();

        // Client-side validation first
        if name.len() < 3 {
            username_error.set(Some("Too short".to_string()));
            return;
        }

        // Server-side validation
        is_checking.set(true);
        validate_username(name).then(|result| {
            is_checking.set(false);

            match result {
                Ok(_) => username_error.set(None),
                Err(err) => username_error.set(Some(err)),
            }
        });
    };

    <div>
        <input
            type="text"
            value={username.get()}
            oninput={(e) => {
                username.set(e.target.value);
                check_username();  // Check on every change
            }}
        />

        {if is_checking.get() {
            <span class="checking">"Checking..."</span>
        } else if username_error.get().is_some() {
            <span class="error">{username_error.get().unwrap()}</span>
        } else if username.get().len() >= 3 {
            <span class="success">"✓ Available"</span>
        }}
    </div>
}
```

---

## Next Steps

Congratulations! You've learned:

✅ Basic Jounce syntax and functions
✅ Math library for calculations
✅ Reactive programming with Signal, Computed, Effect
✅ Building interactive UIs with components
✅ HTTP requests and API integration
✅ Form validation
✅ Complete applications (Counter, Todo List)
✅ Real-world patterns

### Continue Learning

1. **Explore More Examples**
   - See `examples/stdlib/` for comprehensive stdlib examples
   - Check `examples/apps/` for larger applications

2. **Read the API Reference**
   - `docs/guides/STDLIB_API_REFERENCE.md` - Complete function reference
   - Covers all 16 stdlib modules

3. **Build Your Own Projects**
   - Start small: calculator, timer, note-taking app
   - Combine features: todo list + authentication + API sync
   - Share your creations!

4. **Advanced Topics**
   - WebSockets for real-time features
   - Database integration
   - Authentication and security
   - Deployment to production

---

## Common Issues & Solutions

### Issue: "Cannot find module"

**Problem**: Trying to import modules (not yet supported)
**Solution**: All stdlib is auto-imported, just use `Math::`, `Signal::`, etc.

---

### Issue: HTTP requests not working

**Problem**: HTTP code running on client instead of server
**Solution**: Add `@server` annotation to async HTTP functions

```raven
@server  // ← Add this!
async fn fetch_data() {
    let resp = HttpRequest::get(url).send().await;
}
```

---

### Issue: Component not re-rendering

**Problem**: Forgot to use `.get()` to track dependency
**Solution**: Always use `.get()` to read signals in components

```raven
// ❌ Wrong - component won't update
<p>{count}</p>

// ✅ Correct - component updates when count changes
<p>{count.get()}</p>
```

---

### Issue: "ReactiveVec not updating"

**Problem**: Modified vec directly without calling `.notify()`
**Solution**: Call `.notify()` after direct mutations

```raven
// ❌ Wrong
todos[0].completed = true;  // No reactivity

// ✅ Correct
todos[0].completed = true;
todos.notify();  // Triggers updates
```

---

## Tips for Success

1. **Start Small**: Build simple examples before complex apps
2. **Use Computed Values**: Let Jounce handle derived state
3. **Think Reactively**: Data flows down, events flow up
4. **Handle Errors**: Always use `match` for Result types
5. **Test Incrementally**: Compile and test after each feature
6. **Read Examples**: Learn from working code in `examples/`
7. **Ask Questions**: Check documentation and community

---

## Quick Reference Card

### Creating Signals
```raven
let count = Signal::new(0);
let name = Signal::new("Alice");
```

### Reading Signals
```raven
let value = count.get();
println!("{}", name.get());
```

### Updating Signals
```raven
count.set(10);
name.set("Bob");
```

### Computed Values
```raven
let doubled = Computed::new(|| count.get() * 2);
```

### Effects
```raven
create_effect(|| {
    println!("Count: {}", count.get());
});
```

### HTTP GET
```raven
@server
async fn fetch() {
    let resp = HttpRequest::get(url).send().await;
}
```

### HTTP POST
```raven
@server
async fn create() {
    let data = json!({"key": "value"});
    let resp = HttpRequest::post(url).json(data).send().await;
}
```

### Form Validation
```raven
let valid = Forms::validate_email(&email);
```

### Storage
```raven
Storage::set("key", "value");
let value = Storage::get("key");
```

---

**Happy coding with Jounce! 🚀**

For more help:
- API Reference: `docs/guides/STDLIB_API_REFERENCE.md`
- Examples: `examples/stdlib/`
- Getting Started: `GETTING_STARTED.md`
