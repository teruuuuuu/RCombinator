use crate::core::common_parsers::space_or_line_seq;
use crate::core::either::Either;
use crate::core::parse_error::ParseError;
use crate::core::parser::Parser;

use super::common_parsers::{self, end};
use super::parse_result::ParseResult;
use super::parser::{ParserFunctor, ParserTrait};
use super::parser::ParserMonad;

pub trait ParserMethods<'a>: ParserTrait<'a> {
    fn and<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, (Self::Output, B)>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn and_left<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn and_right<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn either<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Either<Self::Output, B>>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn optional(self) -> Self::ParserNext<'a, Option<Self::Output>>
    where
        Self::Output: Clone + 'a;

    fn or(self, parser2: Parser<'a, Self::Output>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a;

    fn pure<B>(self, b: B) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn seq0(self) -> Self::ParserNext<'a, Vec<Self::Output>>
    where
        Self::Output: Clone + 'a;

    fn seq1(self) -> Self::ParserNext<'a, Vec<Self::Output>>
    where
        Self::Output: Clone + 'a;

    fn skip_left<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn skip_right<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn with_skip_space(self) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a;
}

impl<'a, A> ParserMethods<'a> for Parser<'a, A> {
    fn and<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, (A, B)>
    where
        A: Clone + 'a,
        B: Clone + 'a,
    {
        self.flat_map(move |value_a| {
            parser2
                .clone()
                .map(move |value_b| (value_a.clone(), value_b))
        })
    }

    fn and_left<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        self.and(parser2).map(|v| v.0)
    }

    fn and_right<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        self.and(parser2).map(|v| v.1)
    }

    fn either<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Either<Self::Output, B>>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        Parser::new(move |input, location| match self.parse(input, location) {
            ParseResult::Success { value, location } => {
                ParseResult::successful(Either::Left(value), location)
            }
            ParseResult::Failure {
                parse_error: parse_error_1,
                location: location_1,
            } => match parser2.parse(input, location) {
                ParseResult::Success { value, location } => {
                    ParseResult::successful(Either::Right(value), location)
                }
                ParseResult::Failure {
                    parse_error: parse_error_2,
                    location: location_2,
                } =>
                    ParseResult::failure(
                        ParseError::new(
                            "either".to_string(),
                            "not valid parser".to_string(),
                            location,
                            vec![parse_error_1, parse_error_2]
                        ),
                        location
                    )
            },
        })
    }

    fn optional(self) -> Self::ParserNext<'a, Option<Self::Output>>
    where
        Self::Output: Clone + 'a,
    {
        Parser::new(move |input, location| match self.parse(input, location) {
            ParseResult::Success { value, location } => ParseResult::successful(Option::Some(value), location),
            ParseResult::Failure { parse_error,location} => ParseResult::successful(Option::None, location)
        })
    }

    fn or(self, parser2: Parser<'a, Self::Output>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
    {
        Parser::new(move |input, location| match self.parse(input, location) {
            ParseResult::Success { value, location } => ParseResult::successful(value, location),
            ParseResult::Failure {
                parse_error: parse_error_1,
                location,
            } => match parser2.parse(input, location) {
                ParseResult::Success { value, location } => {
                    ParseResult::successful(value, location)
                }
                ParseResult::Failure {
                    parse_error: parse_error_2,
                    location,
                } =>
                    ParseResult::failure(
                        ParseError::new(
                            "or".to_string(),
                            "not valid parser".to_string(),
                            location,
                            vec![parse_error_1, parse_error_2]
                        ),
                        location
                    )
            },
        })
    }

    fn pure<B>(self, b: B) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        self.map(move |_v| b.clone())
    }

    fn seq0(self) -> Self::ParserNext<'a, Vec<Self::Output>>
    where
        Self::Output: Clone + 'a,
    {
        Parser::new(move |input, location| {
            let mut vec = Vec::<Self::Output>::new();
            let mut cur_location = location;
            loop {
                match self.parse(input, cur_location) {
                    ParseResult::Success { value, location } => {
                        vec.push(value);
                        cur_location = location;
                    }
                    _ => break,
                }
            }
            ParseResult::successful(vec, cur_location)
        })
    }

    fn seq1(self) -> Self::ParserNext<'a, Vec<Self::Output>>
    where
        Self::Output: Clone + 'a,
    {
        // self.seq0().flat_map(move |v| {
        //     Parser::new(move |_, location| {
        //         if v.len() > 0 {
        //             ParseResult::Success { value: v.clone(), location }
        //         } else {
        //             ParseResult::Failure { message: "".to_owned(), location }
        //         }
        //     })
        // })
        Parser::new(move |input, location| {
            let mut vec = Vec::<Self::Output>::new();
            let mut cur_location = location;
            let mut parse_errors = Vec::new();
            loop {
                match self.parse(input, cur_location) {
                    ParseResult::Success { value, location } => {
                        vec.push(value);
                        cur_location = location;
                    }
                    ParseResult::Failure { parse_error, location} => {
                        parse_errors.push(parse_error);
                        break
                    }
                }
            }
            if vec.len() > 0 {
                ParseResult::successful(vec, cur_location)
            } else {
                ParseResult::failure(
                    ParseError::new(
                        "seq1".to_string(),
                        "not valid parser".to_string(),
                        location,
                        parse_errors
                    ),
                    location
                )
            }
        })
    }

    fn skip_left<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        self.and(parser2).map(|v| v.1)
    }

    fn skip_right<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        self.and(parser2).map(|v| v.0)
    }

    fn with_skip_space(self) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
    {
        space_or_line_seq().skip_left(self)
    }
}

#[test]
fn test_skip_space() {
    let parser = common_parsers::char('a')
        .seq0()
        .with_skip_space()
        .skip_right(end());
    match parser.parse(" \t\n \r\naaa", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 9);
            assert_eq!(value, vec!['a', 'a', 'a']);
        }
        ParseResult::Failure { parse_error, location } => {
            assert!(false);
        }
    }
}
