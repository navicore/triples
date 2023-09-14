use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub prefix); // synthesized by LALRPOP

fn test_prefix(input: &str) {
    let parser = prefix::PrefixParser::new();
    let result = parser.parse(input);
    assert!(result.is_ok(), "Parse failed with: {:?}", result);
    assert_eq!(
        result.clone().unwrap().0,
        "myns".to_string(),
        "Parse failed with: {:?}",
        result
    );
    assert!(
        result.clone().unwrap().1.ends_with("://example.com/myns#"),
        "Parse failed with: {:?}",
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
}
