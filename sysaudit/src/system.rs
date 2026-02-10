//! System information module.
//!
//! Provides read-only access to OS, CPU, and network information.

use crate::Error;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use sysinfo::System;
use windows_registry::LOCAL_MACHINE;

/// Network interface information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Interface name (e.g., "Ethernet", "Wi-Fi")
    pub name: String,
    /// IP address
    pub ip_address: IpAddr,
    /// Subnet mask
    pub subnet_mask: Option<String>,
    /// Default gateway
    pub gateway: Option<String>,
    /// MAC address
    pub mac_address: Option<String>,
}

/// System information collected from the local machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// OS name (e.g., "Windows 11 Pro")
    pub os_name: String,
    /// OS version (e.g., "23H2")
    pub os_version: String,
    /// Build number with UBR (e.g., "22631.3007")
    pub build_number: String,
    /// Computer name
    pub computer_name: String,
    /// Domain name if joined
    pub domain: Option<String>,
    /// CPU brand string (renamed from cpu_brand)
    pub cpu_info: String,
    /// Network interfaces with IP, mask, gateway
    pub network_interfaces: Vec<NetworkInterface>,

    // --- Phase 2: Enhanced Metrics ---
    /// System Manufacturer (e.g., "Dell Inc.")
    pub manufacturer: Option<String>,
    /// System Model (e.g., "OptiPlex 9020")
    pub model: Option<String>,
    /// Physical core count
    pub cpu_cores_physical: Option<usize>,
    /// Logical core count
    pub cpu_cores_logical: Option<usize>,
    /// CPU frequency in MHz
    pub cpu_frequency_mhz: u64,
    /// Total RAM in bytes
    pub memory_total: u64,
    /// Used RAM in bytes
    pub memory_used: u64,
    /// Free RAM in bytes
    pub memory_free: u64,
}

impl SystemInfo {
    /// Collect system information (READ-ONLY).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sysaudit::SystemInfo;
    ///
    /// let info = SystemInfo::collect().unwrap();
    /// println!("Computer: {}", info.computer_name);
    /// ```
    pub fn collect() -> Result<Self, Error> {
        let mut sys = System::new_all();
        sys.refresh_all();

        // Get OS info from sysinfo
        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());

        // Get build number from registry
        let build_number = Self::get_build_number()?;

        // Get computer name
        let computer_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        // Get domain from registry
        let domain = Self::get_domain();

        // Get CPU details
        let cpu_info = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_default();
        let cpu_cores_physical = sys.physical_core_count();
        let cpu_cores_logical = Some(sys.cpus().len());
        let cpu_frequency_mhz = sys.cpus().first().map(|cpu| cpu.frequency()).unwrap_or(0);

        // Get Memory details
        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();
        let memory_free = sys.free_memory();

        // Get Manufacturer/Model via WMI
        let (manufacturer, model) = Self::get_system_model_info();

        // Get network interfaces
        let network_interfaces = Self::get_network_interfaces();

        Ok(SystemInfo {
            os_name,
            os_version,
            build_number,
            computer_name,
            domain,
            cpu_info,
            network_interfaces,
            manufacturer,
            model,
            cpu_cores_physical,
            cpu_cores_logical,
            cpu_frequency_mhz,
            memory_total,
            memory_used,
            memory_free,
        })
    }

    fn get_system_model_info() -> (Option<String>, Option<String>) {
        use serde::Deserialize;
        use wmi::{COMLibrary, WMIConnection};

        #[derive(Deserialize)]
        #[serde(rename = "Win32_ComputerSystem")]
        #[serde(rename_all = "PascalCase")]
        struct Win32ComputerSystem {
            manufacturer: Option<String>,
            model: Option<String>,
        }

        let com_con = match COMLibrary::new() {
            Ok(c) => c,
            Err(_) => return (None, None),
        };

        let wmi_con = match WMIConnection::new(com_con) {
            Ok(c) => c,
            Err(_) => return (None, None),
        };

        match wmi_con.query::<Win32ComputerSystem>() {
            Ok(results) => {
                if let Some(sys) = results.first() {
                    (sys.manufacturer.clone(), sys.model.clone())
                } else {
                    (None, None)
                }
            }
            Err(_) => (None, None),
        }
    }

    fn get_build_number() -> Result<String, Error> {
        let key = LOCAL_MACHINE.open(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")?;

        let current_build: String = key.get_string("CurrentBuild").unwrap_or_default();
        let ubr: u32 = key.get_u32("UBR").unwrap_or(0);

        if ubr > 0 {
            Ok(format!("{}.{}", current_build, ubr))
        } else {
            Ok(current_build)
        }
    }

    fn get_domain() -> Option<String> {
        let key = LOCAL_MACHINE
            .open(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters")
            .ok()?;
        key.get_string("Domain").ok().filter(|s| !s.is_empty())
    }

    fn get_network_interfaces() -> Vec<NetworkInterface> {
        use sysinfo::Networks;

        let networks = Networks::new_with_refreshed_list();
        let mut interfaces = Vec::new();

        for (name, network) in &networks {
            for ip in network.ip_networks() {
                // Format MAC address as hex (e.g., AC:B4:80:D6:59:1D)
                let mac = network.mac_address();
                let mac_str = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    mac.0[0], mac.0[1], mac.0[2], mac.0[3], mac.0[4], mac.0[5]
                );

                interfaces.push(NetworkInterface {
                    name: name.clone(),
                    ip_address: ip.addr,
                    subnet_mask: Some(format!("/{}", ip.prefix)),
                    gateway: None, // Would need additional API calls
                    mac_address: Some(mac_str),
                });
            }
        }

        interfaces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_system_info() {
        let info = SystemInfo::collect().expect("Should collect system info");

        // Basic sanity checks
        assert!(
            !info.computer_name.is_empty(),
            "Computer name should not be empty"
        );
        assert!(!info.os_name.is_empty(), "OS name should not be empty");
        assert!(
            !info.build_number.is_empty(),
            "Build number should not be empty"
        );
        assert!(!info.cpu_info.is_empty(), "CPU info should not be empty");
    }

    #[test]
    fn test_network_interfaces_have_valid_mac() {
        let info = SystemInfo::collect().expect("Should collect system info");

        for iface in &info.network_interfaces {
            if let Some(mac) = &iface.mac_address {
                // MAC should be in format XX:XX:XX:XX:XX:XX
                assert!(mac.contains(':'), "MAC should contain colons: {}", mac);
                assert_eq!(mac.len(), 17, "MAC should be 17 chars: {}", mac);
            }
        }
    }

    #[test]
    fn test_build_number_format() {
        let info = SystemInfo::collect().expect("Should collect system info");

        // Build number should contain digits
        assert!(
            info.build_number.chars().any(|c| c.is_ascii_digit()),
            "Build number should contain digits: {}",
            info.build_number
        );
    }
}
