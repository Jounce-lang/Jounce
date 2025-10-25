# jounce-cache

In-memory and Redis caching with LRU eviction, TTL support, and cache statistics for Jounce applications.

## Features

- ✅ **In-Memory Cache** - Fast LRU cache with configurable size
- ✅ **Eviction Policies** - LRU, LFU, FIFO eviction strategies
- ✅ **TTL Support** - Automatic expiration with TTL
- ✅ **Redis Adapter** - Distributed caching with Redis
- ✅ **Cache Statistics** - Hit rate, misses, evictions tracking
- ✅ **Multiple Caches** - Named cache instances
- ✅ **Automatic Cleanup** - Periodic expired entry removal
- ✅ **Type-Safe** - Generic type support

## Installation

```bash
jnc pkg add jounce-cache
```

## Quick Start

```jounce
use jounce_cache::{cache_set, cache_get};

fn main() {
    // Set a value in the default cache
    cache_set("user:123", "John Doe");

    // Get a value from the cache
    let user = cache_get("user:123");
    if let Some(name) = user {
        println("User: {}", name);
    }
}
```

## Usage

### Basic In-Memory Caching

```jounce
use jounce_cache::{Cache, CacheConfig};

// Create a cache with default configuration
let config = CacheConfig::default();
let mut cache = Cache::new(config);

// Set and get values
cache.set("key1", "value1");
let value = cache.get("key1"); // Some("value1")

// Check if key exists
if cache.has("key1") {
    println("Key exists!");
}

// Remove a key
cache.remove("key1");

// Clear all entries
cache.clear();
```

### LRU Cache with Custom Size

```jounce
use jounce_cache::{Cache, CacheConfig, EvictionPolicy};

let config = CacheConfig::default()
    .with_max_size(100)
    .with_policy(EvictionPolicy::LRU);

let mut cache = Cache::new(config);

// When cache is full, least recently used items are evicted
for i in 0..150 {
    cache.set("key" + i.to_string(), "value" + i.to_string());
}

// Only the 100 most recently used items remain
assert_eq!(cache.size(), 100);
```

### TTL (Time to Live)

```jounce
use jounce_cache::{Cache, CacheConfig};

let config = CacheConfig::default().with_ttl(60); // 60 seconds
let mut cache = Cache::new(config);

// Set with default TTL (60 seconds)
cache.set("session:abc", "user_data");

// Set with custom TTL (10 seconds)
cache.set_with_ttl("temp:xyz", "temporary_data", 10);

// Manually remove expired entries
let removed = cache.evict_expired();
println("Removed {} expired entries", removed);
```

### Eviction Policies

```jounce
use jounce_cache::{Cache, CacheConfig, EvictionPolicy};

// LRU (Least Recently Used) - evicts least recently accessed
let lru_config = CacheConfig::default()
    .with_max_size(50)
    .with_policy(EvictionPolicy::LRU);

// LFU (Least Frequently Used) - evicts least frequently accessed
let lfu_config = CacheConfig::default()
    .with_max_size(50)
    .with_policy(EvictionPolicy::LFU);

// FIFO (First In First Out) - evicts oldest entries
let fifo_config = CacheConfig::default()
    .with_max_size(50)
    .with_policy(EvictionPolicy::FIFO);

let mut cache = Cache::new(lru_config);
```

### Redis Cache

```jounce
use jounce_cache::{RedisCache, RedisConfig};

// Configure Redis connection
let config = RedisConfig::new("localhost", 6379)
    .with_password("secret")
    .with_database(0)
    .with_prefix("myapp:");

let mut cache = RedisCache::new(config);

// Connect to Redis
if cache.connect() {
    // Set a value
    cache.set("user:123", "John Doe");

    // Set with TTL (60 seconds)
    cache.set_with_ttl("session:abc", "data", 60);

    // Get a value
    let user = cache.get("user:123");

    // Check if key exists
    if cache.exists("user:123") {
        println("Key exists in Redis");
    }

    // Get TTL of a key
    let ttl = cache.ttl("session:abc");

    // Remove a key
    cache.remove("user:123");

    // Clear keys matching a pattern
    cache.clear_pattern("user:*");

    // Flush entire database
    cache.flush();

    cache.disconnect();
}
```

### Cache Manager with Statistics

```jounce
use jounce_cache::{CacheManager, CacheConfig};

let config = CacheConfig::default();
let mut manager = CacheManager::with_memory(config);

// Set and get values
manager.set("key1", "value1");
manager.set("key2", "value2");

manager.get("key1"); // Hit
manager.get("key3"); // Miss
manager.get("key1"); // Hit

// Get statistics
let stats = manager.stats();
println("Hits: {}", stats.hits);       // 2
println("Misses: {}", stats.misses);   // 1
println("Hit Rate: {:.2}", stats.hit_rate()); // 0.67 (67%)
```

### Named Caches

```jounce
use jounce_cache::{get_cache, register_cache, CacheManager, CacheConfig};

// Get or create a named cache
let mut user_cache = get_cache("users");
user_cache.set("123", "John");

let mut session_cache = get_cache("sessions");
session_cache.set("abc", "session_data");

// Register a custom configured cache
let config = CacheConfig::default().with_max_size(500);
let custom_cache = CacheManager::with_memory(config);
register_cache("custom", custom_cache);
```

### Global Convenience Functions

```jounce
use jounce_cache::{cache_set, cache_get, cache_remove, cache_clear};

// Uses the default cache
cache_set("key1", "value1");
cache_set("key2", "value2");

let value = cache_get("key1"); // Some("value1")

cache_remove("key1");

cache_clear(); // Clear all entries
```

## Configuration

### CacheConfig

```jounce
pub struct CacheConfig {
    pub max_size: int,              // Maximum cache size
    pub default_ttl: int,           // Default TTL in seconds (0 = no expiration)
    pub eviction_policy: EvictionPolicy,
    pub check_expiration_interval: int, // Auto-cleanup interval (0 = disabled)
}
```

### RedisConfig

```jounce
pub struct RedisConfig {
    pub host: string,       // Redis host
    pub port: int,          // Redis port
    pub password: string,   // Authentication password
    pub database: int,      // Database number (0-15)
    pub prefix: string,     // Key prefix for namespacing
}
```

## API Reference

### Cache<T>

- `Cache::new(config: CacheConfig) -> Cache<T>` - Create new cache
- `cache.set(key: string, value: T)` - Set value with default TTL
- `cache.set_with_ttl(key: string, value: T, ttl: int)` - Set value with custom TTL
- `cache.get(key: string) -> Option<T>` - Get value (returns None if expired/missing)
- `cache.has(key: string) -> bool` - Check if key exists and is not expired
- `cache.remove(key: string) -> bool` - Remove entry
- `cache.clear()` - Clear all entries
- `cache.size() -> int` - Get number of entries
- `cache.keys() -> Array<string>` - Get all keys
- `cache.evict_expired() -> int` - Manually remove expired entries

### RedisCache

- `RedisCache::new(config: RedisConfig) -> RedisCache` - Create Redis adapter
- `cache.connect() -> bool` - Connect to Redis
- `cache.disconnect()` - Disconnect from Redis
- `cache.is_connected() -> bool` - Check connection status
- `cache.set(key: string, value: string) -> bool` - Set value
- `cache.set_with_ttl(key: string, value: string, ttl: int) -> bool` - Set with TTL
- `cache.get(key: string) -> Option<string>` - Get value
- `cache.remove(key: string) -> bool` - Delete key
- `cache.exists(key: string) -> bool` - Check if key exists
- `cache.expire(key: string, ttl: int) -> bool` - Set expiration
- `cache.ttl(key: string) -> int` - Get remaining TTL
- `cache.clear_pattern(pattern: string) -> int` - Delete keys matching pattern
- `cache.flush() -> bool` - Delete all keys in database

### CacheManager<T>

- `CacheManager::with_memory(config: CacheConfig) -> CacheManager<T>` - In-memory backend
- `CacheManager::with_redis(config: RedisConfig) -> CacheManager<T>` - Redis backend
- `manager.set(key: string, value: T)` - Set value
- `manager.get(key: string) -> Option<T>` - Get value
- `manager.remove(key: string) -> bool` - Remove entry
- `manager.clear()` - Clear all entries
- `manager.stats() -> CacheStats` - Get statistics

### CacheStats

- `stats.hits: int` - Number of cache hits
- `stats.misses: int` - Number of cache misses
- `stats.evictions: int` - Number of evictions
- `stats.expired: int` - Number of expired entries
- `stats.hit_rate() -> float` - Hit rate (hits / (hits + misses))

### Global Functions

- `get_cache(name: string) -> CacheManager<string>` - Get or create named cache
- `register_cache(name: string, cache: CacheManager<string>)` - Register custom cache
- `cache_set(key: string, value: string)` - Set in default cache
- `cache_get(key: string) -> Option<string>` - Get from default cache
- `cache_remove(key: string) -> bool` - Remove from default cache
- `cache_clear()` - Clear default cache

## Eviction Policies

### LRU (Least Recently Used)

Evicts the entry that was accessed longest ago.

```jounce
let config = CacheConfig::default()
    .with_max_size(3)
    .with_policy(EvictionPolicy::LRU);

let mut cache = Cache::new(config);

cache.set("a", "1");
cache.set("b", "2");
cache.set("c", "3");

cache.get("a"); // Access 'a'

cache.set("d", "4"); // Evicts 'b' (least recently used)
```

### LFU (Least Frequently Used)

Evicts the entry with the lowest access count.

```jounce
let config = CacheConfig::default()
    .with_max_size(3)
    .with_policy(EvictionPolicy::LFU);

let mut cache = Cache::new(config);

cache.set("a", "1");
cache.get("a");  // access count: 2
cache.get("a");  // access count: 3

cache.set("b", "2"); // access count: 1
cache.set("c", "3"); // access count: 1

cache.set("d", "4"); // Evicts 'b' or 'c' (lowest access count)
```

### FIFO (First In First Out)

Evicts the oldest entry regardless of access.

```jounce
let config = CacheConfig::default()
    .with_max_size(3)
    .with_policy(EvictionPolicy::FIFO);

let mut cache = Cache::new(config);

cache.set("a", "1");
cache.set("b", "2");
cache.set("c", "3");

cache.set("d", "4"); // Evicts 'a' (first in)
```

## Best Practices

1. **Choose the Right Policy**: LRU for general use, LFU for stable access patterns, FIFO for time-based data
2. **Set Appropriate TTL**: Use TTL for session data, temporary results, and time-sensitive information
3. **Monitor Hit Rate**: Track cache statistics to optimize configuration
4. **Use Named Caches**: Separate caches for different data types
5. **Configure Size**: Set max_size based on memory constraints and data size
6. **Periodic Cleanup**: Use `evict_expired()` or enable automatic cleanup
7. **Key Namespacing**: Use prefixes like `user:`, `session:` to organize keys

## Examples

See `examples/` directory:
- `basic-cache.jnc` - Basic in-memory caching
- `lru-eviction.jnc` - LRU cache with eviction
- `ttl-expiration.jnc` - TTL and expiration
- `redis-cache.jnc` - Redis caching
- `cache-stats.jnc` - Statistics tracking

## Performance

- **In-Memory**: O(1) for get/set operations
- **LRU Eviction**: O(n) worst case for eviction
- **Redis**: Network latency + O(1) Redis operations
- **TTL Checking**: On-demand (only checked during get)

## License

MIT

## Version

0.1.0
