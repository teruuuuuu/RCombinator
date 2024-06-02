#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
    pub location: usize,
    pub label: String,
    pub message: String
}

impl ParseError {
    pub fn new(location: usize, label: String, message: String) -> Self {
        ParseError {
            location,
            label,
            message
        }
    }
}

