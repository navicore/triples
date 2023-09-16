use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::ttl::LineParser;
use triples::ttl_data::ParsedLine;

#[test]
fn test_ttl_to_parsed_line() {
    let path = Path::new("tests/data/k8p.ttl");

    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let parser = LineParser::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read a line");
        if line.len() == 0 {
            continue;
        };
        test_parsed_line(parser.parse(&line).unwrap());
    }

    fn test_parsed_line(parsed1: ParsedLine) {
        match &parsed1 {
            ParsedLine::Prefix(name, uri) => {
                assert!(name.len() > 0);
                assert!(uri.len() > 0);
            }
            ParsedLine::Subject(ns, s) => {
                assert!(ns.is_some());
                assert!(s.len() > 0);
            }
            ParsedLine::PredObj(ns, p, o) => {
                assert!(ns.is_some());
                assert!(p.len() > 0);
                assert!(
                    p == "k8p_type" || p == "k8p_description" || o.len() > 0,
                    "input: {:?}",
                    parsed1
                );
            }
            ParsedLine::PredObjTerm(ns, p, o) => {
                assert!(ns.is_some());
                assert!(p.len() > 0);
                assert!(
                    p == "k8p_description" || o.len() > 0,
                    "input: {:?}",
                    parsed1
                );
            }
        }
    }
}
