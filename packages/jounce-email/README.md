

# jounce-email

Email sending and template management with SMTP support, multiple providers, and fluent API.

## Features

- ✅ **Email Sending** - Complete email composition and sending
- ✅ **SMTP Support** - Full SMTP client with TLS
- ✅ **Email Templates** - Template system with variable substitution
- ✅ **Provider Presets** - Gmail, SendGrid, Mailgun configs
- ✅ **Attachments** - File attachment support
- ✅ **Batch Sending** - Send to multiple recipients
- ✅ **Email Builder** - Fluent API for building emails
- ✅ **Template Manager** - Centralized template management
- ✅ **Validation** - Email address and message validation

## Installation

```bash
jnc pkg add jounce-email
```

## Quick Start

```jounce
use jounce_email::{EmailClient, SmtpConfig, EmailBuilder};

// Configure SMTP (Gmail example)
let config = SmtpConfig::gmail("your-email@gmail.com", "app-password");
let mut client = EmailClient::new(config);

// Build and send email
let email = EmailBuilder::new()
    .from("your-email@gmail.com")
    .to("recipient@example.com")
    .subject("Hello from Jounce!")
    .text_body("This is a test email.")
    .build();

client = client.send(email).unwrap();
```

## Usage

### SMTP Configuration

```jounce
use jounce_email::SmtpConfig;

// Generic SMTP
let config = SmtpConfig::new("smtp.example.com", 587)
    .with_credentials("username", "password")
    .with_tls(true)
    .with_timeout(30000);

// Gmail preset
let gmail = SmtpConfig::gmail("user@gmail.com", "app-password");

// SendGrid preset
let sendgrid = SmtpConfig::sendgrid("api-key");

// Mailgun preset
let mailgun = SmtpConfig::mailgun("username", "password");
```

### Email Builder

```jounce
use jounce_email::EmailBuilder;

let email = EmailBuilder::new()
    .from("from@example.com")
    .to("to@example.com")
    .cc("cc@example.com")
    .bcc("bcc@example.com")
    .subject("Hello!")
    .text_body("Plain text body")
    .html_body("<p>HTML body</p>")
    .attach_file("document.pdf", "file-content")
    .reply_to("reply@example.com")
    .build();
```

### Email with Name

```jounce
let email = EmailBuilder::new()
    .from_with_name("sender@example.com", "John Doe")
    .to_with_name("recipient@example.com", "Jane Smith")
    .subject("Personal Email")
    .text_body("Hello Jane!")
    .build();
```

### Sending Emails

```jounce
use jounce_email::{EmailClient, SmtpConfig};

let config = SmtpConfig::gmail("user@gmail.com", "password");
let mut client = EmailClient::new(config);

// Send single email
let result = client.send(email);
match result {
    Ok(updated_client) => {
        println("Email sent successfully!");
        client = updated_client;
    }
    Err(error) => {
        println("Failed to send: " + error);
    }
}

// Send batch
let emails = [email1, email2, email3];
client = client.send_batch(emails);

// Check statistics
println("Sent: " + client.sent_count.to_string());
println("Failed: " + client.failed_count.to_string());
println("Success rate: " + client.get_success_rate().to_string());
```

### Email Templates

```jounce
use jounce_email::EmailTemplate;

// Create template
let template = EmailTemplate::new("welcome")
    .with_subject("Welcome to {{app_name}}!")
    .with_body("Hello {{user_name}}, welcome to {{app_name}}! Your account is {{status}}.")
    .as_html();

// Render with variables
let mut vars = Map::new();
vars.insert("app_name", "MyApp");
vars.insert("user_name", "John");
vars.insert("status", "active");

let rendered = template.render(vars);

println(rendered.subject); // "Welcome to MyApp!"
println(rendered.body);    // "Hello John, welcome to MyApp! Your account is active."
```

### HTML Templates

```jounce
let template = EmailTemplate::new("newsletter")
    .with_subject("{{month}} Newsletter")
    .with_body("
        <html>
        <body>
            <h1>{{month}} Newsletter</h1>
            <p>Hello {{name}},</p>
            <p>{{content}}</p>
        </body>
        </html>
    ")
    .as_html();

let mut vars = Map::new();
vars.insert("month", "January");
vars.insert("name", "John");
vars.insert("content", "Here's what's new this month...");

let rendered = template.render(vars);
// rendered.is_html == true
```

### Template Manager

```jounce
use jounce_email::TemplateManager;

// Create manager
let mut manager = TemplateManager::new();

// Add templates
manager = manager
    .add_template(EmailTemplate::new("welcome")
        .with_subject("Welcome!")
        .with_body("Hello {{name}}!"))
    .add_template(EmailTemplate::new("reset")
        .with_subject("Password Reset")
        .with_body("Reset link: {{link}}"));

// Check if template exists
if manager.has_template("welcome") {
    println("Welcome template exists");
}

// Render template
let mut vars = Map::new();
vars.insert("name", "John");

let rendered = manager.render_template("welcome", vars);
if rendered.is_some() {
    println(rendered.unwrap().body);
}
```

### Email Builder with Template

```jounce
let template = EmailTemplate::new("welcome")
    .with_subject("Welcome {{name}}!")
    .with_body("<h1>Hello {{name}}!</h1>")
    .as_html();

let mut vars = Map::new();
vars.insert("name", "John");

let email = EmailBuilder::new()
    .from("noreply@example.com")
    .to("user@example.com")
    .from_template(template, vars)
    .build();

// Email now has subject "Welcome John!" and HTML body
```

### Attachments

```jounce
use jounce_email::Attachment;

// Create attachment
let attachment = Attachment::new("report.pdf", "file-content-here")
    .with_content_type("application/pdf");

// Add to email
let email = EmailBuilder::new()
    .from("sender@example.com")
    .to("recipient@example.com")
    .subject("Monthly Report")
    .text_body("Please see attached report")
    .attach_file("report.pdf", "file-content")
    .build();
```

### Email Validation

```jounce
use jounce_email::{EmailAddress, validate_email};

// Validate address
let addr = EmailAddress::new("test@example.com");
if addr.is_valid() {
    println("Valid email");
}

// Helper function
if validate_email("user@domain.com") {
    println("Valid");
}

// Validate entire email
let email = EmailBuilder::new()
    .from("from@example.com")
    .to("to@example.com")
    .subject("Test")
    .text_body("Body")
    .build();

if email.validate() {
    println("Email is valid and ready to send");
}
```

### Extract Domain

```jounce
use jounce_email::extract_domain;

let domain = extract_domain("user@example.com");
println(domain); // "example.com"
```

### Multiple Recipients

```jounce
let email = EmailBuilder::new()
    .from("sender@example.com")
    .to("user1@example.com")
    .to("user2@example.com")
    .to("user3@example.com")
    .cc("manager@example.com")
    .subject("Team Update")
    .text_body("Important update for the team...")
    .build();
```

### Custom Headers

```jounce
let email = Email::new()
    .from(EmailAddress::new("from@example.com"))
    .to(EmailAddress::new("to@example.com"))
    .subject("Test")
    .text_body("Body")
    .header("X-Priority", "1")
    .header("X-Custom-Header", "value");
```

### Complete Example

```jounce
use jounce_email::{EmailClient, SmtpConfig, EmailBuilder, EmailTemplate, TemplateManager};

// 1. Setup SMTP
let config = SmtpConfig::gmail("myapp@gmail.com", "app-password");
let mut client = EmailClient::new(config)
    .with_default_from(EmailAddress::new("myapp@gmail.com"));

// 2. Create templates
let mut templates = TemplateManager::new();
templates = templates.add_template(
    EmailTemplate::new("welcome")
        .with_subject("Welcome to {{app}}!")
        .with_body("<h1>Hello {{name}}!</h1><p>Thanks for joining {{app}}.</p>")
        .as_html()
);

// 3. Send welcome email
let mut vars = Map::new();
vars.insert("name", "John");
vars.insert("app", "MyApp");

let email = EmailBuilder::new()
    .from("myapp@gmail.com")
    .to_with_name("john@example.com", "John Doe")
    .from_template(templates.get_template("welcome").unwrap(), vars)
    .build();

let result = client.send(email);
match result {
    Ok(updated) => {
        println("Welcome email sent!");
        client = updated;
    }
    Err(e) => println("Error: " + e),
}

// 4. Check stats
println("Sent: " + client.sent_count.to_string());
println("Success rate: " + (client.get_success_rate() * 100.0).to_string() + "%");
```

## API Reference

### EmailAddress

```jounce
EmailAddress::new(email: string) -> EmailAddress
address.with_name(name: string) -> EmailAddress
address.to_string() -> string
address.is_valid() -> bool
```

### Attachment

```jounce
Attachment::new(filename: string, content: string) -> Attachment
attachment.with_content_type(content_type: string) -> Attachment
```

### Email

```jounce
Email::new() -> Email
email.from(email: EmailAddress) -> Email
email.to(email: EmailAddress) -> Email
email.cc(email: EmailAddress) -> Email
email.bcc(email: EmailAddress) -> Email
email.subject(subject: string) -> Email
email.text_body(body: string) -> Email
email.html_body(body: string) -> Email
email.attach(attachment: Attachment) -> Email
email.reply_to(email: EmailAddress) -> Email
email.header(key: string, value: string) -> Email
email.validate() -> bool
```

### SmtpConfig

```jounce
SmtpConfig::new(host: string, port: int) -> SmtpConfig
config.with_credentials(username: string, password: string) -> SmtpConfig
config.with_tls(use_tls: bool) -> SmtpConfig
config.with_timeout(timeout: int) -> SmtpConfig
SmtpConfig::gmail(username: string, password: string) -> SmtpConfig
SmtpConfig::sendgrid(api_key: string) -> SmtpConfig
SmtpConfig::mailgun(username: string, password: string) -> SmtpConfig
```

### EmailClient

```jounce
EmailClient::new(config: SmtpConfig) -> EmailClient
client.with_default_from(from: EmailAddress) -> EmailClient
client.send(email: Email) -> Result<EmailClient, string>
client.send_batch(emails: Array<Email>) -> EmailClient
client.get_success_rate() -> float
```

### EmailTemplate

```jounce
EmailTemplate::new(name: string) -> EmailTemplate
template.with_subject(template: string) -> EmailTemplate
template.with_body(template: string) -> EmailTemplate
template.as_html() -> EmailTemplate
template.render(variables: Map<string, string>) -> RenderedEmail
```

### TemplateManager

```jounce
TemplateManager::new() -> TemplateManager
manager.add_template(template: EmailTemplate) -> TemplateManager
manager.get_template(name: string) -> Option<EmailTemplate>
manager.has_template(name: string) -> bool
manager.render_template(name: string, variables: Map<string, string>) -> Option<RenderedEmail>
```

### EmailBuilder

```jounce
EmailBuilder::new() -> EmailBuilder
builder.from(email: string) -> EmailBuilder
builder.from_with_name(email: string, name: string) -> EmailBuilder
builder.to(email: string) -> EmailBuilder
builder.to_with_name(email: string, name: string) -> EmailBuilder
builder.cc(email: string) -> EmailBuilder
builder.bcc(email: string) -> EmailBuilder
builder.subject(subject: string) -> EmailBuilder
builder.text_body(body: string) -> EmailBuilder
builder.html_body(body: string) -> EmailBuilder
builder.attach_file(filename: string, content: string) -> EmailBuilder
builder.reply_to(email: string) -> EmailBuilder
builder.from_template(template: EmailTemplate, vars: Map<string, string>) -> EmailBuilder
builder.build() -> Email
```

### Utility Functions

```jounce
validate_email(email: string) -> bool
extract_domain(email: string) -> string
```

## Best Practices

1. **Use App Passwords** - For Gmail, use app-specific passwords, not your main password
2. **Validate First** - Always validate emails before sending
3. **Use Templates** - Centralize email content in templates
4. **Batch Wisely** - Batch sends for efficiency, but respect rate limits
5. **Check Stats** - Monitor sent/failed counts and success rates
6. **HTML + Text** - Provide both HTML and text bodies for compatibility
7. **Sanitize Input** - Sanitize user input in template variables
8. **Rate Limiting** - Implement rate limiting for production use

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
