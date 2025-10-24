// Compilation cache system for incremental builds
// Phase 9 Sprint 1 - Performance Optimization

pub mod ast_cache;
pub mod dependency_graph;
pub mod disk_cache;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use xxhash_rust::xxh64::xxh64;
use dashmap::DashMap;

use crate::ast::Program;
use crate::errors::CompileError;

/// Fast hash computation for cache keys
pub fn compute_hash(data: &[u8]) -> u64 {
    xxh64(data, 0)
}

/// Main compilation cache manager
pub struct CompilationCache {
    /// AST cache (file hash → cached AST)
    ast_cache: DashMap<u64, ast_cache::CachedAST>,

    /// File metadata (path → last modified time + hash)
    file_metadata: DashMap<PathBuf, FileMetadata>,

    /// Dependency graph
    dependencies: Arc<Mutex<dependency_graph::DependencyGraph>>,

    /// Cache directory
    cache_dir: PathBuf,

    /// Cache statistics (using atomics for thread-safety)
    hits: Arc<AtomicUsize>,
    misses: Arc<AtomicUsize>,
    invalidations: Arc<AtomicUsize>,
}

#[derive(Clone)]
struct FileMetadata {
    last_modified: SystemTime,
    hash: u64,
}

#[derive(Clone)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub invalidations: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

impl CompilationCache {
    /// Create a new compilation cache
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            ast_cache: DashMap::new(),
            file_metadata: DashMap::new(),
            dependencies: Arc::new(Mutex::new(dependency_graph::DependencyGraph::new())),
            cache_dir,
            hits: Arc::new(AtomicUsize::new(0)),
            misses: Arc::new(AtomicUsize::new(0)),
            invalidations: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Get or compute AST for a file
    pub fn get_or_compile<F>(
        &self,
        file_path: &Path,
        source: &str,
        compiler_fn: F,
    ) -> Result<Program, CompileError>
    where
        F: FnOnce(&str) -> Result<Program, CompileError>,
    {
        // Compute source hash
        let hash = compute_hash(source.as_bytes());

        // Check if we have a valid cached entry
        if let Some(cached) = self.ast_cache.get(&hash) {
            // Verify the cache is still valid
            if cached.is_valid(file_path, hash) {
                self.hits.fetch_add(1, Ordering::Relaxed);
                return Ok(cached.ast.clone());
            }
        }

        // Cache miss - compile and store
        self.misses.fetch_add(1, Ordering::Relaxed);
        let ast = compiler_fn(source)?;

        // Store in cache
        let cached_ast = ast_cache::CachedAST::new(
            ast.clone(),
            hash,
            file_path.to_path_buf(),
        );
        self.ast_cache.insert(hash, cached_ast);

        // Update file metadata
        if let Ok(metadata) = std::fs::metadata(file_path) {
            if let Ok(modified) = metadata.modified() {
                self.file_metadata.insert(
                    file_path.to_path_buf(),
                    FileMetadata {
                        last_modified: modified,
                        hash,
                    },
                );
            }
        }

        Ok(ast)
    }

    /// Invalidate cache for a specific file
    pub fn invalidate_file(&self, file_path: &Path) {
        // Remove from metadata
        if let Some(meta) = self.file_metadata.remove(file_path) {
            // Remove from AST cache
            self.ast_cache.remove(&meta.1.hash);
            self.invalidations.fetch_add(1, Ordering::Relaxed);
        }

        // Invalidate dependents
        if let Ok(deps) = self.dependencies.lock() {
            let to_invalidate = deps.get_affected_files(file_path);
            for file in to_invalidate {
                if let Some(meta) = self.file_metadata.remove(&file) {
                    self.ast_cache.remove(&meta.1.hash);
                }
            }
        }
    }

    /// Clear all cache
    pub fn clear(&self) {
        self.ast_cache.clear();
        self.file_metadata.clear();
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
        self.invalidations.store(0, Ordering::Relaxed);
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            invalidations: self.invalidations.load(Ordering::Relaxed),
        }
    }

    /// Get cache size (number of entries)
    pub fn size(&self) -> usize {
        self.ast_cache.len()
    }

    /// Add dependency relationship
    pub fn add_dependency(&self, file: PathBuf, depends_on: PathBuf) {
        if let Ok(mut deps) = self.dependencies.lock() {
            deps.add_dependency(file, depends_on);
        }
    }
}

impl std::default::Default for CompilationCache {
    fn default() -> Self {
        let cache_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".jounce")
            .join("cache");

        Self::new(cache_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_computation() {
        let data1 = b"hello world";
        let data2 = b"hello world";
        let data3 = b"hello world!";

        assert_eq!(compute_hash(data1), compute_hash(data2));
        assert_ne!(compute_hash(data1), compute_hash(data3));
    }

    #[test]
    fn test_cache_stats() {
        let stats = CacheStats {
            hits: 80,
            misses: 20,
            invalidations: 5,
        };

        assert_eq!(stats.hit_rate(), 0.8);
    }
}
