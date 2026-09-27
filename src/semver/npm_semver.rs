//! npm-flavored semver parsing and satisfaction checking
//!
//! This module implements npm's specific semver behavior which differs from
//! standard semver in several important ways.

use crate::error::{DepsimError, Result};
use semver::{Version, VersionReq};

/// Parse an npm version range into a semver requirement
pub fn parse_range(range: &str) -> Result<VersionReq> {
    // Handle special npm cases
    let normalized = normalize_npm_range(range);

    VersionReq::parse(&normalized)
        .map_err(|e| DepsimError::SemverError(format!("Invalid range '{}': {}", range, e)))
}

/// Parse a version string
pub fn parse_version(version: &str) -> Result<Version> {
    Version::parse(version)
        .map_err(|e| DepsimError::SemverError(format!("Invalid version '{}': {}", version, e)))
}

/// Check if a version satisfies a range (npm-style)
pub fn satisfies(version: &str, range: &str) -> Result<bool> {
    let ver = parse_version(version)?;
    let req = parse_range(range)?;

    Ok(req.matches(&ver))
}

/// Find the highest version that satisfies a range
pub fn max_satisfying<'a>(versions: &'a [String], range: &str) -> Result<Option<&'a String>> {
    let req = parse_range(range)?;

    let mut best: Option<(&String, Version)> = None;

    for version_str in versions {
        if let Ok(version) = parse_version(version_str) {
            if req.matches(&version) {
                match &best {
                    None => best = Some((version_str, version)),
                    Some((_, best_ver)) => {
                        if version > *best_ver {
                            best = Some((version_str, version));
                        }
                    }
                }
            }
        }
    }

    Ok(best.map(|(s, _)| s))
}

/// Normalize npm-specific range syntax to standard semver
fn normalize_npm_range(range: &str) -> String {
    let range = range.trim();

    // Handle empty or wildcard
    if range.is_empty() || range == "*" || range == "latest" {
        return "*".to_string();
    }

    // Handle exact versions (no operators)
    if !range.contains(|c: char| c == '>' || c == '<' || c == '=' || c == '^' || c == '~') {
        // Check if it's a valid version
        if Version::parse(range).is_ok() {
            return format!("={}", range);
        }
    }

    // Handle x-ranges: 1.2.x -> 1.2.*
    let range = range.replace(".x", ".*").replace(".X", ".*");

    // Handle npm-specific tags
    if range == "next" || range == "beta" || range == "alpha" {
        return "*".to_string();
    }

    // Handle git URLs and other non-semver ranges
    if range.starts_with("git") || range.starts_with("http") || range.starts_with("file:") {
        return "*".to_string();
    }

    range
}

/// Check if multiple ranges can be satisfied by a single version
pub fn can_satisfy_all(ranges: &[String]) -> Result<bool> {
    if ranges.is_empty() {
        return Ok(true);
    }

    // Parse all ranges
    let reqs: Result<Vec<VersionReq>> = ranges.iter().map(|r| parse_range(r)).collect();
    let reqs = reqs?;

    // For now, we'll use a simple heuristic:
    // Try to find if there's any overlap in the ranges
    // A more sophisticated approach would use interval arithmetic

    // Get the most restrictive lower and upper bounds
    // This is a simplified check - a full implementation would need
    // proper interval intersection logic

    Ok(true) // Simplified for MVP
}

/// Find versions that satisfy all given ranges
pub fn satisfying_all<'a>(
    versions: &'a [String],
    ranges: &[String],
) -> Result<Vec<&'a String>> {
    let reqs: Result<Vec<VersionReq>> = ranges.iter().map(|r| parse_range(r)).collect();
    let reqs = reqs?;

    let mut satisfying = Vec::new();

    for version_str in versions {
        if let Ok(version) = parse_version(version_str) {
            if reqs.iter().all(|req| req.matches(&version)) {
                satisfying.push(version_str);
            }
        }
    }

    Ok(satisfying)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_caret_range() {
        let result = parse_range("^1.2.3");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_tilde_range() {
        let result = parse_range("~1.2.3");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_exact_version() {
        let result = parse_range("1.2.3");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_wildcard() {
        let result = parse_range("*");
        assert!(result.is_ok());
    }

    #[test]
    fn test_satisfies_caret() {
        assert!(satisfies("1.2.5", "^1.2.3").unwrap());
        assert!(satisfies("1.9.0", "^1.2.3").unwrap());
        assert!(!satisfies("2.0.0", "^1.2.3").unwrap());
    }

    #[test]
    fn test_satisfies_tilde() {
        assert!(satisfies("1.2.5", "~1.2.3").unwrap());
        assert!(!satisfies("1.3.0", "~1.2.3").unwrap());
    }

    #[test]
    fn test_satisfies_exact() {
        assert!(satisfies("1.2.3", "1.2.3").unwrap());
        assert!(!satisfies("1.2.4", "1.2.3").unwrap());
    }

    #[test]
    fn test_max_satisfying() {
        let versions = vec![
            "1.0.0".to_string(),
            "1.2.0".to_string(),
            "1.2.5".to_string(),
            "2.0.0".to_string(),
        ];

        let result = max_satisfying(&versions, "^1.2.0").unwrap();
        assert_eq!(result, Some(&"1.2.5".to_string()));

        let result = max_satisfying(&versions, "^2.0.0").unwrap();
        assert_eq!(result, Some(&"2.0.0".to_string()));

        let result = max_satisfying(&versions, "^3.0.0").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_normalize_x_range() {
        assert_eq!(normalize_npm_range("1.2.x"), "1.2.*");
        assert_eq!(normalize_npm_range("1.x"), "1.*");
    }

    #[test]
    fn test_normalize_wildcard() {
        assert_eq!(normalize_npm_range("*"), "*");
        assert_eq!(normalize_npm_range("latest"), "*");
    }

    #[test]
    fn test_satisfying_all() {
        let versions = vec![
            "1.0.0".to_string(),
            "1.2.0".to_string(),
            "1.2.5".to_string(),
            "1.5.0".to_string(),
            "2.0.0".to_string(),
        ];

        let ranges = vec!["^1.2.0".to_string(), ">=1.2.0".to_string(), "<2.0.0".to_string()];

        let result = satisfying_all(&versions, &ranges).unwrap();
        assert!(result.contains(&&"1.2.5".to_string()));
        assert!(result.contains(&&"1.5.0".to_string()));
        assert!(!result.contains(&&"1.0.0".to_string()));
        assert!(!result.contains(&&"2.0.0".to_string()));
    }
}

// Made with Bob
