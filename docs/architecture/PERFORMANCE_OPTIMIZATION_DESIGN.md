# Performance Optimization Design - Phase 9 Sprint 1

**Date**: 2025-10-23
**Status**: 🚀 In Progress
**Goal**: Achieve 10x faster incremental builds through caching and parallelization

---

## Current State Analysis

### Compilation Pipeline (No Caching)
```
Source File → Lexer → Parser → AST
                                ↓
                         Module Loader
                                ↓
                         Semantic Analysis
                                ↓
                          Type Checking
                                ↓
                         Borrow Checking
                                ↓
                         Code Generation → WASM
                                ↓
                          Optimization
                                ↓
                         CSS Generation → styles.css
```

**Problem**: Every compilation runs ALL phases, even for unchanged files.

### Measured Bottlenecks (from profiler.rs data)
1. **Parsing**: ~30% of compile time
2. **Type Checking**: ~25% of compile time
3. **Semantic Analysis**: ~20% of compile time
4. **Code Generation**: ~15% of compile time
5. **Other**: ~10%

**Current Performance**: 96,292 compilations/sec (full rebuild)
**Target**: 500,000+ compilations/sec (incremental)

---

## Optimization 1: Incremental Compilation with AST Caching

### Design

**Cache Structure**:
```rust
struct CompilationCache {
    // File hash → Cached compilation artifacts
    ast_cache: HashMap<u64, CachedAST>,
    type_cache: HashMap<u64, TypeInfo>,
    analysis_cache: HashMap<u64, AnalysisResult>,

    // Dependency graph
    dependencies: DependencyGraph,

    // Metadata
    cache_dir: PathBuf,
    last_modified: HashMap<PathBuf, SystemTime>,
}

struct CachedAST {
    ast: Program,
    source_hash: u64,
    timestamp: SystemTime,
    dependencies: Vec<PathBuf>,
}
```

**Cache Invalidation Strategy**:
1. Hash file contents using `xxhash` (fast, non-cryptographic)
2. Check if hash matches cached entry
3. If match → reuse cached AST + analysis
4. If no match → recompile ONLY this file
5. Propagate changes through dependency graph

**Storage**:
- Location: `.jounce/cache/` directory
- Format: MessagePack (faster than JSON, smaller than bincode)
- Files: `<file-hash>.ast`, `<file-hash>.types`, `<file-hash>.analysis`

### Implementation Plan

**Files to Create**:
- `src/cache/mod.rs` - Cache manager
- `src/cache/ast_cache.rs` - AST caching
- `src/cache/type_cache.rs` - Type info caching
- `src/cache/dependency_graph.rs` - Dependency tracking

**Files to Modify**:
- `src/lib.rs` - Add cache parameter to compile functions
- `src/main.rs` - Initialize cache in CLI
- `Cargo.toml` - Add dependencies: `xxhash-rust`, `rmp-serde`

### Expected Improvement
- **Unchanged files**: 0ms (instant cache hit)
- **Changed file only**: 50-100ms (1 file recompile)
- **Changed file + dependents**: Proportional to dependency tree

**Target**: 10x faster for typical incremental builds

---

## Optimization 2: Parallel Compilation

### Design

**Parallelization Strategy**:
```rust
use rayon::prelude::*;

// Build dependency graph
let dep_graph = DependencyGraph::from_sources(&files);

// Topologically sort for correct order
let levels = dep_graph.topological_levels();

// Compile each level in parallel
for level in levels {
    level.par_iter().for_each(|file| {
        compile_file(file, &cache);
    });
}
```

**Thread Pool**:
- Use `rayon` for work-stealing parallelism
- Thread count: `num_cpus::get()` (auto-detect)
- Shared cache with `Arc<Mutex<Cache>>`

**Parallel Opportunities**:
1. **Independent modules**: Compile in parallel
2. **Type checking**: Parallel for independent functions
3. **CSS generation**: Parallel utility class generation
4. **WASM optimization**: Parallel function optimization

### Implementation Plan

**Files to Create**:
- `src/parallel/mod.rs` - Parallel compilation coordinator
- `src/parallel/thread_pool.rs` - Thread pool management

**Files to Modify**:
- `src/lib.rs` - Add `compile_parallel()` method
- `src/watcher.rs` - Use parallel compilation
- `Cargo.toml` - Add `rayon = "1.8"`, `num_cpus = "1.16"`

### Expected Improvement
- **4-core CPU**: 3-4x faster
- **8-core CPU**: 5-7x faster
- **16-core CPU**: 8-12x faster

---

## Optimization 3: Build Caching

### Design

**Cache Layers**:
```
Layer 1: Memory Cache (CompilationCache in RAM)
Layer 2: Disk Cache (.jounce/cache/ directory)
Layer 3: Distributed Cache (optional, for CI/CD)
```

**Smart Invalidation**:
```rust
struct DependencyGraph {
    // File → files it depends on
    dependencies: HashMap<PathBuf, HashSet<PathBuf>>,

    // File → files that depend on it
    dependents: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl DependencyGraph {
    fn invalidate_file(&mut self, file: &Path) -> Vec<PathBuf> {
        let mut to_rebuild = vec![file.to_path_buf()];

        // Transitively invalidate all dependents
        let mut queue = vec![file.to_path_buf()];
        while let Some(f) = queue.pop() {
            if let Some(deps) = self.dependents.get(&f) {
                for dep in deps {
                    if !to_rebuild.contains(dep) {
                        to_rebuild.push(dep.clone());
                        queue.push(dep.clone());
                    }
                }
            }
        }

        to_rebuild
    }
}
```

**Cache Eviction**:
- Max size: 1GB (configurable)
- Strategy: LRU (Least Recently Used)
- Auto-cleanup: Remove entries older than 7 days

### Implementation Plan

**Files to Create**:
- `src/cache/disk_cache.rs` - Persistent disk storage
- `src/cache/lru.rs` - LRU eviction policy

**Files to Modify**:
- `src/cache/mod.rs` - Add disk cache integration

### Expected Improvement
- **Cache hit rate**: 80-90% for typical workflows
- **Cold start**: Same as before (no cache)
- **Warm start**: 50-100x faster (full cache hit)

---

## Optimization 4: Bundle Optimization & Tree-Shaking

### Design

**Dead Code Elimination**:
```rust
struct TreeShaker {
    // Entry points (exported functions, @client/@server)
    entry_points: HashSet<String>,

    // Call graph
    call_graph: HashMap<String, HashSet<String>>,

    // Reachability
    reachable: HashSet<String>,
}

impl TreeShaker {
    fn analyze(&mut self, ast: &Program) {
        // Build call graph
        for stmt in &ast.statements {
            self.analyze_statement(stmt);
        }

        // Mark reachable from entry points
        for entry in &self.entry_points {
            self.mark_reachable(entry);
        }
    }

    fn remove_dead_code(&self, ast: &mut Program) {
        ast.statements.retain(|stmt| {
            self.is_reachable(stmt)
        });
    }
}
```

**Code Splitting**:
- Separate bundles for client/server
- Lazy loading for routes
- Chunk splitting for large dependencies

### Implementation Plan

**Files to Create**:
- `src/optimizer/tree_shaker.rs` - Dead code elimination
- `src/optimizer/code_splitter.rs` - Bundle splitting

**Files to Modify**:
- `src/lib.rs` - Add tree-shaking pass
- `src/wasm_optimizer.rs` - Integrate tree-shaking

### Expected Improvement
- **Bundle size**: 30-50% smaller
- **Load time**: 20-40% faster
- **Parse time**: Proportional to size reduction

---

## Optimization 5: Benchmarking Suite

### Design

**Benchmark Categories**:
1. **Micro-benchmarks**: Individual compiler phases
2. **Integration benchmarks**: Full compilation
3. **Real-world benchmarks**: Example projects
4. **Regression tests**: Performance over time

**Metrics to Track**:
```rust
struct CompilationMetrics {
    // Time
    total_time: Duration,
    lex_time: Duration,
    parse_time: Duration,
    analysis_time: Duration,
    codegen_time: Duration,

    // Throughput
    lines_per_second: f64,
    tokens_per_second: f64,

    // Memory
    peak_memory: usize,
    cache_hit_rate: f64,

    // Output
    wasm_size: usize,
    css_size: usize,
}
```

### Implementation Plan

**Files to Create**:
- `benches/incremental_compilation.rs` - Incremental build benchmark
- `benches/parallel_compilation.rs` - Parallel build benchmark
- `benches/cache_performance.rs` - Cache hit rate benchmark
- `benches/real_world.rs` - Full project benchmarks

**Files to Modify**:
- `Cargo.toml` - Add benchmark entries

### Expected Deliverables
- Automated performance tracking
- Regression detection
- Optimization validation
- Public performance dashboard (future)

---

## Implementation Timeline

### Phase 1: Incremental Compilation (~3h)
- [x] Design cache structure
- [ ] Implement AST caching
- [ ] Implement hash-based invalidation
- [ ] Add disk persistence
- [ ] Test with real projects

### Phase 2: Parallel Compilation (~2h)
- [ ] Set up Rayon thread pool
- [ ] Build dependency graph
- [ ] Implement parallel compilation
- [ ] Add thread safety
- [ ] Performance testing

### Phase 3: Build Caching (~1.5h)
- [ ] Implement disk cache
- [ ] Add LRU eviction
- [ ] Smart invalidation
- [ ] Cache statistics

### Phase 4: Tree-Shaking (~1h)
- [ ] Build call graph analyzer
- [ ] Implement reachability analysis
- [ ] Dead code elimination
- [ ] Integration testing

### Phase 5: Benchmarking (~0.5h)
- [ ] Create benchmark suite
- [ ] Run baseline measurements
- [ ] Measure all optimizations
- [ ] Document results

**Total Estimated Time**: ~8 hours

---

## Success Criteria

✅ **Performance**:
- Incremental builds: 10x faster
- Parallel builds: 4-8x faster (on multi-core)
- Cache hit rate: >80%
- Bundle size: 30-50% smaller

✅ **Quality**:
- Zero correctness regressions
- All 558 tests passing
- Memory usage <2x baseline
- Cache eviction working

✅ **Developer Experience**:
- Transparent caching (no config needed)
- Clear cache stats in output
- Fast cold starts (<2s for medium projects)
- Instant hot reloads (<100ms)

---

## Future Optimizations (Phase 10+)

- Distributed caching (CI/CD)
- Predictive compilation
- Streaming compilation
- WASM-based compiler (dogfooding)
- GPU-accelerated parsing (experimental)
