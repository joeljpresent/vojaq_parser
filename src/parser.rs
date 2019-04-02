use crate::line::VojaqLine;
use crate::trio::VojaqTrio;
use std::str::Chars;

#[derive(Eq, PartialEq)]
enum ParsingStatus {
    Primo,
    Secondo,
    Terzo
}

pub fn parse_line(line: &str) -> VojaqLine {
    let l = line.trim();
    if l.is_empty() {
        VojaqLine::Empty
    } else if l.starts_with(r"\#") {
        VojaqLine::Comment
    } else {
        match parse_vojaq_trio(line) {
            Ok(trio) => VojaqLine::Trio(trio),
            Err(error) => VojaqLine::Invalid(error)
        }
    }
}

fn parse_vojaq_trio(line: &str) -> Result<VojaqTrio, String> {
    let mut it = line.chars();
    Ok(VojaqTrio::new(
        parse_testo(&mut it, ParsingStatus::Primo)?,
        parse_testo(&mut it, ParsingStatus::Secondo)?,
        parse_testo(&mut it, ParsingStatus::Terzo)?
    ))
}

fn parse_testo(it: &mut Chars, state: ParsingStatus) -> Result<String, String> {
    let mut testo = String::new();
    while let Some(c) = it.next() {
        match c {
            '\\' => {
                if let Some(ch) = it.next() {
                    match ch {
                        '{' | '}' | '\\' => testo.push(ch),
                        _ => return Err("Invalid escape sequence".to_owned())
                    }
                } else {
                    return Err("Backslash not followed by a character".to_owned())
                }
            },
            '{' => {
                if state == ParsingStatus::Primo {
                    return Ok(testo.trim().to_owned());
                } else {
                    return Err("Use of '{' outside of primo".to_owned());
                }
            },
            '}' => {
                if state == ParsingStatus::Secondo {
                    return Ok(testo.trim().to_owned());
                } else {
                    return Err("Use of '}' outside of secondo".to_owned());
                }
            }
            _ => testo.push(c)
        }
    }
    match state {
        ParsingStatus::Terzo => Ok(testo.trim().to_owned()),
        _ => Err("Reach end-of-line outside of terzo".to_owned())
    }
}
