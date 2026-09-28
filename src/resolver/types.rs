//! Core data structures for dependency resolution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a package's metadata from the npm registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Production dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    /// Peer dependencies
    #[serde(default)]
    pub peer_dependencies: HashMap<String, String>,
    /// Optional dependencies
    #[serde(default)]
    pub optional_dependencies: HashMap<String, String>,
    /// Development dependencies (not used in resolution)
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
}

/// Represents a packument (package document) from npm registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packument {
    /// Package name
    pub name: String,
    /// All available versions
    pub versions: HashMap<String, PackageMetadata>,
    /// Latest version tag
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
}

/// A node in the resolved dependency tree
#[derive(Debug, Clone)]
pub struct ResolutionNode {
    /// Package name
    pub package: String,
    /// Resolved version
    pub version: String,
    /// Resolved dependencies (children in the tree)
    pub resolved_deps: Vec<Arc<ResolutionNode>>,
    /// Any conflicts detected for this node
    pub conflicts: Vec<Conflict>,
    /// Depth in the dependency tree
    pub depth: usize,
}

/// Represents a dependency conflict
#[derive(Debug, Clone)]
pub struct Conflict {
    /// Package name with conflict
    pub package: String,
    /// List of (parent package, version range) that requested this package
    pub requested_by: Vec<(String, String)>,
    /// Conflicting version requirements
    pub conflicting_versions: Vec<String>,
    /// Conflict type
    pub conflict_type: ConflictType,
}

/// Types of conflicts that can occur
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    /// No version satisfies all constraints
    Unsatisfiable,
    /// Peer dependency conflict
    PeerDependency,
    /// Version range conflict
    VersionRange,
}

/// Resolution state tracking
#[derive(Debug, Clone)]
pub struct ResolutionState {
    /// Map of package name to selected version
    pub selections: HashMap<String, String>,
    /// Map of package name to all requested version ranges
    pub constraints: HashMap<String, Vec<VersionConstraint>>,
    /// Detected conflicts
    pub conflicts: Vec<Conflict>,
}

impl ResolutionState {
    /// Create a new empty resolution state
    pub fn new() -> Self {
        Self {
            selections: HashMap::new(),
            constraints: HashMap::new(),
            conflicts: Vec::new(),
        }
    }

    /// Add a version constraint for a package
    pub fn add_constraint(&mut self, package: String, constraint: VersionConstraint) {
        self.constraints
            .entry(package)
            .or_insert_with(Vec::new)
            .push(constraint);
    }

    /// Select a version for a package
    pub fn select_version(&mut self, package: String, version: String) {
        self.selections.insert(package, version);
    }

    /// Check if a package has been resolved
    pub fn is_resolved(&self, package: &str) -> bool {
        self.selections.contains_key(package)
    }

    /// Get the selected version for a package
    pub fn get_version(&self, package: &str) -> Option<&String> {
        self.selections.get(package)
    }
}

impl Default for ResolutionState {
    fn default() -> Self {
        Self::new()
    }
}

/// A version constraint from a dependency
#[derive(Debug, Clone)]
pub struct VersionConstraint {
    /// Package that requested this constraint
    pub requested_by: String,
    /// Version range (e.g., "^1.0.0", ">=2.0.0 <3.0.0")
    pub range: String,
    /// Whether this is a peer dependency
    pub is_peer: bool,
    /// Whether this is optional
    pub is_optional: bool,
}

/// Input package.json structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Production dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    /// Development dependencies
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
    /// Peer dependencies
    #[serde(default)]
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: HashMap<String, String>,
    /// Optional dependencies
    #[serde(default)]
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: HashMap<String, String>,
}

/// Resolution result
#[derive(Debug)]
pub struct ResolutionResult {
    /// Root resolution node
    pub root: Arc<ResolutionNode>,
    /// Total number of packages resolved
    pub package_count: usize,
    /// Total number of conflicts
    pub conflict_count: usize,
    /// Time taken to resolve (in milliseconds)
    pub resolution_time_ms: u64,
    /// Whether resolution was successful
    pub success: bool,
}

impl ResolutionResult {
    /// Create a successful resolution result
    pub fn success(root: Arc<ResolutionNode>, package_count: usize, time_ms: u64) -> Self {
        Self {
            root,
            package_count,
            conflict_count: 0,
            resolution_time_ms: time_ms,
            success: true,
        }
    }

    /// Create a failed resolution result with conflicts
    pub fn failure(
        root: Arc<ResolutionNode>,
        package_count: usize,
        conflict_count: usize,
        time_ms: u64,
    ) -> Self {
        Self {
            root,
            package_count,
            conflict_count,
            resolution_time_ms: time_ms,
            success: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution_state_new() {
        let state = ResolutionState::new();
        assert!(state.selections.is_empty());
        assert!(state.constraints.is_empty());
        assert!(state.conflicts.is_empty());
    }

    #[test]
    fn test_resolution_state_add_constraint() {
        let mut state = ResolutionState::new();
        let constraint = VersionConstraint {
            requested_by: "root".to_string(),
            range: "^1.0.0".to_string(),
            is_peer: false,
            is_optional: false,
        };
        state.add_constraint("express".to_string(), constraint);
        assert_eq!(state.constraints.len(), 1);
        assert_eq!(state.constraints.get("express").unwrap().len(), 1);
    }

    #[test]
    fn test_resolution_state_select_version() {
        let mut state = ResolutionState::new();
        state.select_version("express".to_string(), "4.18.2".to_string());
        assert!(state.is_resolved("express"));
        assert_eq!(state.get_version("express"), Some(&"4.18.2".to_string()));
    }
}

// Made with Bob
