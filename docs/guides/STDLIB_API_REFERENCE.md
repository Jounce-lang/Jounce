# Jounce Standard Library API Reference

**Version**: 0.1.0
**Last Updated**: 2025-10-21
**Total Modules**: 16
**Total Functions**: 200+

Complete API reference for all Jounce standard library modules.

---

## Table of Contents

1. [Math](#1-math) - Mathematical operations
2. [Reactive](#2-reactive) - Reactive programming primitives
3. [HTTP](#3-http) - HTTP client and server
4. [Auth](#4-auth) - Authentication and authorization
5. [Collections](#5-collections) - Collection utilities
6. [Database](#6-database) - Database operations
7. [File System](#7-file-system) - File and directory operations
8. [JSON](#8-json) - JSON parsing and serialization
9. [String](#9-string) - String manipulation
10. [Time](#10-time) - Date and time utilities
11. [Crypto](#11-crypto) - Cryptographic functions
12. [Router](#12-router) - Client-side routing
13. [Forms](#13-forms) - Form handling and validation
14. [WebSocket](#14-websocket) - WebSocket connections
15. [Storage](#15-storage) - Browser storage APIs
16. [I18n](#16-i18n) - Internationalization

---

## 1. Math

**Module**: `Math::`
**Size**: 661 lines
**Tests**: 30 passing
**Import**: Auto-imported (global namespace)

### Constants

#### `Math::PI`
```raven
const PI: f64 = 3.141592653589793
```
The mathematical constant π (pi).

**Example**:
```raven
let circumference = 2.0 * Math::PI * radius;
```

#### `Math::E`
```raven
const E: f64 = 2.718281828459045
```
Euler's number (base of natural logarithm).

**Example**:
```raven
let growth = Math::E * rate;
```

#### `Math::TAU`
```raven
const TAU: f64 = 6.283185307179586
```
The mathematical constant τ (tau), equal to 2π.

#### `Math::SQRT_2`
```raven
const SQRT_2: f64 = 1.4142135623730951
```
Square root of 2.

#### `Math::FRAC_1_SQRT_2`
```raven
const FRAC_1_SQRT_2: f64 = 0.7071067811865476
```
1 divided by the square root of 2.

#### `Math::FRAC_PI_2`
```raven
const FRAC_PI_2: f64 = 1.5707963267948966
```
π divided by 2 (90 degrees in radians).

#### `Math::FRAC_PI_4`
```raven
const FRAC_PI_4: f64 = 0.7853981633974483
```
π divided by 4 (45 degrees in radians).

#### `Math::LN_2`
```raven
const LN_2: f64 = 0.6931471805599453
```
Natural logarithm of 2.

#### `Math::LN_10`
```raven
const LN_10: f64 = 2.302585092994046
```
Natural logarithm of 10.

---

### Basic Operations

#### `Math::abs(x: f64) -> f64`
Returns the absolute value of a number.

**Parameters**:
- `x` - The number

**Returns**: Absolute value (always non-negative)

**Example**:
```raven
let positive = Math::abs(-42.5);  // 42.5
```

#### `Math::min(a: f64, b: f64) -> f64`
Returns the smaller of two numbers.

**Parameters**:
- `a` - First number
- `b` - Second number

**Returns**: The minimum value

**Example**:
```raven
let smallest = Math::min(10.5, 20.3);  // 10.5
```

#### `Math::max(a: f64, b: f64) -> f64`
Returns the larger of two numbers.

**Parameters**:
- `a` - First number
- `b` - Second number

**Returns**: The maximum value

**Example**:
```raven
let largest = Math::max(10.5, 20.3);  // 20.3
```

#### `Math::clamp(value: f64, min: f64, max: f64) -> f64`
Constrains a value to a range.

**Parameters**:
- `value` - The value to clamp
- `min` - Minimum allowed value
- `max` - Maximum allowed value

**Returns**: Value clamped to [min, max]

**Example**:
```raven
let clamped = Math::clamp(150.0, 0.0, 100.0);  // 100.0
let clamped2 = Math::clamp(-10.0, 0.0, 100.0); // 0.0
```

#### `Math::sign(x: f64) -> f64`
Returns the sign of a number (-1, 0, or 1).

**Parameters**:
- `x` - The number

**Returns**: -1.0 if negative, 0.0 if zero, 1.0 if positive

**Example**:
```raven
let s1 = Math::sign(42.0);   // 1.0
let s2 = Math::sign(-42.0);  // -1.0
let s3 = Math::sign(0.0);    // 0.0
```

---

### Powers & Roots

#### `Math::pow(base: f64, exponent: f64) -> f64`
Raises a number to a power.

**Parameters**:
- `base` - The base number
- `exponent` - The exponent

**Returns**: base^exponent

**Example**:
```raven
let result = Math::pow(2.0, 8.0);  // 256.0
```

#### `Math::sqrt(x: f64) -> f64`
Returns the square root of a number.

**Parameters**:
- `x` - The number (must be non-negative)

**Returns**: √x

**Example**:
```raven
let root = Math::sqrt(16.0);  // 4.0
```

#### `Math::cbrt(x: f64) -> f64`
Returns the cube root of a number.

**Parameters**:
- `x` - The number

**Returns**: ∛x

**Example**:
```raven
let root = Math::cbrt(27.0);  // 3.0
```

#### `Math::square(x: f64) -> f64`
Returns the square of a number (convenience function).

**Parameters**:
- `x` - The number

**Returns**: x²

**Example**:
```raven
let sq = Math::square(5.0);  // 25.0
```

#### `Math::cube(x: f64) -> f64`
Returns the cube of a number (convenience function).

**Parameters**:
- `x` - The number

**Returns**: x³

**Example**:
```raven
let cb = Math::cube(3.0);  // 27.0
```

#### `Math::exp(x: f64) -> f64`
Returns e raised to the power of x.

**Parameters**:
- `x` - The exponent

**Returns**: e^x

**Example**:
```raven
let result = Math::exp(2.0);  // ~7.389
```

#### `Math::exp2(x: f64) -> f64`
Returns 2 raised to the power of x.

**Parameters**:
- `x` - The exponent

**Returns**: 2^x

**Example**:
```raven
let result = Math::exp2(3.0);  // 8.0
```

---

### Logarithms

#### `Math::ln(x: f64) -> f64`
Returns the natural logarithm (base e).

**Parameters**:
- `x` - The number (must be positive)

**Returns**: ln(x)

**Example**:
```raven
let result = Math::ln(Math::E);  // 1.0
```

#### `Math::log2(x: f64) -> f64`
Returns the base-2 logarithm.

**Parameters**:
- `x` - The number (must be positive)

**Returns**: log₂(x)

**Example**:
```raven
let result = Math::log2(8.0);  // 3.0
```

#### `Math::log10(x: f64) -> f64`
Returns the base-10 logarithm.

**Parameters**:
- `x` - The number (must be positive)

**Returns**: log₁₀(x)

**Example**:
```raven
let result = Math::log10(100.0);  // 2.0
```

#### `Math::log(x: f64, base: f64) -> f64`
Returns the logarithm with custom base.

**Parameters**:
- `x` - The number (must be positive)
- `base` - The logarithm base (must be positive)

**Returns**: log_base(x)

**Example**:
```raven
let result = Math::log(32.0, 2.0);  // 5.0
```

---

### Rounding

#### `Math::round(x: f64) -> f64`
Rounds to the nearest integer.

**Parameters**:
- `x` - The number

**Returns**: Nearest integer (0.5 rounds up)

**Example**:
```raven
let r1 = Math::round(3.7);  // 4.0
let r2 = Math::round(3.2);  // 3.0
let r3 = Math::round(3.5);  // 4.0
```

#### `Math::floor(x: f64) -> f64`
Rounds down to the nearest integer.

**Parameters**:
- `x` - The number

**Returns**: Largest integer ≤ x

**Example**:
```raven
let f = Math::floor(3.7);  // 3.0
```

#### `Math::ceil(x: f64) -> f64`
Rounds up to the nearest integer.

**Parameters**:
- `x` - The number

**Returns**: Smallest integer ≥ x

**Example**:
```raven
let c = Math::ceil(3.2);  // 4.0
```

#### `Math::trunc(x: f64) -> f64`
Removes the decimal part.

**Parameters**:
- `x` - The number

**Returns**: Integer part only

**Example**:
```raven
let t1 = Math::trunc(3.7);   // 3.0
let t2 = Math::trunc(-3.7);  // -3.0
```

#### `Math::fract(x: f64) -> f64`
Returns the fractional part.

**Parameters**:
- `x` - The number

**Returns**: Decimal part only

**Example**:
```raven
let f = Math::fract(3.7);  // 0.7
```

---

### Trigonometry

#### `Math::sin(x: f64) -> f64`
Returns the sine of an angle (in radians).

**Parameters**:
- `x` - Angle in radians

**Returns**: sin(x)

**Example**:
```raven
let s = Math::sin(Math::PI / 2.0);  // 1.0
```

#### `Math::cos(x: f64) -> f64`
Returns the cosine of an angle (in radians).

**Parameters**:
- `x` - Angle in radians

**Returns**: cos(x)

**Example**:
```raven
let c = Math::cos(0.0);  // 1.0
```

#### `Math::tan(x: f64) -> f64`
Returns the tangent of an angle (in radians).

**Parameters**:
- `x` - Angle in radians

**Returns**: tan(x)

**Example**:
```raven
let t = Math::tan(Math::PI / 4.0);  // ~1.0
```

#### `Math::asin(x: f64) -> f64`
Returns the arcsine (inverse sine) in radians.

**Parameters**:
- `x` - Value between -1 and 1

**Returns**: Angle in radians

**Example**:
```raven
let angle = Math::asin(0.5);  // ~0.524 (30°)
```

#### `Math::acos(x: f64) -> f64`
Returns the arccosine (inverse cosine) in radians.

**Parameters**:
- `x` - Value between -1 and 1

**Returns**: Angle in radians

**Example**:
```raven
let angle = Math::acos(0.5);  // ~1.047 (60°)
```

#### `Math::atan(x: f64) -> f64`
Returns the arctangent (inverse tangent) in radians.

**Parameters**:
- `x` - Any number

**Returns**: Angle in radians

**Example**:
```raven
let angle = Math::atan(1.0);  // ~0.785 (45°)
```

#### `Math::atan2(y: f64, x: f64) -> f64`
Returns the angle from the x-axis to point (x, y).

**Parameters**:
- `y` - Y coordinate
- `x` - X coordinate

**Returns**: Angle in radians (-π to π)

**Example**:
```raven
let angle = Math::atan2(1.0, 1.0);  // ~0.785 (45°)
```

#### `Math::sinh(x: f64) -> f64`
Returns the hyperbolic sine.

**Parameters**:
- `x` - The number

**Returns**: sinh(x)

**Example**:
```raven
let h = Math::sinh(1.0);  // ~1.175
```

#### `Math::cosh(x: f64) -> f64`
Returns the hyperbolic cosine.

**Parameters**:
- `x` - The number

**Returns**: cosh(x)

**Example**:
```raven
let h = Math::cosh(1.0);  // ~1.543
```

#### `Math::tanh(x: f64) -> f64`
Returns the hyperbolic tangent.

**Parameters**:
- `x` - The number

**Returns**: tanh(x)

**Example**:
```raven
let h = Math::tanh(1.0);  // ~0.762
```

#### `Math::radians(degrees: f64) -> f64`
Converts degrees to radians.

**Parameters**:
- `degrees` - Angle in degrees

**Returns**: Angle in radians

**Example**:
```raven
let rad = Math::radians(90.0);  // ~1.571 (π/2)
```

#### `Math::degrees(radians: f64) -> f64`
Converts radians to degrees.

**Parameters**:
- `radians` - Angle in radians

**Returns**: Angle in degrees

**Example**:
```raven
let deg = Math::degrees(Math::PI);  // 180.0
```

---

### Random Numbers

#### `Math::random() -> f64`
Returns a random number between 0 (inclusive) and 1 (exclusive).

**Returns**: Random f64 in [0, 1)

**Example**:
```raven
let r = Math::random();  // e.g., 0.742
```

#### `Math::random_range(min: f64, max: f64) -> f64`
Returns a random number in a range.

**Parameters**:
- `min` - Minimum value (inclusive)
- `max` - Maximum value (exclusive)

**Returns**: Random f64 in [min, max)

**Example**:
```raven
let r = Math::random_range(10.0, 20.0);  // e.g., 15.3
```

#### `Math::random_int(min: i32, max: i32) -> i32`
Returns a random integer in a range.

**Parameters**:
- `min` - Minimum value (inclusive)
- `max` - Maximum value (inclusive)

**Returns**: Random i32 in [min, max]

**Example**:
```raven
let dice = Math::random_int(1, 6);  // 1, 2, 3, 4, 5, or 6
```

---

### Utilities

#### `Math::is_nan(x: f64) -> bool`
Checks if a number is NaN (Not a Number).

**Parameters**:
- `x` - The number to check

**Returns**: true if NaN, false otherwise

**Example**:
```raven
let nan = 0.0 / 0.0;
if Math::is_nan(nan) {
    println!("Invalid calculation");
}
```

#### `Math::is_infinite(x: f64) -> bool`
Checks if a number is infinite.

**Parameters**:
- `x` - The number to check

**Returns**: true if ±∞, false otherwise

**Example**:
```raven
let inf = 1.0 / 0.0;
if Math::is_infinite(inf) {
    println!("Overflow");
}
```

#### `Math::is_finite(x: f64) -> bool`
Checks if a number is finite.

**Parameters**:
- `x` - The number to check

**Returns**: true if finite, false if NaN or ±∞

**Example**:
```raven
if Math::is_finite(user_input) {
    // Safe to use
}
```

---

## 2. Reactive

**Module**: `Signal::`, `Computed::`, `Effect::`
**Size**: 342 lines
**Tests**: 17 passing
**Import**: Auto-imported (global namespace)

### Signal

Mutable reactive state that notifies dependents when changed.

#### `Signal::new<T>(initial: T) -> Signal<T>`
Creates a new signal with an initial value.

**Type Parameters**:
- `T` - Type of the value (must be cloneable)

**Parameters**:
- `initial` - Initial value

**Returns**: New Signal instance

**Example**:
```raven
let count = Signal::new(0);
let name = Signal::new("Alice");
let items = Signal::new(vec![1, 2, 3]);
```

#### `signal.get() -> T`
Gets the current value and tracks the dependency.

**Returns**: Current value (cloned)

**Example**:
```raven
let count = Signal::new(10);
let current = count.get();  // 10
```

#### `signal.set(value: T)`
Sets a new value and notifies all dependents.

**Parameters**:
- `value` - New value to set

**Example**:
```raven
let count = Signal::new(0);
count.set(5);  // Triggers all dependent computed values and effects
```

---

### Computed

Derived reactive value that automatically updates when dependencies change.

#### `Computed::new<T>(fn() -> T) -> Computed<T>`
Creates a new computed value.

**Type Parameters**:
- `T` - Type of the computed value

**Parameters**:
- Function that computes the value (must call .get() on signals)

**Returns**: New Computed instance

**Example**:
```raven
let count = Signal::new(10);
let doubled = Computed::new(|| {
    return count.get() * 2;
});

println!("{}", doubled.get());  // 20
count.set(15);
println!("{}", doubled.get());  // 30 (automatically updated)
```

#### `computed.get() -> T`
Gets the current computed value and tracks the dependency.

**Returns**: Computed value (cached, only recomputes if dependencies changed)

**Example**:
```raven
let first = Signal::new("John");
let last = Signal::new("Doe");

let full_name = Computed::new(|| {
    return format!("{} {}", first.get(), last.get());
});

println!("{}", full_name.get());  // "John Doe"
```

---

### Effect

Side effect that runs when dependencies change.

#### `create_effect(fn())`
Creates an effect that runs immediately and on dependency changes.

**Parameters**:
- Function to run (must call .get() on signals/computed values)

**Example**:
```raven
let count = Signal::new(0);

create_effect(|| {
    println!("Count is now: {}", count.get());
});
// Prints: "Count is now: 0"

count.set(5);
// Prints: "Count is now: 5"
```

**Use Cases**:
- Logging and debugging
- DOM updates (in client code)
- Analytics tracking
- Syncing to localStorage
- Network requests based on state

---

### ReactiveVec

Reactive vector that notifies on changes.

#### `ReactiveVec::new<T>() -> ReactiveVec<T>`
Creates a new reactive vector.

**Type Parameters**:
- `T` - Element type

**Returns**: Empty reactive vector

**Example**:
```raven
let items = ReactiveVec::new();
```

#### `vec.push(item: T)`
Adds an item and triggers reactivity.

**Parameters**:
- `item` - Item to add

**Example**:
```raven
let items = ReactiveVec::new();
items.push("Apple");
items.push("Banana");
```

#### `vec.remove(index: usize) -> T`
Removes an item at index and triggers reactivity.

**Parameters**:
- `index` - Index to remove

**Returns**: Removed item

**Example**:
```raven
let items = ReactiveVec::new();
items.push("A");
items.push("B");
let removed = items.remove(0);  // Removes "A"
```

#### `vec.clear()`
Removes all items and triggers reactivity.

**Example**:
```raven
items.clear();
```

#### `vec.len() -> usize`
Returns the number of items (tracks dependency).

**Returns**: Number of elements

**Example**:
```raven
let count = Computed::new(|| items.len());
```

#### `vec.notify()`
Manually trigger reactivity (after direct mutation).

**Example**:
```raven
let first = &mut items[0];
first.quantity = 2;
items.notify();  // Trigger dependents
```

---

## 3. HTTP

**Module**: `HttpRequest::`, `HttpClient::`
**Size**: 485 lines
**Tests**: 9 (marked as ignored - require external service)
**Import**: Auto-imported for `@server` functions

### HttpRequest

Builder API for HTTP requests.

#### `HttpRequest::get(url: &str) -> HttpRequest`
Creates a GET request.

**Parameters**:
- `url` - The URL to request

**Returns**: Request builder

**Example**:
```raven
@server
async fn fetch_users() {
    let response = HttpRequest::get("https://api.example.com/users")
        .send()
        .await;
}
```

#### `HttpRequest::post(url: &str) -> HttpRequest`
Creates a POST request.

**Parameters**:
- `url` - The URL to post to

**Returns**: Request builder

**Example**:
```raven
@server
async fn create_user(user: User) {
    let response = HttpRequest::post("https://api.example.com/users")
        .json(user)
        .send()
        .await;
}
```

#### `HttpRequest::put(url: &str) -> HttpRequest`
Creates a PUT request.

**Parameters**:
- `url` - The URL

**Returns**: Request builder

#### `HttpRequest::delete(url: &str) -> HttpRequest`
Creates a DELETE request.

**Parameters**:
- `url` - The URL

**Returns**: Request builder

#### `request.header(key: &str, value: &str) -> HttpRequest`
Adds a header to the request.

**Parameters**:
- `key` - Header name
- `value` - Header value

**Returns**: Request builder (chainable)

**Example**:
```raven
let response = HttpRequest::get(url)
    .header("Authorization", "Bearer TOKEN")
    .header("Accept", "application/json")
    .send()
    .await;
```

#### `request.query(key: &str, value: &str) -> HttpRequest`
Adds a query parameter.

**Parameters**:
- `key` - Parameter name
- `value` - Parameter value

**Returns**: Request builder (chainable)

**Example**:
```raven
let response = HttpRequest::get("https://api.example.com/search")
    .query("q", "Jounce")
    .query("limit", "10")
    .send()
    .await;
// URL: https://api.example.com/search?q=Jounce&limit=10
```

#### `request.json(data: Value) -> HttpRequest`
Sets JSON body and Content-Type header.

**Parameters**:
- `data` - JSON value (from json! macro)

**Returns**: Request builder (chainable)

**Example**:
```raven
let user = json!({
    "name": "Alice",
    "email": "alice@example.com"
});

let response = HttpRequest::post(url)
    .json(user)
    .send()
    .await;
```

#### `request.form(key: &str, value: &str) -> HttpRequest`
Adds form data field.

**Parameters**:
- `key` - Field name
- `value` - Field value

**Returns**: Request builder (chainable)

**Example**:
```raven
let response = HttpRequest::post("/login")
    .form("username", "alice")
    .form("password", "secret")
    .send()
    .await;
```

#### `request.timeout(ms: u64) -> HttpRequest`
Sets request timeout.

**Parameters**:
- `ms` - Timeout in milliseconds

**Returns**: Request builder (chainable)

**Example**:
```raven
let response = HttpRequest::get(url)
    .timeout(5000)  // 5 seconds
    .send()
    .await;
```

#### `request.send() -> Future<Result<HttpResponse, String>>`
Sends the request (async).

**Returns**: Future resolving to Result with response or error

**Example**:
```raven
@server
async fn fetch() {
    let response = HttpRequest::get(url).send().await;
    match response {
        Ok(resp) => println!("Status: {}", resp.status),
        Err(err) => println!("Error: {}", err),
    }
}
```

---

### HttpResponse

Response from an HTTP request.

#### `response.status -> i32`
HTTP status code.

**Example**:
```raven
if resp.status == 200 {
    println!("Success!");
}
```

#### `response.body -> String`
Response body as string.

**Example**:
```raven
println!("Body: {}", resp.body);
```

#### `response.is_ok() -> bool`
Checks if status is 2xx.

**Returns**: true if successful

**Example**:
```raven
if resp.is_ok() {
    // Process response
}
```

#### `response.is_success() -> bool`
Alias for is_ok().

#### `response.is_client_error() -> bool`
Checks if status is 4xx.

**Returns**: true if client error

**Example**:
```raven
if resp.is_client_error() {
    println!("Bad request");
}
```

#### `response.is_server_error() -> bool`
Checks if status is 5xx.

**Returns**: true if server error

**Example**:
```raven
if resp.is_server_error() {
    println!("Server error");
}
```

#### `response.json() -> Result<Value, String>`
Parses body as JSON.

**Returns**: Result with parsed JSON or error

**Example**:
```raven
match resp.json() {
    Ok(data) => {
        let name = data["name"].as_str().unwrap();
        println!("Name: {}", name);
    }
    Err(err) => println!("Parse error: {}", err),
}
```

---

### HttpClient

HTTP client with base URL and default headers.

#### `HttpClient::new() -> HttpClient`
Creates a new HTTP client.

**Returns**: Client builder

**Example**:
```raven
@server
async fn api_example() {
    let client = HttpClient::new()
        .with_base_url("https://api.github.com")
        .with_header("Accept", "application/json");
}
```

#### `client.with_base_url(url: &str) -> HttpClient`
Sets base URL for all requests.

**Parameters**:
- `url` - Base URL

**Returns**: Client builder (chainable)

#### `client.with_header(key: &str, value: &str) -> HttpClient`
Adds default header for all requests.

**Parameters**:
- `key` - Header name
- `value` - Header value

**Returns**: Client builder (chainable)

#### `client.get(path: &str) -> HttpRequest`
Creates GET request with base URL.

**Parameters**:
- `path` - Path relative to base URL

**Returns**: Request builder

**Example**:
```raven
let client = HttpClient::new()
    .with_base_url("https://api.example.com");

let response = client.get("/users/123").send().await;
// Requests: https://api.example.com/users/123
```

#### `client.post(path: &str) -> HttpRequest`
Creates POST request with base URL.

#### `client.put(path: &str) -> HttpRequest`
Creates PUT request with base URL.

#### `client.delete(path: &str) -> HttpRequest`
Creates DELETE request with base URL.

---

### Convenience Functions

#### `get(url: &str) -> Future<Result<HttpResponse, String>>`
Quick GET request.

**Parameters**:
- `url` - The URL

**Returns**: Future with response

**Example**:
```raven
@server
async fn quick_fetch() {
    let resp = get("https://api.example.com/data").await;
}
```

#### `post_json(url: &str, data: Value) -> Future<Result<HttpResponse, String>>`
Quick POST with JSON.

**Parameters**:
- `url` - The URL
- `data` - JSON data

**Returns**: Future with response

**Example**:
```raven
@server
async fn quick_post() {
    let data = json!({"key": "value"});
    let resp = post_json(url, data).await;
}
```

#### `get_blocking(url: &str) -> Result<HttpResponse, String>`
Synchronous GET request.

**Parameters**:
- `url` - The URL

**Returns**: Response or error (blocks until complete)

**Example**:
```raven
@server
fn sync_fetch() {
    let resp = get_blocking(url);
    // No async/await needed
}
```

#### `post_json_blocking(url: &str, data: Value) -> Result<HttpResponse, String>`
Synchronous POST with JSON.

---

## 4. Auth

**Module**: `Auth::`
**Size**: 312 lines
**Tests**: 8 passing
**Import**: Auto-imported for `@server` functions

### Password Hashing

#### `Auth::hash_password(password: &str) -> String`
Hashes a password using bcrypt.

**Parameters**:
- `password` - Plain text password

**Returns**: Hashed password (safe to store)

**Example**:
```raven
@server
fn register(password: String) {
    let hashed = Auth::hash_password(&password);
    db.save_user(email, hashed);
}
```

#### `Auth::verify_password(password: &str, hash: &str) -> bool`
Verifies a password against a hash.

**Parameters**:
- `password` - Plain text password to check
- `hash` - Stored password hash

**Returns**: true if password matches

**Example**:
```raven
@server
fn login(email: String, password: String) -> bool {
    let user = db.get_user(&email);
    return Auth::verify_password(&password, &user.password_hash);
}
```

---

### JWT (JSON Web Tokens)

#### `Auth::create_jwt(claims: Value, secret: &str) -> String`
Creates a JWT token.

**Parameters**:
- `claims` - Token payload (use json! macro)
- `secret` - Secret key for signing

**Returns**: JWT token string

**Example**:
```raven
@server
fn create_session(user_id: String) -> String {
    let claims = json!({
        "sub": user_id,
        "exp": timestamp + 3600  // 1 hour
    });

    return Auth::create_jwt(claims, "SECRET_KEY");
}
```

#### `Auth::verify_jwt(token: &str, secret: &str) -> Result<Value, String>`
Verifies and decodes a JWT token.

**Parameters**:
- `token` - JWT token string
- `secret` - Secret key for verification

**Returns**: Result with decoded claims or error

**Example**:
```raven
@server
fn verify_session(token: String) -> Option<String> {
    match Auth::verify_jwt(&token, "SECRET_KEY") {
        Ok(claims) => {
            let user_id = claims["sub"].as_str().unwrap();
            return Some(user_id.to_string());
        }
        Err(_) => return None,
    }
}
```

---

### Session Management

#### `Auth::generate_session_id() -> String`
Generates a cryptographically secure session ID.

**Returns**: Random session ID (32 characters)

**Example**:
```raven
@server
fn create_session() -> String {
    let session_id = Auth::generate_session_id();
    sessions.insert(session_id.clone(), user);
    return session_id;
}
```

#### `Auth::generate_token(length: usize) -> String`
Generates a random token.

**Parameters**:
- `length` - Token length in characters

**Returns**: Random token

**Example**:
```raven
let reset_token = Auth::generate_token(64);
email.send_reset_link(user.email, reset_token);
```

---

## 5. Collections

**Module**: `Collections::`
**Size**: 289 lines
**Tests**: 9 passing
**Import**: Auto-imported (global namespace)

### Array/Vector Operations

#### `Collections::first<T>(vec: &Vec<T>) -> Option<&T>`
Gets the first element.

**Type Parameters**:
- `T` - Element type

**Parameters**:
- `vec` - The vector

**Returns**: Some(element) or None if empty

**Example**:
```raven
let items = vec![1, 2, 3];
match Collections::first(&items) {
    Some(first) => println!("First: {}", first),
    None => println!("Empty"),
}
```

#### `Collections::last<T>(vec: &Vec<T>) -> Option<&T>`
Gets the last element.

**Returns**: Some(element) or None if empty

**Example**:
```raven
let last = Collections::last(&items);
```

#### `Collections::find<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> Option<&T>`
Finds first element matching predicate.

**Parameters**:
- `vec` - The vector
- `predicate` - Function that returns true for match

**Returns**: Some(element) or None

**Example**:
```raven
let numbers = vec![1, 2, 3, 4, 5];
let even = Collections::find(&numbers, |n| n % 2 == 0);
// Some(2)
```

#### `Collections::filter<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> Vec<T>`
Filters elements matching predicate.

**Parameters**:
- `vec` - The vector
- `predicate` - Function that returns true to keep element

**Returns**: New vector with matching elements

**Example**:
```raven
let numbers = vec![1, 2, 3, 4, 5];
let evens = Collections::filter(&numbers, |n| n % 2 == 0);
// [2, 4]
```

#### `Collections::map<T, U>(vec: &Vec<T>, mapper: fn(&T) -> U) -> Vec<U>`
Transforms elements.

**Type Parameters**:
- `T` - Input element type
- `U` - Output element type

**Parameters**:
- `vec` - The vector
- `mapper` - Transformation function

**Returns**: New vector with transformed elements

**Example**:
```raven
let numbers = vec![1, 2, 3];
let doubled = Collections::map(&numbers, |n| n * 2);
// [2, 4, 6]
```

#### `Collections::reduce<T, U>(vec: &Vec<T>, initial: U, reducer: fn(U, &T) -> U) -> U`
Reduces to a single value.

**Type Parameters**:
- `T` - Element type
- `U` - Accumulator type

**Parameters**:
- `vec` - The vector
- `initial` - Initial accumulator value
- `reducer` - Reduction function

**Returns**: Final accumulated value

**Example**:
```raven
let numbers = vec![1, 2, 3, 4];
let sum = Collections::reduce(&numbers, 0, |acc, n| acc + n);
// 10
```

#### `Collections::any<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> bool`
Checks if any element matches.

**Parameters**:
- `vec` - The vector
- `predicate` - Test function

**Returns**: true if at least one element matches

**Example**:
```raven
let numbers = vec![1, 2, 3];
let has_even = Collections::any(&numbers, |n| n % 2 == 0);
// true
```

#### `Collections::all<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> bool`
Checks if all elements match.

**Parameters**:
- `vec` - The vector
- `predicate` - Test function

**Returns**: true if all elements match

**Example**:
```raven
let numbers = vec![2, 4, 6];
let all_even = Collections::all(&numbers, |n| n % 2 == 0);
// true
```

#### `Collections::reverse<T>(vec: &Vec<T>) -> Vec<T>`
Reverses a vector.

**Parameters**:
- `vec` - The vector

**Returns**: New reversed vector

**Example**:
```raven
let numbers = vec![1, 2, 3];
let reversed = Collections::reverse(&numbers);
// [3, 2, 1]
```

#### `Collections::sort<T>(vec: &Vec<T>) -> Vec<T>`
Sorts a vector (ascending).

**Parameters**:
- `vec` - The vector

**Returns**: New sorted vector

**Example**:
```raven
let numbers = vec![3, 1, 4, 1, 5];
let sorted = Collections::sort(&numbers);
// [1, 1, 3, 4, 5]
```

#### `Collections::unique<T>(vec: &Vec<T>) -> Vec<T>`
Removes duplicates.

**Parameters**:
- `vec` - The vector

**Returns**: New vector with unique elements

**Example**:
```raven
let numbers = vec![1, 2, 2, 3, 1];
let unique = Collections::unique(&numbers);
// [1, 2, 3]
```

#### `Collections::chunk<T>(vec: &Vec<T>, size: usize) -> Vec<Vec<T>>`
Splits into chunks.

**Parameters**:
- `vec` - The vector
- `size` - Chunk size

**Returns**: Vector of chunks

**Example**:
```raven
let numbers = vec![1, 2, 3, 4, 5];
let chunks = Collections::chunk(&numbers, 2);
// [[1, 2], [3, 4], [5]]
```

---

## 6. Database

**Module**: `Db::`
**Size**: 256 lines
**Tests**: 0 (requires external database)
**Import**: Auto-imported for `@server` functions

### Connection

#### `Db::connect(connection_string: &str) -> Result<Connection, String>`
Connects to a database.

**Parameters**:
- `connection_string` - Database connection string

**Returns**: Result with connection or error

**Example**:
```raven
@server
fn init_db() {
    let conn = Db::connect("postgres://localhost/mydb").unwrap();
}
```

---

### Query Execution

#### `conn.query(sql: &str) -> Result<Vec<Row>, String>`
Executes a SELECT query.

**Parameters**:
- `sql` - SQL query string

**Returns**: Result with rows or error

**Example**:
```raven
@server
fn get_users() -> Vec<User> {
    let rows = conn.query("SELECT * FROM users").unwrap();
    return rows.iter().map(|r| User::from_row(r)).collect();
}
```

#### `conn.execute(sql: &str) -> Result<u64, String>`
Executes an INSERT/UPDATE/DELETE query.

**Parameters**:
- `sql` - SQL statement

**Returns**: Result with affected rows or error

**Example**:
```raven
@server
fn create_user(name: String, email: String) {
    let sql = format!("INSERT INTO users (name, email) VALUES ('{}', '{}')", name, email);
    conn.execute(&sql).unwrap();
}
```

---

### Prepared Statements

#### `conn.prepare(sql: &str) -> Result<Statement, String>`
Prepares a parameterized query.

**Parameters**:
- `sql` - SQL with placeholders ($1, $2, etc.)

**Returns**: Result with statement or error

**Example**:
```raven
@server
fn get_user_by_id(id: i32) -> Option<User> {
    let stmt = conn.prepare("SELECT * FROM users WHERE id = $1").unwrap();
    let rows = stmt.query(vec![id.to_string()]).unwrap();
    // ...
}
```

#### `stmt.query(params: Vec<String>) -> Result<Vec<Row>, String>`
Executes prepared query with parameters.

**Parameters**:
- `params` - Parameter values

**Returns**: Result with rows or error

---

### Transactions

#### `conn.begin_transaction() -> Result<Transaction, String>`
Begins a database transaction.

**Returns**: Result with transaction or error

**Example**:
```raven
@server
fn transfer_funds(from: i32, to: i32, amount: f64) {
    let txn = conn.begin_transaction().unwrap();
    txn.execute("UPDATE accounts SET balance = balance - $1 WHERE id = $2", vec![amount, from]);
    txn.execute("UPDATE accounts SET balance = balance + $1 WHERE id = $2", vec![amount, to]);
    txn.commit();
}
```

#### `txn.commit() -> Result<(), String>`
Commits the transaction.

**Returns**: Result with success or error

#### `txn.rollback() -> Result<(), String>`
Rolls back the transaction.

**Returns**: Result with success or error

---

## 7. File System

**Module**: `Fs::`
**Size**: 234 lines
**Tests**: 0 (requires file system access)
**Import**: Auto-imported for `@server` functions

### Reading Files

#### `Fs::read_file(path: &str) -> Result<String, String>`
Reads file contents as string.

**Parameters**:
- `path` - File path

**Returns**: Result with file contents or error

**Example**:
```raven
@server
fn load_config() -> Config {
    let contents = Fs::read_file("config.json").unwrap();
    return JSON::parse(&contents);
}
```

#### `Fs::read_bytes(path: &str) -> Result<Vec<u8>, String>`
Reads file contents as bytes.

**Parameters**:
- `path` - File path

**Returns**: Result with bytes or error

**Example**:
```raven
@server
fn load_image(path: String) -> Vec<u8> {
    return Fs::read_bytes(&path).unwrap();
}
```

---

### Writing Files

#### `Fs::write_file(path: &str, contents: &str) -> Result<(), String>`
Writes string to file (overwrites).

**Parameters**:
- `path` - File path
- `contents` - String contents

**Returns**: Result with success or error

**Example**:
```raven
@server
fn save_log(message: String) {
    Fs::write_file("app.log", &message).unwrap();
}
```

#### `Fs::write_bytes(path: &str, data: &Vec<u8>) -> Result<(), String>`
Writes bytes to file.

**Parameters**:
- `path` - File path
- `data` - Byte data

**Returns**: Result with success or error

#### `Fs::append_file(path: &str, contents: &str) -> Result<(), String>`
Appends to file.

**Parameters**:
- `path` - File path
- `contents` - String to append

**Returns**: Result with success or error

**Example**:
```raven
@server
fn log(message: String) {
    let line = format!("{}: {}\n", Time::now(), message);
    Fs::append_file("app.log", &line).unwrap();
}
```

---

### Directory Operations

#### `Fs::read_dir(path: &str) -> Result<Vec<String>, String>`
Lists directory contents.

**Parameters**:
- `path` - Directory path

**Returns**: Result with filenames or error

**Example**:
```raven
@server
fn list_uploads() -> Vec<String> {
    return Fs::read_dir("uploads/").unwrap();
}
```

#### `Fs::create_dir(path: &str) -> Result<(), String>`
Creates a directory.

**Parameters**:
- `path` - Directory path

**Returns**: Result with success or error

**Example**:
```raven
@server
fn init_storage() {
    Fs::create_dir("uploads/images/").unwrap();
}
```

#### `Fs::remove_dir(path: &str) -> Result<(), String>`
Removes an empty directory.

**Parameters**:
- `path` - Directory path

**Returns**: Result with success or error

#### `Fs::remove_dir_all(path: &str) -> Result<(), String>`
Removes directory and all contents.

**Parameters**:
- `path` - Directory path

**Returns**: Result with success or error

---

### File Operations

#### `Fs::exists(path: &str) -> bool`
Checks if file or directory exists.

**Parameters**:
- `path` - File/directory path

**Returns**: true if exists

**Example**:
```raven
@server
fn load_or_default(path: String) -> Config {
    if Fs::exists(&path) {
        return load_config(&path);
    } else {
        return Config::default();
    }
}
```

#### `Fs::remove_file(path: &str) -> Result<(), String>`
Deletes a file.

**Parameters**:
- `path` - File path

**Returns**: Result with success or error

**Example**:
```raven
@server
fn delete_upload(filename: String) {
    let path = format!("uploads/{}", filename);
    Fs::remove_file(&path).unwrap();
}
```

#### `Fs::copy(from: &str, to: &str) -> Result<(), String>`
Copies a file.

**Parameters**:
- `from` - Source path
- `to` - Destination path

**Returns**: Result with success or error

#### `Fs::rename(from: &str, to: &str) -> Result<(), String>`
Moves/renames a file.

**Parameters**:
- `from` - Source path
- `to` - Destination path

**Returns**: Result with success or error

---

## 8. JSON

**Module**: `JSON::`
**Size**: 201 lines
**Tests**: 6 passing
**Import**: Auto-imported (global namespace)

### Parsing

#### `JSON::parse(text: &str) -> Result<Value, String>`
Parses JSON string.

**Parameters**:
- `text` - JSON string

**Returns**: Result with parsed value or error

**Example**:
```raven
let json_str = "{\"name\":\"Alice\",\"age\":25}";
match JSON::parse(json_str) {
    Ok(data) => {
        let name = data["name"].as_str().unwrap();
        println!("Name: {}", name);
    }
    Err(err) => println!("Parse error: {}", err),
}
```

---

### Serialization

#### `JSON::stringify(value: &Value) -> String`
Converts value to JSON string.

**Parameters**:
- `value` - Value to serialize

**Returns**: JSON string

**Example**:
```raven
let user = json!({
    "name": "Alice",
    "age": 25
});

let json_str = JSON::stringify(&user);
println!("{}", json_str);
// {"name":"Alice","age":25}
```

#### `JSON::stringify_pretty(value: &Value) -> String`
Converts to formatted JSON string.

**Parameters**:
- `value` - Value to serialize

**Returns**: Formatted JSON string with indentation

**Example**:
```raven
let pretty = JSON::stringify_pretty(&user);
println!("{}", pretty);
// {
//   "name": "Alice",
//   "age": 25
// }
```

---

### json! Macro

#### `json!({ ... })`
Creates a JSON value using JSON literal syntax.

**Returns**: Value object

**Example**:
```raven
let data = json!({
    "string": "hello",
    "number": 42,
    "boolean": true,
    "null": null,
    "array": [1, 2, 3],
    "nested": {
        "key": "value"
    }
});
```

---

### Value Methods

#### `value[key]`
Accesses object property or array element.

**Example**:
```raven
let data = json!({"name": "Alice", "items": [1, 2, 3]});
let name = data["name"];
let first_item = data["items"][0];
```

#### `value.as_str() -> Option<&str>`
Converts to string if possible.

**Returns**: Some(string) or None

**Example**:
```raven
let name = data["name"].as_str().unwrap();
```

#### `value.as_i64() -> Option<i64>`
Converts to integer if possible.

**Returns**: Some(number) or None

**Example**:
```raven
let age = data["age"].as_i64().unwrap() as i32;
```

#### `value.as_f64() -> Option<f64>`
Converts to float if possible.

**Returns**: Some(number) or None

#### `value.as_bool() -> Option<bool>`
Converts to boolean if possible.

**Returns**: Some(bool) or None

#### `value.is_null() -> bool`
Checks if value is null.

**Returns**: true if null

---

## 9. String

**Module**: `String::`
**Size**: 178 lines
**Tests**: 5 passing
**Import**: Auto-imported (global namespace)

### Case Conversion

#### `String::to_uppercase(s: &str) -> String`
Converts to uppercase.

**Parameters**:
- `s` - Input string

**Returns**: Uppercase string

**Example**:
```raven
let upper = String::to_uppercase("hello");  // "HELLO"
```

#### `String::to_lowercase(s: &str) -> String`
Converts to lowercase.

**Parameters**:
- `s` - Input string

**Returns**: Lowercase string

**Example**:
```raven
let lower = String::to_lowercase("HELLO");  // "hello"
```

---

### Trimming

#### `String::trim(s: &str) -> String`
Removes whitespace from both ends.

**Parameters**:
- `s` - Input string

**Returns**: Trimmed string

**Example**:
```raven
let trimmed = String::trim("  hello  ");  // "hello"
```

#### `String::trim_start(s: &str) -> String`
Removes whitespace from start.

**Parameters**:
- `s` - Input string

**Returns**: Left-trimmed string

#### `String::trim_end(s: &str) -> String`
Removes whitespace from end.

**Parameters**:
- `s` - Input string

**Returns**: Right-trimmed string

---

### Splitting & Joining

#### `String::split(s: &str, delimiter: &str) -> Vec<String>`
Splits string by delimiter.

**Parameters**:
- `s` - Input string
- `delimiter` - Split delimiter

**Returns**: Vector of parts

**Example**:
```raven
let parts = String::split("a,b,c", ",");
// ["a", "b", "c"]
```

#### `String::join(parts: &Vec<String>, separator: &str) -> String`
Joins strings with separator.

**Parameters**:
- `parts` - Strings to join
- `separator` - Separator string

**Returns**: Joined string

**Example**:
```raven
let parts = vec!["a", "b", "c"];
let joined = String::join(&parts, ", ");
// "a, b, c"
```

---

### Searching

#### `String::contains(haystack: &str, needle: &str) -> bool`
Checks if string contains substring.

**Parameters**:
- `haystack` - String to search in
- `needle` - Substring to find

**Returns**: true if found

**Example**:
```raven
let has_at = String::contains("alice@example.com", "@");
// true
```

#### `String::starts_with(s: &str, prefix: &str) -> bool`
Checks if string starts with prefix.

**Parameters**:
- `s` - Input string
- `prefix` - Prefix to check

**Returns**: true if starts with prefix

**Example**:
```raven
let is_https = String::starts_with(url, "https://");
```

#### `String::ends_with(s: &str, suffix: &str) -> bool`
Checks if string ends with suffix.

**Parameters**:
- `s` - Input string
- `suffix` - Suffix to check

**Returns**: true if ends with suffix

**Example**:
```raven
let is_js = String::ends_with(filename, ".js");
```

#### `String::index_of(haystack: &str, needle: &str) -> Option<usize>`
Finds first occurrence index.

**Parameters**:
- `haystack` - String to search in
- `needle` - Substring to find

**Returns**: Some(index) or None

**Example**:
```raven
match String::index_of("hello world", "world") {
    Some(idx) => println!("Found at {}", idx),  // 6
    None => println!("Not found"),
}
```

---

### Replacement

#### `String::replace(s: &str, from: &str, to: &str) -> String`
Replaces all occurrences.

**Parameters**:
- `s` - Input string
- `from` - String to replace
- `to` - Replacement string

**Returns**: String with replacements

**Example**:
```raven
let result = String::replace("hello world", "world", "Raven");
// "hello Raven"
```

#### `String::replace_first(s: &str, from: &str, to: &str) -> String`
Replaces first occurrence only.

**Parameters**:
- `s` - Input string
- `from` - String to replace
- `to` - Replacement string

**Returns**: String with first replacement

---

### Character Operations

#### `String::char_at(s: &str, index: usize) -> Option<char>`
Gets character at index.

**Parameters**:
- `s` - Input string
- `index` - Character index

**Returns**: Some(char) or None

**Example**:
```raven
let first = String::char_at("hello", 0);  // Some('h')
```

#### `String::length(s: &str) -> usize`
Gets string length.

**Parameters**:
- `s` - Input string

**Returns**: Number of characters

**Example**:
```raven
let len = String::length("hello");  // 5
```

---

## 10. Time

**Module**: `Time::`
**Size**: 156 lines
**Tests**: 0 (time-dependent)
**Import**: Auto-imported (global namespace)

### Current Time

#### `Time::now() -> i64`
Gets current Unix timestamp (seconds).

**Returns**: Seconds since Unix epoch

**Example**:
```raven
let timestamp = Time::now();
println!("Current time: {}", timestamp);
```

#### `Time::now_millis() -> i64`
Gets current Unix timestamp (milliseconds).

**Returns**: Milliseconds since Unix epoch

**Example**:
```raven
let ms = Time::now_millis();
```

---

### Formatting

#### `Time::format(timestamp: i64, format: &str) -> String`
Formats timestamp as string.

**Parameters**:
- `timestamp` - Unix timestamp
- `format` - Format string (strftime)

**Returns**: Formatted date/time string

**Example**:
```raven
let timestamp = Time::now();
let formatted = Time::format(timestamp, "%Y-%m-%d %H:%M:%S");
// "2025-10-21 14:30:00"
```

#### `Time::parse(date_str: &str, format: &str) -> Result<i64, String>`
Parses date string to timestamp.

**Parameters**:
- `date_str` - Date string
- `format` - Format string (strftime)

**Returns**: Result with timestamp or error

**Example**:
```raven
let timestamp = Time::parse("2025-10-21", "%Y-%m-%d").unwrap();
```

---

### Utilities

#### `Time::sleep(ms: u64)`
Sleeps for specified milliseconds (blocking).

**Parameters**:
- `ms` - Milliseconds to sleep

**Example**:
```raven
Time::sleep(1000);  // Sleep 1 second
```

#### `Time::sleep_async(ms: u64) -> Future<()>`
Sleeps asynchronously (non-blocking).

**Parameters**:
- `ms` - Milliseconds to sleep

**Returns**: Future that resolves after delay

**Example**:
```raven
@server
async fn delayed_action() {
    Time::sleep_async(1000).await;
    println!("1 second later");
}
```

---

## 11. Crypto

**Module**: `Crypto::`
**Size**: 145 lines
**Tests**: 0 (cryptographic functions)
**Import**: Auto-imported for `@server` functions

### Hashing

#### `Crypto::sha256(data: &str) -> String`
Computes SHA-256 hash.

**Parameters**:
- `data` - Input data

**Returns**: Hex-encoded hash

**Example**:
```raven
@server
fn hash_file(contents: String) -> String {
    return Crypto::sha256(&contents);
}
```

#### `Crypto::md5(data: &str) -> String`
Computes MD5 hash (insecure, use for non-security purposes).

**Parameters**:
- `data` - Input data

**Returns**: Hex-encoded hash

**Example**:
```raven
let checksum = Crypto::md5(&file_contents);
```

---

### Encoding

#### `Crypto::base64_encode(data: &Vec<u8>) -> String`
Encodes bytes as base64.

**Parameters**:
- `data` - Byte data

**Returns**: Base64 string

**Example**:
```raven
let bytes = vec![72, 101, 108, 108, 111];
let encoded = Crypto::base64_encode(&bytes);
// "SGVsbG8="
```

#### `Crypto::base64_decode(encoded: &str) -> Result<Vec<u8>, String>`
Decodes base64 string.

**Parameters**:
- `encoded` - Base64 string

**Returns**: Result with bytes or error

**Example**:
```raven
let bytes = Crypto::base64_decode("SGVsbG8=").unwrap();
```

#### `Crypto::hex_encode(data: &Vec<u8>) -> String`
Encodes bytes as hexadecimal.

**Parameters**:
- `data` - Byte data

**Returns**: Hex string

#### `Crypto::hex_decode(hex: &str) -> Result<Vec<u8>, String>`
Decodes hexadecimal string.

**Parameters**:
- `hex` - Hex string

**Returns**: Result with bytes or error

---

### Random Generation

#### `Crypto::random_bytes(length: usize) -> Vec<u8>`
Generates cryptographically secure random bytes.

**Parameters**:
- `length` - Number of bytes

**Returns**: Random bytes

**Example**:
```raven
@server
fn generate_key() -> Vec<u8> {
    return Crypto::random_bytes(32);  // 256-bit key
}
```

#### `Crypto::uuid() -> String`
Generates a UUID v4.

**Returns**: UUID string

**Example**:
```raven
let id = Crypto::uuid();
// "550e8400-e29b-41d4-a716-446655440000"
```

---

## 12. Router

**Module**: `Router::`
**Size**: 123 lines
**Tests**: 0 (requires browser)
**Import**: Auto-imported for `@client` code

### Route Definition

#### `Router::new() -> Router`
Creates a new router instance.

**Returns**: Router

**Example**:
```raven
@client
fn init_app() {
    let router = Router::new();
    router.add_route("/", home_page);
    router.add_route("/about", about_page);
    router.start();
}
```

#### `router.add_route(path: &str, handler: fn())`
Registers a route handler.

**Parameters**:
- `path` - URL path pattern
- `handler` - Function to call when route matches

**Example**:
```raven
router.add_route("/users/:id", show_user);
```

#### `router.start()`
Starts the router (begins listening for navigation).

**Example**:
```raven
router.start();
```

---

### Navigation

#### `Router::navigate(path: &str)`
Programmatically navigates to a path.

**Parameters**:
- `path` - Path to navigate to

**Example**:
```raven
@client
fn go_to_profile() {
    Router::navigate("/profile");
}
```

#### `Router::back()`
Navigates back in history.

**Example**:
```raven
@client
fn handle_back_click() {
    Router::back();
}
```

#### `Router::forward()`
Navigates forward in history.

---

### Route Parameters

#### `Router::get_param(name: &str) -> Option<String>`
Gets route parameter value.

**Parameters**:
- `name` - Parameter name (from :param in path)

**Returns**: Some(value) or None

**Example**:
```raven
@client
fn show_user() {
    match Router::get_param("id") {
        Some(id) => load_user(id),
        None => show_error(),
    }
}
```

#### `Router::get_query(name: &str) -> Option<String>`
Gets query parameter value.

**Parameters**:
- `name` - Query parameter name

**Returns**: Some(value) or None

**Example**:
```raven
// URL: /search?q=raven&limit=10
let query = Router::get_query("q");  // Some("raven")
let limit = Router::get_query("limit");  // Some("10")
```

---

## 13. Forms

**Module**: `Forms::`
**Size**: 112 lines
**Tests**: 0 (requires browser)
**Import**: Auto-imported for `@client` code

### Form Validation

#### `Forms::validate_email(email: &str) -> bool`
Validates email format.

**Parameters**:
- `email` - Email string

**Returns**: true if valid

**Example**:
```raven
@client
fn check_email(email: String) -> bool {
    if !Forms::validate_email(&email) {
        show_error("Invalid email");
        return false;
    }
    return true;
}
```

#### `Forms::validate_url(url: &str) -> bool`
Validates URL format.

**Parameters**:
- `url` - URL string

**Returns**: true if valid

#### `Forms::validate_required(value: &str) -> bool`
Checks if value is non-empty.

**Parameters**:
- `value` - Input value

**Returns**: true if not empty

**Example**:
```raven
if !Forms::validate_required(&username) {
    errors.push("Username is required");
}
```

#### `Forms::validate_min_length(value: &str, min: usize) -> bool`
Checks minimum length.

**Parameters**:
- `value` - Input value
- `min` - Minimum length

**Returns**: true if long enough

**Example**:
```raven
if !Forms::validate_min_length(&password, 8) {
    errors.push("Password must be at least 8 characters");
}
```

#### `Forms::validate_max_length(value: &str, max: usize) -> bool`
Checks maximum length.

**Parameters**:
- `value` - Input value
- `max` - Maximum length

**Returns**: true if short enough

#### `Forms::validate_pattern(value: &str, pattern: &str) -> bool`
Validates against regex pattern.

**Parameters**:
- `value` - Input value
- `pattern` - Regex pattern

**Returns**: true if matches

**Example**:
```raven
// Validate phone number
if !Forms::validate_pattern(&phone, r"^\d{3}-\d{3}-\d{4}$") {
    errors.push("Invalid phone format");
}
```

---

### Form Data

#### `Forms::get_form_data(form_id: &str) -> HashMap<String, String>`
Gets all form field values.

**Parameters**:
- `form_id` - Form element ID

**Returns**: Map of field names to values

**Example**:
```raven
@client
fn handle_submit() {
    let data = Forms::get_form_data("login-form");
    let username = data["username"];
    let password = data["password"];
}
```

#### `Forms::set_field_value(field_id: &str, value: &str)`
Sets form field value.

**Parameters**:
- `field_id` - Field element ID
- `value` - Value to set

**Example**:
```raven
@client
fn prefill_form() {
    Forms::set_field_value("email", "user@example.com");
}
```

#### `Forms::clear_form(form_id: &str)`
Clears all form fields.

**Parameters**:
- `form_id` - Form element ID

**Example**:
```raven
@client
fn reset_form() {
    Forms::clear_form("contact-form");
}
```

---

## 14. WebSocket

**Module**: `WebSocket::`
**Size**: 98 lines
**Tests**: 0 (requires WebSocket server)
**Import**: Auto-imported (both `@server` and `@client`)

### Connection

#### `WebSocket::connect(url: &str) -> Result<WebSocket, String>`
Connects to WebSocket server.

**Parameters**:
- `url` - WebSocket URL (ws:// or wss://)

**Returns**: Result with WebSocket connection or error

**Example**:
```raven
@client
fn init_websocket() {
    match WebSocket::connect("wss://api.example.com/ws") {
        Ok(ws) => {
            println!("Connected!");
            ws.on_message(handle_message);
        }
        Err(err) => println!("Connection failed: {}", err),
    }
}
```

---

### Message Handling

#### `ws.on_message(handler: fn(String))`
Sets message handler.

**Parameters**:
- `handler` - Function called with each message

**Example**:
```raven
@client
fn handle_message(msg: String) {
    println!("Received: {}", msg);
    let data = JSON::parse(&msg).unwrap();
    update_ui(data);
}

ws.on_message(handle_message);
```

#### `ws.send(message: &str)`
Sends a text message.

**Parameters**:
- `message` - Message to send

**Example**:
```raven
@client
fn send_chat_message(text: String) {
    let msg = json!({
        "type": "chat",
        "text": text
    });
    ws.send(&JSON::stringify(&msg));
}
```

#### `ws.send_json(data: &Value)`
Sends JSON data (convenience).

**Parameters**:
- `data` - JSON value

**Example**:
```raven
let data = json!({"event": "ping"});
ws.send_json(&data);
```

---

### Connection Management

#### `ws.on_open(handler: fn())`
Sets connection opened handler.

**Parameters**:
- `handler` - Function called when connection opens

**Example**:
```raven
ws.on_open(|| {
    println!("WebSocket connected");
    send_auth_token();
});
```

#### `ws.on_close(handler: fn())`
Sets connection closed handler.

**Parameters**:
- `handler` - Function called when connection closes

**Example**:
```raven
ws.on_close(|| {
    println!("WebSocket disconnected");
    attempt_reconnect();
});
```

#### `ws.on_error(handler: fn(String))`
Sets error handler.

**Parameters**:
- `handler` - Function called with error message

**Example**:
```raven
ws.on_error(|err| {
    println!("WebSocket error: {}", err);
});
```

#### `ws.close()`
Closes the WebSocket connection.

**Example**:
```raven
@client
fn disconnect() {
    ws.close();
}
```

---

## 15. Storage

**Module**: `Storage::`
**Size**: 89 lines
**Tests**: 0 (requires browser)
**Import**: Auto-imported for `@client` code

### LocalStorage

#### `Storage::set(key: &str, value: &str)`
Stores a value in localStorage.

**Parameters**:
- `key` - Storage key
- `value` - Value to store

**Example**:
```raven
@client
fn save_preferences(theme: String) {
    Storage::set("theme", &theme);
}
```

#### `Storage::get(key: &str) -> Option<String>`
Retrieves a value from localStorage.

**Parameters**:
- `key` - Storage key

**Returns**: Some(value) or None

**Example**:
```raven
@client
fn load_theme() -> String {
    match Storage::get("theme") {
        Some(theme) => theme,
        None => "light".to_string(),
    }
}
```

#### `Storage::remove(key: &str)`
Removes a value from localStorage.

**Parameters**:
- `key` - Storage key

**Example**:
```raven
@client
fn logout() {
    Storage::remove("auth_token");
}
```

#### `Storage::clear()`
Clears all localStorage data.

**Example**:
```raven
@client
fn reset_app() {
    Storage::clear();
}
```

---

### SessionStorage

#### `Storage::session_set(key: &str, value: &str)`
Stores in sessionStorage (cleared on tab close).

**Parameters**:
- `key` - Storage key
- `value` - Value to store

**Example**:
```raven
@client
fn save_temp_data(data: String) {
    Storage::session_set("temp", &data);
}
```

#### `Storage::session_get(key: &str) -> Option<String>`
Retrieves from sessionStorage.

**Parameters**:
- `key` - Storage key

**Returns**: Some(value) or None

#### `Storage::session_remove(key: &str)`
Removes from sessionStorage.

**Parameters**:
- `key` - Storage key

#### `Storage::session_clear()`
Clears all sessionStorage.

---

### JSON Storage Helpers

#### `Storage::set_json(key: &str, value: &Value)`
Stores JSON value in localStorage.

**Parameters**:
- `key` - Storage key
- `value` - JSON value

**Example**:
```raven
@client
fn save_cart(items: Vec<CartItem>) {
    let data = json!({"items": items, "updated": Time::now()});
    Storage::set_json("cart", &data);
}
```

#### `Storage::get_json(key: &str) -> Option<Value>`
Retrieves and parses JSON from localStorage.

**Parameters**:
- `key` - Storage key

**Returns**: Some(parsed JSON) or None

**Example**:
```raven
@client
fn load_cart() -> Vec<CartItem> {
    match Storage::get_json("cart") {
        Some(data) => parse_cart_items(&data["items"]),
        None => vec![],
    }
}
```

---

## 16. I18n

**Module**: `I18n::`
**Size**: 76 lines
**Tests**: 0 (requires translation files)
**Import**: Auto-imported (global namespace)

### Translation

#### `I18n::init(locale: &str, translations: Value)`
Initializes i18n with translations.

**Parameters**:
- `locale` - Locale code (e.g., "en", "es", "fr")
- `translations` - Translation dictionary

**Example**:
```raven
@client
fn init_i18n() {
    let translations = json!({
        "en": {
            "welcome": "Welcome",
            "goodbye": "Goodbye"
        },
        "es": {
            "welcome": "Bienvenido",
            "goodbye": "Adiós"
        }
    });

    I18n::init("en", translations);
}
```

#### `I18n::t(key: &str) -> String`
Translates a key to current locale.

**Parameters**:
- `key` - Translation key

**Returns**: Translated string

**Example**:
```raven
@client
fn render_greeting() {
    let welcome = I18n::t("welcome");
    println!("{}", welcome);  // "Welcome" or "Bienvenido"
}
```

#### `I18n::t_with_params(key: &str, params: &HashMap<String, String>) -> String`
Translates with parameter substitution.

**Parameters**:
- `key` - Translation key
- `params` - Parameters for substitution

**Returns**: Translated string with substituted parameters

**Example**:
```raven
// Translation: "greeting": "Hello, {name}!"
let mut params = HashMap::new();
params.insert("name", "Alice");
let greeting = I18n::t_with_params("greeting", &params);
// "Hello, Alice!"
```

---

### Locale Management

#### `I18n::set_locale(locale: &str)`
Changes current locale.

**Parameters**:
- `locale` - Locale code

**Example**:
```raven
@client
fn switch_to_spanish() {
    I18n::set_locale("es");
    re_render_app();
}
```

#### `I18n::get_locale() -> String`
Gets current locale.

**Returns**: Current locale code

**Example**:
```raven
@client
fn get_current_language() -> String {
    return I18n::get_locale();
}
```

---

### Pluralization

#### `I18n::plural(key: &str, count: i32) -> String`
Handles plural forms.

**Parameters**:
- `key` - Translation key
- `count` - Item count

**Returns**: Appropriate plural form

**Example**:
```raven
// Translations:
// "items.one": "1 item"
// "items.other": "{count} items"

let text1 = I18n::plural("items", 1);   // "1 item"
let text2 = I18n::plural("items", 5);   // "5 items"
```

---

## Appendix A: Common Patterns

### Error Handling

```raven
// Pattern 1: Early return
fn process_user(id: i32) -> Result<User, String> {
    let user = db.get_user(id);
    if user.is_none() {
        return Err("User not found");
    }

    return Ok(user.unwrap());
}

// Pattern 2: Match expression
fn load_config() -> Config {
    match Fs::read_file("config.json") {
        Ok(contents) => JSON::parse(&contents).unwrap(),
        Err(_) => Config::default(),
    }
}
```

### Reactive State Management

```raven
// Counter with derived state
let count = Signal::new(0);
let doubled = Computed::new(|| count.get() * 2);
let is_even = Computed::new(|| count.get() % 2 == 0);

create_effect(|| {
    println!("Count: {} (doubled: {}, even: {})",
             count.get(), doubled.get(), is_even.get());
});

// Update triggers all dependents
count.set(5);
```

### HTTP API Client

```raven
@server
struct ApiClient {
    base_url: String,
    api_key: String,
}

impl ApiClient {
    @server
    async fn get(&self, path: String) -> Result<Value, String> {
        let url = format!("{}{}", self.base_url, path);

        let response = HttpRequest::get(&url)
            .header("X-API-Key", &self.api_key)
            .timeout(5000)
            .send()
            .await?;

        if !response.is_ok() {
            return Err(format!("HTTP {}", response.status));
        }

        return response.json();
    }
}
```

### Form Validation

```raven
@client
fn validate_form(data: FormData) -> Vec<String> {
    let mut errors = vec![];

    if !Forms::validate_required(&data.email) {
        errors.push("Email is required");
    } else if !Forms::validate_email(&data.email) {
        errors.push("Invalid email format");
    }

    if !Forms::validate_min_length(&data.password, 8) {
        errors.push("Password must be at least 8 characters");
    }

    return errors;
}
```

---

## Appendix B: Quick Reference

### Most Common Functions

**Math**: `abs`, `min`, `max`, `pow`, `sqrt`, `round`, `random`
**Reactive**: `Signal::new`, `.get()`, `.set()`, `Computed::new`, `create_effect`
**HTTP**: `HttpRequest::get`, `.json()`, `.send().await`, `get()`, `post_json()`
**Collections**: `filter`, `map`, `find`, `reduce`
**String**: `split`, `join`, `contains`, `replace`, `trim`
**JSON**: `JSON::parse`, `JSON::stringify`, `json!`, `.as_str()`
**Storage**: `Storage::set`, `Storage::get`, `Storage::set_json`
**Time**: `Time::now()`, `Time::format()`
**Auth**: `Auth::hash_password`, `Auth::verify_password`, `Auth::create_jwt`

### Type Conversions

```raven
// String to number
let num = "42".parse::<i32>().unwrap();
let flt = "3.14".parse::<f64>().unwrap();

// Number to string
let s1 = 42.to_string();
let s2 = format!("{}", 3.14);

// JSON value to specific type
let name = data["name"].as_str().unwrap();
let age = data["age"].as_i64().unwrap() as i32;
```

---

**End of API Reference**

For examples and tutorials, see:
- `examples/stdlib/` - Complete working examples
- `GETTING_STARTED.md` - Getting started guide
- `CLAUDE.md` - Development guide
