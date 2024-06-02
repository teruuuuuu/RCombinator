use std::fmt::format;

use crate::core::parse_error::ParseError;

use super::parse_error;

#[derive(Debug, Clone)]
pub struct ParseContext {
    pub parse_errors: Vec<ParseError>
}

impl ParseContext {
    pub fn new_context() -> Self {
        ParseContext {
            parse_errors: Vec::new()
        }
    }

    pub fn init_errors(&mut self) {
        self.parse_errors = Vec::new();
    }

    pub fn add_errors(&mut self, parse_error: ParseError) {
        self.parse_errors.push(parse_error);
    }
}

impl std::fmt::Display for ParseContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse_errors: {}", self.parse_errors.iter().map(|v| format!("{:?}", v)).collect::<Vec<String>>().join(",") );
        Ok(())
    }
}

#[test]
pub fn test() {
    

}