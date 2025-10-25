# jounce-logger

Structured logging with log levels, JSON output, and file rotation for Jounce applications.

## Features

- ✅ **Log Levels** - DEBUG, INFO, WARN, ERROR, FATAL
- ✅ **Structured Logging** - Add context fields to logs
- ✅ **JSON Output** - Machine-readable JSON format
- ✅ **Text Output** - Human-readable text format
- ✅ **File Rotation** - Automatic log file rotation by size
- ✅ **Multiple Loggers** - Namespaced loggers
- ✅ **Filtering** - Filter logs by minimum level
- ✅ **Context Fields** - Add persistent fields to logger

## Installation

```bash
jnc pkg add jounce-logger
```

## Quick Start

```jounce
use jounce_logger::{info, warn, error};

fn main() {
    info("Application started");
    warn("This is a warning");
    error("An error occurred");
}
```

## Usage

### Basic Logging

```jounce
use jounce_logger::{debug, info, warn, error, fatal};

// Log at different levels
debug("Debugging information");
info("Information message");
warn("Warning message");
error("Error message");
fatal("Fatal error - application will exit");
```

### Structured Logging with Fields

```jounce
use jounce_logger::{get_logger};

let logger = get_logger("app");

// Log with additional context
logger.info_with_fields(
    "User logged in",
    Map::from([
        ("user_id", "12345"),
        ("ip", "192.168.1.1"),
        ("method", "oauth")
    ])
);

logger.error_with_fields(
    "Database query failed",
    Map::from([
        ("query", "SELECT * FROM users"),
        ("error", "connection timeout")
    ])
);
```

### Custom Logger Configuration

```jounce
use jounce_logger::{Logger, LoggerConfig, LogLevel, OutputFormat, OutputTarget, register_logger};

let config = LoggerConfig {
    name: "myapp",
    min_level: LogLevel::DEBUG,
    format: OutputFormat::JSON,
    target: OutputTarget::File,
    file_path: "/var/log/myapp.log",
    max_file_size: 10 * 1024 * 1024, // 10MB
    max_files: 5,
};

let logger = Logger::new(config);
register_logger(logger);

// Use the custom logger
let my_logger = get_logger("myapp");
my_logger.info("Custom logger initialized");
```

### Context Fields

```jounce
use jounce_logger::{get_logger};

// Add persistent context fields
let logger = get_logger("api")
    .with_field("service", "user-service")
    .with_field("version", "1.0.0")
    .with_field("environment", "production");

// All logs from this logger will include context fields
logger.info("Request processed"); // Includes service, version, environment
logger.error("Request failed"); // Includes service, version, environment
```

### JSON Output

```jounce
use jounce_logger::{Logger, LoggerConfig, LogLevel, OutputFormat, OutputTarget};

let config = LoggerConfig {
    name: "json-logger",
    min_level: LogLevel::INFO,
    format: OutputFormat::JSON,
    target: OutputTarget::Console,
    file_path: "",
    max_file_size: 0,
    max_files: 0,
};

let logger = Logger::new(config);
logger.info_with_fields(
    "User action",
    Map::from([("action", "login"), ("user_id", "123")])
);

// Output:
// {"timestamp":"2025-10-24 12:00:00","level":"INFO","logger":"json-logger","message":"User action","fields":{"action":"login","user_id":"123"}}
```

### File Logging with Rotation

```jounce
use jounce_logger::{Logger, LoggerConfig, LogLevel, OutputFormat, OutputTarget};

let config = LoggerConfig {
    name: "file-logger",
    min_level: LogLevel::INFO,
    format: OutputFormat::Text,
    target: OutputTarget::File,
    file_path: "/var/log/app.log",
    max_file_size: 10 * 1024 * 1024, // 10MB
    max_files: 5, // Keep 5 rotated files
};

let logger = Logger::new(config);

// Logs to /var/log/app.log
// When file reaches 10MB, rotates to:
// /var/log/app.log.1
// /var/log/app.log.2
// ... up to app.log.5
```

### Log Level Filtering

```jounce
use jounce_logger::{Logger, LoggerConfig, LogLevel, OutputFormat, OutputTarget};

// Only log WARN and above
let config = LoggerConfig {
    name: "prod-logger",
    min_level: LogLevel::WARN,
    format: OutputFormat::JSON,
    target: OutputTarget::Both, // Console + File
    file_path: "/var/log/prod.log",
    max_file_size: 50 * 1024 * 1024,
    max_files: 10,
};

let logger = Logger::new(config);

logger.debug("Debug info"); // Filtered out
logger.info("Info message"); // Filtered out
logger.warn("Warning!"); // Logged
logger.error("Error!"); // Logged
logger.fatal("Fatal!"); // Logged
```

### Multiple Loggers

```jounce
use jounce_logger::{get_logger};

// Different loggers for different components
let db_logger = get_logger("database")
    .with_field("component", "db");

let api_logger = get_logger("api")
    .with_field("component", "api");

let auth_logger = get_logger("auth")
    .with_field("component", "auth");

db_logger.info("Connection established");
api_logger.info("Request received");
auth_logger.warn("Failed login attempt");
```

## Output Formats

### Text Format

```
[2025-10-24 12:00:00] INFO [myapp] User logged in {user_id=12345, ip=192.168.1.1}
[2025-10-24 12:00:05] ERROR [myapp] Database error {query=SELECT *, error=timeout}
```

### JSON Format

```json
{"timestamp":"2025-10-24 12:00:00","level":"INFO","logger":"myapp","message":"User logged in","fields":{"user_id":"12345","ip":"192.168.1.1"}}
{"timestamp":"2025-10-24 12:00:05","level":"ERROR","logger":"myapp","message":"Database error","fields":{"query":"SELECT *","error":"timeout"}}
```

## API Reference

### Log Levels

- `LogLevel::DEBUG` - Detailed debugging information
- `LogLevel::INFO` - General informational messages
- `LogLevel::WARN` - Warning messages
- `LogLevel::ERROR` - Error messages
- `LogLevel::FATAL` - Fatal errors

### Output Formats

- `OutputFormat::JSON` - Machine-readable JSON
- `OutputFormat::Text` - Human-readable text

### Output Targets

- `OutputTarget::Console` - Log to console/stdout
- `OutputTarget::File` - Log to file
- `OutputTarget::Both` - Log to both console and file

### Logger Functions

- `Logger::new(config: LoggerConfig) -> Logger` - Create logger
- `Logger.with_field(key: string, value: string) -> Logger` - Add context field
- `Logger.debug(message: string)` - Log DEBUG
- `Logger.info(message: string)` - Log INFO
- `Logger.warn(message: string)` - Log WARN
- `Logger.error(message: string)` - Log ERROR
- `Logger.fatal(message: string)` - Log FATAL
- `Logger.debug_with_fields(message: string, fields: Map<string, string>)` - Log DEBUG with fields
- `Logger.info_with_fields(message: string, fields: Map<string, string>)` - Log INFO with fields
- `Logger.warn_with_fields(message: string, fields: Map<string, string>)` - Log WARN with fields
- `Logger.error_with_fields(message: string, fields: Map<string, string>)` - Log ERROR with fields
- `Logger.fatal_with_fields(message: string, fields: Map<string, string>)` - Log FATAL with fields

### Global Functions

- `get_logger(name: string) -> Logger` - Get or create named logger
- `register_logger(logger: Logger)` - Register custom logger
- `default_logger() -> Logger` - Get default logger
- `debug(message: string)` - Log DEBUG to default logger
- `info(message: string)` - Log INFO to default logger
- `warn(message: string)` - Log WARN to default logger
- `error(message: string)` - Log ERROR to default logger
- `fatal(message: string)` - Log FATAL to default logger

## File Rotation

When a log file reaches `max_file_size`, it is rotated:

```
app.log       (current log file)
app.log.1     (most recent rotation)
app.log.2     (older)
app.log.3     (older)
app.log.4     (older)
app.log.5     (oldest, will be deleted on next rotation)
```

## Best Practices

1. **Use Appropriate Levels**: DEBUG for development, INFO for production
2. **Add Context**: Use fields to add relevant context to logs
3. **Namespace Loggers**: Use separate loggers for different components
4. **Rotate Files**: Configure rotation to prevent disk space issues
5. **Structured Data**: Use JSON format for log aggregation systems
6. **Security**: Don't log sensitive data (passwords, tokens, etc.)

## Examples

See `examples/` directory:
- `basic-logging.jnc` - Basic logging examples
- `structured-logging.jnc` - Structured logging with fields
- `file-rotation.jnc` - File logging with rotation
- `multiple-loggers.jnc` - Using multiple named loggers

## License

MIT

## Version

0.1.0
