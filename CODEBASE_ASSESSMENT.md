# DEPSIM Codebase Assessment Against PRD

## Assessment Date: 2026-09-27

This document assesses the current codebase against the PRD (source of truth) and identifies what must change.

---

## ✅ What Aligns with PRD

### 1. Project Structure
- ✅ Rust-based CLI tool
- ✅ Modular structure with separate concerns
- ✅ Cargo.toml with appropriate dependencies
- ✅ Source files organized by responsibility

### 2. Core Dependencies
- ✅ tokio for async
- ✅ reqwest for HTTP
- ✅ serde/serde_json for JSON
- ✅ clap for CLI
- ✅ colored/indicatif for terminal UI
- ✅ semver for version parsing

### 3. Registry Client
- ✅ Async HTTP fetching
- ✅ In-memory caching
- ✅ Concurrent request limiting
- ✅ No tarball downloads

### 4. Basic Resolution
- ✅ Greedy resolver skeleton exists
- ✅ Conflict detection structure
- ✅ Semver parsing module

---

## ❌ Critical Misalignments with PRD

### 1. **SAT Solver is NOT Required for MVP**

**Current State:**
- `splr` dependency in Cargo.toml
- `src/resolver/sat.rs` implemented
- SAT solver integrated into resolution flow

**PRD Says:**
> "A solver interface may exist, but SAT/backtracking is not required for MVP completion."
> "What is explicitly not required for MVP: SAT solver"

**Required Action:**
- ❌ Remove `splr` from dependencies
- ❌ Mark `sat.rs` as future milestone code
- ❌ Remove SAT solver from resolution flow
- ✅ Keep greedy resolver only
- ✅ Document SAT as post-MVP feature

---

### 2. **Peer Dependencies Should NOT Be Resolved**

**Current State:**
- CLI has `--peer` flag
- Code attempts to handle peer dependencies
- Resolution includes peer dependency logic

**PRD Says:**
> "The MVP does not resolve peer dependencies."
> "peerDependencies resolution" is explicitly NOT required for MVP

**Required Action:**
- ❌ Remove `--peer` flag from CLI
- ❌ Remove peer dependency resolution logic
- ✅ Store peerDependencies in packument model (for future)
- ✅ Document as unsupported in LIMITATIONS.md

---

### 3. **Missing Unsupported Dependency Detection**

**Current State:**
- No explicit detection of git/file/workspace dependencies
- No warning system for unsupported specs

**PRD Says:**
> "The MVP must explicitly classify and report unsupported dependency specs."
> Must detect: git, git+, http, https, file, link, workspace, GitHub shorthand

**Required Action:**
- ✅ Add unsupported spec detector
- ✅ Classify dependency types
- ✅ Emit warnings for unsupported specs
- ✅ Include in JSON output under "unsupported" field

---

### 4. **JSON Output Schema Mismatch**

**Current State:**
- Custom JSON output format
- Missing required fields from PRD schema

**PRD Requires:**
```json
{
  "schema_version": 1,
  "tool": { "name": "depsim", "version": "0.1.0" },
  "input": { "manifest": "...", "include_dev": false, "registry": "..." },
  "status": "resolved",
  "root": { "name": "...", "version": "..." },
  "nodes": [...],
  "edges": [...],
  "selected": {...},
  "conflicts": [],
  "warnings": [],
  "unsupported": [],
  "metrics": {...}
}
```

**Required Action:**
- ✅ Implement canonical JSON schema exactly as specified
- ✅ Add schema_version field
- ✅ Add tool metadata
- ✅ Add input metadata
- ✅ Add nodes array
- ✅ Add edges array
- ✅ Add selected map
- ✅ Add warnings array
- ✅ Add unsupported array
- ✅ Add metrics object

---

### 5. **CLI Flags Don't Match PRD**

**Current State:**
```bash
--format, --dev, --peer, --optional, --registry, --timeout, --verbose, --no-color
```

**PRD Specifies:**
```bash
-f, --manifest <path>
--dev
--json
--no-color
--concurrency <n>
--timeout <seconds>
--registry <url>
--verbose
```

**Required Action:**
- ✅ Add `-f, --manifest` flag
- ✅ Replace `--format` with `--json` boolean flag
- ❌ Remove `--peer` flag
- ❌ Remove `--optional` flag (optional deps are always included)
- ✅ Add `--concurrency` flag
- ✅ Rename positional arg to match `--manifest`

---

### 6. **Exit Codes Not Implemented**

**Current State:**
- Basic exit on error
- No structured exit code system

**PRD Requires:**
```text
0 = resolved successfully, possibly with warnings
1 = dependency conflict
2 = tool error
```

**Required Action:**
- ✅ Implement exit code enum
- ✅ Return 0 for success with warnings
- ✅ Return 1 for conflicts
- ✅ Return 2 for tool errors (bad input, network failure)

---

### 7. **Missing Dist-Tag Resolution**

**Current State:**
- Semver module doesn't handle dist-tags
- No logic to resolve "latest", "next", etc.

**PRD Requires:**
> "Dist-tags must resolve using the packument dist-tags field"

**Required Action:**
- ✅ Add dist-tag resolution to semver module
- ✅ Handle "latest", "next", "beta", etc.
- ✅ Report error if dist-tag doesn't exist

---

### 8. **Bare Version Not Treated as Exact**

**Current State:**
- Semver module may treat `1.2.3` as `^1.2.3`

**PRD Emphasizes:**
> "Bare versions must be exact. This is important: 1.2.3 must not be treated as ^1.2.3"

**Required Action:**
- ✅ Fix semver parsing to treat bare versions as exact
- ✅ Add unit test: `parse_range("1.2.3")` must equal `=1.2.3`

---

### 9. **Missing npm Alias Support**

**Current State:**
- No support for `npm:real-package@^1.2.3`

**PRD Requires:**
> Must support: "npm:real-package@^1.2.3"

**Required Action:**
- ✅ Add alias parsing
- ✅ Resolve aliased packages
- ✅ Handle in dependency graph

---

### 10. **Missing Scoped Package URL Encoding**

**Current State:**
- May not properly encode scoped packages

**PRD Requires:**
> "Scoped packages must be URL-encoded. @scope/package becomes %40scope%2Fpackage"

**Required Action:**
- ✅ Add URL encoding for scoped packages
- ✅ Test with @scope/package format

---

### 11. **Missing Accept Header**

**Current State:**
- Standard HTTP headers

**PRD Requires:**
```http
Accept: application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*
```

**Required Action:**
- ✅ Add abbreviated metadata Accept header
- ✅ Reduces payload size

---

### 12. **Missing Retry Logic**

**Current State:**
- Basic error handling
- No retry mechanism

**PRD Requires:**
> "Retry on: network timeout, HTTP 429, HTTP 5xx"
> "3 attempts, exponential backoff"

**Required Action:**
- ✅ Implement retry logic
- ✅ Exponential backoff
- ✅ Retry on 429, 5xx, timeout
- ✅ Don't retry on 404, 4xx (except 429)

---

### 13. **Concurrency Default Wrong**

**Current State:**
- Default concurrency: 50

**PRD Requires:**
> "Recommended default: 16 concurrent requests"

**Required Action:**
- ✅ Change default from 50 to 16
- ✅ Make configurable with `--concurrency`

---

### 14. **Missing Circular Dependency Protection**

**Current State:**
- Basic visited set
- May not handle all circular cases

**PRD Requires:**
> "Must track already-expanded nodes: package@version"
> "Must not infinitely recurse"

**Required Action:**
- ✅ Strengthen circular dependency detection
- ✅ Track package@version, not just package
- ✅ Add test for circular dependencies

---

### 15. **Missing Deprecated Package Warnings**

**Current State:**
- No deprecated package detection

**PRD Requires:**
> "Deprecated versions may still be selected, but DEPSIM should emit a warning"

**Required Action:**
- ✅ Check packument deprecated field
- ✅ Emit warning if deprecated version selected
- ✅ Include in warnings array

---

### 16. **Missing Optional Dependency Handling**

**Current State:**
- Optional dependencies treated like regular deps

**PRD Requires:**
> "If an optional dependency cannot be resolved, DEPSIM should emit a warning and continue"

**Required Action:**
- ✅ Detect optional dependencies
- ✅ Continue on optional dep failure
- ✅ Emit warning with code OPTIONAL_DEPENDENCY_UNRESOLVED

---

### 17. **Missing Safety Limits**

**Current State:**
- No limits on resolution

**PRD Requires:**
> "Recommended limits: max packages 5000, max depth 100, max response 50MB"

**Required Action:**
- ✅ Add max_packages limit (5000)
- ✅ Add max_depth limit (100)
- ✅ Add max_response_size limit (50MB)
- ✅ Make configurable

---

### 18. **Missing No-Tarball Test**

**Current State:**
- No test asserting tarballs are never fetched

**PRD Requires:**
> "Add a test asserting that DEPSIM never requests tarball URLs"
> "All URLs must: start with registry base, not end with .tgz, not contain /-/"

**Required Action:**
- ✅ Add test_no_tarball_requests test
- ✅ Assert all URLs match registry pattern
- ✅ Assert no .tgz URLs
- ✅ Assert no /-/ in URLs

---

### 19. **Missing Documentation Files**

**Current State:**
- README.md exists but doesn't match PRD requirements
- Missing SUPPORT.md
- Missing LIMITATIONS.md
- Missing OUTPUT.md

**PRD Requires:**
- SUPPORT.md with feature matrix
- LIMITATIONS.md with known limitations
- OUTPUT.md with schema documentation

**Required Action:**
- ✅ Rewrite README to match PRD definition
- ✅ Create SUPPORT.md
- ✅ Create LIMITATIONS.md
- ✅ Create OUTPUT.md

---

### 20. **Core/CLI Separation Incomplete**

**Current State:**
- Some mixing of concerns
- CLI logic in core modules

**PRD Requires:**
> "The resolver core should not depend on: clap, indicatif, colored, terminal output, stdout printing"

**Required Action:**
- ✅ Audit dependencies
- ✅ Remove terminal output from core
- ✅ Core returns structured results only
- ✅ CLI handles all presentation

---

## 📋 Required File Changes

### Files to Modify

1. **Cargo.toml**
   - Remove `splr` dependency
   - Add `urlencoding` dependency
   - Update reqwest features for rustls

2. **src/cli.rs**
   - Remove `--peer` flag
   - Remove `--optional` flag
   - Add `-f, --manifest` flag
   - Replace `--format` with `--json`
   - Add `--concurrency` flag
   - Update help text

3. **src/resolver/types.rs**
   - Add UnsupportedDependency type
   - Add Warning type with codes
   - Update ResolutionResult to match PRD schema
   - Add metrics struct

4. **src/registry/client.rs**
   - Add Accept header for abbreviated metadata
   - Add retry logic with exponential backoff
   - Add URL encoding for scoped packages
   - Change default concurrency to 16

5. **src/semver/npm_semver.rs**
   - Fix bare version to be exact
   - Add dist-tag resolution
   - Add npm alias parsing
   - Add OR range support

6. **src/resolver/greedy.rs**
   - Remove peer dependency logic
   - Add unsupported spec detection
   - Add optional dependency handling
   - Add deprecated package warnings
   - Add safety limits
   - Strengthen circular dependency protection

7. **src/resolver/sat.rs**
   - Mark as future milestone
   - Add comment: "Not required for MVP"
   - Keep as stub for future

8. **src/output/mod.rs**
   - Implement canonical JSON schema exactly
   - Add schema_version field
   - Add all required fields per PRD

9. **src/main.rs**
   - Implement exit codes (0, 1, 2)
   - Remove spinner in JSON mode
   - Remove colors in JSON mode

### Files to Create

1. **docs/SUPPORT.md** - Feature support matrix
2. **docs/LIMITATIONS.md** - Known limitations
3. **docs/OUTPUT.md** - JSON schema documentation
4. **tests/no_tarball_test.rs** - Assert no tarball requests
5. **tests/fixtures/** - Test fixtures per PRD

### Files to Update

1. **README.md** - Rewrite to match PRD definition
2. **SETUP_GUIDE.md** - Update for MVP scope

---

## 🎯 Priority Order

### P0 - Blocking MVP (Must Fix)

1. Remove SAT solver from MVP
2. Remove peer dependency resolution
3. Implement canonical JSON schema
4. Fix CLI flags to match PRD
5. Implement exit codes
6. Add unsupported dependency detection
7. Fix bare version to be exact
8. Add dist-tag resolution

### P1 - Critical for Correctness

9. Add retry logic
10. Add URL encoding for scoped packages
11. Add Accept header
12. Fix concurrency default (16)
13. Add optional dependency handling
14. Add circular dependency protection
15. Add npm alias support

### P2 - Important for Quality

16. Add deprecated package warnings
17. Add safety limits
18. Add no-tarball test
19. Create documentation files
20. Audit core/CLI separation

---

## 📊 Compliance Score

**Current Compliance: ~40%**

- ✅ Aligned: 10 items
- ❌ Misaligned: 20 items
- 🔄 Partially Aligned: 5 items

**Target: 100% PRD Compliance for MVP**

---

## 🚀 Recommended Action Plan

### Week 1: Core Fixes
- Remove SAT solver
- Fix JSON schema
- Fix CLI flags
- Implement exit codes

### Week 2: Resolution Correctness
- Fix semver (bare versions, dist-tags, aliases)
- Add unsupported spec detection
- Remove peer dependency logic
- Add optional dependency handling

### Week 3: Registry & Safety
- Add retry logic
- Add URL encoding
- Add Accept header
- Add safety limits
- Fix concurrency

### Week 4: Testing & Docs
- Add no-tarball test
- Create fixture tests
- Write SUPPORT.md
- Write LIMITATIONS.md
- Write OUTPUT.md
- Rewrite README

---

## ✅ Definition of Done

MVP is complete when:

1. All P0 items fixed
2. All P1 items fixed
3. All P2 items fixed
4. cargo test passes
5. cargo clippy passes
6. All PRD requirements met
7. Documentation complete

---

## 📝 Notes

This codebase has a good foundation but needs significant refactoring to align with the PRD.

The main issues are:
1. **Scope creep**: SAT solver and peer dependencies are not MVP
2. **Schema mismatch**: JSON output doesn't match PRD
3. **Missing features**: Unsupported spec detection, dist-tags, aliases
4. **Wrong defaults**: Concurrency, CLI flags

The good news: The architecture is sound. We just need to trim scope and fix details.

**Estimated effort to MVP compliance: 2-3 weeks**