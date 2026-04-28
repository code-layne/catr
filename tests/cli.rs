use assert_cmd::Command;
use predicates::prelude::*;
use rand::{RngExt, distr::Alphanumeric};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

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
fn runs_fox() -> TestResult {
    run(&[FOX], FOX)
}
#[test]
fn runs_spiders() -> TestResult {
    run(&[SPIDERS], SPIDERS)
}
#[test]
fn runs_bustle() -> TestResult {
    run(&[BUSTLE], BUSTLE)
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
