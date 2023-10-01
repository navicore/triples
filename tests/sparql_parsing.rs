#[cfg(test)]
mod tests {
    use triples::sparql::QueryParser;
    use triples::sparql_data;

    #[test]
    fn test_parse_query() {
        let parser = QueryParser::new();

        // The SPARQL query from above
        let input_query = r#"SELECT DISTINCT ?appname WHERE {
            ?s <http://k8p.navicore.tech/property/k8p_appname> ?appname .
            ?s <http://k8p.navicore.tech/property/k8p_metric_name> ?metric
        }"#;

        let result = parser.parse(input_query);

        assert!(result.is_ok(), "{result:?}");

        assert_eq!(
            result.unwrap(),
            sparql_data::SparqlQuery {
                select_clause: sparql_data::SelectClause {
                    distinct: true,
                    variables: vec![sparql_data::Variable::Var("appname".to_string())],
                },
                triples_block: vec![
                    sparql_data::TriplePattern {
                        subject: sparql_data::Variable::Var("s".to_string()),
                        predicate: "http://k8p.navicore.tech/property/k8p_appname".to_string(),
                        object: sparql_data::Variable::Var("appname".to_string())
                    },
                    sparql_data::TriplePattern {
                        subject: sparql_data::Variable::Var("s".to_string()),
                        predicate: "http://k8p.navicore.tech/property/k8p_metric_name".to_string(),
                        object: sparql_data::Variable::Var("metric".to_string())
                    }
                ],
            }
        );
    }
}
