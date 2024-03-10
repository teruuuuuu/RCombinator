use crate::core::parser::{Parser, ParsersImpl};
use crate::core::parser_methods::ParserMethods;

use std::ops::Add;

impl<'a, A, B> Add<Parser<'a, B>> for Parser<'a, A>
where
  A: Clone + 'a,
  B: Clone + 'a,
{
  type Output = Parser<'a, (A, B)>;

  fn add(self, rhs: Parser<'a, B>) -> Self::Output {
    ParsersImpl::and(self, rhs)
  }
}
