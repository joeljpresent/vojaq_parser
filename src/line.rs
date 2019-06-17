use std::fmt;
use crate::VojaqField;

/// A Vojaq line is an "array" of Vojaq fields.
/// 
/// The text representation of a Vojaq line is a series of Vojaq
/// fields.
/// One field in two must be enclosed between curly braces.
/// 
/// ``` vojaq
/// the first | field {the second field} the | third | field
/// ```
#[derive(PartialEq, Eq)]
pub struct VojaqLine {
    fields: Vec<VojaqField>
}

impl VojaqLine {
    pub fn new() -> VojaqLine {
        VojaqLine {
            fields: vec![]
        }
    }

    pub fn fields(&self) -> &Vec<VojaqField> {
        &self.fields
    }

    pub fn push(&mut self, field: VojaqField) {
        self.fields.push(field);
    }

    pub fn push_if_not_empty(&mut self, field: VojaqField) {
        if !field.is_empty() {
            self.fields.push(field);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fields().iter().all(|field| field.is_empty())
    }

    pub fn get(&self, field_index: usize) -> Option<&VojaqField> {
        self.fields.get(field_index)
    }
}

impl fmt::Debug for VojaqLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fields.fmt(f)
    }
}
