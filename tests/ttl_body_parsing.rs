use triples::ttl_body::LineParser;
use triples::ttl_data::{ParsedLine, Terminator};

//#[test]
fn test_subject_line_parsing() {
    let parser = LineParser::new();

    let line1 = "res:505776d3-80ea-497f-a4ef-753eeb418c50";
    let parsed1 = parser.parse(line1).unwrap();
    assert_eq!(
        parsed1,
        ParsedLine::Subject("res:505776d3-80ea-497f-a4ef-753eeb418c50".to_string())
    );
}

//#[test]
fn test_predicate_object_line_parsing() {
    let parser = LineParser::new();

    let line2 = "    prop:k8p_metric_name \"envoy_cluster_internal_upstream_rq_200\";";
    let parsed2 = parser.parse(line2).unwrap();
    assert_eq!(
        parsed2,
        ParsedLine::PredObj(
            "prop:k8p_metric_name".to_string(),
            "envoy_cluster_internal_upstream_rq_200".to_string(),
            Terminator::SemiColon
        )
    );
}
