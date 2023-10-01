// A simple struct representation for a SPARQL query
#[derive(Debug, Eq, PartialEq)]
pub struct SparqlQuery {
    pub select_clause: SelectClause,
    pub triples_block: Vec<TriplePattern>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SelectClause {
    pub distinct: bool,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Variable {
    IRI(String),
    Var(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct TriplePattern {
    pub subject: Variable,
    pub predicate: String,
    pub object: Variable,
}
