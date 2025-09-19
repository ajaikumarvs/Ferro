use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use rand;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::types::*;
use crate::utils;

pub struct IsoApi {
    client: Client,
    session_data: SessionData,
}

impl IsoApi {
    pub fn new() -> Result<Self> {
        // Create cookie store for session management
        let cookie_store = Arc::new(CookieStoreMutex::default());
        
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .redirect(reqwest::redirect::Policy::none()) // Don't follow redirects like Fido
            .timeout(Duration::from_secs(30))
            .cookie_provider(cookie_store.clone())
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
                headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());
                headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
                headers.insert("DNT", "1".parse().unwrap());
                headers.insert("Connection", "keep-alive".parse().unwrap());
                headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
                headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
                headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
                headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
                headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
                headers.insert("sec-ch-ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"".parse().unwrap());
                headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
                headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
                headers.insert("sec-ch-ua-platform-version", "\"15.0.0\"".parse().unwrap());
                headers.insert("sec-ch-ua-arch", "\"x86\"".parse().unwrap());
                headers.insert("sec-ch-ua-bitness", "\"64\"".parse().unwrap());
                headers.insert("sec-ch-ua-model", "\"\"".parse().unwrap());
                headers.insert("sec-ch-ua-full-version", "\"120.0.6099.109\"".parse().unwrap());
                headers.insert("viewport-width", "1920".parse().unwrap());
                headers.insert("sec-ch-viewport-width", "1920".parse().unwrap());
                headers.insert("sec-ch-dpr", "1".parse().unwrap());
                headers
            })
            .build()?;

        Ok(IsoApi {
            client,
            session_data: SessionData {
                session_id: Uuid::new_v4().to_string(),
                org_id: "y6jn8c31".to_string(),
                profile_id: "606624d44113c169".to_string(),
            },
        })
    }

    pub async fn get_available_versions(&self) -> Result<Vec<WindowsVersion>> {
        let versions = get_windows_versions();
        Ok(versions
            .into_iter()
            .enumerate()
            .map(|(index, version_data)| WindowsVersion {
                name: version_data.name,
                page_type: version_data.page_type,
                index,
            })
            .collect())
    }

    pub async fn get_releases(&self, version_name: &str) -> Result<Vec<WindowsRelease>> {
        let versions = get_windows_versions();
        let version_data = versions
            .iter()
            .find(|v| v.name.to_lowercase().contains(&version_name.to_lowercase()))
            .ok_or_else(|| anyhow!("Version '{}' not found", version_name))?;

        Ok(version_data
            .releases
            .iter()
            .enumerate()
            .map(|(index, release)| WindowsRelease {
                name: release.name.clone(),
                index,
            })
            .collect())
    }

    // Simulate visiting the main download page like a browser would
    async fn simulate_page_visit(&self, url: &str) -> Result<()> {
        debug!("Simulating page visit to: {}", url);
        
        let _response = self
            .client
            .get(url)
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .send()
            .await
            .context("Failed to visit page")?;
        
        debug!("Page visit completed");
        Ok(())
    }

    pub async fn get_editions(&self, version_name: &str, release_name: &str) -> Result<Vec<WindowsEdition>> {
        // First, simulate visiting the main download page like a browser would
        self.simulate_page_visit("https://www.microsoft.com/software-download/windows11").await?;
        
        // Add delay to simulate reading the page
        let delay = 1000 + (rand::random::<u64>() % 2000); // 1-3 seconds
        tokio::time::sleep(Duration::from_millis(delay)).await;
        
        let versions = get_windows_versions();
        let version_data = versions
            .iter()
            .find(|v| v.name.to_lowercase().contains(&version_name.to_lowercase()))
            .ok_or_else(|| anyhow!("Version '{}' not found", version_name))?;

        let release_data = version_data
            .releases
            .iter()
            .find(|r| r.name.to_lowercase().contains(&release_name.to_lowercase()))
            .ok_or_else(|| anyhow!("Release '{}' not found", release_name))?;

        Ok(release_data
            .editions
            .iter()
            .map(|edition| WindowsEdition {
                name: edition.name.clone(),
                id: edition.ids.clone(),
            })
            .collect())
    }

    pub async fn get_languages(&self, version_name: &str, release_name: &str, edition_name: &str) -> Result<Vec<WindowsLanguage>> {
        // Check if this is a UEFI Shell version
        if version_name.to_lowercase().contains("uefi") {
            return Ok(vec![WindowsLanguage {
                name: "en-us".to_string(),
                display_name: "English (US)".to_string(),
                data: vec![],
            }]);
        }

        let editions = self.get_editions(version_name, release_name).await?;
        let edition = editions
            .iter()
            .find(|e| e.name.to_lowercase().contains(&edition_name.to_lowercase()))
            .ok_or_else(|| anyhow!("Edition '{}' not found", edition_name))?;

        let mut languages = HashMap::new();
        
        for (session_index, &edition_id) in edition.id.iter().enumerate() {
            let session_id = Uuid::new_v4().to_string();
            
            // Whitelist session ID like Fido does
            self.whitelist_session(&session_id).await?;
            
            // Add randomized delay between requests to appear more human-like
            let delay = 500 + (rand::random::<u64>() % 1000); // 500-1500ms
            tokio::time::sleep(Duration::from_millis(delay)).await;
            
            // Get SKU information
            let languages_response = self.get_sku_information_with_retry(edition_id, &session_id).await?;
            
            if let Some(skus) = languages_response.skus {
                for sku in skus {
                    languages
                        .entry(sku.language.clone())
                        .or_insert_with(|| WindowsLanguage {
                            name: sku.language.clone(),
                            display_name: sku.localized_language.clone(),
                            data: vec![],
                        })
                        .data
                        .push(LanguageData {
                            session_index,
                            sku_id: sku.id,
                        });
                }
            }
        }

        Ok(languages.into_values().collect())
    }

    pub async fn get_architectures(&self, version_name: &str, release_name: &str, edition_name: &str, language_name: &str) -> Result<Vec<WindowsArchitecture>> {
        // Check if this is a UEFI Shell version
        if version_name.to_lowercase().contains("uefi") {
            return self.get_uefi_shell_architectures(version_name, release_name, edition_name).await;
        }

        let languages = self.get_languages(version_name, release_name, edition_name).await?;
        let language = languages
            .iter()
            .find(|l| l.name.to_lowercase().contains(&language_name.to_lowercase()) ||
                     l.display_name.to_lowercase().contains(&language_name.to_lowercase()))
            .ok_or_else(|| anyhow!("Language '{}' not found", language_name))?;

        let mut architectures = vec![];
        
        for language_data in &language.data {
            let session_id = Uuid::new_v4().to_string();
            // Whitelist session ID like Fido does
            self.whitelist_session(&session_id).await?;
            
            // Add randomized delay between requests
            let delay = 500 + (rand::random::<u64>() % 1000); // 500-1500ms
            tokio::time::sleep(Duration::from_millis(delay)).await;
            
            let download_links = self.get_download_links(&language_data.sku_id, &session_id).await?;
            
            if let Some(download_options) = download_links.product_download_options {
                for option in download_options {
                    let arch_name = utils::get_arch_from_type(option.download_type);
                    architectures.push(WindowsArchitecture {
                        name: arch_name,
                        url: option.uri,
                    });
                }
            }
        }

        Ok(architectures)
    }

    pub async fn get_download_url(&self, version_name: &str, release_name: &str, edition_name: &str, language_name: &str, architecture_name: &str) -> Result<String> {
        let architectures = self.get_architectures(version_name, release_name, edition_name, language_name).await?;
        
        let architecture = architectures
            .iter()
            .find(|a| a.name.to_lowercase() == architecture_name.to_lowercase())
            .ok_or_else(|| anyhow!("Architecture '{}' not found", architecture_name))?;

        Ok(architecture.url.clone())
    }

    async fn whitelist_session(&self, session_id: &str) -> Result<()> {
        let url = format!(
            "https://vlscppe.microsoft.com/tags?org_id={}&session_id={}",
            self.session_data.org_id, session_id
        );

        debug!("Whitelisting session: {}", url);
        
        // Like Fido, make the request but handle failures gracefully
        match self
            .client
            .get(&url)
            .header("Referer", "https://www.microsoft.com/software-download/windows11")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "cross-site")
            .header("X-Requested-With", "XMLHttpRequest")
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
        {
            Ok(_) => {
                debug!("Session whitelisting request completed successfully");
                Ok(())
            }
            Err(e) => {
                // Like Fido, log the error but don't fail completely
                // Sometimes the API calls work even if whitelisting fails
                warn!("Session whitelisting failed (continuing anyway): {}", e);
                Ok(()) // Continue processing despite whitelisting failure
            }
        }
    }

    async fn get_sku_information_with_retry(&self, product_edition_id: u32, session_id: &str) -> Result<MicrosoftApiResponse> {
        let mut retry_count = 0;
        let max_retries = 3;
        
        while retry_count < max_retries {
            match self.try_get_sku_information(product_edition_id, session_id, retry_count).await {
                Ok(response) => return Ok(response),
                Err(e) if retry_count < max_retries - 1 => {
                    let backoff_secs = 2u64.pow(retry_count + 1); // 2, 4, 8 seconds
                    warn!("SKU request failed (attempt {}), retrying in {} seconds: {}", retry_count + 1, backoff_secs, e);
                    tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                    retry_count += 1;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(anyhow!("Failed to get SKU information after {} attempts", max_retries))
    }
    
    async fn try_get_sku_information(&self, product_edition_id: u32, session_id: &str, attempt: u32) -> Result<MicrosoftApiResponse> {
        // Simulate JavaScript execution delay before API call
        let js_delay = 100 + (rand::random::<u64>() % 300); // 100-400ms like JS execution
        tokio::time::sleep(Duration::from_millis(js_delay)).await;
        
        // Vary the approach based on attempt number
        let (accept_header, user_agent, locale) = match attempt {
            0 => ("application/json, text/plain, */*", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36", "en-US"),
            1 => ("*/*", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36", "en-us"),
            _ => ("application/json", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0", "en-US"),
        };
        
        // Try different parameter variations
        let (sku_param, filename_param) = match attempt {
            0 => ("undefined", "undefined"),
            1 => ("", ""),
            _ => ("null", "null"),
        };
        
        let url = format!(
            "https://www.microsoft.com/software-download-connector/api/getskuinformationbyproductedition?profile={}&productEditionId={}&SKU={}&friendlyFileName={}&Locale={}&sessionID={}",
            self.session_data.profile_id, product_edition_id, sku_param, filename_param, locale, session_id
        );

        debug!("Getting SKU information (attempt {}): {}", attempt + 1, url);

        let referer = "https://www.microsoft.com/software-download/windows11";
        let mut request = self
            .client
            .get(&url)
            .header("Referer", referer)
            .header("Accept", accept_header)
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("User-Agent", user_agent);
            
        // Add more browser-like headers for later attempts
        if attempt > 0 {
            request = request
                .header("Cache-Control", "no-cache")
                .header("Pragma", "no-cache")
                .header("Sec-Fetch-Dest", "empty")
                .header("Sec-Fetch-Mode", "cors")
                .header("Sec-Fetch-Site", "same-site");
        }
        
        // Add XMLHttpRequest header only for first two attempts
        if attempt < 2 {
            request = request.header("X-Requested-With", "XMLHttpRequest");
        }
        
        let response = request
            .send()
            .await
            .context("Failed to get SKU information")?;

        let status = response.status(); 
        let headers = response.headers().clone();
        debug!("SKU API response status: {}", status);
        debug!("SKU API response headers: {:?}", headers);

        let response_text = response.text().await.context("Failed to get response text")?;
        debug!("SKU information response (length {}): {}", response_text.len(), response_text);
        
        // Save response to file for debugging
        if let Err(e) = std::fs::write("api_response.json", &response_text) {
            debug!("Failed to write response to file: {}", e);
        }

        if response_text.trim().is_empty() {
            return Err(anyhow!("API returned empty response. Status: {}. This might indicate that the API is blocking our requests or requires additional authentication.", status));
        }
        
        let api_response: MicrosoftApiResponse = serde_json::from_str(&response_text)
            .with_context(|| format!("Failed to parse SKU information response. Response was: {}", response_text))?;

        if let Some(errors) = &api_response.errors {
            if !errors.is_empty() {
                return Err(anyhow!("API error: {}", errors[0].value));
            }
        }

        Ok(api_response)
    }

    async fn get_download_links(&self, sku_id: &str, session_id: &str) -> Result<MicrosoftApiResponse> {
        let url = format!(
            "https://www.microsoft.com/software-download-connector/api/GetProductDownloadLinksBySku?profile={}&productEditionId=undefined&SKU={}&friendlyFileName=undefined&Locale=en-US&sessionID={}",
            self.session_data.profile_id, sku_id, session_id
        );

        debug!("Getting download links: {}", url);

        let referer = "https://www.microsoft.com/software-download/windows11";
        let response = self
            .client
            .get(&url)
            .header("Referer", referer)
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-site")
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Origin", "https://www.microsoft.com")
            .send()
            .await
            .context("Failed to get download links")?;

        let api_response: MicrosoftApiResponse = response
            .json()
            .await
            .context("Failed to parse download links response")?;

        if let Some(errors) = &api_response.errors {
            if !errors.is_empty() {
                if errors[0].error_type == 9 {
                    return Err(anyhow!("Your IP has been banned by Microsoft (code 715-123130). Session: {}", session_id));
                }
                return Err(anyhow!("API error: {}", errors[0].value));
            }
        }

        Ok(api_response)
    }

    async fn get_uefi_shell_architectures(&self, version_name: &str, release_name: &str, edition_name: &str) -> Result<Vec<WindowsArchitecture>> {
        // Extract version info for UEFI Shell
        let tag = release_name.split(' ').next().unwrap_or("25H1");
        let shell_version = version_name.split(' ').last().unwrap_or("2.2");
        
        let base_url = format!("https://github.com/pbatard/UEFI-Shell/releases/download/{}", tag);
        let link_base = format!("{}/UEFI-Shell-{}-{}", base_url, shell_version, tag);
        
        let link = if edition_name.to_lowercase().contains("release") {
            format!("{}-RELEASE.iso", link_base)
        } else {
            format!("{}-DEBUG.iso", link_base)
        };

        // Try to get supported architectures from Version.xml
        let version_url = format!("{}/Version.xml", base_url);
        
        match self.client.get(&version_url).send().await {
            Ok(response) if response.status().is_success() => {
                if let Ok(xml_content) = response.text().await {
                    // Simple XML parsing for architectures
                    let archs = self.parse_uefi_architectures(&xml_content);
                    if !archs.is_empty() {
                        return Ok(vec![WindowsArchitecture {
                            name: archs.join(", "),
                            url: link,
                        }]);
                    }
                }
            }
            _ => {
                warn!("Could not fetch UEFI Shell version information");
            }
        }

        // Fallback to default architectures
        Ok(vec![WindowsArchitecture {
            name: "x64, ARM64, IA32".to_string(),
            url: link,
        }])
    }

    fn parse_uefi_architectures(&self, xml_content: &str) -> Vec<String> {
        // Simple regex-based XML parsing for <arch> elements
        let arch_regex = regex::Regex::new(r"<arch>([^<]+)</arch>").unwrap();
        arch_regex
            .captures_iter(xml_content)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    #[allow(dead_code)]
    pub async fn check_locale(&self, locale: &str) -> Result<bool> {
        let url = format!("https://www.microsoft.com/{}/software-download/", locale);
        
        debug!("Checking locale: {}", url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
