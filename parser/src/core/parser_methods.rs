use crate::core::parser::{Parser, Parsers};

use super::parser::{ParserFunctuor, ParsersImpl};
use super::parser::ParserMonad;

pub trait ParserMethods: Parsers {

    fn and<'a, A,B>(paser1: Self::ParserNext<'a, A>, parser2: Parser<'a, B>) -> Self::ParserNext<'a, (A,B)> 
    where
    A: Clone + 'a,
    B: Clone + 'a;
}

impl ParserMethods for ParsersImpl {

    fn and<'a, A,B>(parser1: Self::ParserNext<'a, A>, parser2: Parser<'a, B>) -> Self::ParserNext<'a, (A,B)> 
    where
    A: Clone + 'a,
    B: Clone + 'a {
        parser1.flat_map(move |value_a| 
            parser2.clone().map(move |value_b| (value_a.clone(), value_b))
        )
    }
}
