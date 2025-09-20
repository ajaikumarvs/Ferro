use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::path::PathBuf;

mod cli;
mod downloader;
mod iso_api;
mod types;
mod utils;

use crate::cli::Cli;
use crate::downloader::Downloader;
use crate::iso_api::IsoApi;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match run(cli).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn run(cli: Cli) -> Result<()> {
    let mut api = IsoApi::new().await?;
    
    match cli.command {
        Some(crate::cli::Commands::List { item_type }) => {
            handle_list_command(item_type, &mut api).await
        }
        Some(crate::cli::Commands::Download { options }) => {
            handle_download_command(options, &mut api).await
        }
        None => {
            // Interactive mode - for future implementation
            eprintln!("Interactive mode not yet implemented. Use --help for available commands.");
            Ok(())
        }
    }
}

async fn handle_list_command(item_type: crate::cli::ListType, api: &mut IsoApi) -> Result<()> {
    match item_type {
        crate::cli::ListType::Versions => {
            let versions = api.get_available_versions().await?;
            println!("Available Windows versions:");
            for version in versions {
                println!("  - {}", version.name);
            }
        }
        crate::cli::ListType::Releases { version } => {
            let releases = api.get_releases(&version).await?;
            println!("Available releases for {}:", version);
            for release in releases {
                println!("  - {}", release.name);
            }
        }
        crate::cli::ListType::Editions { version, release } => {
            let editions = api.get_editions(&version, &release).await?;
            println!("Available editions for {} {}:", version, release);
            for edition in editions {
                println!("  - {}", edition.name);
            }
        }
        crate::cli::ListType::Languages { version, release, edition } => {
            let languages = api.get_languages(&version, &release, &edition).await?;
            println!("Available languages for {} {} {}:", version, release, edition);
            for language in languages {
                println!("  - {} ({})", language.display_name, language.name);
            }
        }
        crate::cli::ListType::Architectures { version, release, edition, language } => {
            let architectures = api.get_architectures(&version, &release, &edition, &language).await?;
            println!("Available architectures for {} {} {} {}:", version, release, edition, language);
            for arch in architectures {
                println!("  - {}", arch.name);
            }
        }
    }
    Ok(())
}

async fn handle_download_command(options: crate::cli::DownloadOptions, api: &mut IsoApi) -> Result<()> {
    info!("Starting download process...");
    
    // Resolve defaults if not specified
    let version = options.version.unwrap_or_else(|| "Windows 11".to_string());
    let release = if let Some(r) = options.release {
        r
    } else {
        let releases = api.get_releases(&version).await?;
        releases.first()
            .context("No releases found")?.
            name.clone()
    };
    
    let edition = if let Some(e) = options.edition {
        e
    } else {
        let editions = api.get_editions(&version, &release).await?;
        editions.first()
            .context("No editions found")?.
            name.clone()
    };
    
    let language = if let Some(l) = options.language {
        l
    } else {
        let languages = api.get_languages(&version, &release, &edition).await?;
        // Try to find system locale or default to English
        let system_locale = utils::get_system_locale();
        languages.iter()
            .find(|lang| lang.name.starts_with(&system_locale))
            .or_else(|| languages.iter().find(|lang| lang.name.starts_with("en")))
            .or_else(|| languages.first())
            .context("No languages found")?.
            name.clone()
    };
    
    let architecture = if let Some(a) = options.architecture {
        a
    } else {
        let archs = api.get_architectures(&version, &release, &edition, &language).await?;
        let system_arch = utils::get_system_architecture();
        archs.iter()
            .find(|arch| arch.name == system_arch)
            .or_else(|| archs.first())
            .context("No architectures found")?.
            name.clone()
    };
    
    println!("Selected: {} {} {} {} {}", version, release, edition, language, architecture);
    
    let download_url = api.get_download_url(&version, &release, &edition, &language, &architecture).await?;
    
    if options.get_url {
        println!("{}", download_url);
        return Ok(());
    }
    
    let downloader = Downloader::new();
    let output_path = options.output.unwrap_or_else(|| {
        let filename = utils::extract_filename_from_url(&download_url)
            .unwrap_or_else(|| format!("{}_{}_{}_{}.iso", version.replace(" ", ""), release, language, architecture));
        PathBuf::from(filename)
    });
    
    downloader.download(&download_url, &output_path).await?;
    
    println!("Download completed: {}", output_path.display());
    Ok(())
}
