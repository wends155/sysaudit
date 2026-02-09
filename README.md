# sysaudit

Windows System & Software Auditor - A Rust library and CLI tool for auditing Windows systems.

## Features

- **System Information**: OS version, build number, CPU, network interfaces (IP, subnet mask, gateway)
- **Installed Software**: Enumerate from Windows Registry (HKLM/HKCU, 32/64-bit)
- **Industrial Software Detection**: Citect/AVEVA, Digifort, ABB, Rockwell/Allen-Bradley, Siemens, Schneider Electric
- **Windows Updates**: List installed hotfixes via WMI
- **Output Formats**: Console tables, JSON, CSV

## Installation

```bash
# From crates.io
cargo install sysaudit-cli

# From source
git clone https://github.com/wends155/sysaudit.git
cd sysaudit
cargo install --path sysaudit-cli
```

## CLI Usage

```bash
# System information
sysaudit system
sysaudit system --format json

# Installed software
sysaudit software
sysaudit software --filter "Microsoft"
sysaudit software --format csv --output software.csv

# Industrial software
sysaudit industrial
sysaudit industrial --vendors citect,rockwell

# Windows updates
sysaudit updates
sysaudit updates --format json

# Full audit
sysaudit all --output report.csv
```

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

## Build

```bash
make build      # Debug build
make release    # Release build
make test       # Run tests
make lint       # Run clippy
make docs       # Generate docs
```

## License

MIT License - see [LICENSE](LICENSE)
