#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Subject(String),
    PredObj(String, String),
    PredObjTerm(String, String),
}
