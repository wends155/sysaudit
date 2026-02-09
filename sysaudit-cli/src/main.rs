//! sysaudit CLI - Windows System & Software Auditor
//!
//! A command-line tool to audit Windows system configuration,
//! installed software, and Windows Update patches.

use clap::{Parser, Subcommand};
use sysaudit::{
    SystemInfo, SoftwareScanner, IndustrialScanner, WindowsUpdate, Vendor,
    output::{ConsoleFormatter, CsvExporter},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sysaudit")]
#[command(author, version, about = "Windows System & Software Auditor")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display system information
    System {
        /// Output format: table, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// List installed software
    Software {
        /// Filter by name (case-insensitive)
        #[arg(short, long)]
        filter: Option<String>,

        /// Output format: table, json, csv
        #[arg(long, default_value = "table")]
        format: String,

        /// Output file for csv format
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Detect industrial software
    Industrial {
        /// Comma-separated vendor list (citect,rockwell,abb,siemens,schneider,digifort)
        #[arg(short, long)]
        vendors: Option<String>,

        /// Output format: table, json, csv
        #[arg(long, default_value = "table")]
        format: String,

        /// Output file for csv format
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List Windows Updates / Hotfixes
    Updates {
        /// Output format: table, json, csv
        #[arg(long, default_value = "table")]
        format: String,

        /// Output file for csv format
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Run full audit
    All {
        /// Output file (CSV)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::System { format } => cmd_system(&format),
        Commands::Software { filter, format, output } => cmd_software(filter.as_deref(), &format, output.as_deref()),
        Commands::Industrial { vendors, format, output } => cmd_industrial(vendors.as_deref(), &format, output.as_deref()),
        Commands::Updates { format, output } => cmd_updates(&format, output.as_deref()),
        Commands::All { output } => cmd_all(output.as_deref()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn cmd_system(format: &str) -> Result<(), sysaudit::Error> {
    let info = SystemInfo::collect()?;

    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&info)?),
        _ => println!("{}", ConsoleFormatter::format_system_info(&info)),
    }

    Ok(())
}

fn cmd_software(filter: Option<&str>, format: &str, output: Option<&std::path::Path>) -> Result<(), sysaudit::Error> {
    let mut software = SoftwareScanner::new().scan()?;

    // Apply filter
    if let Some(f) = filter {
        let f_lower = f.to_lowercase();
        software.retain(|sw| sw.name.to_lowercase().contains(&f_lower));
    }

    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&software)?),
        "csv" => {
            let path = output.unwrap_or(std::path::Path::new("software.csv"));
            CsvExporter::export_software(&software, path)?;
            println!("Exported {} items to {}", software.len(), path.display());
        }
        _ => println!("{}", ConsoleFormatter::format_software(&software)),
    }

    Ok(())
}

fn cmd_industrial(vendors: Option<&str>, format: &str, output: Option<&std::path::Path>) -> Result<(), sysaudit::Error> {
    let scanner = if let Some(v) = vendors {
        let vendor_list: Vec<Vendor> = v
            .split(',')
            .filter_map(|s| match s.trim().to_lowercase().as_str() {
                "citect" => Some(Vendor::Citect),
                "rockwell" | "allen-bradley" => Some(Vendor::Rockwell),
                "abb" => Some(Vendor::ABB),
                "siemens" => Some(Vendor::Siemens),
                "schneider" => Some(Vendor::SchneiderElectric),
                "digifort" => Some(Vendor::Digifort),
                _ => None,
            })
            .collect();
        IndustrialScanner::with_vendors(vendor_list)
    } else {
        IndustrialScanner::all_vendors()
    };

    let industrial = scanner.scan()?;

    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&industrial)?),
        "csv" => {
            let path = output.unwrap_or(std::path::Path::new("industrial.csv"));
            CsvExporter::export_industrial(&industrial, path)?;
            println!("Exported {} items to {}", industrial.len(), path.display());
        }
        _ => println!("{}", ConsoleFormatter::format_industrial(&industrial)),
    }

    Ok(())
}

fn cmd_updates(format: &str, output: Option<&std::path::Path>) -> Result<(), sysaudit::Error> {
    let updates = WindowsUpdate::collect_all()?;

    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&updates)?),
        "csv" => {
            let path = output.unwrap_or(std::path::Path::new("updates.csv"));
            CsvExporter::export_updates(&updates, path)?;
            println!("Exported {} items to {}", updates.len(), path.display());
        }
        _ => println!("{}", ConsoleFormatter::format_updates(&updates)),
    }

    Ok(())
}

fn cmd_all(output: Option<&std::path::Path>) -> Result<(), sysaudit::Error> {
    println!("Running full system audit...\n");

    // System info
    let system = SystemInfo::collect()?;
    println!("{}\n", ConsoleFormatter::format_system_info(&system));

    // Software
    let software = SoftwareScanner::new().scan()?;
    println!("{}\n", ConsoleFormatter::format_software(&software));

    // Industrial
    let industrial = IndustrialScanner::all_vendors().scan()?;
    if !industrial.is_empty() {
        println!("{}\n", ConsoleFormatter::format_industrial(&industrial));
    }

    // Updates
    let updates = WindowsUpdate::collect_all()?;
    println!("{}\n", ConsoleFormatter::format_updates(&updates));

    // Export to CSV if requested
    if let Some(path) = output {
        CsvExporter::export_software(&software, path)?;
        println!("Exported to {}", path.display());
    }

    Ok(())
}
