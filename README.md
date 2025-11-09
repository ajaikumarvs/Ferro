# Ferro - Cross-Platform Windows ISO Downloader

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Cross Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)](https://github.com/ajaikumarvs/Ferro)

# ⚠️ **FERRO IS IN ACTIVE TESTING PHASE** ⚠️

> **Multiple or rapid download requests WILL trigger a temporary IP ban**  
> (typically **24 hours**, sometimes longer) from Microsoft's servers.

You will see error: `715-123130`

**Do NOT run automated scripts, loops, or concurrent downloads.**  
One request at a time · Wait at least **2–3 minutes** between sessions.

**By using Ferro, you acknowledge and accept full responsibility for any temporary or permanent IP bans** 
triggered by Microsoft's download servers (error 715-123130). 

**No warranties are provided. Use at your own risk.**


## Description

Ferro is a cross-platform Windows ISO downloader written in Rust - a full 1:1 rewrite of the popular [Fido PowerShell script](https://github.com/pbatard/Fido) by Pete Batard. Ferro provides a modern CLI interface for downloading official Microsoft Windows retail ISOs and UEFI Shell images without requiring PowerShell or Windows.


This tool exists because, while Microsoft does make retail ISO download links freely and publicly available (at least for Windows 8 through Windows 11), these links were historically only available after forcing users to jump through many unwarranted hoops that created an exceedingly counterproductive consumer experience.

### Why Retail ISOs?

Using official retail ISOs is currently the only way to assert with absolute certainty that the OS content has not been altered. Because there exists only a single master for each retail ISO, Microsoft retail ISOs are the only ones you can obtain an official SHA-1 for (from MSDN or sites like [msdn.rg-adguard.net](https://msdn.rg-adguard.net/public.php)), allowing you to be 100% sure that the image you are using has not been corrupted and is safe to use.

Unlike Microsoft's Media Creation Tool (MCT), which generates ISOs on-the-fly (making each one unique), retail ISOs provide verifiable, bit-for-bit identical content that matches Microsoft's original release.

##  Key Features

- **Cross-Platform**: Works on Windows, macOS, and Linux without PowerShell dependency
- **Official ISOs**: Downloads genuine Microsoft Windows retail ISOs with verifiable checksums  
- **Multiple Versions**: Supports Windows 11, Windows 10, and UEFI Shell images
- **Modern CLI**: Clean command-line interface with helpful error messages
- **Smart Defaults**: Automatically selects appropriate language and architecture based on system
- **Progress Tracking**: Real-time download progress with speed and ETA information
- **URL-Only Mode**: Option to retrieve download URLs without downloading
- **Session Management**: Proper Microsoft API integration with anti-ban measures

## Installation

### From Source
```bash
git clone https://github.com/ajaikumarvs/Ferro.git
cd Ferro/ferro
cargo build --release
./target/release/ferro --help
```

### Prerequisites
- Rust 1.70 or later
- Internet connection for downloading ISOs

## Usage

### Quick Start

```bash
# Show help
ferro --help

# List all available Windows versions
ferro list versions

# Download Windows 11 with defaults (auto-detects locale and architecture)
ferro download --version "Windows 11"

# Get download URL only (no actual download)
ferro download --get-url --version "Windows 11"
```

### List Available Options

```bash
# List available Windows versions
ferro list versions

# List releases for Windows 11
ferro list releases "Windows 11"

# List editions for Windows 11 24H2
ferro list editions "Windows 11" "24H2 (Build 26100.1742 - 2024.10)"

# List languages for Windows 11 24H2 Home/Pro/Edu
ferro list languages "Windows 11" "24H2 (Build 26100.1742 - 2024.10)" "Windows 11 Home/Pro/Edu"

# List architectures for Windows 11 24H2 Home/Pro/Edu English
ferro list architectures "Windows 11" "24H2 (Build 26100.1742 - 2024.10)" "Windows 11 Home/Pro/Edu" "English"
```

### Download Examples

#### Windows 11 Downloads
```bash
# Download Windows 11 with full specification
ferro download \
  --version "Windows 11" \
  --release "24H2 (Build 26100.1742 - 2024.10)" \
  --edition "Windows 11 Home/Pro/Edu" \
  --language "English" \
  --architecture "x64" \
  --output "Windows11_24H2_x64_English.iso"

# Download with automatic defaults (uses system locale and architecture)
ferro download --version "Windows 11"

# Get download URL without downloading
ferro download --get-url \
  --version "Windows 11" \
  --architecture "x64"

# Download ARM64 version for Apple Silicon Macs
ferro download \
  --version "Windows 11" \
  --architecture "ARM64" \
  --output "Windows11_ARM64.iso"
```

#### UEFI Shell Downloads
```bash
# Download latest UEFI Shell (Release build)
ferro download \
  --version "UEFI Shell 2.2" \
  --release "25H1 (edk2-stable202505)" \
  --edition "Release" \
  --output "UEFI_Shell_2.2_25H1_Release.iso"

# Download Debug build  
ferro download \
  --version "UEFI Shell 2.2" \
  --release "25H1 (edk2-stable202505)" \
  --edition "Debug" \
  --output "UEFI_Shell_2.2_25H1_Debug.iso"
```

### Command Line Options

- `-w, --version <VERSION>`: Windows version (e.g., "Windows 11", "Windows 10")
- `-r, --release <RELEASE>`: Windows release (e.g., "24H2", "22H2")  
- `-e, --edition <EDITION>`: Windows edition (e.g., "Home/Pro/Edu", "Pro")
- `-l, --language <LANGUAGE>`: Language (e.g., "English", "en-US")
- `-a, --architecture <ARCH>`: Architecture (e.g., "x64", "x86", "ARM64")
- `-o, --output <PATH>`: Output file path
- `--get-url`: Only display download URL without downloading

## Examples

```bash
# Quick download with defaults
ferro download

# Download Windows 10 Pro in French for x64
ferro download -w "Windows 10" -e "Pro" -l "French" -a "x64"

# Get download URL for Windows 11 ARM64
ferro download -w "Windows 11" -a "ARM64" --get-url

# Download UEFI Shell
ferro download -w "UEFI Shell 2.2" -e "Release"
```

## Technical Details

Ferro replicates the functionality of the original Fido PowerShell script by:

1. **Session Management**: Creates authenticated sessions with Microsoft's download servers
2. **API Communication**: Uses Microsoft's software-download-connector API endpoints
3. **Multi-step Process**: Follows the same version → release → edition → language → architecture → download workflow
4. **Anti-automation Handling**: Implements session whitelisting and proper request headers
5. **Cross-platform Compatibility**: Pure Rust implementation works on any platform Rust supports

### Architecture

- `src/main.rs`: CLI entry point and command coordination
- `src/cli.rs`: Command-line argument parsing and validation  
- `src/iso_api.rs`: Microsoft API communication and session management
- `src/downloader.rs`: HTTP download implementation with progress tracking
- `src/types.rs`: Data structures for API responses and internal state
- `src/utils.rs`: Cross-platform utilities for locale/architecture detection

## Comparison with Fido

| Feature | Fido (PowerShell) | Ferro (Rust) |
|---------|-------------------|--------------|
| Platform Support | Windows only | Windows, macOS, Linux |
| Dependencies | PowerShell 3.0+ | None (single binary) |
| Performance | Interpreted | Compiled (faster) |
| Memory Usage | Higher (PowerShell runtime) | Lower (native binary) |
| Distribution | Script file | Single executable |
| GUI Mode | Yes | CLI only (for now) |


## Supported Downloads

### Windows Versions
- **Windows 11**: 24H2 (Build 26100.1742 - 2024.10)
- **Windows 10**: 22H2 v1 (Build 19045.2965 - 2023.05)
- **UEFI Shell 2.2**: Multiple versions from 25H1 to 20H2
- **UEFI Shell 2.0**: Version 4.632

### Languages
38+ languages including:
- English (US & International)
- Chinese (Simplified & Traditional)  
- Spanish, French, German, Italian
- Japanese, Korean, Arabic, Russian
- Portuguese, Dutch, Polish, Czech
- And many more...

### Architectures
- **x64** (64-bit Intel/AMD)
- **x86** (32-bit Intel/AMD)
- **ARM64** (64-bit ARM)

## Troubleshooting

### Common Issues

**Q: I'm getting a 715-123130 error**  
A: This is a temporary IP ban from Microsoft due to too many requests. Wait 1-24 hours and try again. This is normal behavior when testing multiple downloads.

**Q: Download is slow**  
A: Download speed depends on your internet connection and Microsoft's servers. Ferro shows progress bars to track download status.

**Q: Invalid architecture error**  
A: Make sure you're using the correct architecture name: "x64", "x86", or "ARM64" (case-sensitive).

### Debug Mode

For troubleshooting, run with debug logging:
```bash
RUST_LOG=debug ferro download --version "Windows 11"
```

## License

[GNU General Public License version 3.0](https://www.gnu.org/licenses/gpl-3.0) or later.

## Acknowledgments

Ferro is inspired by and based on the excellent [Fido PowerShell script](https://github.com/pbatard/Fido) by Pete Batard. This Rust implementation aims to provide the same functionality in a cross-platform, dependency-free package.

The original Fido documentation and README can be found in the `/README/` directory of this repository for reference.
