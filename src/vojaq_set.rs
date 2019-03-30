use crate::vojaq_trio::VojaqTrio;

#[derive(Debug)]
struct VojaqSet {
    trios : Vec<VojaqTrio>
}

impl VojaqSet {
    pub fn new() -> Self {
        VojaqSet {
            trios: vec![]
        }
    }

    pub fn trios_ref(&self) -> &[VojaqTrio] {
        self.trios.as_slice()
    }

    pub fn push_trio(&mut self, trio: VojaqTrio) {
        self.trios.push(trio);
    }
}