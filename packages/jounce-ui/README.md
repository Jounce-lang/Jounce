# jounce-ui

Reusable UI component library for Jounce applications.

## Features

- ✅ **Button** - Multiple variants, sizes, loading states
- ✅ **Input** - Text, password, email, number inputs
- ✅ **Textarea** - Multi-line text input
- ✅ **Modal** - Dialog component with overlay
- ✅ **Toast** - Notification messages
- ✅ **Alert** - Inline alerts with variants
- ✅ **Card** - Content card with header/footer
- ✅ **Badge** - Status indicators
- ✅ **Dropdown** - Select component

## Installation

```bash
jnc pkg add jounce-ui
```

## Quick Start

```jounce
use jounce_ui::{Button, ButtonProps, ButtonVariant, ButtonSize};

fn main() {
    let button = Button(ButtonProps {
        text: "Click Me",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Medium,
        disabled: false,
        loading: false,
        on_click: || { println!("Clicked!"); },
        class_name: "",
        aria_label: "",
    });

    println!("{}", button);
}
```

## Components

### Button

```jounce
use jounce_ui::{Button, ButtonProps, ButtonVariant, ButtonSize};

// Primary button
let primary = Button(ButtonProps {
    text: "Primary",
    variant: ButtonVariant::Primary,
    size: ButtonSize::Medium,
    disabled: false,
    loading: false,
    on_click: || {},
    class_name: "",
    aria_label: "Primary action",
});

// Loading button
let loading = Button(ButtonProps {
    text: "Loading...",
    variant: ButtonVariant::Secondary,
    size: ButtonSize::Large,
    disabled: false,
    loading: true,
    on_click: || {},
    class_name: "",
    aria_label: "",
});

// Disabled button
let disabled = Button(ButtonProps {
    text: "Disabled",
    variant: ButtonVariant::Danger,
    size: ButtonSize::Small,
    disabled: true,
    loading: false,
    on_click: || {},
    class_name: "",
    aria_label: "",
});
```

**Variants**: Primary, Secondary, Danger, Success, Ghost
**Sizes**: Small, Medium, Large

### Input

```jounce
use jounce_ui::{Input, InputProps, InputType};

let email_input = Input(InputProps {
    input_type: InputType::Email,
    value: "",
    placeholder: "Enter your email",
    disabled: false,
    required: true,
    min_length: 0,
    max_length: 100,
    on_change: |value| { println!("Changed: {}", value); },
    on_blur: || {},
    class_name: "",
    aria_label: "Email address",
});
```

**Types**: Text, Password, Email, Number, Tel, Url

### Textarea

```jounce
use jounce_ui::{Textarea, TextareaProps};

let textarea = Textarea(TextareaProps {
    value: "",
    placeholder: "Enter text...",
    rows: 5,
    cols: 40,
    disabled: false,
    required: false,
    max_length: 500,
    on_change: |value| {},
    class_name: "",
    aria_label: "Message",
});
```

### Modal

```jounce
use jounce_ui::{Modal, ModalProps};

let modal = Modal(ModalProps {
    title: "Confirm Action",
    content: "Are you sure you want to proceed?",
    is_open: true,
    on_close: || { println!("Modal closed"); },
    show_close_button: true,
    class_name: "",
});
```

### Toast

```jounce
use jounce_ui::{Toast, ToastProps, ToastType};

let success_toast = Toast(ToastProps {
    message: "Successfully saved!",
    toast_type: ToastType::Success,
    duration: 3000,
    on_dismiss: || {},
    class_name: "",
});

let error_toast = Toast(ToastProps {
    message: "An error occurred",
    toast_type: ToastType::Error,
    duration: 5000,
    on_dismiss: || {},
    class_name: "",
});
```

**Types**: Info, Success, Warning, Error

### Alert

```jounce
use jounce_ui::{Alert, AlertProps, ToastType};

let warning_alert = Alert(AlertProps {
    title: "Warning",
    message: "Your session will expire soon",
    alert_type: ToastType::Warning,
    dismissible: true,
    on_dismiss: || {},
    class_name: "",
});
```

### Card

```jounce
use jounce_ui::{Card, CardProps};

let card = Card(CardProps {
    title: "Product Name",
    content: "This is a great product description.",
    footer: "<button>Buy Now</button>",
    image_url: "/images/product.jpg",
    class_name: "",
});
```

### Badge

```jounce
use jounce_ui::{Badge, BadgeProps, ButtonVariant};

let badge = Badge(BadgeProps {
    text: "New",
    variant: ButtonVariant::Success,
    class_name: "",
});
```

### Dropdown

```jounce
use jounce_ui::{Dropdown, DropdownProps, DropdownOption};

let dropdown = Dropdown(DropdownProps {
    options: vec![
        DropdownOption { value: "1", label: "Option 1", disabled: false },
        DropdownOption { value: "2", label: "Option 2", disabled: false },
        DropdownOption { value: "3", label: "Option 3", disabled: true },
    ],
    selected_value: "1",
    placeholder: "Select an option",
    disabled: false,
    on_change: |value| { println!("Selected: {}", value); },
    class_name: "",
    aria_label: "Options",
});
```

## Styling

All components use CSS classes with the `jnc-` prefix. Integrate with jounce-theme for consistent styling:

```jounce
// In your styles
style Button {
    background: theme.Primary.primary;
    color: white;
    padding: var(--spacing-md);
    border-radius: 4px;
}

style ButtonPrimary {
    background: theme.Primary.primary;
}

style ButtonDanger {
    background: theme.Primary.error;
}
```

## Accessibility

All components include proper ARIA attributes:
- `aria-label` for screen readers
- `role` attributes for semantic meaning
- `disabled` states
- Keyboard navigation support

## API Reference

### Button
- `ButtonProps` - text, variant, size, disabled, loading, on_click, class_name, aria_label
- `ButtonVariant` - Primary, Secondary, Danger, Success, Ghost
- `ButtonSize` - Small, Medium, Large

### Input
- `InputProps` - input_type, value, placeholder, disabled, required, min_length, max_length, on_change, on_blur, class_name, aria_label
- `InputType` - Text, Password, Email, Number, Tel, Url

### Textarea
- `TextareaProps` - value, placeholder, rows, cols, disabled, required, max_length, on_change, class_name, aria_label

### Modal
- `ModalProps` - title, content, is_open, on_close, show_close_button, class_name

### Toast
- `ToastProps` - message, toast_type, duration, on_dismiss, class_name
- `ToastType` - Info, Success, Warning, Error

### Alert
- `AlertProps` - title, message, alert_type, dismissible, on_dismiss, class_name

### Card
- `CardProps` - title, content, footer, image_url, class_name

### Badge
- `BadgeProps` - text, variant, class_name

### Dropdown
- `DropdownProps` - options, selected_value, placeholder, disabled, on_change, class_name, aria_label
- `DropdownOption` - value, label, disabled

## Examples

See `examples/` directory:
- `button-showcase.jnc` - All button variants
- `form-components.jnc` - Input and textarea
- `modal-dialog.jnc` - Modal usage
- `notifications.jnc` - Toast and alert

## Best Practices

1. **Accessibility**: Always provide aria-label for interactive elements
2. **Variants**: Use semantic variants (Primary for main action, Danger for destructive)
3. **Loading States**: Show loading indicators for async operations
4. **Validation**: Use required/min_length/max_length for form validation
5. **Custom Styling**: Add class_name for custom styles without modifying components

## License

MIT

## Version

0.1.0
