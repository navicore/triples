use crate::data::RdfName;
use crate::data::Subject;
/// Functions in support of csv file handling.
///
/// Prefer to process data via stdin and stdout to enable *nix style
/// command pipelining.
///
use crate::data::TriplesError;
use crate::db_api::DbApi;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

// Utility function to determine the display name based on the strip_ns flag
fn get_display_name(name_string: &str, export_ns_name: bool) -> Result<&str, TriplesError> {
    if export_ns_name {
        Ok(name_string)
    } else {
        name_string
            .rsplit_once('/')
            .map(|(_, name)| name)
            .ok_or(TriplesError::InvalidIRI {
                uri: name_string.to_string(),
            })
    }
}

fn sanitize_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('\n') || field.contains('"') {
        // Escape any double quotes and surround the whole field with double quotes
        format!("\"{}\"", field.replace('\"', "\"\""))
    } else {
        field.to_string()
    }
}

fn print_csv(subject: &str, predicate: &str, object: &str) {
    let sanitized_object = sanitize_csv_field(object);
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

    let subject_names = db_api.query_all_subject_names().await?;

    for name in &subject_names {
        if let Some(subject) = db_api.query(name).await? {
            let subject_str = subject.name().to_string();
            let rdf_sub_name = get_display_name(&subject_str, export_ns_name)?;

            for pair in subject.predicate_object_pairs() {
                let predicate_str = pair.0.to_string();
                let rdf_predicate_name = get_display_name(&predicate_str, export_ns_name)?;

                // Print the CSV line
                print_csv(rdf_sub_name, rdf_predicate_name, pair.1);
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

        let mut subject_entry = Subject::new(RdfName::new(rdf_sub_name.to_string())?);
        subject_entry.add(RdfName::new(rdf_predicate_name)?, object);

        db_api.insert(&subject_entry).await?;
        line.clear();
    }

    Ok(())
}
