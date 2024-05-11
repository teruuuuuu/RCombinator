#[derive(Debug, Clone)]
pub struct ParseError<'a> {
    pub location: usize,
    pub label: &'a str,
    pub message: &'a str,
    pub child: Vec<&'a ParseError<'a>>
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

    pub fn add_error(&mut self, parse_error: &'a ParseError<'a>) -> Self {
        
        let mut child = Vec::new();
        self.child.iter().for_each(|c| {
            child.push(*c);
        });
        child.push(&parse_error);
        ParseError {
            location: std::cmp::max(self.location, parse_error.location),
            label: self.label,
            message: self.message,
            child
        }
    }

    pub fn location_change(&mut self, location: usize) -> Self {
        let mut child = Vec::new();
        self.child.iter().for_each(|c| {
            child.push(*c);
        });
        ParseError {
            location: location,
            label: self.label,
            message: self.message,
            child
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