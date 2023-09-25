use triples::data::Pre;
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
}

#[test]
fn test_subject_line_parsing() {
    let parser = LineParser::new();

    let line1 = "res:505776d3-80ea-497f-a4ef-753eeb418c50";
    let parsed1 = parser.parse(line1).unwrap();
    assert_eq!(
        parsed1,
        ParsedLine::Subject(
            Some(Pre::new("res".to_string())),
            RdfName::new("505776d3-80ea-497f-a4ef-753eeb418c50".to_string())
        )
    );
}

#[test]
fn test_predicate_object_no_ns_parsing() {
    let parser = LineParser::new();

    let line2 = "k8p_metric_name \"envoy_cluster_internal_upstream_rq_200\";";
    let parsed2 = parser.parse(line2).unwrap();
    assert_eq!(
        parsed2,
        ParsedLine::PredObj(
            None,
            RdfName::new("k8p_metric_name".to_string()),
            None,
            "envoy_cluster_internal_upstream_rq_200".to_string()
        )
    );
}

#[test]
fn test_predicate_object_line_parsing() {
    let parser = LineParser::new();

    let line2 = "    prop:k8p_metric_name \"envoy_cluster_internal_upstream_rq_200\";";
    let parsed2 = parser.parse(line2).unwrap();
    assert_eq!(
        parsed2,
        ParsedLine::PredObj(
            Some(Pre::new("prop".to_string())),
            RdfName::new("k8p_metric_name".to_string()),
            None,
            "envoy_cluster_internal_upstream_rq_200".to_string()
        )
    );
}

#[test]
fn test_predicate_object_line_parsing_term() {
    let parser = LineParser::new();
    // add some white space and chars that mean something elsewhere in the grammar
    let line2 = "    prop:k8p_metric_name \"envoy_cluster:internal upstream_rq_200\"; .";
    let parsed2 = parser.parse(line2).unwrap();
    assert_eq!(
        parsed2,
        ParsedLine::PredObjTerm(
            Some(Pre::new("prop".to_string())),
            RdfName::new("k8p_metric_name".to_string()),
            None,
            "envoy_cluster:internal upstream_rq_200".to_string()
        )
    );
}

#[test]
fn test_escaped_quotes_in_object() {
    let parser = LineParser::new();
    let line2 = r#"prop:k8p_description "The \"recent cpu usage\" of the system the application is running in" ;"#;
    let parsed2 = parser.parse(line2).unwrap();
    assert_eq!(
        parsed2,
        ParsedLine::PredObj(
            Some(Pre::new("prop".to_string())),
            RdfName::new("k8p_description".to_string()),
            None,
            r#"The \"recent cpu usage\" of the system the application is running in"#.to_string()
        )
    );
}

#[test]
fn test_parsing_error_handling() {
    let parser = LineParser::new();
    // add some white space and chars that mean something elsewhere in the grammar
    let line2 = "    prop-k8p_metric_name \"envoy_cluster:internal upstream_rq_200\"; .";
    let parsed2 = parser.parse(line2);
    assert!(!parsed2.is_ok());
}
