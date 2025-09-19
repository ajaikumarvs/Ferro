use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use reqwest::Client;
use std::collections::HashMap;
use uuid::Uuid;

use crate::types::*;
use crate::utils;

pub struct IsoApi {
    client: Client,
    session_data: SessionData,
}

impl IsoApi {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            session_data: SessionData::default(),
        }
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

    pub async fn get_editions(&self, version_name: &str, release_name: &str) -> Result<Vec<WindowsEdition>> {
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
            
            // Whitelist session ID
            self.whitelist_session(&session_id).await?;
            
            // Get SKU information
            let languages_response = self.get_sku_information(edition_id, &session_id).await?;
            
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
            self.whitelist_session(&session_id).await?;
            
            let download_response = self.get_download_links(language_data.sku_id, &session_id).await?;
            
            if let Some(download_options) = download_response.product_download_options {
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
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to whitelist session")?;

        if !response.status().is_success() {
            warn!("Session whitelisting returned status: {}", response.status());
        }

        Ok(())
    }

    async fn get_sku_information(&self, product_edition_id: u32, session_id: &str) -> Result<MicrosoftApiResponse> {
        let url = format!(
            "https://www.microsoft.com/software-download-connector/api/getskuinformationbyproductedition?profile={}&productEditionId={}&SKU=undefined&friendlyFileName=undefined&Locale=en-US&sessionID={}",
            self.session_data.profile_id, product_edition_id, session_id
        );

        debug!("Getting SKU information: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get SKU information")?;

        let api_response: MicrosoftApiResponse = response
            .json()
            .await
            .context("Failed to parse SKU information response")?;

        if let Some(errors) = &api_response.errors {
            if !errors.is_empty() {
                return Err(anyhow!("API error: {}", errors[0].value));
            }
        }

        Ok(api_response)
    }

    async fn get_download_links(&self, sku_id: u32, session_id: &str) -> Result<MicrosoftApiResponse> {
        let url = format!(
            "https://www.microsoft.com/software-download-connector/api/GetProductDownloadLinksBySku?profile={}&productEditionId=undefined&SKU={}&friendlyFileName=undefined&Locale=en-US&sessionID={}",
            self.session_data.profile_id, sku_id, session_id
        );

        debug!("Getting download links: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Referer", "https://www.microsoft.com/software-download/windows11")
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
