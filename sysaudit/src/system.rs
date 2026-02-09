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
    /// CPU brand string
    pub cpu_brand: String,
    /// Network interfaces with IP, mask, gateway
    pub network_interfaces: Vec<NetworkInterface>,
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

        // Get CPU brand
        let cpu_brand = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Get network interfaces
        let network_interfaces = Self::get_network_interfaces();

        Ok(SystemInfo {
            os_name,
            os_version,
            build_number,
            computer_name,
            domain,
            cpu_brand,
            network_interfaces,
        })
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
                interfaces.push(NetworkInterface {
                    name: name.clone(),
                    ip_address: ip.addr,
                    subnet_mask: Some(format!("/{}", ip.prefix)),
                    gateway: None, // Would need additional API calls
                    mac_address: Some(format!("{:?}", network.mac_address())),
                });
            }
        }

        interfaces
    }
}
