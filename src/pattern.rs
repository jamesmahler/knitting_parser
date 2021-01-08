//! Turns `Vec`s of stitches into a Pattern

use std::{
    cmp,
    collections::VecDeque,
    io::{BufRead, BufReader},
};

use crate::Options;
use crate::ParseError;
use crate::ParseErrorType;
use crate::Side;
use crate::Stitch;

use crate::parse_line;

/// The representation of a knitting pattern.
///
/// A pattern will be a rectangle in size.
pub struct Pattern {
    lines: Vec<VecDeque<Stitch>>,
    pattern_width: usize,
    options: Options,
}

fn calculate_line_width(stitches: &VecDeque<Stitch>) -> usize {
    let mut width = 0;
    for stitch in stitches {
        width += stitch.width();
    }
    width
}

impl Pattern {
    /// Create a new Pattern from the given Reader.
    ///
    /// This will pad the rows so they are all the same total width.
    ///
    /// # Arguments
    ///
    /// * `reader` - Where to read the stitches from
    ///
    pub fn new<R: std::io::Read>(reader: R) -> Result<Pattern, ParseError> {
        let reader = BufReader::new(reader);

        let mut lines: Vec<VecDeque<Stitch>> = Vec::new();
        let mut line_number = 1;
        let mut pattern_width = 0;

        // TODO: check first line is options, if so parse it out.
        let mut options = Options::default();

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.starts_with("##") {
                        // Options line
                        options.parse_options(&line, line_number)?;
                    } else if line.starts_with("#") {
                        // Ignored line
                    } else {
                        // Treat everything else as a stitch
                        let line_stitches = parse_line::parse_stitches(&line, line_number)?;
                        let line_width = calculate_line_width(&line_stitches);
                        pattern_width = cmp::max(pattern_width, line_width);

                        lines.push(line_stitches);
                    }
                }
                Err(error) => {
                    return Err(ParseError::new(ParseErrorType::UnableToReadFromReader(Box::new(error)), line_number));
                }
            }
            line_number += 1;
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

        Ok(Pattern { lines, pattern_width, options })
    }

    /// Returns the first line number for the pattern
    pub fn first_line_number(&self) -> usize {
        self.options.first_line_number
    }

    /// Returns the lines for the pattern
    pub fn lines(&self) -> &Vec<VecDeque<Stitch>> {
        &self.lines
    }

    /// Return what side the pattern starts on
    pub fn starting_side(&self) -> Side {
        self.options.starting_side
    }

    /// Return if the patter is in the round
    pub fn in_round(&self) -> bool {
        self.options.in_round
    }

    /// Returns the pattern switch width
    pub fn pattern_width(&self) -> usize {
        self.pattern_width
    }
}

#[cfg(test)]
mod test {
    use super::Stitch::*;
    use super::*;

    #[test]
    fn simple_pattern() {
        let input = b"k x5\nk x7";
        let pattern = Pattern::new(&input[..]).unwrap();

        assert_eq!(
            pattern.lines,
            vec![VecDeque::from(vec![NoStitch, K, K, K, K, K, NoStitch]), VecDeque::from(vec![K; 7]),]
        );
    }

    #[test]
    fn stitch_widths() {
        let input = b"k x4\n1lcf, 1rcb";
        let pattern = Pattern::new(&input[..]).unwrap();

        assert_eq!(pattern.lines, vec![VecDeque::from(vec![K, K, K, K]), VecDeque::from(vec![Lcf1, Rcb1]),]);
    }

    #[test]
    fn simple_exception() {
        let input = b"k x6\nk x7";

        if let Err(parse_error) = Pattern::new(&input[..]) {
            if let ParseErrorType::InvalidStitchCount(count) = **parse_error.error_type() {
                assert_eq!(count, 6);
            } else {
                assert!(false, "Wrong error type returned");
            }
        } else {
            assert!(false, "Should not have been valid");
        }
    }
}
