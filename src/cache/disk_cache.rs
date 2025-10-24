// Disk-based persistent cache
// Stores cache metadata to disk for cross-session tracking

use std::fs;
use std::path::PathBuf;
use std::io;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Metadata for cached files
#[derive(Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub hash: u64,
    pub timestamp: u64,
}

/// Disk cache for persistent storage
/// Note: Currently stores only metadata, not full ASTs
/// Full AST serialization would require Serialize/Deserialize on all AST types
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

    /// Get cache metadata file path
    fn get_metadata_path(&self) -> PathBuf {
        self.cache_dir.join("metadata.json")
    }

    /// Save cache metadata
    pub fn save_metadata(&self, metadata: &HashMap<PathBuf, CacheMetadata>) -> io::Result<()> {
        let path = self.get_metadata_path();
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load cache metadata
    pub fn load_metadata(&self) -> io::Result<HashMap<PathBuf, CacheMetadata>> {
        let path = self.get_metadata_path();
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Clear all cache data
    pub fn clear(&self) -> io::Result<()> {
        let path = self.get_metadata_path();
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
