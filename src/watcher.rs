/// File watching and auto-recompilation for Jounce
///
/// This module provides file watching functionality that monitors .jnc files
/// and automatically recompiles them when changes are detected.

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant, SystemTime};

/// Configuration for the file watcher
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Path to watch (file or directory)
    pub path: PathBuf,
    /// Output directory for compiled files
    pub output_dir: PathBuf,
    /// Debounce delay in milliseconds (default: 150ms)
    pub debounce_ms: u64,
    /// Whether to clear console on recompile
    pub clear_console: bool,
    /// Whether to show verbose output
    pub verbose: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            output_dir: PathBuf::from("dist"),
            debounce_ms: 150,
            clear_console: false,
            verbose: false,
        }
    }
}

/// Statistics about compilation
#[derive(Debug, Clone, Default)]
pub struct CompileStats {
    /// Number of files compiled
    pub compiled: usize,
    /// Number of files cached
    pub cached: usize,
    /// Compilation duration in milliseconds
    pub duration_ms: u64,
    /// Whether compilation succeeded
    pub success: bool,
}

/// Simple file-based cache for incremental compilation
pub struct CompilationCache {
    /// Map of file path to (modification time, content hash)
    file_states: HashMap<PathBuf, (SystemTime, u64)>,
}

impl CompilationCache {
    /// Create a new compilation cache
    pub fn new() -> Self {
        Self {
            file_states: HashMap::new(),
        }
    }

    /// Check if a file has changed since last compilation
    pub fn has_changed(&mut self, path: &Path) -> bool {
        // Get current file metadata
        let Ok(metadata) = std::fs::metadata(path) else {
            return true; // File doesn't exist or can't be read, consider changed
        };

        let Ok(modified) = metadata.modified() else {
            return true; // Can't get modification time, consider changed
        };

        // Get cached state
        if let Some((cached_time, _cached_hash)) = self.file_states.get(path) {
            // Compare modification times
            if modified != *cached_time {
                // File modified, update cache
                self.file_states.insert(path.to_path_buf(), (modified, 0));
                return true;
            }
            false
        } else {
            // File not in cache, add it
            self.file_states.insert(path.to_path_buf(), (modified, 0));
            true
        }
    }

    /// Mark a file as compiled
    pub fn mark_compiled(&mut self, path: &Path) {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                self.file_states.insert(path.to_path_buf(), (modified, 0));
            }
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.file_states.clear();
    }
}

/// File watcher that monitors .jnc files and triggers recompilation
pub struct FileWatcher {
    config: WatchConfig,
    cache: CompilationCache,
    _watcher: RecommendedWatcher,
    receiver: Receiver<PathBuf>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(config: WatchConfig) -> Result<Self, String> {
        let (tx, rx) = channel();

        // Create a watcher that sends events through the channel
        let watcher = Self::create_watcher(tx, config.verbose)?;

        Ok(Self {
            config,
            cache: CompilationCache::new(),
            _watcher: watcher,
            receiver: rx,
        })
    }

    /// Create the underlying notify watcher
    fn create_watcher(
        tx: Sender<PathBuf>,
        verbose: bool,
    ) -> Result<RecommendedWatcher, String> {
        notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Only process modify and create events
                    match event.kind {
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            // Filter for .jnc files
                            for path in event.paths {
                                if path.extension().and_then(|s| s.to_str()) == Some("raven") {
                                    if verbose {
                                        println!("[watch] File changed: {}", path.display());
                                    }
                                    let _ = tx.send(path);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("Watch error: {:?}", e);
                }
            }
        })
        .map_err(|e| format!("Failed to create watcher: {}", e))
    }

    /// Start watching the configured path
    pub fn watch(&mut self) -> Result<(), String> {
        let path = &self.config.path;

        // Determine recursive mode based on path type
        let recursive_mode = if path.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        self._watcher
            .watch(path, recursive_mode)
            .map_err(|e| format!("Failed to watch {}: {}", path.display(), e))?;

        if self.config.verbose {
            println!(
                "[watch] Watching {} (recursive: {})",
                path.display(),
                matches!(recursive_mode, RecursiveMode::Recursive)
            );
        }

        Ok(())
    }

    /// Wait for the next file change event (with debouncing)
    ///
    /// This method implements debouncing: if multiple events arrive within
    /// the debounce window, it will only return once after the last event.
    pub fn wait_for_change(&self) -> Option<PathBuf> {
        // Wait for first event
        let first_path = self.receiver.recv().ok()?;
        let debounce_duration = Duration::from_millis(self.config.debounce_ms);
        let deadline = Instant::now() + debounce_duration;

        // Collect any additional events within debounce window
        let mut latest_path = first_path;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }

            match self.receiver.recv_timeout(remaining) {
                Ok(path) => {
                    // Got another event, update latest path
                    latest_path = path;
                }
                Err(_) => {
                    // Timeout or disconnected, debounce window expired
                    break;
                }
            }
        }

        Some(latest_path)
    }

    /// Get a reference to the compilation cache
    pub fn cache(&mut self) -> &mut CompilationCache {
        &mut self.cache
    }

    /// Get a reference to the watch configuration
    pub fn config(&self) -> &WatchConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert_eq!(config.debounce_ms, 150);
        assert!(!config.clear_console);
        assert!(!config.verbose);
    }

    #[test]
    fn test_compilation_cache_new() {
        let cache = CompilationCache::new();
        assert_eq!(cache.file_states.len(), 0);
    }

    #[test]
    fn test_compile_stats_default() {
        let stats = CompileStats::default();
        assert_eq!(stats.compiled, 0);
        assert_eq!(stats.cached, 0);
        assert_eq!(stats.duration_ms, 0);
        assert!(!stats.success);
    }
}
