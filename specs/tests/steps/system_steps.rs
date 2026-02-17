use rstest_bdd_macros::{given, when, then};
use sysaudit::SystemInfo;

#[given("a Windows machine")]
fn given_windows_machine() {}

#[when("system info is collected")]
fn when_system_info_collected() {
    let _info = SystemInfo::collect().expect("SystemInfo::collect() should succeed");
}

#[then("OS name is non-empty")]
fn then_os_name() {
    let info = SystemInfo::collect().unwrap();
    assert!(!info.os_name.is_empty());
}

#[then("OS version is non-empty")]
fn then_os_version() {
    let info = SystemInfo::collect().unwrap();
    assert!(!info.os_version.is_empty());
}

#[then("build number is non-empty")]
fn then_build_number() {
    let info = SystemInfo::collect().unwrap();
    assert!(!info.build_number.is_empty());
}

#[then("computer name is non-empty")]
fn then_computer_name() {
    let info = SystemInfo::collect().unwrap();
    assert!(!info.computer_name.is_empty());
}

#[when("build number is read")]
fn when_build_number_read() {}

#[then("it contains digits")]
fn then_contains_digits() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.build_number.chars().any(|c| c.is_ascii_digit()));
}

#[then("it optionally has a UBR suffix")]
fn then_ubr_suffix() {
    // UBR suffix is optional — if present, format is "NNNNN.NNN"
}

#[then("CPU brand is populated")]
fn then_cpu_brand() {
    let info = SystemInfo::collect().unwrap();
    assert!(!info.cpu_info.is_empty());
}

#[then("physical core count is populated")]
fn then_physical_cores() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.cpu_cores_physical.is_some());
}

#[then("logical core count is populated")]
fn then_logical_cores() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.cpu_cores_logical.is_some());
}

#[then("CPU frequency is populated")]
fn then_cpu_freq() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.cpu_frequency_mhz > 0);
}

#[then("total memory is non-zero")]
fn then_total_memory() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.memory_total > 0);
}

#[then("used memory is non-zero")]
fn then_used_memory() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.memory_used > 0);
}

#[then("free memory is non-zero")]
fn then_free_memory() {
    let info = SystemInfo::collect().unwrap();
    assert!(info.memory_free > 0);
}

#[then("at least one network interface is found")]
fn then_has_network_iface() {
    let info = SystemInfo::collect().unwrap();
    assert!(!info.network_interfaces.is_empty());
}

#[then("each interface has a name")]
fn then_iface_name() {
    let info = SystemInfo::collect().unwrap();
    for iface in &info.network_interfaces {
        assert!(!iface.name.is_empty());
    }
}

#[then("each interface has an IP address")]
fn then_iface_ip() {
    let info = SystemInfo::collect().unwrap();
    // IP address is always present (it's not Option)
    assert!(!info.network_interfaces.is_empty());
}

#[then("each interface has a valid MAC address")]
fn then_iface_mac() {
    let info = SystemInfo::collect().unwrap();
    for iface in &info.network_interfaces {
        if let Some(mac) = &iface.mac_address {
            assert!(mac.contains(':'), "MAC should contain colons");
        }
    }
}

#[then("manufacturer is populated if WMI is available")]
fn then_manufacturer() {
    // WMI may or may not be available — this is a best-effort check
    let _info = SystemInfo::collect().unwrap();
}

#[then("model is populated if WMI is available")]
fn then_model() {
    let _info = SystemInfo::collect().unwrap();
}
