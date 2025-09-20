# Ferro: A Cross-Platform Windows ISO Downloader

[![License](https://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square)](https://www.gnu.org/licenses/gpl-3.0.en.html)

## Description

Ferro is a cross-platform Windows ISO downloader written in Rust, inspired by and based on the excellent [Fido PowerShell script](https://github.com/pbatard/Fido) by Pete Batard. Ferro automates access to official Microsoft Windows retail ISO download links and provides convenient access to [bootable UEFI Shell images](https://github.com/pbatard/UEFI-Shell).

**Note:** The original Fido documentation and README can be found in the `/README/` directory of this repository.

This tool exists because, while Microsoft does make retail ISO download links freely and publicly available (at least for Windows 8 through Windows 11), these links were historically only available after forcing users to jump through many unwarranted hoops that created an exceedingly counterproductive consumer experience.

### Why Retail ISOs?

Using official retail ISOs is currently the only way to assert with absolute certainty that the OS content has not been altered. Because there exists only a single master for each retail ISO, Microsoft retail ISOs are the only ones you can obtain an official SHA-1 for (from MSDN or sites like [msdn.rg-adguard.net](https://msdn.rg-adguard.net/public.php)), allowing you to be 100% sure that the image you are using has not been corrupted and is safe to use.

Unlike Microsoft's Media Creation Tool (MCT), which generates ISOs on-the-fly (making each one unique), retail ISOs provide verifiable, bit-for-bit identical content that matches Microsoft's original release.

## Key Features

- **Cross-platform**: Works on Windows, macOS, and Linux without PowerShell dependency
- **Official ISOs**: Downloads genuine Microsoft retail ISOs with verifiable checksums  
- **Multiple Windows versions**: Supports Windows 10, Windows 11, and UEFI Shell images
- **Command-line interface**: Full CLI support with flexible options
- **Smart defaults**: Automatically selects appropriate language and architecture based on system
- **Progress tracking**: Real-time download progress with speed and ETA information
- **URL-only mode**: Option to retrieve download URLs without downloading

## Installation

### From Source
```bash
git clone https://github.com/ajaikumarvs/Ferro.git
cd ferro
cargo build --release
./target/release/ferro --help
```

### Prerequisites
- Rust 1.70 or later
- Internet connection for downloading ISOs

## Usage

### List Available Options

```bash
# List available Windows versions
ferro list versions

# List releases for Windows 11
ferro list releases "Windows 11"

# List editions for Windows 11 24H2
ferro list editions "Windows 11" "24H2"

# List languages for Windows 11 24H2 Home/Pro/Edu
ferro list languages "Windows 11" "24H2" "Windows 11 Home/Pro/Edu"

# List architectures for Windows 11 24H2 Home/Pro/Edu English
ferro list architectures "Windows 11" "24H2" "Windows 11 Home/Pro/Edu" "English"
```

### Download Windows ISOs

```bash
# Download latest Windows 11 with system defaults
ferro download

# Download specific Windows 10 version
ferro download -w "Windows 10" -r "22H2" -e "Home/Pro" -l "English" -a "x64"

# Get download URL only (without downloading)
ferro download -w "Windows 11" --get-url

# Specify custom output path
ferro download -w "Windows 11" -o /path/to/windows11.iso
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

## Current Status

**Note:** As of 2023.05, Microsoft has removed access to older releases of Windows ISOs, and the list of releases that can be downloaded has been reduced to only the latest for each version.

Ferro is actively being developed to match Fido's functionality. Current implementation status:
- ✅ CLI interface and argument parsing
- ✅ Microsoft API structure understanding
- ✅ Session management implementation
- 🔄 Anti-automation bypass (work in progress)
- 🔄 Full download functionality (debugging in progress)

## License

[GNU General Public License version 3.0](https://www.gnu.org/licenses/gpl-3.0) or later.

## Acknowledgments

Ferro is inspired by and based on the excellent [Fido PowerShell script](https://github.com/pbatard/Fido) by Pete Batard. This Rust implementation aims to provide the same functionality in a cross-platform, dependency-free package.

The original Fido documentation and README can be found in the `/README/` directory of this repository for reference.

## Additional Notes

Because of its intended usage similar to Fido, this tool is not designed to cover every possible retail ISO download. Instead, we focus on the downloads that the general public is likely to request. We currently have no plans to add support for LTSB/LTSC Windows ISO downloads.

If you are interested in such downloads, please visit the relevant download pages from Microsoft such as [this one](https://www.microsoft.com/evalcenter/evaluate-windows-10-enterprise) for LTSC versions.
