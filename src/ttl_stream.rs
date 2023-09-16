use crate::data::DataError;
use crate::data::RdfName;
use crate::data::Subject;
use crate::ttl::LineParser;
use crate::ttl_data::ParsedLine;
use std::collections::HashMap;

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

impl TtlStream {
    pub fn new() -> Self {
        let parser = LineParser::new();
        Self {
            state: ParserState::SubjectLoading,
            parser,
            prefixes: HashMap::new(),
            current_subject: None,
        }
    }

    fn resolve_iri(&self, prefix: Option<&String>, local_name: &str) -> Result<String, DataError> {
        match prefix {
            Some(n) => match self.prefixes.get(n) {
                Some(ns) => {
                    let cleaned_ns = ns.trim_end_matches('/');
                    Ok(format!("{}/{}", cleaned_ns, local_name))
                }
                None => Err(DataError::UnresolvableURIPrefix),
            },
            None => Ok(local_name.to_string()),
        }
    }

    fn handle_subject(
        &mut self,
        prefix: Option<String>,
        name: &str,
    ) -> Result<Option<Subject>, DataError> {
        if self.current_subject.is_some() {
            return Err(DataError::PreviousSubjectNotComplete);
        }
        let subject_iri_text = self.resolve_iri(prefix.as_ref(), name)?;
        let subject_iri = RdfName::new(subject_iri_text).map_err(|_| DataError::InvalidIRI)?;
        self.current_subject = Some(Subject::new(subject_iri));
        self.state = ParserState::PredicateLoading;
        Ok(None)
    }

    fn handle_predicate(
        &mut self,
        prefix: Option<String>,
        predicate: &str,
        object: &str,
    ) -> Result<Option<Subject>, DataError> {
        let predicate_iri_text = self.resolve_iri(prefix.as_ref(), predicate)?;
        let predicate_iri = RdfName::new(predicate_iri_text).map_err(|_| DataError::InvalidIRI)?;
        if let Some(ref mut subject) = self.current_subject {
            subject.add(predicate_iri, object.to_string());
            Ok(None)
        } else {
            Err(DataError::NoSubjectDeclaired)
        }
    }

    fn handle_predicate_term(
        &mut self,
        prefix: Option<String>,
        predicate: &str,
        object: &str,
    ) -> Result<Option<Subject>, DataError> {
        // Since the logic is same as handle_predicate for now, reuse it
        let result = self.handle_predicate(prefix, predicate, object)?;
        if result.is_none() && self.current_subject.is_some() {
            let finished_subject = self.current_subject.clone();
            self.current_subject = None;
            self.state = ParserState::SubjectLoading;
            // If successfully added and current subject exists, clone and return
            return Ok(Some(finished_subject.unwrap()));
        }
        Ok(result)
    }

    pub fn load(&mut self, line: &str) -> Result<Option<Subject>, DataError> {
        let parsed = self.parser.parse(line).unwrap();
        match self.state {
            ParserState::SubjectLoading => match parsed {
                ParsedLine::Prefix(name, uri) => {
                    self.prefixes.insert(name, uri);
                    Ok(None)
                }
                ParsedLine::Subject(prefix, name) => self.handle_subject(prefix, &name),
                _ => Err(DataError::NoSubjectDeclaired),
            },
            ParserState::PredicateLoading => match parsed {
                // you can load prefixes later in the file but only in-between subject blocks
                ParsedLine::Prefix(name, uri) => {
                    if self.current_subject.is_some() {
                        return Err(DataError::PreviousSubjectNotComplete);
                    }
                    self.prefixes.insert(name, uri);
                    Ok(None)
                }
                ParsedLine::PredObj(prefix, predicate, object) => {
                    self.handle_predicate(prefix, &predicate, &object)
                }
                ParsedLine::PredObjTerm(prefix, predicate, object) => {
                    self.handle_predicate_term(prefix, &predicate, &object)
                }
                _ => Err(DataError::NotImplemented),
            },
        }
    }
}
