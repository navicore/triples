use clap::Parser;
use triples::csv_file;
use triples::db_api::DbApi;
use triples::ttl_file;

#[derive(Parser, Debug, Clone)]
enum Command {
    ImportTurtle,
    ExportTurtle,
    ImportCSV(ImportCsvArgs),
    ExportCSV(ExportCsvArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/tmp/triples.db")]
    db_location: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug, Clone)]
struct ImportCsvArgs {
    #[arg(long)]
    subject_default_ns: Option<String>,

    #[arg(long)]
    predicate_default_ns: Option<String>,

    #[arg(long, default_value = "false")]
    skip_headers: bool,
}

#[derive(Parser, Debug, Clone)]
struct ExportCsvArgs {
    #[arg(long, default_value = "false")]
    export_ns_name: bool,

    #[arg(long, default_value = "false")]
    export_headers: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let db_api = DbApi::new(args.db_location).await?;

    match args.command {
        Command::ImportTurtle => ttl_file::import_turtle(&db_api).await?,
        Command::ExportTurtle => ttl_file::export_turtle(&db_api).await?,
        Command::ImportCSV(import_csv_args) => {
            csv_file::import_csv(
                import_csv_args.subject_default_ns,
                import_csv_args.predicate_default_ns,
                import_csv_args.skip_headers,
                &db_api,
            )
            .await?;
        }
        Command::ExportCSV(export_csv_args) => {
            csv_file::export_csv(
                export_csv_args.export_ns_name,
                export_csv_args.export_headers,
                &db_api,
            )
            .await?;
        }
    }

    Ok(())
}
