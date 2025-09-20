# Ferro Release Guide

This document explains how to create and manage releases for Ferro.

## Automated Release Process

Ferro uses GitHub Actions to automatically build binaries for all supported platforms and create releases.

### Release Triggers

**Automatic Release (Recommended)**:
1. Create and push a git tag starting with `v`:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

**Manual Release**:
1. Go to Actions tab in GitHub
2. Select "Build and Release" workflow
3. Click "Run workflow"
4. Set "Create a new release" to `true`

## Release Checklist

### Pre-Release

- [ ] Update version in `ferro/Cargo.toml`
- [ ] Update version in README files
- [ ] Run tests locally: `cd ferro && cargo test`
- [ ] Run clippy: `cd ferro && cargo clippy`
- [ ] Update CHANGELOG.md with new features/fixes
- [ ] Commit version bump: `git commit -am "Bump version to v1.0.0"`

### Creating the Release

1. **Create and push tag**:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```

2. **Monitor GitHub Actions**:
   - Go to Actions tab
   - Watch the "Build and Release" workflow
   - Builds will run for all platforms:
     - Windows (x64, x86, ARM64)
     - Linux (x64, ARM64)  
     - macOS (x64, ARM64)

3. **Release is automatically created** with:
   - All platform binaries
   - SHA256 checksums
   - Release notes template

### Post-Release

- [ ] Test download links work
- [ ] Update installation instructions if needed
- [ ] Announce on relevant channels
- [ ] Close milestone if using GitHub milestones

## Supported Platforms

### Windows
- **ferro-windows-x64.exe** - Windows 64-bit (Intel/AMD)
- **ferro-windows-x86.exe** - Windows 32-bit (Intel/AMD)
- **ferro-windows-arm64.exe** - Windows ARM64

### Linux
- **ferro-linux-x64** - Linux 64-bit (Intel/AMD)
- **ferro-linux-arm64** - Linux ARM64 (Raspberry Pi, etc.)

### macOS
- **ferro-macos-x64** - macOS 64-bit (Intel)
- **ferro-macos-arm64** - macOS ARM64 (Apple Silicon)

## Version Numbers

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (1.1.0): New features, backward compatible
- **PATCH** (1.0.1): Bug fixes, backward compatible

### Examples:
- `v1.0.0` - First stable release
- `v1.1.0` - Added new Windows version support
- `v1.0.1` - Fixed download issue
- `v2.0.0` - Breaking CLI changes

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

## Troubleshooting Releases

### Build Failures

**Rust compilation error**:
- Check Cargo.toml syntax
- Ensure all dependencies are compatible
- Check for missing Rust toolchain targets

**Cross-compilation issues**:
- Linux ARM64 builds require `gcc-aarch64-linux-gnu`
- Windows ARM64 requires Windows SDK components

**Test failures**:
- All tests must pass before release creation
- Check logs in GitHub Actions

### Release Creation Issues

**Tag already exists**:
```bash
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
```

**Missing binaries in release**:
- Check matrix build configuration
- Verify artifact upload paths
- Ensure all platform builds succeeded

## Manual Release Creation

If automated release fails, you can create releases manually:

1. **Build locally for your platform**:
   ```bash
   cd ferro
   cargo build --release
   ```

2. **Create release on GitHub**:
   - Go to Releases page
   - Click "Create a new release"
   - Choose tag or create new tag
   - Upload binary files manually
   - Add release notes

## Security Considerations

- [ ] Binaries are built in clean GitHub Actions environment
- [ ] No secrets or credentials in binaries
- [ ] SHA256 checksums provided for verification
- [ ] Signed releases (future enhancement)

## Future Enhancements

- [ ] Code signing for Windows binaries
- [ ] macOS notarization
- [ ] Package manager distribution (Homebrew, Chocolatey, etc.)
- [ ] Docker images
- [ ] Automated changelog generation
