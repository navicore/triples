#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Subject(String, String),
    PredObj(String, String, String),
    PredObjTerm(String, String, String),
}
