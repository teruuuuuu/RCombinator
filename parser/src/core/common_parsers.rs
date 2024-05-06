use crate::core::parse_result;
use crate::core::parse_result::*;
use crate::core::parser::*;
use crate::core::parser_input::ParserInput;
use crate::core::parser_methods::ParserMethods;

use super::parse_context::ParseContext;

const CR_CODE: u8 = 13;
const LF_CODE: u8 = 10;
const SPACE_CODE: u8 = b' ';
const TAB_CODE: u8 = b'\t';
const ZERO_CODE: u8 = b'0';
const NINE_CODE: u8 = b'9';
const MINUS_CODE: u8 = b'-';
const PLUS_CODE: u8 = b'+';

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
    let inner_parser = Parser::new(move |input, context| {
        let mut current_context = context;
        let mut vec = Vec::new();

        let (next_contet, parse_result) = parser.parse(input, current_context);
        match parse_result {
            ParseResult::Success { value } => {
                vec.push(value);
                current_context = &mut next_contet.clone();
            }
            ParseResult::Failure { } => {
                return (next_contet, ParseResult::failure());
            }
        }

        loop {
            let (next_context1, parse_result1) = separator.parse(input, &mut current_context.clone());
            match parse_result1 {
                ParseResult::Success { value } => {

                }
                ParseResult::Failure { } => {
                    return (next_contet, ParseResult::successful(vec));
                }
            }
            let (next_context2, parse_result2) = parser.parse(input, &mut next_context1);
            match parse_result2 {
                ParseResult::Success { value } => {
                    vec.push(value);
                    current_context = &mut next_context2.clone();
                }
                ParseResult::Failure { } => {
                    return (*current_context, ParseResult::successful(vec));
                }
            }
        }
    });

    left_bracket.skip_left(inner_parser).skip_right(right_bracket)
}

#[test]
fn test_array() {
    let parser = array(number_i32().with_skip_space(), char('[').with_skip_space(), char(']').with_skip_space(),char(',').with_skip_space());
    let r = parser.parse(&mut ParserInput::text("[ 123 , 456 , 789 ]"), 0);
    println!("{:?}", r);

    let parser = array(number_i32(), char(','), char(','),char(','));
    let r = parser.parse(&mut ParserInput::text(",123,456,789,"), 0);
    println!("{:?}", r);

}

pub fn break_line<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, context| {
        match input.read_by_size(context.location, 1) {
            Result::Ok(read1) => match read1[0] {
                CR_CODE => match input.read_by_size(context.location + 1, 1) {
                    Result::Ok(read2) => match read2[0] {
                        LF_CODE => (context.move_location(2),ParseResult::successful(())),
                        _ => (context.move_location(1),ParseResult::successful(()))
                    }
                    _ => (context.move_location(1),ParseResult::successful(()))
                },
                LF_CODE => (context.move_location(1),ParseResult::successful(())),
                _ => (context.new_error("break_line", "read failed"), ParseResult::failure()),
            },
            Result::Err(_error) => (context.new_error("break_line", "read failed"), ParseResult::failure())
        }
    })
}

#[test]
fn test_break_line() {
    let break_line_parser = break_line();
    match break_line_parser.parse(&mut ParserInput::text("abc\ndef"), &mut ParseContext::new_context(3)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 4);
        }
        (next_context, ParseResult::Failure { }) => {
            assert!(false);
        }
    }
}

pub fn char<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        // 最大4バイト
        let mut tmp = [0u8; 4];
        let bytes = c.encode_utf8(&mut tmp).as_bytes();
        let len = bytes.len();

        match input.read_by_size(location, len) {
            Result::Ok(txt) => {
                if txt == bytes {
                    ParseResult::Success {
                        value: c,
                        location: location + len,
                    }
                } else {
                    ParseResult::Failure {
                        message: format!("not match "),
                        location,
                    }
                }
            }
            Result::Err(err) => ParseResult::Failure {
                message: format!("not match "),
                location,
            },
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

pub fn end<'a>() -> Parser<'a, ()> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        if input.has_more(location) {
            ParseResult::Failure {
                message: format!(
                    "not end current location{:?}, substring{:?}",
                    location,
                    input.read_line(location)
                ),
                location,
            }
        } else {
            ParseResult::Success {
                value: (),
                location,
            }
        }
    })
}

pub fn head<'a>() -> Parser<'a, ()> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        if location == 0 {
            ParseResult::Success {
                value: (),
                location,
            }
        } else {
            match input.read_by_size(location - 1, 2) {
                Result::Ok(bytes) => {
                    if bytes[0] == CR_CODE || bytes[0] == LF_CODE {
                        ParseResult::Success {
                            value: (),
                            location,
                        }
                    } else {
                        ParseResult::Failure {
                            message: format!("not head"),
                            location,
                        }
                    }
                }
                Result::Err(_err) => ParseResult::Failure {
                    message: format!("read failed"),
                    location,
                },
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
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        let bytes = v.as_bytes();
        let len = bytes.len();

        match input.read_by_size(location, len) {
            Result::Ok(reads) => {
                if reads == bytes {
                    ParseResult::Success {
                        value: v,
                        location: location + len,
                    }
                } else {
                    ParseResult::Failure {
                        message: format!("text not equal."),
                        location,
                    }
                }
            }
            Result::Err(_err) => ParseResult::Failure {
                message: format!("read error location[{}] len[{}]", location, len),
                location,
            },
        }
    })
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

pub fn number_i32<'a>() -> Parser<'a, i32> {
    Parser::new(move |input, location| {
        let mut sign = 1;
        let mut number = 0;
        let mut current_location = location;

        let mut read;
        match input.read_by_size(current_location, 1) {
            Result::Ok(r) => {
                read = r;
            },
            Result::Err(_) => {
                return ParseResult::failure("not number".to_owned(), current_location);
            }
        };
        if read == [MINUS_CODE] {
            sign = -1;
            current_location = current_location + 1;
        
        } else if read == [PLUS_CODE] {
            sign = 1;
            current_location = current_location + 1;
        }

        match input.read_by_size(current_location, 1) {
            Result::Ok(r) => {
                read = r;
            },
            Result::Err(_) => {
                return ParseResult::failure("not number".to_owned(), current_location);
            }
        };
        if read[0] >= ZERO_CODE && read[0] <= NINE_CODE {

            loop {
                if read[0] >= ZERO_CODE && read[0] <= NINE_CODE {
                    number = number * 10 + (read[0] - ZERO_CODE) as i32;
                    current_location += 1;
                    match input.read_by_size(current_location, 1) {
                        Result::Ok(r) => {
                            read = r;
                        },
                        Result::Err(_) => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }

            ParseResult::successful(sign * number, current_location)
        } else {
            ParseResult::failure("not number".to_owned(), current_location)
        }
    })
}

#[test]
fn test_number_i32() {
    let parser = number_i32();
    if let ParseResult::Success { value, location } = parser.parse(&mut ParserInput::text("1234"), 0) {
        assert_eq!(value, 1234);
        assert_eq!(location, 4);
    } else {
        assert!(false)
    }

}

pub fn space<'a>() -> Parser<'a, ()> {
    Parser::new(move |input: &mut ParserInput<'a>, location: usize| {
        match input.read_by_size(location, 1) {
            Result::Ok(read) => match read[0] {
                SPACE_CODE => ParseResult::Success {
                    value: (),
                    location: location + 1,
                },
                TAB_CODE => ParseResult::Success {
                    value: (),
                    location: location + 1,
                },
                _ => ParseResult::Failure {
                    message: format!("not format"),
                    location,
                },
            },
            Result::Err(_error) => ParseResult::Failure {
                message: format!("read failed"),
                location,
            },
        }
    })
}

#[test]
fn test_space() {
    let break_line_parser = space();
    match break_line_parser.parse(&mut ParserInput::text("abc def"), 3) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 4);
        }
        ParseResult::Failure { message, location } => {
            assert!(false);
        }
    }
    match break_line_parser.parse(&mut ParserInput::text("abc\tdef"), 3) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 4);
        }
        ParseResult::Failure { message, location } => {
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
    match parser.parse(&mut ParserInput::text("abc  \t \r\n\ndef"), 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 13);
            assert_eq!(value.0 .0, "abc");
            assert_eq!(value.0 .1, ());
            assert_eq!(value.1, "def");
        }
        ParseResult::Failure { message, location } => {
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
    match parser.parse(&mut ParserInput::text("abc  \t \r\n\ndef"), 0) {
        ParseResult::Success { value, location } => {
            assert!(true);
            assert_eq!(location, 0);
            assert_eq!(value, "aaa");
        }
        ParseResult::Failure { message, location } => {
            assert!(false);
        }
    }
}
