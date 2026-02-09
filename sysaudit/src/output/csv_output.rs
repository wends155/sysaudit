//! CSV export functionality.

use crate::{Error, Software, WindowsUpdate, IndustrialSoftware};
use std::path::Path;

/// CSV exporter for audit data.
pub struct CsvExporter;

impl CsvExporter {
    /// Export software list to CSV.
    pub fn export_software(software: &[Software], path: &Path) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;
        
        wtr.write_record(["Name", "Version", "Publisher", "Install Date", "Install Location", "Source"])?;
        
        for sw in software {
            wtr.write_record([
                &sw.name,
                sw.version.as_deref().unwrap_or(""),
                sw.publisher.as_deref().unwrap_or(""),
                &sw.install_date.map(|d| d.to_string()).unwrap_or_default(),
                &sw.install_location.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
                &sw.source.to_string(),
            ])?;
        }
        
        wtr.flush()?;
        Ok(())
    }

    /// Export industrial software to CSV.
    pub fn export_industrial(software: &[IndustrialSoftware], path: &Path) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;
        
        wtr.write_record(["Vendor", "Product", "Version", "Install Path"])?;
        
        for sw in software {
            wtr.write_record([
                &sw.vendor.to_string(),
                &sw.product,
                sw.version.as_deref().unwrap_or(""),
                &sw.install_path.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            ])?;
        }
        
        wtr.flush()?;
        Ok(())
    }

    /// Export Windows updates to CSV.
    pub fn export_updates(updates: &[WindowsUpdate], path: &Path) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;
        
        wtr.write_record(["HotFix ID", "Description", "Installed On", "Installed By"])?;
        
        for update in updates {
            wtr.write_record([
                &update.hotfix_id,
                update.description.as_deref().unwrap_or(""),
                &update.installed_on.map(|d| d.to_string()).unwrap_or_default(),
                update.installed_by.as_deref().unwrap_or(""),
            ])?;
        }
        
        wtr.flush()?;
        Ok(())
    }
}
