/// A Vojaq field is an "array" of variants.
/// 
/// A variant is basically a string which does not contain
/// delimiters (`{`, `}`, `|`, linefeed…).
/// 
/// The text representation of a Vojaq field is a series
/// of variants (strings) separated by a pipeline character.
/// 
/// ``` vojaq
/// first variant | second variant | third variant 
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct VojaqField {
    variants: Vec<String>
}

impl VojaqField {
    pub fn new() -> VojaqField {
        VojaqField {
            variants: vec![]
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