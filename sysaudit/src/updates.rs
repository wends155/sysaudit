//! Windows Updates module.
//!
//! Provides read-only access to installed Windows Updates via WMI.


use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use wmi::{COMLibrary, WMIConnection};

/// Windows Update / Hotfix entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsUpdate {
    /// Hotfix ID (e.g., "KB5034441")
    pub hotfix_id: String,
    /// Description
    pub description: Option<String>,
    /// Installation date
    pub installed_on: Option<NaiveDate>,
    /// Installed by user
    pub installed_by: Option<String>,
}

/// WMI result struct for Win32_QuickFixEngineering.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32QuickFixEngineering {
    #[serde(rename = "HotFixID")]
    hot_fix_id: Option<String>,
    description: Option<String>,
    installed_on: Option<String>,
    installed_by: Option<String>,
}

impl WindowsUpdate {
    /// Collect all installed Windows Updates (READ-ONLY).
    ///
    /// Returns empty vec if WMI query fails (graceful degradation).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sysaudit::WindowsUpdate;
    ///
    /// let updates = WindowsUpdate::collect_all();
    /// for update in updates {
    ///     println!("{}: {:?}", update.hotfix_id, update.description);
    /// }
    /// ```
    pub fn collect_all() -> Vec<Self> {
        match Self::try_collect() {
            Ok(updates) => updates,
            Err(e) => {
                eprintln!("Warning: Could not query Windows Updates: {}", e);
                Vec::new()
            }
        }
    }

    fn try_collect() -> Result<Vec<Self>, crate::Error> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        let results: Vec<Win32QuickFixEngineering> = wmi_con.query()?;

        let updates = results
            .into_iter()
            .filter_map(|r| {
                let hotfix_id = r.hot_fix_id?;
                
                // Skip empty hotfix IDs
                if hotfix_id.trim().is_empty() {
                    return None;
                }

                let installed_on = r.installed_on.as_ref().and_then(|s| parse_wmi_date(s.as_str()));

                Some(WindowsUpdate {
                    hotfix_id,
                    description: r.description.filter(|s| !s.is_empty()),
                    installed_on,
                    installed_by: r.installed_by.filter(|s| !s.is_empty()),
                })
            })
            .collect();

        Ok(updates)
    }
}

/// Parse WMI date format (various formats possible).
fn parse_wmi_date(s: &str) -> Option<NaiveDate> {
    // Try common formats
    // MM/DD/YYYY
    if let Ok(date) = NaiveDate::parse_from_str(s, "%m/%d/%Y") {
        return Some(date);
    }
    // YYYY-MM-DD
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(date);
    }
    // YYYYMMDD
    if s.len() == 8 {
        if let (Ok(year), Ok(month), Ok(day)) = (
            s[0..4].parse(),
            s[4..6].parse(),
            s[6..8].parse(),
        ) {
            return NaiveDate::from_ymd_opt(year, month, day);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wmi_date_slash() {
        assert_eq!(
            parse_wmi_date("01/15/2024"),
            NaiveDate::from_ymd_opt(2024, 1, 15)
        );
    }

    #[test]
    fn test_parse_wmi_date_iso() {
        assert_eq!(
            parse_wmi_date("2024-01-15"),
            NaiveDate::from_ymd_opt(2024, 1, 15)
        );
    }
}
