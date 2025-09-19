use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use reqwest::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::utils;

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout for downloads
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn download<P: AsRef<Path>>(&self, url: &str, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();
        
        info!("Starting download: {}", url);
        info!("Output file: {}", output_path.display());

        // Get file size first
        let head_response = self
            .client
            .head(url)
            .send()
            .await
            .context("Failed to get file information")?;

        let content_length = head_response
            .headers()
            .get("content-length")
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok());

        if let Some(size) = content_length {
            info!("File size: {}", utils::bytes_to_human_readable(size));
        }

        // Start the actual download
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to start download")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Download failed with status: {}",
                response.status()
            ));
        }

        // Create progress bar
        let progress_bar = if let Some(total_size) = content_length {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
                    .progress_chars("#>-"),
            );
            Some(pb)
        } else {
            warn!("Content-Length header not found, progress bar disabled");
            None
        };

        // Create output file
        let mut file = File::create(output_path)
            .await
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

        // Stream the download
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read chunk from response")?;
            file.write_all(&chunk)
                .await
                .context("Failed to write chunk to file")?;
            
            downloaded += chunk.len() as u64;
            
            if let Some(pb) = &progress_bar {
                pb.set_position(downloaded);
            }
        }

        // Ensure all data is written to disk
        file.flush().await.context("Failed to flush file")?;
        drop(file);

        if let Some(pb) = progress_bar {
            pb.finish_with_message("Download completed");
        }

        info!("Download completed successfully");
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_file_size(&self, url: &str) -> Result<Option<u64>> {
        let response = self
            .client
            .head(url)
            .send()
            .await
            .context("Failed to get file information")?;

        let content_length = response
            .headers()
            .get("content-length")
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok());

        Ok(content_length)
    }

    #[allow(dead_code)]
    pub async fn verify_url(&self, url: &str) -> Result<bool> {
        let response = self
            .client
            .head(url)
            .send()
            .await
            .context("Failed to verify URL")?;

        Ok(response.status().is_success())
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_downloader_creation() {
        let downloader = Downloader::new();
        // Just test that we can create a downloader without panicking
        assert!(std::mem::size_of_val(&downloader) > 0);
    }

    #[tokio::test]
    async fn test_verify_url_invalid() {
        let downloader = Downloader::new();
        let result = downloader.verify_url("https://httpbin.org/status/404").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_url_valid() {
        let downloader = Downloader::new();
        let result = downloader.verify_url("https://httpbin.org/status/200").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
