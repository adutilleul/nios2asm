use assert_cmd::prelude::*;
use std::io::{Read, Write};
use std::process::Command;
use tempfile::NamedTempFile;

mod fixtures;

const BIN_NAME: &str = "nios2asm";

#[test]
fn test_main_case_1() {
    use fixtures::{INPUT_CASE_1, OUTPUT_CASE_1};

    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(INPUT_CASE_1.as_bytes()).unwrap();

    let mut output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(&[input_file.path(), output_file.path()])
        .assert()
        .success();

    let mut actual = String::new();
    output_file.read_to_string(&mut actual).unwrap();
    assert_eq!(actual, OUTPUT_CASE_1);
}