use crate::{VojaqField, VojaqLine, VojaqSet};
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Eq)]
enum ParsingState {
    StartNewLine(String),
    StartNewField(String),
    NewVariant(String),
    Done(String)
}

struct VojaqParser<'a> {
    it : Peekable<Chars<'a>>,
    set: VojaqSet
}

pub fn parse_vojaq(text: &str) -> VojaqSet {
    let mut parser = VojaqParser::new(text);
    parser.parse_text();
    parser.vojaq_set()
}

impl<'a> VojaqParser<'a> {
    pub fn new(text: &'a str) -> VojaqParser<'a> {
        VojaqParser {
            it : text.chars().peekable(),
            set: VojaqSet::new()
        }
    }

    pub fn parse_text(&mut self) {

    }

    fn parse_line(&mut self) {

    }

    fn parse_field(&mut self) {
        let mut field = VojaqField::new();
        loop {
            match self.parse_variant() {
                ParsingState::NewVariant(variant) => {
                    field.push(variant)
                    // TODO
                },
                _ => println!("Tiri")
            }
        }
    }

    fn parse_variant(&mut self) -> ParsingState {
        let mut variant = String::new();
        while let Some(c) = self.it.next() {
            match c {
                '|' => return ParsingState::NewVariant(variant),
                '{' | '}' => return ParsingState::StartNewField(variant),
                '\n' => return ParsingState::StartNewLine(variant),
                c => variant.push(c)
            }
        }
        // if end-of-line is reached
        ParsingState::Done(variant)
    }

    pub fn vojaq_set(self) -> VojaqSet {
        self.set
    }
}

