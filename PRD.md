# DEPSIM — Product Requirements Document (Source of Truth)

## 1. Product Definition

DEPSIM is a **standalone Rust command-line tool** that performs **registry-only, side-effect-free npm dependency resolution preflight**.

It reads a `package.json` file, fetches only npm registry metadata, and predicts:

```text
selected package versions
dependency edges
resolution warnings
unsupported dependency specs
version conflicts
```

It does **not** install packages.

It does **not** download tarballs.

It does **not** execute scripts.

It does **not** require Node.js or npm in production.

The MVP is not a full npm replacement. It is a resolution preflight tool for a clearly defined subset of npm behavior.

---

## 2. Non-Negotiable Constraints

These constraints are mandatory.

### 2.1 Registry Metadata Only

DEPSIM may only fetch npm registry metadata.

Allowed:

```text
https://registry.npmjs.org/{package-name}
```

Not allowed in production resolution:

```text
tarball downloads
.tgz downloads
package content downloads
install script execution
```

### 2.2 No Node.js/npm Runtime

The shipped CLI must work without:

```text
Node.js
npm
npx
JavaScript runtime
```

Node/npm may only be used in the validation harness, not in the production tool.

### 2.3 No Filesystem Mutation

DEPSIM must not modify:

```text
package.json
package-lock.json
node_modules
user project files
```

It may only write to a cache directory if caching is explicitly enabled.

### 2.4 Machine-Readable Output

DEPSIM must provide JSON output suitable for CI.

It must also provide human-readable terminal output.

### 2.5 Explicit Scope

DEPSIM must not claim full npm compatibility.

It must document:

```text
supported features
unsupported features
known limitations
```

---

## 3. MVP Goal

The MVP must answer this question:

```text
Given a package.json containing registry dependencies,
can DEPSIM predict the resolved dependency graph or report conflicts
without installing anything?
```

The MVP must be small, testable, and honest.

It must not attempt to reproduce all npm behavior.

---

## 4. MVP Scope

### 4.1 Supported Input

The MVP must support:

```text
package.json dependencies
package.json optionalDependencies
package.json devDependencies when --dev is passed
```

It must support registry dependency specs such as:

```text
^1.2.3
~1.2.3
1.2.3
1.x
1.2.x
*
>=1.0.0 <2.0.0
1.x || 2.x
latest
npm:real-package@^1.2.3
```

It must support scoped packages:

```text
@scope/package
```

---

### 4.2 Unsupported Input

The MVP must explicitly classify and report unsupported dependency specs.

These must not crash the tool.

Unsupported specs include:

```text
git dependencies
git+ dependencies
http dependencies
https dependencies
file dependencies
link dependencies
workspace dependencies
GitHub shorthand like user/repo
```

Example output:

```json
{
  "unsupported": [
    {
      "name": "my-lib",
      "spec": "file:../my-lib",
      "reason": "file dependency is outside registry-only MVP scope"
    }
  ]
}
```

Unsupported specs should produce warnings, not silent omission.

---

### 4.3 Not Included in MVP

The MVP must not include these as required features:

```text
peerDependencies resolution
package-lock.json validation
npm overrides
exact node_modules placement
hoisting prediction
workspace resolution
git dependency resolution
file dependency resolution
platform-specific filtering
engine filtering
full npm error reproduction
dependency visualization
SAT solver completion
```

A solver interface may exist, but SAT/backtracking is not required for MVP completion.

---

## 5. Resolution Model

### 5.1 MVP Resolver Strategy

The MVP must use a **greedy resolver**.

For each package:

1. Collect all constraints seen so far.
2. Fetch the packument from the registry.
3. Choose the highest version satisfying all constraints.
4. Expand that version's dependencies.
5. Repeat until the graph is resolved or a conflict is found.

The MVP may use a simplified global model:

```text
one selected version per package name
```

This limitation must be documented.

The MVP must not claim exact npm hoisting or nesting behavior.

---

### 5.2 Version Selection Rules

For each dependency range:

#### Exact Version

```text
1.2.3
```

must mean exactly `1.2.3`.

#### Dist-Tag

```text
latest
next
beta
```

must resolve using the packument `dist-tags` field.

Example:

```json
{
  "dist-tags": {
    "latest": "2.4.1"
  }
}
```

If the dist-tag does not exist, report a conflict or unsupported range.

#### Highest Satisfying Version

For normal ranges, DEPSIM should choose the highest available version satisfying constraints.

Deprecated versions may still be selected, but DEPSIM should emit a warning if a deprecated version is selected.

---

### 5.3 Optional Dependency Behavior

Optional dependencies should be resolved if possible.

If an optional dependency cannot be resolved, DEPSIM should emit a warning and continue, unless the same package is also required as a non-optional dependency.

Example:

```json
{
  "status": "resolved_with_warnings",
  "warnings": [
    {
      "code": "OPTIONAL_DEPENDENCY_UNRESOLVED",
      "package": "fsevents",
      "message": "optional dependency could not be resolved"
    }
  ]
}
```

---

### 5.4 devDependencies Behavior

`devDependencies` must only be included when the user passes:

```bash
--dev
```

Without `--dev`, dev dependencies must be ignored.

---

### 5.5 peerDependencies Behavior

The MVP does not resolve peer dependencies.

The packument model may store `peerDependencies`, but they must not affect MVP resolution.

This limitation must be documented.

---

### 5.6 Cycles

DEPSIM must handle circular dependencies safely.

It must track already-expanded nodes:

```text
package@version
```

It must not infinitely recurse.

---

### 5.7 Conflict Handling

The MVP must detect at least these conflict cases:

```text
no version satisfies constraints
selected version conflicts with a later constraint
invalid version range
missing registry package
dist-tag not found
```

Conflicts must be machine-readable.

Example:

```json
{
  "status": "conflict",
  "conflicts": [
    {
      "code": "NO_SATISFYING_VERSION",
      "package": "foo",
      "required_by": "bar@1.0.0",
      "range": "^2.0.0",
      "constraints": [
        {
          "source": "root",
          "range": "^1.0.0"
        },
        {
          "source": "bar@1.0.0",
          "range": "^2.0.0"
        }
      ],
      "message": "no version of foo satisfies all constraints"
    }
  ]
}
```

---

## 6. npm Semver Expectations

The MVP must use an npm-compatible semver module or a clearly isolated wrapper.

The module should be named something like:

```text
npm_semver
```

The rest of the codebase must not directly depend on low-level semver details.

---

### 6.1 Required Semver Cases

The MVP must handle at least:

```text
*
empty string
1.2.3
^1.2.3
~1.2.3
1.x
1.2.x
>=1.2.3
<=1.2.3
>1.2.3
<1.2.3
>=1.0.0 <2.0.0
1.x || 2.x
1.0.0 - 2.0.0
latest
```

Bare versions must be exact.

This is important:

```text
1.2.3
```

must not be treated as:

```text
^1.2.3
```

---

### 6.2 Prerelease Handling

For the MVP, prerelease behavior can be conservative.

Recommended rule:

```text
Prerelease versions are only selected if the constraint explicitly references a prerelease or exact prerelease version.
```

If full npm prerelease semantics are too complex for MVP, document the limitation.

---

## 7. Registry Behavior

### 7.1 Registry Endpoint

Default registry:

```text
https://registry.npmjs.org
```

Package metadata endpoint:

```text
https://registry.npmjs.org/{encoded-package-name}
```

Scoped packages must be URL-encoded.

Example:

```text
@scope/package
```

becomes:

```text
%40scope%2Fpackage
```

---

### 7.2 Accept Header

DEPSIM should request abbreviated metadata:

```http
Accept: application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*
```

This reduces payload size.

---

### 7.3 Retries

DEPSIM must retry transient failures.

Retry on:

```text
network timeout
HTTP 429
HTTP 5xx
```

Do not retry indefinitely.

Recommended:

```text
3 attempts
exponential backoff
```

---

### 7.4 Timeouts

DEPSIM must set HTTP timeouts.

Recommended defaults:

```text
connect timeout: 10 seconds
request timeout: 30 seconds
```

These should be configurable.

---

### 7.5 Caching

The MVP must include at least an in-memory cache.

Each packument should be fetched once per run.

A disk cache is not required for MVP, but the code must be structured so disk caching can be added later.

Recommended abstraction:

```text
PackumentSource trait
MemoryCachePackumentSource
HttpPackumentSource
```

---

### 7.6 Concurrency

DEPSIM must fetch packuments concurrently where practical.

It must limit concurrency.

Recommended default:

```text
16 concurrent requests
```

Configurable with:

```bash
--concurrency
```

---

## 8. CLI Contract

The MVP should provide a simple CLI.

Example:

```bash
depsim resolve --manifest package.json
```

Recommended options:

```text
-f, --manifest <path>        Path to package.json
    --dev                    Include devDependencies
    --json                   Output JSON
    --no-color               Disable colored output
    --concurrency <n>        Maximum concurrent registry requests
    --timeout <seconds>      HTTP timeout
    --registry <url>         Registry base URL
    --verbose                More detailed output
```

---

### 8.1 Exit Codes

DEPSIM must use stable exit codes.

Recommended:

```text
0 = resolved successfully, possibly with warnings
1 = dependency conflict
2 = tool error
```

Examples of tool errors:

```text
package.json not found
invalid package.json
network failure after retries
internal error
```

---

## 9. Canonical Output Format

DEPSIM must output a canonical JSON result.

The JSON output must be stable and machine-readable.

Do not output logs, spinners, or ANSI colors in JSON mode.

---

### 9.1 Recommended JSON Schema

```json
{
  "schema_version": 1,
  "tool": {
    "name": "depsim",
    "version": "0.1.0"
  },
  "input": {
    "manifest": "package.json",
    "include_dev": false,
    "registry": "https://registry.npmjs.org"
  },
  "status": "resolved",
  "root": {
    "name": "my-app",
    "version": "1.0.0"
  },
  "nodes": [
    {
      "id": "foo@2.4.1",
      "name": "foo",
      "version": "2.4.1",
      "deprecated": false
    }
  ],
  "edges": [
    {
      "from": "root",
      "to": "foo@2.4.1",
      "range": "^2.0.0",
      "kind": "dependency"
    }
  ],
  "selected": {
    "foo": "2.4.1"
  },
  "conflicts": [],
  "warnings": [],
  "unsupported": [],
  "metrics": {
    "duration_ms": 830,
    "packuments_requested": 12,
    "packuments_cache_hits": 0,
    "packages_resolved": 12
  }
}
```

---

### 9.2 Status Values

Use these statuses:

```text
resolved
resolved_with_warnings
conflict
error
```

`resolved_with_warnings` should be used when resolution succeeds but unsupported specs or optional dependency failures were encountered.

---

### 9.3 Edge Kinds

For MVP, edge kinds can include:

```text
dependency
optional_dependency
dev_dependency
```

Peer dependencies are not resolved in MVP.

---

## 10. Human-Readable Output

Human output should be readable and CI-friendly.

Example success output:

```text
Resolved 12 packages in 830 ms

  foo@2.4.1
  bar@1.8.0
  baz@3.1.4
```

Example warning output:

```text
warning: skipped git dependency: some-package
warning: optional dependency unresolved: fsevents
```

Example conflict output:

```text
Conflict detected

  package: foo
  required by: bar@1.0.0
  constraint: ^2.0.0
  existing constraints:
    root requires ^1.0.0
    bar@1.0.0 requires ^2.0.0
```

Colors may be used for terminal output, but colors must be disabled with `--no-color` and must never appear in JSON output.

---

## 11. Expected Codebase Structure

The codebase should be modular from the start.

A good MVP structure is:

```text
depsim/
  Cargo.toml
  src/
    main.rs
    cli.rs
    error.rs
    model.rs
    registry.rs
    cache.rs
    npm_semver.rs
    resolver/
      mod.rs
      greedy.rs
      solver.rs
    output.rs
  tests/
    fixtures/
    integration/
  docs/
    SUPPORT.md
    OUTPUT.md
    LIMITATIONS.md
```

Alternatively, use a Cargo workspace if the project is expected to grow:

```text
crates/
  depsim-core/
  depsim-cli/
  depsim-harness/
```

For MVP, a single crate with clean modules is acceptable.

---

### 11.1 Core Separation

The resolver core should not depend on:

```text
clap
indicatif
colored
terminal output
stdout printing
```

The CLI layer should handle:

```text
argument parsing
spinners
colors
printing
exit codes
```

The core should return structured results.

This separation is important.

---

### 11.2 Suggested Module Responsibilities

#### `cli.rs`

Handles command-line parsing and invocation.

#### `model.rs`

Defines:

```text
PackageJson
Packument
VersionMetadata
ResolutionResult
ResolvedNode
DependencyEdge
Conflict
Warning
UnsupportedDependency
```

#### `registry.rs`

Handles HTTP fetching of packuments.

#### `cache.rs`

Handles in-memory caching and future disk cache abstraction.

#### `npm_semver.rs`

Handles npm range parsing and matching.

#### `resolver/greedy.rs`

Implements greedy resolution.

#### `resolver/solver.rs`

Optional placeholder for future SAT/backtracking solver.

For MVP, this can contain only a trait or stub.

#### `output.rs`

Converts resolution result into human or JSON output.

---

## 12. Rust Implementation Expectations

Recommended dependencies:

```toml
anyhow
thiserror
tokio
reqwest
serde
serde_json
clap
colored
indicatif
semver
urlencoding
```

Use `reqwest` with `rustls` to avoid OpenSSL issues.

Example:

```toml
reqwest = {
  version = "0.11",
  default-features = false,
  features = ["json", "rustls-tls", "gzip", "brotli"]
}
```

Use async Rust with `tokio`.

Use `serde` for JSON deserialization.

Use typed errors where practical.

Avoid panics in normal operation.

---

## 13. Testing Expectations

The MVP must include tests.

Do not rely only on live network tests.

---

### 13.1 Unit Tests

Required for:

```text
npm semver parsing
alias parsing
unsupported spec detection
version selection
conflict detection
canonical output serialization
```

Important semver tests:

```text
exact version
caret range
tilde range
wildcard range
OR range
dist-tag
bare version exactness
```

---

### 13.2 Fixture Tests

Create local fixture files.

Example:

```text
tests/fixtures/basic/package.json
tests/fixtures/conflict/package.json
tests/fixtures/alias/package.json
tests/fixtures/optional/package.json
tests/fixtures/unsupported-git/package.json
```

Each fixture should have expected output or assertions.

---

### 13.3 Mock Registry Tests

The MVP should avoid live network calls in tests.

Use one of:

```text
local fixture server
mock HTTP responses
recorded packument JSON files
```

Tests must be deterministic.

---

### 13.4 No-Tarball Test

Add a test asserting that DEPSIM never requests tarball URLs.

For example, all requested URLs must:

```text
start with the configured registry base URL
not end with .tgz
not contain /-/
```

This is an important safety test.

---

## 14. Validation Direction

The MVP does not need a full npm conformance corpus.

However, the architecture must allow future differential validation.

Future validation will compare:

```text
DEPSIM canonical output
npm canonical output
```

Therefore, DEPSIM output must be canonical and stable.

Do not design output around npm error strings.

Design it around observable outcomes:

```text
selected versions
edges
conflicts
warnings
unsupported specs
```

---

## 15. Performance Expectations

Performance claims must be qualified.

The MVP should target:

```text
small graphs: fast
typical graphs: seconds
large graphs: best effort
```

Recommended benchmark targets:

```text
50 packages: under 1 second with warm cache or mock registry
500 packages: under 2 seconds with warm cache or mock registry
```

Live registry performance may vary.

DEPSIM should output metrics:

```text
duration_ms
packuments_requested
packuments_cache_hits
packages_resolved
```

Do not make unconditional claims like:

```text
always under 2 seconds
```

---

## 16. Security and Safety Expectations

DEPSIM must be safe to run on untrusted `package.json` files.

It must not execute anything.

It must not download package contents.

It should enforce limits.

Recommended limits:

```text
maximum packages resolved: 5000
maximum dependency depth: 100
maximum packument response size: 50 MB
```

These can be configurable later.

---

## 17. Documentation Requirements

The MVP must include:

### 17.1 README

Explain:

```text
what DEPSIM is
what DEPSIM is not
how to install
how to run
how to interpret output
```

### 17.2 SUPPORT.md

List supported and unsupported features.

Example:

| Feature | MVP support |
|---|---:|
| dependencies | yes |
| optionalDependencies | yes |
| devDependencies | yes, with flag |
| peerDependencies | no |
| package-lock.json | no |
| overrides | no |
| git dependencies | unsupported |
| file dependencies | unsupported |
| workspace dependencies | unsupported |
| exact node_modules placement | no |
| JSON output | yes |
| conflict detection | yes |

### 17.3 LIMITATIONS.md

List known limitations.

Example:

```text
DEPSIM uses a simplified global version model.
It does not yet model npm hoisting or nested duplicates.
It does not resolve peer dependencies.
It does not validate package-lock.json.
```

---

## 18. Milestones

### Milestone 1: Core Parsing

Deliver:

```text
package.json parsing
dependency spec normalization
unsupported spec detection
alias parsing
```

Acceptance:

```text
Can read dependencies, devDependencies, optionalDependencies.
Can detect git/file/workspace specs.
Can parse npm aliases.
```

---

### Milestone 2: Registry Fetching

Deliver:

```text
packument fetching
in-memory caching
retries
timeouts
concurrency limiting
```

Acceptance:

```text
Can fetch scoped and unscoped packages.
Does not fetch tarballs.
Handles 404 and transient errors cleanly.
```

---

### Milestone 3: Semver Module

Deliver:

```text
npm_semver module
range parsing
range matching
dist-tag handling
```

Acceptance:

```text
Unit tests pass for required semver cases.
Bare versions are exact.
OR ranges work.
Dist-tags resolve.
```

---

### Milestone 4: Greedy Resolver

Deliver:

```text
greedy resolution
dependency expansion
cycle protection
conflict detection
```

Acceptance:

```text
Can resolve simple real dependency graphs.
Can detect impossible constraints.
Does not infinite loop.
```

---

### Milestone 5: CLI and Output

Deliver:

```text
CLI flags
human output
JSON output
exit codes
warnings
conflicts
unsupported reporting
```

Acceptance:

```text
JSON output is valid and stable.
Exit codes are correct.
No spinner or color output in JSON mode.
```

---

### Milestone 6: Tests and Docs

Deliver:

```text
fixture tests
mock registry tests
README
SUPPORT.md
LIMITATIONS.md
```

Acceptance:

```text
cargo test passes.
No live network required for core tests.
Docs accurately describe scope.
```

---

## 19. Definition of Done for MVP

The MVP is done when all of the following are true.

### Functional

```text
DEPSIM can read a package.json.
DEPSIM can resolve registry dependencies.
DEPSIM can resolve optional dependencies.
DEPSIM can include devDependencies with --dev.
DEPSIM can detect unsupported dependency specs.
DEPSIM can report conflicts.
DEPSIM can output JSON.
DEPSIM can output human-readable results.
DEPSIM uses correct exit codes.
```

### Technical

```text
No tarballs are downloaded.
No Node.js/npm is required at runtime.
No project files are modified.
HTTP fetches are cached in memory.
HTTP fetches are concurrency-limited.
Scoped packages work.
Circular dependencies do not crash the tool.
```

### Quality

```text
cargo fmt passes.
cargo clippy passes with no serious warnings.
cargo test passes.
Core tests do not require live network.
JSON output is deterministic.
```

### Documentation

```text
README exists.
SUPPORT.md exists.
LIMITATIONS.md exists.
Output schema is documented.
Exit codes are documented.
```

---

## 20. What is Explicitly Not Required for MVP

The following are not required for MVP completion:

```text
SAT solver
peer dependency resolution
lockfile validation
overrides
workspace support
git dependency resolution
file dependency resolution
platform filtering
engine filtering
exact node_modules placement
dependency visualization
full npm conformance corpus
mutation testing
disk cache
remote API
```

These may be planned for later versions.

They must not block the MVP.

---

## 21. Final MVP Statement

The MVP is:

```text
A Rust CLI that reads package.json, fetches only npm registry metadata,
greedily resolves a documented subset of npm registry dependencies,
reports conflicts and unsupported specs, outputs JSON for CI,
and never installs packages or downloads tarballs.
```

That is the target.

Anything beyond that should be treated as a separate milestone.