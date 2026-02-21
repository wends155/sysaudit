pub mod payload;
pub mod transport;

use bon::Builder;
use secrecy::SecretString;
use std::time::Duration;
use sysaudit_common::SysauditReport;

use crate::remote::payload::WINRM_PAYLOAD;
use crate::remote::transport::{HttpWinrmTransport, WinrmTransport};
use crate::scanner::{ScanError, Scanner};

/// Collects system data from a remote Windows machine via WinRM.
///
/// # Examples
///
/// ```no_run
/// use sysaudit::{Scanner, RemoteScanner};
/// use secrecy::SecretString;
///
/// # async fn example() -> Result<(), sysaudit::ScanError> {
/// let scanner = RemoteScanner::builder()
///     .host("192.168.1.100")
///     .username("admin".to_string())
///     .password(SecretString::from("hunter2"))
///     .build();
///
/// let report = scanner.scan().await?;
/// println!("Host: {}", report.system.host_name);
/// # Ok(())
/// # }
/// ```
#[derive(Builder)]
pub struct RemoteScanner {
    /// Target hostname or IP address.
    #[builder(into)]
    host: String,

    /// Username for WinRM authentication.
    #[builder(into)]
    username: String,

    /// Password (secured in memory).
    password: SecretString,

    /// WinRM port (default: 5985 for HTTP, 5986 for HTTPS).
    #[builder(default = 5985)]
    port: u16,

    /// Use HTTPS instead of HTTP.
    #[builder(default = false)]
    use_https: bool,

    /// Skip TLS certificate verification (for self-signed certs).
    #[builder(default = false)]
    skip_cert_verify: bool,

    /// Timeout for the entire scan operation.
    #[builder(default = Duration::from_secs(30))]
    timeout: Duration,
}

impl Scanner for RemoteScanner {
    async fn scan(&self) -> Result<SysauditReport, ScanError> {
        let transport = HttpWinrmTransport::new(
            self.host.clone(),
            self.port,
            self.use_https,
            self.skip_cert_verify,
            self.username.clone(),
            self.password.clone(),
            self.timeout,
        )?;

        // Encode the payload in Base64 (UTF-16LE) for WinRM execution
        // WinRM expects PowerShell commands to be encoded this way.

        let utf16_bytes: Vec<u8> = WINRM_PAYLOAD
            .encode_utf16()
            .flat_map(|u| u.to_le_bytes())
            .collect();
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        let b64_payload = STANDARD.encode(&utf16_bytes);

        // Command to run the encoded payload without profile to speed it up
        let command = format!(
            "powershell -NonInteractive -NoProfile -EncodedCommand {}",
            b64_payload
        );

        RemoteScanner::scan_with_transport(transport, &command).await
    }
}

impl RemoteScanner {
    /// Internal method to allow passing a mocked transport in tests.
    async fn scan_with_transport<T: WinrmTransport>(
        transport: T,
        command: &str,
    ) -> Result<SysauditReport, ScanError> {
        let json_stdout = transport.execute(command).await?;
        let report: SysauditReport = serde_json::from_str(&json_stdout)?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::remote::transport::MockWinrmTransport;
    use chrono::Utc;
    use sysaudit_common::SystemInfoDto;

    #[tokio::test]
    async fn test_remote_scanner_success() {
        let mut mock_transport = MockWinrmTransport::new();

        let mock_report = SysauditReport {
            system: SystemInfoDto {
                os_name: "Mock OS".to_string(),
                os_version: "10.0".to_string(),
                host_name: "MOCK-PC".to_string(),
                cpu_info: "Mock CPU".to_string(),
                cpu_physical_cores: Some(4),
                memory_total_bytes: 8000000,
                memory_used_bytes: 4000000,
                manufacturer: None,
                model: None,
                network_interfaces: vec![],
            },
            software: vec![],
            industrial: vec![],
            timestamp: Utc::now(),
        };
        let response_json = serde_json::to_string(&mock_report).unwrap();

        mock_transport
            .expect_execute()
            .with(mockall::predicate::str::contains("powershell"))
            .times(1)
            .returning(move |_| Ok(response_json.clone()));

        let result = RemoteScanner::scan_with_transport(mock_transport, "powershell mock").await;

        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.system.host_name, "MOCK-PC");
    }

    #[tokio::test]
    async fn test_remote_scanner_execution_error() {
        let mut mock_transport = MockWinrmTransport::new();

        mock_transport.expect_execute().times(1).returning(|_| {
            Err(ScanError::RemoteExecution {
                host: "test".to_string(),
                message: "execution failed".to_string(),
            })
        });

        let result = RemoteScanner::scan_with_transport(mock_transport, "powershell mock").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ScanError::RemoteExecution { message, .. } => assert_eq!(message, "execution failed"),
            _ => panic!("Expected RemoteExecution error"),
        }
    }

    #[tokio::test]
    async fn test_remote_scanner_deserialization_error() {
        let mut mock_transport = MockWinrmTransport::new();

        mock_transport
            .expect_execute()
            .times(1)
            .returning(|_| Ok("{ invalid_json ]".to_string()));

        let result = RemoteScanner::scan_with_transport(mock_transport, "powershell mock").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ScanError::Deserialization(_) => {} // Expected
            _ => panic!("Expected Deserialization error"),
        }
    }
}
