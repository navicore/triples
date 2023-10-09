/// Functions in support of csv file handling.
///
use crate::data::TriplesError;

// Utility function to determine the display name based on the strip_ns flag
///
/// # Errors
///
/// return `Err` on `InvalidIRI`
pub fn get_display_name(name_string: &str, export_ns_name: bool) -> Result<&str, TriplesError> {
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

#[must_use]
pub fn sanitize_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('\n') || field.contains('"') {
        // Escape any double quotes and surround the whole field with double quotes
        format!("\"{}\"", field.replace('\"', "\"\""))
    } else {
        field.to_string()
    }
}
