use std::vec;

use crate::core::parse_result::*;
use crate::core::parser::*;
use crate::core::parser_input::ParserInput;
use crate::core::parser_methods::ParserMethods;

const CR_CODE: u8 = 13;
const LF_CODE: u8 = 10;

pub fn char<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        // 最大4バイト
        let mut tmp = [0u8; 4];
        let bytes = c.encode_utf8(&mut tmp).as_bytes();;
        let len = bytes.len();

        match input.read_by_size(location, len) {
            Result::Ok(txt) => {
                if txt == bytes {
                    ParseResult::Success { 
                        value: c, 
                        location: location + len 
                    }
                } else {
                    ParseResult::Failure {
                        message: format!("not match "),
                        location
                    }
                }
            }, 
            Result::Err(err) => {
                ParseResult::Failure {
                    message: format!("not match "),
                    location
                }
            }
        }
    })
}

pub fn end<'a>() -> Parser<'a, ()> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        if input.has_more(location) {
            ParseResult::Failure {
                message: format!("not end current location{:?}, substring{:?}", location, input.read_line(location)),
                location,
            }
        } else {
            ParseResult::Success { value: (), location }
        }
    })
}

pub fn head<'a>() -> Parser<'a, ()> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        if location == 0 {
            ParseResult::Success { value: (), location }
        } else {
            match input.read_by_size(location - 1, 2) {
                Result::Ok(bytes) => {
                    if bytes[0] == CR_CODE || bytes[0] == LF_CODE {
                        ParseResult::Success { value: (), location }
                    } else {
                        ParseResult::Failure {
                            message: format!("not head"),
                            location,
                        }
                    }
                },
                Result::Err(_err) => {
                    ParseResult::Failure {
                        message: format!("read failed"),
                        location,
                    }
                }
            }
        }
    })
}

pub fn literal<'a>(v: &'a str) -> Parser<'a, &str> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        let bytes = v.as_bytes();
        let len = bytes.len();

        match input.read_by_size(location, len) {
            Result::Ok(reads) => {
                if reads == bytes {
                    ParseResult::Success { 
                        value: v, 
                        location: location + len
                    }
                } else {
                    ParseResult::Failure {
                        message: format!("text not equal."),
                        location,
                    }
                }
            },
            Result::Err(_err) => {
                ParseResult::Failure {
                    message: format!("read error location[{}] len[{}]", location, len),
                    location,
                }
            }
        }
    })
}


#[test]
fn test_char() {
    let parser = char('a') + char('b');

    match parser.parse(&mut ParserInput::text("abcd"), 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(('a', 'b'), value);
            assert_eq!(2, location)
        }
        _ => assert!(false),
    }

    let parser = char('a').seq1() + end();
    match parser.parse(&mut ParserInput::text("aaaa"), 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!((vec!['a', 'a', 'a', 'a'], ()), value);
            assert_eq!(4, location);
        }
        _ => assert!(false),
    }

    let parser = (char('🍣') | char('🍺')).seq0() + end();
    match parser.parse(&mut ParserInput::text("🍣🍣🍺🍺🍣🍺🍺"), 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!((vec!['🍣', '🍣', '🍺', '🍺', '🍣', '🍺', '🍺'], ()), value);
            assert_eq!(28, location);
        }
        _ => assert!(false),
    }
    
}

#[test]
fn test_literal() {
    let parser = head() + literal("stuvwxyz.");
    match parser.parse(&mut ParserInput::text("abc\ndef\nstuvwxyz."), 8) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(((), "stuvwxyz."), value);
            assert_eq!(17, location);
        }
        _ => assert!(false),
    }
}
