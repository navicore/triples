use crate::data::TriplesError;
/// functions in support of ttl file handling
///
/// Prefer to process data via stdin and stdout to enable *nix style
/// command pipelining.
///
use crate::data::{RdfName, Subject};
use crate::db_api::DbApi;
use crate::turtle_stream::TurtleStream;
use std::collections::{HashMap, HashSet};
use tokio::io::{stdin, AsyncBufReadExt, BufReader}; // assuming you have a Result type alias, adjust as necessary
use tracing::{error, trace};

/// read ttl from stdin and load db
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
pub async fn import_turtle(db_api: &DbApi) -> Result<(), Box<dyn std::error::Error>> {
    trace!("import_turtle");
    let mut stream = TurtleStream::new();
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);

    let tx = db_api.begin_txn().await?;

    let mut line = String::new();
    while reader.read_line(&mut line).await? != 0 {
        import_line(&line, &mut stream, db_api).await?;
        line.clear();
    }

    tx.commit().await?;

    Ok(())
}

/// export ttl to stdout of entire db
///
/// # Errors
///
/// Will return `Err` if any entry can not be marshaled out as valid turtle
pub async fn export_turtle(db_api: &DbApi) -> Result<(), Box<dyn std::error::Error>> {
    trace!("export_turtle");
    let subject_names = db_api.get_subject_names().await?;
    let prefixes = compute_prefixes(&subject_names, db_api).await?;

    print_prefixes(&prefixes);

    for name in &subject_names {
        if let Some(subject) = db_api.query(name).await? {
            let name_string = subject.name().to_string();
            let (ns, local_name) = extract_namespace_and_local_name(&name_string)?;

            if ns.is_empty() && local_name.contains(":/") {
                println!("<{local_name}>");
            } else if ns.is_empty() {
                println!("{local_name}");
            } else if let Some(prefix) = prefixes.get(ns) {
                println!("{prefix}:{local_name}");
            } else {
                let e = TriplesError::UnresolvableURIPrefix {
                    prefix_name: ns.to_string(),
                    name: local_name.to_string(),
                };
                error!("export_turtle no prefix: {e:?}");
                return Err(Box::new(e));
            };

            let pairs: Vec<_> = subject.predicate_object_pairs().collect();
            print_predicate_object_pairs(&pairs, &prefixes)?;
        }
    }

    Ok(())
}

fn extract_namespace_and_local_name(name_string: &str) -> Result<(&str, &str), TriplesError> {
    if let Some(idx) = name_string.rfind('#') {
        let (ns, name) = name_string.split_at(idx + 1);
        if name.contains('/') {
            return Ok(("", name_string));
        }
        return Ok((ns, name));
    } else if let Some(idx) = name_string.rfind('/') {
        let (ns, name) = name_string.split_at(idx + 1);
        return Ok((ns, &name[1..]));
    }

    Err(TriplesError::InvalidIRI {
        uri: name_string.to_string(),
    })
}

/// use statefull stream to build subject objects and as they become
/// complete, insert into db.
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
async fn import_line(
    line: &str,
    stream: &mut TurtleStream,
    db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    trace!("import line");
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

        let meaningful_substring = ns
            .split('/')
            .last()
            .unwrap_or("ns")
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        let mut alias = meaningful_substring
            .chars()
            .take(5)
            .collect::<String>()
            .to_lowercase();

        let mut suffix = 1;
        while prefixes.values().any(|v| v == &alias) {
            alias = format!("{}{}", alias.chars().take(4).collect::<String>(), suffix);
            suffix += 1;
        }

        prefixes.insert(ns.to_string(), alias);
    }

    prefixes
        .get(ns)
        .ok_or_else(|| TriplesError::UnresolvableURIPrefix {
            prefix_name: ns.to_string(),
            name: String::new(),
        })
        .map(String::as_str)
}

fn handle_name_string<'a>(
    name_string: &'a str,
    prefixes: &mut HashMap<String, String>,
    unique_ns_count: &mut u32,
) -> Result<Option<&'a str>, Box<dyn std::error::Error>> {
    trace!("handle_name_string");
    let (ns, _) = if let Some(idx) = name_string.rfind('#') {
        let (ns, _) = name_string.split_at(idx + 1); // +1 to include '#' in ns
        (ns, &name_string[idx + 1..])
    } else if let Some(idx) = name_string.rfind('/') {
        let (ns, _) = name_string.split_at(idx + 1); // +1 to include '/' in ns if needed
        (ns, &name_string[idx + 1..]) // remove '/' from the start of name
    } else {
        let e = TriplesError::InvalidIRI {
            uri: name_string.to_string(),
        };
        error!("handle_name_string: {e}");
        return Err(Box::new(e));
    };

    get_or_insert_prefix(ns, prefixes, unique_ns_count)?;
    Ok(Some(ns))
}

fn handle_subject(
    subject: &Subject,
    prefixes: &mut HashMap<String, String>,
    unique_ns_count: &mut u32,
) -> Result<(), Box<dyn std::error::Error>> {
    if handle_name_string(&subject.name().to_string(), prefixes, unique_ns_count)?.is_none() {
        let e = TriplesError::InvalidIRI {
            uri: subject.name().to_string(),
        };
        error!("handle_subject: {e}");
        return Err(Box::new(e));
    }

    for pair in subject.predicate_object_pairs() {
        if handle_name_string(&pair.0.to_string(), prefixes, unique_ns_count)?.is_none() {
            let e = TriplesError::InvalidIRI {
                uri: pair.0.to_string(),
            };
            error!("handle_subject: {e}");
            return Err(Box::new(e));
        };
        for obj_name in pair.1 {
            if handle_name_string(obj_name, prefixes, unique_ns_count)?.is_none() {
                let e = TriplesError::InvalidIRI {
                    uri: obj_name.to_string(),
                };
                error!("handle_subject: {e}");
                return Err(Box::new(e));
            };
        }
    }

    Ok(())
}

async fn compute_prefixes(
    subject_names: &Vec<RdfName>,
    db_api: &DbApi,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    trace!("compute_prefixes");
    let mut prefixes: HashMap<String, String> = HashMap::new();
    let mut unique_ns_count = 0;

    for name in subject_names {
        if let Some(subject) = db_api.query(name).await? {
            handle_subject(&subject, &mut prefixes, &mut unique_ns_count)?;
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
    pairs: &[(&RdfName, &HashSet<String>)],
    prefixes: &HashMap<String, String>,
) -> Result<(), TriplesError> {
    trace!("print_predicate_object_pairs");
    for (idx, (predicate, objects)) in pairs.iter().enumerate() {
        let name_string = predicate.to_string();

        let (ns, name) = if let Some(idx) = name_string.rfind('#') {
            let (ns, name) = name_string.split_at(idx + 1); // +1 to include '#' in ns
            (ns, name)
        } else if let Some(idx) = name_string.rfind('/') {
            let (ns, name) = name_string.split_at(idx + 1); // +1 to include '/' in ns if needed
            (ns, &name[1..]) // remove '/' from the start of name
        } else {
            let e = TriplesError::InvalidIRI {
                uri: name_string.to_string(),
            };
            error!("print_predicate_object_pairs: {:?}", e);
            return Err(e);
        };

        match prefixes.get(ns) {
            Some(prefix) => {
                let formatted_objects: Vec<String> = objects
                    .iter()
                    .map(|object| {
                        let (ons, objname) = object.rfind('#').map_or_else(
                            || {
                                object.rfind('/').map_or(("", object.as_str()), |idx| {
                                    let (ons, objname) = object.split_at(idx + 1);
                                    let objname = &objname[1..];
                                    (ons, objname)
                                })
                            },
                            |idx| {
                                let (ons, objname) = object.split_at(idx + 1);
                                (ons, objname)
                            },
                        );

                        let fmt_oname = match prefixes.get(ons) {
                            Some(obj_prefix) if !objname.contains('/') => {
                                format!("{obj_prefix}:{objname}")
                            }
                            _ => object.to_string(),
                        };

                        if fmt_oname.contains(":/") {
                            format!("<{fmt_oname}>")
                        } else {
                            format!("\"{fmt_oname}\"")
                        }
                    })
                    .collect();

                // Calculate the dynamic indentation
                let indentation = "    ".len() + prefix.len() + ":".len() + name.len() + 1; // +1 for the space after name
                let spaces = " ".repeat(indentation);

                let object_list = formatted_objects.join(&format!(" ,\n{spaces}"));
                let is_last_pair = idx == pairs.len() - 1;
                if is_last_pair {
                    println!("    {prefix}:{name} {object_list} .\n");
                } else {
                    println!("    {prefix}:{name} {object_list} ;");
                }
            }
            _ => {
                return Err(TriplesError::UnresolvableURIPrefix {
                    prefix_name: ns.to_string(),
                    name: name.to_string(),
                });
            }
        };
    }
    Ok(())
}
