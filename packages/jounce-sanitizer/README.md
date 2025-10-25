# jounce-sanitizer

Security utilities and input sanitization for preventing XSS, SQL injection, path traversal, and other vulnerabilities.

## Features

- ✅ **HTML Sanitization** - XSS prevention with HTML escaping
- ✅ **SQL Injection Prevention** - Escape SQL values and identifiers
- ✅ **Path Traversal Prevention** - Safe file paths and filenames
- ✅ **URL Sanitization** - Block dangerous URL schemes
- ✅ **Header Injection Prevention** - Prevent CRLF injection
- ✅ **Command Injection Prevention** - Safe shell argument escaping
- ✅ **Input Validation** - Length and pattern validation
- ✅ **Common Patterns** - Email, alphanumeric, numeric validation

## Installation

```bash
jnc pkg add jounce-sanitizer
```

## Quick Start

```jounce
use jounce_sanitizer::Sanitizer;

// HTML sanitization (XSS prevention)
let safe_html = Sanitizer::html("<script>alert('xss')</script>");
// Output: "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"

// SQL injection prevention
let safe_sql = Sanitizer::sql("O'Reilly");
// Output: "O''Reilly"

// URL sanitization
let safe_url = Sanitizer::url("javascript:alert('xss')");
// Output: ""

// Filename sanitization
let safe_filename = Sanitizer::filename("../../etc/passwd");
// Output: "etcpasswd"
```

## Usage

### HTML Sanitization (XSS Prevention)

```jounce
use jounce_sanitizer::{escape_html, unescape_html, strip_html, HtmlSanitizer};

// Escape HTML entities
let escaped = escape_html("<script>alert('xss')</script>");
// "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"

// Unescape HTML
let unescaped = unescape_html("&lt;div&gt;");
// "<div>"

// Strip all HTML tags
let plain = strip_html("<p>Hello <strong>world</strong></p>");
// "Hello world"

// Custom sanitizer
let sanitizer = HtmlSanitizer::basic();  // Allows p, br, strong, em, etc.
let safe = sanitizer.sanitize(user_input);
```

### SQL Injection Prevention

```jounce
use jounce_sanitizer::{escape_sql, quote_sql_identifier, is_safe_sql_identifier};

// Escape SQL string values
let safe_value = escape_sql("O'Reilly");
// "O''Reilly"

// Quote SQL identifiers (table/column names)
let safe_identifier = quote_sql_identifier("user_table");
// "\"user_table\""

// Validate identifier safety
if is_safe_sql_identifier(table_name) {
    // Safe to use without quoting
}

// Example query (use parameterized queries in production!)
let query = "SELECT * FROM " + quote_sql_identifier(table) +
            " WHERE name = '" + escape_sql(name) + "'";
```

### Path Traversal Prevention

```jounce
use jounce_sanitizer::{sanitize_path, is_safe_filename, sanitize_filename};

// Sanitize file paths
let safe_path = sanitize_path("../../../etc/passwd");
// "etcpasswd"

// Check filename safety
if is_safe_filename(filename) {
    // Safe to use
} else {
    // Reject - contains path traversal or dangerous characters
}

// Sanitize filename (remove unsafe characters)
let safe_filename = sanitize_filename("my document!@#$.pdf");
// "mydocument.pdf"
```

### URL Sanitization

```jounce
use jounce_sanitizer::{is_safe_url, sanitize_url, encode_url_component};

// Check URL safety
if is_safe_url(url) {
    // Safe - http://, https://, or relative
} else {
    // Dangerous - javascript:, data:, file:, etc.
}

// Sanitize URL (returns empty string if unsafe)
let safe_url = sanitize_url("javascript:alert('xss')");
// ""

let safe_url = sanitize_url("https://example.com");
// "https://example.com"

// URL encode components
let encoded = encode_url_component("hello world");
// "hello%20world"
```

### Header Injection Prevention

```jounce
use jounce_sanitizer::{sanitize_header_value, is_safe_header_value};

// Remove CRLF from header values
let safe_header = sanitize_header_value("value\r\nX-Injected: evil");
// "valueX-Injected: evil"

// Check header safety
if is_safe_header_value(value) {
    // Safe to use in HTTP header
}
```

### Command Injection Prevention

```jounce
use jounce_sanitizer::{escape_shell_arg, is_safe_shell_arg};

// Escape shell arguments
let safe_arg = escape_shell_arg("file name.txt");
// "'file name.txt'"

// Check argument safety
if is_safe_shell_arg(arg) {
    // Safe - no shell metacharacters
} else {
    // Dangerous - contains ;, |, &, $, etc.
}
```

### Input Validation

```jounce
use jounce_sanitizer::InputValidator;

// Create validator
let validator = InputValidator::new()
    .with_min_length(5)
    .with_max_length(100)
    .require_non_empty();

// Validate input
let result = validator.validate(user_input);

if result.valid {
    // Input is valid
} else {
    println("Error: " + result.error);
}
```

### Common Pattern Validation

```jounce
use jounce_sanitizer::{is_alphanumeric, is_email, is_numeric};

// Alphanumeric check
if is_alphanumeric(username) {
    // Only letters and numbers
}

// Email validation
if is_email(email) {
    // Contains @ and .
}

// Numeric check
if is_numeric(input) {
    // Only digits
}
```

### Complete Example: User Input

```jounce
use jounce_sanitizer::{escape_html, is_email, InputValidator};

fn process_user_input(name: string, email: string, bio: string) -> Result<(), string> {
    // Validate name length
    let name_validator = InputValidator::new()
        .with_min_length(2)
        .with_max_length(50)
        .require_non_empty();

    let name_result = name_validator.validate(name.clone());
    if !name_result.valid {
        return Err("Invalid name: " + name_result.error);
    }

    // Validate email format
    if !is_email(email.clone()) {
        return Err("Invalid email format");
    }

    // Sanitize bio for display (prevent XSS)
    let safe_bio = escape_html(bio);

    // Store in database (use parameterized queries!)
    println("Name: " + name);
    println("Email: " + email);
    println("Bio: " + safe_bio);

    Ok(())
}
```

### Complete Example: File Upload

```jounce
use jounce_sanitizer::{is_safe_filename, sanitize_filename};

fn handle_file_upload(filename: string, content: string) -> Result<string, string> {
    // Check filename safety
    if !is_safe_filename(filename.clone()) {
        return Err("Unsafe filename");
    }

    // Sanitize filename
    let safe_name = sanitize_filename(filename);

    if safe_name.len() == 0 {
        return Err("Invalid filename");
    }

    // Save file
    let path = "/uploads/" + safe_name;
    println("Saving to: " + path);

    Ok(path)
}
```

### Complete Example: API Security

```jounce
use jounce_sanitizer::{escape_html, sanitize_header_value, InputValidator};

fn handle_api_request(user_id: string, content: string, custom_header: string) -> Response {
    // Validate user ID format
    if !is_alphanumeric(user_id.clone()) {
        return Response::bad_request("Invalid user ID");
    }

    // Validate content length
    let validator = InputValidator::new().with_max_length(1000);
    let result = validator.validate(content.clone());

    if !result.valid {
        return Response::bad_request(result.error);
    }

    // Sanitize content for storage/display
    let safe_content = escape_html(content);

    // Sanitize custom header
    let safe_header = sanitize_header_value(custom_header);

    Response::ok()
        .with_header("X-Custom", safe_header)
        .with_body(safe_content)
}
```

## API Reference

### HTML Sanitization

```jounce
escape_html(text: string) -> string
unescape_html(text: string) -> string
strip_html(html: string) -> string
HtmlSanitizer::new() -> HtmlSanitizer
HtmlSanitizer::strict() -> HtmlSanitizer
HtmlSanitizer::basic() -> HtmlSanitizer
sanitizer.with_tag(tag: string) -> HtmlSanitizer
sanitizer.sanitize(html: string) -> string
```

### SQL Sanitization

```jounce
escape_sql(value: string) -> string
quote_sql_identifier(identifier: string) -> string
is_safe_sql_identifier(identifier: string) -> bool
```

### Path Sanitization

```jounce
sanitize_path(path: string) -> string
is_safe_filename(filename: string) -> bool
sanitize_filename(filename: string) -> string
```

### URL Sanitization

```jounce
is_safe_url(url: string) -> bool
sanitize_url(url: string) -> string
encode_url_component(value: string) -> string
```

### Header Sanitization

```jounce
sanitize_header_value(value: string) -> string
is_safe_header_value(value: string) -> bool
```

### Shell Sanitization

```jounce
escape_shell_arg(arg: string) -> string
is_safe_shell_arg(arg: string) -> bool
```

### Input Validation

```jounce
InputValidator::new() -> InputValidator
validator.with_min_length(min: int) -> InputValidator
validator.with_max_length(max: int) -> InputValidator
validator.require_non_empty() -> InputValidator
validator.validate(input: string) -> ValidationResult
```

### Common Patterns

```jounce
is_alphanumeric(text: string) -> bool
is_email(email: string) -> bool
is_numeric(text: string) -> bool
```

### Sanitizer Presets

```jounce
Sanitizer::html(text: string) -> string
Sanitizer::sql(value: string) -> string
Sanitizer::url(url: string) -> string
Sanitizer::path(path: string) -> string
Sanitizer::filename(filename: string) -> string
Sanitizer::header(value: string) -> string
Sanitizer::shell(arg: string) -> string
```

## Security Best Practices

1. **Defense in Depth** - Use multiple layers of security
2. **Validate Input** - Always validate user input
3. **Sanitize Output** - Sanitize based on context (HTML, SQL, URL, etc.)
4. **Parameterized Queries** - Use prepared statements for SQL
5. **Content Security Policy** - Add CSP headers
6. **HTTPS Only** - Use HTTPS for all communication
7. **Principle of Least Privilege** - Minimize permissions
8. **Regular Updates** - Keep dependencies updated

## Common Vulnerabilities Prevented

| Vulnerability | Prevention |
|---------------|------------|
| **XSS (Cross-Site Scripting)** | `escape_html()`, `HtmlSanitizer` |
| **SQL Injection** | `escape_sql()`, `quote_sql_identifier()` |
| **Path Traversal** | `sanitize_path()`, `is_safe_filename()` |
| **Command Injection** | `escape_shell_arg()`, `is_safe_shell_arg()` |
| **Header Injection** | `sanitize_header_value()` |
| **URL Injection** | `is_safe_url()`, `sanitize_url()` |

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
