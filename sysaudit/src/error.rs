//! Error types for sysaudit.

use thiserror::Error;

/// Main error type for sysaudit operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Registry access error
    #[error("Registry error: {0}")]
    Registry(#[from] windows_result::Error),

    /// WMI query error
    #[error("WMI error: {0}")]
    Wmi(#[from] wmi::WMIError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV writing error
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    /// JSON serialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Date parsing error
    #[error("Date parse error: {0}")]
    DateParse(String),

    /// General error with message
    #[error("{0}")]
    General(String),
}
