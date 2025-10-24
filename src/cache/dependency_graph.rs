// Dependency graph for smart cache invalidation
// Tracks which files depend on which other files

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Dependency graph for tracking file relationships
#[derive(Clone, Default)]
pub struct DependencyGraph {
    /// Forward edges: file → files it depends on
    dependencies: HashMap<PathBuf, HashSet<PathBuf>>,

    /// Reverse edges: file → files that depend on it
    dependents: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    /// Add a dependency relationship: `file` depends on `depends_on`
    pub fn add_dependency(&mut self, file: PathBuf, depends_on: PathBuf) {
        // Add to dependencies
        self.dependencies
            .entry(file.clone())
            .or_insert_with(HashSet::new)
            .insert(depends_on.clone());

        // Add to reverse map (dependents)
        self.dependents
            .entry(depends_on)
            .or_insert_with(HashSet::new)
            .insert(file);
    }

    /// Get all files that depend on the given file (directly or transitively)
    pub fn get_affected_files(&self, file: &std::path::Path) -> Vec<PathBuf> {
        let mut affected = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![file.to_path_buf()];

        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            // Add all files that depend on this one
            if let Some(deps) = self.dependents.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        queue.push(dep.clone());
                        affected.push(dep.clone());
                    }
                }
            }
        }

        affected
    }

    /// Get all files that a given file depends on
    pub fn get_dependencies(&self, file: &std::path::Path) -> Vec<PathBuf> {
        self.dependencies
            .get(file)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Remove a file from the dependency graph
    pub fn remove_file(&mut self, file: &std::path::Path) {
        // Remove from dependencies
        if let Some(deps) = self.dependencies.remove(file) {
            // Remove from reverse map
            for dep in deps {
                if let Some(dependents) = self.dependents.get_mut(&dep) {
                    dependents.remove(file);
                }
            }
        }

        // Remove from dependents
        if let Some(dependents) = self.dependents.remove(file) {
            // Remove from forward map
            for dependent in dependents {
                if let Some(deps) = self.dependencies.get_mut(&dependent) {
                    deps.remove(file);
                }
            }
        }
    }

    /// Get topological levels for parallel compilation
    /// Returns groups of files that can be compiled in parallel
    pub fn topological_levels(&self) -> Vec<Vec<PathBuf>> {
        let mut levels = Vec::new();
        let mut in_degree: HashMap<PathBuf, usize> = HashMap::new();
        let mut remaining: HashSet<PathBuf> = self.dependencies.keys().cloned().collect();

        // Calculate in-degrees
        for (file, deps) in &self.dependencies {
            in_degree.insert(file.clone(), deps.len());
        }

        // Process levels
        while !remaining.is_empty() {
            let mut current_level = Vec::new();

            // Find all files with in-degree 0
            for file in &remaining {
                if in_degree.get(file).copied().unwrap_or(0) == 0 {
                    current_level.push(file.clone());
                }
            }

            if current_level.is_empty() {
                // Cycle detected or isolated nodes
                break;
            }

            // Remove from remaining and update in-degrees
            for file in &current_level {
                remaining.remove(file);

                // Decrease in-degree of dependents
                if let Some(dependents) = self.dependents.get(file) {
                    for dependent in dependents {
                        if let Some(degree) = in_degree.get_mut(dependent) {
                            *degree = degree.saturating_sub(1);
                        }
                    }
                }
            }

            levels.push(current_level);
        }

        levels
    }

    /// Clear all dependencies
    pub fn clear(&mut self) {
        self.dependencies.clear();
        self.dependents.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_dependency() {
        let mut graph = DependencyGraph::new();

        let file_a = PathBuf::from("a.jnc");
        let file_b = PathBuf::from("b.jnc");

        graph.add_dependency(file_a.clone(), file_b.clone());

        let deps = graph.get_dependencies(&file_a);
        assert_eq!(deps.len(), 1);
        assert!(deps.contains(&file_b));
    }

    #[test]
    fn test_get_affected_files() {
        let mut graph = DependencyGraph::new();

        let file_a = PathBuf::from("a.jnc");
        let file_b = PathBuf::from("b.jnc");
        let file_c = PathBuf::from("c.jnc");

        // b depends on a, c depends on b
        graph.add_dependency(file_b.clone(), file_a.clone());
        graph.add_dependency(file_c.clone(), file_b.clone());

        // Changing a should affect b and c
        let affected = graph.get_affected_files(&file_a);
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&file_b));
        assert!(affected.contains(&file_c));
    }

    #[test]
    fn test_topological_levels() {
        let mut graph = DependencyGraph::new();

        let file_a = PathBuf::from("a.jnc");
        let file_b = PathBuf::from("b.jnc");
        let file_c = PathBuf::from("c.jnc");

        // b and c both depend on a
        graph.add_dependency(file_b.clone(), file_a.clone());
        graph.add_dependency(file_c.clone(), file_a.clone());

        let levels = graph.topological_levels();

        // First level should have just a (no dependencies)
        assert_eq!(levels[0].len(), 1);
        assert!(levels[0].contains(&file_a));

        // Second level should have b and c (can be compiled in parallel)
        assert_eq!(levels[1].len(), 2);
        assert!(levels[1].contains(&file_b));
        assert!(levels[1].contains(&file_c));
    }
}
