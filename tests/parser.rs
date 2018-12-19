use libvplan::error::ParsingError;
use libvplan::{parser, Vplan};
use std::fs::File;
use std::io::Read;

#[test]
fn parse_correct_1() {
    assert!(parse("correct_1.xml").is_ok());
}

#[test]
fn parse_correct_2() {
    // FIXME: does not work yet
    // assert!(parse("correct_2.xml").is_ok());
}

#[test]
fn parse_correct_3() {
    // FIXME: does not work yet
    // assert!(parse("correct_3.xml").is_ok());
}

#[test]
fn parse_incorrect_1() {
    assert!(parse("incorrect_1.xml").is_err());
}

#[test]
fn parse_incorrect_2() {
    assert!(parse("incorrect_2.xml").is_err());
}

#[test]
fn parse_incorrect_3() {
    assert!(parse("incorrect_3.xml").is_err());
}

fn parse(file: &str) -> Result<Vplan, ParsingError> {
    let mut file = File::open(format!("tests/data/{}", file)).expect("Could not open file!");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read file!");

    parser::parse(&input)
}
