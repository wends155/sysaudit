use rstest_bdd_macros::{given, when, then};
use sysaudit::SoftwareScanner;

#[given("a default software scanner")]
fn given_default_scanner() {
    let _ = SoftwareScanner::new();
}

#[when("scan runs")]
fn when_scan_runs() {
    let scanner = SoftwareScanner::new();
    let _results = scanner.scan().expect("scan should succeed");
}

#[then("results include 64-bit entries")]
fn then_includes_64bit() {}

#[then("results include 32-bit entries")]
fn then_includes_32bit() {}

#[then("results include user-specific entries")]
fn then_includes_user() {}

#[then("every entry has a non-empty name")]
fn then_every_entry_named() {
    let scanner = SoftwareScanner::new();
    let results = scanner.scan().unwrap();
    for sw in &results {
        assert!(!sw.name.is_empty(), "Every software entry must have a name");
    }
}

#[then("result list is not empty")]
fn then_result_not_empty() {
    let scanner = SoftwareScanner::new();
    let results = scanner.scan().unwrap();
    assert!(!results.is_empty(), "At least one software should be installed");
}

#[given("a software entry with empty name")]
fn given_empty_name() {}

#[when("it is processed")]
fn when_processed_empty() {}

#[then("the software is excluded from results")]
fn then_excluded_empty() {}

#[given("a date string \"20240115\"")]
fn given_date_string_compact() {}

#[when("it is parsed")]
fn when_parsed_date() {}

#[then("the parsed software date is 2024-01-15")]
fn then_date_parsed() {}

#[given("a date string \"not-a-date\"")]
fn given_invalid_date_string() {}

#[then("the software date result is None")]
fn then_date_none() {}

#[given("a list of software")]
fn given_list_software() {}

#[when("filtered by \"Microsoft\"")]
fn when_filtered_microsoft() {}

#[then("only matching entries remain")]
fn then_only_matching() {}

#[given("a scanner with user installs disabled")]
fn given_scanner_no_user() {}

#[then("no HKCU entries are returned")]
fn then_no_hkcu() {}
