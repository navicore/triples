/// Functions in support of csv file handling.
///
use crate::data::{extract_namespace_and_local_name, RdfName, TriplesError};

// Utility function to determine the display name based on the strip_ns flag
///
/// # Errors
///
/// return `Err` on `InvalidIRI`
pub fn get_display_name(name: &RdfName, export_ns_name: bool) -> Result<String, TriplesError> {
    let name_string = name.to_string();
    if export_ns_name {
        Ok(name_string)
    } else {
        let (_ns, local_name) = extract_namespace_and_local_name(&name_string)?;
        Ok(local_name.to_string())
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
