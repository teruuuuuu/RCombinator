use crate::core::parser::Parser;
use crate::core::parser_methods::ParserMethods;

use std::ops::BitOr;

impl<'a, A> BitOr for Parser<'a, A>
where
  A: Clone + 'a,
{
  type Output = Self;

  fn bitor(self, parser2: Parser<'a, A>) -> Self::Output {
    self.or(parser2)
  }
}