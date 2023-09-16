use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataError {
    InvalidIRI,
    UnresolvableURIPrefix,
    NoSubjectDeclaired,
    PreviousSubjectNotComplete,
    NotImplemented,
    // Add more error variants here as needed.
}
impl std::error::Error for DataError {}
impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidIRI => write!(f, "Invalid IRI"),
            Self::UnresolvableURIPrefix => write!(f, "can not locate URI for prefix"),
            Self::NoSubjectDeclaired => write!(f, "can not load predicate without a subject"),
            Self::PreviousSubjectNotComplete => write!(f, "previous subject stanza not terminated"),
            Self::NotImplemented => write!(f, "not implemented"),
        }
    }
}

/// Represents an RDF name (a simple IRI validation is performed).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdfName(String);

impl RdfName {
    /// # Errors
    /// Will return `Err` if name is not a valid IRI
    pub fn new(name: String) -> Result<Self, DataError> {
        if Self::is_valid(&name) {
            Ok(Self(name))
        } else {
            Err(DataError::InvalidIRI)
        }
    }

    /// Checks if the given string is a valid IRI.
    fn is_valid(name: &str) -> bool {
        Regex::new(r"^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$")
            .map_or(false, |iri_regex| iri_regex.is_match(name))
    }
}

impl fmt::Display for RdfName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Encapsulates a single subject that has multiple predicate/object pairs.
#[derive(Debug, Clone)]
pub struct Subject {
    subject: RdfName,
    predicate_object_pairs: HashMap<RdfName, String>,
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
    /// Creates a new `Subject` with the given subject.
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

    pub fn predicate_object_pairs(&self) -> impl Iterator<Item = (&RdfName, &String)> {
        self.predicate_object_pairs.iter()
    }
    /// Adds or updates a predicate/object pair for the subject.
    pub fn add(&mut self, predicate: RdfName, object: String) {
        self.predicate_object_pairs.insert(predicate, object);
    }

    /// Removes a predicate/object pair given the predicate. Returns true if the predicate was present.
    pub fn remove(&mut self, predicate: &RdfName) -> bool {
        self.predicate_object_pairs.remove(predicate).is_some()
    }

    /// Fetches the object for a given predicate, if it exists.
    #[must_use]
    pub fn get(&self, predicate: &RdfName) -> Option<&String> {
        self.predicate_object_pairs.get(predicate)
    }

    /// Returns an iterator over all predicates.
    pub fn all_predicates(&self) -> impl Iterator<Item = &RdfName> {
        self.predicate_object_pairs.keys()
    }

    /// Returns an iterator over all objects.
    pub fn all_objects(&self) -> impl Iterator<Item = &String> {
        self.predicate_object_pairs.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdf_name_valid() {
        let valid_iri = RdfName::new("https://www.example.com/test".to_string());
        assert!(valid_iri.is_ok());
    }

    #[test]
    fn rdf_name_invalid() {
        let invalid_iri = RdfName::new("invalid_string".to_string());
        assert!(!invalid_iri.is_ok());
    }

    #[test]
    fn subject_basic_operations() {
        let subject_iri = RdfName::new("https://www.example.com/subject".to_string()).unwrap();
        let mut subject = Subject::new(subject_iri.clone());

        assert_eq!(subject.name(), &subject_iri);

        let predicate_iri = RdfName::new("https://www.example.com/predicate".to_string()).unwrap();
        let object_value = "Object Value".to_string();

        // Adding
        subject.add(predicate_iri.clone(), object_value.clone());
        assert_eq!(subject.get(&predicate_iri), Some(&object_value));

        // Removing
        assert!(subject.remove(&predicate_iri));
        assert_eq!(subject.get(&predicate_iri), None);
    }

    #[test]
    fn subject_non_existent_predicate() {
        let subject_iri = RdfName::new("https://www.example.com/subject".to_string()).unwrap();
        let subject = Subject::new(subject_iri);

        let non_existent_predicate =
            RdfName::new("https://www.example.com/nonexistent".to_string()).unwrap();

        assert_eq!(subject.get(&non_existent_predicate), None);
    }
}
