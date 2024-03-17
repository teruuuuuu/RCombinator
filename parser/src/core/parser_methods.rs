use std::fmt::format;
use std::panic::Location;

use crate::core::either::Either;
use crate::core::parser::Parser;

use super::parse_result::ParseResult;
use super::parser::ParserMonad;
use super::parser::{ParserFunctuor, ParserTrait};

pub trait ParserMethods<'a>: ParserTrait<'a> {
    fn pure<B>(self, b: B) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn and<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, (Self::Output, B)>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

    fn or<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Either<Self::Output, B>>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;
}

impl<'a, A> ParserMethods<'a> for Parser<'a, A> {
    fn pure<B>(self, b: B) -> Self::ParserNext<'a, B>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        // parser.map(|)
        // ParsersMethods::pure(parser, b)
        self.map(move |_v| b.clone())
    }

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

    fn or<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Either<Self::Output, B>>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        Parser::new(move |input, location| match self.parse(input, location) {
            ParseResult::Success { value, location } => {
                ParseResult::successful(Either::Left(value), location)
            }
            ParseResult::Failure {
                message: message1,
                location,
            } => match parser2.parse(input, location) {
                ParseResult::Success { value, location } => 
                    ParseResult::successful(Either::Right(value), location),
                ParseResult::Failure {message: message2,location,} => 
                    ParseResult::failure(format!("{},{}", message1, message2), location),
            },
        })
    }
}

// pub struct Parsers{}
// pub trait ParsersMethods {
//     fn pure<'a,A,B>(paser1: Parser<'a, A>, value: B) -> Parser<'a, B>
//     where
//         A: Clone + 'a,
//         B: Clone + 'a;
// }

// impl ParsersMethods for Parsers {

//     fn pure<'a,A,B>(paser1: Parser<'a, A>, value: B) -> Parser<'a, B>
//     where
//         A: Clone + 'a,
//         B: Clone + 'a {
//             Parser::new(|input, location| ParseResult::successful(value, location))
//         }
// }
