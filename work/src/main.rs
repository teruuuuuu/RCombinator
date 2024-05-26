

fn main() {
    println!("hello");
}

#[test]
fn test() {
    println!("test")
}

#[derive(Debug, Clone)]
pub struct ParseError<'a> {
    pub location: usize,
    pub label: &'a str,
    pub message: &'a str,
    pub child: Vec<ParseError<'a>>
}

#[derive(Debug, Clone)]
pub struct ParseContext<'a> {
    pub label: &'a str,
    pub location: usize,
    pub parse_error_opt: Option<ParseError<'a>>
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

    pub fn location_by_error(&mut self, parse_error: &'a ParseError<'a>) -> Self {
        let mut child = self.child.clone();
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

impl <'a>ParseContext<'a> {
    pub fn new_context(location: usize) -> Self {
        ParseContext {
            label: "",
            location,
            parse_error_opt: Option::None
        }
    }

    pub fn new_labeled_context(label: &'a str, location: usize) -> Self {
        ParseContext {
            label: label,
            location,
            parse_error_opt: Option::None
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

}

#[test]
fn test1() {

}