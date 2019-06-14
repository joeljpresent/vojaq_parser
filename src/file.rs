use std::fs::read_to_string;
use std::path::Path;
use crate::parse_vojaq;
use crate::VojaqSet;

/// Extract the content of a Vojaq file into a VojaqSet.
/// 
/// The Vojaq file must represent a valid Vojaq set and 
/// be encoded in UTF-8.
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