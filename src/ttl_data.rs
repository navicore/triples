#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Subject(String),
    PredObj(String, String, Terminator),
    PredObjTerm(String, String, Terminator),
}

#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Terminator {
    SemiColon,
    SemiColonDot,
}
