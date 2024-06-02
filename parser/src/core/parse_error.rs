#[derive(Debug, Clone)]
pub struct ParseError<'a> {
    pub location: usize,
    pub label: &'a str,
    pub message: &'a str,
    pub child: Vec<ParseError<'a>>
}

impl <'a>ParseError<'a> {
    pub fn new_error(location: usize, label: &'a str, message: &'a str) -> Self {
        ParseError {
            location,
            label,
            message,
            child: Vec::new()
        }
    }

    pub fn add_error(&mut self, parse_error: ParseError<'a>) -> Self {        
        let mut child = self.child.clone();
        let location = parse_error.location;
        child.push(parse_error);
        ParseError {
            location: std::cmp::max(self.location, location),
            label: self.label,
            message: self.message,
            child
        }
    }

    pub fn location_by_error(&mut self, parse_error: ParseError<'a>) -> Self {
        let child = self.child.clone();
        ParseError {
            location: std::cmp::max(self.location, parse_error.location),
            label: self.label,
            message: self.message,
            child
        }
    }


    pub fn location_change(&mut self, location: usize) -> Self {
        ParseError {
            location: location,
            label: self.label,
            message: self.message,
            child: self.child.clone()
        }
    }
}

impl <'a>std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let b = self.child.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",");
        write!(f, "(location: {}, label: {}, message: {}, child: [{}])", self.location, self.label, self.message, b)?;
        Ok(())
    }
}



#[test]
fn test() {
    use crate::core::parser_methods::ParserMethods;
    use crate::core::common_parsers;
    use crate::core::parse_context::ParseContext;
    use crate::core::parser_input::ParserInput;
    use crate::core::parse_result::ParseResult;
    use crate::core::parser::ParserTrait;

    let p1 = common_parsers::char('a');
    let parser = p1.seq0();


    match parser.parse(&mut ParserInput::text("aaab"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 3);
            assert_eq!(value, vec!['a','a','a']);
            assert!(next_context.parse_error_opt.is_some());
        }
        (_, ParseResult::Failure { }) => {
            assert!(false);
        }
    }
}