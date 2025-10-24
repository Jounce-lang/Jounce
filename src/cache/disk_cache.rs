// Disk-based persistent cache
// Stores compilation artifacts to disk for cross-session caching

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use rmp_serde::{Serializer, Deserializer};
use serde::{Serialize, Deserialize};

use super::ast_cache::CachedAST;

/// Disk cache for persistent storage
pub struct DiskCache {
    cache_dir: PathBuf,
}

impl DiskCache {
    /// Create a new disk cache
    pub fn new(cache_dir: PathBuf) -> io::Result<Self> {
        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir)?;

        Ok(Self { cache_dir })
    }

    /// Get cache file path for a given hash
    fn get_cache_path(&self, hash: u64) -> PathBuf {
        self.cache_dir.join(format!("{:x}.cache", hash))
    }

    /// Save AST to disk
    pub fn save_ast(&self, hash: u64, cached_ast: &CachedAST) -> io::Result<()> {
        let path = self.get_cache_path(hash);

        // Serialize to MessagePack
        let mut buf = Vec::new();
        cached_ast
            .serialize(&mut Serializer::new(&mut buf))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Write to file
        let mut file = fs::File::create(path)?;
        file.write_all(&buf)?;

        Ok(())
    }

    /// Load AST from disk
    pub fn load_ast(&self, hash: u64) -> io::Result<CachedAST> {
        let path = self.get_cache_path(hash);

        // Read file
        let mut file = fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        // Deserialize from MessagePack
        let mut de = Deserializer::new(&buf[..]);
        CachedAST::deserialize(&mut de)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Check if cache entry exists
    pub fn has_entry(&self, hash: u64) -> bool {
        self.get_cache_path(hash).exists()
    }

    /// Remove cache entry
    pub fn remove_entry(&self, hash: u64) -> io::Result<()> {
        let path = self.get_cache_path(hash);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Clear all cache entries
    pub fn clear(&self) -> io::Result<()> {
        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("cache") {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        Ok(())
    }

    /// Get cache size in bytes
    pub fn size(&self) -> io::Result<u64> {
        let mut total_size = 0u64;

        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("cache") {
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                    }
                }
            }
        }

        Ok(total_size)
    }

    /// Get number of cached entries
    pub fn entry_count(&self) -> io::Result<usize> {
        let mut count = 0;

        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("cache") {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::ast::Program;

    #[test]
    fn test_disk_cache() -> io::Result<()> {
        // Create temp directory for testing
        let temp_dir = env::temp_dir().join("jounce_cache_test");
        let cache = DiskCache::new(temp_dir.clone())?;

        // Create a test AST
        let ast = Program { statements: Vec::new() };
        let cached = CachedAST::new(ast, 12345, PathBuf::from("test.jnc"));

        // Save to disk
        cache.save_ast(12345, &cached)?;

        // Verify it exists
        assert!(cache.has_entry(12345));

        // Load from disk
        let loaded = cache.load_ast(12345)?;
        assert_eq!(loaded.source_hash, 12345);

        // Clean up
        cache.clear()?;
        fs::remove_dir_all(temp_dir)?;

        Ok(())
    }
}
