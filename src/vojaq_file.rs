use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::parser::parse_line;
use crate::vojaq_trio::VojaqTrio;
use crate::vojaq_set::VojaqSet;

/// Extract the content of a Vojaq file into a VojaqSet.
pub fn read_file<P>(filename: P) -> std::io::Result<VojaqSet>
    where P: AsRef<Path>
{
    let file = File::open(filename)?;
    let trios: Vec<VojaqTrio> = BufReader::new(file).lines()
        .filter_map(|l| l.ok())
        .map(|l| parse_line(l.as_str()).unpack_trio())
        .filter_map(|l| l)
        .collect();
    Ok(VojaqSet::with_trios(trios))
}