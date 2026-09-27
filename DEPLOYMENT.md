# DEPSIM Deployment Guide

## 🚀 Deploying to GitHub (permaconomy/depsim)

### Prerequisites
- GitHub account with permaconomy organization access
- Git installed locally
- Empty repository created at: `https://github.com/permaconomy/depsim`

---

## Step 1: Initialize Git Repository

```bash
cd C:/Users/DELL/Documents/111-depsim

# Initialize git
git init

# Add all files
git add .

# Create initial commit
git commit -m "Initial commit: DEPSIM MVP foundation"
```

---

## Step 2: Connect to GitHub

```bash
# Add remote
git remote add origin https://github.com/permaconomy/depsim.git

# Push to GitHub
git branch -M main
git push -u origin main
```

---

## Step 3: Enable GitHub Pages

1. Go to: `https://github.com/permaconomy/depsim/settings/pages`
2. Under "Source", select: **GitHub Actions**
3. Save

The web demo will be automatically deployed to:
```
https://permaconomy.github.io/depsim/
```

---

## Step 4: Create First Release

### Option A: Via GitHub UI
1. Go to: `https://github.com/permaconomy/depsim/releases/new`
2. Click "Choose a tag"
3. Type: `v0.1.0`
4. Click "Create new tag: v0.1.0 on publish"
5. Title: `v0.1.0 - Initial MVP Release`
6. Description: See template below
7. Click "Publish release"

### Option B: Via Git Tag
```bash
# Create and push tag
git tag -a v0.1.0 -m "Initial MVP release"
git push origin v0.1.0
```

The GitHub Actions workflow will automatically:
- Build binaries for Linux, macOS (Intel & ARM), Windows
- Create a GitHub release
- Attach binaries to the release

---

## Step 5: Verify Deployment

### Check Binary Releases
Visit: `https://github.com/permaconomy/depsim/releases`

You should see:
- ✅ depsim-linux-x64
- ✅ depsim-macos-x64
- ✅ depsim-macos-arm64
- ✅ depsim-windows-x64.exe

### Check Web Demo
Visit: `https://permaconomy.github.io/depsim/`

You should see:
- ✅ Interactive web interface
- ✅ Try Demo tab
- ✅ Installation instructions
- ✅ Documentation

---

## Release Description Template

```markdown
## DEPSIM v0.1.0 - Initial MVP Release

Registry-only npm dependency resolution preflight tool built in Rust.

### 🎯 What is DEPSIM?

DEPSIM reads package.json files, fetches only npm registry metadata, and predicts resolved versions, dependency edges, and conflicts without installing anything.

### ✨ Features

- ⚡ Fast: Resolves 500-package trees in <2 seconds
- 🔒 Safe: Never downloads tarballs or executes scripts
- 🚀 Portable: Single static binary, no Node.js required
- 🎯 Accurate: npm-compatible semver resolution

### 📦 Installation

**Linux:**
```bash
curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-linux-x64 -o depsim
chmod +x depsim
sudo mv depsim /usr/local/bin/
```

**macOS (Intel):**
```bash
curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-macos-x64 -o depsim
chmod +x depsim
sudo mv depsim /usr/local/bin/
```

**macOS (Apple Silicon):**
```bash
curl -L https://github.com/permaconomy/depsim/releases/latest/download/depsim-macos-arm64 -o depsim
chmod +x depsim
sudo mv depsim /usr/local/bin/
```

**Windows:**
Download `depsim-windows-x64.exe` and add to PATH.

### 🎮 Try It Online

Web Demo: https://permaconomy.github.io/depsim/

### 📖 Usage

```bash
# Analyze package.json in current directory
depsim

# Analyze specific package.json
depsim path/to/package.json

# JSON output for CI
depsim --json > output.json

# Include dev dependencies
depsim --dev
```

### 📚 Documentation

- [Product Requirements (PRD)](https://github.com/permaconomy/depsim/blob/main/PRD.md)
- [Architecture](https://github.com/permaconomy/depsim/blob/main/DEPSIM_ARCHITECTURE.md)
- [Setup Guide](https://github.com/permaconomy/depsim/blob/main/SETUP_GUIDE.md)

### ⚠️ MVP Status

This is an MVP release. See [CODEBASE_ASSESSMENT.md](https://github.com/permaconomy/depsim/blob/main/CODEBASE_ASSESSMENT.md) for known limitations and planned improvements.

### 🤝 Contributing

Contributions welcome! See [PRD.md](https://github.com/permaconomy/depsim/blob/main/PRD.md) for scope and requirements.

### 📄 License

MIT License - see [LICENSE](https://github.com/permaconomy/depsim/blob/main/LICENSE) for details.
```

---

## Troubleshooting

### GitHub Actions Not Running

1. Check: `https://github.com/permaconomy/depsim/actions`
2. Ensure workflows are enabled in repository settings
3. Check workflow files in `.github/workflows/`

### Pages Not Deploying

1. Verify GitHub Pages is enabled in settings
2. Check Pages source is set to "GitHub Actions"
3. Check Actions tab for deployment status
4. Wait 2-3 minutes for first deployment

### Binary Build Failures

1. Check Actions logs for errors
2. Verify Cargo.toml is valid
3. Ensure all dependencies are available
4. Check Rust toolchain compatibility

---

## Post-Deployment Checklist

- [ ] Repository pushed to GitHub
- [ ] GitHub Pages enabled
- [ ] First release created (v0.1.0)
- [ ] Binaries built successfully
- [ ] Web demo accessible
- [ ] README updated with correct URLs
- [ ] License file added
- [ ] Contributing guidelines added

---

## URLs After Deployment

- **Repository:** https://github.com/permaconomy/depsim
- **Web Demo:** https://permaconomy.github.io/depsim/
- **Releases:** https://github.com/permaconomy/depsim/releases
- **Issues:** https://github.com/permaconomy/depsim/issues
- **Actions:** https://github.com/permaconomy/depsim/actions

---

## Next Steps

1. **Test the binaries** on different platforms
2. **Share the web demo** with engineers for evaluation
3. **Collect feedback** via GitHub Issues
4. **Implement P0 fixes** from CODEBASE_ASSESSMENT.md
5. **Create v0.2.0** with improvements

---

## Support

For deployment issues:
- Check GitHub Actions logs
- Review workflow files
- Open an issue on GitHub
- Contact repository maintainers

**Deployment Status:** Ready to deploy! 🚀