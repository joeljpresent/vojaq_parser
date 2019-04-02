/// Ensemble de trois phrases.
#[derive(Debug, PartialEq, Eq)]
pub struct VojaqTrio {
    primo: String,
    secondo: String,
    terzo: String
}

impl VojaqTrio {
    pub fn new(primo: String, secondo: String, terzo: String) -> VojaqTrio {
        VojaqTrio {
            primo : primo,
            secondo : secondo,
            terzo : terzo
        }
    }

    pub fn primo_ref(&self) -> &String {
        &self.primo
    }

    pub fn secondo_ref(&self) -> &String {
        &self.secondo
    }

    pub fn terzo_ref(&self) -> &String {
        &self.terzo
    }
}