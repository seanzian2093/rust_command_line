#![allow(unused)]
// cargo test --test cli2 -q
// cargo test --test cli2 --test-thread=1 to ensure test in order
use assert_cmd::Command;

#[test]
fn run_rust_command_line() {
    // assert_cmd::Command is able to find cargo bin in `target/debug/`
    let mut cmd = Command::cargo_bin("rust_command_line").unwrap();
    cmd.assert().success();
}

#[test]
fn run_true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
#[test]
fn run_false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
#[test]
fn run_rust_command_line_output() {
    // assert_cmd::Command is able to find cargo bin in `target/debug/`
    let mut cmd = Command::cargo_bin("ch1").unwrap();
    // Remember the terminating new line character
    cmd.assert().success().stdout("Hello, world from rust_command_line ch1!\n");
}
