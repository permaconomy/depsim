# Contributing to DEPSIM

Thank you for your interest in contributing to DEPSIM! This document provides guidelines and information for contributors.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Priority Areas](#priority-areas)

---

## Code of Conduct

Be respectful, constructive, and professional. We're all here to build something useful.

---

## Getting Started

1. **Read the documentation:**
   - [PRD.md](PRD.md) - Product requirements and scope
   - [CODEBASE_ASSESSMENT.md](CODEBASE_ASSESSMENT.md) - Current status and known issues
   - [DEPSIM_ARCHITECTURE.md](DEPSIM_ARCHITECTURE.md) - Technical architecture

2. **Check existing issues:**
   - Browse [open issues](https://github.com/permaconomy/depsim/issues)
   - Look for issues labeled `good first issue` or `help wanted`

3. **Discuss major changes:**
   - Open an issue before starting work on significant features
   - Get feedback on your approach

---

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git

### Clone and Build

```bash
# Clone repository
git clone https://github.com/permaconomy/depsim.git
cd depsim

# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run -- --help
```

See [SETUP_GUIDE.md](SETUP_GUIDE.md) for detailed setup instructions.

---

## Project Structure

```
depsim/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library root
│   ├── cli.rs            # CLI argument parsing
│   ├── error.rs          # Error types
│   ├── registry/         # npm registry client
│   │   ├── client.rs     # HTTP client
│   │   ├── cache.rs      # Response caching
│   │   └── mod.rs
│   ├── resolver/         # Dependency resolution
│   │   ├── types.rs      # Core types
│   │   ├── greedy.rs     # Greedy resolver
│   │   ├── sat.rs        # SAT solver (future)
│   │   ├── graph.rs      # Dependency graph
│   │   └── mod.rs
│   ├── semver/           # Semver handling
│   │   ├── npm_semver.rs # npm-compatible semver
│   │   └── mod.rs
│   └── output/           # Output formatting
│       ├── tree.rs       # Tree visualization
│       ├── conflicts.rs  # Conflict reporting
│       └── mod.rs
├── tests/                # Integration tests
├── benches/              # Benchmarks
├── demo/                 # Web demo
└── .github/workflows/    # CI/CD
```

---

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 2. Make Your Changes

- Follow the [coding standards](#coding-standards)
- Write tests for new functionality
- Update documentation as needed
- Keep commits focused and atomic

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

---

## Testing

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```rust
// tests/integration_test.rs
use depsim::*;

#[test]
fn test_full_resolution() {
    // Test complete workflow
}
```

### Benchmarks

Place benchmarks in `benches/`:

```rust
// benches/resolution_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_resolution(c: &mut Criterion) {
    c.bench_function("resolve 500 packages", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });
}

criterion_group!(benches, benchmark_resolution);
criterion_main!(benches);
```

---

## Submitting Changes

### 1. Commit Your Changes

```bash
git add .
git commit -m "feat: add support for npm aliases"
```

**Commit message format:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

### 2. Push to GitHub

```bash
git push origin feature/your-feature-name
```

### 3. Create Pull Request

1. Go to https://github.com/permaconomy/depsim/pulls
2. Click "New Pull Request"
3. Select your branch
4. Fill in the PR template:
   - **Description:** What does this PR do?
   - **Related Issue:** Link to issue (if applicable)
   - **Testing:** How was this tested?
   - **Checklist:** Complete the checklist

### 4. Address Review Feedback

- Respond to comments
- Make requested changes
- Push updates to the same branch

---

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write idiomatic Rust code

### Code Quality

- **Clear naming:** Use descriptive variable and function names
- **Documentation:** Add doc comments for public APIs
- **Error handling:** Use `Result` and proper error types
- **No unwrap:** Avoid `.unwrap()` in production code
- **Tests:** Write tests for new functionality

### Example

```rust
/// Resolves dependencies for a package.json file.
///
/// # Arguments
///
/// * `manifest_path` - Path to package.json
/// * `include_dev` - Whether to include devDependencies
///
/// # Returns
///
/// Returns a `Result` containing the resolved dependency graph or an error.
///
/// # Errors
///
/// Returns an error if:
/// - The manifest file cannot be read
/// - The JSON is invalid
/// - Registry requests fail
pub async fn resolve_dependencies(
    manifest_path: &Path,
    include_dev: bool,
) -> Result<DependencyGraph, Error> {
    // Implementation
}
```

---

## Priority Areas

See [CODEBASE_ASSESSMENT.md](CODEBASE_ASSESSMENT.md) for detailed priorities.

### P0 - Blocking MVP (High Priority)

1. Fix CLI flags (remove `--peer`, add `--manifest`, `--json`, `--concurrency`)
2. Implement canonical JSON schema
3. Add exit codes (0, 1, 2)
4. Detect unsupported dependencies (git, file, workspace)
5. Fix bare version = exact in semver
6. Add dist-tag resolution (latest, next, beta)

### P1 - Critical (Medium Priority)

7. Add retry logic with exponential backoff
8. Add URL encoding for scoped packages
9. Add Accept header for abbreviated metadata
10. Fix concurrency default (50 → 16)
11. Handle optional dependencies
12. Support npm aliases

### P2 - Important (Lower Priority)

13. Add deprecated package warnings
14. Add safety limits (max packages, max depth)
15. Add no-tarball verification test
16. Create SUPPORT.md, LIMITATIONS.md, OUTPUT.md

---

## Questions?

- **General questions:** Open a [discussion](https://github.com/permaconomy/depsim/discussions)
- **Bug reports:** Open an [issue](https://github.com/permaconomy/depsim/issues)
- **Feature requests:** Open an [issue](https://github.com/permaconomy/depsim/issues) with `enhancement` label

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to DEPSIM! 🚀**