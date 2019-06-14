use crate::VojaqLine;

#[derive(Debug)]
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

    //pub fn parse(content: &str) -> Result<VojaqSet, Box<dyn std::error::Error>> {
    //
    //}

    pub fn lines(&self) -> &[VojaqLine] {
        self.lines.as_slice()
    }

    pub fn push(&mut self, line: VojaqLine) {
        self.lines.push(line);
    }

    pub fn get(&self, line_index: usize) -> Option<&VojaqLine> {
        self.lines.get(line_index)
    }
}