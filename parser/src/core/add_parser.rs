use std::ops::Add;

use crate::core::parser::Parser;
use crate::core::parser_methods::ParserMethods;

impl<'a, A, B> Add<Parser<'a, B>> for Parser<'a, A>
where
    A: Clone + 'a,
    B: Clone + 'a,
{
    type Output = Parser<'a, (A, B)>;

    fn add(self, parser2: Parser<'a, B>) -> Self::Output {
        ParserMethods::and(self, parser2)
    }
}
