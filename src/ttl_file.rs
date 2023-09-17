/// functions in support of ttl file handling
///
/// Prefer to process data via stdin and stdout to enable *nix style
/// command pipelining.
///
use crate::data::RdfName;
use crate::data::TriplesError;
use crate::db_api::DbApi;
use crate::ttl_stream::TtlStream;
use std::collections::HashMap;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

/// read ttl from stdin and load db
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
pub async fn import_turtle(db_api: &DbApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TtlStream::new();
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);

    let mut line = String::new();
    while reader.read_line(&mut line).await? != 0 {
        import_line(&line, &mut stream, db_api).await?;
        line.clear();
    }

    Ok(())
}

/// export ttl to stdout of entire db
///
/// # Errors
///
/// Will return `Err` if any entry can not be marshaled out as valid turtle
pub async fn export_turtle(db_api: &DbApi) -> Result<(), Box<dyn std::error::Error>> {
    let subject_names = db_api.query_all_subject_names().await?;
    let prefixes = compute_prefixes(&subject_names, db_api).await?;

    print_prefixes(&prefixes);

    // print each subject with lines for each predicate / object pair
    // terminating each subject block with a '.' and a newline.
    for name in &subject_names {
        if let Some(subject) = db_api.query(name).await? {
            let name_string = subject.name().to_string();

            match name_string.rsplit_once('/') {
                Some((ns, name)) => match prefixes.get(ns) {
                    Some(prefix) => {
                        println!("{prefix}:{name}");
                    }
                    _ => {
                        return Err(Box::new(TriplesError::InvalidIRI {
                            uri: name.to_string(),
                        }))
                    }
                },
                None => {
                    return Err(Box::new(TriplesError::InvalidIRI {
                        uri: name.to_string(),
                    }))
                }
            };
            let pairs: Vec<_> = subject.predicate_object_pairs().collect();
            print_predicate_object_pairs(&pairs, &prefixes)?;
        }
    }

    Ok(())
}

/// use statefull stream to build subject objects and as they become
/// complete, insert into db.
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
async fn import_line(
    line: &str,
    stream: &mut TtlStream,
    db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    if line.trim().is_empty() {
        return Ok(());
    }

    match stream.load(line) {
        Ok(Some(subject)) => db_api.insert(&subject).await?,
        Ok(None) => {} // still loading predicate/object pairs - keep reading stream
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}

// This utility function returns the prefix for a given namespace string.
// It updates the prefixes map and unique_count if the namespace is not already present.
fn get_or_insert_prefix<'a>(
    ns: &'a str,
    prefixes: &'a mut HashMap<String, String>,
    unique_count: &mut u32,
) -> Result<&'a str, TriplesError> {
    if !prefixes.contains_key(ns) {
        *unique_count += 1;
        let alias = format!("ns{unique_count}");
        prefixes.insert(ns.to_string(), alias);
    }
    Ok(prefixes
        .get(ns)
        .ok_or(TriplesError::UnresolvableURIPrefix {
            prefix_name: ns.to_string(),
        })?
        .as_str())
}

fn handle_name_string<'a>(
    name_string: &'a str,
    prefixes: &mut HashMap<String, String>,
    unique_ns_count: &mut u32,
) -> Result<Option<&'a str>, Box<dyn std::error::Error>> {
    match name_string.rsplit_once('/') {
        Some((ns, _)) => {
            get_or_insert_prefix(ns, prefixes, unique_ns_count)?;
            Ok(Some(ns))
        }
        None => Err(Box::new(TriplesError::InvalidIRI {
            uri: name_string.to_string(),
        })),
    }
}

async fn compute_prefixes(
    subject_names: &Vec<RdfName>,
    db_api: &DbApi,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut prefixes: HashMap<String, String> = HashMap::new();
    let mut unique_ns_count = 0;

    for name in subject_names {
        if let Some(subject) = db_api.query(name).await? {
            if handle_name_string(
                &subject.name().to_string(),
                &mut prefixes,
                &mut unique_ns_count,
            )?
            .is_none()
            {
                return Err(Box::new(TriplesError::InvalidIRI {
                    uri: name.to_string(),
                }));
            }
            for pair in subject.predicate_object_pairs() {
                if handle_name_string(&pair.0.to_string(), &mut prefixes, &mut unique_ns_count)?
                    .is_none()
                {
                    return Err(Box::new(TriplesError::InvalidIRI {
                        uri: name.to_string(),
                    }));
                }
            }
        }
    }

    Ok(prefixes)
}

fn print_prefixes(prefixes: &HashMap<String, String>) {
    for (name, prefix) in prefixes {
        println!("@prefix {prefix}: <{name}/> .\n");
    }
}

fn print_predicate_object_pairs(
    pairs: &[(&RdfName, &String)],
    prefixes: &HashMap<String, String>,
) -> Result<(), TriplesError> {
    let mut pairs_iter = pairs.iter().peekable();
    while let Some((predicate, object)) = pairs_iter.next() {
        let name_string = predicate.to_string();

        match name_string.rsplit_once('/') {
            Some((ns, name)) => match prefixes.get(ns) {
                Some(prefix) => {
                    if pairs_iter.peek().is_none() {
                        println!("    {prefix}:{name} \"{object}\" ; .\n");
                    } else {
                        println!("    {prefix}:{name} \"{object}\" ;");
                    }
                }
                _ => {
                    return Err(TriplesError::UnresolvableURIPrefix {
                        prefix_name: ns.to_string(),
                    })
                }
            },
            None => {
                return Err(TriplesError::InvalidIRI {
                    uri: name_string.to_string(),
                })
            }
        };
    }
    Ok(())
}
