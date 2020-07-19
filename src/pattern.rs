use std::cmp::max;

use std::collections::VecDeque;

use crate::error::ParseError;
use crate::error::ParseErrorType;
use crate::stitches::Stitch;

pub struct Pattern {
    first_line_number: usize,
    lines: Vec<VecDeque<Stitch>>,
}

impl Pattern {
    pub fn new(first_line_number: usize, mut lines: Vec<VecDeque<Stitch>>) -> Result<Pattern, ParseError> {
        let mut pattern_width = 0;
    
        for line in &lines {
            pattern_width = max(pattern_width, line.len());
        }
    
        for (line_number, line) in lines.iter_mut().enumerate() {
            if line.len() == pattern_width {
                continue;
            }
    
            let mut needed_stitches = pattern_width - line.len();
    
            if needed_stitches % 2 != 0 {
                let error_type = ParseErrorType::InvalidStitchCount(line.len());
    
                return Err(ParseError::new(error_type, line_number));
            }
    
            while needed_stitches != 0 {
                line.push_front(Stitch::NoStitch);
                line.push_back(Stitch::NoStitch);
    
                needed_stitches -= 2;
            }
        }
    
        Ok(Pattern{first_line_number, lines})
    }

    pub fn first_line_number(&self) -> usize {
        self.first_line_number
    }

    pub fn lines(&self) -> &Vec<VecDeque<Stitch>> {
        &self.lines
    }
}

#[cfg(test)]
mod test {
    use super::Stitch::*;
    use super::*;

    #[test]
    fn simple_parse() {
        let lines = vec![VecDeque::from(vec![K; 5]), VecDeque::from(vec![K; 7])];
        let pattern = Pattern::new(1, lines).unwrap();

        assert_eq!(
            pattern.lines,
            vec![VecDeque::from(vec![NoStitch, K, K, K, K, K, NoStitch]), VecDeque::from(vec![K; 7]),]
        );
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
