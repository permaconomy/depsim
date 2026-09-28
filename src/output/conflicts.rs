//! Conflict reporting and formatting

use crate::resolver::types::{Conflict, ConflictType, ResolutionNode};
use colored::*;

/// Format conflicts for display
pub fn format_conflicts(conflicts: &[Conflict]) -> String {
    if conflicts.is_empty() {
        return format!("{} No conflicts detected\n", "✓".green().bold());
    }

    let mut output = String::new();
    output.push_str(&format!(
        "{} {} detected:\n\n",
        "⚠".yellow().bold(),
        if conflicts.len() == 1 {
            "Dependency conflict".to_string()
        } else {
            format!("{} dependency conflicts", conflicts.len())
        }
    ));

    for (i, conflict) in conflicts.iter().enumerate() {
        output.push_str(&format_single_conflict(conflict, i + 1));
        output.push('\n');
    }

    output
}

/// Format a single conflict
fn format_single_conflict(conflict: &Conflict, number: usize) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "{}. {}: {}\n",
        number,
        "Conflict".red().bold(),
        conflict.package.cyan()
    ));

    // Show what requested this package
    output.push_str(&format!("  {}:\n", "Required by".bold()));
    for (requester, range) in &conflict.requested_by {
        output.push_str(&format!(
            "    - {} requires {}\n",
            requester.yellow(),
            range.cyan()
        ));
    }

    // Show conflict type
    match conflict.conflict_type {
        ConflictType::Unsatisfiable => {
            output.push_str(&format!(
                "\n  {} No version satisfies all constraints\n",
                "❌".red()
            ));
        }
        ConflictType::PeerDependency => {
            output.push_str(&format!(
                "\n  {} Peer dependency conflict\n",
                "❌".red()
            ));
        }
        ConflictType::VersionRange => {
            output.push_str(&format!(
                "\n  {} Version range conflict\n",
                "❌".red()
            ));
        }
    }

    // Show suggestions if available
    if !conflict.conflicting_versions.is_empty() {
        output.push_str(&format!("\n  {}:\n", "Conflicting versions".bold()));
        for version in &conflict.conflicting_versions {
            output.push_str(&format!("    - {}\n", version));
        }
    }

    output
}

/// Collect all conflicts from a resolution tree
pub fn collect_conflicts(root: &ResolutionNode) -> Vec<Conflict> {
    let mut conflicts = Vec::new();
    collect_conflicts_recursive(root, &mut conflicts);
    conflicts
}

fn collect_conflicts_recursive(node: &ResolutionNode, conflicts: &mut Vec<Conflict>) {
    conflicts.extend(node.conflicts.clone());
    for child in &node.resolved_deps {
        collect_conflicts_recursive(child, conflicts);
    }
}

/// Format conflicts with suggestions
pub fn format_conflicts_with_suggestions(conflicts: &[Conflict]) -> String {
    let mut output = format_conflicts(conflicts);

    if !conflicts.is_empty() {
        output.push_str(&format!("\n{}\n", "Suggestions:".bold()));
        output.push_str("  1. Check if package versions are compatible\n");
        output.push_str("  2. Update dependencies to use compatible version ranges\n");
        output.push_str("  3. Consider using resolutions/overrides in package.json\n");
    }

    output
}

/// Format a summary of conflicts
pub fn format_conflict_summary(conflicts: &[Conflict]) -> String {
    if conflicts.is_empty() {
        return format!("{} No conflicts", "✓".green());
    }

    let unsatisfiable = conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::Unsatisfiable)
        .count();
    let peer = conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::PeerDependency)
        .count();
    let version_range = conflicts
        .iter()
        .filter(|c| c.conflict_type == ConflictType::VersionRange)
        .count();

    format!(
        "{} {} conflicts ({} unsatisfiable, {} peer, {} version range)",
        "⚠".yellow(),
        conflicts.len(),
        unsatisfiable,
        peer,
        version_range
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_conflict() -> Conflict {
        Conflict {
            package: "test-package".to_string(),
            requested_by: vec![
                ("parent1".to_string(), "^1.0.0".to_string()),
                ("parent2".to_string(), "^2.0.0".to_string()),
            ],
            conflicting_versions: vec!["1.0.0".to_string(), "2.0.0".to_string()],
            conflict_type: ConflictType::Unsatisfiable,
        }
    }

    #[test]
    fn test_format_empty_conflicts() {
        let output = format_conflicts(&[]);
        assert!(output.contains("No conflicts"));
    }

    #[test]
    fn test_format_single_conflict() {
        let conflict = create_test_conflict();
        let output = format_conflicts(&[conflict]);
        assert!(output.contains("test-package"));
        assert!(output.contains("parent1"));
        assert!(output.contains("parent2"));
    }

    #[test]
    fn test_format_conflict_summary() {
        let conflict = create_test_conflict();
        let output = format_conflict_summary(&[conflict]);
        assert!(output.contains("1 conflicts"));
    }

    #[test]
    fn test_collect_conflicts_empty() {
        let node = ResolutionNode {
            package: "test".to_string(),
            version: "1.0.0".to_string(),
            resolved_deps: vec![],
            conflicts: vec![],
            depth: 0,
        };
        let conflicts = collect_conflicts(&node);
        assert_eq!(conflicts.len(), 0);
    }
}

// Made with Bob
