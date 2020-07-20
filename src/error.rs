//! The standard error types used to make understanding parse errors easier

/// What type of parsing issue was it
#[derive(Debug, Clone, Copy)]
pub enum ParseErrorType {
    /// Used when a range of syntax is unparsable giving the start and end locations
    InvalidSyntaxRange(usize, usize),

    /// Used when the stitch count is not right giving the found count.  Stitch counts should go up or down by even numbers from one row to the next.
    InvalidStitchCount(usize),
}

impl std::fmt::Display for ParseErrorType {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorType::InvalidStitchCount(count) => write!(out, "{{ \"type\" : \"Invalid stitch count\", \"count\" : {} }}", count),
            ParseErrorType::InvalidSyntaxRange(range_start, range_end) => write!(
                out,
                "{{ \"type\" : \"Invalid syntax range\", \"start\" : {}, \"end\" : {} }}",
                range_start, range_end
            ),
        }
    }
}

/// The parse error.
///
/// The is the error type returned from all parsing functions.
#[derive(Debug, Clone)]
pub struct ParseError {
    error_type: ParseErrorType,
    line_number: usize,
}

impl ParseError {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// * `error_type` - What type of error it is
    /// * `line_number` - What line did it occure on
    pub fn new(error_type: ParseErrorType, line_number: usize) -> ParseError {
        ParseError { error_type, line_number }
    }

    /// Returns the contained error type
    pub fn error_type(&self) -> ParseErrorType {
        self.error_type
    }

    /// Returns the contained line number
    pub fn line_number(&self) -> usize {
        self.line_number
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "{{ \"error\":{}, \"line\":{} }}", self.error_type, self.line_number)
    }
}

impl std::error::Error for ParseError {}
