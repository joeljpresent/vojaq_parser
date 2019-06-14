#[derive(Debug, PartialEq, Eq)]
pub struct VojaqField {
    variants: Vec<String>
}

impl VojaqField {
    pub fn new() -> VojaqField {
        VojaqField {
            variants: vec![String::new()]
        }
    }

    pub fn variants(&self) -> &Vec<String> {
        &self.variants
    }

    pub fn push(&mut self, variant: String) {
        self.variants.push(variant);
    }

    pub fn get(&self, field_number: usize) -> Option<&String> {
        self.variants.get(field_number)
    }
}