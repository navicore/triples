use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RdfNameError {
    InvalidIRI,
    // Add more error variants here as needed.
}
impl std::error::Error for RdfNameError {}
impl fmt::Display for RdfNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidIRI => write!(f, "Invalid IRI"),
        }
    }
}

/// Represents an RDF name (a simple IRI validation is performed).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdfName(String);

impl RdfName {
    /// # Errors
    /// Will return `Err` if name is not a valid IRI
    pub fn new(name: String) -> Result<Self, RdfNameError> {
        if Self::is_valid(&name) {
            Ok(Self(name))
        } else {
            Err(RdfNameError::InvalidIRI)
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
    predicates_objects: HashMap<RdfName, String>,
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Subject IRI: {}, Predicate Objects: {}",
            self.subject.0,
            self.predicates_objects.len()
        )
    }
}

impl Subject {
    /// Creates a new `Subject` with the given subject.
    #[must_use]
    pub fn new(subject: RdfName) -> Self {
        Self {
            subject,
            predicates_objects: HashMap::new(),
        }
    }

    #[must_use]
    pub const fn name(&self) -> &RdfName {
        &self.subject
    }

    #[must_use]
    pub const fn predicates_objects(&self) -> &HashMap<RdfName, String> {
        &self.predicates_objects
    }

    /// Adds or updates a predicate/object pair for the subject.
    pub fn add(&mut self, predicate: RdfName, object: String) {
        self.predicates_objects.insert(predicate, object);
    }

    /// Removes a predicate/object pair given the predicate. Returns true if the predicate was present.
    pub fn remove(&mut self, predicate: &RdfName) -> bool {
        self.predicates_objects.remove(predicate).is_some()
    }

    /// Fetches the object for a given predicate, if it exists.
    #[must_use]
    pub fn get(&self, predicate: &RdfName) -> Option<&String> {
        self.predicates_objects.get(predicate)
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
