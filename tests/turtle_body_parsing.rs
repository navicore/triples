use triples::data::RdfName;
use triples::turtle::LineParser;
use triples::turtle::RDFNameParser;
use triples::turtle_stream::ParsedLine;

#[test]
fn test_subject_as_uri_parsing() {
    let parser = RDFNameParser::new();

    let input = "<http://cmu.edu/building/ontology/ghc#8Floor>";
    let parsed1 = parser.parse(input).unwrap();
    assert_eq!(
        parsed1,
        (
            None,
            "http://cmu.edu/building/ontology/ghc#8Floor".to_string()
        )
    );
}

#[test]
fn test_subject_as_uuid_parsing() {
    let parser = RDFNameParser::new();
    let input = "e5913d92-5ad7-11ee-9008-4b63415ab399";
    let parsed1 = parser.parse(input).unwrap();
    assert_eq!(
        parsed1,
        (None, "e5913d92-5ad7-11ee-9008-4b63415ab399".to_string())
    );
}

#[test]
fn test_isa_parsing() {
    let parser = LineParser::new();
    let input = "<http://cmu.edu/building/ontology/ghc#8Floor> a brick:Floor .";
    let parsed1 = parser.parse(input).unwrap();
    if let ParsedLine::SubjectPredObjTerm(spre, subj, predpre, pred, opre, obj) = parsed1 {
        assert!(spre.is_none());
        assert_eq!(
            subj,
            RdfName::new("http://cmu.edu/building/ontology/ghc#8Floor".to_string())
        );
        assert!(predpre.is_some());
        assert_eq!(pred, RdfName::new("type".to_string()));
        assert!(opre.is_some());
        assert_eq!(obj, "Floor".to_string());
    } else {
        assert!(false)
    };
    // assert_eq!(
    //     parsed1,
    //     (
    //         None,
    //         "http://cmu.edu/building/ontology/ghc#8Floor".to_string()
    //     )
    // );
}
