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
        let mut new_context;
        let mut vec = Vec::new();

        let (next_contet, parse_result) = parser.parse(input, current_context);
        match parse_result {
            ParseResult::Success { value } => {
                vec.push(value);
                new_context = next_contet;
                current_context = &mut new_context;
            }
            ParseResult::Failure { } => {
                return (next_contet, ParseResult::failure());
            }
        }


        loop {
            let (mut next_context1, parse_result1) = separator.parse(input, &mut current_context.clone());
            match parse_result1 {
                ParseResult::Success { value } => {},
                ParseResult::Failure { } => {
                    return (current_context.clone(), ParseResult::successful(vec));
                }
            }
            let (next_context2, parse_result2) = parser.parse(input, &mut next_context1);
            match parse_result2 {
                ParseResult::Success { value } => {
                    vec.push(value);
                    new_context = next_context2;
                    current_context = &mut new_context;
                }
                ParseResult::Failure { } => {
                    return (current_context.clone(), ParseResult::successful(vec));
                }
            }
        }
    });

    left_bracket.skip_left(inner_parser).skip_right(right_bracket)
}

#[test]
fn test_array() {
    let parser = array(number_i32().with_skip_space(), char('[').with_skip_space(), char(']').with_skip_space(),char(',').with_skip_space());
    let (next_context, r) = parser.parse(&mut ParserInput::text("[ 123 , 456 , 789 ]"), &mut ParseContext::new_context(0));
    println!("{:?}", r);

    let parser = array(number_i32(), char(','), char(','),char(','));
    let (next_context, r) = parser.parse(&mut ParserInput::text(",123,456,789,"), &mut ParseContext::new_context(0));
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
    Parser::new(move |input, context| {
        // 最大4バイト
        let mut tmp = [0u8; 4];
        let bytes = c.encode_utf8(&mut tmp).as_bytes();
        let len = bytes.len();

        match input.read_by_size(context.location, len) {
            Result::Ok(txt) => {
                if txt == bytes {
                    (context.move_location(len),ParseResult::successful(c))
                } else {
                    (context.new_error("char", "not match"), ParseResult::failure())
                }
            }
            Result::Err(err) => (context.new_error("char", "read error"), ParseResult::failure())
        }
    })
}

#[test]
fn test_char() {
    let parser = char('a') + char('b');

    match parser.parse(&mut ParserInput::text("abcd"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            assert!(true);
            assert_eq!(('a', 'b'), value);
            assert_eq!(2, next_context.location)
        }
        _ => assert!(false),
    }

    let parser = char('a').seq1() + end();
    match parser.parse(&mut ParserInput::text("aaaa"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            assert!(true);
            assert_eq!((vec!['a', 'a', 'a', 'a'], ()), value);
            assert_eq!(4, next_context.location);
        }
        _ => assert!(false),
    }

    let parser = (char('🍣') | char('🍺')).seq0() + end();
    match parser.parse(&mut ParserInput::text("🍣🍣🍺🍺🍣🍺🍺"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            assert!(true);
            assert_eq!((vec!['🍣', '🍣', '🍺', '🍺', '🍣', '🍺', '🍺'], ()), value);
            assert_eq!(28, next_context.location);
        }
        _ => assert!(false),
    }
}

pub fn end<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, context| {
        if input.has_more(context.location) {
            (context.new_error("end", "not end"), ParseResult::failure())

        } else {
            (context.move_location(0), ParseResult::successful(()))
        }
    })
}

pub fn head<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, context| {
        if context.location == 0 {
            (context.move_location(0), ParseResult::successful(()))
        } else {
            match input.read_by_size(context.location - 1, 2) {
                Result::Ok(bytes) => {
                    if bytes[0] == CR_CODE || bytes[0] == LF_CODE {
                        (context.move_location(0), ParseResult::successful(()))
                    } else {
                        (context.new_error("head", "not head"), ParseResult::failure())
                    }
                }
                Result::Err(_err) => {
                    (context.new_error("head", "read failed"), ParseResult::failure())
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
    Parser::new(move |input, context| {
        let bytes = v.as_bytes();
        let len = bytes.len();

        match input.read_by_size(context.location, len) {
            Result::Ok(reads) => {
                if reads == bytes {
                    (context.move_location(len), ParseResult::successful(v))
                } else {
                    (context.new_error("literal", "text not equal"), ParseResult::failure())
                }
            }
            Result::Err(_err) => {
                (
                    context.new_error(
                        "literal", 
                        "read error"), 
                    ParseResult::failure())
            },
        }
    })
}

#[test]
fn test_literal() {
    let parser = head() + literal("stuvwxyz.");
    match parser.parse(&mut ParserInput::text("abc\ndef\nstuvwxyz."), &mut ParseContext::new_context(8)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(((), "stuvwxyz."), value);
            assert_eq!(17, next_context.location);
        }
        _ => assert!(false),
    }
}

pub fn number_i32<'a>() -> Parser<'a, i32> {
    Parser::new(move |input, context| {
        let mut sign = 1;
        let mut number = 0;
        let mut current_location = context.location;

        let mut read;
        match input.read_by_size(current_location, 1) {
            Result::Ok(r) => {
                read = r;
            },
            Result::Err(_) => {
                return (context.new_error("number", "not number"), ParseResult::failure());
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
                return (context.new_error("number", "not number"), ParseResult::failure());
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
            (context.new_location(current_location), ParseResult::successful(sign * number))
        } else {
            (context.new_error("number", "not number"), ParseResult::failure())
        }
    })
}

#[test]
fn test_number_i32() {
    let parser = number_i32();
    let (next_context, parse_result) = parser.parse(&mut ParserInput::text("1234"), &mut ParseContext::new_context(0));
    match parse_result {
        ParseResult::Success { value } => {
            assert_eq!(value, 1234);
            assert_eq!(next_context.location, 4);
        },
        _ => {
            assert!(false)
        }
    }
}

pub fn space<'a>() -> Parser<'a, ()> {
    Parser::new(move |input, context| {
        match input.read_by_size(context.location, 1) {
            Result::Ok(read) => match read[0] {
                SPACE_CODE | TAB_CODE => {
                    (context.move_location(1), ParseResult::successful(()))
                },
                _ => {
                    (context.new_error("space", "not space"), ParseResult::failure())
                },
            },
            Result::Err(_error) => {
                (context.new_error("space", "read failed"), ParseResult::failure())
            },
        }
    })
}

#[test]
fn test_space() {
    let break_line_parser = space();
    match break_line_parser.parse(&mut ParserInput::text("abc def"), &mut ParseContext::new_context(3)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 4);
        }, 
        _ => {
            assert!(false);
        }
    }
    match break_line_parser.parse(&mut ParserInput::text("abc\tdef"), &mut ParseContext::new_context(3)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 4);
        }
        _ => {
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
    match parser.parse(&mut ParserInput::text("abc  \t \r\n\ndef"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 13);
            assert_eq!(value.0 .0, "abc");
            assert_eq!(value.0 .1, ());
            assert_eq!(value.1, "def");
        }
        _ => {
            assert!(false);
        }
    }
}

pub fn unit<'a>() -> Parser<'a, ()>
where {
    Parser::new(move |input, context|  {
        (context.move_location(0), ParseResult::successful(()))
    })
}

#[test]
fn test_unit() {
    let parser = unit()
        .flat_map(move |v| Parser::new(|input, context| (context.move_location(0), ParseResult::successful("aaa"))));
    match parser.parse(&mut ParserInput::text("abc  \t \r\n\ndef"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value }) => {
            assert!(true);
            assert_eq!(next_context.location, 0);
            assert_eq!(value, "aaa");
        }
        _ => {
            assert!(false);
        }
    }
}
