use std::fs::read_to_string;
use std::path::Path;
use crate::parsing::parser::parse_vojaq;
use crate::structs::set::VojaqSet;

/// Extract the content of a Vojaq file into a VojaqSet.
pub fn read_file<P>(path: P) -> Result<VojaqSet, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let content = read_to_string(path)?;

    // Remove the UTF-8 BOM if any.
    let content = if content.starts_with("\u{FEFF}") {
        &content[3..]
    } else {
        &content[..]
    };

    Ok(parse_vojaq(content))
}