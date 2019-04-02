use crate::line::VojaqLine;
use super::assert_line;

#[test]
fn empty_comment() {
    assert_line(r"\#", VojaqLine::Empty);
}

#[test]
fn this_is_a_comment() {
    assert_line(r"\#Ceci est un commentaire", VojaqLine::Empty);
}

#[test]
fn this_is_not_a_trio() {
    assert_line(r"\# Ceci {n'est pas} un trio.", VojaqLine::Empty);
}