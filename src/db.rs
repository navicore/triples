// #[cfg(feature = "postgres")]
// use anyhow::Error;
use sqlx::Pool;
use sqlx::Sqlite;
use std::fs::File;
use tracing::debug;

/// # Errors
///
/// Will return `Err` if function cannot create db table
#[cfg(all(feature = "sqlite", not(feature = "disable-sqlite")))]
async fn create_names_table(pool: &Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS names (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_names ON names (name);
        "#,
    )
    .execute(pool)
    .await?;

    debug!("db names table initialized");
    Ok(())
}

/// # Errors
///
/// Will return `Err` if function cannot create db table
#[cfg(all(feature = "sqlite", not(feature = "disable-sqlite")))]
async fn create_triples_table(pool: &Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS triples (
            id INTEGER PRIMARY KEY,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            subject INTEGER NOT NULL,
            predicate INTEGER NOT NULL,
            object TEXT NOT NULL,
            FOREIGN KEY (subject) REFERENCES names(id),
            FOREIGN KEY (predicate) REFERENCES names(id)
        );

        CREATE INDEX IF NOT EXISTS idx_subject ON triples (subject);
        CREATE INDEX IF NOT EXISTS idx_predicate ON triples (predicate);
        CREATE INDEX IF NOT EXISTS idx_object ON triples (object);
        "#,
    )
    .execute(pool)
    .await?;

    debug!("db triples table initialized");
    Ok(())
}

/// # Errors
///
/// Will return `Err` if function cannot init db file
#[cfg(all(feature = "sqlite", not(feature = "disable-sqlite")))]
pub async fn init(db_location: String) -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    let db_url = format!("sqlite:{db_location}");

    let db_path = std::path::Path::new(&db_location);
    if db_path.exists() {
        debug!("adding to db {}", db_url);
    } else {
        debug!("creating db {}", db_url);
        File::create(&db_location)?;
    }

    let pool = Pool::connect(&db_url).await?;

    create_names_table(&pool).await?;

    create_triples_table(&pool).await?;

    Ok(pool)
}

// /// # Errors
// ///
// /// Will return `Err` if function cannot init db server connection
// #[cfg(feature = "postgres")]
// pub async fn init(db_location: String) -> Result<Pool<sqlx::Postgres>, Error> {
//     Err(Error::msg(
//         "Postgres initialization is not yet implemented.",
//     ))
// }
//
// /// # Errors
// ///
// /// Will return `Err` if function cannot create table
// #[cfg(feature = "postgres")]
// async fn create_triples_table(pool: &Pool<sqlx::Postgres>) -> Result<(), Error> {
//     Err(Error::msg(
//         "Postgres initialization is not yet implemented.",
//     ))
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tokio::runtime::Runtime;

    #[test]
    fn test_init() {
        let db_location = "/tmp/test_triples.db";

        // Ensure there's no db file before the test
        let _ = fs::remove_file(db_location);

        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let pool = init(db_location.to_string()).await.unwrap();

            // Check if the names table has been created
            let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM names")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(row.0, 0);

            // Check if the triples table has been created
            let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM triples")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(row.0, 0);
        });

        // Clean up after the test
        let _ = fs::remove_file(db_location);
    }
}
