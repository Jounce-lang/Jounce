# jounce-queue

Job queue and background task processing with priority queues, retry logic, workers, and scheduling.

## Features

- ✅ **Job Queue** - FIFO queue with priority support
- ✅ **Priority Jobs** - High, normal, low priority
- ✅ **Retry Logic** - Automatic retries with configurable attempts
- ✅ **Worker Pool** - Concurrent job processing
- ✅ **Job Status** - Pending, processing, completed, failed, retrying
- ✅ **Delayed Jobs** - Schedule jobs for later execution
- ✅ **Queue Management** - Pause, resume, clear queues
- ✅ **Statistics** - Track job counts and worker stats

## Installation

```bash
jnc pkg add jounce-queue
```

## Quick Start

### Basic Queue

```jounce
use jounce_queue::{Queue, Job};

// Create queue
let mut queue = Queue::new("emails");

// Add jobs
let job1 = Job::new("job1", "send_email", "user@example.com");
queue = queue.add(job1);

// Get next job
let next = queue.get_next_job();
if next.is_some() {
    let job = next.unwrap();
    println("Processing: " + job.name);
}
```

### With Workers

```jounce
use jounce_queue::{QueueManager, Job};

// Create manager with queue and 3 workers
let mut manager = QueueManager::new()
    .create_queue_with_workers("emails", 3);

// Add jobs
manager = manager.add_job("emails", Job::new("job1", "send_email", "data"));
```

## Usage

### Creating Jobs

```jounce
use jounce_queue::{Job, JobBuilder, JobPriority};

// Simple job
let job = Job::new("job1", "process_image", "image_data");

// Job with priority
let high_priority = Job::new("job2", "urgent_task", "data")
    .with_priority(JobPriority::High);

// Job with retry limit
let retry_job = Job::new("job3", "api_call", "data")
    .with_max_attempts(5);

// Using JobBuilder (fluent API)
let job = JobBuilder::new("job4", "complex_task")
    .with_data("task data")
    .with_priority(JobPriority::High)
    .with_max_attempts(3)
    .build();
```

### Queue Operations

```jounce
use jounce_queue::{Queue, Job};

let mut queue = Queue::new("tasks");

// Add jobs
queue = queue.add(Job::new("job1", "task", "data"));

// Add with priority
queue = queue.add_high_priority(Job::new("job2", "urgent", "data"));
queue = queue.add_low_priority(Job::new("job3", "batch", "data"));

// Configure concurrency
queue = queue.with_max_concurrent(10);

// Get next job (respects priority)
let next = queue.get_next_job();

// Pause/resume
queue = queue.pause();
queue = queue.resume();

// Get stats
let pending = queue.get_pending_count();
let processing = queue.get_processing_count();
let completed = queue.get_completed_count();
let failed = queue.get_failed_count();
let total = queue.get_total_count();
```

### Job Processing

```jounce
// Start processing
let mut job = Job::new("job1", "task", "data");
job = job.start();

// Complete successfully
job = job.complete();

// Fail with error (will retry if attempts < max)
job = job.fail("Network timeout");

// Check if can retry
if job.can_retry() {
    println("Will retry job");
}
```

### Priority Queue

```jounce
let mut queue = Queue::new("tasks");

// Add jobs with different priorities
queue = queue
    .add_low_priority(Job::new("job1", "batch", "data"))
    .add(Job::new("job2", "normal", "data"))
    .add_high_priority(Job::new("job3", "urgent", "data"));

// Get next job - returns high priority first
let next = queue.get_next_job();
// Returns job3 (high priority)
```

### Workers

```jounce
use jounce_queue::{Worker, Job};

// Create worker
let mut worker = Worker::new("worker1", "email_queue");

// Start worker
worker = worker.start();

// Process job
let job = Job::new("job1", "send_email", "data");
worker = worker.process_job(job);

// Check if busy
if worker.is_busy() {
    println("Worker is processing");
}

// Finish job (success)
worker = worker.finish_job(true);

// Finish job (failure)
worker = worker.finish_job(false);

// Get stats
let stats = worker.get_stats();
println("Processed: " + stats.processed.to_string());
println("Failed: " + stats.failed.to_string());
```

### Worker Pool

```jounce
use jounce_queue::{WorkerPool, Job};

// Create pool with 5 workers
let mut pool = WorkerPool::new("tasks", 5);

// Start all workers
pool = pool.start_all();

// Get available worker
let worker = pool.get_available_worker();
if worker.is_some() {
    println("Worker available");
}

// Get totals
let processed = pool.get_total_processed();
let failed = pool.get_total_failed();

// Stop all workers
pool = pool.stop_all();
```

### Queue Manager

```jounce
use jounce_queue::{QueueManager, Job};

let mut manager = QueueManager::new();

// Create queues
manager = manager
    .create_queue("emails")
    .create_queue("images")
    .create_queue_with_workers("notifications", 3);

// Add jobs to specific queues
manager = manager
    .add_job("emails", Job::new("job1", "send_email", "data"))
    .add_job("images", Job::new("job2", "resize", "data"));

// Get queue
let email_queue = manager.get_queue("emails");
if email_queue.is_some() {
    println("Email queue has " + email_queue.unwrap().jobs.len().to_string() + " jobs");
}

// Get worker pool
let workers = manager.get_worker_pool("notifications");

// Delete queue
manager = manager.delete_queue("emails");
```

### Delayed Jobs

```jounce
use jounce_queue::{DelayedQueue, Job};

let mut delayed = DelayedQueue::new("scheduled");

// Add job with delay (in milliseconds)
let job = Job::new("job1", "send_reminder", "data");
delayed = delayed.add_delayed_job(job, 5000); // Delay 5 seconds

// Check for ready jobs
let ready = delayed.get_ready_jobs();
```

### Retry Logic

```jounce
// Job with automatic retry
let mut job = Job::new("job1", "api_call", "data")
    .with_max_attempts(3);

// First attempt fails
job = job.start().fail("Connection timeout");
// Status: Retrying

// Second attempt fails
job = job.start().fail("Connection timeout");
// Status: Retrying

// Third attempt fails
job = job.start().fail("Connection timeout");
// Status: Failed (max attempts reached)
```

### Complete Workflow

```jounce
use jounce_queue::{QueueManager, Job, JobPriority};

// 1. Setup manager with queue and workers
let mut manager = QueueManager::new()
    .create_queue_with_workers("emails", 3);

// 2. Add jobs with priorities
manager = manager
    .add_job("emails",
        Job::new("job1", "welcome_email", "user1@test.com")
            .with_priority(JobPriority::High))
    .add_job("emails",
        Job::new("job2", "newsletter", "user2@test.com")
            .with_priority(JobPriority::Low));

// 3. Get queue and process
let mut queue = manager.get_queue("emails").unwrap();

// 4. Get next job (high priority first)
let next = queue.get_next_job();
if next.is_some() {
    let mut job = next.unwrap();

    // 5. Process job
    queue = queue.start_processing(job.clone());

    // ... do work ...

    // 6. Complete or fail
    queue = queue.complete_job(job.id);
    // OR: queue = queue.fail_job(job.id, "Error message");
}

// 7. Get stats
println("Pending: " + queue.get_pending_count().to_string());
println("Processing: " + queue.get_processing_count().to_string());
println("Completed: " + queue.get_completed_count().to_string());
```

## API Reference

### Job

```jounce
Job::new(id: string, name: string, data: string) -> Job
job.with_priority(priority: JobPriority) -> Job
job.with_max_attempts(max: int) -> Job
job.with_delay(delay_ms: int) -> Job
job.start() -> Job
job.complete() -> Job
job.fail(error: string) -> Job
job.can_retry() -> bool
job.is_ready() -> bool
```

### JobPriority

```jounce
JobPriority::High
JobPriority::Normal
JobPriority::Low
```

### JobStatus

```jounce
JobStatus::Pending
JobStatus::Processing
JobStatus::Completed
JobStatus::Failed
JobStatus::Retrying
JobStatus::Delayed
```

### Queue

```jounce
Queue::new(name: string) -> Queue
queue.with_max_concurrent(max: int) -> Queue
queue.add(job: Job) -> Queue
queue.add_high_priority(job: Job) -> Queue
queue.add_low_priority(job: Job) -> Queue
queue.get_next_job() -> Option<Job>
queue.start_processing(job: Job) -> Queue
queue.complete_job(job_id: string) -> Queue
queue.fail_job(job_id: string, error: string) -> Queue
queue.pause() -> Queue
queue.resume() -> Queue
queue.get_pending_count() -> int
queue.get_processing_count() -> int
queue.get_completed_count() -> int
queue.get_failed_count() -> int
queue.get_total_count() -> int
queue.clear_completed() -> Queue
queue.clear_failed() -> Queue
```

### Worker

```jounce
Worker::new(id: string, queue_name: string) -> Worker
worker.start() -> Worker
worker.stop() -> Worker
worker.process_job(job: Job) -> Worker
worker.finish_job(success: bool) -> Worker
worker.is_busy() -> bool
worker.get_stats() -> WorkerStats
```

### WorkerPool

```jounce
WorkerPool::new(queue_name: string, worker_count: int) -> WorkerPool
pool.start_all() -> WorkerPool
pool.stop_all() -> WorkerPool
pool.get_available_worker() -> Option<Worker>
pool.get_total_processed() -> int
pool.get_total_failed() -> int
```

### QueueManager

```jounce
QueueManager::new() -> QueueManager
manager.create_queue(name: string) -> QueueManager
manager.create_queue_with_workers(name: string, worker_count: int) -> QueueManager
manager.add_job(queue_name: string, job: Job) -> QueueManager
manager.get_queue(name: string) -> Option<Queue>
manager.get_worker_pool(name: string) -> Option<WorkerPool>
manager.delete_queue(name: string) -> QueueManager
```

### DelayedQueue

```jounce
DelayedQueue::new(name: string) -> DelayedQueue
delayed.add_delayed_job(job: Job, delay_ms: int) -> DelayedQueue
delayed.get_ready_jobs() -> Array<Job>
```

### JobBuilder

```jounce
JobBuilder::new(id: string, name: string) -> JobBuilder
builder.with_data(data: string) -> JobBuilder
builder.with_priority(priority: JobPriority) -> JobBuilder
builder.with_max_attempts(max: int) -> JobBuilder
builder.with_delay(delay_ms: int) -> JobBuilder
builder.build() -> Job
```

## Best Practices

1. **Use Priority** - Assign appropriate priorities to jobs
2. **Configure Retries** - Set max_attempts based on job criticality
3. **Worker Pools** - Use multiple workers for concurrent processing
4. **Monitor Stats** - Track queue health with get_*_count methods
5. **Clean Up** - Periodically clear_completed() and clear_failed()
6. **Pause When Needed** - Use pause() during maintenance
7. **Delayed Jobs** - Use for scheduled tasks and reminders
8. **Queue Manager** - Organize different job types in separate queues

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
