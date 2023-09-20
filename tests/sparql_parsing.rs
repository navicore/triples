#[cfg(test)]
mod tests {
    use triples::sparql::QueryParser;

    #[test]
    fn test_parse_query() {
        let parser = QueryParser::new(); // Create a new instance of the generated parser

        // The SPARQL query from above
        let input_query = r#"SELECT DISTINCT ?appname WHERE {
            ?s <http://k8p.navicore.tech/property/k8p_appname> ?appname .
            ?s <http://k8p.navicore.tech/property/k8p_metric_name> ?metric
        }"#;

        // Parse the query
        let result = parser.parse(input_query);

        // Assert that the result is Ok and matches the expected output
        // This is just a basic check; you can adjust based on how you've set up the grammar.
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(result.unwrap(), "SELECT DISTINCT ?appname WHERE { ?s <http://k8p.navicore.tech/property/k8p_appname> ?appname . ?s <http://k8p.navicore.tech/property/k8p_metric_name> ?metric }");
    }
}
