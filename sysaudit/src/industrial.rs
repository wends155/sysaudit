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
        let name_lower = name.to_lowercase();
        let version = key.get_string("DisplayVersion").ok();
        let install_path = key
            .get_string("InstallLocation")
            .ok()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from);

        // Pattern matching for industrial software
        let vendor = if name_lower.contains("citect")
            || name_lower.contains("aveva") && name_lower.contains("scada")
        {
            if self.vendors.contains(&Vendor::Citect) {
                Some(Vendor::Citect)
            } else {
                None
            }
        } else if name_lower.contains("digifort") {
            if self.vendors.contains(&Vendor::Digifort) {
                Some(Vendor::Digifort)
            } else {
                None
            }
        } else if name_lower.contains("abb")
            && (name_lower.contains("automation") || name_lower.contains("builder"))
        {
            if self.vendors.contains(&Vendor::ABB) {
                Some(Vendor::ABB)
            } else {
                None
            }
        } else if name_lower.contains("rockwell")
            || name_lower.contains("allen-bradley")
            || name_lower.contains("studio 5000")
        {
            if self.vendors.contains(&Vendor::Rockwell) {
                Some(Vendor::Rockwell)
            } else {
                None
            }
        } else if name_lower.contains("simatic")
            || name_lower.contains("tia portal")
            || name_lower.contains("wincc")
        {
            if self.vendors.contains(&Vendor::Siemens) {
                Some(Vendor::Siemens)
            } else {
                None
            }
        } else if name_lower.contains("schneider") && name_lower.contains("electric") {
            if self.vendors.contains(&Vendor::SchneiderElectric) {
                Some(Vendor::SchneiderElectric)
            } else {
                None
            }
        } else {
            None
        };

        vendor.map(|v| IndustrialSoftware {
            vendor: v,
            product: name.to_string(),
            version,
            install_path,
        })
    }
}
