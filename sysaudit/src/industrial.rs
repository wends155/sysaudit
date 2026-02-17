//! Industrial software detection module.
//!
//! Provides read-only detection of industrial automation software.

use crate::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE};

/// Industrial software vendor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vendor {
    /// Citect SCADA / AVEVA Plant SCADA
    Citect,
    /// Digifort VMS
    Digifort,
    /// ABB Automation
    ABB,
    /// Rockwell Automation / Allen-Bradley
    Rockwell,
    /// Siemens Industrial
    Siemens,
    /// Schneider Electric
    SchneiderElectric,
    /// Other vendor
    Other(String),
}

impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vendor::Citect => write!(f, "Citect"),
            Vendor::Digifort => write!(f, "Digifort"),
            Vendor::ABB => write!(f, "ABB"),
            Vendor::Rockwell => write!(f, "Rockwell"),
            Vendor::Siemens => write!(f, "Siemens"),
            Vendor::SchneiderElectric => write!(f, "Schneider Electric"),
            Vendor::Other(name) => write!(f, "{}", name),
        }
    }
}

/// Industrial software entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustrialSoftware {
    /// Vendor
    pub vendor: Vendor,
    /// Product name
    pub product: String,
    /// Version
    pub version: Option<String>,
    /// Installation path
    pub install_path: Option<PathBuf>,
}

/// Scanner for industrial software.
pub struct IndustrialScanner {
    vendors: Vec<Vendor>,
}

impl Default for IndustrialScanner {
    fn default() -> Self {
        Self::all_vendors()
    }
}

impl IndustrialScanner {
    /// Create scanner for all known vendors.
    pub fn all_vendors() -> Self {
        IndustrialScanner {
            vendors: vec![
                Vendor::Citect,
                Vendor::Digifort,
                Vendor::ABB,
                Vendor::Rockwell,
                Vendor::Siemens,
                Vendor::SchneiderElectric,
            ],
        }
    }

    /// Create scanner for specific vendors.
    pub fn with_vendors(vendors: Vec<Vendor>) -> Self {
        IndustrialScanner { vendors }
    }

    /// Scan for industrial software (READ-ONLY).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sysaudit::{IndustrialScanner, Vendor};
    ///
    /// let scanner = IndustrialScanner::with_vendors(vec![Vendor::Rockwell, Vendor::Citect]);
    /// let industrial = scanner.scan().unwrap();
    /// for sw in industrial {
    ///     println!("{}: {}", sw.vendor, sw.product);
    /// }
    /// ```
    pub fn scan(&self) -> Result<Vec<IndustrialSoftware>, Error> {
        tracing::info!("Scanning for industrial software (vendors: {:?})", self.vendors);
        let mut result = Vec::new();

        for vendor in &self.vendors {
            match vendor {
                Vendor::Citect => result.extend(self.scan_citect()),
                Vendor::Digifort => result.extend(self.scan_digifort()),
                Vendor::ABB => result.extend(self.scan_abb()),
                Vendor::Rockwell => result.extend(self.scan_rockwell()),
                Vendor::Siemens => result.extend(self.scan_siemens()),
                Vendor::SchneiderElectric => result.extend(self.scan_schneider()),
                Vendor::Other(_) => {}
            }
        }

        // Also scan standard Uninstall keys for industrial patterns
        result.extend(self.scan_uninstall_keys());

        // Remove duplicates by product name
        result.sort_by(|a, b| a.product.cmp(&b.product));
        result.dedup_by(|a, b| a.product == b.product);

        Ok(result)
    }

    fn scan_citect(&self) -> Vec<IndustrialSoftware> {
        let mut result = Vec::new();

        // Check Citect SCADA Installs
        if let Ok(key) = LOCAL_MACHINE.open(r"SOFTWARE\WOW6432Node\Citect\SCADA Installs") {
            for version in key.keys().into_iter().flatten() {
                if let Ok(subkey) = key.open(&version) {
                    let install_path = subkey.get_string("DefaultINIPath").ok().map(PathBuf::from);

                    result.push(IndustrialSoftware {
                        vendor: Vendor::Citect,
                        product: format!("AVEVA Plant SCADA {}", version),
                        version: Some(version),
                        install_path,
                    });
                }
            }
        }

        result
    }

    fn scan_digifort(&self) -> Vec<IndustrialSoftware> {
        let mut result = Vec::new();

        for (root, name) in [
            (&LOCAL_MACHINE, r"SOFTWARE\Digifort"),
            (&CURRENT_USER, r"Software\Digifort"),
        ] {
            if root.open(name).is_ok() {
                result.push(IndustrialSoftware {
                    vendor: Vendor::Digifort,
                    product: "Digifort VMS".to_string(),
                    version: None,
                    install_path: None,
                });
                break;
            }
        }

        result
    }

    fn scan_abb(&self) -> Vec<IndustrialSoftware> {
        // ABB typically uses standard Uninstall keys
        Vec::new()
    }

    fn scan_rockwell(&self) -> Vec<IndustrialSoftware> {
        let mut result = Vec::new();

        // Check Rockwell Software registry
        if let Ok(key) = LOCAL_MACHINE.open(r"SOFTWARE\Wow6432Node\Rockwell Software") {
            for subkey_name in key.keys().into_iter().flatten() {
                result.push(IndustrialSoftware {
                    vendor: Vendor::Rockwell,
                    product: subkey_name.clone(),
                    version: None,
                    install_path: None,
                });
            }
        }

        result
    }

    fn scan_siemens(&self) -> Vec<IndustrialSoftware> {
        // Siemens typically uses standard Uninstall keys
        Vec::new()
    }

    fn scan_schneider(&self) -> Vec<IndustrialSoftware> {
        let mut result = Vec::new();

        if let Ok(key) = CURRENT_USER.open(r"Software\Schneider Electric") {
            for subkey_name in key.keys().into_iter().flatten() {
                result.push(IndustrialSoftware {
                    vendor: Vendor::SchneiderElectric,
                    product: subkey_name.clone(),
                    version: None,
                    install_path: None,
                });
            }
        }

        result
    }

    fn scan_uninstall_keys(&self) -> Vec<IndustrialSoftware> {
        let mut result = Vec::new();

        let paths = [
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ];

        for path in paths {
            if let Ok(key) = LOCAL_MACHINE.open(path) {
                for subkey_name in key.keys().into_iter().flatten() {
                    if let Ok(subkey) = key.open(&subkey_name) {
                        if let Ok(name) = subkey.get_string("DisplayName") {
                            if let Some(sw) = self.match_industrial(&name, &subkey) {
                                result.push(sw);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    fn match_industrial(&self, name: &str, key: &Key) -> Option<IndustrialSoftware> {
        let version = key.get_string("DisplayVersion").ok();
        let install_path = key
            .get_string("InstallLocation")
            .ok()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from);

        classify_industrial(name, version, install_path, &self.vendors)
    }
}

/// Pure classification logic for industrial software (fully testable).
fn classify_industrial(
    name: &str,
    version: Option<String>,
    install_path: Option<PathBuf>,
    vendors: &[Vendor],
) -> Option<IndustrialSoftware> {
    let name_lower = name.to_lowercase();

    // Pattern matching for industrial software
    let vendor = if name_lower.contains("citect")
        || (name_lower.contains("aveva") && name_lower.contains("scada"))
    {
        if vendors.contains(&Vendor::Citect) {
            Some(Vendor::Citect)
        } else {
            None
        }
    } else if name_lower.contains("digifort") {
        if vendors.contains(&Vendor::Digifort) {
            Some(Vendor::Digifort)
        } else {
            None
        }
    } else if name_lower.contains("abb")
        && (name_lower.contains("automation") || name_lower.contains("builder"))
    {
        if vendors.contains(&Vendor::ABB) {
            Some(Vendor::ABB)
        } else {
            None
        }
    } else if name_lower.contains("rockwell")
        || name_lower.contains("allen-bradley")
        || name_lower.contains("studio 5000")
    {
        if vendors.contains(&Vendor::Rockwell) {
            Some(Vendor::Rockwell)
        } else {
            None
        }
    } else if name_lower.contains("simatic")
        || name_lower.contains("tia portal")
        || name_lower.contains("wincc")
    {
        if vendors.contains(&Vendor::Siemens) {
            Some(Vendor::Siemens)
        } else {
            None
        }
    } else if name_lower.contains("schneider") && name_lower.contains("electric") {
        if vendors.contains(&Vendor::SchneiderElectric) {
            Some(Vendor::SchneiderElectric)
        } else {
            None
        }
    } else {
        None
    }?;

    Some(IndustrialSoftware {
        vendor,
        product: name.to_string(),
        version,
        install_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_vendors() -> Vec<Vendor> {
        vec![
            Vendor::Citect,
            Vendor::Digifort,
            Vendor::ABB,
            Vendor::Rockwell,
            Vendor::Siemens,
            Vendor::SchneiderElectric,
        ]
    }

    #[test]
    fn test_vendor_display() {
        assert_eq!(Vendor::Citect.to_string(), "Citect");
        assert_eq!(Vendor::ABB.to_string(), "ABB");
        assert_eq!(Vendor::SchneiderElectric.to_string(), "Schneider Electric");
        assert_eq!(Vendor::Other("Custom".into()).to_string(), "Custom");
    }

    #[test]
    fn test_all_vendors_constructor() {
        let scanner = IndustrialScanner::all_vendors();
        assert_eq!(scanner.vendors.len(), 6);
    }

    #[test]
    fn test_classify_citect() {
        let v = all_vendors();
        let result = classify_industrial("Citect SCADA 2023", Some("8.0".into()), None, &v);
        assert!(result.is_some());
        assert_eq!(result.unwrap().vendor, Vendor::Citect);
    }

    #[test]
    fn test_classify_aveva_scada() {
        let v = all_vendors();
        let result = classify_industrial("AVEVA Plant SCADA 2023", None, None, &v);
        assert!(result.is_some());
        assert_eq!(result.unwrap().vendor, Vendor::Citect);
    }

    #[test]
    fn test_classify_aveva_without_scada_no_match() {
        let v = all_vendors();
        // "aveva" alone without "scada" should NOT match
        let result = classify_industrial("AVEVA Edge 2024", None, None, &v);
        assert!(result.is_none());
    }

    #[test]
    fn test_classify_rockwell() {
        let v = all_vendors();
        for name in [
            "Rockwell Automation",
            "Allen-Bradley Tools",
            "Studio 5000 Logix",
        ] {
            let result = classify_industrial(name, None, None, &v);
            assert!(result.is_some(), "should match: {}", name);
            assert_eq!(result.unwrap().vendor, Vendor::Rockwell);
        }
    }

    #[test]
    fn test_classify_siemens() {
        let v = all_vendors();
        for name in ["SIMATIC WinCC", "TIA Portal V18", "WinCC Unified"] {
            let result = classify_industrial(name, None, None, &v);
            assert!(result.is_some(), "should match: {}", name);
            assert_eq!(result.unwrap().vendor, Vendor::Siemens);
        }
    }

    #[test]
    fn test_classify_abb() {
        let v = all_vendors();
        let result = classify_industrial("ABB Automation Builder 2.x", None, None, &v);
        assert!(result.is_some());
        assert_eq!(result.unwrap().vendor, Vendor::ABB);
    }

    #[test]
    fn test_classify_abb_no_keyword_no_match() {
        let v = all_vendors();
        // "abb" alone without "automation" or "builder" should NOT match
        let result = classify_industrial("ABB Robot Studio", None, None, &v);
        assert!(result.is_none());
    }

    #[test]
    fn test_classify_schneider() {
        let v = all_vendors();
        let result = classify_industrial("Schneider Electric EcoStruxure", None, None, &v);
        assert!(result.is_some());
        assert_eq!(result.unwrap().vendor, Vendor::SchneiderElectric);
    }

    #[test]
    fn test_classify_unrecognized_no_match() {
        let v = all_vendors();
        let result = classify_industrial("Microsoft Visual Studio", None, None, &v);
        assert!(result.is_none());
    }

    #[test]
    fn test_classify_vendor_not_in_filter() {
        // Only scanning for Citect — Rockwell should not match
        let v = vec![Vendor::Citect];
        let result = classify_industrial("Rockwell Automation", None, None, &v);
        assert!(result.is_none());
    }

    #[test]
    fn test_classify_preserves_metadata() {
        let v = all_vendors();
        let path = PathBuf::from(r"C:\Program Files\Citect");
        let result = classify_industrial(
            "Citect SCADA",
            Some("8.1.0".into()),
            Some(path.clone()),
            &v,
        );
        let sw = result.unwrap();
        assert_eq!(sw.version.as_deref(), Some("8.1.0"));
        assert_eq!(sw.install_path, Some(path));
        assert_eq!(sw.product, "Citect SCADA");
    }
}
