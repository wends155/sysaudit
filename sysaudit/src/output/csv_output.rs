//! CSV export functionality.

use crate::{Error, IndustrialSoftware, Software, WindowsUpdate};
use std::path::Path;

/// CSV exporter for audit data.
pub struct CsvExporter;

impl CsvExporter {
    /// Export software list to CSV.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the file cannot be created or written.
    pub fn export_software(software: &[Software], path: &Path) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record([
            "Name",
            "Version",
            "Publisher",
            "Install Date",
            "Install Location",
            "Source",
        ])?;

        for sw in software {
            wtr.write_record([
                &sw.name,
                sw.version.as_deref().unwrap_or(""),
                sw.publisher.as_deref().unwrap_or(""),
                &sw.install_date.map(|d| d.to_string()).unwrap_or_default(),
                &sw.install_location
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
                &sw.source.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    /// Export industrial software to CSV.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the file cannot be created or written.
    pub fn export_industrial(software: &[IndustrialSoftware], path: &Path) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record(["Vendor", "Product", "Version", "Install Path"])?;

        for sw in software {
            wtr.write_record([
                &sw.vendor.to_string(),
                &sw.product,
                sw.version.as_deref().unwrap_or(""),
                &sw.install_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    /// Export Windows updates to CSV.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the file cannot be created or written.
    pub fn export_updates(updates: &[WindowsUpdate], path: &Path) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record(["HotFix ID", "Description", "Installed On", "Installed By"])?;

        for update in updates {
            wtr.write_record([
                &update.hotfix_id,
                update.description.as_deref().unwrap_or(""),
                &update
                    .installed_on
                    .map(|d| d.to_string())
                    .unwrap_or_default(),
                update.installed_by.as_deref().unwrap_or(""),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RegistrySource, Vendor};
    use chrono::NaiveDate;
    use std::path::PathBuf;

    fn temp_csv(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("sysaudit_test_{}.csv", name))
    }

    #[test]
    fn test_export_software_csv() {
        let path = temp_csv("software");
        let sw = vec![Software {
            name: "TestApp".into(),
            version: Some("1.0".into()),
            publisher: Some("Acme".into()),
            install_date: NaiveDate::from_ymd_opt(2024, 1, 15),
            install_location: Some(PathBuf::from(r"C:\App")),
            source: RegistrySource::LocalMachine64,
        }];

        CsvExporter::export_software(&sw, &path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("TestApp"));
        assert!(content.contains("1.0"));
        assert!(content.contains("Acme"));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_export_updates_csv() {
        let path = temp_csv("updates");
        let updates = vec![WindowsUpdate {
            hotfix_id: "KB5034441".into(),
            description: Some("Security Update".into()),
            installed_on: NaiveDate::from_ymd_opt(2024, 1, 15),
            installed_by: Some("NT AUTHORITY".into()),
        }];

        CsvExporter::export_updates(&updates, &path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("KB5034441"));
        assert!(content.contains("Security Update"));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_export_industrial_csv() {
        let path = temp_csv("industrial");
        let sw = vec![IndustrialSoftware {
            vendor: Vendor::Rockwell,
            product: "Studio 5000".into(),
            version: Some("33.0".into()),
            install_path: None,
        }];

        CsvExporter::export_industrial(&sw, &path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("Rockwell"));
        assert!(content.contains("Studio 5000"));
        std::fs::remove_file(&path).ok();
    }
}
