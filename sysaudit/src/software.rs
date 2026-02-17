//! Installed software enumeration module.
//!
//! Provides read-only access to installed software from Windows Registry.

use crate::Error;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE};

/// Registry source for software entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrySource {
    /// HKLM 64-bit
    LocalMachine64,
    /// HKLM WOW6432Node (32-bit on 64-bit OS)
    LocalMachine32,
    /// HKCU
    CurrentUser,
}

impl std::fmt::Display for RegistrySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrySource::LocalMachine64 => write!(f, "HKLM\\64-bit"),
            RegistrySource::LocalMachine32 => write!(f, "HKLM\\32-bit"),
            RegistrySource::CurrentUser => write!(f, "HKCU"),
        }
    }
}

/// Installed software entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Software {
    /// Software name
    pub name: String,
    /// Version string
    pub version: Option<String>,
    /// Publisher/vendor
    pub publisher: Option<String>,
    /// Installation date
    pub install_date: Option<NaiveDate>,
    /// Installation location
    pub install_location: Option<PathBuf>,
    /// Registry source
    pub source: RegistrySource,
}

/// Scanner for installed software.
pub struct SoftwareScanner {
    include_user_installs: bool,
    include_32bit: bool,
}

impl Default for SoftwareScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareScanner {
    /// Create a new scanner with default settings (all sources enabled).
    pub fn new() -> Self {
        SoftwareScanner {
            include_user_installs: true,
            include_32bit: true,
        }
    }

    /// Include or exclude user-specific installations.
    pub fn include_user_installs(mut self, include: bool) -> Self {
        self.include_user_installs = include;
        self
    }

    /// Include or exclude 32-bit software on 64-bit OS.
    pub fn include_32bit(mut self, include: bool) -> Self {
        self.include_32bit = include;
        self
    }

    /// Scan for installed software (READ-ONLY).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sysaudit::SoftwareScanner;
    ///
    /// let scanner = SoftwareScanner::new();
    /// let software = scanner.scan().unwrap();
    /// for sw in software {
    ///     println!("{}", sw.name);
    /// }
    /// ```
    pub fn scan(&self) -> Result<Vec<Software>, Error> {
        tracing::info!("Starting software scan");
        let mut result = Vec::new();

        // HKLM 64-bit
        if let Ok(software) = self.scan_key(
            LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            RegistrySource::LocalMachine64,
        ) {
            result.extend(software);
        }

        // HKLM 32-bit (WOW6432Node)
        if self.include_32bit {
            if let Ok(software) = self.scan_key(
                LOCAL_MACHINE,
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
                RegistrySource::LocalMachine32,
            ) {
                result.extend(software);
            }
        }

        // HKCU
        if self.include_user_installs {
            if let Ok(software) = self.scan_key(
                CURRENT_USER,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
                RegistrySource::CurrentUser,
            ) {
                result.extend(software);
            }
        }

        // Sort by name
        result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        Ok(result)
    }

    fn scan_key(
        &self,
        root: &Key,
        path: &str,
        source: RegistrySource,
    ) -> Result<Vec<Software>, Error> {
        let key = root.open(path)?;
        let mut result = Vec::new();

        for subkey_name in key.keys()? {
            if let Ok(subkey) = key.open(&subkey_name) {
                if let Some(software) = self.parse_software_key(&subkey, source) {
                    result.push(software);
                }
            }
        }

        Ok(result)
    }

    fn parse_software_key(&self, key: &Key, source: RegistrySource) -> Option<Software> {
        let name = key.get_string("DisplayName").ok()?;
        let version = key.get_string("DisplayVersion").ok();
        let publisher = key.get_string("Publisher").ok();
        let install_location = key.get_string("InstallLocation").ok();
        let install_date_str = key.get_string("InstallDate").ok();

        build_software(
            name,
            version,
            publisher,
            install_location,
            install_date_str,
            source,
        )
    }
}

/// Pure construction logic for software entry (fully testable).
fn build_software(
    name: String,
    version: Option<String>,
    publisher: Option<String>,
    install_location: Option<String>,
    install_date_str: Option<String>,
    source: RegistrySource,
) -> Option<Software> {
    if name.trim().is_empty() {
        return None;
    }

    let install_location = install_location
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    let install_date = install_date_str.and_then(|s| parse_install_date(&s));

    Some(Software {
        name,
        version,
        publisher,
        install_date,
        install_location,
        source,
    })
}

/// Parse install date from registry format (YYYYMMDD).
fn parse_install_date(s: &str) -> Option<NaiveDate> {
    if s.len() != 8 {
        return None;
    }

    let year: i32 = s[0..4].parse().ok()?;
    let month: u32 = s[4..6].parse().ok()?;
    let day: u32 = s[6..8].parse().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_install_date_valid() {
        assert_eq!(
            parse_install_date("20240115"),
            NaiveDate::from_ymd_opt(2024, 1, 15)
        );
    }

    #[test]
    fn test_parse_install_date_invalid() {
        assert_eq!(parse_install_date("invalid"), None);
        assert_eq!(parse_install_date("2024"), None);
        assert_eq!(parse_install_date(""), None);
        assert_eq!(parse_install_date("20240230"), None); // Invalid day
        assert_eq!(parse_install_date("20241301"), None); // Invalid month
        assert_eq!(parse_install_date("ABCDEFGH"), None); // Non-numeric
    }

    #[test]
    fn test_parse_install_date_future() {
        // Technically valid format, logic doesn't reject future dates
        assert_eq!(
            parse_install_date("99991231"),
            NaiveDate::from_ymd_opt(9999, 12, 31)
        );
    }

    #[test]
    fn test_build_software_full() {
        let sw = build_software(
            "Test App".into(),
            Some("2.0".into()),
            Some("Acme".into()),
            Some(r"C:\Acme".into()),
            Some("20240115".into()),
            RegistrySource::LocalMachine64,
        );
        let sw = sw.unwrap();
        assert_eq!(sw.name, "Test App");
        assert_eq!(sw.version.as_deref(), Some("2.0"));
        assert_eq!(sw.publisher.as_deref(), Some("Acme"));
        assert_eq!(sw.install_date, NaiveDate::from_ymd_opt(2024, 1, 15));
        assert_eq!(sw.install_location, Some(PathBuf::from(r"C:\Acme")));
        assert_eq!(sw.source, RegistrySource::LocalMachine64);
    }

    #[test]
    fn test_build_software_empty_name_rejected() {
        let sw = build_software(
            "".into(),
            None,
            None,
            None,
            None,
            RegistrySource::CurrentUser,
        );
        assert!(sw.is_none());
    }

    #[test]
    fn test_build_software_whitespace_name_rejected() {
        let sw = build_software(
            "   ".into(),
            None,
            None,
            None,
            None,
            RegistrySource::LocalMachine32,
        );
        assert!(sw.is_none());
    }

    #[test]
    fn test_build_software_empty_install_location_filtered() {
        let sw = build_software(
            "App".into(),
            None,
            None,
            Some("".into()), // empty string
            None,
            RegistrySource::LocalMachine64,
        );
        assert!(sw.unwrap().install_location.is_none());
    }

    #[test]
    fn test_build_software_invalid_date_ignored() {
        let sw = build_software(
            "App".into(),
            None,
            None,
            None,
            Some("not-a-date".into()),
            RegistrySource::LocalMachine64,
        );
        assert!(sw.unwrap().install_date.is_none());
    }
}
