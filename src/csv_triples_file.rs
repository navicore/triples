use crate::csv;
/// Functions in support of csv file handling.
///
/// Prefer to process data via stdin and stdout to enable *nix style
/// command pipelining.
///
use crate::data::RdfName;
use crate::data::Subject;
use crate::db_api::DbApi;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

fn print_csv(subject: &str, predicate: &str, object: &str) {
    let sanitized_object = csv::sanitize_csv_field(object);
    println!("{subject},{predicate},{sanitized_object}");
}

/// write csv format to stdout all db entries
///
/// # Errors
///
/// return `Err` on db read errors
pub async fn export_csv(
    export_ns_name: bool,
    export_headers: bool,
    db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    if export_headers {
        print_csv("Subject", "Predicate", "Object");
    };

    let subject_names = db_api.get_subject_names().await?;

    for name in &subject_names {
        if let Some(subject) = db_api.query(name).await? {
            let subject_rdf_name = subject.name();
            let rdf_sub_name = csv::get_display_name(subject_rdf_name, export_ns_name)?;

            for (predicate, objects) in subject.predicate_object_pairs() {
                let rdf_predicate_name = csv::get_display_name(predicate, export_ns_name)?;

                for object in objects {
                    // Print the CSV line for each object associated with the subject-predicate pair
                    print_csv(&rdf_sub_name, &rdf_predicate_name, object);
                }
            }
        }
    }

    Ok(())
}
fn parse_csv_line(line: &str) -> Result<(String, String, String), &'static str> {
    let mut components = line.splitn(3, ',');

    let subject = components
        .next()
        .ok_or("Missing subject")?
        .trim()
        .to_string();
    let predicate = components
        .next()
        .ok_or("Missing predicate")?
        .trim()
        .to_string();
    let object = components
        .next()
        .ok_or("Missing object")?
        .trim()
        .to_string();

    if components.next().is_some() {
        Err("Too many components in the CSV line")
    } else {
        Ok((subject, predicate, object))
    }
}

/// read csv from stdin and load db
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
pub async fn import_csv(
    default_subject_ns: Option<String>,
    default_predicate_ns: Option<String>,
    skip_headers: bool,
    db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);

    let mut discard = String::new();
    if skip_headers {
        reader.read_line(&mut discard).await?;
    };

    let tx = db_api.begin_txn().await?;

    let mut line = String::new();
    while reader.read_line(&mut line).await? != 0 {
        let (subject, predicate, object) = parse_csv_line(&line)?;
        let rdf_sub_name = if let Some(ns) = &default_subject_ns {
            format!("{ns}/{subject}")
        } else {
            subject
        };
        let rdf_predicate_name = if let Some(ns) = &default_predicate_ns {
            format!("{ns}/{predicate}")
        } else {
            predicate
        };

        let mut subject_entry = Subject::new(RdfName::new(rdf_sub_name.to_string()));
        subject_entry.add(RdfName::new(rdf_predicate_name), object);

        db_api.insert(&subject_entry).await?;
        line.clear();
    }
    tx.commit().await?;

    Ok(())
}
