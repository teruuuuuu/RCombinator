use std::rc::Rc;

use super::parse_context::ParseContext;
use super::parse_result::ParseResult;
use super::parser_input::ParserInput;

type Parse<'a, A> = dyn Fn(&mut ParserInput<'a>, &mut ParseContext<'a>) -> (ParseContext<'a>, ParseResult<A>) + 'a;

pub struct Parser<'a, A> {
    pub parse: Rc<Parse<'a, A>>,
}

impl<'a, A> Parser<'a, A> {
    pub fn new<F>(parse: F) -> Parser<'a, A>
    where
        F: Fn(&mut ParserInput<'a>, &mut ParseContext<'a>) -> (ParseContext<'a>, ParseResult<A>) + 'a,
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

    fn parse(&self, input: &mut ParserInput<'a>, context: &mut ParseContext<'a>) -> (ParseContext<'a>, ParseResult<Self::Output>);

}

impl<'a, A> ParserTrait<'a> for Parser<'a, A> {
    type Output = A;
    type ParserNext<'m, X> = Parser<'m, X>;

    fn parse(&self, input: &mut ParserInput<'a>, context: &mut ParseContext<'a>) -> (ParseContext<'a>, ParseResult<Self::Output>) {
        (self.parse)(input, context)
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
        Parser::new(move |input, context| match self.parse(input, context) {


            (next_context, ParseResult::Success { value }) 
                => (next_context, ParseResult::successful(f(value))),
            (next_context, ParseResult::Failure { }) 
                => (next_context, ParseResult::failure()),
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
            move |input: &mut ParserInput<'a>, context: &mut ParseContext<'a>| match self.parse(input, context) {
                (next_context, ParseResult::Success { value }) 
                    => f(value).parse(input, &mut next_context.clone()),
                    (next_context, ParseResult::Failure { }) 
                    => (next_context, ParseResult::Failure { })
            },
        )
    }

}
