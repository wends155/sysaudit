use rstest_bdd_macros::{given, when, then};

#[given("the CLI application")]
fn given_cli_app() {}

#[when("run with \"--help\" flag")]
fn when_run_help() {}

#[then("stdout contains \"Windows System & Software Auditor\"")]
fn then_stdout_help() {}

#[when("run with \"--version\" flag")]
fn when_run_version() {}

#[then("stdout contains version number")]
fn then_stdout_version() {}

#[when("run with invalid argument")]
fn when_run_invalid() {}

#[then("stderr contains error information")]
fn then_stderr_error() {}

#[then("exit code is non-zero")]
fn then_exit_nonzero() {}
