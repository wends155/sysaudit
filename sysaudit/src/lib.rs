//! # sysaudit
//!
//! Windows System & Software Auditor library.
//!
//! Provides read-only access to:
//! - System information (OS, CPU, network interfaces)
//! - Installed software (from Windows Registry)
//! - Industrial software detection (Citect, ABB, Rockwell, etc.)
//! - Windows Updates (via WMI)
//!
//! ## Example
//!
//! ```no_run
//! use sysaudit::{SystemInfo, SoftwareScanner};
//!
//! fn main() -> Result<(), sysaudit::Error> {
//!     let system = SystemInfo::collect()?;
//!     println!("Computer: {}", system.computer_name);
//!
//!     let software = SoftwareScanner::new().scan()?;
//!     for sw in software {
//!         println!("{} v{}", sw.name, sw.version.as_deref().unwrap_or("?"));
//!     }
//!     Ok(())
//! }
//! ```

pub mod error;
#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "remote")]
pub mod remote;
pub mod scanner;

#[cfg(feature = "local")]
pub mod industrial;
#[cfg(feature = "local")]
pub mod output;
#[cfg(feature = "local")]
pub mod software;
#[cfg(feature = "local")]
pub mod system;
#[cfg(feature = "local")]
pub mod updates;

pub use error::Error;
pub use scanner::{ScanError, Scanner};

#[cfg(feature = "local")]
pub use local::LocalScanner;
#[cfg(feature = "remote")]
pub use remote::RemoteScanner;

#[cfg(feature = "local")]
pub use industrial::{IndustrialScanner, IndustrialSoftware, Vendor};
#[cfg(feature = "local")]
pub use software::{RegistrySource, Software, SoftwareScanner};
#[cfg(feature = "local")]
pub use system::{NetworkInterface, SystemInfo};
#[cfg(feature = "local")]
pub use updates::WindowsUpdate;
