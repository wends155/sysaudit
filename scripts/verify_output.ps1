$ErrorActionPreference = "Stop"

Write-Host "Verifying sysaudit output against Windows System Info..."

# 1. Get Windows Info via CIM
$cimCS = Get-CimInstance Win32_ComputerSystem
$cimOS = Get-CimInstance Win32_OperatingSystem
$cimProc = Get-CimInstance Win32_Processor | Select-Object -First 1

# 2. Get sysaudit Info (JSON)
# Ensure we are in project root or valid path
if (Test-Path ".\target\debug\sysaudit.exe") {
    $jsonRaw = & ".\target\debug\sysaudit.exe" system --format json
} else {
    Write-Host "Building project first..."
    cargo build
    $jsonRaw = & ".\target\debug\sysaudit.exe" system --format json
}

$sysaudit = $jsonRaw | ConvertFrom-Json

# 3. Compare Results

# Manufacturer
if ($sysaudit.manufacturer -ne $cimCS.Manufacturer) {
    Write-Warning "Manufacturer mismatch! Sysaudit: '$($sysaudit.manufacturer)' vs Windows: '$($cimCS.Manufacturer)'"
} else {
    Write-Host "Manufacturer: OK ($($sysaudit.manufacturer))" -ForegroundColor Green
}

# Model
if ($sysaudit.model -ne $cimCS.Model) {
    Write-Warning "Model mismatch! Sysaudit: '$($sysaudit.model)' vs Windows: '$($cimCS.Model)'"
} else {
    Write-Host "Model: OK ($($sysaudit.model))" -ForegroundColor Green
}

# Total Memory (Allow small difference due to byte conversions or reversed bytes?)
# Actually sysinfo usually matches well.
$memDiff = [math]::Abs($sysaudit.memory_total - $cimCS.TotalPhysicalMemory)
if ($memDiff -gt 1048576) { # 1MB tolerance
    Write-Warning "Memory mismatch! Sysaudit: $($sysaudit.memory_total) vs Windows: $($cimCS.TotalPhysicalMemory)"
} else {
    Write-Host "Total Memory: OK" -ForegroundColor Green
}

# CPU Cores
if ($sysaudit.cpu_cores_physical -ne $cimProc.NumberOfCores) {
    Write-Warning "Physical Cores mismatch! Sysaudit: $($sysaudit.cpu_cores_physical) vs Windows: $($cimProc.NumberOfCores)"
} else {
    Write-Host "Physical Cores: OK ($($sysaudit.cpu_cores_physical))" -ForegroundColor Green
}

if ($sysaudit.cpu_cores_logical -ne $cimProc.NumberOfLogicalProcessors) {
    Write-Warning "Logical Cores mismatch! Sysaudit: $($sysaudit.cpu_cores_logical) vs Windows: $($cimProc.NumberOfLogicalProcessors)"
} else {
    Write-Host "Logical Cores: OK ($($sysaudit.cpu_cores_logical))" -ForegroundColor Green
}


Write-Host "Verification Complete."
