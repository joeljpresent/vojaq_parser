use vojaq_parser::{parse_vojaq, VojaqField, VojaqLine, VojaqSet};

fn set_of_one_line(fields: Vec<&[&str]>) -> VojaqSet {
    let mut line = VojaqLine::new();
    for field_variants in fields.iter() {
        let mut field = VojaqField::new();
        for variant in field_variants.iter().map(|s| (*s).to_owned()) {
            field.push(variant);
        }
        line.push(field)
    }
    let mut set = VojaqSet::new();
    set.push(line);
    set
}

fn assert_good_result(text: &str, fields: Vec<&[&str]>) {
    let resulting_set: VojaqSet = parse_vojaq(text).unwrap();
    assert_eq!(resulting_set, set_of_one_line(fields));
}

#[test]
fn simple_line() {
    assert_good_result("Je {t'aime} pas", vec![&["Je"], &["t'aime"], &["pas"]])
}

#[test]
fn two_fields() {
    assert_good_result("Je | 私 | I {t'aime | <3}", vec![&["Je", "私", "I"], &["t'aime", "<3"]])
}

#[test]
fn three_fields() {
    assert_good_result("Je | 私 | I {t'aime | <3} pas", vec![&["Je", "私", "I"], &["t'aime", "<3"], &["pas"]])
}

#[test]
fn fields_with_comment() {
    assert_good_result(r"lol | MDR {:) | 8D} ha \#drôle", vec![&["lol", "MDR"], &[":)", "8D"], &["ha"]])
}

#[test]
fn one_empty_field() {
    assert_good_result("{t'aime | <3} pas", vec![&[], &["t'aime", "<3"], &["pas"]])
}
