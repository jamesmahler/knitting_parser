#[derive(Debug, Clone, Copy)]
pub enum ParseErrorType {
    InvalidSyntaxRange(usize, usize),
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

#[derive(Debug, Clone)]
pub struct ParseError {
    error_type: ParseErrorType,
    line_number: usize,
}

impl ParseError {
    pub fn new(error_type: ParseErrorType, line_number: usize) -> ParseError {
        ParseError{error_type, line_number}
    }

    pub fn error_type(&self) -> ParseErrorType {
        self.error_type
    }

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
