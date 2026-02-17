use rstest_bdd_macros::scenario;

// Import step modules so their macros register into the global step registry
mod steps;

#[scenario(path = "features/system_info.feature")]
fn system_info_feature() {}

#[scenario(path = "features/software_scan.feature")]
fn software_scan_feature() {}

#[scenario(path = "features/industrial_detection.feature")]
fn industrial_detection_feature() {}

#[scenario(path = "features/windows_updates.feature")]
fn windows_updates_feature() {}

#[scenario(path = "features/cli_interface.feature")]
fn cli_interface_feature() {}

#[scenario(path = "features/output_formats.feature")]
fn output_formats_feature() {}

#[scenario(path = "features/error_handling.feature")]
fn error_handling_feature() {}
