use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::db_api::DbApi;
use triples::turtle_stream::TurtleStream;

const TEST_DB_FILE: &str = "/tmp/csv_import_bricks_test.db";

/// util fixture
async fn load_bricks_from_ttl() {
    let path = Path::new("tests/data/bricks_ex1.ttl");
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TurtleStream::new();

    let _ = fs::remove_file(TEST_DB_FILE);
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

    let subject_names = db_api.get_all_subject_names().await.unwrap();
    assert_eq!(subject_names.len(), 119);
    tx.commit().await.unwrap();
}

#[tokio::test]
async fn test_export_bricks_as_csv() {
    tracing_subscriber::fmt::init();
    load_bricks_from_ttl().await;

    let _db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
}
