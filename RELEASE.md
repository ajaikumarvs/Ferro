# Ferro Release Guide

This document explains how to create and manage releases for Ferro using our automated CI/CD pipeline.

## 🚀 Quick Start

**To create a new release:**
```bash
# 1. Update version in ferro/Cargo.toml
# 2. Commit changes
git add . && git commit -m "Bump version to v1.0.0"

# 3. Create and push tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# 4. GitHub Actions automatically builds and releases!
```

## Automated Release Process

Ferro uses GitHub Actions to automatically build binaries for **7 platforms** and create professional releases with zero manual work.

### Release Triggers

**🎯 Automatic Release (Recommended)**:
- **Trigger**: Push a git tag starting with `v*`
- **Example**: `v1.0.0`, `v2.1.3`, `v1.0.0-beta.1`
- **Result**: Full cross-platform build and release

```bash
git tag v1.0.0
git push origin v1.0.0
```

**⚡ Manual Release**:
- **When**: Testing or emergency releases
- **How**: GitHub Actions UI
  1. Go to **Actions** tab in GitHub
  2. Select **"Build and Release"** workflow
  3. Click **"Run workflow"**
  4. Set **"Create a new release"** to `true`

## 📋 Release Checklist

### 🔍 Pre-Release Quality Checks

- [ ] **Update version** in `ferro/Cargo.toml`
- [ ] **Update version** in README files and documentation
- [ ] **Run local tests**: `cd ferro && cargo test --verbose`
- [ ] **Check code quality**: `cd ferro && cargo clippy -- -D warnings`
- [ ] **Verify formatting**: `cd ferro && cargo fmt -- --check`
- [ ] **Update CHANGELOG.md** with new features/fixes/breaking changes
- [ ] **Test locally** with `cargo build --release`
- [ ] **Commit version bump**: `git commit -am "Bump version to v1.0.0"`

### 🚀 Creating the Release

1. **Create annotated tag**:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0: Add Windows 11 24H2 support"
   git push origin v1.0.0
   ```

2. **Monitor GitHub Actions** (takes ~15-20 minutes):
   - Go to **Actions** tab
   - Watch **"Build and Release"** workflow
   - **7 platform builds** will run in parallel:
     - 🪟 Windows (x64, x86, ARM64)
     - 🐧 Linux (x64, ARM64)  
     - 🍎 macOS (x64, ARM64)

3. **Release automatically created** with:
   - ✅ All 7 platform binaries
   - ✅ SHA256 checksums (`checksums.txt`)
   - ✅ Professional release notes
   - ✅ Download instructions

### ✅ Post-Release Verification

- [ ] **Test downloads**: Verify all platform binaries work
- [ ] **Check checksums**: Validate SHA256 hashes match
- [ ] **Update installation docs**: Refresh download links if needed
- [ ] **Announce release**: Share on relevant channels/social media
- [ ] **Close milestone**: If using GitHub project management
- [ ] **Monitor for issues**: Watch for user reports in first 24-48 hours

## 📦 Supported Platforms

Our automated builds create binaries for **7 platforms**:

### 🪟 Windows
- **ferro-windows-x64.exe** - Windows 64-bit (Intel/AMD) - *Most common*
- **ferro-windows-x86.exe** - Windows 32-bit (Intel/AMD) - *Legacy systems*
- **ferro-windows-arm64.exe** - Windows ARM64 - *Surface Pro X, etc.*

### 🐧 Linux  
- **ferro-linux-x64** - Linux 64-bit (Intel/AMD) - *Ubuntu, Debian, CentOS, etc.*
- **ferro-linux-arm64** - Linux ARM64 - *Raspberry Pi 4+, ARM servers*

### 🍎 macOS
- **ferro-macos-x64** - macOS 64-bit (Intel) - *Intel Macs*
- **ferro-macos-arm64** - macOS ARM64 (Apple Silicon) - *M1/M2/M3 Macs*

> **Note**: All binaries are statically linked with no external dependencies required!

## 🔢 Version Numbers

Follow [Semantic Versioning](https://semver.org/) strictly:

- **MAJOR** (`1.0.0`): Breaking changes that require user action
- **MINOR** (`1.1.0`): New features, backward compatible
- **PATCH** (`1.0.1`): Bug fixes, backward compatible

### 📝 Version Examples:
- `v1.0.0` - 🎉 First stable release
- `v1.1.0` - ✨ Added Windows 11 24H2 support
- `v1.0.1` - 🐛 Fixed ARM64 architecture detection
- `v2.0.0` - 💥 Breaking CLI changes (new argument format)
- `v1.2.0-beta.1` - 🧪 Beta release with UEFI Shell 2.3 support

### 🏷️ Tag Naming Convention:
```bash
# Stable releases
git tag -a v1.0.0 -m "Release v1.0.0"

# Pre-releases  
git tag -a v1.1.0-beta.1 -m "Beta release v1.1.0-beta.1"
git tag -a v2.0.0-rc.1 -m "Release candidate v2.0.0-rc.1"
```

## Release Notes Template

Each release includes automatic release notes. You can customize them by editing the workflow or manually after creation:

```markdown
# Ferro v1.0.0

Cross-platform Windows ISO downloader - Rust rewrite of Fido

## 🚀 New Features
- Added Windows 11 24H2 support
- Improved error handling

## 🐛 Bug Fixes  
- Fixed ARM64 architecture detection
- Resolved locale detection on macOS

## 📦 Downloads
[Links to all platform binaries are auto-generated]

## 🔒 Verification
SHA256 checksums are provided in `checksums.txt`.
```

## 🔨 Troubleshooting Releases

### 🚫 Build Failures

**❌ Rust compilation error**:
- ✅ Check `Cargo.toml` syntax and version constraints
- ✅ Ensure all dependencies are compatible
- ✅ Verify Rust toolchain targets are available
- ✅ Check for breaking changes in dependencies

**❌ Cross-compilation issues**:
- 🐧 **Linux ARM64**: Requires `gcc-aarch64-linux-gnu` (auto-installed in CI)
- 🪟 **Windows ARM64**: Requires Windows SDK components (available in `windows-latest`)
- 🍎 **macOS**: Both Intel and Apple Silicon supported on `macos-latest`

**❌ Test failures**:
- ✅ All tests must pass before release creation
- ✅ Check logs in **GitHub Actions** → **Build and Release** workflow
- ✅ Fix failing tests and re-tag to trigger new build

### 🏷️ Release Creation Issues

**❌ Tag already exists**:
```bash
# Delete local and remote tag
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# Recreate with new commit
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

**❌ Missing binaries in release**:
- ✅ Check **matrix build configuration** in `.github/workflows/release.yml`
- ✅ Verify **artifact upload paths** match expected locations  
- ✅ Ensure **all 7 platform builds** succeeded (check Actions tab)
- ✅ Look for **red X marks** in the build matrix

**❌ Workflow permission issues**:
```bash
# Ensure GITHUB_TOKEN has proper permissions
# In repository Settings → Actions → General → Workflow permissions
# Select: "Read and write permissions"
```

## 🔧 Manual Release Creation

If automated release fails, you can create releases manually:

### Option 1: Local Build (Single Platform)
```bash
cd ferro
cargo build --release
# Binary will be in target/release/ferro (or ferro.exe on Windows)
```

### Option 2: GitHub UI (Recommended)
1. Go to **Releases** page on GitHub
2. Click **"Create a new release"**
3. Choose existing tag or create new tag
4. Upload binary files manually
5. Add release notes using template above

### Option 3: GitHub CLI
```bash
# Install GitHub CLI first: https://cli.github.com/
gh release create v1.0.0 ./target/release/ferro --title "Ferro v1.0.0" --notes "Release notes here"
```

## 🔒 Security Considerations

Our release process includes multiple security measures:

- ✅ **Clean build environment**: All binaries built in fresh GitHub Actions runners
- ✅ **No secrets**: No API keys, passwords, or credentials embedded in binaries
- ✅ **Checksum verification**: SHA256 checksums provided for all binaries
- ✅ **Reproducible builds**: Same source always produces same binary
- ✅ **Minimal dependencies**: Statically linked with no external runtime deps
- 🔄 **Code signing**: Windows binaries (planned future enhancement)
- 🔄 **macOS notarization**: Apple notarized binaries (planned)

### 🛡️ For Users:
```bash
# Verify downloads with provided checksums
sha256sum ferro-linux-x64
# Should match value in checksums.txt
```

## 🚀 Future Enhancements

### 📦 Distribution
- [ ] **Homebrew formula** (`brew install ferro`)
- [ ] **Chocolatey package** (`choco install ferro`)  
- [ ] **Debian/Ubuntu packages** (`.deb` files)
- [ ] **RPM packages** for RHEL/CentOS/Fedora
- [ ] **Docker images** (`docker run ferro`)
- [ ] **Snap packages** (`snap install ferro`)

### 🔐 Security & Quality
- [ ] **Windows code signing** with valid certificate
- [ ] **macOS notarization** for Gatekeeper compatibility
- [ ] **Linux AppImage** format for universal compatibility
- [ ] **Reproducible builds** verification

### 🤖 Automation
- [ ] **Automated changelog** generation from commits
- [ ] **Release notes** from PR descriptions
- [ ] **Version bumping** automation
- [ ] **Performance benchmarks** in CI
- [ ] **Integration tests** with real Microsoft APIs (in isolated environment)

---

## 📞 Need Help?

- **Issues**: Open an issue on GitHub
- **Discussions**: Use GitHub Discussions for questions
- **Security**: Email maintainer for security issues
