// A simple struct representation for a SPARQL query
#[derive(Debug, PartialEq)]
pub struct SparqlQuery {
    pub select_clause: SelectClause,
    pub triples_block: Vec<TriplePattern>,
}

#[derive(Debug, PartialEq)]
pub struct SelectClause {
    pub distinct: bool,
    pub variables: Vec<Variable>,
}

#[derive(Debug, PartialEq)]
pub enum Variable {
    IRI(String),
    Var(String),
}

#[derive(Debug, PartialEq)]
pub struct TriplePattern {
    pub subject: Variable,
    pub predicate: String,
    pub object: Variable,
}
