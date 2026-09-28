//! Dependency graph utilities

use crate::resolver::types::ResolutionNode;
use std::collections::{HashMap, HashSet};

/// Build a flat map of all packages in the resolution tree
pub fn flatten_tree(root: &ResolutionNode) -> HashMap<String, String> {
    let mut packages = HashMap::new();
    flatten_recursive(root, &mut packages);
    packages
}

fn flatten_recursive(node: &ResolutionNode, packages: &mut HashMap<String, String>) {
    packages.insert(node.package.clone(), node.version.clone());
    for child in &node.resolved_deps {
        flatten_recursive(child, packages);
    }
}

/// Get all unique packages in the tree
pub fn get_unique_packages(root: &ResolutionNode) -> HashSet<String> {
    let mut packages = HashSet::new();
    collect_packages(root, &mut packages);
    packages
}

fn collect_packages(node: &ResolutionNode, packages: &mut HashSet<String>) {
    packages.insert(node.package.clone());
    for child in &node.resolved_deps {
        collect_packages(child, packages);
    }
}

/// Calculate the maximum depth of the dependency tree
pub fn max_depth(root: &ResolutionNode) -> usize {
    let mut max = root.depth;
    for child in &node.resolved_deps {
        let child_max = max_depth(child);
        if child_max > max {
            max = child_max;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_flatten_single_node() {
        let node = ResolutionNode {
            package: "test".to_string(),
            version: "1.0.0".to_string(),
            resolved_deps: vec![],
            conflicts: vec![],
            depth: 0,
        };

        let flat = flatten_tree(&node);
        assert_eq!(flat.len(), 1);
        assert_eq!(flat.get("test"), Some(&"1.0.0".to_string()));
    }

    #[test]
    fn test_get_unique_packages() {
        let node = ResolutionNode {
            package: "test".to_string(),
            version: "1.0.0".to_string(),
            resolved_deps: vec![],
            conflicts: vec![],
            depth: 0,
        };

        let packages = get_unique_packages(&node);
        assert_eq!(packages.len(), 1);
        assert!(packages.contains("test"));
    }
}

// Made with Bob
