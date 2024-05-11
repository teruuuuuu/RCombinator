#[derive(Clone, PartialEq, Debug)]
pub enum ParseResult<A> {
    Success { value: A },
    Failure { },
}

impl<A> ParseResult<A> {
    pub fn successful(value: A) -> Self {
        ParseResult::Success { value }
    }

    pub fn failure() -> Self {
        ParseResult::Failure {  }
    }

    pub fn map<B, F>(self, f: F) -> ParseResult<B>
    where
        F: Fn(A) -> B,
    {
        match self {
            ParseResult::Success { value } => ParseResult::successful(f(value)),
            ParseResult::Failure {  } => {
                ParseResult::Failure {}
            }
        }
    }
}

