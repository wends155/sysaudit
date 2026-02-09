# Data Sources

This document describes where `sysaudit` retrieves its information from the Windows system.

## 1. System Information

| Field | Source | Details |
|-------|--------|---------|
| **OS Name** | API | via `sysinfo` crate |
| **OS Version** | API | via `sysinfo` crate |
| **Build Number** | Registry | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion` <br> Keys: `CurrentBuild`, `UBR` |
| **Computer Name** | API | via `sysinfo` crate |
| **Domain** | Registry | `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters` <br> Key: `Domain` |
| **CPU** | API | via `sysinfo` crate |
| **Network Interfaces** | API | via `sysinfo` crate (enumerates IP, Subnet, MAC) |

## 2. Installed Software

The application scans the Windows Registry for installed software. It does **not** use the Windows Installer API (MSI) directly, but rather the Uninstall keys which populate "Add/Remove Programs".

### Registry Locations Scanned
1.  **System-wide (64-bit)**
    *   `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`
2.  **System-wide (32-bit on 64-bit OS)**
    *   `HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall`
3.  **Per-User**
    *   `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`

### Fields Retrieved
*   `DisplayName` (Required)
*   `DisplayVersion`
*   `Publisher`
*   `InstallDate` (Format: YYYYMMDD)
*   `InstallLocation`

## 3. Industrial Software
Identifies software from specific vendors (Citect, Rockwell, Siemens, etc.) by filtering the software list obtained from the registry locations above.

## 4. Windows Updates
Current implementation uses **WMI** (Windows Management Instrumentation).

*   **Class**: `Win32_QuickFixEngineering`
*   **Fields**: `HotFixID`, `Description`, `InstalledOn`, `InstalledBy`
*   **Note**: If WMI is corrupted or the class is missing, this section returns an empty list with a warning.
