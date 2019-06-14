use crate::parse_vojaq;
use crate::VojaqSet;

#[test]
fn empty_string() {
    let text = "sanic | Sonic {leuh} airissont | sont sont | les petits cons";
    assert_eq!(parse_vojaq(text), VojaqSet::new())
}