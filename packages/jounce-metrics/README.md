

# jounce-metrics

Performance metrics and monitoring for Jounce - track counters, gauges, histograms, timers, and custom metrics.

## Features

- ✅ **Counter** - Monotonically increasing values
- ✅ **Gauge** - Values that can go up or down
- ✅ **Histogram** - Distribution of values with statistics
- ✅ **Timer** - Measure duration and track multiple runs
- ✅ **Summary** - Statistical summaries with sliding windows
- ✅ **Tags & Labels** - Tag metrics for filtering
- ✅ **Metric Registry** - Centralized metric management
- ✅ **Stopwatch** - Simple timing utility with laps
- ✅ **Performance Monitor** - Track performance with thresholds
- ✅ **Metric Reporter** - Generate metric reports
- ✅ **Utility Functions** - Time conversion helpers

## Installation

```bash
jnc pkg add jounce-metrics
```

## Quick Start

```jounce
use jounce_metrics::{Counter, Gauge, Timer, MetricRegistry};

// Count requests
let requests = Counter::new("http_requests")
    .increment_by(1);

// Track CPU usage
let cpu = Gauge::new("cpu_usage")
    .set(75.5);

// Time operations
let timer = Timer::new("query_time")
    .start()
    .stop();

// Register metrics
let registry = MetricRegistry::new()
    .register_counter(requests)
    .register_gauge(cpu);
```

## Usage

### Counter - Monotonically Increasing

```jounce
use jounce_metrics::Counter;

// Create counter
let counter = Counter::new("api_requests");

// Increment by 1
let counter = counter.increment();

// Increment by specific amount
let counter = counter.increment_by(100);

// Add tags
let counter = Counter::new("http_requests")
    .with_tag("method", "GET")
    .with_tag("endpoint", "/users");

// Get value
let value = counter.get_value();

// Reset counter
let counter = counter.reset();
```

### Gauge - Value That Can Change

```jounce
use jounce_metrics::Gauge;

// Create gauge
let gauge = Gauge::new("memory_usage");

// Set value
let gauge = gauge.set(1024.5);

// Increment/decrement
let gauge = gauge.increment();  // +1
let gauge = gauge.decrement();  // -1

// Increment/decrement by amount
let gauge = gauge.increment_by(50.5);
let gauge = gauge.decrement_by(25.0);

// With tags
let gauge = Gauge::new("cpu_usage")
    .with_tag("host", "server1")
    .with_tag("region", "us-east");

// Get value
let value = gauge.get_value();
```

### Histogram - Distribution of Values

```jounce
use jounce_metrics::Histogram;

// Create histogram
let hist = Histogram::new("response_time");

// Record observations
let hist = hist.observe(0.5)
               .observe(1.2)
               .observe(0.8)
               .observe(2.1);

// Get statistics
let count = hist.count();        // Number of observations
let sum = hist.sum();            // Sum of all values
let mean = hist.mean();          // Average value
let min = hist.min();            // Minimum value
let max = hist.max();            // Maximum value

// Custom buckets
let mut buckets = Array::new();
buckets.push(0.1);
buckets.push(0.5);
buckets.push(1.0);
buckets.push(5.0);

let hist = Histogram::new("latency")
    .with_buckets(buckets);

// With tags
let hist = Histogram::new("request_size")
    .with_tag("route", "/api/users");
```

### Timer - Measure Duration

```jounce
use jounce_metrics::Timer;

// Create and use timer
let timer = Timer::new("database_query")
    .start();

// ... perform operation ...

let timer = timer.stop();

// Get elapsed time
let elapsed = timer.elapsed();

// Multiple runs
let timer = Timer::new("operation")
    .start()
    .stop()
    .start()
    .stop();

// Get average duration
let avg = timer.average_duration();

// Get total duration
let total = timer.total_duration();

// With tags
let timer = Timer::new("query")
    .with_tag("database", "postgres")
    .with_tag("table", "users");
```

### Summary - Statistical Summary

```jounce
use jounce_metrics::Summary;

// Create summary
let summary = Summary::new("response_time");

// Record observations
let summary = summary.observe(1.5)
                     .observe(2.3)
                     .observe(1.8);

// Get statistics
let count = summary.count();
let sum = summary.sum();
let mean = summary.mean();
let min = summary.min();
let max = summary.max();

// With maximum size (sliding window)
let summary = Summary::new("metrics")
    .with_max_size(1000);  // Keep only last 1000 values

// With tags
let summary = Summary::new("latency")
    .with_tag("service", "api");
```

### Stopwatch - Simple Timing

```jounce
use jounce_metrics::Stopwatch;

// Create and start stopwatch
let sw = Stopwatch::new()
    .start();

// Record laps
let sw = sw.lap();  // Lap 1
let sw = sw.lap();  // Lap 2
let sw = sw.lap();  // Lap 3

// Stop stopwatch
let sw = sw.stop();

// Get lap times
let lap_count = sw.lap_count();
let first_lap = sw.get_lap(0);

// Get elapsed time
let elapsed = sw.elapsed();

// Reset stopwatch
let sw = sw.reset();
```

### Performance Monitor - Track Performance

```jounce
use jounce_metrics::PerformanceMonitor;

// Create monitor with threshold
let monitor = PerformanceMonitor::new("api_response")
    .with_threshold(1000.0)  // 1 second threshold
    .with_max_samples(100);   // Keep last 100 samples

// Record samples
let monitor = monitor.record(150.0)
                     .record(250.0)
                     .record(1200.0);  // Violation!

// Get statistics
let avg = monitor.average();
let is_slow = monitor.is_above_threshold();
let violations = monitor.violations();  // Count of threshold violations
```

### Metric Registry - Centralized Management

```jounce
use jounce_metrics::{MetricRegistry, Counter, Gauge, Histogram};

// Create metrics
let requests = Counter::new("requests");
let cpu = Gauge::new("cpu");
let latency = Histogram::new("latency");

// Create registry and register metrics
let registry = MetricRegistry::new()
    .register_counter(requests)
    .register_gauge(cpu)
    .register_histogram(latency);

// Retrieve metrics
let counter = registry.get_counter("requests");
let gauge = registry.get_gauge("cpu");

// Get counts
let counter_count = registry.counter_count();
let gauge_count = registry.gauge_count();
```

### Metric Reporter - Generate Reports

```jounce
use jounce_metrics::{MetricReporter, MetricRegistry};

// Create registry with metrics
let registry = MetricRegistry::new()
    .register_counter(Counter::new("total"))
    .register_gauge(Gauge::new("current"));

// Create reporter
let reporter = MetricReporter::new(registry);

// Generate report
let reports = reporter.generate_report();
```

### Utility Functions

```jounce
use jounce_metrics::{milliseconds_to_seconds, seconds_to_milliseconds};

// Convert milliseconds to seconds
let seconds = milliseconds_to_seconds(5000);  // 5.0

// Convert seconds to milliseconds
let ms = seconds_to_milliseconds(2.5);  // 2500
```

### Complete Example: API Monitoring

```jounce
use jounce_metrics::{Counter, Gauge, Histogram, Timer, MetricRegistry};

struct ApiMetrics {
    pub requests: Counter,
    pub active_connections: Gauge,
    pub response_times: Histogram,
    pub db_timer: Timer,
}

impl ApiMetrics {
    pub fn new() -> ApiMetrics {
        ApiMetrics {
            requests: Counter::new("api_requests")
                .with_tag("service", "user-api"),
            active_connections: Gauge::new("connections"),
            response_times: Histogram::new("response_time"),
            db_timer: Timer::new("db_query"),
        }
    }

    pub fn handle_request(mut self) -> ApiMetrics {
        // Increment request count
        self.requests = self.requests.increment();

        // Increase active connections
        self.active_connections = self.active_connections.increment();

        // Time database query
        self.db_timer = self.db_timer.start();
        // ... execute query ...
        self.db_timer = self.db_timer.stop();

        // Record response time
        let response_time = 0.125;  // seconds
        self.response_times = self.response_times.observe(response_time);

        // Decrease active connections
        self.active_connections = self.active_connections.decrement();

        self
    }
}
```

### Complete Example: Performance Tracking

```jounce
use jounce_metrics::{PerformanceMonitor, Stopwatch};

fn track_performance() {
    // Create performance monitor
    let mut monitor = PerformanceMonitor::new("batch_processing")
        .with_threshold(5000.0)  // 5 second threshold
        .with_max_samples(50);

    // Create stopwatch for detailed timing
    let mut sw = Stopwatch::new().start();

    // Process batches
    let mut batch = 0;
    while batch < 10 {
        sw = sw.lap();

        // ... process batch ...

        let lap_time = sw.get_lap(batch);
        monitor = monitor.record(lap_time as float);

        batch = batch + 1;
    }

    sw = sw.stop();

    // Check performance
    let avg = monitor.average();
    let violations = monitor.violations();

    if monitor.is_above_threshold() {
        println("WARNING: Average processing time exceeded threshold");
        println("Average: " + avg.to_string() + "ms");
        println("Violations: " + violations.to_string());
    }
}
```

### Complete Example: Application Metrics

```jounce
use jounce_metrics::{Counter, Gauge, Histogram, Timer, Summary, MetricRegistry, MetricReporter};

fn setup_application_metrics() -> MetricRegistry {
    // Create counters
    let http_requests = Counter::new("http_requests")
        .with_tag("app", "web-server");

    let errors = Counter::new("errors")
        .with_tag("severity", "error");

    // Create gauges
    let cpu_usage = Gauge::new("cpu_usage");
    let memory_usage = Gauge::new("memory_mb");
    let active_users = Gauge::new("active_users");

    // Create histograms
    let request_duration = Histogram::new("request_duration");
    let response_size = Histogram::new("response_bytes");

    // Create timers
    let db_query_timer = Timer::new("db_query");
    let cache_timer = Timer::new("cache_lookup");

    // Create summaries
    let latency_summary = Summary::new("latency_p99")
        .with_max_size(1000);

    // Create registry and register all metrics
    MetricRegistry::new()
        .register_counter(http_requests)
        .register_counter(errors)
        .register_gauge(cpu_usage)
        .register_gauge(memory_usage)
        .register_gauge(active_users)
        .register_histogram(request_duration)
        .register_histogram(response_size)
        .register_timer(db_query_timer)
        .register_timer(cache_timer)
        .register_summary(latency_summary)
}

fn report_metrics(registry: MetricRegistry) {
    let reporter = MetricReporter::new(registry);
    let reports = reporter.generate_report();

    println("Generated " + reports.len().to_string() + " metric reports");
}
```

## API Reference

### Counter

```jounce
Counter::new(name: string) -> Counter
counter.with_tag(key: string, value: string) -> Counter
counter.increment() -> Counter
counter.increment_by(amount: int) -> Counter
counter.get_value() -> int
counter.reset() -> Counter
```

### Gauge

```jounce
Gauge::new(name: string) -> Gauge
gauge.with_tag(key: string, value: string) -> Gauge
gauge.set(value: float) -> Gauge
gauge.increment() -> Gauge
gauge.decrement() -> Gauge
gauge.increment_by(amount: float) -> Gauge
gauge.decrement_by(amount: float) -> Gauge
gauge.get_value() -> float
```

### Histogram

```jounce
Histogram::new(name: string) -> Histogram
histogram.with_buckets(buckets: Array<float>) -> Histogram
histogram.with_tag(key: string, value: string) -> Histogram
histogram.observe(value: float) -> Histogram
histogram.count() -> int
histogram.sum() -> float
histogram.mean() -> float
histogram.min() -> float
histogram.max() -> float
```

### Timer

```jounce
Timer::new(name: string) -> Timer
timer.with_tag(key: string, value: string) -> Timer
timer.start() -> Timer
timer.stop() -> Timer
timer.elapsed() -> int
timer.average_duration() -> float
timer.total_duration() -> int
```

### Summary

```jounce
Summary::new(name: string) -> Summary
summary.with_max_size(max_size: int) -> Summary
summary.with_tag(key: string, value: string) -> Summary
summary.observe(value: float) -> Summary
summary.count() -> int
summary.sum() -> float
summary.mean() -> float
summary.min() -> float
summary.max() -> float
```

### Stopwatch

```jounce
Stopwatch::new() -> Stopwatch
stopwatch.start() -> Stopwatch
stopwatch.lap() -> Stopwatch
stopwatch.stop() -> Stopwatch
stopwatch.elapsed() -> int
stopwatch.lap_count() -> int
stopwatch.get_lap(index: int) -> int
stopwatch.reset() -> Stopwatch
```

### PerformanceMonitor

```jounce
PerformanceMonitor::new(name: string) -> PerformanceMonitor
monitor.with_max_samples(max: int) -> PerformanceMonitor
monitor.with_threshold(threshold: float) -> PerformanceMonitor
monitor.record(value: float) -> PerformanceMonitor
monitor.average() -> float
monitor.is_above_threshold() -> bool
monitor.violations() -> int
```

### MetricRegistry

```jounce
MetricRegistry::new() -> MetricRegistry
registry.register_counter(counter: Counter) -> MetricRegistry
registry.register_gauge(gauge: Gauge) -> MetricRegistry
registry.register_histogram(histogram: Histogram) -> MetricRegistry
registry.register_timer(timer: Timer) -> MetricRegistry
registry.register_summary(summary: Summary) -> MetricRegistry
registry.get_counter(name: string) -> Option<Counter>
registry.get_gauge(name: string) -> Option<Gauge>
registry.counter_count() -> int
registry.gauge_count() -> int
```

### Utility Functions

```jounce
milliseconds_to_seconds(ms: int) -> float
seconds_to_milliseconds(seconds: float) -> int
```

## Metric Types

| Type | Use Case | Example |
|------|----------|---------|
| **Counter** | Monotonically increasing values | HTTP requests, errors, bytes sent |
| **Gauge** | Values that go up and down | CPU usage, memory, active connections |
| **Histogram** | Distribution of values | Request duration, response size |
| **Timer** | Measure duration | Database queries, API calls |
| **Summary** | Statistical summaries | Percentiles, sliding windows |

## Best Practices

1. **Use Tags** - Tag metrics for filtering and aggregation
2. **Choose Correct Type** - Use counter for counts, gauge for current values
3. **Set Thresholds** - Use PerformanceMonitor for SLA tracking
4. **Centralize Registry** - Use MetricRegistry for all metrics
5. **Monitor Key Metrics** - Track request rate, error rate, latency
6. **Regular Reporting** - Generate reports periodically
7. **Sliding Windows** - Use Summary with max_size for recent data only

## Common Metrics

### Application Metrics
- HTTP requests (counter)
- Error rate (counter)
- Response time (histogram)
- Active connections (gauge)

### System Metrics
- CPU usage (gauge)
- Memory usage (gauge)
- Disk I/O (counter)
- Network traffic (counter)

### Business Metrics
- User signups (counter)
- Active users (gauge)
- Revenue (counter)
- Conversion rate (gauge)

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
