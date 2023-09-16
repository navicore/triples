use clap::Parser;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
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
        Command::ExportTurtle => export_turtle(),
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
        Ok(None) => {}
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}

const fn export_turtle() {
    // TODO: dump to stdout
}
