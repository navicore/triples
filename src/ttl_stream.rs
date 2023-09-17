use crate::data::RdfName;
use crate::data::Subject;
use crate::data::TriplesError;
use crate::ttl::LineParser;
use std::collections::HashMap;

#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Prefix(String, String),
    Subject(Option<String>, String),
    PredObj(Option<String>, String, String),
    PredObjTerm(Option<String>, String, String),
}

pub struct TtlStream {
    state: ParserState,
    parser: LineParser,
    prefixes: HashMap<String, String>,
    current_subject: Option<Subject>,
}

enum ParserState {
    SubjectLoading,
    PredicateLoading,
}

impl Default for TtlStream {
    fn default() -> Self {
        Self::new()
    }
}

impl TtlStream {
    #[must_use]
    pub fn new() -> Self {
        let parser = LineParser::new();
        Self {
            state: ParserState::SubjectLoading,
            parser,
            prefixes: HashMap::new(),
            current_subject: None,
        }
    }

    fn resolve_iri(
        &self,
        prefix: Option<&String>,
        local_name: &str,
    ) -> Result<String, TriplesError> {
        prefix.map_or_else(
            || Ok(local_name.to_string()),
            |ns| {
                self.prefixes.get(ns).map_or_else(
                    || {
                        Err(TriplesError::UnresolvableURIPrefix {
                            prefix_name: ns.to_string(),
                        })
                    },
                    |ns| {
                        let cleaned_ns = ns.trim_end_matches('/');
                        Ok(format!("{cleaned_ns}/{local_name}"))
                    },
                )
            },
        )
    }

    fn handle_subject(
        &mut self,
        prefix: &Option<String>,
        name: &str,
    ) -> Result<Option<Subject>, TriplesError> {
        if self.current_subject.is_some() {
            return Err(TriplesError::PreviousSubjectNotComplete);
        }
        let subject_iri_text = self.resolve_iri(prefix.as_ref(), name)?;
        let subject_iri =
            RdfName::new(subject_iri_text.clone()).map_err(|_| TriplesError::InvalidIRI {
                uri: subject_iri_text.to_string(),
            })?;
        self.current_subject = Some(Subject::new(subject_iri));
        self.state = ParserState::PredicateLoading;
        Ok(None)
    }

    fn handle_predicate(
        &mut self,
        prefix: &Option<String>,
        predicate: &str,
        object: &str,
    ) -> Result<Option<Subject>, TriplesError> {
        let predicate_iri_text = self.resolve_iri(prefix.as_ref(), predicate)?;
        let predicate_iri =
            RdfName::new(predicate_iri_text.clone()).map_err(|_| TriplesError::InvalidIRI {
                uri: predicate_iri_text.to_string(),
            })?;

        self.current_subject
            .as_mut()
            .map_or(Err(TriplesError::NoSubjectDeclaired), |subject| {
                subject.add(predicate_iri, object.to_string());
                Ok(None)
            })
    }

    fn handle_predicate_term(
        &mut self,
        prefix: &Option<String>,
        predicate: &str,
        object: &str,
    ) -> Result<Option<Subject>, TriplesError> {
        // Since the logic is same as handle_predicate for now, reuse it
        let result = self.handle_predicate(prefix, predicate, object)?;
        if self.current_subject.is_some() {
            let finished_subject = self.current_subject.clone();
            self.current_subject = None;
            self.state = ParserState::SubjectLoading;
            // If successfully added and current subject exists, clone and return
            match finished_subject {
                Some(s) => {
                    return Ok(Some(s));
                }
                _ => return Err(TriplesError::NoSubjectDeclaired),
            }
        }
        Ok(result)
    }

    /// enables a stream processor to load one line of ttl at a time
    ///
    /// # Errors
    ///
    /// Will return `Err` if line can not be processed
    pub fn load(&mut self, line: &str) -> Result<Option<Subject>, TriplesError> {
        let parsed = self
            .parser
            .parse(line)
            .map_err(|e| TriplesError::ParseError {
                reason: e.to_string(),
            })?;
        match self.state {
            ParserState::SubjectLoading => match parsed {
                ParsedLine::Prefix(name, uri) => {
                    self.prefixes.insert(name, uri);
                    Ok(None)
                }
                ParsedLine::Subject(prefix, name) => self.handle_subject(&prefix, &name),
                ParsedLine::PredObj(_, _, _) | ParsedLine::PredObjTerm(_, _, _) => {
                    Err(TriplesError::NotImplemented)
                }
            },
            ParserState::PredicateLoading => match parsed {
                // you can load prefixes later in the file but only in-between subject blocks
                ParsedLine::Prefix(name, uri) => {
                    if self.current_subject.is_some() {
                        return Err(TriplesError::PreviousSubjectNotComplete);
                    }
                    self.prefixes.insert(name, uri);
                    Ok(None)
                }
                ParsedLine::PredObj(prefix, predicate, object) => {
                    self.handle_predicate(&prefix, &predicate, &object)
                }
                ParsedLine::PredObjTerm(prefix, predicate, object) => {
                    self.handle_predicate_term(&prefix, &predicate, &object)
                }
                ParsedLine::Subject(_, _) => Err(TriplesError::NotImplemented),
            },
        }
    }
}
