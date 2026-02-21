use std::time::Duration;
use sysaudit_common::SysauditReport;

/// Unified error type for all scanning strategies.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ScanError {
    /// Local collection failure (registry, WMI, sysinfo).
    #[error("local scan failed: {0}")]
    Local(String),

    /// Remote WinRM connection or transport failure.
    #[error("remote connection to {host} failed: {message}")]
    RemoteConnection { host: String, message: String },

    /// Remote authentication rejected.
    #[error("authentication failed for {user}@{host}")]
    RemoteAuth { host: String, user: String },

    /// PowerShell execution error on remote host.
    #[error("remote execution error on {host}: {message}")]
    RemoteExecution { host: String, message: String },

    /// Response deserialization failure.
    #[error("deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    /// Operation timed out.
    #[error("operation timed out after {0:?}")]
    Timeout(Duration),
}

impl From<crate::Error> for ScanError {
    fn from(err: crate::Error) -> Self {
        ScanError::Local(err.to_string())
    }
}

/// The core strategy trait for system auditing.
///
/// Implement this to add new collection backends (Local, Remote, SSH, etc.).
pub trait Scanner: Send + Sync {
    /// Execute a full system audit.
    ///
    /// # Errors
    ///
    /// Returns [`ScanError`] if collection fails for any reason.
    fn scan(&self) -> impl std::future::Future<Output = Result<SysauditReport, ScanError>> + Send;
}
