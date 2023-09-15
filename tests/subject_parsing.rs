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

    let max_lines_to_process: usize = 5; // Change this value as your test gets better

    for line in reader.lines().take(max_lines_to_process) {
        let line = line.expect("Failed to read a line");
        if line.len() == 0 {
            continue;
        }
        match stream.load(&line) {
            Ok(r) => {
                assert!(r.is_none());
            }
            Err(e) => {
                assert!(false, "error: {}", e)
            }
        }
    }
}
