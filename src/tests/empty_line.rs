use crate::vojaq_line::VojaqLine;
use super::assert_line;

#[test]
fn empty() {
    assert_line("", VojaqLine::Empty);
}

#[test]
fn space() {
    assert_line(" ", VojaqLine::Empty);
}

#[test]
fn lots_of_whitespace() {
    assert_line(" \u{A0} \u{2001}\t\t\u{3000}\u{2003} \t \u{85}", VojaqLine::Empty);
}