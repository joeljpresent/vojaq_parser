use crate::VojaqField;

#[derive(Debug, PartialEq, Eq)]
pub struct VojaqLine {
    fields: Vec<VojaqField>
}

impl VojaqLine {
    pub fn new() -> VojaqLine {
        VojaqLine {
            fields: vec![VojaqField::new()]
        }
    }

    pub fn fields(&self) -> &Vec<VojaqField> {
        &self.fields
    }

    pub fn add_field(&mut self) {
        self.fields.push(VojaqField::new());
    }

    pub fn get(&self, field_index: usize) -> Option<&VojaqField> {
        self.fields.get(field_index)
    }

    pub fn current_field_number(&self) -> usize {
        self.fields.len() - 1
    }
}