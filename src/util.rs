
use crate::ParseError;
use crate::ParseErrorType;

/// Convert from from the starting line and remaining line into parse error
///
/// # Arguments
///
/// * `starting_line` - The line before any parsing
/// * `line` - The line remaining after any successful parsing
/// * `line_number` - What line number this line is (used for error reporting)
///
pub fn into_parse_error(starting_line: &str, line: &str, line_number: usize) -> ParseError {
    let error_type = extract_parse_error_type(starting_line, line);
    ParseError::new(error_type, line_number)
}

/// Extracts a syntax error from the starting string and string after parsing.
///
/// # Arguments
///
/// * `starting_line` - The line before any parsing
/// * `line` - The line remaining after any successful parsing
///
pub fn extract_parse_error_type(starting_line: &str, line: &str) -> ParseErrorType {
    let range_start = starting_line.len() - line.len();
    let range_end = starting_line.len() - 1;

    ParseErrorType::InvalidSyntaxRange(range_start, range_end)
}
