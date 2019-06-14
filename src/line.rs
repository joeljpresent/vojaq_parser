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
#[derive(Debug, PartialEq, Eq)]
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

    pub fn get(&self, field_index: usize) -> Option<&VojaqField> {
        self.fields.get(field_index)
    }
}