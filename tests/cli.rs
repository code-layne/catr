use assert_cmd::Command;
use rand::{RngExt, distr::Alphanumeric};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const EMPTY_B: &str = "tests/expected/empty.b.txt";
const EMPTY_N: &str = "tests/expected/empty.n.txt";
const FOX: &str = "tests/inputs/fox.txt";
const FOX_B: &str = "tests/expected/fox.b.txt";
const FOX_N: &str = "tests/expected/fox.n.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const SPIDERS_B: &str = "tests/expected/spiders.b.txt";
const SPIDERS_N: &str = "tests/expected/spiders.n.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";
const BUSTLE_B: &str = "tests/expected/the-bustle.b.txt";
const BUSTLE_N: &str = "tests/expected/the-bustle.n.txt";

#[test]
fn reads_stdin() -> TestResult {
    let input = "hello\nworld\n";
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .assert()
        .success()
        .stdout(input);
    Ok(())
}

#[test]
fn runs_empty() -> TestResult {
    run(&[EMPTY], EMPTY)
}
#[test]
fn runs_empty_b() -> TestResult {
    run(&["-b", EMPTY], EMPTY_B)
}
#[test]
fn runs_empty_n() -> TestResult {
    run(&["-n", EMPTY], EMPTY_N)
}

#[test]
fn runs_fox() -> TestResult {
    run(&[FOX], FOX)
}
#[test]
fn runs_fox_b() -> TestResult {
    run(&["-b", FOX], FOX_B)
}
#[test]
fn runs_fox_n() -> TestResult {
    run(&["-n", FOX], FOX_N)
}
#[test]
fn runs_spiders() -> TestResult {
    run(&[SPIDERS], SPIDERS)
}
#[test]
fn runs_spiders_b() -> TestResult {
    run(&["-b", SPIDERS], SPIDERS_B)
}
#[test]
fn runs_spiders_n() -> TestResult {
    run(&["-n", SPIDERS], SPIDERS_N)
}
#[test]
fn runs_bustle() -> TestResult {
    run(&[BUSTLE], BUSTLE)
}
#[test]
fn runs_bustle_b() -> TestResult {
    run(&["-b", BUSTLE], BUSTLE_B)
}
#[test]
fn runs_bustle_n() -> TestResult {
    run(&["-n", BUSTLE], BUSTLE_N)
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn dies_bad_file() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&[gen_bad_file()])
        .assert()
        .failure()
        .stderr("No such file or directory (os error 2)\n");
    Ok(())
}
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
