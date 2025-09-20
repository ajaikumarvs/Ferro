use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::types::*;
use crate::utils;

pub struct IsoApi {
    client: Client,
    session_data: SessionData,
    session_ids: HashMap<usize, String>, // Store session IDs by index for reuse like Fido
    query_locale: String, // $QueryLocale like Fido - can be different from system locale
}

impl IsoApi {
    pub async fn new() -> Result<Self> {
        // Create simple client like PowerShell's Invoke-RestMethod with -UseBasicParsing
        let cookie_store = Arc::new(CookieStoreMutex::default());

        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) WindowsPowerShell/5.1.19041.4170",
            )
            .redirect(reqwest::redirect::Policy::none()) // MaximumRedirection 0 like Fido
            .timeout(Duration::from_secs(30)) // DefaultTimeout like Fido
            .cookie_provider(cookie_store.clone())
            .build()?;

        let mut api = IsoApi {
            client,
            session_data: SessionData {
                session_id: Uuid::new_v4().to_string(),
                org_id: "y6jn8c31".to_string(),
                profile_id: "606624d44113".to_string(), // Matches Fido exactly
            },
            session_ids: HashMap::new(),
            query_locale: "en-US".to_string(), // Default, will be validated
        };

        // Check and set proper locale like Fido's Check-Locale function
        api.check_and_set_locale().await?;

        Ok(api)
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
    #[allow(dead_code)]
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

    pub async fn get_editions(
        &self,
        version_name: &str,
        release_name: &str,
    ) -> Result<Vec<WindowsEdition>> {
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

    pub async fn get_languages(
        &mut self,
        version_name: &str,
        release_name: &str,
        edition_name: &str,
    ) -> Result<Vec<WindowsLanguage>> {
        // Check if this is a UEFI Shell version
        if version_name.to_lowercase().contains("uefi") {
            return Ok(vec![WindowsLanguage {
                name: "en-us".to_string(),
                display_name: "English (US)".to_string(),
                data: vec![LanguageData {
                    session_index: 0,
                    sku_id: "1".to_string(),
                }],
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

            // Store the session ID for later reuse (like Fido does)
            self.session_ids.insert(session_index, session_id.clone());

            // Whitelist session ID like Fido does
            self.whitelist_session(&session_id).await?;

            // Add randomized delay between requests to appear more human-like
            let delay = 500 + (uuid::Uuid::new_v4().as_u128() % 1000) as u64; // 500-1500ms
            tokio::time::sleep(Duration::from_millis(delay)).await;

            // Get SKU information using exact Fido approach
            let languages_response = self
                .try_get_sku_information(edition_id, &session_id, 0)
                .await?;

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

        // Store session IDs in a way that can be accessed later
        // For now, we'll need to modify the approach to pass session IDs through

        Ok(languages.into_values().collect())
    }

    pub async fn get_architectures(
        &mut self,
        version_name: &str,
        release_name: &str,
        edition_name: &str,
        language_name: &str,
    ) -> Result<Vec<WindowsArchitecture>> {
        // Check if this is a UEFI Shell version
        if version_name.to_lowercase().contains("uefi") {
            return self
                .get_uefi_shell_architectures(version_name, release_name, edition_name)
                .await;
        }

        let languages = self
            .get_languages(version_name, release_name, edition_name)
            .await?;
        let language = languages
            .iter()
            .find(|l| {
                l.name
                    .to_lowercase()
                    .contains(&language_name.to_lowercase())
                    || l.display_name
                        .to_lowercase()
                        .contains(&language_name.to_lowercase())
            })
            .ok_or_else(|| anyhow!("Language '{}' not found", language_name))?;

        let mut architectures = vec![];

        for language_data in &language.data {
            // Reuse the session ID from the SKU information call (like Fido does with $SessionId[$Entry.SessionIndex])
            // Don't create a new session or whitelist again - reuse existing session

            // Add randomized delay between requests
            let delay = 500 + (uuid::Uuid::new_v4().as_u128() % 1000) as u64; // 500-1500ms
            tokio::time::sleep(Duration::from_millis(delay)).await;

            // Get the stored session ID for this session index
            let session_id = self
                .session_ids
                .get(&language_data.session_index)
                .ok_or_else(|| {
                    anyhow!(
                        "Session ID not found for index {}",
                        language_data.session_index
                    )
                })?;

            let download_links = self
                .get_download_links(&language_data.sku_id, session_id)
                .await?;

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

    pub async fn get_download_url(
        &mut self,
        version_name: &str,
        release_name: &str,
        edition_name: &str,
        language_name: &str,
        architecture_name: &str,
    ) -> Result<String> {
        let architectures = self
            .get_architectures(version_name, release_name, edition_name, language_name)
            .await?;

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

        // Exact replication of Fido: Invoke-WebRequest -UseBasicParsing -TimeoutSec $DefaultTimeout -MaximumRedirection 0 $url | Out-Null
        match self.client.get(&url).send().await {
            Ok(_) => {
                debug!("Session whitelisting request completed successfully");
                Ok(())
            }
            Err(e) => {
                // Like Fido: catch { Error($_.Exception.Message); return @() }
                Err(anyhow!("Session whitelisting failed: {}", e))
            }
        }
    }

    #[allow(dead_code)]
    async fn get_sku_information_with_retry(
        &self,
        product_edition_id: u32,
        session_id: &str,
    ) -> Result<MicrosoftApiResponse> {
        let mut retry_count = 0;
        let max_retries = 3;

        while retry_count < max_retries {
            match self
                .try_get_sku_information(product_edition_id, session_id, retry_count)
                .await
            {
                Ok(response) => return Ok(response),
                Err(e) if retry_count < max_retries - 1 => {
                    let backoff_secs = 2u64.pow(retry_count + 1); // 2, 4, 8 seconds
                    warn!(
                        "SKU request failed (attempt {}), retrying in {} seconds: {}",
                        retry_count + 1,
                        backoff_secs,
                        e
                    );
                    tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                    retry_count += 1;
                }
                Err(e) => return Err(e),
            }
        }

        Err(anyhow!(
            "Failed to get SKU information after {} attempts",
            max_retries
        ))
    }

    async fn try_get_sku_information(
        &self,
        product_edition_id: u32,
        session_id: &str,
        attempt: u32,
    ) -> Result<MicrosoftApiResponse> {
        // Use exact same URL format as Fido with $QueryLocale
        let url = format!(
            "https://www.microsoft.com/software-download-connector/api/getskuinformationbyproductedition?profile={}&productEditionId={}&SKU=undefined&friendlyFileName=undefined&Locale={}&sessionID={}",
            self.session_data.profile_id, product_edition_id, self.query_locale, session_id
        );

        debug!("Getting SKU information (attempt {}): {}", attempt + 1, url);

        // Use minimal headers like Fido's -UseBasicParsing
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get SKU information")?;

        let status = response.status();
        let headers = response.headers().clone();
        debug!("SKU API response status: {}", status);
        debug!("SKU API response headers: {:?}", headers);

        let response_text = response
            .text()
            .await
            .context("Failed to get response text")?;
        debug!(
            "SKU information response (length {}): {}",
            response_text.len(),
            response_text
        );

        // Save response to file for debugging
        if let Err(e) = std::fs::write("api_response.json", &response_text) {
            debug!("Failed to write response to file: {}", e);
        }

        if response_text.trim().is_empty() {
            return Err(anyhow!("API returned empty response. Status: {}. This might indicate that the API is blocking our requests or requires additional authentication.", status));
        }

        let api_response: MicrosoftApiResponse = serde_json::from_str(&response_text)
            .with_context(|| {
                format!(
                    "Failed to parse SKU information response. Response was: {}",
                    response_text
                )
            })?;

        // Check for errors in ValidationContainer (newer API format)
        if let Some(validation_container) = &api_response.validation_container {
            debug!(
                "ValidationContainer errors count: {}",
                validation_container.errors.len()
            );
            if !validation_container.errors.is_empty() {
                return Err(anyhow!("API error: {:?}", validation_container.errors[0]));
            }
        }

        // Check for legacy errors format
        if let Some(errors) = &api_response.errors {
            debug!("Legacy errors count: {}", errors.len());
            if !errors.is_empty() {
                return Err(anyhow!("API error: {}", errors[0].value));
            }
        }

        debug!(
            "No API errors found, SKUs count: {:?}",
            api_response.skus.as_ref().map(|s| s.len())
        );

        Ok(api_response)
    }

    async fn get_download_links(
        &self,
        sku_id: &str,
        session_id: &str,
    ) -> Result<MicrosoftApiResponse> {
        let url = format!(
            "https://www.microsoft.com/software-download-connector/api/GetProductDownloadLinksBySku?profile={}&productEditionId=undefined&SKU={}&friendlyFileName=undefined&Locale={}&sessionID={}",
            self.session_data.profile_id, sku_id, self.query_locale, session_id
        );

        debug!("Getting download links: {}", url);

        // Must add a referer for this request, else Microsoft's servers may deny it (from Fido comment)
        let referer = "https://www.microsoft.com/software-download/windows11";
        let response = self
            .client
            .get(&url)
            .header("Referer", referer)
            .send()
            .await
            .context("Failed to get download links")?;

        let response_text = response
            .text()
            .await
            .context("Failed to get download links response text")?;
        debug!(
            "Download links response (length {}): {}",
            response_text.len(),
            response_text
        );

        // Save response to file for debugging
        if let Err(e) = std::fs::write("download_links_response.json", &response_text) {
            debug!("Failed to write download links response to file: {}", e);
        }

        let api_response: MicrosoftApiResponse = serde_json::from_str(&response_text)
            .with_context(|| {
                format!(
                    "Failed to parse download links response. Response was: {}",
                    response_text
                )
            })?;

        // Check for errors in ValidationContainer first (newer API format)
        if let Some(validation_container) = &api_response.validation_container {
            if !validation_container.errors.is_empty() {
                return Err(anyhow!("API error: {:?}", validation_container.errors[0]));
            }
        }

        // Check for legacy errors format (like Fido does)
        if let Some(errors) = &api_response.errors {
            if !errors.is_empty() {
                if errors[0].error_type == 9 {
                    let ban_message = self.get_code_715_123130_message().await;
                    return Err(anyhow!("{} {}", ban_message, session_id));
                }
                return Err(anyhow!("API error: {}", errors[0].value));
            }
        }

        Ok(api_response)
    }

    async fn get_uefi_shell_architectures(
        &self,
        version_name: &str,
        release_name: &str,
        edition_name: &str,
    ) -> Result<Vec<WindowsArchitecture>> {
        // Extract version info for UEFI Shell
        let tag = release_name.split(' ').next().unwrap_or("25H1");
        let shell_version = version_name.split(' ').next_back().unwrap_or("2.2");

        let base_url = format!(
            "https://github.com/pbatard/UEFI-Shell/releases/download/{}",
            tag
        );
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

    // Check if the locale we want is available - Fall back to en-US otherwise (like Fido)
    async fn check_and_set_locale(&mut self) -> Result<()> {
        let system_locale = utils::get_system_locale();

        // Try system locale first
        if self.check_locale(&system_locale).await? {
            self.query_locale = system_locale;
            debug!("Using system locale: {}", self.query_locale);
        } else {
            self.query_locale = "en-US".to_string();
            debug!("Falling back to en-US locale");
        }

        Ok(())
    }

    async fn check_locale(&self, locale: &str) -> Result<bool> {
        let url = format!("https://www.microsoft.com/{}/software-download/", locale);

        debug!("Checking locale: {}", url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => {
                debug!("Locale check failed for: {}", locale);
                Ok(false)
            }
        }
    }

    // Get the 715-123130 ban message like Fido does
    async fn get_code_715_123130_message(&self) -> String {
        let url = format!(
            "https://www.microsoft.com/{}/software-download/windows11",
            self.query_locale
        );

        if let Ok(response) = self.client.get(&url).send().await {
            if let Ok(html) = response.text().await {
                // Try to extract the actual ban message from HTML like Fido does
                let pattern = r#"<input id="msg-01" type="hidden" value="(.*?)"/>"#;
                if let Ok(re) = regex::Regex::new(pattern) {
                    if let Some(captures) = re.captures(&html) {
                        if let Some(msg) = captures.get(1) {
                            let msg = msg
                                .as_str()
                                .replace("&lt;", "<")
                                .replace("&gt;", ">")
                                .replace("&amp;", "&");
                            // Remove HTML tags and clean up whitespace
                            let clean_msg =
                                regex::Regex::new(r"<[^>]+>").unwrap().replace_all(&msg, "");
                            let clean_msg = regex::Regex::new(r"\s+")
                                .unwrap()
                                .replace_all(&clean_msg, " ");
                            if clean_msg.contains("715-123130") {
                                return clean_msg.trim().to_string() + " Session ID: ";
                            }
                        }
                    }
                }
            }
        }

        // Fallback message like Fido
        let msg = "Your IP address has been banned by Microsoft for issuing too many ISO download requests or for belonging to a region of the world where sanctions currently apply. Please try again later.\nIf you believe this ban to be in error, you can try contacting Microsoft by referring to message code 715-123130 and session ID ";
        msg.to_string()
    }
}
