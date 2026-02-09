//! Console output formatting.

use crate::{Software, SystemInfo, WindowsUpdate, IndustrialSoftware};
use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS};

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
            .set_header(vec!["SYSTEM INFORMATION", ""]);
        
        table.add_row(vec!["Computer Name", &info.computer_name]);
        if let Some(domain) = &info.domain {
            table.add_row(vec!["Domain", domain]);
        }
        table.add_row(vec!["OS", &format!("{} {}", info.os_name, info.os_version)]);
        table.add_row(vec!["Build", &info.build_number]);
        table.add_row(vec!["CPU", &info.cpu_brand]);
        
        output.push_str(&table.to_string());
        output.push_str("\n\n");

        // Network interfaces table
        if !info.network_interfaces.is_empty() {
            let mut net_table = Table::new();
            net_table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
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
            .set_header(vec!["Name", "Version", "Publisher", "Install Date", "Source"]);
        
        for sw in software {
            table.add_row(vec![
                &sw.name,
                sw.version.as_deref().unwrap_or("-"),
                sw.publisher.as_deref().unwrap_or("-"),
                &sw.install_date.map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
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
            .set_header(vec!["Vendor", "Product", "Version", "Install Path"]);
        
        for sw in software {
            table.add_row(vec![
                &sw.vendor.to_string(),
                &sw.product,
                sw.version.as_deref().unwrap_or("-"),
                &sw.install_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "-".to_string()),
            ]);
        }
        
        format!("{}\nFound: {} industrial applications", table, software.len())
    }

    /// Format Windows updates as a table.
    pub fn format_updates(updates: &[WindowsUpdate]) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["HotFix ID", "Description", "Installed On", "Installed By"]);
        
        for update in updates {
            table.add_row(vec![
                &update.hotfix_id,
                update.description.as_deref().unwrap_or("-"),
                &update.installed_on.map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                update.installed_by.as_deref().unwrap_or("-"),
            ]);
        }
        
        format!("{}\nFound: {} updates", table, updates.len())
    }
}
