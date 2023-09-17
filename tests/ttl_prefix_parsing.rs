use triples::ttl::LineParser;
use triples::ttl_stream::ParsedLine;

fn test_prefix(input: &str) {
    let parser = LineParser::new();
    let result = parser.parse(input);
    assert!(result.is_ok(), "Parse failed with: {:?}", result);
    match result {
        Ok(ParsedLine::Prefix(ns, name)) => {
            assert_eq!(ns, "myns");
            assert!(name.ends_with("://example.com/myns#"));
        }
        _ => {}
    };
}

fn test_bad_prefix(input: &str) {
    let parser = LineParser::new();
    let result = parser.parse(input);
    assert!(
        !result.is_ok(),
        "Parse error handling failed with: {:?}",
        result
    );
}

#[test]
fn test_prefix_basic() {
    let input = "@prefix myns: <http://example.com/myns#> .";
    test_prefix(input);
    let input = "@prefix myns: <https://example.com/myns#> .";
    test_prefix(input);
    let input = "@prefix myns:<https://example.com/myns#> .";
    test_prefix(input);
    let input = "@prefix myns:<https://example.com/myns#>.";
    test_prefix(input);
    let input = "    @prefix myns:<https://example.com/myns#>.";
    test_prefix(input);
    let input = "\t@prefix myns:<https://example.com/myns#>.";
    test_prefix(input);
}

#[test]
fn test_error_handling() {
    let input = "\t@pefix myns:<https://example.com/myns#>.";
    test_bad_prefix(input);
}
