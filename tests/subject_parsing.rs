use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::turtle_stream::TurtleStream;

#[test]
fn test_ttl_to_subject() {
    let path = Path::new("tests/data/k8p.ttl");
    let file = File::open(path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TurtleStream::new();

    let max_lines_to_process: usize = 38;

    let mut line_num = 0;
    for line in reader.lines().take(max_lines_to_process) {
        line_num += 1;
        let line = line.expect("Failed to read a line");
        if line.is_empty() {
            continue;
        }
        match stream.load(&line) {
            Ok(r) => {
                match line_num {
                    13 => {
                        assert!(r.is_some());
                        assert_eq!(r.unwrap().name().to_string(), "http://k8p.navicore.tech/resource/84e296b9-af09-4921-ac4c-a9a8fae376a3");
                    }
                    25 => {
                        assert!(r.is_some());
                        assert_eq!(r.unwrap().name().to_string(), "http://k8p.navicore.tech/resource/55a53692-a25f-456b-956f-d17a9124b234");
                    }
                    38 => {
                        assert!(r.is_some());
                        assert_eq!(
                            r.unwrap().name().to_string(),
                            "http://k8p.navicore.tech/resource/6278dd73-66e8-4fd4-8141-33cc022e8e07"
                        );
                    }

                    _ => {
                        assert!(r.is_none());
                    }
                }
            }
            Err(e) => {
                panic!(
                    "error: {} for state {} for input: {}",
                    e,
                    stream.get_state(),
                    line
                );
            }
        }
    }
}
