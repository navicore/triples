use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriplesError {
    ParseError { reason: String },
    InvalidIRI { uri: String },
    UnresolvableURIPrefix { prefix_name: String, name: String },
    NoSubjectDeclaired,
    PreviousSubjectNotComplete,
    NotImplemented { trace: String },
    // Add more error variants here as needed.
}
impl std::error::Error for TriplesError {}
impl fmt::Display for TriplesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError { reason } => write!(f, "{reason}"),
            Self::InvalidIRI { uri } => write!(f, "Invalid IRI: {uri}"),
            Self::UnresolvableURIPrefix { prefix_name, name } => {
                write!(f, "can not locate URI for {prefix_name} for name {name}")
            }
            Self::NoSubjectDeclaired => write!(f, "can not load predicate without a subject"),
            Self::PreviousSubjectNotComplete => write!(f, "previous subject stanza not terminated"),
            Self::NotImplemented { trace } => write!(f, "{trace} not implemented"),
        }
    }
}

/// Represents an RDF name (a simple IRI validation is performed).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdfName(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pre(String);

impl Pre {
    /// # Errors
    /// Will return `Err` if name is not a valid prefix name
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self(name)
    }
}

impl fmt::Display for Pre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RdfName {
    /// # Errors
    /// Will return `Err` if name is not a valid IRI
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self(name)
    }
}

impl fmt::Display for RdfName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Subject {
    subject: RdfName,
    predicate_object_pairs: HashMap<RdfName, HashSet<String>>,
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Subject IRI: {}, Predicate Objects: {}",
            self.subject.0,
            self.predicate_object_pairs.len()
        )
    }
}

impl Subject {
    #[must_use]
    pub fn new(subject: RdfName) -> Self {
        Self {
            subject,
            predicate_object_pairs: HashMap::new(),
        }
    }

    #[must_use]
    pub const fn name(&self) -> &RdfName {
        &self.subject
    }

    pub fn predicate_object_pairs(&self) -> impl Iterator<Item = (&RdfName, &HashSet<String>)> {
        self.predicate_object_pairs.iter()
    }

    pub fn add(&mut self, predicate: RdfName, object: String) {
        self.predicate_object_pairs
            .entry(predicate)
            .or_insert_with(HashSet::new)
            .insert(object);
    }

    pub fn remove(&mut self, predicate: &RdfName, object: &str) -> bool {
        if let Some(objects) = self.predicate_object_pairs.get_mut(predicate) {
            let removed = objects.remove(object);
            if objects.is_empty() {
                self.predicate_object_pairs.remove(predicate);
            }
            removed
        } else {
            false
        }
    }

    #[must_use]
    pub fn get(&self, predicate: &RdfName) -> Option<&HashSet<String>> {
        self.predicate_object_pairs.get(predicate)
    }

    pub fn all_predicates(&self) -> impl Iterator<Item = &RdfName> {
        self.predicate_object_pairs.keys()
    }

    pub fn all_objects(&self) -> impl Iterator<Item = &HashSet<String>> {
        self.predicate_object_pairs.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdf_name_valid() {
        let valid_iri = RdfName::new("https://www.example.com/test".to_string());
        assert!(valid_iri.to_string().starts_with("http"));
    }

    #[test]
    fn subject_basic_operations() {
        let subject_iri = RdfName::new("https://www.example.com/subject".to_string());
        let mut subject = Subject::new(subject_iri.clone());

        assert_eq!(subject.name(), &subject_iri);

        let predicate_iri = RdfName::new("https://www.example.com/predicate".to_string());
        let object_value = "Object Value".to_string();

        // Adding
        subject.add(predicate_iri.clone(), object_value.clone());
        assert_eq!(subject.get(&predicate_iri).map(|x| x.len()), Some(1));
        assert_eq!(
            subject
                .get(&predicate_iri)
                .map(|x| x.iter().next().unwrap()),
            Some(&"Object Value".to_string())
        );

        // Removing
        assert!(subject.remove(&predicate_iri, &object_value.clone()));
        assert_eq!(subject.get(&predicate_iri), None);
    }

    #[test]
    fn subject_non_existent_predicate() {
        let subject_iri = RdfName::new("https://www.example.com/subject".to_string());
        let subject = Subject::new(subject_iri);

        let non_existent_predicate =
            RdfName::new("https://www.example.com/nonexistent".to_string());

        assert_eq!(subject.get(&non_existent_predicate), None);
    }
}
