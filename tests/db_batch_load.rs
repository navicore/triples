use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::db_api::DbApi;
use triples::ttl_stream::TtlStream;

#[tokio::test]
async fn test_ttl_to_db() {
    let path = Path::new("tests/data/k8p_sm.ttl");
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TtlStream::new();

    const TEST_DB_FILE: &str = "/tmp/triples_batch_load_test.db";
    let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read a line");
        if line.len() == 0 {
            continue;
        }
        match stream.load(&line) {
            Ok(r) => {
                match r {
                    Some(subject) => {
                        // insert into db
                        db_api.insert(&subject).await.expect("Insert failed");
                    }
                    _ => {} // noop
                };
            }
            Err(e) => {
                assert!(false, "error: {}", e)
            }
        }
    }
}
