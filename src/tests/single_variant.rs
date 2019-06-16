use crate::{parse_vojaq, ParsingError, VojaqField, VojaqLine, VojaqSet};

fn set_of_one_variant(variant: &str) -> VojaqSet {
    let mut field = VojaqField::new();
    field.push(variant.into());
    let mut line = VojaqLine::new();
    line.push(field);
    let mut set = VojaqSet::new();
    set.push(line);
    set
}

fn assert_good_result(text: &str, variant: &str) {
    let resulting_set: VojaqSet = parse_vojaq(text).unwrap();
    assert_eq!(resulting_set, set_of_one_variant(variant));
}

#[test]
fn single_world() {
    assert_good_result("sanic", "sanic");
}

#[test]
fn full_phrase() {
    assert_good_result("Voix ambiguë d'un cœur…", "Voix ambiguë d'un cœur…");
}

#[test]
fn newline_alone() {
    assert_good_result(r"\n", "\n");
}

#[test]
fn newline_in_phrase() {
    assert_good_result(r"Je n'eût\npoint l'œil", "Je n'eût\npoint l'œil");
}

#[test]
fn escaped_n() {
    assert_good_result(r"Je n'eût\\npoint l'œil", r"Je n'eût\npoint l'œil");
}

#[test]
fn escaped_braces() {
    assert_good_result(r"x ∈ \{3, 7\}", "x ∈ {3, 7}");
}

#[test]
fn escaped_backslash() {
    assert_good_result(r"C:\\Programs\\torix", r"C:\Programs\torix");
}

#[test]
fn escaped_backslash_and_braces() {
    assert_good_result(r"x ∈ \\\{3, 7\\\}", r"x ∈ \{3, 7\}");
}

#[test]
fn bad_escape() {
    let result = parse_vojaq(r"Je n'eût\ypoint l'œil".into());
    assert_eq!(result, Err(ParsingError::BadEscapedSequence('y')));
}

#[test]
fn small_comment() {
    assert_good_result(r"Joël # 1\# number one", "Joël # 1");
}

#[test]
fn tricky_comment() {
    assert_good_result(r"Moi\\#toi\#lui\#nous{vous}eux", r"Moi\#toi");
}
