//! Dependency resolution engine

pub mod greedy;
pub mod sat;
pub mod types;
pub mod graph;

pub use greedy::GreedyResolver;
pub use sat::SatResolver;
pub use types::{
    Conflict, ConflictType, PackageJson, PackageMetadata, Packument, ResolutionNode,
    ResolutionResult, ResolutionState, VersionConstraint,
};

use crate::error::Result;
use crate::registry::RegistryService;
use std::sync::Arc;
use std::time::Instant;

/// Main resolver that combines greedy and SAT-based approaches
pub struct Resolver {
    registry: Arc<RegistryService>,
}

impl Resolver {
    /// Create a new resolver
    pub fn new(registry: Arc<RegistryService>) -> Self {
        Self { registry }
    }

    /// Resolve dependencies from a package.json
    pub async fn resolve(&self, package_json: &PackageJson) -> Result<ResolutionResult> {
        let start = Instant::now();

        // Try greedy resolution first
        let mut greedy = GreedyResolver::new(Arc::clone(&self.registry));
        match greedy.resolve(package_json).await {
            Ok(root) => {
                let elapsed = start.elapsed().as_millis() as u64;
                let package_count = count_packages(&root);

                Ok(ResolutionResult::success(root, package_count, elapsed))
            }
            Err(e) => {
                // Greedy failed - could try SAT solver here
                // For MVP, we'll just return the error
                let elapsed = start.elapsed().as_millis() as u64;
                let conflicts = greedy.conflicts();

                // Create a partial result with conflicts
                let root = Arc::new(ResolutionNode {
                    package: package_json.name.clone(),
                    version: package_json.version.clone(),
                    resolved_deps: vec![],
                    conflicts: conflicts.to_vec(),
                    depth: 0,
                });

                Ok(ResolutionResult::failure(
                    root,
                    0,
                    conflicts.len(),
                    elapsed,
                ))
            }
        }
    }
}

/// Count total packages in resolution tree
fn count_packages(node: &ResolutionNode) -> usize {
    let mut count = 1;
    for child in &node.resolved_deps {
        count += count_packages(child);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_resolver() {
        let registry = Arc::new(RegistryService::new().unwrap());
        let resolver = Resolver::new(registry);
        // Just test creation
    }
}

// Made with Bob
