#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Subject(Option<String>, String),
    PredObj(Option<String>, String, String),
    PredObjTerm(Option<String>, String, String),
}
