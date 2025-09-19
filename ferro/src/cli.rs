use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ferro")]
#[command(about = "A cross-platform Windows ISO downloader - Rust rewrite of Fido")]
#[command(version = "0.1.0")]
#[command(author = "Ferro Contributors")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List available items
    List {
        #[command(subcommand)]
        item_type: ListType,
    },
    /// Download Windows ISO
    Download {
        #[command(flatten)]
        options: DownloadOptions,
    },
}

#[derive(Subcommand)]
pub enum ListType {
    /// List available Windows versions
    Versions,
    /// List available releases for a Windows version
    Releases { version: String },
    /// List available editions for a Windows version and release
    Editions { version: String, release: String },
    /// List available languages for a Windows version, release, and edition
    Languages { version: String, release: String, edition: String },
    /// List available architectures for a Windows version, release, edition, and language
    Architectures { version: String, release: String, edition: String, language: String },
}

#[derive(clap::Args)]
pub struct DownloadOptions {
    /// Windows version (e.g., "Windows 11", "Windows 10")
    #[arg(short = 'w', long)]
    pub version: Option<String>,
    
    /// Windows release (e.g., "24H2", "22H2")
    #[arg(short = 'r', long)]
    pub release: Option<String>,
    
    /// Windows edition (e.g., "Home/Pro/Edu", "Pro")
    #[arg(short = 'e', long)]
    pub edition: Option<String>,
    
    /// Language (e.g., "English", "en-US")
    #[arg(short = 'l', long)]
    pub language: Option<String>,
    
    /// Architecture (e.g., "x64", "x86", "ARM64")
    #[arg(short = 'a', long)]
    pub architecture: Option<String>,
    
    /// Output file path
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,
    
    /// Only get download URL without downloading
    #[arg(long)]
    pub get_url: bool,
}
