use crate::parse_vojaq;
use crate::VojaqSet;

#[test]
fn empty_string() {
    let text = "sanic";
    assert_eq!(parse_vojaq(text), VojaqSet::new())
}