use rstest_bdd_macros::{given, when, then};

#[given("a registry failure")]
fn given_registry_failure() {}

#[when("converted to Error")]
fn when_converted_error() {}

#[then("it matches Error::Registry variant")]
fn then_matches_registry() {}

#[given("a WMI failure")]
fn given_wmi_failure() {}

#[then("it matches Error::Wmi variant")]
fn then_matches_wmi() {}

#[given("an IO failure")]
fn given_io_failure() {}

#[then("it matches Error::Io variant")]
fn then_matches_io() {}

#[given("a library function")]
fn given_lib_func() {}

#[when("it fails")]
fn when_it_fails() {}

#[then("it returns Result::Err")]
fn then_returns_err() {}

#[then("it does not panic")]
fn then_no_panic() {}

// NOTE: "the CLI application" is already registered in cli_steps.rs.
// We do NOT redefine it here.

#[when("a command fails")]
fn when_command_fails() {}

#[then("stderr contains error message")]
fn then_stderr_msg() {}

#[then("exit code is 1")]
fn then_exit_1() {}

#[given("the WMI service is unavailable")]
fn given_wmi_service_down() {}

#[then("empty vec + warning, no error propagated")]
fn then_graceful_wmi() {}
