use crate::parse_vojaq;
use crate::VojaqField;
use crate::VojaqLine;
use crate::VojaqSet;

#[test]
fn single_world() {
    let text = "sanic";
    
    let mut field = VojaqField::new();
    field.push("sanic".into());
    let mut line = VojaqLine::new();
    line.push(field);
    let mut set = VojaqSet::new();
    set.push(line);

    assert_eq!(parse_vojaq(text), set)
}