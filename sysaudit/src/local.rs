use crate::scanner::{ScanError, Scanner};
use crate::{IndustrialScanner, SoftwareScanner, SystemInfo};
use sysaudit_common::{
    IndustrialSoftwareDto, IpVersion, NetworkInterfaceDto, SoftwareDto, SysauditReport,
    SystemInfoDto,
};

/// Collects system data from the local machine.
///
/// Wraps existing `SystemInfo::collect()`, `SoftwareScanner::scan()`,
/// `IndustrialScanner::scan()`, and `WindowsUpdate::collect_all()`.
pub struct LocalScanner;

impl Scanner for LocalScanner {
    #[tracing::instrument(skip(self))]
    async fn scan(&self) -> Result<SysauditReport, ScanError> {
        let system_info = SystemInfo::collect()?;
        let software = SoftwareScanner::new().scan()?;
        let industrial = IndustrialScanner::default().scan()?;
        // let updates = WindowsUpdate::collect_all(); // Currently not mapped to SysauditReport in DTO, skip for now.

        // Map sysaudit structures to the DTOs expected by sysaudit-common
        let system_dto = SystemInfoDto {
            os_name: system_info.os_name,
            os_version: system_info.os_version,
            host_name: system_info.computer_name,
            cpu_info: system_info.cpu_info,
            cpu_physical_cores: system_info.cpu_cores_physical.map(|c| c as u32),
            memory_total_bytes: system_info.memory_total,
            memory_used_bytes: system_info.memory_used,
            manufacturer: system_info.manufacturer,
            model: system_info.model,
            network_interfaces: system_info
                .network_interfaces
                .into_iter()
                .map(|iface| {
                    let ip_version = if iface.ip_address.is_ipv4() {
                        IpVersion::IPv4
                    } else {
                        IpVersion::IPv6
                    };

                    NetworkInterfaceDto {
                        name: iface.name,
                        ip_address: iface.ip_address.to_string(),
                        ip_version,
                        mac_address: iface.mac_address,
                    }
                })
                .collect(),
        };

        let software_dto = software
            .into_iter()
            .map(|sw| {
                let install_date = sw
                    .install_date
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .map(|d| d.and_utc());
                SoftwareDto {
                    name: sw.name,
                    version: sw.version,
                    vendor: sw.publisher,
                    install_date,
                }
            })
            .collect();

        let industrial_dto = industrial
            .into_iter()
            .map(|sw| IndustrialSoftwareDto {
                vendor: sw.vendor.to_string(),
                product: sw.product,
                version: sw.version,
                install_path: sw.install_path,
            })
            .collect();

        Ok(SysauditReport {
            system: system_dto,
            software: software_dto,
            industrial: industrial_dto,
            timestamp: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;

    #[tokio::test]
    async fn test_local_scanner_produces_report() {
        let scanner = LocalScanner;
        let report = scanner.scan().await;
        assert!(
            report.is_ok(),
            "LocalScanner should succeed on a Windows machine"
        );
        let report = report.unwrap();
        assert!(
            !report.system.host_name.is_empty(),
            "host_name should not be empty"
        );
        assert!(
            !report.system.os_name.is_empty(),
            "os_name should not be empty"
        );
    }
}
