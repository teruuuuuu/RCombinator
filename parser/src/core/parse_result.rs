#[derive(Clone, PartialEq, Debug)]
pub enum ParseResult<A> {
    Success { value: A, location: usize },
    Failure { message: String, location: usize },
}

impl<A> ParseResult<A> {
    pub fn successful(value: A, location: usize) -> Self {
        ParseResult::Success { value, location }
    }

    pub fn failure(message: String, location: usize) -> Self {
        ParseResult::Failure { message, location }
    }

    pub fn map<B, F>(self, f: F) -> ParseResult<B>
    where
        F: Fn(A) -> B,
    {
        match self {
            ParseResult::Success { value, location } => ParseResult::successful(f(value), location),
            ParseResult::Failure { message, location } => {
                ParseResult::Failure { message, location }
            }
        }
    }
}
