// Cache performance benchmark
// Tests incremental compilation speed with caching enabled

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use jounce_compiler::{Compiler, BuildTarget, cache::CompilationCache};
use std::sync::Arc;
use std::path::PathBuf;

// Sample Jounce source code for testing
const SAMPLE_SOURCE: &str = r#"
@client
fn Counter() -> JSX {
    let count = 0;

    <div class="p-4 bg-white rounded shadow">
        <h2 class="text-2xl font-bold mb-4">Counter: {count}</h2>
        <button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
            Increment
        </button>
    </div>
}

@client
fn App() -> JSX {
    <div class="container mx-auto p-8">
        <h1 class="text-4xl font-bold mb-8">Welcome to Jounce!</h1>
        <Counter />
    </div>
}

fn main() {
    println("Hello from Jounce!");
}
"#;

fn bench_compilation_no_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation_no_cache");

    group.bench_function("compile_without_cache", |b| {
        b.iter(|| {
            let compiler = Compiler::new();
            let result = compiler.compile_source_with_css(
                black_box(SAMPLE_SOURCE),
                BuildTarget::Client
            );
            assert!(result.is_ok());
        });
    });

    group.finish();
}

fn bench_compilation_with_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation_with_cache");

    // First compilation (cold cache)
    group.bench_function("first_compile_cold_cache", |b| {
        b.iter(|| {
            let cache = Arc::new(CompilationCache::default());
            let result = jounce_compiler::cache::compile_source_cached(
                black_box(SAMPLE_SOURCE),
                &PathBuf::from("test.jnc"),
                BuildTarget::Client,
                &cache,
                true,
            );
            assert!(result.is_ok());
        });
    });

    // Second compilation (warm cache)
    group.bench_function("second_compile_warm_cache", |b| {
        let cache = Arc::new(CompilationCache::default());

        // Prime the cache
        let _ = jounce_compiler::cache::compile_source_cached(
            SAMPLE_SOURCE,
            &PathBuf::from("test.jnc"),
            BuildTarget::Client,
            &cache,
            true,
        );

        b.iter(|| {
            let result = jounce_compiler::cache::compile_source_cached(
                black_box(SAMPLE_SOURCE),
                &PathBuf::from("test.jnc"),
                BuildTarget::Client,
                &cache,
                true,
            );
            assert!(result.is_ok());
        });
    });

    group.finish();
}

fn bench_cache_hit_rate(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_hit_rate");

    group.bench_function("10_sequential_compiles", |b| {
        b.iter(|| {
            let cache = Arc::new(CompilationCache::default());

            for _ in 0..10 {
                let _ = jounce_compiler::cache::compile_source_cached(
                    black_box(SAMPLE_SOURCE),
                    &PathBuf::from("test.jnc"),
                    BuildTarget::Client,
                    &cache,
                    true,
                );
            }

            let stats = cache.stats();
            assert!(stats.hit_rate() > 0.8, "Cache hit rate should be > 80%");
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_compilation_no_cache,
    bench_compilation_with_cache,
    bench_cache_hit_rate
);
criterion_main!(benches);
