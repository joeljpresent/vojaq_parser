/**
 * Ensemble de trois phrases.
 */
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

    pub fn get_primo(&self) -> &String {
        &self.primo
    }

    pub fn get_secondo(&self) -> &String {
        &self.secondo
    }

    pub fn get_terzo(&self) -> &String {
        &self.terzo
    }
}