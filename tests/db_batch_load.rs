use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::db_api::DbApi;
use triples::turtle_stream::TurtleStream;

#[tokio::test]
async fn test_ttl_to_db() {
    let path = Path::new("tests/data/k8p_sm.ttl");
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TurtleStream::new();

    const TEST_DB_FILE: &str = "/tmp/triples_batch_load_test.db";
    let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
    let tx = db_api.begin_txn().await.unwrap();

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
                assert!(
                    false,
                    "error: {} for state {} for input: {}",
                    e,
                    stream.get_state(),
                    line
                )
            }
        }
    }

    let subject_names = db_api.query_all_subject_names().await.unwrap();
    assert_eq!(subject_names.len(), 33);
    tx.commit().await.unwrap();
}

#[tokio::test]
async fn test_bricks_to_db() {
    let path = Path::new("tests/data/bricks_ex1.ttl");
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TurtleStream::new();

    const TEST_DB_FILE: &str = "/tmp/triples_bricks_load_test.db";
    let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
    let tx = db_api.begin_txn().await.unwrap();

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
                assert!(false, "error: {} on input: {}", e, line)
            }
        }
    }

    let subject_names = db_api.query_all_subject_names().await.unwrap();
    assert_eq!(subject_names.len(), 119);
    tx.commit().await.unwrap();
}
