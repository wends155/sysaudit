//! Console output formatting.

use crate::{IndustrialSoftware, Software, SystemInfo, WindowsUpdate};
use comfy_table::{ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

/// Max table width in characters
const MAX_TABLE_WIDTH: u16 = 120;

/// Console formatter for pretty output.
pub struct ConsoleFormatter;

impl ConsoleFormatter {
    /// Format system info as a table.
    pub fn format_system_info(info: &SystemInfo) -> String {
        let mut output = String::new();

        // System info table
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(MAX_TABLE_WIDTH)
            .set_header(vec!["SYSTEM INFORMATION", ""]);

        table.add_row(vec!["Computer Name", &info.computer_name]);
        if let Some(domain) = &info.domain {
            table.add_row(vec!["Domain", domain]);
        }

        // Manufacturer / Model
        if let (Some(man), Some(mod_)) = (&info.manufacturer, &info.model) {
            table.add_row(vec!["System", &format!("{} / {}", man, mod_)]);
        } else if let Some(man) = &info.manufacturer {
            table.add_row(vec!["Manufacturer", man]);
        } else if let Some(mod_) = &info.model {
            table.add_row(vec!["Model", mod_]);
        }

        table.add_row(vec!["OS", &format!("{} {}", info.os_name, info.os_version)]);
        table.add_row(vec!["Build", &info.build_number]);

        // CPU
        let cpu_cores =
            if let (Some(phys), Some(log)) = (info.cpu_cores_physical, info.cpu_cores_logical) {
                format!("{} (Phys) / {} (Log)", phys, log)
            } else {
                "-".to_string()
            };
        table.add_row(vec!["CPU", &info.cpu_info]);
        table.add_row(vec!["CPU Cores", &cpu_cores]);
        table.add_row(vec!["CPU Freq", &format!("{} MHz", info.cpu_frequency_mhz)]);

        // Memory
        let mem_used_gb = info.memory_used as f64 / 1_073_741_824.0;
        let mem_total_gb = info.memory_total as f64 / 1_073_741_824.0;
        let mem_percent = if info.memory_total > 0 {
            (info.memory_used as f64 / info.memory_total as f64) * 100.0
        } else {
            0.0
        };

        table.add_row(vec![
            "Memory",
            &format!(
                "{:.2} GB / {:.2} GB ({:.1}%)",
                mem_used_gb, mem_total_gb, mem_percent
            ),
        ]);

        output.push_str(&table.to_string());
        output.push_str("\n\n");

        // Network interfaces table
        if !info.network_interfaces.is_empty() {
            let mut net_table = Table::new();
            net_table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(MAX_TABLE_WIDTH)
                .set_header(vec!["Interface", "IP Address", "Prefix", "MAC"]);

            for iface in &info.network_interfaces {
                net_table.add_row(vec![
                    &iface.name,
                    &iface.ip_address.to_string(),
                    iface.subnet_mask.as_deref().unwrap_or("-"),
                    iface.mac_address.as_deref().unwrap_or("-"),
                ]);
            }

            output.push_str(&net_table.to_string());
        }

        output
    }

    /// Format software list as a table.
    pub fn format_software(software: &[Software]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(MAX_TABLE_WIDTH)
            .set_header(vec![
                "Name",
                "Version",
                "Publisher",
                "Install Date",
                "Source",
            ]);

        for sw in software {
            table.add_row(vec![
                &sw.name,
                sw.version.as_deref().unwrap_or("-"),
                sw.publisher.as_deref().unwrap_or("-"),
                &sw.install_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                &sw.source.to_string(),
            ]);
        }

        format!("{}\nFound: {} items", table, software.len())
    }

    /// Format industrial software as a table.
    pub fn format_industrial(software: &[IndustrialSoftware]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(MAX_TABLE_WIDTH)
            .set_header(vec!["Vendor", "Product", "Version", "Install Path"]);

        for sw in software {
            table.add_row(vec![
                &sw.vendor.to_string(),
                &sw.product,
                sw.version.as_deref().unwrap_or("-"),
                &sw.install_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ]);
        }

        format!(
            "{}\nFound: {} industrial applications",
            table,
            software.len()
        )
    }

    /// Format Windows updates as a table.
    pub fn format_updates(updates: &[WindowsUpdate]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(MAX_TABLE_WIDTH)
            .set_header(vec![
                "HotFix ID",
                "Description",
                "Installed On",
                "Installed By",
            ]);

        for update in updates {
            table.add_row(vec![
                &update.hotfix_id,
                update.description.as_deref().unwrap_or("-"),
                &update
                    .installed_on
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                update.installed_by.as_deref().unwrap_or("-"),
            ]);
        }

        format!("{}\nFound: {} updates", table, updates.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::path::PathBuf;

    #[test]
    fn test_format_software_table() {
        let sw = Software {
            name: "Test App".to_string(),
            version: Some("1.0.0".to_string()),
            publisher: Some("Test Corp".to_string()),
            install_date: NaiveDate::from_ymd_opt(2024, 1, 1),
            install_location: Some(PathBuf::from("C:\\App")),
            source: crate::RegistrySource::LocalMachine64,
        };

        let output = ConsoleFormatter::format_software(&[sw]);

        assert!(output.contains("Test App"));
        assert!(output.contains("1.0.0"));
        assert!(output.contains("Test Corp"));
        assert!(output.contains("2024-01-01"));
        assert!(output.contains("Found: 1 items"));
    }

    #[test]
    fn test_format_updates_empty() {
        let output = ConsoleFormatter::format_updates(&[]);
        assert!(output.contains("HotFix ID"));
        assert!(output.contains("Found: 0 updates"));
    }

    #[test]
    fn test_format_system_info() {
        let info = SystemInfo {
            os_name: "Windows 11 Pro".into(),
            os_version: "23H2".into(),
            build_number: "22631.3007".into(),
            computer_name: "TEST-PC".into(),
            domain: Some("contoso.local".into()),
            cpu_info: "Intel i7-9700".into(),
            network_interfaces: vec![],
            manufacturer: Some("Dell Inc.".into()),
            model: Some("OptiPlex 7090".into()),
            cpu_cores_physical: Some(8),
            cpu_cores_logical: Some(8),
            cpu_frequency_mhz: 3000,
            memory_total: 17_179_869_184, // 16 GB
            memory_used: 8_589_934_592,   // 8 GB
            memory_free: 8_589_934_592,
        };

        let output = ConsoleFormatter::format_system_info(&info);
        assert!(output.contains("TEST-PC"));
        assert!(output.contains("Windows 11 Pro"));
        assert!(output.contains("22631.3007"));
        assert!(output.contains("Dell Inc."));
        assert!(output.contains("contoso.local"));
    }

    #[test]
    fn test_format_industrial_table() {
        use crate::Vendor;
        let sw = IndustrialSoftware {
            vendor: Vendor::Citect,
            product: "AVEVA Plant SCADA 2023".into(),
            version: Some("8.0".into()),
            install_path: Some(PathBuf::from(r"C:\Citect")),
        };

        let output = ConsoleFormatter::format_industrial(&[sw]);
        assert!(output.contains("Citect"));
        assert!(output.contains("AVEVA Plant SCADA 2023"));
        assert!(output.contains("8.0"));
        assert!(output.contains("Found: 1 industrial"));
    }
}
