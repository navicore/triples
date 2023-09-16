use clap::Parser;
use std::collections::HashMap;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use triples::data::TriplesError;
use triples::db_api::DbApi;
use triples::ttl_stream::TtlStream;

#[derive(Parser, Debug, Clone)]
enum Command {
    ImportTurtle,
    ExportTurtle,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/tmp/triples.db")]
    db_location: String,

    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let db_api = DbApi::new(args.db_location.clone()).await?;

    match args.command {
        Command::ImportTurtle => import_turtle(&db_api).await?,
        Command::ExportTurtle => export_turtle(&db_api).await?,
    }

    Ok(())
}

async fn import_turtle(db_api: &DbApi) -> Result<(), Box<dyn std::error::Error>> {
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

fn split_last_slash(s: &str) -> Option<(&str, &str)> {
    s.rsplit_once('/')
}

async fn export_turtle(db_api: &DbApi) -> Result<(), Box<dyn std::error::Error>> {
    let subject_names = db_api.query_all_subject_names().await?;

    let mut prefixes = HashMap::new();
    let mut unique_ns_count = 0; // Keep track of the number of unique namespace values
    let mut unique_predicate_count = 0; // Keep track of the number of unique namespace values

    // look up each subject to calculate the prefixes used in ttl files
    for name in &subject_names {
        if let Some(subject) = db_api.query(&name).await? {
            let name_string = subject.name().to_string();
            match split_last_slash(&name_string) {
                Some((ns, name)) => {
                    // Check if the ns is already present in prefixes. If not, add it with a new alias.
                    if !prefixes.contains_key(ns) {
                        unique_ns_count += 1; // Increment the count
                        let alias = format!("ns{}", unique_ns_count); // Generate the alias using the count
                        prefixes.insert(ns.to_string(), alias); // Add the new ns and its alias to the map
                    }
                }
                None => return Err(Box::new(TriplesError::InvalidIRI)),
            };
            // inspect each predicate/object pair to calculate the prefixes used in ttl files
            for pair in subject.predicate_object_pairs() {
                let name_string = pair.0.to_string();
                match split_last_slash(&name_string) {
                    Some((ns, name)) => {
                        // Check if the ns is already present in prefixes. If not, add it with a new alias.
                        if !prefixes.contains_key(ns) {
                            unique_predicate_count += 1; // Increment the count
                            let alias = format!("ref{}", unique_predicate_count); // Generate the alias using the count
                            prefixes.insert(ns.to_string(), alias); // Add the new ns and its alias to the map
                        }
                    }
                    None => return Err(Box::new(TriplesError::InvalidIRI)),
                };
            }
        }
    }

    // print the prefixes
    for (name, prefix) in &prefixes {
        println!("@prefix {prefix}: <{name}/> .\n");
    }

    // print the ttl
    for name in &subject_names {
        if let Some(subject) = db_api.query(&name).await? {
            let name_string = subject.name().to_string();
            match split_last_slash(&name_string) {
                Some((ns, name)) => match prefixes.get(ns) {
                    Some(prefix) => {
                        println!("{prefix}:{name}");
                    }
                    _ => return Err(Box::new(TriplesError::InvalidIRI)),
                },
                None => return Err(Box::new(TriplesError::InvalidIRI)),
            };
            // inspect each predicate/object pair to calculate the prefixes used in ttl files

            let mut pairs = subject.predicate_object_pairs().peekable();

            while let Some(pair) = pairs.next() {
                let name_string = pair.0.to_string();
                match split_last_slash(&name_string) {
                    Some((ns, name)) => {
                        // print the predicate / object pair
                        match prefixes.get(ns) {
                            Some(prefix) => {
                                if pairs.peek().is_none() {
                                    println!(
                                        "    {}:{} \"{}\" .\n",
                                        prefix,
                                        name,
                                        pair.1.to_string()
                                    );
                                } else {
                                    println!(
                                        "    {}:{} \"{}\" ;",
                                        prefix,
                                        name,
                                        pair.1.to_string()
                                    );
                                }
                            }
                            _ => return Err(Box::new(TriplesError::InvalidIRI)),
                        }
                    }
                    None => return Err(Box::new(TriplesError::InvalidIRI)),
                };
            }
        }
    }

    Ok(())
}
