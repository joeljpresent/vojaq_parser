use crate::{read_file, VojaqSet};

#[test]
fn read_from_file() {
    let s = read_file("vojaq_files/new.vojaq");
    let s = s.unwrap();
    let z = VojaqSet::new();
    assert_eq!(s, z);
}