// AST caching module
// Stores parsed ASTs with metadata for fast retrieval

use std::path::PathBuf;
use std::time::SystemTime;

use crate::ast::Program;

/// Cached AST with metadata
/// Note: We don't serialize the AST itself (would require Serialize/Deserialize on all AST nodes)
/// Instead, we keep it in memory for the current session
#[derive(Clone)]
pub struct CachedAST {
    /// The parsed AST (in-memory only)
    pub ast: Program,

    /// Source file hash (for validation)
    pub source_hash: u64,

    /// File path
    pub file_path: PathBuf,

    /// Timestamp when cached
    pub timestamp: SystemTime,

    /// List of files this AST depends on
    pub dependencies: Vec<PathBuf>,
}

impl CachedAST {
    /// Create a new cached AST entry
    pub fn new(ast: Program, source_hash: u64, file_path: PathBuf) -> Self {
        Self {
            ast,
            source_hash,
            file_path,
            timestamp: SystemTime::now(),
            dependencies: Vec::new(),
        }
    }

    /// Check if this cache entry is still valid
    pub fn is_valid(&self, file_path: &std::path::Path, current_hash: u64) -> bool {
        // Check hash matches
        if self.source_hash != current_hash {
            return false;
        }

        // Check file path matches
        if self.file_path != file_path {
            return false;
        }

        // Check file still exists and hasn't been modified
        if let Ok(metadata) = std::fs::metadata(file_path) {
            if let Ok(modified) = metadata.modified() {
                // Cache is valid if file hasn't been modified since caching
                return modified <= self.timestamp;
            }
        }

        false
    }

    /// Add a dependency to this cached AST
    pub fn add_dependency(&mut self, dep_path: PathBuf) {
        if !self.dependencies.contains(&dep_path) {
            self.dependencies.push(dep_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_cached_ast_creation() {
        let ast = Program {
            statements: Vec::new(),
        };

        let cached = CachedAST::new(
            ast,
            12345,
            PathBuf::from("test.jnc"),
        );

        assert_eq!(cached.source_hash, 12345);
        assert_eq!(cached.file_path, PathBuf::from("test.jnc"));
        assert!(cached.dependencies.is_empty());
    }
}
