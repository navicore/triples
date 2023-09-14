use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub prefix); // synthesized by LALRPOP

fn test_prefix(input: &str) {
    let parser = prefix::PrefixParser::new();
    let result = parser.parse(input);
    assert!(result.is_ok(), "Parse failed with: {:?}", result);
    assert_eq!(
        result.clone().unwrap(),
        ("myns".to_string(), "http://example.com/myns#".to_string()),
        "Parse failed with: {:?}",
        result
    );
}

#[test]
fn test_prefix_basic() {
    let input = "@prefix myns: <http://example.com/myns#> .";
    test_prefix(input);
    // let parser = prefix::PrefixParser::new();
    // let result = parser.parse(input);
    // assert!(result.is_ok(), "Parse failed with: {:?}", result);
    // assert_eq!(
    //     result.clone().unwrap(),
    //     ("myns".to_string(), "http://example.com/myns#".to_string()),
    //     "Parse failed with: {:?}",
    //     result
    // );
}
// #[test]
// fn test_prefix_parsing() {
//     // Valid input
//     //let input = "@prefix myns: <http://example.com/myns#> .";
//     //let input = "@prefix ns: http .";
//     let input = "http";
//     match prefix::PrefixParser::new().parse(input) {
//         Ok((ns, uri)) => {
//             assert_eq!(ns, "ns");
//             assert_eq!(uri, "http://example.com/ns#");
//         }
//         d => assert!(false, "Failed to parse valid input: {:?}", d),
//     }
//
//     // Optionally, you can add more checks for valid inputs.
//
//     // Test for invalid input
//     let invalid_input = "@prefix ns http://example.com/ns# .";
//     assert!(
//         prefix::PrefixParser::new().parse(invalid_input).is_err(),
//         "Invalid input was wrongly accepted"
//     );
//
//     // Test for input with different whitespace patterns
//     let input_with_spaces = "@prefix    ns  :    <http://example.com/ns#>    .";
//     match prefix::PrefixParser::new().parse(input_with_spaces) {
//         Ok((ns, uri)) => {
//             assert_eq!(ns, "ns");
//             assert_eq!(uri, "http://example.com/ns#");
//         }
//         _ => assert!(false, "Failed to parse input with spaces"),
//     }
//
//     // ... And you can add more test cases as needed
// }
