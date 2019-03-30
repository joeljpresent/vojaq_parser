use crate::vojaq_trio::VojaqTrio;

#[derive(Debug, PartialEq, Eq)]
pub enum VojaqLine {
    Trio(VojaqTrio),
    Comment,
    Empty,
    Invalid(String)
}

impl VojaqLine {
    pub fn from_str(line: &str) -> VojaqLine {
        crate::parser::parse_line(line)
    }
}