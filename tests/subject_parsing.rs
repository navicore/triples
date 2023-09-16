use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::ttl_stream::TtlStream;

#[test]
fn test_ttl_to_subject() {
    let path = Path::new("tests/data/k8p.ttl");
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TtlStream::new();

    let max_lines_to_process: usize = 13;

    let mut line_num = 0;
    for line in reader.lines().take(max_lines_to_process) {
        line_num += 1;
        let line = line.expect("Failed to read a line");
        if line.len() == 0 {
            continue;
        }
        match stream.load(&line) {
            Ok(r) => {
                match line_num {
                    13 => {
                        assert!(r.is_some());
                        assert_eq!(r.unwrap().name().to_string(), "http://k8p.navicore.tech/resource/84e296b9-af09-4921-ac4c-a9a8fae376a3");
                    }
                    _ => {
                        assert!(r.is_none());
                    }
                }
            }
            Err(e) => {
                assert!(false, "error: {}", e)
            }
        }
    }
}
