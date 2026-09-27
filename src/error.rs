//! Error types for DEPSIM

use thiserror::Error;

/// Main error type for DEPSIM
#[derive(Error, Debug)]
pub enum DepsimError {
    /// Error reading or parsing package.json
    #[error("Failed to read package.json: {0}")]
    PackageJsonError(String),

    /// Error fetching from npm registry
    #[error("Registry error: {0}")]
    RegistryError(String),

    /// Network/HTTP error
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON parsing error
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Semver parsing error
    #[error("Invalid semver: {0}")]
    SemverError(String),

    /// Resolution error
    #[error("Resolution failed: {0}")]
    ResolutionError(String),

    /// Timeout error
    #[error("Operation timed out: {0}")]
    TimeoutError(String),

    /// SAT solver error
    #[error("SAT solver error: {0}")]
    SatError(String),

    /// Package not found
    #[error("Package not found: {0}")]
    PackageNotFound(String),

    /// Version not found
    #[error("Version {1} not found for package {0}")]
    VersionNotFound(String, String),

    /// Unsatisfiable constraints
    #[error("Unsatisfiable constraints for package {0}")]
    UnsatisfiableConstraints(String),
}

/// Result type alias for DEPSIM operations
pub type Result<T> = std::result::Result<T, DepsimError>;

// Made with Bob
