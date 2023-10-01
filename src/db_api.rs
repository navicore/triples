use crate::data::RdfName;
use crate::data::Subject;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::Transaction;
use tracing::debug;

// #[cfg(feature = "postgres")]
// use sqlx::Postgres;

use crate::db;

#[must_use]
pub fn str_to_string(s: &str) -> String {
    s.to_string()
}

pub struct DbApi {
    pool: Pool<Sqlite>,
}

impl<'a> DbApi {
    /// Constructs a new instance of `DbApi` and initializes the pool.
    ///
    /// # Errors
    ///
    /// Will return `Err` if db can not be initialized
    pub async fn new(db_location: String) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = db::init(db_location.clone()).await?;
        debug!("db {db_location} initialized");
        Ok(Self { pool })
    }

    /// # Errors
    ///
    /// Will return `Err` if db cannot start a transaction
    #[cfg(all(feature = "sqlite", not(feature = "disable-sqlite")))]
    pub async fn begin_txn(&self) -> Result<Transaction<Sqlite>, Box<dyn std::error::Error>> {
        let pool = &self.pool;

        let tx: Transaction<Sqlite> = pool.begin().await?;

        Ok(tx)
    }

    async fn get_or_insert_name(&self, name: &str) -> Result<i64, sqlx::Error> {
        let pool = &self.pool;
        let query = "
        -- Try to insert the item
        INSERT OR IGNORE INTO names (name) VALUES (?);

        -- Get the ID of the item, either the one just inserted or the existing one
        SELECT id FROM names WHERE name = ?;
        ";

        let row: (i64,) = sqlx::query_as(query)
            .bind(name)
            .bind(name)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }

    async fn get_or_insert_object(&self, object: &str) -> Result<i64, sqlx::Error> {
        let pool = &self.pool;
        let query = "
        -- Try to insert the item
        INSERT OR IGNORE INTO objects (object) VALUES (?);

        -- Get the ID of the item, either the one just inserted or the existing one
        SELECT id FROM objects WHERE object = ?;
    ";

        let row: (i64,) = sqlx::query_as(query)
            .bind(object)
            .bind(object)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }

    async fn insert_triple(
        &self,
        subject_id: i64,
        predicate_id: i64,
        object_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
        INSERT INTO triples (subject, predicate, object) VALUES (?1, ?2, ?3)
        "#,
        )
        .bind(subject_id)
        .bind(predicate_id)
        .bind(object_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Inserts a given RDF subject into the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if insertion cannot be performed.
    pub async fn insert(&self, subject: &Subject) -> Result<(), Box<dyn std::error::Error>> {
        let fetched_subject_id = self.get_or_insert_name(&subject.name().to_string()).await?;

        for (predicate, objects) in subject.predicate_object_pairs() {
            let fetched_predicate_id = self.get_or_insert_name(&predicate.to_string()).await?;

            for object in objects {
                let fetched_object_id = self.get_or_insert_object(object).await?;
                self.insert_triple(fetched_subject_id, fetched_predicate_id, fetched_object_id)
                    .await?;
            }
        }

        Ok(())
    }
    /// Queries data from the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the data cannot be queried from the database.
    pub async fn query(
        &self,
        subject_name: &RdfName,
    ) -> Result<Option<Subject>, Box<dyn std::error::Error>> {
        let pool = &self.pool;

        // Use the provided subject name to query the database for all predicate/object pairs
        let results: Vec<(String, String)> = sqlx::query_as(
            r#"
        SELECT predicates.name, objects.object
        FROM triples
        JOIN names AS subjects ON triples.subject = subjects.id
        JOIN names AS predicates ON triples.predicate = predicates.id
        JOIN objects AS objects ON triples.object = objects.id
        WHERE subjects.name = ?1
        "#,
        )
        .bind(subject_name.to_string())
        .fetch_all(pool)
        .await?;

        // If no results are found, return Ok(None)
        if results.is_empty() {
            return Ok(None);
        }

        // Create a Subject object using the provided subject name
        let mut subject = crate::data::Subject::new(subject_name.clone());

        // Add all the predicate/object pairs to the Subject object
        for (predicate_iri, object_value) in results {
            let predicate_name = crate::data::RdfName::new(predicate_iri);
            subject.add(predicate_name, object_value);
        }

        Ok(Some(subject))
    }

    /// Queries all distinct subject names from the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the data cannot be queried from the database or if any invalid IRI is encountered.
    pub async fn query_all_subject_names(
        &self,
    ) -> Result<Vec<RdfName>, Box<dyn std::error::Error>> {
        let pool = &self.pool;

        let names_strings: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT subjects.name
            FROM triples
            JOIN names AS subjects ON triples.subject = subjects.id
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut names_rdf = Vec::new();

        for name_str in names_strings {
            let name = RdfName::new(name_str);

            names_rdf.push(name);
        }

        Ok(names_rdf)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use glob::glob;
    use std::fs;

    const TEST_DB_FILE: &str = "/tmp/triples_unit_test.db";
    const TEST_DB_FILE_2: &str = "/tmp/triples_unit_test_2.db";

    fn delete_test_db(file: &str) {
        for entry in glob(&format!("{file}*")).unwrap() {
            let path = entry.unwrap();
            fs::remove_file(path).unwrap();
        }
    }

    fn create_test_subject() -> Subject {
        let subject_iri = "https://www.example.com/subject".to_string();
        let subject_name = crate::data::RdfName::new(subject_iri.clone());

        let predicate_1_iri = "https://www.example.com/predicate1".to_string();
        let predicate_1_name = crate::data::RdfName::new(predicate_1_iri.clone());
        let object_1_value = "Object Value 1".to_string();

        let predicate_2_iri = "https://www.example.com/predicate2".to_string();
        let predicate_2_name = crate::data::RdfName::new(predicate_2_iri.clone());
        let object_2_value = "Object Value 2".to_string();

        let mut subject = crate::data::Subject::new(subject_name);
        subject.add(predicate_1_name, object_1_value.clone());
        subject.add(predicate_2_name, object_2_value.clone());

        subject
    }

    #[tokio::test]
    async fn test_insert() {
        delete_test_db(TEST_DB_FILE);
        // Create an in-memory SQLite database for testing purposes
        let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
        let subject = create_test_subject();

        // Use the insert function
        db_api.insert(&subject).await.expect("Insert failed");

        // Fetch all matching rows
        let results: Vec<(String, String, String)> = sqlx::query_as(
            r#"
            SELECT subjects.name, predicates.name, objects.object
            FROM triples
            JOIN names AS subjects ON triples.subject = subjects.id
            JOIN names AS predicates ON triples.predicate = predicates.id
            JOIN objects AS objects ON triples.object = objects.id
            WHERE subjects.name = ?1
            ORDER BY predicates.name
            "#,
        )
        .bind(&subject.name().to_string())
        .fetch_all(&db_api.pool)
        .await
        .expect("Failed to fetch from test DB");

        // Check that there are 2 rows
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_query() {
        delete_test_db(TEST_DB_FILE_2);
        // Create an in-memory SQLite database for testing purposes
        let db_api = DbApi::new(TEST_DB_FILE_2.to_string()).await.unwrap();
        let subject = create_test_subject();

        // Use the insert function
        db_api.insert(&subject).await.expect("Insert failed");

        // Query the data back from the database
        let queried_subject = db_api
            .query(subject.name())
            .await
            .expect("Query failed")
            .expect("Subject not found in the database");

        // Validate the results
        assert_eq!(
            subject.name().to_string(),
            queried_subject.name().to_string()
        );

        let original_predicates: Vec<_> = subject.all_predicates().collect();
        let queried_predicates: Vec<_> = queried_subject.all_predicates().collect();

        // First, ensure that the predicates are the same len
        assert_eq!(original_predicates.len(), queried_predicates.len());

        // Now compare the associated objects using the predicate keys
        for predicate in original_predicates {
            let original_object = subject.get(predicate).unwrap();
            let queried_object = queried_subject.get(predicate).unwrap();
            assert_eq!(original_object, queried_object);
        }
    }
}
