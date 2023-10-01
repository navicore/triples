use crate::data::{Pre, RdfName, Subject, TriplesError};
use crate::turtle::LineParser;
use std::collections::HashMap;
use std::fmt;
use tracing::trace;

#[allow(dead_code)] // clippy can't see lalrpop
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParsedLine {
    Prefix(Pre, RdfName),
    Subject(Option<Pre>, RdfName),
    PredObj(Option<Pre>, RdfName, Option<Pre>, String, bool),
    PredObjTerm(Option<Pre>, RdfName, Option<Pre>, String),
    ContinueObj(Option<Pre>, RdfName, bool),
    SubjectPredObj(
        Option<Pre>,
        RdfName,
        Option<Pre>,
        RdfName,
        Option<Pre>,
        String,
        bool,
    ),
}

pub struct TurtleStream {
    state: ParserState,
    parser: LineParser,
    prefixes: HashMap<Pre, RdfName>,
    current_subject: Option<Subject>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParserState {
    SubjectLoading,
    PredicateLoading,
    ObjectLoading(Option<Pre>, RdfName),
}

impl fmt::Display for ParserState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SubjectLoading => write!(f, "subject loading state"),
            Self::PredicateLoading => write!(f, "predicate loading state"),
            Self::ObjectLoading(pre, name) => write!(f, "object loading state: {pre:?}:{name}"),
        }
    }
}

impl Default for TurtleStream {
    fn default() -> Self {
        Self::new()
    }
}

impl TurtleStream {
    #[must_use]
    pub fn get_state(&self) -> ParserState {
        self.state.clone()
    }

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

    fn resolve_obj_iri(
        &self,
        prefix: Option<&Pre>,
        object: &String,
    ) -> Result<String, TriplesError> {
        prefix.map_or_else(
            || Ok(object.clone()),
            |ns| {
                self.prefixes.get(ns).map_or_else(
                    || {
                        Err(TriplesError::UnresolvableURIPrefix {
                            prefix_name: ns.to_string(),
                            name: object.to_string(),
                        })
                    },
                    |ns| {
                        let ns_str = ns.to_string();
                        if ns_str.ends_with('#') {
                            Ok(format!("{ns_str}{object}"))
                        } else {
                            Ok(format!("{ns_str}/{object}"))
                        }
                    },
                )
            },
        )
    }

    fn resolve_iri(
        &self,
        prefix: Option<&Pre>,
        local_name: &RdfName,
    ) -> Result<String, TriplesError> {
        prefix.map_or_else(
            || Ok(local_name.to_string()),
            |ns| {
                self.prefixes.get(ns).map_or_else(
                    || {
                        Err(TriplesError::UnresolvableURIPrefix {
                            prefix_name: ns.to_string(),
                            name: local_name.to_string(),
                        })
                    },
                    |ns| {
                        let ns_str = ns.to_string();
                        trace!("resolve_iri for ns: {ns_str}");
                        if ns_str.ends_with('#') {
                            Ok(format!("{ns_str}{local_name}"))
                        } else {
                            Ok(format!("{ns_str}/{local_name}"))
                        }
                    },
                )
            },
        )
    }

    fn handle_subject(
        &mut self,
        prefix: &Option<Pre>,
        name: &RdfName,
    ) -> Result<Option<Subject>, TriplesError> {
        if self.current_subject.is_some() {
            return Err(TriplesError::PreviousSubjectNotComplete);
        }
        let subject_iri_text = self.resolve_iri(prefix.as_ref(), name)?;
        let subject_iri = RdfName::new(subject_iri_text);
        self.current_subject = Some(Subject::new(subject_iri));
        self.state = ParserState::PredicateLoading;
        Ok(None)
    }

    fn handle_predicate(
        &mut self,
        prefix: &Option<Pre>,
        predicate: &RdfName,
        opre: &Option<Pre>,
        object: &str,
        has_more: bool,
    ) -> Result<Option<Subject>, TriplesError> {
        let predicate_iri_text = self.resolve_iri(prefix.as_ref(), predicate)?;
        let predicate_iri = RdfName::new(predicate_iri_text);

        let object_iri_text = self.resolve_obj_iri(opre.as_ref(), &object.to_string())?;

        if has_more {
            self.state = ParserState::ObjectLoading(prefix.clone(), predicate.clone());
        }
        self.current_subject
            .as_mut()
            .map_or(Err(TriplesError::NoSubjectDeclaired), |subject| {
                subject.add(predicate_iri, object_iri_text);
                Ok(None)
            })
    }

    fn handle_predicate_term(
        &mut self,
        prefix: &Option<Pre>,
        predicate: &RdfName,
        objpre: &Option<Pre>,
        object: &str,
    ) -> Result<Option<Subject>, TriplesError> {
        // Since the logic is same as handle_predicate for now, reuse it
        let result = self.handle_predicate(prefix, predicate, objpre, object, false)?;
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
        trace!("load: {line}");
        let parsed = &self
            .parser
            .parse(line)
            .map_err(|e| TriplesError::ParseError {
                reason: e.to_string(),
            })?;
        match self.state.clone() {
            ParserState::ObjectLoading(pre, predicate) => {
                self.handle_object_loading_state(parsed, &pre, &predicate)
            }
            ParserState::SubjectLoading => self.handle_subject_loading_state(parsed),
            ParserState::PredicateLoading => self.handle_predicate_loading_state(parsed),
        }
    }

    fn handle_object_loading_state(
        &mut self,
        parsed: &ParsedLine,
        pre: &Option<Pre>,
        predicate: &RdfName,
    ) -> Result<Option<Subject>, TriplesError> {
        trace!("object loading");
        // Handling for ObjectLoading state
        // ... (implement the specific logic for ObjectLoading state here)
        trace!("object loading");
        match parsed {
            ParsedLine::Prefix(_, _) => Err(TriplesError::NotImplemented {
                trace: "object loading prefix".to_string(),
            }),
            ParsedLine::Subject(_, _) => Err(TriplesError::NotImplemented {
                trace: "object loading subject".to_string(),
            }),
            ParsedLine::PredObj(prefix, predicate, objpre, object, is_end) => {
                self.handle_predicate(prefix, predicate, objpre, object, *is_end)
            }
            ParsedLine::PredObjTerm(prefix, predicate, objpre, object) => {
                self.handle_predicate_term(prefix, predicate, objpre, object)
            }
            ParsedLine::SubjectPredObj(_, _, _, _, _, _, _) => Err(TriplesError::NotImplemented {
                trace: "object loading subpredobj".to_string(),
            }),
            ParsedLine::ContinueObj(objpre, obj, has_more) => {
                if *has_more {
                    self.handle_predicate(pre, predicate, objpre, &obj.to_string(), *has_more)
                } else {
                    self.handle_predicate_term(pre, predicate, objpre, &obj.to_string())
                }
            }
        }
    }

    fn handle_subject_loading_state(
        &mut self,
        parsed: &ParsedLine,
    ) -> Result<Option<Subject>, TriplesError> {
        trace!("subject loading");
        match parsed {
            ParsedLine::Prefix(pre, uri) => {
                self.prefixes.insert(pre.clone(), uri.clone());
                Ok(None)
            }
            ParsedLine::Subject(prefix, name) => self.handle_subject(prefix, name),
            ParsedLine::PredObj(_, _, _, _, _) | ParsedLine::PredObjTerm(_, _, _, _) => {
                Err(TriplesError::NotImplemented {
                    trace: "subject loading predobj".to_string(),
                })
            }
            ParsedLine::SubjectPredObj(
                name_prefix,
                name,
                prefix,
                predicate,
                objpre,
                object,
                has_more,
            ) => {
                self.handle_subject(name_prefix, name)?;
                let result = self.handle_predicate(prefix, predicate, objpre, object, *has_more);
                if *has_more {
                    self.state = ParserState::ObjectLoading(prefix.clone(), predicate.clone());
                } else {
                    self.current_subject = None;
                    self.state = ParserState::SubjectLoading;
                };
                result
            }
            ParsedLine::ContinueObj(_, _, _) => Err(TriplesError::NotImplemented {
                trace: "subject loading contobj".to_string(),
            }),
        }
    }

    fn handle_predicate_loading_state(
        &mut self,
        parsed: &ParsedLine,
    ) -> Result<Option<Subject>, TriplesError> {
        trace!("predicate loading");
        match parsed {
            // you can load prefixes later in the file but only in-between subject blocks
            ParsedLine::Prefix(name, uri) => {
                if self.current_subject.is_some() {
                    return Err(TriplesError::PreviousSubjectNotComplete);
                }
                self.prefixes.insert(name.clone(), uri.clone());
                Ok(None)
            }
            ParsedLine::PredObj(prefix, predicate, opre, object, is_end) => {
                self.handle_predicate(prefix, predicate, opre, object, *is_end)
            }
            ParsedLine::PredObjTerm(prefix, predicate, opre, object) => {
                self.handle_predicate_term(prefix, predicate, opre, object)
            }
            ParsedLine::Subject(_, _) => Err(TriplesError::NotImplemented {
                trace: "predicate loading subj".to_string(),
            }),
            ParsedLine::SubjectPredObj(_, _, _, _, _, _, _) => Err(TriplesError::NotImplemented {
                trace: "predicate loading subjpredobj".to_string(),
            }),
            ParsedLine::ContinueObj(_, _, _) => Err(TriplesError::NotImplemented {
                trace: "predicate loading contobj".to_string(),
            }),
        }
    }
}
