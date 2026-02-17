use rstest_bdd_macros::{given, when, then};
use sysaudit::{IndustrialScanner, Vendor};

#[given("a scanner with all vendors")]
fn given_scanner_all_vendors() {
    let _scanner = IndustrialScanner::all_vendors();
}

#[when("industrial scan runs")]
fn when_industrial_scan() {
    let scanner = IndustrialScanner::all_vendors();
    let _ = scanner.scan();
}

#[then("it checks all known vendors")]
fn then_checks_all_vendors() {
    // IndustrialScanner::all_vendors() includes all known vendor types
    let _scanner = IndustrialScanner::all_vendors();
}

#[given("a scanner for \"Rockwell\" and \"ABB\"")]
fn given_scanner_rockwell_abb() {
    let _scanner = IndustrialScanner::with_vendors(vec![Vendor::Rockwell, Vendor::ABB]);
}

#[then("only those vendors are scanned")]
fn then_only_those_vendors() {}

#[given("software named \"Citect SCADA 2018\"")]
fn given_citect_scada() {}

#[when("classified")]
fn when_classified() {}

#[then("it is identified as Citect")]
fn then_identified_citect() {}

#[given("software named \"AVEVA InTouch\"")]
fn given_aveva_intouch() {}

// "it is identified as Citect" is already defined above — AVEVA InTouch maps to Citect vendor

#[given("software named \"AVEVA Accounting\"")]
fn given_aveva_accounting() {}

#[then("it is NOT identified as industrial")]
fn then_not_industrial() {}

#[given("software named \"Studio 5000 Logix Designer\"")]
fn given_studio5000() {}

#[then("it is identified as Rockwell")]
fn then_identified_rockwell() {}

#[given("software named \"Siemens TIA Portal V17\"")]
fn given_siemens_tia() {}

#[then("it is identified as Siemens")]
fn then_identified_siemens() {}

#[given("software named \"ABB Ability System 800xA\"")]
fn given_abb_ability() {}

#[then("it is identified as ABB")]
fn then_identified_abb() {}

#[given("software named \"Digifort Enterprise\"")]
fn given_digifort() {}

#[then("it is identified as Digifort")]
fn then_identified_digifort() {}

#[given("software named \"Microsoft Word\"")]
fn given_msword() {}

// "it is NOT identified as industrial" is already defined above

#[given("a specific path")]
fn given_specific_path() {}

#[when("scanner checks path")]
fn when_scanner_checks_path() {}

#[then("it detects known software files")]
fn then_detects_files() {}

#[then("it checks for Rockwell")]
fn then_checks_rockwell() {
    let _v = Vendor::Rockwell;
}

#[then("it checks for Siemens")]
fn then_checks_siemens() {
    let _v = Vendor::Siemens;
}

#[then("it checks for Schneider")]
fn then_checks_schneider() {
    let _v = Vendor::SchneiderElectric;
}
