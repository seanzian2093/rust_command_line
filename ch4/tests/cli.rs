// cargo test -p ch4-headr --test cli dies_bad_bytes -- --exact --no-capture
#![allow(unused)]
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distr::Alphanumeric, Rng};
use std::fs::{self, File};
use std::io::prelude::*;

const PRG: &str = "ch4-headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TWELVE: &str = "./tests/inputs/twelve.txt";

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
fn run(args: &[&str], expected_file: &str) -> TestResult {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    args: &[&str],
    input_file: &str,
    expected_file: &str,
) -> TestResult {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .output()
        .expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> TestResult {
    let bad = random_string();
    let expected = format!(
        "illegal byte count -- {bad}"
    );

    Command::cargo_bin(PRG)?
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> TestResult {
    let bad = random_string();
    let expected = format!("illegal line count -- {bad}");
    Command::cargo_bin(PRG)?
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> TestResult {
    let msg = "the argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    Command::cargo_bin(PRG)?
        .args(["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args([EMPTY, &bad, ONE])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n2() -> TestResult {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

// --------------------------------------------------
#[test]
fn empty_n4() -> TestResult {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

// --------------------------------------------------
#[test]
fn empty_c2() -> TestResult {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

// --------------------------------------------------
#[test]
fn empty_c4() -> TestResult {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() -> TestResult {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> TestResult {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> TestResult {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn one_stdin() -> TestResult {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1_stdin() -> TestResult {
    run_stdin(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> TestResult {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> TestResult {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> TestResult {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> TestResult {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn two_stdin() -> TestResult {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> TestResult {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> TestResult {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> TestResult {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> TestResult {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn three_stdin() -> TestResult {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn twelve() -> TestResult {
    run(&[TWELVE], "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n2() -> TestResult {
    run(&[TWELVE, "-n", "2"], "tests/expected/twelve.txt.n2.out")
}

#[test]
fn twelve_n4() -> TestResult {
    run(&[TWELVE, "-n", "4"], "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_c2() -> TestResult {
    run(&[TWELVE, "-c", "2"], "tests/expected/twelve.txt.c2.out")
}

#[test]
fn twelve_c4() -> TestResult {
    run(&[TWELVE, "-c", "4"], "tests/expected/twelve.txt.c4.out")
}

#[test]
fn twelve_stdin() -> TestResult {
    run_stdin(&[], TWELVE, "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TWELVE, "tests/expected/twelve.txt.n2.out")
}

#[test]
fn twelve_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TWELVE, "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], TWELVE, "tests/expected/twelve.txt.c2.out")
}

#[test]
fn twelve_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], TWELVE, "tests/expected/twelve.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TWELVE], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() -> TestResult {
    run(
        &["-n", "4", EMPTY, ONE, TWO, THREE, TWELVE],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() -> TestResult {
    run(
        &["-c", "4", EMPTY, ONE, TWO, THREE, TWELVE],
        "tests/expected/all.c4.out",
    )
}