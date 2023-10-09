/// Functions in support of csv file handling where all column headers
/// are predicate names.
///
/// Prefer to process data via stdin and stdout to enable *nix style
/// command pipelining.
///
use crate::db_api::DbApi;
use tracing::error;

/// write csv format to stdout all db entries
///
/// # Errors
///
/// return `Err` on db read errors
pub async fn export_csv(
    _export_ns_name: bool,
    _subject_column_name: Option<String>,
    _db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    error!("not implemented");
    Ok(())
}

/// read csv from stdin and load db
///
/// # Errors
///
/// return `Err` if any entry can not be loaded
pub async fn import_csv(
    _default_subject_ns: Option<String>,
    _subject_pos: i32,
    _default_predicate_ns: Option<String>,
    _db_api: &DbApi,
) -> Result<(), Box<dyn std::error::Error>> {
    error!("not implemented");
    Ok(())
}
