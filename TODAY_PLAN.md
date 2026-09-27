# DEPSIM MVP - TODAY'S COMPLETION PLAN

**Goal:** Complete MVP today for submission, testing, and engineer evaluation  
**Time:** Today (2026-09-27)  
**Status:** IN PROGRESS

---

## 🎯 Critical Path to MVP (Today)

### Phase 1: Remove Scope Creep (30 min)
- [ ] Remove splr from Cargo.toml
- [ ] Comment out SAT solver code
- [ ] Remove --peer flag from CLI
- [ ] Update documentation

### Phase 2: Fix Core Issues (2 hours)
- [ ] Implement canonical JSON schema
- [ ] Fix CLI flags (add --manifest, --json, --concurrency)
- [ ] Implement exit codes (0, 1, 2)
- [ ] Add unsupported dependency detection
- [ ] Fix bare version = exact in semver

### Phase 3: Add Critical Features (2 hours)
- [ ] Add dist-tag resolution
- [ ] Add npm alias support
- [ ] Add retry logic
- [ ] Add URL encoding for scoped packages
- [ ] Fix concurrency default to 16

### Phase 4: Testing & Docs (1 hour)
- [ ] Add basic unit tests
- [ ] Add no-tarball test
- [ ] Create SUPPORT.md
- [ ] Create LIMITATIONS.md
- [ ] Update README

### Phase 5: Build & Verify (30 min)
- [ ] cargo fmt
- [ ] cargo clippy
- [ ] cargo test
- [ ] cargo build --release
- [ ] Test with real package.json

---

## 📋 Simplified MVP Scope for Today

**Must Have:**
1. ✅ Reads package.json
2. ✅ Fetches registry metadata only
3. ✅ Greedy resolution (no SAT)
4. ✅ Detects unsupported specs
5. ✅ Canonical JSON output
6. ✅ Correct exit codes
7. ✅ No tarballs
8. ✅ Basic documentation

**Nice to Have (if time):**
- Optional dependency warnings
- Deprecated package warnings
- Safety limits

**Explicitly Out:**
- SAT solver
- Peer dependencies
- Lockfile generation
- Workspace support

---

## 🚀 Let's Start!

Beginning implementation now...