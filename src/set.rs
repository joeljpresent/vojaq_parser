use crate::trio::VojaqTrio;

#[derive(Debug)]
pub struct VojaqSet {
    trios : Vec<VojaqTrio>
}

impl VojaqSet {
    pub fn new() -> Self {
        VojaqSet {
            trios: vec![]
        }
    }

    pub fn with_trios(trios: Vec<VojaqTrio>) -> Self {
        VojaqSet {
            trios: trios
        }
    }

    pub fn trios_ref(&self) -> &[VojaqTrio] {
        self.trios.as_slice()
    }

    pub fn push_trio(&mut self, trio: VojaqTrio) {
        self.trios.push(trio);
    }
}