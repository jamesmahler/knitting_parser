use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::opt,
    multi::separated_list,
    sequence::preceded,
    IResult,
};

use std::collections::VecDeque;
use std::str;

use crate::error::ParseError;
use crate::error::ParseErrorType;
use crate::stitches::Stitch;

fn stitch<'a>(stitch_str: &'a str, stitch_type: Stitch) -> impl Fn(&'a str) -> IResult<&'a str, Stitch> {
    move |line: &'a str| {
        let res: IResult<_, _> = match tag(stitch_str)(line) {
            Ok((line, _)) => Ok((line, stitch_type)),
            Err(e) => Err(e),
        };

        res
    }
}

fn multiplier(line: &str) -> IResult<&str, usize> {
    let (line, _) = tag("x")(line)?;
    let (line, mult) = digit1(line)?;

    // This unwrap should be fine, as we only allow valid chars from above.
    Ok((line, mult.parse::<usize>().unwrap()))
}

fn padded_group(line: &str) -> IResult<&str, std::vec::Vec<Stitch>> {
    let (line, _) = space0(line)?;
    let (line, _) = tag("(")(line)?;
    let (line, vecs) = separated_list(char(','), alt((padded_group, padded_stitch)))(line)?;
    let (line, _) = space0(line)?;
    let (line, _) = tag(")")(line)?;
    let (line, _) = space0(line)?;
    let (line, mult) = opt(multiplier)(line)?;

    let mut vec = std::vec::Vec::new();
    match mult {
        Some(val) => {
            for _ in 0..val {
                for vec_to_append in &vecs {
                    vec.extend(vec_to_append);
                }
            }
        }
        None => {
            for vec_to_append in &vecs {
                vec.extend(vec_to_append);
            }
        }
    }

    Ok((line, vec))
}

fn padded_stitch(line: &str) -> IResult<&str, std::vec::Vec<Stitch>> {
    let (line, _) = space0(line)?;

    // alt can't support all the stitches, so they are broken up by prefix when there are duplicates.
    // while a more verbose, this is probably faster anyways as it cuts down on duplicate checks.
    let (line, stitch) = alt((
        preceded(
            tag("1"),
            alt((
                // 1 prefixed stitches
                stitch("lcf", Stitch::Lcf1),
                stitch("rcb", Stitch::Rcb1),
            )),
        ),
        preceded(
            tag("2"),
            alt((
                // 2 prefixed stitches
                stitch("lcf", Stitch::Lcf2),
                stitch("rcb", Stitch::Rcb2),
            )),
        ),
        preceded(
            tag("3"),
            alt((
                // 3 prefixed stitches
                stitch("lcf", Stitch::Lcf3),
                stitch("rcb", Stitch::Rcb3),
            )),
        ),
        preceded(
            tag("4"),
            alt((
                // 4 prefixed stitches
                stitch("lcf", Stitch::Lcf4),
                stitch("rcb", Stitch::Rcb4),
            )),
        ),
        preceded(
            tag("b"),
            alt((
                // b prefixed stitches
                stitch("ead", Stitch::Bead),
                stitch("obble", Stitch::Bobble),
                stitch("o", Stitch::Bo),
            )),
        ),
        preceded(
            tag("k"),
            alt((
                // k prefixed stitches
                stitch("tbl", Stitch::Ktbl),
                stitch("bf", Stitch::Kbf),
                stitch("fb", Stitch::Kfb),
                stitch("2tog", Stitch::K2Tog),
                stitch("", Stitch::K),
            )),
        ),
        preceded(
            tag("m"),
            alt((
                // m prefixed stitches
                stitch(" pwise", Stitch::MPwise),
                stitch(" kwise", Stitch::MKwise),
                stitch("l", Stitch::Ml),
                stitch("r", Stitch::Mr),
            )),
        ),
        preceded(
            tag("p"),
            alt((
                // p prefixed stitches
                stitch("tbl", Stitch::Ptbl),
                stitch("bf", Stitch::Pbf),
                stitch("fb", Stitch::Pfb),
                stitch("2tog", Stitch::P2Tog),
                stitch("", Stitch::P),
            )),
        ),
        preceded(
            tag("s"),
            alt((
                // s prefixed stitches
                stitch("l pwise", Stitch::SlPwise),
                stitch("l kwise", Stitch::SlKwise),
                stitch("sp", Stitch::Ssp),
                stitch("sk", Stitch::Ssk),
            )),
        ),
        stitch("nostitch", Stitch::NoStitch),
        stitch("yo", Stitch::Yo),
    ))(line)?;

    let (line, _) = space0(line)?;
    let (line, mult) = opt(multiplier)(line)?;
    match mult {
        Some(val) => Ok((line, vec![stitch; val])),
        None => Ok((line, vec![stitch])),
    }
}

fn extract_parse_error_type(starting_line: &str, line: &str) -> ParseErrorType {
    let range_start = starting_line.len() - line.len();
    let range_end = starting_line.len() - 1;

    ParseErrorType::InvalidSyntaxRange(range_start, range_end)
}

pub fn parse_stitches(line: &str, line_number: usize) -> Result<VecDeque<Stitch>, ParseError> {
    // TODO: need to parse off line number override and return

    let starting_line = line;

    match separated_list(char(','), alt((padded_group, padded_stitch)))(line) {
        Ok((line, stitches)) => {
            // Combine all the above vectors into one.
            let mut vec = VecDeque::new();

            let mut total_size = 0;
            for vec_to_append in &stitches {
                total_size += vec_to_append.len();
            }
            vec.reserve(total_size);

            for vec_to_append in &stitches {
                vec.extend(vec_to_append);
            }

            if !line.is_empty() {
                let error_type = extract_parse_error_type(starting_line, line);

                return Err(ParseError::new(error_type, line_number));
            }

            Ok(vec)
        }

        Err(err) => match err {
            nom::Err::Incomplete(_) => {
                let error_type = extract_parse_error_type(starting_line, line);
                Err(ParseError::new(error_type, line_number))
            }
            nom::Err::Error((line, _)) => {
                let error_type = extract_parse_error_type(starting_line, line);
                Err(ParseError::new(error_type, line_number))
            }
            nom::Err::Failure((line, _)) => {
                let error_type = extract_parse_error_type(starting_line, line);
                Err(ParseError::new(error_type, line_number))
            }
        },
    }
}

#[cfg(test)]
mod test {
    use super::Stitch::*;
    use super::*;

    #[test]
    fn simple_parse() {
        let stitches = parse_stitches("k, k2tog", 0).unwrap();
        assert_eq!(stitches, vec![K, K2Tog]);
    }

    #[test]
    fn simple_exception() {
        if let Err(parse_error) = parse_stitches("k, p, bad", 2) {
            assert_eq!(parse_error.line_number(), 2);

            // 'k, p' will parse
            // ', bad' will not
            if let ParseErrorType::InvalidSyntaxRange(range_start, range_end) = parse_error.error_type() {
                assert_eq!(range_start, 4);
                assert_eq!(range_end, 8);
            } else {
                assert!(false, "Wrong error type returned");
            }
        } else {
            assert!(false, "Should not have parsed");
        }
    }

    #[test]
    fn with_modifier() {
        let stitches = parse_stitches("sl kwise", 0).unwrap();
        assert_eq!(stitches, VecDeque::from(vec![SlKwise]));
    }

    #[test]
    fn with_multipler() {
        let stitches = parse_stitches("k x3", 0).unwrap();
        assert_eq!(stitches, VecDeque::from(vec![K; 3]));
    }

    #[test]
    fn group() {
        let stitches = parse_stitches("(k, p)", 0).unwrap();
        assert_eq!(stitches, VecDeque::from(vec![K, P]));
    }

    #[test]
    fn embeded_group() {
        let stitches = parse_stitches("(k, (p, k))", 0).unwrap();
        assert_eq!(stitches, VecDeque::from(vec![K, P, K]));
    }

    #[test]
    fn group_multiplier() {
        let stitches = parse_stitches("(k, p) x2", 0).unwrap();
        assert_eq!(stitches, VecDeque::from(vec![K, P, K, P]));
    }

    #[test]
    fn embeded_group_multiplier() {
        let stitches = parse_stitches("(k, (p, k) x2) x2", 0).unwrap();
        assert_eq!(stitches.len(), 10);
        assert_eq!(stitches, VecDeque::from(vec![K, P, K, P, K, K, P, K, P, K]));
    }
}
