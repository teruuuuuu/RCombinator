use std::rc::Rc;
use std::task::Context;

use super::parse_context::ParseContext;
use super::parse_result::ParseResult;

type Parse<'a, A> = dyn Fn(ParseContext, &'a str, usize) -> (ParseContext, ParseResult<A>) + 'a;

pub struct Parser<'a, A> {
    pub parse: Rc<Parse<'a, A>>,
}

impl<'a, A> Parser<'a, A> {
    pub fn new<F>(parse: F) -> Parser<'a, A>
    where
        F: Fn(ParseContext, &'a str, usize) -> (ParseContext, ParseResult<A>) + 'a,
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

    fn parse(&self, context: ParseContext, input: &'a str, location: usize) -> (ParseContext, ParseResult<Self::Output>);

}

impl<'a, A> ParserTrait<'a> for Parser<'a, A> {
    type Output = A;
    type ParserNext<'m, X> = Parser<'m, X>;

    fn parse(&self, context: ParseContext, input: &'a str, location: usize) -> (ParseContext, ParseResult<Self::Output>) {
        (self.parse)(context, input, location)
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
        Parser::new(move |context, input, location| match self.parse(context, input, location) {


            (next_context, ParseResult::Success { value, location }) 
                => (next_context, ParseResult::successful(f(value), location)),
            (error_context, ParseResult::Failure { parse_error, location}) 
                => (error_context, ParseResult::failure(parse_error, location)),
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
            move |context, input, location| match self.parse(context, input, location) {
                (next_context, ParseResult::Success { value, location}) 
                    => {
                        f(value).parse(next_context, input, location)
                    },
                (error_context, ParseResult::Failure { parse_error, location}) 
                    => (error_context, ParseResult::Failure { parse_error, location })
            },
        )
    }

}
