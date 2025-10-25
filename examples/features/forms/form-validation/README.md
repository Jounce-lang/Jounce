# Form Validation - Jounce Reactivity Example

A form validation system demonstrating reactive validation, computed error messages, cross-field validation, and real-time feedback with Jounce's reactivity system.

## Features Demonstrated

- **Reactive Validation**: Validation updates as user types
- **Computed Error Messages**: Error messages derived from validation state
- **Cross-Field Validation**: Validate relationships between fields
- **Overall Form State**: Computed from all field validations
- **Real-Time Feedback**: Instant validation without manual checks

## Code Overview

```jounce
// Form fields as signals
let email = signal("");
let password = signal("");
let confirmPassword = signal("");

// Computed validation
let emailValid = computed(() => {
    email.value.contains("@") && email.value.contains(".")
});

let passwordValid = computed(() => {
    password.value.length >= 8
});

// Cross-field validation
let passwordsMatch = computed(() => {
    password.value == confirmPassword.value
});

// Overall form validity
let formValid = computed(() => {
    emailValid.value && passwordValid.value && passwordsMatch.value
});

// Computed error messages
let emailError = computed(() => {
    if !emailValid.value {
        "Please enter a valid email address"
    } else {
        ""
    }
});

// Effect: Show/hide error messages
effect(() => {
    if emailValid.value {
        hideError("email");
    } else {
        showError("email", emailError.value);
    }
});

// Effect: Enable/disable submit button
effect(() => {
    submitButton.disabled = !formValid.value;
});
```

## Key Patterns

### Field Validation
```jounce
let field = signal("");
let fieldValid = computed(() => {
    // Validation logic
    field.value.length > 0
});
let fieldError = computed(() => {
    if fieldValid.value { "" } else { "Error message" }
});
```

### Cross-Field Validation
```jounce
let password = signal("");
let confirm = signal("");
let match = computed(() => password.value == confirm.value);
```

### Aggregate Validation
```jounce
let formValid = computed(() => {
    field1Valid.value && field2Valid.value && field3Valid.value
});
```

### Conditional Error Display
```jounce
effect(() => {
    if !fieldValid.value && field.value.length > 0 {
        showError(fieldError.value);
    } else {
        hideError();
    }
});
```

## Validation Strategies

### Immediate Validation
Validate as user types:
```jounce
effect(() => {
    // Runs on every keystroke
    updateError(fieldError.value);
});
```

### Lazy Validation
Only validate after user leaves field:
```jounce
let touched = signal(false);
effect(() => {
    if touched.value && !fieldValid.value {
        showError(fieldError.value);
    }
});
```

### Async Validation
Check availability (username, email):
```jounce
effect(() => {
    if email.value.length > 0 {
        checkEmailAvailability(email.value);
    }
});
```

## Benefits

- **Instant Feedback**: Users see validation as they type
- **No Manual Checks**: Validation happens automatically
- **Composable**: Combine multiple validation rules
- **Declarative**: Describe what's valid, not how to check
- **Efficient**: Only revalidates affected fields

## Common Validation Rules

```jounce
// Email format
let emailValid = computed(() => {
    email.value.matches(/^[^\s@]+@[^\s@]+\.[^\s@]+$/)
});

// Password strength
let strongPassword = computed(() => {
    let pwd = password.value;
    pwd.length >= 8 &&
    pwd.contains_digit() &&
    pwd.contains_special_char()
});

// Age range
let ageValid = computed(() => {
    let n = age.value.parse_int();
    n >= 18 && n <= 120
});

// URL format
let urlValid = computed(() => {
    url.value.starts_with("http://") ||
    url.value.starts_with("https://")
});
```

## Learn More

- [Jounce Reactivity System](../../docs/design/REACTIVITY_SYSTEM.md)
- [Counter Example](../counter-app/)
- [Todo App Example](../todo-app-reactive/)
