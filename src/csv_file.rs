use crate::csv::get_display_name;
use crate::data::RdfName;
use crate::db_api::DbApi;
use std::collections::HashMap;
use tracing::error;

/// write csv format to stdout all db entries
///
/// # Errors
///
/// return `Err` on db read errors
pub async fn export_csv(
    export_ns_name: bool,
    subject_column_name: Option<String>,
    db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject_names: Vec<RdfName> = db_api.get_subject_names().await?;
    let mut headers_map: HashMap<RdfName, usize> = HashMap::new();

    // First pass: determine headers
    for name in &subject_names {
        if let Some(subject) = db_api.query(name).await? {
            for (predicate, objects) in subject.predicate_object_pairs() {
                let count = headers_map.entry(predicate.clone()).or_insert(0);
                *count = (*count).max(objects.len());
            }
        }
    }

    let mut headers = vec![subject_column_name.unwrap_or_else(|| "subject".to_string())];
    for (predicate, max_count) in &headers_map {
        let predicate_display_name = get_display_name(predicate, export_ns_name)?;
        for i in 0..*max_count {
            if *max_count > 1 {
                headers.push(format!("{}{}", predicate_display_name, i + 1)); // for example: pred1, pred2, ...
            } else {
                headers.push(predicate_display_name.clone());
            }
        }
    }

    // Write out header to stdout
    println!("{}", headers.join(","));

    // Second pass: write out rows for each subject
    for name in &subject_names {
        if let Some(subject) = db_api.query(name).await? {
            let subject_display_name = get_display_name(subject.name(), export_ns_name)?;
            let mut line = vec![subject_display_name];
            for (predicate, max_count) in &headers_map {
                //let predicate_display_name = get_display_name(predicate, export_ns_name)?;
                if let Some(objects) = subject.get(predicate) {
                    let mut objects_vec: Vec<_> = objects.iter().collect();
                    objects_vec.sort(); // sort to keep order consistent
                    for i in 0..*max_count {
                        if i < objects_vec.len() {
                            line.push(objects_vec[i].clone());
                        } else {
                            line.push(String::new()); // fill with empty strings for missing values
                        }
                    }
                } else {
                    for _ in 0..*max_count {
                        line.push(String::new()); // fill with empty strings for missing predicates
                    }
                }
            }

            // Write out row to stdout
            println!("{}", line.join(","));
        }
    }

    Ok(())
}

/// read csv from stdin and load db
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
pub fn import_csv(
    _default_subject_ns: &Option<String>,
    _subject_pos: i32,
    _default_predicate_ns: &Option<String>,
    _db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    error!("not implemented");
    Ok(())
}
