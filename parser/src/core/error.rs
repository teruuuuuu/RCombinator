use std::fmt;


#[derive(Debug, Clone)]
pub enum MyError {   
    IO(String),
    Common(String)
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IO(message) => {
                write!(f, "io error message[{message}]")
            },
            MyError::Common(message) => {
                write!(f, "error message[{message}]")
            }
        }
    }
}