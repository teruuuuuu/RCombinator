use crate::core::parse_result::*;
use crate::core::parser::*;
use crate::core::parser_methods::ParserMethods;

pub fn char<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |input: &str, location: usize| {
        let chars = input.chars();
        let count = chars.count();
        if location >= count {
            ParseResult::Failure {
                message: format!("index invalid "),
                location,
            }
        } else {
            match input.chars().nth(location) {
                Some(d) if c == d => ParseResult::Success {
                    value: c,
                    location: location + 1,
                },
                _ => ParseResult::Failure {
                    message: format!("not match "),
                    location,
                },
            }
        }
    })
}

pub fn end<'a>() -> Parser<'a, ()> {
    Parser::new(move |input: &str, location: usize| {
        if input.len() == location {
            ParseResult::Success {
                value: (),
                location,
            }
        } else {
            ParseResult::Failure {
                message: format!("not end current location{:?}, expected locatoin{:?}", location, input.len()),
                location,
            }
        }
    })
}

pub fn literal<'a>(v: &'a str) -> Parser<'a, &str> {
    Parser::new(move |input: &str, location: usize| {
        if input[location..].starts_with(v) {
            ParseResult::Success {
                value: v,
                location: location + v.len(),
            }
        } else {
            ParseResult::Failure {
                message: format!("not match "),
                location,
            }
        }
    })
}



#[test]
fn test_char() {
    let p1 = char('a');
    let p2 = char('b');
    let p3 = p1 + p2;

    match p3.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
        }
        _ => assert!(false),
    }

    match p3.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
        }
        _ => assert!(false),
    }

    let a = char('a');

    let b = a.pure('b');
    let d = b.pure("kkk");

    match d.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
        }
        _ => assert!(false),
    }

    let p1 = char('🍣');
    match p1.parse("🍣abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_end() {
    let parser = literal("abcdefg").skip_right(end());
    match parser.parse("abcdefg", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(value, "abcdefg");
            assert_eq!(location, "abcdefg".len());
        }
        _ => assert!(false),
    }

}

#[test]
fn test_literal() {
    let parser = literal("abcdefg");
    match parser.parse("abcdefghi", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(value, "abcdefg");
            assert_eq!(location, "abcdefg".len());
        }
        _ => assert!(false),
    }

    let parser = literal("abcdefghi");
    match parser.parse("abcdefg", 0) {
        ParseResult::Failure { message, location } => {
            assert!(true);
        }
        _ => assert!(false),
    }
}

