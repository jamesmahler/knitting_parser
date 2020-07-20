//! Turns `Vec`s of stitches into a Pattern

use std::cmp::max;

use std::collections::VecDeque;

use crate::error::ParseError;
use crate::error::ParseErrorType;
use crate::stitches::Stitch;

/// The representation of a knitting pattern.
///
/// A pattern will be a rectangle in size.
pub struct Pattern {
    first_line_number: usize,
    lines: Vec<VecDeque<Stitch>>,
}

fn calculate_line_width(stitches: &VecDeque<Stitch>) -> usize {
    let mut width = 0;
    for stitch in stitches {
        width += stitch.width();
    }
    width
}

impl Pattern {
    /// Create a new Pattern from the given stitches.
    ///
    /// This will pad the rows so they are all the same total width.
    ///
    /// # Arguments
    ///
    /// * `first_line_number` - The first line number to give the pattern
    /// * `lines` - The lines of stitches to give to the pattern
    pub fn new(first_line_number: usize, mut lines: Vec<VecDeque<Stitch>>) -> Result<Pattern, ParseError> {
        let mut pattern_width = 0;

        for line in &lines {
            pattern_width = max(pattern_width, calculate_line_width(line));
        }

        for (line_number, line) in lines.iter_mut().enumerate() {
            let line_width = calculate_line_width(line);
            if line_width == pattern_width {
                continue;
            }

            let mut needed_stitches = pattern_width - line_width;

            if needed_stitches % 2 != 0 {
                let error_type = ParseErrorType::InvalidStitchCount(line_width);

                return Err(ParseError::new(error_type, line_number));
            }

            while needed_stitches != 0 {
                line.push_front(Stitch::NoStitch);
                line.push_back(Stitch::NoStitch);

                needed_stitches -= 2;
            }
        }

        Ok(Pattern { first_line_number, lines })
    }

    /// Returns the first line number for the pattern
    pub fn first_line_number(&self) -> usize {
        self.first_line_number
    }

    /// Returns the lines for the pattern
    pub fn lines(&self) -> &Vec<VecDeque<Stitch>> {
        &self.lines
    }
}

#[cfg(test)]
mod test {
    use super::Stitch::*;
    use super::*;

    #[test]
    fn simple_pattern() {
        let lines = vec![VecDeque::from(vec![K; 5]), VecDeque::from(vec![K; 7])];
        let pattern = Pattern::new(1, lines).unwrap();

        assert_eq!(
            pattern.lines,
            vec![VecDeque::from(vec![NoStitch, K, K, K, K, K, NoStitch]), VecDeque::from(vec![K; 7]),]
        );
    }

    #[test]
    fn stitch_widths() {
        let lines = vec![VecDeque::from(vec![K; 4]), VecDeque::from(vec![Lcf1, Rcb1])];
        let pattern = Pattern::new(1, lines).unwrap();

        assert_eq!(pattern.lines, vec![VecDeque::from(vec![K, K, K, K]), VecDeque::from(vec![Lcf1, Rcb1]),]);
    }

    #[test]
    fn simple_exception() {
        let lines = vec![VecDeque::from(vec![K; 6]), VecDeque::from(vec![K; 7])];

        if let Err(parse_error) = Pattern::new(1, lines) {
            if let ParseErrorType::InvalidStitchCount(count) = parse_error.error_type() {
                assert_eq!(count, 6);
            } else {
                assert!(false, "Wrong error type returned");
            }
        } else {
            assert!(false, "Should not have been valid");
        }
    }
}
