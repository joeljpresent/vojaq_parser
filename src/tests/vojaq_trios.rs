use super::{assert_line,line_trio};

#[test]
fn simple_as_abc() {
    assert_line("a{b}c", line_trio("a","b","c"));
}

#[test]
fn easy_as_123() {
    assert_line("one{two}three", line_trio("one","two","three"));
}

#[test]
fn i_pooped_in_my_schoolbag() {
    assert_line("J'ai fait {caca} dans mon cartable !", 
        line_trio("J'ai fait","caca","dans mon cartable !"));
}

#[test]
fn ambiguous_voice() {
    assert_line("Voix ambiguë d'un cœur {qui au zéphyr} préfère les jattes de kiwi.", 
        line_trio("Voix ambiguë d'un cœur","qui au zéphyr","préfère les jattes de kiwi."));
}

#[test]
fn sushi() {
    assert_line("寿司 {sushi} Poisson cru cuisiné.", 
        line_trio("寿司","sushi","Poisson cru cuisiné."));
}

#[test]
fn yeah_sure() {
    assert_line("🙄 {U+1F644} Mais bien sûr !", 
        line_trio("🙄","U+1F644","Mais bien sûr !"));
}

#[test]
fn james_d_hyneman() {
    assert_line(r#"James "Dangerous" Hyneman {Jamie Hyneman} Ancien MythBuster."#, 
        line_trio("James \"Dangerous\" Hyneman","Jamie Hyneman","Ancien MythBuster."));
}

#[test]
fn croisillon() {
    assert_line("# {croisillon} À ne pas confondre avec le dièse.", 
        line_trio("#", "croisillon", "À ne pas confondre avec le dièse."));
}

#[test]
fn windows_path() {
    assert_line(r#"C:\\User\\Documents#file {\{Windows\} path} Path to \{Documents\}"#,
        line_trio(r"C:\User\Documents#file", r#"{Windows} path"#, r#"Path to {Documents}"#));
}

#[test]
fn escape_fiesta() {
    assert_line(r#"\\forall x \\in \{1,2,3\} {accolade : "\}"} \{pas fini"#,
        line_trio(r"\forall x \in {1,2,3}", r#"accolade : "}""#, r"{pas fini"));
}

#[test]
fn with_bom() {
    assert_line("\u{feff}BOM {Byte Order Mark} Marqueur d'ordre d'octets",
        line_trio("BOM", "Byte Order Mark", "Marqueur d'ordre d'octets"))
}

#[test]
fn with_comment() {
    assert_line(r"いただきます {itadakimasu} Bon appétit \# Ce n'est pas un équivalent exact.",
        line_trio("いただきます", "itadakimasu", "Bon appétit"))
}

#[test]
fn double_backslash_pound() {
    assert_line(r"Double \\# {backslash} pound (comment)",
        line_trio(r"Double \#", "backslash", "pound (comment)"))
}