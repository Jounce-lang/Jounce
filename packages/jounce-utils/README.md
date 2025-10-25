# jounce-utils

Essential utility functions for Jounce applications.

## Features

- ✅ **String Utilities** - slugify, truncate, capitalize, case conversion
- ✅ **Array Utilities** - chunk, unique, flatten, partition, zip
- ✅ **Object Utilities** - merge, clone, pick, omit, keys, values
- ✅ **Date Utilities** - format, parse, diff, add, subtract

## Installation

```bash
jnc pkg add jounce-utils
```

## Usage

### String Utilities

```jounce
use jounce_utils::{slugify, truncate, capitalize, camel_case, snake_case, kebab_case};

// Convert to URL-friendly slug
let slug = slugify("Hello World!"); // "hello-world"

// Truncate with ellipsis
let short = truncate("This is a long string", 10); // "This is..."

// Capitalize first letter
let cap = capitalize("hello"); // "Hello"

// Case conversions
let camel = camel_case("hello_world"); // "helloWorld"
let snake = snake_case("HelloWorld"); // "hello_world"
let kebab = kebab_case("HelloWorld"); // "hello-world"

// String padding
let padded = pad_start("5", 3, "0"); // "005"

// Repeat string
let repeated = repeat("ha", 3); // "hahaha"
```

### Array Utilities

```jounce
use jounce_utils::{chunk, unique, flatten, partition, take, drop, zip};

// Split into chunks
let chunks = chunk(vec![1, 2, 3, 4, 5], 2);
// [[1, 2], [3, 4], [5]]

// Get unique values
let uniq = unique(vec![1, 2, 2, 3, 1]);
// [1, 2, 3]

// Flatten nested arrays
let flat = flatten(vec![vec![1, 2], vec![3, 4]]);
// [1, 2, 3, 4]

// Partition by condition
let (evens, odds) = partition(vec![1, 2, 3, 4], |x| x % 2 == 0);
// evens: [2, 4], odds: [1, 3]

// Take first n elements
let first_three = take(vec![1, 2, 3, 4, 5], 3);
// [1, 2, 3]

// Drop first n elements
let rest = drop(vec![1, 2, 3, 4, 5], 2);
// [3, 4, 5]

// Zip two arrays
let pairs = zip(vec![1, 2, 3], vec!["a", "b", "c"]);
// [(1, "a"), (2, "b"), (3, "c")]

// Group by key
let grouped = group_by(vec![1, 2, 3, 4], |x| x % 2);
// {0: [2, 4], 1: [1, 3]}
```

### Object Utilities

```jounce
use jounce_utils::{merge, clone, pick, omit, keys, values, entries};

let obj1 = Map::from([("a", 1), ("b", 2)]);
let obj2 = Map::from([("b", 3), ("c", 4)]);

// Merge objects (obj2 overwrites obj1)
let merged = merge(obj1, obj2);
// {a: 1, b: 3, c: 4}

// Clone object
let cloned = clone(obj1);

// Pick specific keys
let picked = pick(merged, vec!["a", "c"]);
// {a: 1, c: 4}

// Omit specific keys
let omitted = omit(merged, vec!["b"]);
// {a: 1, c: 4}

// Get keys, values, entries
let all_keys = keys(merged); // ["a", "b", "c"]
let all_values = values(merged); // [1, 3, 4]
let all_entries = entries(merged); // [("a", 1), ("b", 3), ("c", 4)]
```

### Date Utilities

```jounce
use jounce_utils::{now, format_date, parse_date, diff_days, add_days, subtract_days, is_before, is_after, DateTime};

// Get current date/time
let current = now();

// Format date
let formatted = format_date(current, "YYYY-MM-DD HH:mm:ss");
// "2025-10-24 12:00:00"

// Parse date from string
let parsed = parse_date("2025-10-24", "YYYY-MM-DD");

// Date arithmetic
let tomorrow = add_days(current, 1);
let yesterday = subtract_days(current, 1);

// Date comparison
let diff = diff_days(yesterday, tomorrow); // 2 days
let before = is_before(yesterday, tomorrow); // true
let after = is_after(tomorrow, yesterday); // true
```

## API Reference

### String Functions

- `slugify(s: string) -> string` - Convert to URL-friendly slug
- `truncate(s: string, max_length: int) -> string` - Truncate with ellipsis
- `capitalize(s: string) -> string` - Capitalize first letter
- `camel_case(s: string) -> string` - Convert to camelCase
- `snake_case(s: string) -> string` - Convert to snake_case
- `kebab_case(s: string) -> string` - Convert to kebab-case
- `trim(s: string) -> string` - Trim whitespace
- `pad_start(s: string, target_len: int, pad_char: string) -> string` - Pad at start
- `pad_end(s: string, target_len: int, pad_char: string) -> string` - Pad at end
- `repeat(s: string, count: int) -> string` - Repeat string n times

### Array Functions

- `chunk<T>(arr: Vec<T>, size: int) -> Vec<Vec<T>>` - Split into chunks
- `unique<T>(arr: Vec<T>) -> Vec<T>` - Get unique values
- `flatten<T>(arr: Vec<Vec<T>>) -> Vec<T>` - Flatten one level
- `partition<T>(arr: Vec<T>, predicate: fn(T) -> bool) -> (Vec<T>, Vec<T>)` - Split by condition
- `take<T>(arr: Vec<T>, n: int) -> Vec<T>` - Take first n elements
- `drop<T>(arr: Vec<T>, n: int) -> Vec<T>` - Drop first n elements
- `zip<T, U>(arr1: Vec<T>, arr2: Vec<U>) -> Vec<(T, U)>` - Zip two arrays
- `group_by<T, K>(arr: Vec<T>, key_fn: fn(T) -> K) -> Map<K, Vec<T>>` - Group by key

### Object Functions

- `merge<K, V>(obj1: Map<K, V>, obj2: Map<K, V>) -> Map<K, V>` - Merge objects
- `clone<K, V>(obj: Map<K, V>) -> Map<K, V>` - Deep clone
- `pick<K, V>(obj: Map<K, V>, keys: Vec<K>) -> Map<K, V>` - Pick keys
- `omit<K, V>(obj: Map<K, V>, keys: Vec<K>) -> Map<K, V>` - Omit keys
- `keys<K, V>(obj: Map<K, V>) -> Vec<K>` - Get all keys
- `values<K, V>(obj: Map<K, V>) -> Vec<V>` - Get all values
- `entries<K, V>(obj: Map<K, V>) -> Vec<(K, V)>` - Get key-value pairs

### Date Functions

- `now() -> DateTime` - Get current date/time
- `format_date(dt: DateTime, pattern: string) -> string` - Format date
- `parse_date(s: string, pattern: string) -> DateTime` - Parse date
- `diff_days(dt1: DateTime, dt2: DateTime) -> int` - Days difference
- `add_days(dt: DateTime, days: int) -> DateTime` - Add days
- `subtract_days(dt: DateTime, days: int) -> DateTime` - Subtract days
- `is_before(dt1: DateTime, dt2: DateTime) -> bool` - Check if before
- `is_after(dt1: DateTime, dt2: DateTime) -> bool` - Check if after

## Types

```jounce
struct DateTime {
    year: int,
    month: int,
    day: int,
    hour: int,
    minute: int,
    second: int,
}
```

## Date Format Patterns

- `YYYY` - 4-digit year (2025)
- `MM` - 2-digit month (01-12)
- `DD` - 2-digit day (01-31)
- `HH` - 2-digit hour (00-23)
- `mm` - 2-digit minute (00-59)
- `ss` - 2-digit second (00-59)

Examples:
- `"YYYY-MM-DD"` → "2025-10-24"
- `"MM/DD/YYYY"` → "10/24/2025"
- `"YYYY-MM-DD HH:mm:ss"` → "2025-10-24 14:30:00"

## Examples

See `examples/` directory for complete examples:
- `string-utils.jnc` - String manipulation examples
- `array-utils.jnc` - Array operation examples
- `object-utils.jnc` - Object handling examples
- `date-utils.jnc` - Date formatting and manipulation

## Best Practices

1. **String Operations**: Use built-in utilities for consistent string handling
2. **Array Processing**: Leverage functional utilities for cleaner code
3. **Object Manipulation**: Use `pick`/`omit` for safer object operations
4. **Date Handling**: Always use formatted dates for consistency

## Performance

All utilities are implemented with performance in mind:
- String operations use efficient concatenation
- Array operations minimize allocations
- Object operations use shallow clones where possible
- Date utilities use simple integer math

## License

MIT

## Version

0.1.0
