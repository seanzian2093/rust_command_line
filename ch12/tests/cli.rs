#![allow(deprecated, unused)]
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distr::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "ch12-fortuner";
const FORTUNE_DIR: &str = "./tests/inputs";
const EMPTY_DIR: &str = "./tests/inputs/empty";
const JOKES: &str = "./tests/inputs/jokes.txt";
const LITERATURE: &str = "./tests/inputs/literature.txt";
const QUOTES: &str = "./tests/inputs/quotes.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
fn random_string() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_not_enough_args() -> TestResult {
    let expected = "the following required arguments were not provided:\n  \
        <FILE>...";
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args([LITERATURE, &bad])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_pattern() -> TestResult {
    let expected = r#"Invalid --pattern "*""#;
    Command::cargo_bin(PRG)?
        .args(["--pattern", "*", LITERATURE])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_seed() -> TestResult {
    let bad = random_string();
    let expected = format!("\"{}\" not a valid integer", bad);
    Command::cargo_bin(PRG)?
        .args([LITERATURE, "--seed", &bad])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn no_fortunes_found() -> TestResult {
    run(&[EMPTY_DIR], "No fortunes found\n")
}

// --------------------------------------------------
#[test]
fn quotes_seed_1() -> TestResult {
    run(
        &[QUOTES, "-s", "1"],
        "You can observe a lot just by watching.\n-- Yogi Berra\n",
    )
}

// --------------------------------------------------
#[test]
fn jokes_seed_1() -> TestResult {
    run(
        &[JOKES, "-s", "1"],
        "Q: What happens when frogs park illegally?\nA: They get toad.\n",
    )
}

// --------------------------------------------------
#[test]
fn dir_seed_10() -> TestResult {
    run(
        &[FORTUNE_DIR, "-s", "10"],
        "Q: Why did the fungus and the alga marry?\n\
        A: Because they took a lichen to each other!\n",
    )
}

// --------------------------------------------------
fn run(args: &[&str], expected: &'static str) -> TestResult {
    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
fn run_outfiles(args: &[&str], out_file: &str, err_file: &str) -> TestResult {
    let expected_out = fs::read_to_string(out_file)?;
    let expected_err = fs::read_to_string(err_file)?;

    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout =
        String::from_utf8(output.clone().stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected_out);

    let stderr = String::from_utf8(output.stderr).expect("invalid UTF-8");
    assert_eq!(stderr, expected_err);

    Ok(())
}

// --------------------------------------------------
#[test]
fn yogi_berra_cap() -> TestResult {
    run_outfiles(
        &["--pattern", "Yogi Berra", FORTUNE_DIR],
        "tests/expected/berra_cap.out",
        "tests/expected/berra_cap.err",
    )
}

// --------------------------------------------------
#[test]
fn mark_twain_cap() -> TestResult {
    run_outfiles(
        &["-m", "Mark Twain", FORTUNE_DIR],
        "tests/expected/twain_cap.out",
        "tests/expected/twain_cap.err",
    )
}

// --------------------------------------------------
#[test]
fn yogi_berra_lower() -> TestResult {
    run_outfiles(
        &["--pattern", "yogi berra", FORTUNE_DIR],
        "tests/expected/berra_lower.out",
        "tests/expected/berra_lower.err",
    )
}

// --------------------------------------------------
#[test]
fn mark_twain_lower() -> TestResult {
    run_outfiles(
        &["-m", "will twain", FORTUNE_DIR],
        "tests/expected/twain_lower.out",
        "tests/expected/twain_lower.err",
    )
}

// --------------------------------------------------
#[test]
fn yogi_berra_lower_i() -> TestResult {
    run_outfiles(
        &["--insensitive", "--pattern", "yogi berra", FORTUNE_DIR],
        "tests/expected/berra_lower_i.out",
        "tests/expected/berra_lower_i.err",
    )
}

// --------------------------------------------------
#[test]
fn mark_twain_lower_i() -> TestResult {
    run_outfiles(
        &["-i", "-m", "mark twain", FORTUNE_DIR],
        "tests/expected/twain_lower_i.out",
        "tests/expected/twain_lower_i.err",
    )
}