#![allow(unused)]
// cargo test -p ch2 --test cli
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use rand;
use rand::{Rng, distr::Alphanumeric};

const PRG: &str = "ch2-echor";
const EMPTY: &str = "tests/expected/empty.txt";
const FOX: &str = "tests/expected/fox.txt";
const SPIDERS: &str = "tests/expected/spiders.txt";
const BUSTLE: &str = "tests/expected/the-bustle.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("ch2-echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("ch2-echor").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("ch2-echor").unwrap();
    cmd.arg("hello world").assert().success();
}
#[test]
fn hello0() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("ch2-echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_new_line() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_new_line() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
