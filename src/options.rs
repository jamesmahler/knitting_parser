//! Additional options that can be set on a pattern.

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0},
    combinator::opt,
    IResult,
};

use std::str;

use crate::Side;
use crate::ParseError;
use crate::util::into_parse_error;

/// The options for a pattern.
#[derive(Debug)]
pub struct Options {
    /// What the first line should be labeled.
    pub first_line_number: usize,

    /// What side to start with.
    pub starting_side: Side,

    /// If the pattern is in the round.
    pub in_round: bool,
}

impl Default for Options {
    fn default() -> Self {
        let first_line_number = 1;
        let starting_side = Side::RS;
        let in_round = false;
        Options {
            first_line_number,
            starting_side,
            in_round,
        }
    }
}

impl Options {
    fn parse_in_round<'a>(&mut self, line: &'a str) -> IResult<&'a str, ()> {
        let (line, in_round) = opt(tag("in_round"))(line)?;
        match in_round {
            Some(_) => {
                if !line.is_empty() {
                    line_ending(line)?;
                }
                self.in_round = true;
                Ok((line, ()))
            },
            None => {
                Ok((line, ()))
            }
        }
    }

    fn parse_start_wrong<'a>(&mut self, line: &'a str) -> IResult<&'a str, ()> {
        let (line, start_wrong) = opt(tag("start_wrong_side"))(line)?;
        match start_wrong {
            Some(_) => {
                if !line.is_empty() {
                    line_ending(line)?;
                }
                self.starting_side = Side::WS;
                Ok((line, ()))
            },
            None => {
                Ok((line, ()))
            }
        }
    }

    fn parse_first_line<'a>(&mut self, line: &'a str) -> IResult<&'a str, ()> {
        let (line, first_line) = opt(tag("first_line="))(line)?;
        match first_line {
            Some(_) => {
                let (line, first_line_number) = digit1(line)?;
                if !line.is_empty() {
                    line_ending(line)?;
                }

                // This unwrap should be fine, we check it's valid above.
                self.first_line_number = first_line_number.parse::<usize>().unwrap();
                Ok((line, ()))
            }
            None => {
                Ok((line, ()))
            }
        }
    }

    fn internal_parse<'a>(&mut self, line: &'a str) -> IResult<&'a str, ()> {
        let (line, _) = space0(line)?;

        let (line, _) = self.parse_in_round(line)?;
        let (line, _) = self.parse_first_line(line)?;
        let (line, _) = self.parse_start_wrong(line)?;

        // make sure we fully parsed the line
        if !line.is_empty() {
            line_ending(line)?;
        }

        return Ok((line, ()));
    }

    /// Parse the options from the given line.
    pub fn parse_options(&mut self, line: &str, line_number: usize) -> Result<(), ParseError> {
        let starting_line = line;

        // take off the starting ##
        let line = &line[2..];

        match self.internal_parse(line) {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                Err(into_parse_error(starting_line, line, line_number))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in_round() {
        let mut options = Options::default();
        options.parse_options("## in_round", 0).unwrap();
        assert!(options.in_round);
    }

    #[test]
    fn line_number() {
        let mut options = Options::default();
        options.parse_options("## first_line=3", 0).unwrap();
        assert_eq!(options.first_line_number, 3);
    }

    #[test]
    fn start_wrong() {
        let mut options = Options::default();
        options.parse_options("## start_wrong_side", 0).unwrap();
        assert_eq!(options.starting_side, Side::WS);
    }
}
