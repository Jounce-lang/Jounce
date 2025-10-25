# jounce-config

Configuration management with environment variables, typed config loading, and validation for Jounce applications.

## Features

- ✅ **Environment Variables** - Load from .env files and process.env
- ✅ **Typed Config** - Strong typing for config values (string, int, bool, float)
- ✅ **Default Values** - Set defaults for missing config
- ✅ **Config Validation** - Schema-based validation with custom rules
- ✅ **Multiple Environments** - Dev, prod, test, staging configs
- ✅ **Secret Management** - Secure handling of sensitive config
- ✅ **Fluent API** - Chainable config builder
- ✅ **Global Config** - Singleton config access

## Installation

```bash
jnc pkg add jounce-config
```

## Quick Start

```jounce
use jounce_config::{ConfigBuilder, Environment};

// Build configuration
let config = ConfigBuilder::new()
    .with_env_file(".env")
    .with_env(Environment::Production)
    .with_default_string("host", "localhost")
    .with_default_int("port", 8080)
    .load_from_system_env()
    .build();

// Get values
let host = config.get_string("host");
let port = config.get_int("port").unwrap();
let debug = config.get_bool_or("debug", false);

println("Server: " + host + ":" + port.to_string());
```

## Usage

### Basic Configuration

```jounce
use jounce_config::Config;

// Create config
let mut config = Config::new();

// Set values
config = config
    .set_string("api_key", "secret123")
    .set_int("timeout", 5000)
    .set_bool("debug", true);

// Get values
let api_key = config.get_string("api_key");
let timeout = config.get_int("timeout").unwrap();
let is_debug = config.get_bool("debug");
```

### Config Builder

```jounce
use jounce_config::{ConfigBuilder, Environment};

let config = ConfigBuilder::new()
    .with_env(Environment::Production)
    .with_env_file(".env")
    .with_env_file(".env.local")
    .with_default_string("database_host", "localhost")
    .with_default_int("database_port", 5432)
    .with_default_bool("ssl_enabled", false)
    .load_from_system_env()
    .build();
```

### Environment-Specific Config

```jounce
use jounce_config::{EnvConfig, Environment, Config};

let mut env_config = EnvConfig::new();

// Configure development
let mut dev_config = Config::new();
dev_config = dev_config
    .set_string("database_url", "localhost:5432")
    .set_bool("debug", true);

env_config = env_config.set_for_env(Environment::Development, dev_config);

// Configure production
let mut prod_config = Config::new();
prod_config = prod_config
    .set_string("database_url", "prod-db.example.com:5432")
    .set_bool("debug", false);

env_config = env_config.set_for_env(Environment::Production, prod_config);

// Get current environment config
env_config = env_config.with_current_env(Environment::Production);
let current = env_config.get_current();
```

### Default Values

```jounce
use jounce_config::Config;

let config = Config::new();

// Get with defaults
let host = config.get_string_or("host", "localhost");
let port = config.get_int_or("port", 3000);
let ssl = config.get_bool_or("ssl_enabled", false);

println("Server: " + host + ":" + port.to_string());
```

### Config Validation

```jounce
use jounce_config::{ConfigSchema, Config};

// Define schema
let schema = ConfigSchema::new()
    .require("database_url")
    .require("api_key")
    .optional("debug")
    .optional("cache_ttl");

// Validate config
let result = schema.validate(config);

if !result.is_valid() {
    for error in result.get_errors() {
        println("Config error: " + error);
    }
}
```

### Custom Validators

```jounce
use jounce_config::{ConfigSchema, ConfigValue};

let schema = ConfigSchema::new()
    .require("port")
    .with_validator("port", |value| {
        let port = value.as_int().unwrap_or(0);
        return port > 0 && port < 65536;
    });

let result = schema.validate(config);
```

### Secret Management

```jounce
use jounce_config::SecretManager;

let mut secrets = SecretManager::new()
    .with_encryption();

secrets = secrets
    .set_secret("api_key", "super-secret-key")
    .set_secret("db_password", "secure-password");

// Get secret
if let Some(key) = secrets.get_secret("api_key") {
    println("API Key retrieved");
}
```

### Global Config

```jounce
use jounce_config::{set_global_config, config_get_string, config_get_int};

// Set global config once at app startup
let config = ConfigBuilder::new()
    .with_default_string("app_name", "MyApp")
    .with_default_int("version", 1)
    .build();

set_global_config(config);

// Access anywhere in your app
let app_name = config_get_string("app_name");
let version = config_get_int("version").unwrap();
```

### Config Merging

```jounce
use jounce_config::Config;

// Default config
let mut defaults = Config::new();
defaults = defaults
    .set_string("host", "localhost")
    .set_int("port", 8080);

// User config
let mut user_config = Config::new();
user_config = user_config
    .set_int("port", 3000); // Override port

// Merge (user config takes precedence)
let final_config = user_config.merge(defaults);

assert_eq!(final_config.get_int("port").unwrap(), 3000);
assert_eq!(final_config.get_string("host"), "localhost");
```

### Required Config

```jounce
use jounce_config::Config;

let config = Config::new();

// Panic if required key is missing
let api_key = config.require_string("api_key");
let port = config.require_int("port");
let enabled = config.require_bool("enabled");
```

### Environment Detection

```jounce
use jounce_config::Environment;

let env = Environment::from_string("production");

if env.is_production() {
    println("Running in production!");
}

if env.is_development() {
    println("Running in development!");
}

let env_name = env.to_string(); // "production"
```

## API Reference

### Config

```jounce
Config::new() -> Config
config.with_environment(env: Environment) -> Config
config.load_env_file(file: string) -> Config
config.load_from_env() -> Config

config.set_string(key: string, value: string) -> Config
config.set_int(key: string, value: int) -> Config
config.set_bool(key: string, value: bool) -> Config
config.set_float(key: string, value: float) -> Config

config.get_string(key: string) -> string
config.get_int(key: string) -> Option<int>
config.get_bool(key: string) -> bool
config.get_float(key: string) -> Option<float>

config.get_string_or(key: string, default: string) -> string
config.get_int_or(key: string, default: int) -> int
config.get_bool_or(key: string, default: bool) -> bool

config.has(key: string) -> bool
config.require_string(key: string) -> string
config.require_int(key: string) -> int
config.require_bool(key: string) -> bool

config.merge(other: Config) -> Config
config.validate(fn) -> bool
```

### ConfigBuilder

```jounce
ConfigBuilder::new() -> ConfigBuilder
builder.with_env_file(file: string) -> ConfigBuilder
builder.with_env(env: Environment) -> ConfigBuilder
builder.with_default_string(key: string, value: string) -> ConfigBuilder
builder.with_default_int(key: string, value: int) -> ConfigBuilder
builder.with_default_bool(key: string, value: bool) -> ConfigBuilder
builder.load_from_system_env() -> ConfigBuilder
builder.build() -> Config
```

### ConfigSchema

```jounce
ConfigSchema::new() -> ConfigSchema
schema.require(key: string) -> ConfigSchema
schema.optional(key: string) -> ConfigSchema
schema.with_validator(key: string, fn) -> ConfigSchema
schema.validate(config: Config) -> ValidationResult
```

### Environment

```jounce
Environment::Development
Environment::Production
Environment::Test
Environment::Staging

Environment::from_string(s: string) -> Environment
env.to_string() -> string
env.is_production() -> bool
env.is_development() -> bool
```

### SecretManager

```jounce
SecretManager::new() -> SecretManager
manager.with_encryption() -> SecretManager
manager.set_secret(key: string, value: string) -> SecretManager
manager.get_secret(key: string) -> Option<string>
manager.has_secret(key: string) -> bool
manager.load_from_env(prefix: string) -> SecretManager
```

### Global Functions

```jounce
set_global_config(config: Config)
get_global_config() -> Option<Config>
config_get_string(key: string) -> string
config_get_int(key: string) -> Option<int>
config_get_bool(key: string) -> bool
```

## Best Practices

1. **Use ConfigBuilder** - Simplifies configuration setup with fluent API
2. **Validate Early** - Use ConfigSchema to validate required keys at startup
3. **Environment-Specific** - Separate configs for dev/prod/test
4. **Secure Secrets** - Use SecretManager for sensitive data
5. **Default Values** - Always provide sensible defaults
6. **Global Config** - Set once, access everywhere
7. **Fail Fast** - Use `require()` for mandatory config

## Examples

### Web Server Configuration

```jounce
let config = ConfigBuilder::new()
    .with_env_file(".env")
    .with_default_string("host", "0.0.0.0")
    .with_default_int("port", 8080)
    .with_default_bool("ssl_enabled", false)
    .with_default_int("max_connections", 100)
    .build();

let schema = ConfigSchema::new()
    .require("host")
    .require("port")
    .optional("ssl_enabled");

if !schema.validate(config).is_valid() {
    panic("Invalid server configuration");
}
```

### Database Configuration

```jounce
let config = ConfigBuilder::new()
    .with_env_file(".env.database")
    .with_default_string("db_host", "localhost")
    .with_default_int("db_port", 5432)
    .with_default_string("db_name", "myapp")
    .with_default_int("pool_size", 10)
    .build();

let db_url = config.get_string("db_host") + ":" +
             config.get_int("db_port").unwrap().to_string();
```

### API Client Configuration

```jounce
let config = ConfigBuilder::new()
    .with_default_string("api_base_url", "https://api.example.com")
    .with_default_int("timeout_ms", 5000)
    .with_default_int("retry_attempts", 3)
    .with_default_bool("verify_ssl", true)
    .build();

// Secrets separate from regular config
let mut secrets = SecretManager::new();
secrets = secrets.set_secret("api_key", load_from_secure_storage());
```

## License

MIT

## Version

0.1.0
