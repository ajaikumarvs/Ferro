use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsVersion {
    pub name: String,
    pub page_type: String,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsRelease {
    pub name: String,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsEdition {
    pub name: String,
    pub id: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsLanguage {
    pub name: String,
    pub display_name: String,
    pub data: Vec<LanguageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageData {
    pub session_index: usize,
    pub sku_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsArchitecture {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoftApiResponse {
    #[serde(rename = "Skus")]
    pub skus: Option<Vec<Sku>>,
    #[serde(rename = "ProductDownloadOptions")]
    pub product_download_options: Option<Vec<ProductDownloadOption>>,
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<ApiError>>,
    // New fields from actual API response
    #[serde(rename = "ValidationContainer")]
    pub validation_container: Option<ValidationContainer>,
    #[serde(rename = "Tickets")]
    pub tickets: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sku {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "LocalizedLanguage")]
    pub localized_language: String,
    #[serde(rename = "LocalizedProductDisplayName")]
    pub localized_product_display_name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ProductDisplayName")]
    pub product_display_name: Option<String>,
    #[serde(rename = "ProductEditionName")]
    pub product_edition_name: Option<String>,
    #[serde(rename = "FriendlyFileNames")]
    pub friendly_file_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDownloadOption {
    #[serde(rename = "Uri")]
    pub uri: String,
    #[serde(rename = "DownloadType")]
    pub download_type: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(rename = "Type")]
    pub error_type: u32,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContainer {
    #[serde(rename = "ErrorList")]
    pub error_list: Vec<serde_json::Value>,
    #[serde(rename = "Errors")]
    pub errors: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SessionData {
    #[allow(dead_code)]
    pub session_id: String,
    pub org_id: String,
    pub profile_id: String,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            org_id: "y6jn8c31".to_string(),
            profile_id: "606624d44113".to_string(),
        }
    }
}

// Static data exactly matching Fido's $WindowsVersions array
pub fn get_windows_versions() -> Vec<WindowsVersionData> {
    vec![
        WindowsVersionData {
            name: "Windows 11".to_string(),
            page_type: "windows11".to_string(),
            releases: vec![WindowsReleaseData {
                name: "24H2 (Build 26100.1742 - 2024.10)".to_string(),
                editions: vec![
                    WindowsEditionData {
                        name: "Windows 11 Home/Pro/Edu".to_string(),
                        ids: vec![3113, 3131],
                    },
                    WindowsEditionData {
                        name: "Windows 11 Home China ".to_string(),
                        ids: vec![3115, 3132],
                    },
                    WindowsEditionData {
                        name: "Windows 11 Pro China ".to_string(),
                        ids: vec![3114, 3133],
                    },
                ],
            }],
        },
        WindowsVersionData {
            name: "Windows 10".to_string(),
            page_type: "Windows10ISO".to_string(),
            releases: vec![WindowsReleaseData {
                name: "22H2 v1 (Build 19045.2965 - 2023.05)".to_string(),
                editions: vec![
                    WindowsEditionData {
                        name: "Windows 10 Home/Pro/Edu".to_string(),
                        ids: vec![2618],
                    },
                    WindowsEditionData {
                        name: "Windows 10 Home China ".to_string(),
                        ids: vec![2378],
                    },
                ],
            }],
        },
        WindowsVersionData {
            name: "UEFI Shell 2.2".to_string(),
            page_type: "UEFI_SHELL 2.2".to_string(),
            releases: vec![
                WindowsReleaseData {
                    name: "25H1 (edk2-stable202505)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "24H2 (edk2-stable202411)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "24H1 (edk2-stable202405)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "23H2 (edk2-stable202311)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "23H1 (edk2-stable202305)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "22H2 (edk2-stable202211)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "22H1 (edk2-stable202205)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "21H2 (edk2-stable202108)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "21H1 (edk2-stable202105)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
                WindowsReleaseData {
                    name: "20H2 (edk2-stable202011)".to_string(),
                    editions: vec![
                        WindowsEditionData {
                            name: "Release".to_string(),
                            ids: vec![0],
                        },
                        WindowsEditionData {
                            name: "Debug".to_string(),
                            ids: vec![1],
                        },
                    ],
                },
            ],
        },
        WindowsVersionData {
            name: "UEFI Shell 2.0".to_string(),
            page_type: "UEFI_SHELL 2.0".to_string(),
            releases: vec![WindowsReleaseData {
                name: "4.632 [20100426]".to_string(),
                editions: vec![WindowsEditionData {
                    name: "Release".to_string(),
                    ids: vec![0],
                }],
            }],
        },
    ]
}

#[derive(Debug, Clone)]
pub struct WindowsVersionData {
    pub name: String,
    pub page_type: String,
    pub releases: Vec<WindowsReleaseData>,
}

#[derive(Debug, Clone)]
pub struct WindowsReleaseData {
    pub name: String,
    pub editions: Vec<WindowsEditionData>,
}

#[derive(Debug, Clone)]
pub struct WindowsEditionData {
    pub name: String,
    pub ids: Vec<u32>,
}
