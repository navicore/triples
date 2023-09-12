use crate::data::Subject;
use sqlx::Pool;
use sqlx::Sqlite;
use tracing::debug;

// #[cfg(feature = "postgres")]
// use sqlx::Postgres;

use crate::db;

pub struct DbApi {
    pool: Pool<Sqlite>,
}

impl DbApi {
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

    async fn insert_name_and_get_id(&self, name: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let pool = &self.pool;

        // Insert the name if it doesn't exist
        sqlx::query(
            r#"
            INSERT INTO names (name) VALUES (?1)
            ON CONFLICT(name) DO NOTHING
            "#,
        )
        .bind(name)
        .execute(pool)
        .await?;

        // Fetch and return the ID of the name
        let id: i64 = sqlx::query_scalar(
            r#"
            SELECT id FROM names WHERE name = ?1
            "#,
        )
        .bind(name)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    async fn insert_triple(
        &self,
        subject_id: i64,
        predicate_id: i64,
        object: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pool = &self.pool;

        sqlx::query(
            r#"
            INSERT INTO triples (subject, predicate, object) VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(subject_id)
        .bind(predicate_id)
        .bind(object)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Inserts a given RDF subject into the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if insertion cannot be performed.
    pub async fn insert(&self, subject: &Subject) -> Result<(), Box<dyn std::error::Error>> {
        let pool = &self.pool;

        let tx = pool.begin().await?;

        let fetched_subject_id = self
            .insert_name_and_get_id(&subject.name().to_string())
            .await?;

        for (predicate, object) in subject.predicates_objects() {
            let fetched_predicate_id = self.insert_name_and_get_id(&predicate.to_string()).await?;
            self.insert_triple(fetched_subject_id, fetched_predicate_id, object)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    /// Queries data from the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the data cannot be queried from the database.
    pub fn query(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement the query logic
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use glob::glob;
    use std::fs;

    const TEST_DB_FILE: &str = "/tmp/triples_unit_test.db";

    fn delete_test_db() {
        for entry in glob(&format!("{TEST_DB_FILE}*")).unwrap() {
            let path = entry.unwrap();
            fs::remove_file(path).unwrap();
        }
    }

    #[tokio::test]
    async fn test_insert() {
        delete_test_db();
        // Create an in-memory SQLite database for testing purposes
        let db_api = DbApi::new(TEST_DB_FILE.to_string()).await.unwrap();
        // Create a sample Subject
        let subject_iri = "https://www.example.com/subject".to_string();
        let subject_name = crate::data::RdfName::new(subject_iri.clone()).unwrap();

        let predicate_1_iri = "https://www.example.com/predicate1".to_string();
        let predicate_1_name = crate::data::RdfName::new(predicate_1_iri.clone()).unwrap();
        let object_1_value = "Object Value 1".to_string();

        let predicate_2_iri = "https://www.example.com/predicate2".to_string();
        let predicate_2_name = crate::data::RdfName::new(predicate_2_iri.clone()).unwrap();
        let object_2_value = "Object Value 2".to_string();

        let mut subject = crate::data::Subject::new(subject_name);
        subject.add(predicate_1_name, object_1_value.clone());
        subject.add(predicate_2_name, object_2_value.clone());

        // Use the insert function
        db_api.insert(&subject).await.expect("Insert failed");

        // Fetch all matching rows
        let results: Vec<(String, String, String)> = sqlx::query_as(
            r#"
            SELECT subjects.name, predicates.name, triples.object
            FROM triples
            JOIN names AS subjects ON triples.subject = subjects.id
            JOIN names AS predicates ON triples.predicate = predicates.id
            WHERE subjects.name = ?1
            ORDER BY predicates.name
            "#,
        )
        .bind(&subject_iri)
        .fetch_all(&db_api.pool)
        .await
        .expect("Failed to fetch from test DB");

        // Check that there are 2 rows
        assert_eq!(results.len(), 2);

        // Assert the values of the first row
        assert_eq!(results[0].0, subject_iri);
        assert_eq!(results[0].1, predicate_1_iri);
        assert_eq!(results[0].2, object_1_value);

        // Assert the values of the second row
        assert_eq!(results[1].0, subject_iri);
        assert_eq!(results[1].1, predicate_2_iri);
        assert_eq!(results[1].2, object_2_value);
    }

    #[tokio::test]
    async fn test_query() {
        // TODO: Implement the test for the query method
    }
}
