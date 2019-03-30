use crate::vojaq_line::VojaqLine;
use crate::vojaq_trio::VojaqTrio;

#[cfg(test)]
mod empty_line;

#[cfg(test)]
mod vojaq_trios;

#[cfg(test)]
mod comments;

fn line_trio(primo: &str, secondo: &str, terzo: &str) -> VojaqLine {
    VojaqLine::Trio(VojaqTrio::new(primo.to_owned(), secondo.to_owned(), terzo.to_owned()))
}

fn assert_line(line: &str, expected: VojaqLine) {
    let got: VojaqLine = VojaqLine::from_str(line);
    assert!(
        got == expected,
        "For line “{}”: expected {:?}, got {:?}.",
        line, expected, got
    );
}