# sysaudit

Windows System & Software Auditor - A Rust library for auditing Windows fleets locally or remotely over WinRM.

## Features

- **System Information**: OS version, build, CPU (Brand, Cores, Freq), Memory (Total/Used), Manufacturer/Model, Network Interfaces.
- **Installed Software**: Enumerate from Windows Registry (HKLM/HKCU, 32-bit and 64-bit).
- **Industrial Software Detection**: Detect SCADA and ICS vendor software (Citect/AVEVA, Digifort, ABB, Rockwell/Allen-Bradley, Siemens, Schneider Electric).
- **Windows Updates**: List installed hotfixes via WMI.
- **Local & Remote Auditing**: Perform audits directly on the host or remotely over WS-Man (WinRM).

## Architecture

The crate uses a unified `Scanner` trait to provide both local and remote data collection backends. 
It makes use of `thiserror` for library-wide error handling and `tracing` for structured observability.

### Local Auditing

Enable the `local` feature (enabled by default) to compile in the `LocalScanner`, which uses `sysinfo`, `windows-registry`, and `wmi` to collect data natively.

```rust
use sysaudit::{LocalScanner, Scanner};

#[tokio::main]
async fn main() -> Result<(), sysaudit::ScanError> {
    let report = LocalScanner.scan().await?;
    println!("Computer: {}", report.system.computer_name);
    Ok(())
}
```

### Remote Auditing

Enable the `remote` feature to enable the `RemoteScanner`, which uses `reqwest` and a builder pattern via `bon` to execute auditing payloads over WinRM.

```rust
use sysaudit::{RemoteScanner, Scanner};
use secrecy::SecretString;

#[tokio::main]
async fn main() -> Result<(), sysaudit::ScanError> {
    let scanner = RemoteScanner::builder()
        .host("192.168.1.50".to_string())
        .username("admin".to_string())
        .password(SecretString::new("hunter2".into()))
        .use_https(true)
        .build();

    let report = scanner.scan().await?;
    println!("Computer: {}", report.system.computer_name);
    Ok(())
}
```

## Detailed Scanning (Individual Components)

You can also use the underlying scanners directly:

```rust
use sysaudit::{SystemInfo, SoftwareScanner, IndustrialScanner, Vendor};

fn main() -> Result<(), sysaudit::Error> {
    // 1. System info
    let system = SystemInfo::collect()?;
    println!("OS: {} {}", system.os_name, system.build_number);

    // 2. Installed software
    let software = SoftwareScanner::new().scan()?;
    println!("Found {} software entries", software.len());

    // 3. Industrial software
    let industrial = IndustrialScanner::with_vendors(vec![
        Vendor::Rockwell,
        Vendor::Citect,
    ]).scan()?;
    
    for sw in industrial {
        println!("{}: {}", sw.vendor, sw.product);
    }

    Ok(())
}
```

## Output Formatting

The library also provides console and CSV formatting utilities under the `sysaudit::output` module.

## Verification Gate

The codebase enforces strict verification gates requiring `cargo fmt`, `cargo clippy -D warnings`, and `cargo test` to exit `0` prior to commits.

## License

MIT License
