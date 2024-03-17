use crate::core::parse_result::ParseResult;

use std::rc::Rc;

type Parse<'a, A> = dyn Fn(&'a str, usize) -> ParseResult<A> + 'a;

pub struct Parser<'a, A> {
    pub parse: Rc<Parse<'a, A>>,
}

impl<'a, A> Parser<'a, A> {
    pub fn new<F>(parse: F) -> Parser<'a, A>
    where
        F: Fn(&'a str, usize) -> ParseResult<A> + 'a,
    {
        Parser {
            parse: Rc::new(parse),
        }
    }
}

impl<'a, A> Clone for Parser<'a, A> {
    fn clone(&self) -> Self {
        Self {
            parse: self.parse.clone(),
        }
    }
}

pub trait ParserTrait<'a> {
    type Output;
    type ParserNext<'m, X>: ParserTrait<'m, Output = X>;

    fn parse(&self, input: &'a str, location: usize) -> ParseResult<Self::Output>;

}

impl<'a, A> ParserTrait<'a> for Parser<'a, A> {
    type Output = A;
    type ParserNext<'m, X> = Parser<'m, X>;

    fn parse(&self, input: &'a str, location: usize) -> ParseResult<Self::Output> {
        (self.parse)(input, location)
    }
}

pub trait ParserFunctuor<'a>: ParserTrait<'a> {
    fn map<B, F>(self, f: F) -> Self::ParserNext<'a, B>
    where
        F: Fn(Self::Output) -> B + 'a,
        Self::Output: Clone + 'a,
        B: Clone + 'a;
}

impl<'a, A> ParserFunctuor<'a> for Parser<'a, A> {
    fn map<B, F>(self, f: F) -> Self::ParserNext<'a, B>
    where
        F: Fn(Self::Output) -> B + 'a,
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        Parser::new(move |input, location| match self.parse(input, location) {
            ParseResult::Success { value, location } => ParseResult::successful(f(value), location),
            ParseResult::Failure { message, location } => ParseResult::failure(message, location),
        })
    }
}

pub trait ParserMonad<'a>: ParserFunctuor<'a> {
    fn flat_map<B, F>(self, f: F) -> Self::ParserNext<'a, B>
    where
        F: Fn(Self::Output) -> Parser<'a, B> + 'a,
        Self::Output: Clone + 'a,
        B: Clone + 'a;

}

impl<'a, A> ParserMonad<'a> for Parser<'a, A> {
    fn flat_map<B, F>(self, f: F) -> Self::ParserNext<'a, B>
    where
        F: Fn(Self::Output) -> Parser<'a, B> + 'a,
        Self::Output: Clone + 'a,
        B: Clone + 'a,
    {
        Parser::new(
            move |input: &str, location: usize| match self.parse(input, location) {
                ParseResult::Success { value, location } => f(value).parse(input, location),
                ParseResult::Failure { message, location } => {
                    ParseResult::Failure { message, location }
                }
            },
        )
    }

}
