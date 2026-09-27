# DEPSIM Quick Start Guide

**For Engineers Evaluating DEPSIM**

---

## 🎯 What is DEPSIM?

DEPSIM is a **registry-only npm dependency resolution preflight tool** that:
- ✅ Predicts what `npm install` will resolve **before** running it
- ✅ Fetches only JSON metadata from npm registry (never tarballs)
- ✅ Resolves 500-package trees in **under 2 seconds**
- ✅ Ships as a **single static binary** (no Node.js required)
- ✅ Provides **npm-accurate** semver resolution

---

## 🚀 Try It Now (3 Ways)

### 1. Web Demo (Fastest - 30 seconds)

Visit: **https://permaconomy.github.io/depsim/**

1. Click "Try Demo" tab
2. Paste your `package.json` content
3. Click "Analyze Dependencies"
4. See resolved versions, conflicts, and dependency tree

**Perfect for:** Quick evaluation, sharing with team

---

### 2. Download Binary (Recommended - 2 minutes)

**Linux:**
```bash
curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-linux-x64 -o depsim
chmod +x depsim
./depsim --help
```

**macOS (Intel):**
```bash
curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-macos-x64 -o depsim
chmod +x depsim
./depsim --help
```

**macOS (Apple Silicon):**
```bash
curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-macos-arm64 -o depsim
chmod +x depsim
./depsim --help
```

**Windows:**
```powershell
# Download from: https://github.com/permaconomy/depsim/releases/latest
# Get: depsim-windows-x64.exe
.\depsim-windows-x64.exe --help
```

**Perfect for:** Local testing, CI/CD integration

---

### 3. Build from Source (5 minutes)

```bash
# Clone repository
git clone https://github.com/permaconomy/depsim.git
cd depsim

# Build (requires Rust 1.70+)
cargo build --release

# Run
./target/release/depsim --help
```

**Perfect for:** Contributing, customization

---

## 📖 Basic Usage

### Analyze Current Project
```bash
# In directory with package.json
depsim
```

### Analyze Specific File
```bash
depsim /path/to/package.json
```

### Include Dev Dependencies
```bash
depsim --dev
```

### JSON Output (for CI/CD)
```bash
depsim --json > resolution.json
```

### Increase Concurrency
```bash
depsim --concurrency 32
```

---

## 🎮 Example Scenarios

### Scenario 1: Check for Conflicts Before Install

```bash
# Before running npm install
depsim

# Output shows:
# ✅ All dependencies resolved successfully
# OR
# ❌ Conflict detected: package-a@1.0.0 vs package-a@2.0.0
```

**Use Case:** Prevent failed installs in CI/CD

---

### Scenario 2: Compare Resolution Across Environments

```bash
# Development
depsim --dev --json > dev-resolution.json

# Production
depsim --json > prod-resolution.json

# Compare
diff dev-resolution.json prod-resolution.json
```

**Use Case:** Ensure consistent dependencies

---

### Scenario 3: Audit Before Upgrade

```bash
# Current state
depsim --json > before.json

# Edit package.json (upgrade dependency)
# ...

# New state
depsim --json > after.json

# Compare impact
diff before.json after.json
```

**Use Case:** Safe dependency upgrades

---

### Scenario 4: CI/CD Integration

```yaml
# .github/workflows/check-deps.yml
name: Check Dependencies
on: [pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Download DEPSIM
        run: |
          curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-linux-x64 -o depsim
          chmod +x depsim
      - name: Check Resolution
        run: ./depsim --json
```

**Use Case:** Automated dependency validation

---

## 📊 Understanding Output

### Human-Readable Output

```
✓ Resolved 42 packages in 1.2s

Dependencies:
  express@4.18.2
  ├─ body-parser@1.20.1
  ├─ cookie@0.5.0
  └─ debug@2.6.9
      └─ ms@2.0.0

Conflicts:
  ❌ lodash: 4.17.21 (required by express) vs 3.10.1 (required by old-package)
```

### JSON Output

```json
{
  "status": "conflict",
  "resolved": {
    "express": "4.18.2",
    "body-parser": "1.20.1"
  },
  "conflicts": [
    {
      "package": "lodash",
      "versions": ["4.17.21", "3.10.1"],
      "requiredBy": ["express", "old-package"]
    }
  ],
  "edges": [
    {"from": "express@4.18.2", "to": "body-parser@1.20.1"}
  ]
}
```

---

## 🔍 What to Evaluate

### 1. Speed ⚡
- Run on your largest project
- Should complete in <2 seconds for typical projects
- Compare with `npm install --dry-run` (if it works)

### 2. Accuracy 🎯
- Compare DEPSIM output with actual `npm install`
- Check if resolved versions match
- Verify conflict detection

### 3. Usability 🎨
- Is the output clear and actionable?
- Are error messages helpful?
- Does it integrate well with your workflow?

### 4. Portability 🚀
- Test on different platforms (Linux, macOS, Windows)
- Try in CI/CD pipeline
- Check binary size and dependencies

---

## 🐛 Known Limitations (MVP)

Current version does **NOT** support:
- ❌ Peer dependencies (planned for v0.2.0)
- ❌ Git dependencies (shows as unsupported)
- ❌ File/workspace dependencies (shows as unsupported)
- ❌ npm aliases (e.g., `pkg@npm:other-pkg`)
- ❌ SAT-based backtracking (greedy-only for now)

See [CODEBASE_ASSESSMENT.md](https://github.com/permaconomy/depsim/blob/main/CODEBASE_ASSESSMENT.md) for full list.

---

## 💡 Comparison with Existing Tools

| Feature | DEPSIM | npm --dry-run | npm-check | depsly |
|---------|--------|---------------|-----------|--------|
| Simulates resolution | ✅ | ⚠️ (buggy) | ❌ | ❌ (Python) |
| No Node.js required | ✅ | ❌ | ❌ | ✅ |
| Speed (<2s) | ✅ | ❌ | ✅ | N/A |
| Conflict detection | ✅ | ⚠️ | ❌ | ❌ |
| JSON output | ✅ | ❌ | ❌ | ✅ |
| Single binary | ✅ | ❌ | ❌ | ✅ |

---

## 📚 Further Reading

- **[PRD.md](https://github.com/permaconomy/depsim/blob/main/PRD.md)** - Complete product requirements
- **[DEPSIM_ARCHITECTURE.md](https://github.com/permaconomy/depsim/blob/main/DEPSIM_ARCHITECTURE.md)** - Technical architecture
- **[CODEBASE_ASSESSMENT.md](https://github.com/permaconomy/depsim/blob/main/CODEBASE_ASSESSMENT.md)** - Current status and roadmap

---

## 🤝 Feedback

We want your feedback! Please:

1. **Try the tool** on your projects
2. **Report issues** at: https://github.com/permaconomy/depsim/issues
3. **Share results** - what worked, what didn't
4. **Suggest features** - what would make this more useful?

---

## ⏱️ Evaluation Checklist

- [ ] Tried web demo
- [ ] Downloaded and ran binary
- [ ] Tested on real project
- [ ] Compared with npm install results
- [ ] Checked speed on large project
- [ ] Tested JSON output format
- [ ] Tried in CI/CD pipeline
- [ ] Read PRD and architecture docs
- [ ] Provided feedback via GitHub Issues

---

## 🎯 Target Use Cases

### Solo Developers
- Quick dependency checks before install
- Understand dependency trees
- Catch conflicts early

### Enterprise Architects
- Validate dependency policies
- Audit before production deploys
- CI/CD integration for safety gates
- Dependency impact analysis

### DevOps Engineers
- Automated dependency validation
- Pre-deployment checks
- Consistent environment verification
- Fast feedback in pipelines

---

## 📞 Support

- **Documentation:** https://github.com/permaconomy/depsim
- **Issues:** https://github.com/permaconomy/depsim/issues
- **Discussions:** https://github.com/permaconomy/depsim/discussions

---

**Ready to evaluate? Start here:** https://permaconomy.github.io/depsim/

**Questions? Open an issue:** https://github.com/permaconomy/depsim/issues/new