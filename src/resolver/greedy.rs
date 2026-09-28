//! Greedy-first dependency resolution algorithm

use crate::error::{DepsimError, Result};
use crate::registry::RegistryService;
use crate::resolver::types::{
    Conflict, ConflictType, PackageJson, ResolutionNode, ResolutionState, VersionConstraint,
};
use crate::semver::{max_satisfying, satisfying_all};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Greedy resolver that attempts to resolve dependencies by always
/// selecting the highest version that satisfies constraints
pub struct GreedyResolver {
    registry: Arc<RegistryService>,
    state: ResolutionState,
    visited: HashSet<String>,
}

impl GreedyResolver {
    /// Create a new greedy resolver
    pub fn new(registry: Arc<RegistryService>) -> Self {
        Self {
            registry,
            state: ResolutionState::new(),
            visited: HashSet::new(),
        }
    }

    /// Resolve dependencies from a package.json
    pub async fn resolve(&mut self, package_json: &PackageJson) -> Result<Arc<ResolutionNode>> {
        // Add root constraints
        for (name, range) in &package_json.dependencies {
            self.state.add_constraint(
                name.clone(),
                VersionConstraint {
                    requested_by: package_json.name.clone(),
                    range: range.clone(),
                    is_peer: false,
                    is_optional: false,
                },
            );
        }

        // Resolve root dependencies
        let mut resolved_deps = Vec::new();
        for (name, range) in &package_json.dependencies {
            match self.resolve_package(name, range, 1).await {
                Ok(node) => resolved_deps.push(node),
                Err(e) => {
                    // Record conflict
                    self.state.conflicts.push(Conflict {
                        package: name.clone(),
                        requested_by: vec![(package_json.name.clone(), range.clone())],
                        conflicting_versions: vec![],
                        conflict_type: ConflictType::Unsatisfiable,
                    });
                    return Err(e);
                }
            }
        }

        // Create root node
        Ok(Arc::new(ResolutionNode {
            package: package_json.name.clone(),
            version: package_json.version.clone(),
            resolved_deps,
            conflicts: self.state.conflicts.clone(),
            depth: 0,
        }))
    }

    /// Resolve a single package
    async fn resolve_package(
        &mut self,
        package_name: &str,
        version_range: &str,
        depth: usize,
    ) -> Result<Arc<ResolutionNode>> {
        // Check if already resolved
        if let Some(version) = self.state.get_version(package_name) {
            // Verify it still satisfies the new constraint
            if crate::semver::satisfies(version, version_range)? {
                // Return existing resolution (simplified - should return actual node)
                return Ok(Arc::new(ResolutionNode {
                    package: package_name.to_string(),
                    version: version.clone(),
                    resolved_deps: vec![],
                    conflicts: vec![],
                    depth,
                }));
            } else {
                // Conflict detected
                return Err(DepsimError::UnsatisfiableConstraints(
                    package_name.to_string(),
                ));
            }
        }

        // Prevent infinite loops
        if self.visited.contains(package_name) {
            return Err(DepsimError::ResolutionError(format!(
                "Circular dependency detected: {}",
                package_name
            )));
        }
        self.visited.insert(package_name.to_string());

        // Fetch packument
        let packument = self.registry.fetch_packument(package_name).await?;

        // Get all available versions
        let available_versions: Vec<String> = packument.versions.keys().cloned().collect();

        // Get all constraints for this package
        let constraints = self.state.constraints.get(package_name);
        let ranges: Vec<String> = if let Some(constraints) = constraints {
            constraints.iter().map(|c| c.range.clone()).collect()
        } else {
            vec![version_range.to_string()]
        };

        // Find versions that satisfy all constraints
        let satisfying = satisfying_all(&available_versions, &ranges)?;

        if satisfying.is_empty() {
            return Err(DepsimError::UnsatisfiableConstraints(
                package_name.to_string(),
            ));
        }

        // Select the highest satisfying version (greedy choice)
        let selected_version = max_satisfying(&available_versions, version_range)?
            .ok_or_else(|| DepsimError::UnsatisfiableConstraints(package_name.to_string()))?;

        // Record selection
        self.state
            .select_version(package_name.to_string(), selected_version.clone());

        // Get metadata for selected version
        let metadata = packument
            .versions
            .get(selected_version)
            .ok_or_else(|| {
                DepsimError::VersionNotFound(package_name.to_string(), selected_version.clone())
            })?;

        // Recursively resolve dependencies
        let mut resolved_deps = Vec::new();
        for (dep_name, dep_range) in &metadata.dependencies {
            // Add constraint
            self.state.add_constraint(
                dep_name.clone(),
                VersionConstraint {
                    requested_by: package_name.to_string(),
                    range: dep_range.clone(),
                    is_peer: false,
                    is_optional: false,
                },
            );

            // Resolve dependency
            match self.resolve_package(dep_name, dep_range, depth + 1).await {
                Ok(node) => resolved_deps.push(node),
                Err(e) => {
                    // For greedy resolution, we fail fast on conflicts
                    return Err(e);
                }
            }
        }

        // Remove from visited set
        self.visited.remove(package_name);

        Ok(Arc::new(ResolutionNode {
            package: package_name.to_string(),
            version: selected_version.clone(),
            resolved_deps,
            conflicts: vec![],
            depth,
        }))
    }

    /// Get the current resolution state
    pub fn state(&self) -> &ResolutionState {
        &self.state
    }

    /// Get detected conflicts
    pub fn conflicts(&self) -> &[Conflict] {
        &self.state.conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::types::PackageMetadata;
    use std::collections::HashMap;

    fn create_test_package_json() -> PackageJson {
        let mut dependencies = HashMap::new();
        dependencies.insert("express".to_string(), "^4.0.0".to_string());

        PackageJson {
            name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            dependencies,
            dev_dependencies: HashMap::new(),
            peer_dependencies: HashMap::new(),
            optional_dependencies: HashMap::new(),
        }
    }

    #[test]
    fn test_create_resolver() {
        let registry = Arc::new(RegistryService::new().unwrap());
        let resolver = GreedyResolver::new(registry);
        assert_eq!(resolver.state.selections.len(), 0);
    }

    // Integration tests would go here with mocked registry
}

// Made with Bob
