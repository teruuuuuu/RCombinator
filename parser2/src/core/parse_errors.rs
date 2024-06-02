use crate::core::parse_error::ParseError;

use super::parse_error;

#[derive(Debug, Clone)]
pub struct ParseErrors<'a> {
    pub errors: Vec<ParseError<'a>>
}

impl <'a>ParseContext<'a> {
    pub fn new_context(location: usize) -> Self {
        ParseContext {
            label: "",
            location,
            parse_errors: Vec::new()
        }
    }

    pub fn new_labeled_context(label: &'a str, location: usize) -> Self {
        ParseContext {
            label: label,
            location,
            parse_errors: Vec::new()
        }
    }

    pub fn new_error(&mut self, label: &'a str, message: &'a str) -> Self {
        ParseContext {
            label: self.label,
            location: self.location,
            parse_error_opt: Option::Some(ParseError::new_error(self.location, label, message))
        }
        
    }

    pub fn add_error(&mut self, context: ParseContext<'a>) -> Self {
        if self.parse_error_opt.is_some() && context.parse_error_opt.is_some() {
            let parse_error = context.parse_error_opt.unwrap();
            if self.label.eq("") || self.label.eq(parse_error.label) {
                ParseContext {
                    label: self.label,
                    location: std::cmp::max(self.location, parse_error.location),
                    parse_error_opt: Option::Some(self.parse_error_opt.as_mut().unwrap().add_error(parse_error))
                }
            } else {
                ParseContext {
                    label: self.label,
                    location: self.location,
                    parse_error_opt: Option::Some(self.parse_error_opt.as_mut().unwrap().location_change(std::cmp::max(self.location, parse_error.location)))
                }    
            }
        } else {
            if self.parse_error_opt.is_some() {
                ParseContext {
                    label: self.label,
                    location: self.location,
                    parse_error_opt: self.parse_error_opt.as_ref().map(|e| e.clone())
                }
            } else {
                ParseContext {
                    label: self.label,
                    location: self.location,
                    parse_error_opt: self.parse_error_opt.as_ref().map(|e| e.clone())
                }
            }
        }
    }

    pub fn add_error2(&mut self, parse_error: ParseError<'a>) -> Self {

        
        if self.label.eq("") || self.label.eq(parse_error.label) {
            ParseContext {
                label: self.label,
                location: std::cmp::max(self.location, parse_error.location),
                parse_error_opt: Option::Some(self.parse_error_opt.as_mut().unwrap().add_error(parse_error))
            }
        } else {
            ParseContext {
                label: self.label,
                location: self.location,
                parse_error_opt: Option::Some(self.parse_error_opt.as_mut().unwrap().location_change(std::cmp::max(self.location, parse_error.location)))
            }    
        }
    }

    pub fn move_location(&mut self, move_size: usize) -> Self {
        ParseContext {
            label: self.label,
            location: self.location + move_size,
            parse_error_opt: self.parse_error_opt.as_ref().map(|e| e.clone())
        }
    }

    pub fn new_location(&mut self, location: usize) -> Self {
        ParseContext {
            label: self.label,
            location,
            parse_error_opt: self.parse_error_opt.as_ref().map(|e| e.clone())
        }
    }

}

impl <'a>std::fmt::Display for ParseContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.parse_error_opt.is_some() {
            write!(f, "(location: {}, label: {}, parseError: [{}])", self.location, self.label, self.parse_error_opt.as_ref().unwrap())?;
        } else {
            write!(f, "(location: {}, label: {})", self.location, self.label)?;
        }
        Ok(())
    }
}

#[test]
pub fn test() {
    let mut a = ParseError::new_error(0, "A", "message A");
    let mut b = ParseError::new_error(1, "B", "message B");
    a = a.add_error(b);
    println!("{}", a);


    let mut parser_context1 = ParseContext::new_labeled_context("label1", 0).new_error("kkk", "def");
    let mut parser_context2 = ParseContext::new_labeled_context("label1", 2).new_error("label1", "ghi");
    parser_context1 = parser_context1.add_error(parser_context2);
    println!("{}", parser_context1);

}