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
pub mod industrial;
pub mod output;
pub mod software;
pub mod system;
pub mod updates;

pub use error::Error;
pub use industrial::{IndustrialScanner, IndustrialSoftware, Vendor};
pub use software::{RegistrySource, Software, SoftwareScanner};
pub use system::{NetworkInterface, SystemInfo};
pub use updates::WindowsUpdate;
