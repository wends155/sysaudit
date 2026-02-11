# sysaudit-cli

CLI for Windows system auditor - audit Windows systems for software, updates, and hardware information.

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

## Installation

```bash
cargo install sysaudit-cli
```

## License

MIT License
