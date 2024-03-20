use crate::core::either::Either;
use crate::core::parser::Parser;

use super::parse_result::ParseResult;
use super::parser::ParserMonad;
use super::parser::{ParserFunctuor, ParserTrait};

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

    fn or<B>(self, parser2: Parser<'a, B>) -> Self::ParserNext<'a, Either<Self::Output, B>>
    where
        Self::Output: Clone + 'a,
        B: Clone + 'a;

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
                ParseResult::Success { value, location } => {
                    ParseResult::successful(Either::Right(value), location)
                }
                ParseResult::Failure {
                    message: message2,
                    location,
                } => ParseResult::failure(format!("{},{}", message1, message2), location),
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
                    },
                    _ => break
                }
            }
            ParseResult::successful(vec, location)
        })
    }

    fn seq1(self) -> Self::ParserNext<'a, Vec<Self::Output>>
    where
        Self::Output: Clone + 'a {
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
                loop {
                    
                    match self.parse(input, cur_location) {
                        ParseResult::Success { value, location } => {
                            vec.push(value);
                            cur_location = location;
                        },
                        _ => break
                    }
                }
                if vec.len() > 0 {
                    ParseResult::successful(vec, location)
                } else {
                    ParseResult::Failure { message: "not matched".to_owned(), location }
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
}
