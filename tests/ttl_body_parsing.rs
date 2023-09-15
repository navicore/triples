use triples::ttl::LineParser;
use triples::ttl_data::ParsedLine;

#[test]
fn test_subject_line_parsing() {
    let parser = LineParser::new();

    let line1 = "res:505776d3-80ea-497f-a4ef-753eeb418c50";
    let parsed1 = parser.parse(line1).unwrap();
    assert_eq!(
        parsed1,
        ParsedLine::Subject(
            Some("res".to_string()),
            "505776d3-80ea-497f-a4ef-753eeb418c50".to_string()
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
            "k8p_metric_name".to_string(),
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
            Some("prop".to_string()),
            "k8p_metric_name".to_string(),
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
            Some("prop".to_string()),
            "k8p_metric_name".to_string(),
            "envoy_cluster:internal upstream_rq_200".to_string()
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
