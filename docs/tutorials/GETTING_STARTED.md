# Getting Started with Jounce

**Welcome to Jounce!** This tutorial will guide you through installing Jounce, writing your first application, and exploring the powerful features that make Jounce a modern, fast, and expressive programming language.

## What is Jounce?

Jounce is a modern programming language that combines:
- **Type-safe** Rust-inspired syntax
- **Fast compilation** with caching (100x+ faster builds)
- **Full-stack development** with JSX and CSS support
- **Rich standard library** (JSON, DateTime, Crypto, File I/O, YAML)
- **WebAssembly compilation** for high performance
- **Built-in testing** framework

## Installation

### Prerequisites
- Rust toolchain (1.70+)
- Node.js (for running compiled output)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/jounce.git
cd jounce

# Build the compiler
cargo build --release

# Add to PATH (optional)
export PATH="$PATH:$(pwd)/target/release"
```

### Verify Installation

```bash
jnc --version
# Should output: jnc 0.3.0
```

## Your First Jounce Program

### 1. Create a New File

Create a file called `hello.jnc`:

```jounce
fn main() {
    console.log("Hello, Jounce!");
}
```

### 2. Compile and Run

```bash
# Compile to WebAssembly and JavaScript
jnc compile hello.jnc

# Run the output
cd dist && node server.js
```

You should see: `Hello, Jounce!`

## Core Language Features

### Variables and Types

```jounce
// Immutable by default
let name = "Alice";
let age = 30;
let pi = 3.14159;

// Mutable variables
let mut count = 0;
count = count + 1;

// Explicit types
let number: i64 = 42;
let text: String = "Hello";
let flag: bool = true;
```

### Functions

```jounce
// Basic function
fn greet(name: String) -> String {
    return "Hello, " + name + "!";
}

// Function with multiple parameters
fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

// No return value (unit type)
fn log_message(msg: String) {
    console.log(msg);
}

fn main() {
    let greeting = greet("World");
    console.log(greeting);  // "Hello, World!"

    let sum = add(10, 20);
    console.log(sum.to_string());  // "30"
}
```

### Control Flow

```jounce
// If expressions
fn check_age(age: i64) -> String {
    if age >= 18 {
        return "Adult";
    } else {
        return "Minor";
    }
}

// Loops
fn count_to_ten() {
    let mut i = 1;
    while i <= 10 {
        console.log(i.to_string());
        i = i + 1;
    }
}

// For loops
fn iterate_numbers() {
    for i in 0..5 {
        console.log("Number: " + i.to_string());
    }
}
```

### Pattern Matching

```jounce
enum Status {
    Success,
    Error(String),
    Pending,
}

fn handle_status(status: Status) -> String {
    match status {
        Status::Success => return "Operation succeeded!",
        Status::Error(msg) => return "Error: " + msg,
        Status::Pending => return "Still processing...",
    }
}
```

### Option and Result Types

```jounce
// Option for values that might not exist
fn find_user(id: i64) -> Option<String> {
    if id == 1 {
        return Option::Some("Alice");
    }
    return Option::None;
}

// Result for operations that can fail
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Result::Err("Division by zero");
    }
    return Result::Ok(a / b);
}

fn main() {
    // Using Option
    let user = find_user(1);
    match user {
        Option::Some(name) => console.log("Found: " + name),
        Option::None => console.log("User not found"),
    }

    // Using Result with try operator
    let result = divide(10.0, 2.0)?;
    console.log("Result: " + result.to_string());
}
```

## Working with the Standard Library

### JSON Module

```jounce
fn parse_json_example() {
    let json_str = "{\"name\": \"Alice\", \"age\": 30}";
    let result = json::parse(json_str);

    if result.is_ok() {
        let value = result.unwrap();
        let name = value.get("name").unwrap().as_string().unwrap();
        console.log("Name: " + name);  // "Name: Alice"
    }
}

fn create_json_example() {
    let mut obj = json::json_object();
    obj.set("title", json::json_string("Hello World"));
    obj.set("count", json::json_number(42.0));

    let json_str = json::stringify(obj);
    console.log(json_str);  // {"title":"Hello World","count":42}
}
```

### DateTime Module

```jounce
fn datetime_example() {
    // Create duration
    let one_hour = time::Duration::from_hours(1);
    let thirty_mins = time::Duration::from_minutes(30);

    // Date and time operations
    let now = time::DateTime::now();
    let later = now.add_duration(one_hour);

    console.log("Now: " + now.format("%Y-%m-%d %H:%M:%S"));
    console.log("Later: " + later.format("%Y-%m-%d %H:%M:%S"));
}
```

### YAML Module

```jounce
fn yaml_example() {
    // Parse YAML
    let yaml_str = "{name: Bob, age: 25, active: true}";
    let result = yaml::parse(yaml_str).unwrap();

    let name = result.get("name").unwrap().as_string().unwrap();
    let age = result.get("age").unwrap().as_number().unwrap();

    console.log("Name: " + name);
    console.log("Age: " + age.to_string());

    // Create YAML
    let mut map = yaml::yaml_mapping();
    map.set("greeting", yaml::yaml_string("Hello"));
    map.set("count", yaml::yaml_number(42.0));

    let yaml_output = yaml::stringify(map);
    console.log(yaml_output);
}
```

### File I/O

```jounce
fn file_example() {
    // Write file
    let content = "Hello, file system!";
    let write_result = fs::write_file("output.txt", content);

    if write_result.is_ok() {
        console.log("File written successfully");
    }

    // Read file
    let read_result = fs::read_file("output.txt");
    if read_result.is_ok() {
        let data = read_result.unwrap();
        console.log("File contents: " + data);
    }

    // Check if file exists
    if fs::file_exists("output.txt") {
        console.log("File exists!");
    }
}
```

### Cryptography

```jounce
fn crypto_example() {
    // Hash a string
    let hash = crypto::sha256("Hello, World!");
    console.log("SHA-256: " + hash);

    // Base64 encoding
    let text = "Hello";
    let encoded = crypto::base64_encode(text);
    console.log("Base64: " + encoded);

    let decoded = crypto::base64_decode(encoded);
    console.log("Decoded: " + decoded);

    // Password hashing
    let password = "my_secret_password";
    let hash_result = crypto::hash_password(password);

    if hash_result.is_ok() {
        let hashed = hash_result.unwrap();
        console.log("Password hash: " + hashed);

        // Verify password
        let is_valid = crypto::verify_password(password, hashed);
        console.log("Password valid: " + is_valid.to_string());
    }
}
```

## Testing Your Code

Jounce has a built-in testing framework:

```jounce
// tests/calculator_test.jnc

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn subtract(a: i64, b: i64) -> i64 {
    return a - b;
}

fn test_addition() {
    let result = add(2, 3);
    assert_eq(result, 5, "2 + 3 should equal 5");
}

fn test_subtraction() {
    let result = subtract(10, 4);
    assert_eq(result, 6, "10 - 4 should equal 6");
}

fn test_negative_numbers() {
    let result = add(-5, 3);
    assert_eq(result, -2, "-5 + 3 should equal -2");
}
```

Run tests with:

```bash
jnc test
jnc test --verbose
jnc test --filter "addition"
```

## JSX and UI Components

Jounce supports JSX for building user interfaces:

```jounce
fn Button(props: ButtonProps) -> JSX {
    return <button class="btn">{props.label}</button>;
}

fn App() -> JSX {
    return (
        <div class="container">
            <h1>Welcome to Jounce</h1>
            <Button label="Click me!" />
        </div>
    );
}
```

## CSS Utilities

Jounce includes a built-in CSS utility system:

```jounce
fn StyledComponent() -> JSX {
    // Utility classes are automatically generated
    return (
        <div class="p-4 bg-blue-500 text-white rounded-lg">
            <h2 class="text-2xl font-bold">Styled Content</h2>
            <p class="mt-2">With utility classes!</p>
        </div>
    );
}
```

## Using Packages

Jounce has a complete package ecosystem with a built-in package manager. You can install and use production-ready libraries to accelerate your development.

### Package Manager Commands

```bash
# Install a package
jnc pkg install jounce-router

# Add dependency to jounce.toml
jnc pkg add jounce-http

# Install all dependencies from jounce.toml
jnc pkg install

# Search for packages
jnc pkg search forms

# Show package information
jnc pkg info jounce-store

# Update dependencies
jnc pkg update

# Show dependency tree
jnc pkg tree

# Security audit
jnc pkg audit
```

### Available Packages

#### jounce-router v0.1.0

Client-side routing with history API, route guards, and navigation hooks.

```jounce
use jounce_router::{Router, Route};

fn setup_routes() {
    let router = Router::new();

    // Add routes
    router.add_route("/", home_handler);
    router.add_route("/users/:id", user_handler);
    router.add_route("/about", about_handler);

    // Navigate programmatically
    router.navigate("/users/42");
}

fn home_handler() -> JSX {
    return <div><h1>Home Page</h1></div>;
}

fn user_handler(params: RouteParams) -> JSX {
    let user_id = params.get("id");
    return <div><h1>User {user_id}</h1></div>;
}
```

#### jounce-http v0.1.0

HTTP client for making API requests with configuration support.

```jounce
use jounce_http::HttpClient;

async fn fetch_users() {
    // Create client with base URL
    let client = HttpClient::new("https://api.example.com");

    // GET request
    let response = client.get("/users").send().await;

    if response.is_ok() {
        let data = response.unwrap();
        console.log("Users: " + data);
    }

    // POST request with JSON
    let user_data = "{\"name\": \"Alice\", \"email\": \"alice@example.com\"}";
    let post_response = client.post("/users")
        .body(user_data)
        .send()
        .await;
}
```

#### jounce-forms v1.0.0

Form handling, validation, and form builders.

```jounce
use jounce_forms::{Form, Validator};

fn create_signup_form() -> Form {
    let form = Form::new()
        .add_field("email", Validator::email())
        .add_field("password", Validator::min_length(8))
        .add_field("age", Validator::range(18, 100));

    return form;
}

fn handle_submit(form_data: FormData) {
    let form = create_signup_form();

    let validation_result = form.validate(form_data);

    if validation_result.is_valid() {
        console.log("Form is valid!");
    } else {
        let errors = validation_result.errors();
        console.log("Validation errors: " + errors.to_string());
    }
}
```

#### jounce-i18n v1.0.0

Internationalization and localization support.

```jounce
use jounce_i18n::Translator;

fn setup_translations() {
    // Initialize translator with locale
    let t = Translator::new("en");

    // Load translations
    t.load_translations("en", "{
        \"welcome\": \"Welcome, {name}!\",
        \"goodbye\": \"Goodbye!\"
    }");

    // Translate with parameters
    let message = t.translate("welcome", {name: "Alice"});
    console.log(message);  // "Welcome, Alice!"

    // Switch locale
    t.set_locale("es");
}
```

#### jounce-store v1.0.0

State management with actions, reducers, and middleware.

```jounce
use jounce_store::Store;

enum Action {
    Increment,
    Decrement,
    SetValue(i64),
}

struct AppState {
    count: i64,
}

fn reducer(state: AppState, action: Action) -> AppState {
    match action {
        Action::Increment => {
            return AppState { count: state.count + 1 };
        }
        Action::Decrement => {
            return AppState { count: state.count - 1 };
        }
        Action::SetValue(value) => {
            return AppState { count: value };
        }
    }
}

fn main() {
    let initial_state = AppState { count: 0 };
    let store = Store::new(initial_state, reducer);

    // Dispatch actions
    store.dispatch(Action::Increment);
    store.dispatch(Action::Increment);

    let state = store.get_state();
    console.log("Count: " + state.count.to_string());  // "Count: 2"
}
```

### Creating a Full Application with Packages

Here's an example combining multiple packages:

```jounce
use jounce_router::{Router, Route};
use jounce_http::HttpClient;
use jounce_store::Store;
use jounce_forms::{Form, Validator};

fn main() {
    // Setup router
    let router = Router::new();
    router.add_route("/", home_page);
    router.add_route("/users", users_page);

    // Initialize store
    let store = Store::new(initial_state, reducer);

    // Start app
    router.start();
}

async fn users_page() -> JSX {
    // Fetch users with HTTP client
    let client = HttpClient::new("https://api.example.com");
    let response = client.get("/users").send().await;

    if response.is_ok() {
        let users = response.unwrap();
        return render_users_list(users);
    }

    return <div>Error loading users</div>;
}

fn render_users_list(users: String) -> JSX {
    return (
        <div class="container">
            <h1>Users</h1>
            <div class="users-list">
                {/* Render users here */}
            </div>
        </div>
    );
}
```

### Package Configuration

To manage dependencies, create a `jounce.toml` file in your project:

```toml
[package]
name = "my-app"
version = "1.0.0"

[dependencies]
jounce-router = "0.1.0"
jounce-http = "0.1.0"
jounce-forms = "1.0.0"
jounce-i18n = "1.0.0"
jounce-store = "1.0.0"
```

Then install all dependencies at once:

```bash
jnc pkg install
```

## Compilation Options

### Basic Compilation

```bash
jnc compile app.jnc
```

### With Minification

```bash
jnc compile app.jnc --minify
```

### Custom Output Directory

```bash
jnc compile app.jnc --output build/
```

### Watch Mode (auto-recompile on changes)

```bash
jnc watch src --output dist
```

## Performance Tips

1. **Use the cache**: Jounce automatically caches compiled ASTs for 100x+ faster builds
2. **Watch mode**: Use `jnc watch` during development for instant recompilation
3. **Minification**: Enable `--minify` for production builds to reduce file size
4. **Type annotations**: Explicit types help the compiler optimize better

## Next Steps

- Explore the [YAML Module API](../api/YAML_MODULE.md)
- Check out example projects in `/examples`
- Read the [Language Reference](../reference/LANGUAGE_SPEC.md)
- Join the community and contribute!

## Common Patterns

### Error Handling

```jounce
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Result::Err("Cannot divide by zero");
    }
    return Result::Ok(a / b);
}

fn main() {
    let result = safe_divide(10.0, 0.0);

    match result {
        Result::Ok(value) => {
            console.log("Result: " + value.to_string());
        }
        Result::Err(error) => {
            console.log("Error: " + error);
        }
    }
}
```

### Working with Collections

```jounce
fn collection_example() {
    // Vectors
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    console.log("Length: " + numbers.len().to_string());
    console.log("First: " + numbers[0].to_string());

    // HashMaps
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);

    if scores.contains_key("Alice") {
        let score = scores.get("Alice");
        console.log("Alice's score: " + score.to_string());
    }
}
```

## Getting Help

- **Documentation**: Check the `/docs` directory
- **Examples**: See `/examples` for sample projects
- **Issues**: Report bugs on GitHub
- **Community**: Join our Discord/forum

---

**Congratulations!** You're now ready to build amazing applications with Jounce! 🚀

**Version**: 0.3.0
**Last Updated**: 2025-10-24
