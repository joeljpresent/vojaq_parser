use crate::{VojaqField, VojaqLine, VojaqSet};
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Eq, Debug)]
enum VariantParsingState {
    StartNewLine(String),
    StartNewField(String),
    NewVariant(String),
    Done(String)
}

#[derive(PartialEq, Eq, Debug)]
enum FieldParsingState {
    StartNewLine(VojaqField),
    NewField(VojaqField),
    Done(VojaqField)
}

#[derive(PartialEq, Eq, Debug)]
enum LineParsingState {
    NewLine(VojaqLine),
    Done(VojaqLine)
}

struct VojaqParser<'a> {
    it : Peekable<Chars<'a>>
}

/// Parse a Vojaq text into a Vojaq set.
pub fn parse_vojaq(text: &str) -> VojaqSet {
    VojaqParser::new(text).parse_text()
}

impl<'a> VojaqParser<'a> {
    pub fn new(text: &'a str) -> VojaqParser<'a> {
        VojaqParser {
            it : text.chars().peekable()
        }
    }

    pub fn parse_text(&mut self) -> VojaqSet {
        let mut set = VojaqSet::new();
        loop {
            match self.parse_line() {
                LineParsingState::NewLine(line) => {
                    set.push(line);
                },
                LineParsingState::Done(line) => {
                    set.push(line);
                    return set;
                }
            }
        }
    }

    fn parse_line(&mut self) -> LineParsingState {
        let mut line = VojaqLine::new();
            loop {
                match self.parse_field() {
                    FieldParsingState::NewField(field) => {
                        line.push(field);
                    },
                    FieldParsingState::StartNewLine(field) => {
                        line.push(field);
                        return LineParsingState::NewLine(line);
                    },
                    FieldParsingState::Done(field) => {
                        line.push(field);
                        return LineParsingState::Done(line);
                    }
                }
            }
    }

    fn parse_field(&mut self) -> FieldParsingState {
        let mut field = VojaqField::new();
        loop {
            match self.parse_variant() {
                VariantParsingState::NewVariant(variant) => {
                    field.push(variant);
                },
                VariantParsingState::StartNewField(variant) => {
                    field.push(variant);
                    return FieldParsingState::NewField(field);
                },
                VariantParsingState::StartNewLine(variant) => {
                    field.push(variant);
                    return FieldParsingState::StartNewLine(field);
                },
                VariantParsingState::Done(variant) => {
                    field.push(variant);
                    return FieldParsingState::Done(field);
                }
            }
        }
    }

    fn parse_variant(&mut self) -> VariantParsingState {
        let mut variant = String::new();
        while let Some(c) = self.it.next() {
            match c {
                '|' => return VariantParsingState::NewVariant(variant),
                '{' | '}' => return VariantParsingState::StartNewField(variant),
                '\n' => return VariantParsingState::StartNewLine(variant),
                '\\' => self.push_escaped(&mut variant),
                c => variant.push(c)
            }
        }
        // if end-of-line is reached
        VariantParsingState::Done(variant)
    }

    fn push_escaped(&mut self, variant: &mut String) {
        if let Some(c) = self.it.next() {
            match c {
                '|' | '{' | '}' | '\\' => variant.push(c),
                't' => variant.push('\t'),
                'n' => variant.push('\n'),
                'r' => variant.push('\r'),
                _ => variant.push_str("ERROR !! TODO !!")
            }
        }
    }
}

