# jounce-validation

Comprehensive form and data validation with built-in validators, custom rules, and clear error messaging.

## Features

- ✅ **Field Validators** - Validate individual fields with chained rules
- ✅ **Form Validators** - Validate entire forms with multiple fields
- ✅ **Built-in Validators** - 15+ common validation rules (email, URL, phone, etc.)
- ✅ **Custom Validators** - Create your own validation rules
- ✅ **Conditional Validation** - Apply validation based on conditions
- ✅ **Async Validation** - Support for server-side validation
- ✅ **Clear Error Messages** - User-friendly validation messages
- ✅ **Fluent API** - Chainable validation rules

## Installation

```bash
jnc pkg add jounce-validation
```

## Quick Start

```jounce
use jounce_validation::{FieldValidator, FormValidator};

// Validate a single field
let email_validator = FieldValidator::new("email")
    .required()
    .email()
    .max_length(100);

let result = email_validator.validate("user@example.com");

if result.is_valid() {
    println("Email is valid!");
} else {
    println("Errors: " + result.errors.len().to_string());
}

// Validate a form
let mut form_validator = FormValidator::new();

form_validator = form_validator
    .add_field(FieldValidator::new("email").required().email())
    .add_field(FieldValidator::new("password").required().min_length(8));

let mut data = Map::new();
data.insert("email", "user@example.com");
data.insert("password", "secure123");

let form_result = form_validator.validate(data);
```

## Usage

### Field Validation

```jounce
use jounce_validation::FieldValidator;

// Required field
let name_validator = FieldValidator::new("name")
    .required();

// Email validation
let email_validator = FieldValidator::new("email")
    .required()
    .email()
    .max_length(100);

// Password validation
let password_validator = FieldValidator::new("password")
    .required()
    .min_length(8)
    .max_length(50);

// URL validation
let website_validator = FieldValidator::new("website")
    .url();

// Numeric validation
let age_validator = FieldValidator::new("age")
    .numeric()
    .min_value(18)
    .max_value(120);

// Alphanumeric validation
let username_validator = FieldValidator::new("username")
    .required()
    .alphanumeric()
    .min_length(3)
    .max_length(20);
```

### Form Validation

```jounce
use jounce_validation::{FormValidator, FieldValidator};

// Create form validator
let mut form_validator = FormValidator::new();

// Add fields
let email_field = FieldValidator::new("email").required().email();
let password_field = FieldValidator::new("password").required().min_length(8);
let name_field = FieldValidator::new("name").required().min_length(2);

form_validator = form_validator
    .add_field(email_field)
    .add_field(password_field)
    .add_field(name_field);

// Validate data
let mut data = Map::new();
data.insert("email", "user@example.com");
data.insert("password", "secure123");
data.insert("name", "John Doe");

let result = form_validator.validate(data);

if result.is_valid() {
    println("Form is valid!");
} else {
    for error in result.errors {
        println(error.field + ": " + error.message);
    }
}
```

### Optional Fields

```jounce
use jounce_validation::FieldValidator;

// Optional field - only validates if not empty
let phone_validator = FieldValidator::new("phone")
    .with_optional()
    .min_length(10);

let result = phone_validator.validate(""); // Valid - optional field can be empty
```

### Built-in Validators

```jounce
use jounce_validation::Validators;

// Required
let required_rule = Validators::required();

// Email
let email_rule = Validators::email();

// URL
let url_rule = Validators::url();

// Phone number
let phone_rule = Validators::phone();

// Postal code
let postal_rule = Validators::postal_code();

// Credit card
let card_rule = Validators::credit_card();

// IP address
let ip_rule = Validators::ip_address();

// Min/max length
let min_rule = Validators::min_length(5);
let max_rule = Validators::max_length(100);
let exact_rule = Validators::exact_length(10);

// Matches another field
let confirm_rule = Validators::matches("password", password_value);

// Different from another field
let different_rule = Validators::different_from("username", "admin");

// In list
let allowed = ["red", "green", "blue"];
let in_list_rule = Validators::in_list(allowed);

// Not in list
let forbidden = ["admin", "root"];
let not_in_list_rule = Validators::not_in_list(forbidden);
```

### Custom Validators

```jounce
use jounce_validation::Validators;

// Create custom validator
let starts_with_jnc = Validators::custom(
    "starts_with_jnc",
    |value| value.starts_with("jnc_"),
    "Must start with jnc_"
);

// Use in field validator
let field = FieldValidator::new("filename")
    .required()
    .add_rule(starts_with_jnc);
```

### Validation Schema

```jounce
use jounce_validation::ValidationSchema;

// Build schema
let mut schema = ValidationSchema::new();

let email_field = schema.field("email").required().email();
let password_field = schema.field("password").required().min_length(8);

schema = schema
    .add_validator(email_field)
    .add_validator(password_field);

let form_validator = schema.build();
```

### Conditional Validation

```jounce
use jounce_validation::ConditionalValidator;

// Only validate zip code if country is US
let condition = |data: Map<string, string>| {
    data.get("country").unwrap_or("") == "US"
};

let zip_validator = FieldValidator::new("zip_code")
    .required()
    .exact_length(5);

let conditional = ConditionalValidator::new(condition, zip_validator);

let mut data = Map::new();
data.insert("country", "US");
data.insert("zip_code", "12345");

let result = conditional.validate(data, "zip_code");
```

### Async Validation

```jounce
use jounce_validation::AsyncValidator;

// Validate email is not already taken (server-side check)
let check_email = |email: string| {
    // In real app, query database
    return email != "taken@example.com";
};

let email_validator = AsyncValidator::new(
    "email",
    check_email,
    "Email is already taken"
);

let result = email_validator.validate("new@example.com");
```

### Handling Validation Errors

```jounce
use jounce_validation::FormValidator;

// ... set up form_validator ...

let result = form_validator.validate(data);

// Check if valid
if !result.is_valid() {
    // Get all errors
    for error in result.errors {
        println(error.field + ": " + error.message);
    }

    // Get errors for specific field
    let email_errors = result.get_errors_for_field("email");
    for error in email_errors {
        println("Email error: " + error);
    }

    // Check if field has error
    if result.has_error_for_field("password") {
        println("Password is invalid");
    }
}
```

### Validate Single Field

```jounce
// Validate a single field from a form
let result = form_validator.validate_field("email", "user@example.com");

if result.is_valid() {
    println("Email is valid!");
}
```

## API Reference

### ValidationResult

```jounce
ValidationResult::new() -> ValidationResult
result.add_error(field: string, message: string) -> ValidationResult
result.is_valid() -> bool
result.get_errors() -> Array<ValidationError>
result.get_errors_for_field(field: string) -> Array<string>
result.has_error_for_field(field: string) -> bool
```

### ValidationError

```jounce
struct ValidationError {
    field: string,
    message: string,
}
```

### ValidationRule

```jounce
ValidationRule::new(name: string, validator: fn, message: string) -> ValidationRule
rule.validate(value: string) -> bool
```

### FieldValidator

```jounce
FieldValidator::new(field_name: string) -> FieldValidator
validator.with_optional() -> FieldValidator
validator.required() -> FieldValidator
validator.min_length(min: int) -> FieldValidator
validator.max_length(max: int) -> FieldValidator
validator.email() -> FieldValidator
validator.url() -> FieldValidator
validator.numeric() -> FieldValidator
validator.alpha() -> FieldValidator
validator.alphanumeric() -> FieldValidator
validator.pattern(regex: string) -> FieldValidator
validator.min_value(min: int) -> FieldValidator
validator.max_value(max: int) -> FieldValidator
validator.validate(value: string) -> ValidationResult
```

### FormValidator

```jounce
FormValidator::new() -> FormValidator
validator.add_field(field: FieldValidator) -> FormValidator
validator.validate(data: Map<string, string>) -> ValidationResult
validator.validate_field(field: string, value: string) -> ValidationResult
```

### Validators

```jounce
Validators::required() -> ValidationRule
Validators::email() -> ValidationRule
Validators::url() -> ValidationRule
Validators::phone() -> ValidationRule
Validators::postal_code() -> ValidationRule
Validators::credit_card() -> ValidationRule
Validators::ip_address() -> ValidationRule
Validators::min_length(min: int) -> ValidationRule
Validators::max_length(max: int) -> ValidationRule
Validators::exact_length(length: int) -> ValidationRule
Validators::matches(field: string, value: string) -> ValidationRule
Validators::different_from(field: string, value: string) -> ValidationRule
Validators::in_list(allowed: Array<string>) -> ValidationRule
Validators::not_in_list(forbidden: Array<string>) -> ValidationRule
Validators::custom(name: string, fn, message: string) -> ValidationRule
```

### ValidationSchema

```jounce
ValidationSchema::new() -> ValidationSchema
schema.field(name: string) -> FieldValidator
schema.add_validator(validator: FieldValidator) -> ValidationSchema
schema.build() -> FormValidator
```

### ConditionalValidator

```jounce
ConditionalValidator::new(condition: fn, validator: FieldValidator) -> ConditionalValidator
conditional.validate(data: Map, field: string) -> ValidationResult
```

### AsyncValidator

```jounce
AsyncValidator::new(field: string, fn, message: string) -> AsyncValidator
validator.validate(value: string) -> ValidationResult
```

## Common Validation Patterns

### Registration Form

```jounce
let mut form = FormValidator::new();

form = form
    .add_field(FieldValidator::new("username")
        .required()
        .alphanumeric()
        .min_length(3)
        .max_length(20))
    .add_field(FieldValidator::new("email")
        .required()
        .email())
    .add_field(FieldValidator::new("password")
        .required()
        .min_length(8))
    .add_field(FieldValidator::new("confirm_password")
        .required()
        .add_rule(Validators::matches("password", password_value)));
```

### Contact Form

```jounce
let mut form = FormValidator::new();

form = form
    .add_field(FieldValidator::new("name").required())
    .add_field(FieldValidator::new("email").required().email())
    .add_field(FieldValidator::new("phone").with_optional())
    .add_field(FieldValidator::new("message").required().min_length(10));
```

### Payment Form

```jounce
let mut form = FormValidator::new();

form = form
    .add_field(FieldValidator::new("card_number")
        .required()
        .add_rule(Validators::credit_card()))
    .add_field(FieldValidator::new("cvv")
        .required()
        .numeric()
        .exact_length(3))
    .add_field(FieldValidator::new("zip_code")
        .required()
        .add_rule(Validators::postal_code()));
```

## Best Practices

1. **Use Built-in Validators** - Leverage the 15+ built-in validators before creating custom ones
2. **Chain Rules** - Combine multiple validation rules for comprehensive validation
3. **Clear Messages** - Provide user-friendly error messages
4. **Optional Fields** - Use `.with_optional()` for non-required fields
5. **Validate Early** - Validate fields as users type for better UX
6. **Server-Side Too** - Always validate on the server, not just client-side
7. **Custom Rules** - Use `Validators::custom()` for app-specific validation logic

## Examples

See the `tests/` directory for comprehensive examples of all validation patterns.

## License

MIT

## Version

0.1.0
