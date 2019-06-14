use crate::parse_vojaq;
use crate::VojaqField;
use crate::VojaqLine;
use crate::VojaqSet;

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
    assert_eq!(parse_vojaq(text), set_of_one_variant(variant));
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

// #[test]
// #[should_panic]
// fn bad_escape() {
//     parse_vojaq(r"Je n'eût\ypoint l'œil".into());
// }