# sysaudit

Windows System & Software Auditor - A Rust library for auditing Windows systems.

## Features

- **System Information**: OS version, build, CPU (Brand, Cores, Freq), Memory (Total/Used), Manufacturer/Model, Network Interfaces
- **Installed Software**: Enumerate from Windows Registry (HKLM/HKCU, 32/64-bit)
- **Industrial Software Detection**: Citect/AVEVA, Digifort, ABB, Rockwell/Allen-Bradley, Siemens, Schneider Electric
- **Windows Updates**: List installed hotfixes via WMI

## Library Usage

```rust
use sysaudit::{SystemInfo, SoftwareScanner, IndustrialScanner, Vendor};

fn main() -> Result<(), sysaudit::Error> {
    // System info
    let system = SystemInfo::collect()?;
    println!("Computer: {}", system.computer_name);
    println!("OS: {} {}", system.os_name, system.build_number);

    // Installed software
    let software = SoftwareScanner::new().scan()?;
    println!("Found {} software entries", software.len());

    // Industrial software
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

## License

MIT License
