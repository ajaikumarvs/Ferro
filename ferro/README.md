# Ferro - Cross-Platform Windows ISO Downloader

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Cross Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)](https://github.com/ajaikumarvs/Ferro)

Ferro is a cross-platform Windows ISO downloader written in Rust - a complete rewrite of the popular [Fido PowerShell script](https://github.com/pbatard/Fido). It provides a clean, modern CLI interface for downloading official Microsoft Windows ISOs and UEFI Shell images without requiring PowerShell or Windows.

## Features

✅ **Cross-Platform**: Works on Windows, macOS, and Linux  
✅ **No Dependencies**: Pure Rust implementation with no PowerShell requirement  
✅ **Official ISOs**: Downloads genuine Microsoft Windows retail ISOs  
✅ **Multiple Versions**: Supports Windows 11, Windows 10, and UEFI Shell  
✅ **Complete Selection**: Version → Release → Edition → Language → Architecture  
✅ **Modern CLI**: Clean command-line interface with helpful error messages  
✅ **Progress Tracking**: Download progress bars and speed indicators  
✅ **Session Management**: Proper Microsoft API integration with anti-ban measures  
✅ **Locale Detection**: Automatic system locale detection with fallback support  
✅ **1:1 Accuracy**: Exact replication of Fido's behavior and API calls  

## Supported Downloads

### Windows Versions
- **Windows 11**: 24H2 (Build 26100.1742 - 2024.10)
- **Windows 10**: 22H2 v1 (Build 19045.2965 - 2023.05)
- **UEFI Shell 2.2**: Multiple versions from 25H1 to 20H2
- **UEFI Shell 2.0**: Version 4.632

### Editions
- Windows 11/10 Home/Pro/Education
- Windows 11/10 China variants (when applicable)
- UEFI Shell Release/Debug builds

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

## Installation

### From Source
```bash
git clone https://github.com/ajaikumarvs/Ferro.git
cd Ferro/ferro
cargo build --release
```

The binary will be available at `target/release/ferro`.

### Prerequisites
- Rust 1.70 or later
- Internet connection for downloading ISOs

## Usage

Ferro provides two main commands: `list` for exploring available options and `download` for getting ISOs.

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

### Exploring Available Options

```bash
# Show all available Windows versions
ferro list versions

# Show releases for Windows 11
ferro list releases "Windows 11"

# Show editions for a specific version and release
ferro list editions "Windows 11" "24H2 (Build 26100.1742 - 2024.10)"

# Show available languages
ferro list languages "Windows 11" "24H2 (Build 26100.1742 - 2024.10)" "Windows 11 Home/Pro/Edu"

# Show available architectures
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
  --release "24H2 (Build 26100.1742 - 2024.10)" \
  --edition "Windows 11 Home/Pro/Edu" \
  --language "English" \
  --architecture "x64"

# Download ARM64 version for Apple Silicon Macs
ferro download \
  --version "Windows 11" \
  --architecture "ARM64" \
  --output "Windows11_ARM64.iso"
```

#### Windows 10 Downloads

```bash
# Download Windows 10
ferro download \
  --version "Windows 10" \
  --release "22H2 v1 (Build 19045.2965 - 2023.05)" \
  --edition "Windows 10 Home/Pro/Edu" \
  --language "English" \
  --architecture "x64" \
  --output "Windows10_22H2_x64.iso"

# Windows 10 with different language
ferro download \
  --version "Windows 10" \
  --language "Spanish" \
  --output "Windows10_Spanish.iso"
```

#### UEFI Shell Downloads

```bash
# List UEFI Shell versions
ferro list versions

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

### Command Reference

#### List Command
```bash
ferro list <SUBCOMMAND>

Subcommands:
  versions                                   List all available Windows versions
  releases <VERSION>                         List releases for a Windows version  
  editions <VERSION> <RELEASE>               List editions for version and release
  languages <VERSION> <RELEASE> <EDITION>   List available languages
  architectures <VERSION> <RELEASE> <EDITION> <LANGUAGE>  List architectures
```

#### Download Command
```bash
ferro download [OPTIONS]

Options:
  -w, --version <VERSION>            Windows version (e.g., "Windows 11", "Windows 10")
  -r, --release <RELEASE>            Windows release (e.g., "24H2", "22H2")
  -e, --edition <EDITION>            Windows edition (e.g., "Home/Pro/Edu", "Pro")
  -l, --language <LANGUAGE>          Language (e.g., "English", "Spanish")
  -a, --architecture <ARCHITECTURE>  Arch (e.g., "x64", "x86", "ARM64")
  -o, --output <OUTPUT>              Output file path
      --get-url                      Only get download URL without downloading
  -h, --help                         Print help
```

## Technical Details

### API Integration
Ferro replicates Fido's exact behavior for Microsoft API integration:
- Uses Microsoft's `software-download-connector` API
- Implements proper session whitelisting via `vlscppe.microsoft.com/tags`
- Maintains session ID management across multiple API calls
- Includes proper error handling for IP bans (code 715-123130)
- Uses correct referer headers and user agent strings

### Anti-Ban Measures
- Randomized delays between API requests (500-1500ms)
- Proper session ID reuse patterns
- Locale detection and validation
- Request rate limiting
- Authentic browser-like request patterns

### Cross-Platform Features
- System locale detection on all platforms
- Native architecture detection (x64, ARM64, etc.)
- Cross-platform file path handling
- Unicode support for international content

## Error Handling

Ferro provides clear error messages for common issues:

```bash
# IP ban detection (temporary, usually resolves in a few hours)
Error: Your IP address has been banned by Microsoft for issuing too many ISO 
download requests or for belonging to a region of the world where sanctions 
currently apply. Please try again later.
If you believe this ban to be in error, you can try contacting Microsoft by 
referring to message code 715-123130 and session ID <session-id>

# Invalid version/release/edition
Error: Version 'Windows 12' not found

# Network issues
Error: Failed to connect to Microsoft servers
```

## Troubleshooting

### Common Issues

**Q: I'm getting a 715-123130 error**  
A: This is a temporary IP ban from Microsoft due to too many requests. Wait 1-24 hours and try again. This is normal behavior when testing multiple downloads.

**Q: Download is slow**  
A: Download speed depends on your internet connection and Microsoft's servers. Ferro shows progress bars to track download status.

**Q: Invalid architecture error**  
A: Make sure you're using the correct architecture name: "x64", "x86", or "ARM64" (case-sensitive).

**Q: Language not found**  
A: Use `ferro list languages` to see available languages for your selected version/release/edition.

### Debug Mode

For troubleshooting, run with debug logging:
```bash
RUST_LOG=debug ferro download --version "Windows 11"
```

This will show detailed API calls, session management, and response handling.

## Comparison with Fido

| Feature | Fido (PowerShell) | Ferro (Rust) |
|---------|-------------------|--------------|
| Cross-Platform | ❌ Windows only | ✅ Windows, macOS, Linux |
| Dependencies | ❌ Requires PowerShell | ✅ Single binary |
| Performance | ⚠️ Slower startup | ✅ Fast startup |
| CLI Interface | ⚠️ Limited | ✅ Modern, user-friendly |
| API Accuracy | ✅ Original | ✅ 1:1 replica |
| Session Management | ✅ Advanced | ✅ Exact replica |
| Error Handling | ✅ Comprehensive | ✅ Enhanced |

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### Development Setup

```bash
git clone https://github.com/ajaikumarvs/Ferro.git
cd Ferro/ferro
cargo build
cargo test
```

## License

Ferro is licensed under the GNU General Public License v3.0 or later - see the [LICENSE](../LICENSE) file for details.

This project is a rewrite of [Fido](https://github.com/pbatard/Fido) by Pete Batard, maintaining the same GPL v3.0 license.

## Acknowledgments

- **Pete Batard** - Original Fido PowerShell script
- **flx5** - Command line support contributions to Fido
- **Chris Carter** - ConvertTo-ImageSource function
- **Microsoft** - Windows ISO distribution infrastructure

## Disclaimer

Ferro downloads official Microsoft Windows ISOs from Microsoft's servers. Users must comply with Microsoft's terms of service and licensing agreements. This tool is for convenience and does not modify or bypass any Microsoft licensing requirements.

---

**Made with ❤️ in Rust**