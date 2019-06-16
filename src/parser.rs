use crate::{ParsingError, ParsingResult, VojaqField, VojaqLine, VojaqSet};
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
pub fn parse_vojaq(text: &str) -> ParsingResult<VojaqSet> {
    VojaqParser::new(text).parse_text()
}

impl<'a> VojaqParser<'a> {
    pub fn new(text: &'a str) -> VojaqParser<'a> {
        VojaqParser {
            it : text.chars().peekable()
        }
    }

    pub fn parse_text(&mut self) -> ParsingResult<VojaqSet> {
        let mut set = VojaqSet::new();
        loop {
            match self.parse_line() {
                Ok(LineParsingState::NewLine(line)) => {
                    set.push(line);
                },
                Ok(LineParsingState::Done(line)) => {
                    set.push(line);
                    return Ok(set);
                },
                Err(e) => return Err(e)
            }
        }
    }

    fn parse_line(&mut self) -> ParsingResult<LineParsingState> {
        let mut line = VojaqLine::new();
            loop {
                match self.parse_field() {
                    Ok(FieldParsingState::NewField(field)) => {
                        line.push(field);
                    },
                    Ok(FieldParsingState::StartNewLine(field)) => {
                        line.push(field);
                        return Ok(LineParsingState::NewLine(line));
                    },
                    Ok(FieldParsingState::Done(field)) => {
                        line.push(field);
                        return Ok(LineParsingState::Done(line));
                    },
                    Err(e) => return Err(e)
                }
            }
    }

    fn parse_field(&mut self) -> ParsingResult<FieldParsingState> {
        let mut field = VojaqField::new();
        loop {
            match self.parse_variant() {
                Ok(VariantParsingState::NewVariant(variant)) => {
                    field.push(variant);
                },
                Ok(VariantParsingState::StartNewField(variant)) => {
                    field.push(variant);
                    return Ok(FieldParsingState::NewField(field));
                },
                Ok(VariantParsingState::StartNewLine(variant)) => {
                    field.push(variant);
                    return Ok(FieldParsingState::StartNewLine(field));
                },
                Ok(VariantParsingState::Done(variant)) => {
                    field.push(variant);
                    return Ok(FieldParsingState::Done(field));
                },
                Err(e) => return Err(e)
            }
        }
    }

    fn parse_variant(&mut self) -> ParsingResult<VariantParsingState> {
        let mut variant = String::new();
        while let Some(c) = self.it.next() {
            match c {
                '|' => return Ok(VariantParsingState::NewVariant(variant)),
                '{' | '}' => return Ok(VariantParsingState::StartNewField(variant)),
                '\n' => return Ok(VariantParsingState::StartNewLine(variant)),
                '\\' => self.push_escaped(&mut variant)?,
                c => variant.push(c)
            };
        }
        // if end-of-line is reached
        Ok(VariantParsingState::Done(variant))
    }

    fn push_escaped(&mut self, variant: &mut String) -> ParsingResult<()> {
        if let Some(c) = self.it.next() {
            match c {
                '|' | '{' | '}' | '\\' => variant.push(c),
                't' => variant.push('\t'),
                'n' => variant.push('\n'),
                'r' => variant.push('\r'),
                '#' => self.parse_comment(),
                c => return Err(ParsingError::BadEscapedSequence(c))
            }
        };
        Ok(())
    }

    fn parse_comment(&mut self) {
        loop {
            if let Some(ch) = self.it.peek() {
                if *ch == '\n' {
                    return;
                }
                self.it.next();
            } else {
                return;
            }
        }
    }

}

