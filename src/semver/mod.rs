//! Semver parsing and satisfaction checking with npm compatibility

pub mod npm_semver;

pub use npm_semver::{
    can_satisfy_all, max_satisfying, parse_range, parse_version, satisfies, satisfying_all,
};

// Made with Bob
