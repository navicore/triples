#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, PartialEq, Clone)]
pub enum ParsedLine<'input> {
    Subject(&'input str),
    PredObj(&'input str, &'input str, Terminator),
    PredObjTerm(&'input str, &'input str, Terminator),
}

#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Terminator {
    SemiColon,
    SemiColonDot,
}
