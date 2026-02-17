use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "-p", "sysaudit-cli", "--", "--help"])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Windows System & Software Auditor"));
    assert!(stdout.contains("Commands:"));
}

#[test]
fn test_cli_invalid_arg() {
    let output = Command::new("cargo")
        .args(["run", "-p", "sysaudit-cli", "--", "--invalid-flag"])
        .output()
        .expect("Failed to run CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument"));
}

#[test]
fn test_cli_system_cmd() {
    let output = Command::new("cargo")
        .args(["run", "-p", "sysaudit-cli", "--", "system"])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SYSTEM INFORMATION"));
    assert!(stdout.contains("Computer Name"));
}

#[test]
fn test_cli_software_cmd() {
    // This might take longer, but good integration test
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "sysaudit-cli",
            "--",
            "software",
            "--format",
            "json",
            "--filter",
            "Microsoft",
        ])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should be valid JSON array
    assert!(stdout.trim().starts_with('['));
    assert!(stdout.trim().ends_with(']'));
}
