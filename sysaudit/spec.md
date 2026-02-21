# Behavioral Spec: sysaudit

## 1. Module Contracts

### `scanner` — Strategy trait

**Purpose**: Define the unified auditing interface implemented by all backends.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `Scanner` | `async fn scan(&self) -> Result<SysauditReport, ScanError>` | `ScanError::*` | Never panics. Result is always a complete `SysauditReport` or an error. |
| `ScanError` | `#[non_exhaustive]` enum | — | All variants carry host/context strings for diagnostics. |

**Required test coverage**:
- [x] Success path returns a valid `SysauditReport`
- [x] `RemoteExecution` error is propagated correctly
- [x] Deserialization error is wrapped in `ScanError::Deserialization`

---

### `local` — Local collection

**Purpose**: Collect system data from the local Windows machine.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `LocalScanner::scan` | `async fn scan(&self) -> Result<SysauditReport, ScanError>` | `ScanError::Local` wrapping any `crate::Error` | Always collects system + software + industrial; never panics. |

**Required test coverage**:
- [x] `test_local_scanner_produces_report` — non-empty host_name and os_name

---

### `remote` — Remote collection via WinRM

**Purpose**: Execute a PowerShell audit payload on a remote Windows host via WinRM.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `RemoteScanner::scan` | `async fn scan(&self) -> Result<SysauditReport, ScanError>` | `RemoteConnection`, `RemoteExecution`, `Deserialization` | Base64-encodes payload for `EncodedCommand`. Never panics. |

**Required test coverage**:
- [x] Success path with mock transport
- [x] `RemoteExecution` error propagated
- [x] `Deserialization` error on invalid JSON

---

### `system` — System information

**Purpose**: Read OS, CPU, RAM, and network information from the local machine.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `SystemInfo::collect` | `fn collect() -> Result<SystemInfo, Error>` | `Error::Registry` | Always returns non-empty `computer_name` and `os_name` on a valid Windows system. |

**Required test coverage**:
- [x] `computer_name` non-empty
- [x] `os_name` non-empty
- [x] `build_number` contains digits
- [x] MAC address format `XX:XX:XX:XX:XX:XX`

---

### `software` — Installed software

**Purpose**: Enumerate installed software from Windows registry Uninstall keys.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `SoftwareScanner::scan` | `fn scan(&self) -> Result<Vec<Software>, Error>` | `Error::Registry` | Result is sorted by name (case-insensitive). No entries with empty `DisplayName`. |

**Required test coverage**:
- [x] `parse_install_date` valid YYYYMMDD
- [x] `parse_install_date` invalid inputs → `None`
- [x] Future dates are not rejected

---

### `industrial` — Industrial software detection

**Purpose**: Detect SCADA/ICS vendor software via registry pattern matching.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `IndustrialScanner::scan` | `fn scan(&self) -> Result<Vec<IndustrialSoftware>, Error>` | `Error::Registry` | Result is deduplicated by product name. |

**Required test coverage**:
- [ ] Rockwell registry key presence is detected (environment-dependent; deferred)

---

### `updates` — Windows Update history

**Purpose**: Query installed hotfixes from WMI `Win32_QuickFixEngineering`.

| Symbol | Signature | Errors | Invariants |
|--------|-----------|--------|------------|
| `WindowsUpdate::collect_all` | `fn collect_all() -> Vec<WindowsUpdate>` | Never errors (graceful degradation) | Returns empty `Vec` on WMI failure with a `tracing::warn!`. |

**Required test coverage**:
- [x] `parse_wmi_date` MM/DD/YYYY format
- [x] `parse_wmi_date` ISO YYYY-MM-DD format

---

### `output` — Formatting

**Purpose**: Format audit data for console display and CSV export.

| Symbol | Errors | Invariants |
|--------|--------|------------|
| `ConsoleFormatter::format_*` | Never errors | Always returns a non-empty `String`. |
| `CsvExporter::export_*` | `Error::Csv`, `Error::Io` | Flushes before returning `Ok`. |

**Required test coverage**:
- [x] Software table contains expected columns
- [x] Empty updates table renders correctly

---

## 2. Data Models

### `SysauditReport` (from `sysaudit-common`)

| Field | Type | Required | Constraint |
|-------|------|----------|------------|
| `system` | `SystemInfoDto` | ✅ | Non-null |
| `software` | `Vec<SoftwareDto>` | ✅ | May be empty |
| `industrial` | `Vec<IndustrialSoftwareDto>` | ✅ | May be empty |
| `timestamp` | `DateTime<Utc>` | ✅ | Must be UTC |

### `ScanError`

| Variant | When |
|---------|------|
| `Local(String)` | Any `crate::Error` from local collection |
| `RemoteConnection{host, message}` | HTTP client build failure or network error |
| `RemoteAuth{host, user}` | WinRM 401 Unauthorized |
| `RemoteExecution{host, message}` | PowerShell non-zero exit or WS-Man fault |
| `Deserialization(serde_json::Error)` | JSON parse failure from remote response |
| `Timeout(Duration)` | Operation exceeded configured timeout |

### `Software`

| Field | Type | Required |
|-------|------|----------|
| `name` | `String` | ✅ non-empty |
| `version` | `Option<String>` | ❌ |
| `publisher` | `Option<String>` | ❌ |
| `install_date` | `Option<NaiveDate>` | ❌ |
| `install_location` | `Option<PathBuf>` | ❌ |
| `source` | `RegistrySource` | ✅ |

### `IndustrialSoftware`

| Field | Type | Required |
|-------|------|----------|
| `vendor` | `Vendor` | ✅ |
| `product` | `String` | ✅ non-empty |
| `version` | `Option<String>` | ❌ |
| `install_path` | `Option<PathBuf>` | ❌ |

---

## 3. Integration Points

### WinRM Transport

| Aspect | Specification |
|--------|--------------|
| Protocol | WS-Man over HTTP (port 5985) or HTTPS (port 5986) |
| Auth | HTTP Basic (username + password) |
| Payload | UTF-16LE + Base64 encoded PowerShell via `-EncodedCommand` |
| Response | UTF-8 JSON matching `SysauditReport` structure |
| Timeout | Configurable via `RemoteScanner::builder().timeout()` (default: 30s) |
| TLS | `skip_cert_verify` flag (currently pending `reqwest/rustls` API) |
| Error boundary | All transport errors map to `ScanError` variants at the boundary |
