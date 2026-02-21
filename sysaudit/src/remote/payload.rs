//! PowerShell payload executed remotely via WinRM.

/// The PowerShell script that collects system, software, and industrial data.
/// It outputs a JSON string matching the `SysauditReport` structure.
pub const WINRM_PAYLOAD: &str = r#"
$ErrorActionPreference = "Stop"

function Get-HardwareInfo {
    $os = Get-CimInstance Win32_OperatingSystem
    $cs = Get-CimInstance Win32_ComputerSystem
    $cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
    
    # Calculate Build Number + UBR
    $regCurrentVersion = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion"
    $build = if ($null -ne $regCurrentVersion.UBR) { "$($regCurrentVersion.CurrentBuild).$($regCurrentVersion.UBR)" } else { $regCurrentVersion.CurrentBuild }

    # Calculate Used RAM
    $totalRam = $cs.TotalPhysicalMemory
    $freeRam = $os.FreePhysicalMemory * 1024
    $usedRam = $totalRam - $freeRam

    # Networking
    $netInterfaces = @()
    $adapters = Get-NetAdapter | Where-Object Status -eq "Up"
    foreach ($adapter in $adapters) {
        $ips = Get-NetIPAddress -InterfaceIndex $adapter.ifIndex
        foreach ($ip in $ips) {
            $ipVer = if ($ip.AddressFamily -eq "IPv4") { "IPv4" } else { "IPv6" }
            $netInterfaces += [ordered]@{
                name = $adapter.Name
                ip_address = $ip.IPAddress
                ip_version = $ipVer
                mac_address = $adapter.MacAddress
            }
        }
    }

    $system = [ordered]@{
        os_name = $os.Caption
        os_version = $os.Version
        host_name = $env:COMPUTERNAME
        cpu_info = $cpu.Name
        cpu_physical_cores = $cpu.NumberOfCores
        memory_total_bytes = [uint64]$totalRam
        memory_used_bytes = [uint64]$usedRam
        manufacturer = $cs.Manufacturer
        model = $cs.Model
        network_interfaces = $netInterfaces
    }
    return $system
}

function Get-InstalledSoftware {
    $software = @()
    $paths = @(
        "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*",
        "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*"
    )
    
    foreach ($path in $paths) {
        $keys = Get-ItemProperty $path -ErrorAction SilentlyContinue | Where-Object DisplayName -ne $null
        foreach ($key in $keys) {
            # Format date nicely if possible, or leave null to let Rust handle it
            $date = $null
            if ($null -ne $key.InstallDate -and $key.InstallDate.Length -eq 8) {
                # Format: YYYYMMDD string is fine, Rust expects NaiveDate
                $dateString = "$($key.InstallDate.Substring(0,4))-$($key.InstallDate.Substring(4,2))-$($key.InstallDate.Substring(6,2))T00:00:00Z"
                $date = $dateString
            }
            
            $software += [ordered]@{
                name = $key.DisplayName
                version = if ($null -ne $key.DisplayVersion) { $key.DisplayVersion.ToString() } else { $null }
                vendor = if ($null -ne $key.Publisher) { $key.Publisher.ToString() } else { $null }
                install_date = $date
            }
        }
    }
    return $software
}

function Get-IndustrialSoftware {
    # Stubbed implementation based on LocalScanner logic
    $industrial = @()
    
    # Rockwell
    $rockwellPath = "HKLM:\SOFTWARE\WOW6432Node\Rockwell Software"
    if (Test-Path $rockwellPath) {
        Get-ChildItem $rockwellPath -ErrorAction SilentlyContinue | ForEach-Object {
            $industrial += [ordered]@{
                vendor = "Rockwell"
                product = $_.PSChildName
                version = $null
                install_path = $null
            }
        }
    }
    
    return $industrial
}

# Assemble Final Structure
$report = [ordered]@{
    system = Get-HardwareInfo
    software = Get-InstalledSoftware
    industrial = Get-IndustrialSoftware
    timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
}

# Convert to JSON with maximum depth to prevent truncation
$report | ConvertTo-Json -Depth 5 -Compress
"#;
