use std::fmt;

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
#[derive(PartialEq, Eq)]
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

    pub fn push_trimmed(&mut self, variant: String) {
        let v = variant.trim();
        if v != "" {
            self.variants.push(variant.trim().into());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.variants().iter().all(|variant| variant == "")
    }

    pub fn get(&self, field_number: usize) -> Option<&String> {
        self.variants.get(field_number)
    }
}

impl fmt::Debug for VojaqField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.variants.fmt(f)
    }
}