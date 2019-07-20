use vojaq_parser::{parse_vojaq, VojaqField, VojaqLine, VojaqSet};

fn set_of_one_field(variants: &[&str]) -> VojaqSet {
    let mut field = VojaqField::new();
    for variant in variants.iter().map(|s| (*s).to_owned()) {
        field.push(variant);
    }
    let mut line = VojaqLine::new();
    line.push(field);
    let mut set = VojaqSet::new();
    set.push(line);
    set
}

fn assert_good_result(text: &str, variants: &[&str]) {
    let resulting_set: VojaqSet = parse_vojaq(text).unwrap();
    assert_eq!(resulting_set, set_of_one_field(variants));
}

#[test]
fn two_variants() {
    assert_good_result("jolly|jumper", 
        &["jolly", "jumper"]);
}

#[test]
fn three_variants() {
    assert_good_result("po|pi|po", 
        &["po", "pi", "po"]);
}

#[test]
fn ten_variants() {
    assert_good_result("zéro|UN|2|trois|QUATRE|c!nq|six|SEPT|hUiT|n9uf|DIX", 
        &["zéro", "UN", "2", "trois", "QUATRE", "c!nq", "six", "SEPT", "hUiT", "n9uf", "DIX"]);
}

#[test]
fn variants_with_space() {
    assert_good_result("bon sang | de | bois !!", 
        &["bon sang", "de", "bois !!"]);
}

#[test]
fn variants_with_tricky_whitespace() {
    assert_good_result(r#"\tbon  \nsang \n |\r de   |\t bois !!  \r\n\t "#,
        &["bon  \nsang", "de", "bois !!"]);
}

#[test]
fn escaped_pipelines() {
    assert_good_result(r#"ls \| grep txt | make | \|\\/\|"#, 
        &["ls | grep txt", "make", r#"|\/|"#]);
}

