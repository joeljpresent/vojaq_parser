use crate::VojaqLine;
use std::fmt;

/// A Vojaq set is an "array" of Vojaq lines.
/// 
/// The text representation of a Vojaq set is a series of lines
/// seperated by a linefeed character.
/// 
/// ``` vojaq
/// The {first} line
/// The {second} line
/// ```
#[derive(PartialEq, Eq)]
pub struct VojaqSet {
    lines : Vec<VojaqLine>
}

impl VojaqSet {
    pub fn new() -> VojaqSet {
        VojaqSet {
            lines: vec![]
        }
    }

    pub fn with_lines(lines: Vec<VojaqLine>) -> VojaqSet {
        VojaqSet {
            lines: lines
        }
    }

    pub fn lines(&self) -> &[VojaqLine] {
        self.lines.as_slice()
    }

    pub fn push(&mut self, line: VojaqLine) {
        self.lines.push(line);
    }

    pub fn push_if_not_empty(&mut self, line: VojaqLine) {
        if !line.is_empty() {
            self.lines.push(line)
        }
    }

    pub fn get(&self, line_index: usize) -> Option<&VojaqLine> {
        self.lines.get(line_index)
    }

    pub fn is_empty(&self) -> bool {
        self.lines().iter().all(|line| line.is_empty())
    }
}

impl fmt::Debug for VojaqSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.lines.fmt(f)
    }
}