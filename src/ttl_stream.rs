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

    pub fn load(&mut self, line: &str) -> Result<Option<Subject>, DataError> {
        let parsed = self.parser.parse(line).unwrap();
        match self.state {
            ParserState::SubjectLoading => match parsed {
                ParsedLine::Prefix(name, uri) => {
                    self.prefixes.insert(name, uri);
                    Ok(None)
                }
                ParsedLine::Subject(pre, name) => {
                    if self.current_subject.is_some() {
                        Err(DataError::PreviousSubjectNotComplete)
                    } else {
                        let subject_iri_text = match pre {
                            Some(n) => match self.prefixes.get(&n) {
                                Some(ns) => {
                                    let cleaned_ns = ns.trim_end_matches('/');
                                    let fullname = format!("{}/{}", cleaned_ns, name);
                                    Ok(fullname)
                                }
                                _ => Err(DataError::UnresolvableURIPrefix),
                            },
                            _ => Ok(name),
                        }?;
                        match RdfName::new(subject_iri_text) {
                            Ok(subject_iri) => {
                                let subject = Subject::new(subject_iri);
                                self.current_subject = Some(subject);
                                self.state = ParserState::PredicateLoading;
                                Ok(None)
                            }
                            _ => Err(DataError::InvalidIRI),
                        }
                    }
                }
                _ => Err(DataError::NoSubjectDeclaired),
            },
            ParserState::PredicateLoading => match parsed {
                // these are normally loaded at the top of a file / stream but could be in between
                // subjects for readability
                ParsedLine::Prefix(name, uri) => {
                    if self.current_subject.is_some() {
                        Err(DataError::PreviousSubjectNotComplete)
                    } else {
                        self.prefixes.insert(name, uri);
                        Ok(None)
                    }
                }

                ParsedLine::PredObj(pre, predicate, object) => {
                    let predicate_iri_text = match pre {
                        Some(n) => match self.prefixes.get(&n) {
                            Some(ns) => {
                                let cleaned_ns = ns.trim_end_matches('/');
                                let fullname = format!("{}/{}", cleaned_ns, predicate);
                                Ok(fullname)
                            }
                            _ => Err(DataError::UnresolvableURIPrefix),
                        },
                        _ => Ok(predicate),
                    }?;
                    match RdfName::new(predicate_iri_text) {
                        Ok(predicate_iri) => {
                            if let Some(ref mut subject) = self.current_subject {
                                subject.add(predicate_iri, object);
                                Ok(None)
                            } else {
                                Err(DataError::NoSubjectDeclaired)
                            }
                        }
                        _ => Err(DataError::InvalidIRI),
                    }
                }
                ParsedLine::PredObjTerm(pre, predicate, object) => {
                    let predicate_iri_text = match pre {
                        Some(n) => match self.prefixes.get(&n) {
                            Some(ns) => {
                                let fullname = format!("{ns}/{predicate}");
                                Ok(fullname)
                            }
                            _ => Err(DataError::UnresolvableURIPrefix),
                        },
                        _ => Ok(predicate),
                    }?;
                    match RdfName::new(predicate_iri_text) {
                        Ok(predicate_iri) => {
                            if let Some(ref mut subject) = self.current_subject {
                                subject.add(predicate_iri, object);
                                Ok(Some(subject.clone()))
                            } else {
                                Err(DataError::NoSubjectDeclaired)
                            }
                        }
                        _ => Err(DataError::InvalidIRI),
                    }
                }

                _ => Err(DataError::NotImplemented),
            },
        }
    }
}
