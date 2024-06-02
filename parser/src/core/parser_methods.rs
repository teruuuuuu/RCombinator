use crate::core::common_parsers::space_or_line_seq;
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
        Parser::new(move |input, context| {
            match self.parse(input, context) {
            
                (next_context1, ParseResult::Success { value }) => {
                    (next_context1, ParseResult::successful(Either::Left(value)))
                }
                (next_context1, ParseResult::Failure {}) => match parser2.parse(input, context) {
                    (next_context2, ParseResult::Success { value }) => {
                        (next_context2, ParseResult::successful(Either::Right(value)))
                    }
                    (next_context2, ParseResult::Failure {}) => {
                        (
                            context.new_error("either", "no valid parsers").add_error(next_context1).add_error(next_context2),
                            ParseResult::failure()
                        )
                    },
                },
            }
        })
    }

    fn optional(self) -> Self::ParserNext<'a, Option<Self::Output>>
    where
        Self::Output: Clone + 'a,
    {
        Parser::new(move |input, context| match self.parse(input, context) {
            (next_context, ParseResult::Success { value }) 
                => (next_context, ParseResult::successful(Option::Some(value))),
                (next_context, ParseResult::Failure {}) 
                => (next_context, ParseResult::successful(Option::None))
        })
    }

    fn or(self, parser2: Parser<'a, Self::Output>) -> Self::ParserNext<'a, Self::Output>
    where
        Self::Output: Clone + 'a,
    {
        Parser::new(move |input, context| match self.parse(input, context) {
            (next_context1, ParseResult::Success { value }) 
                => (next_context1, ParseResult::successful(value)),
            (next_context1, ParseResult::Failure {}) => match parser2.parse(input, context) {
                (next_context2, ParseResult::Success { value }) => {
                    (next_context2, ParseResult::successful(value))
                }
                (next_context2, ParseResult::Failure {}) => 
                    (
                        context.new_error("or", "no valid parsers").add_error(next_context1).add_error(next_context2),
                        ParseResult::failure(),
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
        Parser::new(move |input, context| {
            let mut vec = Vec::<Self::Output>::new();
            let mut cur_context = context;
            let mut new_context;
            loop {
                match self.parse(input, cur_context) {
                    (next_context, ParseResult::Success { value }) => {
                        vec.push(value);
                        new_context = next_context;
                        cur_context = &mut new_context;
                    }
                    _ => {
                        new_context = cur_context.new_error("seq0", "parse seq stop");
                        cur_context = &mut new_context;
                        break;
                    }
                }
            }
            (cur_context.clone(),  ParseResult::successful(vec))
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
        Parser::new(move |input, context| {
            let mut vec = Vec::<Self::Output>::new();
            let mut cur_context = context;
            let mut new_context;
            loop {
                match self.parse(input, cur_context) {
                    (next_context, ParseResult::Success { value }) => {
                        vec.push(value);
                        new_context = next_context;
                        cur_context = &mut new_context;
                    }
                    _ => {
                        new_context = cur_context.new_error("seq1", "parse seq stop");
                        cur_context = &mut new_context;
                        break;
                    },
                }
            }
            if vec.len() > 0 {
                (cur_context.clone(),  ParseResult::successful(vec))
            } else {
                (cur_context.clone(),  ParseResult::failure())
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
    use super::common_parsers;
    use super::parse_context::ParseContext;
    use super::parser_input::ParserInput;

    
    let parser = common_parsers::char('a')
        .seq0()
        .with_skip_space()
        .skip_right(common_parsers::end());
    match parser.parse(&mut ParserInput::text(" \t\n \r\naaa"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 9);
            assert_eq!(value, vec!['a', 'a', 'a']);
        }
        (next_context, ParseResult::Failure { }) => {
            assert!(false);
        }
    }
}
