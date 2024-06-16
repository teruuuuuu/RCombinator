use crate::core::parse_result::*;
use crate::core::parser::*;
use crate::core::parser_methods::ParserMethods;
use crate::prelude::parse_error::ParseError;

const CR_CODE: u8 = 13;

const CR_CHAR: char = '\u{d}';
const LF_CODE: u8 = 10;
const LF_CHAR: char = '\u{a}';
const SPACE_CODE: u8 = b' ';
const SPACE_CHAR: char = '\u{20}';
const TAB_CODE: u8 = b'\t';
const TAB_CHAR: char = '\u{9}';
const ZERO_CODE: u8 = b'0';
const NINE_CODE: u8 = b'9';
const MINUS_CODE: u8 = b'-';
const MINUS_CHAR: char = '-';
const PLUS_CODE: u8 = b'+';
const PLUS_CHAR: char = '+';

pub fn array<'a, A, B, C, D>(
    parser: Parser<'a, A>,
    left_bracket: Parser<'a, B>,
    right_bracket: Parser<'a, C>,
    separator: Parser<'a, D>,
) -> Parser<'a, Vec<A>> 
where
    A: 'a + Clone,
    B: 'a + Clone,
    C: 'a + Clone,
    D: 'a + Clone
{
    let inner_parser = Parser::new(move |input, location| {
        let mut current_location = location;
        let mut vec = Vec::new();
        if let ParseResult::Success { value, location } = parser.parse(input, current_location) {
            vec.push(value);
            current_location = location;
        } else {
            return ParseResult::Success { value: vec, location: current_location };
        }
        let mut location_tmp;
        loop {
            if let ParseResult::Success { value:_, location } = separator.parse(input, current_location) {
                location_tmp = current_location;
                current_location = location;
            } else {
                return ParseResult::Success { value: vec, location: current_location };
            }
            if let ParseResult::Success { value, location } = parser.parse(input, current_location) {
                vec.push(value);
                current_location = location;
            } else {
                return ParseResult::Success { value: vec, location: location_tmp };
            }
        }
    });

    left_bracket.skip_left(inner_parser).skip_right(right_bracket)
}

#[test]
fn test_array() {
    let parser = array(number_i32().with_skip_space(), char('[').with_skip_space(), char(']').with_skip_space(),char(',').with_skip_space());
    let r = parser.parse("[ 123 , 456 , 789 ]", 0);
    println!("{:?}", r);

    let parser = array(number_i32(), char(','), char(','),char(','));
    let r = parser.parse(",123,456,789,", 0);
    println!("{:?}", r);

}

pub fn break_line<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, location| {
        if input[location..].starts_with(CR_CHAR) {
            if input[location+1..].starts_with(LF_CHAR) {
                ParseResult::Success {
                    value: (),
                    location: location + 2,
                }
            } else {
                ParseResult::Success {
                    value: (),
                    location: location + 1,
                }
            }
        } else if input[location..].starts_with(LF_CHAR) {
            ParseResult::Success {
                value: (),
                location: location + 1,
            }
        } else {
            ParseResult::Failure {
                parse_error: ParseError::new(
                    "break_line".to_string(),
                    "not break line".to_string(),
                    location,
                    vec![]

                ),
                location: location
            }
        }
    })
}

#[test]
fn test_break_line() {
    let break_line_parser = break_line();
    match break_line_parser.parse("abc\ndef", 3) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 4);
        }
        ParseResult::Failure { parse_error, location } => {
            assert!(false);
        }
    }
}

pub fn char<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |input, location| {
        let check = input.chars().nth(location).map(|f| f == c);

        if check.unwrap_or(false) {
            ParseResult::Success { value: c, location: location + 1}
        } else {
            ParseResult::Failure {
                parse_error: ParseError {
                    label: "char".to_string(),
                    message: "not match".to_string(),
                    location,
                    children: vec![]
                },
                location,
            }
        }
    })
}

#[test]
fn test_char() {
    let parser = char('a') + char('b');
    match parser.parse("abcd", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(('a', 'b'), value);
            assert_eq!(2, location)
        }
        _ => assert!(false),
    }

    let parser = char('a').seq1() + end();
    match parser.parse("aaaa", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!((vec!['a', 'a', 'a', 'a'], ()), value);
            assert_eq!(4, location);
        }
        _ => assert!(false),
    }

    let parser = (char('🍣') | char('🍺')).seq0() + end();
    match parser.parse("🍣🍣🍺🍺🍣🍺🍺", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!((vec!['🍣', '🍣', '🍺', '🍺', '🍣', '🍺', '🍺'], ()), value);
            assert_eq!(7, location);
        }
        _ => assert!(false),
    }

}

pub fn end<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, location| {
        if input.chars().nth(location).is_none() {
            ParseResult::Success {
                value: (),
                location,
            }
        } else {
            ParseResult::Failure {
                parse_error: ParseError::new("end".to_string(), "not end".to_string(), location, vec![]),
                location,
            }
        }
    })
}

pub fn head<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, location| {
        if location == 0 {
            ParseResult::Success {
                value: (),
                location,
            }
        } else {
            if location > 1 && input[location-1..].starts_with(LF_CHAR) {
                ParseResult::Success {
                    value: (),
                    location,
                }
            } else {
                ParseResult::failure(ParseError::new("head".to_string(), "not head".to_string(), location, vec![]), location)
            }
        }
    })
}

pub fn lazy<'a, A, F>(f: F) -> Parser<'a, A>
where
    F: Fn() -> Parser<'a, A> + 'a,
    A: 'a + Clone,
{
    unit().flat_map(move |_| f())
}

#[test]
fn lazy_test() {
    let a_parser = char('a');
}

pub fn literal<'a>(v: &'a str) -> Parser<'a, &str> {
    Parser::new(move |input, location| {
        let bytes = v.as_bytes();
        let len = bytes.len();

        if input[location..].starts_with(v) {
            ParseResult::Success {
                value: v,
                location: location + len,
            }
        } else {
            ParseResult::failure(ParseError::new("literal".to_string(), "not match".to_string(), location, vec![]), location)
        }
    })
}

#[test]
fn test_literal() {
    let parser = head() + literal("stuvwxyz.");
    match parser.parse("abc\ndef\nstuvwxyz.", 8) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(((), "stuvwxyz."), value);
            assert_eq!(17, location);
        }
        _ => assert!(false),
    }
}

pub fn number_i32<'a>() -> Parser<'a, i32> {
    Parser::new(move |input, location| {
        let mut sign = 1;
        let mut number = 0;
        let mut current_location = 0;

        let from_location_bytes = input[location..].as_bytes();
        let from_location_len = from_location_bytes.len();
        if from_location_len <= current_location {
            return ParseResult::failure(
                ParseError::new("number".to_string(), "not number".to_string(), location, vec![]),
                current_location);
        }

        if from_location_bytes[current_location] == MINUS_CODE {
            sign = -1;
            current_location += 1;
        } else if from_location_bytes[current_location] == PLUS_CODE {
            sign = 1;
            current_location += 1;
        }

        if from_location_len < current_location ||
                (from_location_bytes[current_location] < ZERO_CODE || from_location_bytes[current_location] > NINE_CODE) {
            return ParseResult::failure(
                ParseError::new("number".to_string(), "not number".to_string(), location, vec![]),
                current_location);
        }
        loop {
            if from_location_len <= current_location ||
                (from_location_bytes[current_location] < ZERO_CODE || from_location_bytes[current_location] > NINE_CODE) {
                break
            } else {
                number = number * 10 + ((from_location_bytes[current_location] - ZERO_CODE) as i32);
                current_location += 1;
            }
        }
        ParseResult::successful(sign * number, location + current_location)
    })
}

#[test]
fn test_number_i32() {
    let parser = number_i32();
    if let ParseResult::Success { value, location } = parser.parse("1234", 0) {
        assert_eq!(value, 1234);
        assert_eq!(location, 4);
    } else {
        assert!(false)
    }

}

pub fn space<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, location| {
        if input[location..].starts_with(SPACE_CHAR) {
            ParseResult::Success {
                value: (),
                location: location + 1,
            }
        } else if input[location..].starts_with(TAB_CHAR) {
            ParseResult::Success {
                value: (),
                location: location + 1,
            }
        } else {
            ParseResult::failure(ParseError::new("space".to_string(), "not space".to_string(), location, vec![]), location)
        }
    })
}

#[test]
fn test_space() {
    let break_line_parser = space();
    match break_line_parser.parse("abc def", 3) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 4);
        }
        ParseResult::Failure { parse_error, location } => {
            assert!(false);
        }
    }
    match break_line_parser.parse("abc\tdef", 3) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 4);
        }
        ParseResult::Failure { parse_error, location } => {
            assert!(false);
        }
    }
}

pub fn space_or_line<'a>() -> Parser<'a, ()> {
    space().or(break_line())
}

pub fn space_or_line_seq<'a>() -> Parser<'a, ()> {
    space().or(break_line()).seq0().map(move |_| ())
}

#[test]
fn test_space_or_line_seq() {
    let parser = literal("abc") + space_or_line_seq() + literal("def");
    match parser.parse("abc  \t \r\n\ndef", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 13);
            assert_eq!(value.0 .0, "abc");
            assert_eq!(value.0 .1, ());
            assert_eq!(value.1, "def");
        }
        ParseResult::Failure { parse_error, location } => {
            assert!(false);
        }
    }
}

pub fn unit<'a>() -> Parser<'a, ()>
where {
    Parser::new(move |parser_input, location| ParseResult::Success {
        value: (),
        location,
    })
}

#[test]
fn test_unit() {
    let parser = unit()
        .flat_map(move |v| Parser::new(|input, location| ParseResult::successful("aaa", location)));
    match parser.parse("abc  \t \r\n\ndef", 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 0);
            assert_eq!(value, "aaa");
        }
        ParseResult::Failure { parse_error, location } => {
            assert!(false);
        }
    }
}
