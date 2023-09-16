#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Prefix(String, String),
    Subject(Option<String>, String),
    PredObj(Option<String>, String, String),
    PredObjTerm(Option<String>, String, String),
}
