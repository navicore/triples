use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use triples::db_api::DbApi;
use triples::turtle_stream::TurtleStream;

#[tokio::test]
async fn test_ttl_to_db() {
    let path = Path::new("tests/data/k8p_sm.ttl");

    let file = File::open(path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TurtleStream::new();

    const TEST_DB_FILE: &str = "/tmp/triples_batch_load_test.db";
    let _ = fs::remove_file(TEST_DB_FILE);
    let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
    let tx = db_api.begin_txn().await.unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read a line");
        if line.is_empty() {
            continue;
        }
        match stream.load(&line) {
            Ok(r) => {
                if let Some(subject) = r {
                    db_api.insert(&subject).await.expect("Insert failed");
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
    tx.commit().await.unwrap();

    let subject_names = db_api.get_subject_names().await.unwrap();
    assert_eq!(subject_names.len(), 33);

    let first_name = subject_names.first().unwrap();
    assert_eq!(
        first_name.to_string(),
        "http://k8p.navicore.tech/resource/0604c9f2-a656-4384-ab8d-7291ac60dd34"
    );
    let first_subject = db_api.query(first_name).await.unwrap();
    assert!(first_subject.is_some());
    let first_subject = first_subject.unwrap();

    let mut pairs: Vec<(_, _)> = first_subject.predicate_object_pairs().collect();
    pairs.sort_by(|(p1, _), (p2, _)| p1.cmp(p2));

    if let Some((predicate, objects)) = pairs.into_iter().next() {
        assert_eq!(
            predicate.to_string(),
            "http://k8p.navicore.tech/property/k8p_appname"
        );
        assert_eq!(objects.len(), 1);
    }
}

#[tokio::test]
async fn test_bricks_to_db() {
    tracing_subscriber::fmt::init();
    let path = Path::new("tests/data/bricks_ex1.ttl");
    let file = File::open(path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut stream = TurtleStream::new();

    const TEST_DB_FILE: &str = "/tmp/triples_bricks_load_test.db";
    let _ = fs::remove_file(TEST_DB_FILE);
    let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
    let tx = db_api.begin_txn().await.unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read a line");
        if line.is_empty() {
            continue;
        }
        match stream.load(&line) {
            Ok(r) => {
                if let Some(subject) = r {
                    db_api.insert(&subject).await.expect("Insert failed");
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

    let subject_names = db_api.get_subject_names().await.unwrap();
    assert_eq!(subject_names.len(), 119);
    tx.commit().await.unwrap();
}
