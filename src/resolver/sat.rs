//! SAT-based conflict resolution using splr
//!
//! This module encodes dependency constraints as a Boolean satisfiability problem
//! and uses the splr SAT solver to find solutions when greedy resolution fails.

use crate::error::{DepsimError, Result};
use crate::resolver::types::{Conflict, ResolutionState, VersionConstraint};
use splr::*;
use std::collections::HashMap;

/// SAT-based resolver for handling complex dependency conflicts
pub struct SatResolver {
    /// Variable mapping: (package, version) -> variable ID
    var_map: HashMap<(String, String), i32>,
    /// Reverse mapping: variable ID -> (package, version)
    reverse_map: HashMap<i32, (String, String)>,
    /// Next available variable ID
    next_var: i32,
}

impl SatResolver {
    /// Create a new SAT resolver
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            reverse_map: HashMap::new(),
            next_var: 1,
        }
    }

    /// Get or create a variable for a (package, version) pair
    fn get_or_create_var(&mut self, package: &str, version: &str) -> i32 {
        let key = (package.to_string(), version.to_string());
        if let Some(&var) = self.var_map.get(&key) {
            var
        } else {
            let var = self.next_var;
            self.next_var += 1;
            self.var_map.insert(key.clone(), var);
            self.reverse_map.insert(var, key);
            var
        }
    }

    /// Attempt to resolve conflicts using SAT solving
    pub fn resolve_conflicts(
        &mut self,
        state: &ResolutionState,
        available_versions: &HashMap<String, Vec<String>>,
    ) -> Result<Option<HashMap<String, String>>> {
        let mut solver = Solver::default();

        // Encode constraints
        self.encode_constraints(&mut solver, state, available_versions)?;

        // Solve
        match solver.solve() {
            Ok(Certificate::SAT(model)) => {
                // Extract solution
                let solution = self.extract_solution(&model)?;
                Ok(Some(solution))
            }
            Ok(Certificate::UNSAT) => {
                // No solution exists
                Ok(None)
            }
            Err(e) => Err(DepsimError::SatError(format!("Solver error: {:?}", e))),
        }
    }

    /// Encode dependency constraints as SAT clauses
    fn encode_constraints(
        &mut self,
        solver: &mut Solver,
        state: &ResolutionState,
        available_versions: &HashMap<String, Vec<String>>,
    ) -> Result<()> {
        // For each package, exactly one version must be selected
        for (package, versions) in available_versions {
            let vars: Vec<i32> = versions
                .iter()
                .map(|v| self.get_or_create_var(package, v))
                .collect();

            // At least one version must be selected
            solver
                .add_clause(vars.clone())
                .map_err(|e| DepsimError::SatError(format!("Failed to add clause: {:?}", e)))?;

            // At most one version can be selected (pairwise exclusion)
            for i in 0..vars.len() {
                for j in (i + 1)..vars.len() {
                    solver
                        .add_clause(vec![-vars[i], -vars[j]])
                        .map_err(|e| {
                            DepsimError::SatError(format!("Failed to add clause: {:?}", e))
                        })?;
                }
            }
        }

        // Encode dependency constraints
        for (package, constraints) in &state.constraints {
            for constraint in constraints {
                self.encode_dependency_constraint(
                    solver,
                    package,
                    constraint,
                    available_versions,
                )?;
            }
        }

        Ok(())
    }

    /// Encode a single dependency constraint
    fn encode_dependency_constraint(
        &mut self,
        solver: &mut Solver,
        package: &str,
        constraint: &VersionConstraint,
        available_versions: &HashMap<String, Vec<String>>,
    ) -> Result<()> {
        // Get versions that satisfy the constraint
        let satisfying_versions = if let Some(versions) = available_versions.get(package) {
            versions
                .iter()
                .filter(|v| crate::semver::satisfies(v, &constraint.range).unwrap_or(false))
                .collect::<Vec<_>>()
        } else {
            return Ok(());
        };

        if satisfying_versions.is_empty() {
            // No satisfying version - add empty clause (UNSAT)
            solver
                .add_clause(vec![])
                .map_err(|e| DepsimError::SatError(format!("Failed to add clause: {:?}", e)))?;
            return Ok(());
        }

        // If the requesting package is selected, one of the satisfying versions must be selected
        let requesting_var = if let Some(versions) = available_versions.get(&constraint.requested_by)
        {
            // For simplicity, we'll use the first version of the requesting package
            // A full implementation would handle this more carefully
            if let Some(first_version) = versions.first() {
                Some(self.get_or_create_var(&constraint.requested_by, first_version))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(req_var) = requesting_var {
            // req_var => (v1 OR v2 OR ... OR vn)
            // Equivalent to: (NOT req_var) OR v1 OR v2 OR ... OR vn
            let mut clause = vec![-req_var];
            for version in satisfying_versions {
                clause.push(self.get_or_create_var(package, version));
            }
            solver
                .add_clause(clause)
                .map_err(|e| DepsimError::SatError(format!("Failed to add clause: {:?}", e)))?;
        }

        Ok(())
    }

    /// Extract solution from SAT model
    fn extract_solution(&self, model: &[i32]) -> Result<HashMap<String, String>> {
        let mut solution = HashMap::new();

        for &lit in model {
            if lit > 0 {
                // Positive literal means this variable is true
                if let Some((package, version)) = self.reverse_map.get(&lit) {
                    solution.insert(package.clone(), version.clone());
                }
            }
        }

        Ok(solution)
    }
}

impl Default for SatResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sat_resolver() {
        let resolver = SatResolver::new();
        assert_eq!(resolver.next_var, 1);
        assert!(resolver.var_map.is_empty());
    }

    #[test]
    fn test_get_or_create_var() {
        let mut resolver = SatResolver::new();
        let var1 = resolver.get_or_create_var("express", "4.18.2");
        let var2 = resolver.get_or_create_var("express", "4.18.2");
        let var3 = resolver.get_or_create_var("express", "4.18.1");

        assert_eq!(var1, var2);
        assert_ne!(var1, var3);
    }

    // More comprehensive tests would require setting up full constraint scenarios
}

// Made with Bob
