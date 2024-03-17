use crate::core::parse_result::*;
use crate::core::parser::*;
use crate::core::parser_methods::ParserMethods;

pub fn char<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |input: &str, location: usize| {
        let chars = input.chars();
        let count = chars.count();
        if location >= count {
            ParseResult::Failure { message: format!("index invalid "), location }
        } else {
            match input.chars().nth(location) {
                Some(d) if c == d => ParseResult::Success { value: c, location: location + 1 },
                _ => ParseResult::Failure { message: format!("not match "), location }
            }    
        }
    })
}


#[test]
fn test_char_parser() {
    let p1 = char('a');
    let p2 = char('b');
    let p3 = p1 + p2;
    
    match p3.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            println!("{}:{}", value.0, value.1)
        },
        _ => assert!(false)
    }

    match p3.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            println!("{}:{}", value.0, value.1)
        },
        _ => assert!(false)
    }

    let a = char('a');
    
    let b = a.pure('b');
    let d = b.pure("kkk");

    match d.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            println!("{}", value)
        },
        _ => assert!(false)
    }


}