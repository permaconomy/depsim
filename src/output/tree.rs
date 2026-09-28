//! Tree visualization for dependency resolution

use crate::resolver::types::ResolutionNode;
use colored::*;

/// Format resolution tree as a visual tree structure
pub fn format_tree(root: &ResolutionNode) -> String {
    let mut output = String::new();
    output.push_str(&format!("📦 {}@{}\n", root.package.bold(), root.version));
    format_tree_recursive(root, &mut output, "", true);
    output
}

fn format_tree_recursive(node: &ResolutionNode, output: &mut String, prefix: &str, is_last: bool) {
    let children = &node.resolved_deps;

    for (i, child) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        let connector = if is_last_child { "└─" } else { "├─" };
        let child_prefix = if is_last_child { "  " } else { "│ " };

        // Format the node
        let node_str = if child.conflicts.is_empty() {
            format!("{}@{}", child.package, child.version)
        } else {
            format!(
                "{}@{} {}",
                child.package,
                child.version,
                "⚠".yellow()
            )
        };

        output.push_str(&format!("{}{}{}\n", prefix, connector, node_str));

        // Recursively format children
        let new_prefix = format!("{}{}", prefix, child_prefix);
        format_tree_recursive(child, output, &new_prefix, is_last_child);
    }
}

/// Format a compact tree (only show direct dependencies)
pub fn format_compact_tree(root: &ResolutionNode) -> String {
    let mut output = String::new();
    output.push_str(&format!("📦 {}@{}\n", root.package.bold(), root.version));

    for (i, child) in root.resolved_deps.iter().enumerate() {
        let is_last = i == root.resolved_deps.len() - 1;
        let connector = if is_last { "└─" } else { "├─" };

        let node_str = if child.conflicts.is_empty() {
            format!("{}@{}", child.package, child.version)
        } else {
            format!(
                "{}@{} {}",
                child.package,
                child.version,
                "⚠".yellow()
            )
        };

        output.push_str(&format!("{}{}\n", connector, node_str));
    }

    output
}

/// Format tree with statistics
pub fn format_tree_with_stats(root: &ResolutionNode, package_count: usize, time_ms: u64) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "{} Resolving dependencies for {}@{}...\n",
        "✓".green().bold(),
        root.package,
        root.version
    ));
    output.push_str(&format!(
        "  Resolved {} packages in {}ms\n\n",
        package_count.to_string().cyan(),
        time_ms.to_string().cyan()
    ));

    // Tree
    output.push_str(&format_tree(root));

    // Footer
    output.push_str(&format!(
        "\n{} Resolution complete: {} packages, {} conflicts\n",
        "✓".green().bold(),
        package_count,
        count_conflicts(root)
    ));

    output
}

/// Count total conflicts in tree
fn count_conflicts(node: &ResolutionNode) -> usize {
    let mut count = node.conflicts.len();
    for child in &node.resolved_deps {
        count += count_conflicts(child);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_node(name: &str, version: &str) -> ResolutionNode {
        ResolutionNode {
            package: name.to_string(),
            version: version.to_string(),
            resolved_deps: vec![],
            conflicts: vec![],
            depth: 0,
        }
    }

    #[test]
    fn test_format_single_node() {
        let node = create_test_node("test", "1.0.0");
        let output = format_tree(&node);
        assert!(output.contains("test"));
        assert!(output.contains("1.0.0"));
    }

    #[test]
    fn test_format_with_children() {
        let child = Arc::new(create_test_node("child", "2.0.0"));
        let mut node = create_test_node("parent", "1.0.0");
        node.resolved_deps.push(child);

        let output = format_tree(&node);
        assert!(output.contains("parent"));
        assert!(output.contains("child"));
    }

    #[test]
    fn test_count_conflicts() {
        let node = create_test_node("test", "1.0.0");
        assert_eq!(count_conflicts(&node), 0);
    }
}

// Made with Bob
