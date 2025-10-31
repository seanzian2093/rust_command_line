#![allow(unused)]
// std::process:Command is not very handy
use std::process::Command;

#[test]
fn works() {
    assert!(true)
}

#[test]
fn run_ls() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn run_rust_command_line() {
    // Command is not able to find the bin of rust_command_line so res is an error
    let mut cmd = Command::new("ch1");
    let res = cmd.output();
    assert!(res.is_err());
}
