# jounce-rate-limit

Rate limiting strategies with multiple algorithms and configurations for API protection and resource management.

## Features

- ✅ **Fixed Window** - Simple fixed time window rate limiting
- ✅ **Sliding Window** - Precise sliding time window
- ✅ **Token Bucket** - Burst handling with token refill
- ✅ **Leaky Bucket** - Smooth request rate with leak mechanism
- ✅ **Multiple Users** - Per-user/per-IP rate limiting
- ✅ **Configurable** - Flexible limits and time windows
- ✅ **Rate Limit Headers** - Standard X-RateLimit-* headers
- ✅ **Preset Configs** - Per-second, per-minute, per-hour, per-day

## Installation

```bash
jnc pkg add jounce-rate-limit
```

## Quick Start

```jounce
use jounce_rate_limit::FixedWindowLimiter;

// Create limiter: 100 requests per minute
let mut limiter = FixedWindowLimiter::new(100, 60000);

// Check request
let (updated_limiter, result) = limiter.check("user123", now());
limiter = updated_limiter;

if result.allowed {
    // Process request
    println("Allowed! Remaining: " + result.remaining.to_string());
} else {
    // Reject request
    println("Rate limited! Retry after: " + result.retry_after.to_string() + "ms");
}
```

## Usage

### Fixed Window Rate Limiter

Simple fixed time window algorithm.

```jounce
use jounce_rate_limit::FixedWindowLimiter;

// 10 requests per second
let mut limiter = FixedWindowLimiter::new(10, 1000);

// Check request for user
let (updated, result) = limiter.check("user1", 1000);
limiter = updated;

if result.allowed {
    println("Allowed");
    println("Remaining: " + result.remaining.to_string());
    println("Reset at: " + result.reset_at.to_string());
}
```

### Token Bucket Rate Limiter

Allows bursts while maintaining average rate.

```jounce
use jounce_rate_limit::TokenBucketLimiter;

// Capacity: 100 tokens
// Refill: 10 tokens per second
let mut limiter = TokenBucketLimiter::new(100, 10, 1000);

let (updated, result) = limiter.check("user1", now());
limiter = updated;

// Allows burst of 100 requests, then 10 per second
```

### Sliding Window Rate Limiter

Precise rate limiting without fixed window edge cases.

```jounce
use jounce_rate_limit::SlidingWindowLimiter;

// 50 requests per minute with sliding window
let mut limiter = SlidingWindowLimiter::new(50, 60000);

let (updated, result) = limiter.check("user1", now());
limiter = updated;
```

### Leaky Bucket Rate Limiter

Smooth request rate with queue-like behavior.

```jounce
use jounce_rate_limit::LeakyBucketLimiter;

// Capacity: 20
// Leak: 5 requests per second
let mut limiter = LeakyBucketLimiter::new(20, 5, 1000);

let (updated, result) = limiter.check("user1", now());
limiter = updated;
```

### Rate Limiter Configs

Preset configurations for common use cases.

```jounce
use jounce_rate_limit::RateLimiterConfig;

// Per second
let config = RateLimiterConfig::per_second(60);

// Per minute
let config = RateLimiterConfig::per_minute(1000);

// Per hour
let config = RateLimiterConfig::per_hour(10000);

// Per day
let config = RateLimiterConfig::per_day(100000);

// Custom
let config = RateLimiterConfig::new(100, 5000)
    .with_strategy(RateLimitStrategy::TokenBucket);
```

### Rate Limit Headers

Generate standard rate limit headers for HTTP responses.

```jounce
use jounce_rate_limit::RateLimitHeaders;

let (updated, result) = limiter.check("user1", now());
limiter = updated;

// Create headers from result
let headers = RateLimitHeaders::from_result(result, 100);

// Convert to map for HTTP response
let header_map = headers.to_map();

// header_map contains:
// - X-RateLimit-Limit: "100"
// - X-RateLimit-Remaining: "95"
// - X-RateLimit-Reset: "1234567890"
// - Retry-After: "5000" (if denied)
```

### Multiple Users

Each user/IP has independent rate limits.

```jounce
let mut limiter = FixedWindowLimiter::new(10, 1000);

// User 1
let (updated, result1) = limiter.check("user1", 1000);
limiter = updated;

// User 2
let (updated, result2) = limiter.check("user2", 1000);
limiter = updated;

// Each user has their own 10 requests/second
```

### Reset Rate Limit

Clear rate limit for a specific user.

```jounce
// Reset user1's rate limit
limiter = limiter.reset("user1");

// User1 can now make requests as if they just started
```

### Creating Keys

Organize rate limits by different contexts.

```jounce
use jounce_rate_limit::create_key;

// API endpoint rate limit
let key = create_key("api:users:create", "user123");
// "api:users:create:user123"

// IP-based rate limit
let key = create_key("ip", "192.168.1.1");
// "ip:192.168.1.1"
```

### Handling Rate Limit Results

```jounce
let (updated, result) = limiter.check("user1", now());
limiter = updated;

if result.allowed {
    // Process request
    println("Remaining: " + result.remaining.to_string());
    println("Reset at: " + result.reset_at.to_string());
} else {
    // Rate limited
    println("Rate limited!");
    println("Retry after: " + result.retry_after.to_string() + "ms");
    println("Reset at: " + result.reset_at.to_string());

    // Return 429 Too Many Requests
    // with Retry-After header
}
```

### Complete API Example

```jounce
use jounce_rate_limit::{FixedWindowLimiter, RateLimitHeaders, create_key};

// Create limiter: 1000 requests per hour per user
let mut limiter = FixedWindowLimiter::new(1000, 3600000);

fn handle_request(limiter: FixedWindowLimiter, user_id: string, endpoint: string) -> (FixedWindowLimiter, Response) {
    let key = create_key(endpoint, user_id);
    let (updated, result) = limiter.check(key, now());

    if !result.allowed {
        // Rate limited
        let headers = RateLimitHeaders::from_result(result, 1000);
        return (updated, Response::new(429)
            .with_headers(headers.to_map())
            .with_body("Rate limit exceeded"));
    }

    // Process request
    let headers = RateLimitHeaders::from_result(result, 1000);
    let response = Response::new(200)
        .with_headers(headers.to_map())
        .with_body("Success");

    (updated, response)
}
```

### Token Bucket with Bursts

```jounce
// Allow bursts up to 100 requests
// Refill 10 tokens per second
let mut limiter = TokenBucketLimiter::new(100, 10, 1000);

// User can burst 100 requests immediately
let mut i = 0;
while i < 100 {
    let (updated, result) = limiter.check("user1", 1000 + i);
    limiter = updated;
    assert!(result.allowed, "Burst should be allowed");
    i = i + 1;
}

// 101st request denied
let (updated, result) = limiter.check("user1", 1100);
limiter = updated;
assert!(!result.allowed, "Should be rate limited");

// After 1 second, 10 more requests allowed
let (updated, result) = limiter.check("user1", 2000);
limiter = updated;
assert!(result.allowed, "Should be allowed after refill");
```

### Sliding Window Precision

```jounce
// Sliding window avoids fixed window edge cases
let mut limiter = SlidingWindowLimiter::new(2, 1000);

// Requests at: 900ms, 1000ms, 1100ms
let (updated, _) = limiter.check("user1", 900);
limiter = updated;

let (updated, _) = limiter.check("user1", 1000);
limiter = updated;

// This would be allowed in fixed window (new window)
// but denied in sliding window (within 1000ms of first request)
let (updated, result) = limiter.check("user1", 1100);
limiter = updated;
assert!(!result.allowed, "Sliding window tracks precisely");

// After 1900ms, first request expires
let (updated, result) = limiter.check("user1", 1900);
limiter = updated;
assert!(result.allowed, "Old requests expire");
```

## API Reference

### FixedWindowLimiter

```jounce
FixedWindowLimiter::new(max_requests: int, window_size: int) -> FixedWindowLimiter
limiter.check(key: string, now: int) -> (FixedWindowLimiter, RateLimitResult)
limiter.reset(key: string) -> FixedWindowLimiter
```

### TokenBucketLimiter

```jounce
TokenBucketLimiter::new(capacity: int, refill_rate: int, refill_interval: int) -> TokenBucketLimiter
limiter.check(key: string, now: int) -> (TokenBucketLimiter, RateLimitResult)
limiter.reset(key: string) -> TokenBucketLimiter
```

### SlidingWindowLimiter

```jounce
SlidingWindowLimiter::new(max_requests: int, window_size: int) -> SlidingWindowLimiter
limiter.check(key: string, now: int) -> (SlidingWindowLimiter, RateLimitResult)
limiter.reset(key: string) -> SlidingWindowLimiter
```

### LeakyBucketLimiter

```jounce
LeakyBucketLimiter::new(capacity: int, leak_rate: int, leak_interval: int) -> LeakyBucketLimiter
limiter.check(key: string, now: int) -> (LeakyBucketLimiter, RateLimitResult)
limiter.reset(key: string) -> LeakyBucketLimiter
```

### RateLimitResult

```jounce
struct RateLimitResult {
    allowed: bool,
    remaining: int,
    reset_at: int,
    retry_after: int,
}

RateLimitResult::allowed(remaining: int, reset_at: int) -> RateLimitResult
RateLimitResult::denied(retry_after: int, reset_at: int) -> RateLimitResult
```

### RateLimiterConfig

```jounce
RateLimiterConfig::new(max_requests: int, window_size: int) -> RateLimiterConfig
config.with_strategy(strategy: RateLimitStrategy) -> RateLimiterConfig
RateLimiterConfig::per_second(max_requests: int) -> RateLimiterConfig
RateLimiterConfig::per_minute(max_requests: int) -> RateLimiterConfig
RateLimiterConfig::per_hour(max_requests: int) -> RateLimiterConfig
RateLimiterConfig::per_day(max_requests: int) -> RateLimiterConfig
```

### RateLimitHeaders

```jounce
RateLimitHeaders::from_result(result: RateLimitResult, limit: int) -> RateLimitHeaders
headers.to_map() -> Map<string, string>
```

### Utility Functions

```jounce
create_key(prefix: string, identifier: string) -> string
```

## Algorithm Comparison

| Algorithm | Pros | Cons | Use Case |
|-----------|------|------|----------|
| **Fixed Window** | Simple, memory efficient | Burst at window edges | General API limits |
| **Sliding Window** | Precise, no edge cases | More memory | Strict rate enforcement |
| **Token Bucket** | Allows bursts, flexible | Complex | Bursty traffic patterns |
| **Leaky Bucket** | Smooth rate, queue-like | Rejects bursts | Steady traffic flow |

## Best Practices

1. **Choose the Right Algorithm** - Fixed window for simplicity, sliding window for precision, token bucket for bursts
2. **Set Appropriate Limits** - Balance user experience with resource protection
3. **Use Rate Limit Headers** - Inform clients of their limits
4. **Monitor Usage** - Track rate limit hits to adjust limits
5. **Per-User Limits** - Prevent single user from affecting others
6. **Graceful Degradation** - Return helpful error messages with retry info
7. **Reset Carefully** - Only reset when explicitly needed (e.g., admin action)
8. **Test Thoroughly** - Ensure limits work under load

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
