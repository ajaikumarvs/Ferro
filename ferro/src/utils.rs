use regex::Regex;
use sys_locale::get_locale;

/// Get the system locale, defaulting to "en-US" if not available
pub fn get_system_locale() -> String {
    get_locale().unwrap_or_else(|| "en-US".to_string())
}

/// Get the system architecture
pub fn get_system_architecture() -> String {
    match std::env::consts::ARCH {
        "x86_64" => "x64".to_string(),
        "x86" => "x86".to_string(),
        "aarch64" => "ARM64".to_string(),
        "arm" => "ARM32".to_string(),
        arch => arch.to_string(),
    }
}

/// Extract filename from URL
pub fn extract_filename_from_url(url: &str) -> Option<String> {
    let re = Regex::new(r".*\/(.+\.iso).*").ok()?;
    re.captures(url)?
        .get(1)?
        .as_str()
        .to_string()
        .into()
}

/// Convert bytes to human readable format
pub fn bytes_to_human_readable(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Convert Microsoft architecture type code to formal architecture name
pub fn get_arch_from_type(arch_type: u32) -> String {
    match arch_type {
        0 => "x86".to_string(),
        1 => "x64".to_string(),
        2 => "ARM64".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Select language based on system locale
#[allow(dead_code)]
pub fn select_language_by_locale(language_name: &str, system_locale: &str) -> bool {
    let locale = system_locale.to_lowercase();
    let lang = language_name.to_lowercase();
    
    // Check for various language matches based on locale
    (locale.starts_with("ar") && lang.contains("arabic")) ||
    (locale == "pt-br" && lang.contains("brazil")) ||
    (locale.starts_with("bg") && lang.contains("bulgar")) ||
    (locale == "zh-cn" && lang.contains("chinese") && lang.contains("simp")) ||
    (locale == "zh-tw" && lang.contains("chinese") && lang.contains("trad")) ||
    (locale.starts_with("hr") && lang.contains("croat")) ||
    (locale.starts_with("cs") && lang.contains("czech")) ||
    (locale.starts_with("da") && lang.contains("danish")) ||
    (locale.starts_with("nl") && lang.contains("dutch")) ||
    (locale == "en-us" && lang == "english") ||
    (locale.starts_with("en") && lang.contains("english") && (lang.contains("inter") || lang.contains("kingdom"))) ||
    (locale.starts_with("et") && lang.contains("eston")) ||
    (locale.starts_with("fi") && lang.contains("finn")) ||
    (locale == "fr-ca" && lang.contains("french") && lang.contains("canad")) ||
    (locale.starts_with("fr") && lang == "french") ||
    (locale.starts_with("de") && lang.contains("german")) ||
    (locale.starts_with("el") && lang.contains("greek")) ||
    (locale.starts_with("he") && lang.contains("hebrew")) ||
    (locale.starts_with("hu") && lang.contains("hungar")) ||
    (locale.starts_with("id") && lang.contains("indones")) ||
    (locale.starts_with("it") && lang.contains("italia")) ||
    (locale.starts_with("ja") && lang.contains("japan")) ||
    (locale.starts_with("ko") && lang.contains("korea")) ||
    (locale.starts_with("lv") && lang.contains("latvia")) ||
    (locale.starts_with("lt") && lang.contains("lithuania")) ||
    (locale.starts_with("ms") && lang.contains("malay")) ||
    (locale.starts_with("nb") && lang.contains("norw")) ||
    (locale.starts_with("fa") && lang.contains("persia")) ||
    (locale.starts_with("pl") && lang.contains("polish")) ||
    (locale == "pt-pt" && lang == "portuguese") ||
    (locale.starts_with("ro") && lang.contains("romania")) ||
    (locale.starts_with("ru") && lang.contains("russia")) ||
    (locale.starts_with("sr") && lang.contains("serbia")) ||
    (locale.starts_with("sk") && lang.contains("slovak")) ||
    (locale.starts_with("sl") && lang.contains("slovenia")) ||
    (locale == "es-es" && lang == "spanish") ||
    (locale.starts_with("es") && locale != "es-es" && lang.contains("spanish")) ||
    (locale.starts_with("sv") && lang.contains("swed")) ||
    (locale.starts_with("th") && lang.contains("thai")) ||
    (locale.starts_with("tr") && lang.contains("turk")) ||
    (locale.starts_with("uk") && lang.contains("ukrain")) ||
    (locale.starts_with("vi") && lang.contains("vietnam"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_human_readable() {
        assert_eq!(bytes_to_human_readable(1024), "1.0 KB");
        assert_eq!(bytes_to_human_readable(1536), "1.5 KB");
        assert_eq!(bytes_to_human_readable(1024 * 1024), "1.0 MB");
        assert_eq!(bytes_to_human_readable(5 * 1024 * 1024 * 1024), "5.0 GB");
    }

    #[test]
    fn test_extract_filename_from_url() {
        let url = "https://example.com/path/to/file.iso?param=value";
        assert_eq!(extract_filename_from_url(url), Some("file.iso".to_string()));
    }

    #[test]
    fn test_get_arch_from_type() {
        assert_eq!(get_arch_from_type(0), "x86");
        assert_eq!(get_arch_from_type(1), "x64");
        assert_eq!(get_arch_from_type(2), "ARM64");
        assert_eq!(get_arch_from_type(99), "Unknown");
    }

    #[test]
    fn test_select_language_by_locale() {
        assert!(select_language_by_locale("English", "en-us"));
        assert!(select_language_by_locale("French", "fr-fr"));
        assert!(select_language_by_locale("German", "de-de"));
        assert!(!select_language_by_locale("Spanish", "en-us"));
    }
}
