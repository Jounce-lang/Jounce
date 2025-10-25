# jounce-rpc

RPC middleware, interceptors, error handling, and retry logic for Jounce applications.

## Features

- ✅ **RPC Client** - Type-safe remote procedure calls
- ✅ **Error Handling** - Standard RPC error codes
- ✅ **Middleware** - Retry, timeout, rate limiting
- ✅ **Interceptors** - Request and response interceptors
- ✅ **Batch Requests** - Send multiple requests in one call
- ✅ **Global Client** - Singleton RPC client
- ✅ **Fluent API** - Chainable builder pattern

## Installation

```bash
jnc pkg add jounce-rpc
```

## Quick Start

```jounce
use jounce_rpc::{RpcClient, RpcClientConfig, RpcRequest};

fn main() {
    // Create client
    let config = RpcClientConfig::new("https://api.example.com/rpc");
    let client = RpcClient::new(config);

    // Make RPC call
    let request = RpcRequest::new("getUser")
        .with_param("id", "123");

    let response = client.call(request);

    match response {
        Ok(resp) => println("Success!"),
        Err(error) => println("Error: {}", error.to_string()),
    }
}
```

## Usage

### Basic RPC Call

```jounce
use jounce_rpc::{RpcClient, RpcClientConfig, RpcRequest};

let config = RpcClientConfig::new("https://api.example.com/rpc");
let client = RpcClient::new(config);

let request = RpcRequest::new("getUser")
    .with_param("id", "123");

let response = client.call(request);
```

### Request with Headers and Metadata

```jounce
let request = RpcRequest::new("createUser")
    .with_param("name", "John Doe")
    .with_param("email", "john@example.com")
    .with_header("Content-Type", "application/json")
    .with_header("Authorization", "Bearer token123")
    .with_metadata("trace_id", "abc-123");
```

### Error Handling

```jounce
use jounce_rpc::{RpcError, RpcErrorCode};

let error = RpcError::new(RpcErrorCode::MethodNotFound, "Method 'getUser' not found")
    .with_data("method", "getUser")
    .with_data("available", "listUsers,createUser");

println(error.to_string());
// "[−32601] Method 'getUser' not found"
```

### Client Configuration

```jounce
use jounce_rpc::RpcClientConfig;

let config = RpcClientConfig::new("https://api.example.com/rpc")
    .with_timeout(10000)           // 10 second timeout
    .with_retries(3)                // Retry up to 3 times
    .with_header("X-API-Key", "secret")
    .with_header("Content-Type", "application/json");

let client = RpcClient::new(config);
```

### Middleware

```jounce
use jounce_rpc::{RetryMiddleware, TimeoutMiddleware, RateLimitMiddleware};

// Retry middleware
let retry = RetryMiddleware::new(3)
    .with_delay(1000);  // 1 second delay

// Timeout middleware
let timeout = TimeoutMiddleware::new(5000);  // 5 second timeout

// Rate limiting middleware
let rate_limit = RateLimitMiddleware::new(100, 60000);  // 100 requests per minute

if rate_limit.is_allowed() {
    // Make RPC call
} else {
    // Rate limit exceeded
}
```

### Batch Requests

```jounce
use jounce_rpc::{BatchRequest, RpcRequest};

let batch = BatchRequest::new()
    .add(RpcRequest::new("getUser").with_param("id", "1"))
    .add(RpcRequest::new("getUser").with_param("id", "2"))
    .add(RpcRequest::new("getUser").with_param("id", "3"));

println("Batch size: {}", batch.size());  // 3
```

### Batch Responses

```jounce
use jounce_rpc::{BatchResponse, RpcResponse};

let batch = BatchResponse::new()
    .add(RpcResponse::success("req1", "user1_data"))
    .add(RpcResponse::success("req2", "user2_data"));

if batch.all_success() {
    println("All requests successful!");
}

if batch.has_errors() {
    println("Some requests failed");
}
```

### Global Client

```jounce
use jounce_rpc::{set_global_client, get_global_client, rpc_call};

// Set up global client once
let config = RpcClientConfig::new("https://api.example.com/rpc");
let client = RpcClient::new(config);
set_global_client(client);

// Use global client anywhere
let params = Map::new();
params.insert("id", "123");

let response = rpc_call("getUser", params);
```

## API Reference

### RpcRequest

```jounce
RpcRequest::new(method: string) -> RpcRequest
request.with_param(key: string, value: string) -> RpcRequest
request.with_header(key: string, value: string) -> RpcRequest
request.with_metadata(key: string, value: string) -> RpcRequest
```

### RpcResponse

```jounce
RpcResponse::success(id: string, result: string) -> RpcResponse
RpcResponse::failure(id: string, error: RpcError) -> RpcResponse
response.is_success() -> bool
response.is_error() -> bool
```

### RpcError

```jounce
RpcError::new(code: RpcErrorCode, message: string) -> RpcError
error.with_data(key: string, value: string) -> RpcError
error.to_string() -> string
```

### RpcErrorCode

- `ParseError` (-32700)
- `InvalidRequest` (-32600)
- `MethodNotFound` (-32601)
- `InvalidParams` (-32602)
- `InternalError` (-32603)
- `ServerError` (-32000)
- `Timeout` (-32001)
- `NetworkError` (-32002)
- `AuthenticationError` (-32003)
- `AuthorizationError` (-32004)

### RpcClientConfig

```jounce
RpcClientConfig::new(endpoint: string) -> RpcClientConfig
config.with_timeout(timeout: int) -> RpcClientConfig
config.with_retries(max_retries: int) -> RpcClientConfig
config.with_header(key: string, value: string) -> RpcClientConfig
```

### RpcClient

```jounce
RpcClient::new(config: RpcClientConfig) -> RpcClient
client.call(request: RpcRequest) -> Result<RpcResponse, RpcError>
client.call_with_retry(request: RpcRequest) -> Result<RpcResponse, RpcError>
client.add_request_interceptor(interceptor: RequestInterceptor)
client.add_response_interceptor(interceptor: ResponseInterceptor)
```

### Middleware

```jounce
RetryMiddleware::new(max_retries: int) -> RetryMiddleware
retry.with_delay(delay: int) -> RetryMiddleware

TimeoutMiddleware::new(timeout: int) -> TimeoutMiddleware

RateLimitMiddleware::new(max_requests: int, window: int) -> RateLimitMiddleware
rate_limit.is_allowed() -> bool
rate_limit.reset()
```

### BatchRequest

```jounce
BatchRequest::new() -> BatchRequest
batch.add(request: RpcRequest) -> BatchRequest
batch.size() -> int
```

### BatchResponse

```jounce
BatchResponse::new() -> BatchResponse
batch.add(response: RpcResponse) -> BatchResponse
batch.all_success() -> bool
batch.has_errors() -> bool
```

## Best Practices

1. **Use Global Client**: Set up once, use everywhere
2. **Handle Errors**: Always check for RPC errors
3. **Add Retries**: Use retry middleware for transient failures
4. **Set Timeouts**: Prevent hanging requests
5. **Rate Limiting**: Protect your API from overload
6. **Add Tracing**: Use metadata for distributed tracing
7. **Batch When Possible**: Reduce network overhead

## Examples

### API Client with Retries

```jounce
let config = RpcClientConfig::new("https://api.example.com/rpc")
    .with_timeout(5000)
    .with_retries(3);

let client = RpcClient::new(config);

let request = RpcRequest::new("getUser")
    .with_param("id", "123");

// Automatically retries up to 3 times
let response = client.call_with_retry(request);
```

### Rate Limited Client

```jounce
let rate_limit = RateLimitMiddleware::new(10, 1000);  // 10 req/sec

for i in 0..100 {
    if rate_limit.is_allowed() {
        let request = RpcRequest::new("ping");
        client.call(request);
    } else {
        println("Rate limit exceeded, waiting...");
        // Wait and retry
    }
}
```

### Error Recovery

```jounce
let response = client.call(request);

match response {
    Ok(resp) => {
        if let Some(result) = resp.result {
            println("Success: {}", result);
        }
    },
    Err(error) => {
        match error.code {
            RpcErrorCode::MethodNotFound => println("Method not found"),
            RpcErrorCode::Timeout => println("Request timed out, retry"),
            RpcErrorCode::NetworkError => println("Network error"),
            _ => println("Unknown error: {}", error.to_string()),
        }
    },
}
```

## License

MIT

## Version

0.1.0
