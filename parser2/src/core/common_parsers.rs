use std::io::Read;

use crate::core::parse_result;
use crate::core::parse_result::*;
use crate::core::parser::*;
use crate::core::parser_methods::ParserMethods;

use super::parse_context::ParseContext;
use super::parse_error::ParseError;

const CR_CODE: u8 = 13;
const LF_CODE: u8 = 10;
const LF_CHAR: char= '\n';
const SPACE_CODE: u8 = b' ';
const TAB_CODE: u8 = b'\t';
const ZERO_CODE: u8 = b'0';
const NINE_CODE: u8 = b'9';
const MINUS_CODE: u8 = b'-';
const PLUS_CODE: u8 = b'+';
const DOT_CODE: u8 = b'.';

// pub fn array<'a, A, B, C, D>(
//     parser: Parser<'a, A>,
//     left_bracket: Parser<'a, B>,
//     right_bracket: Parser<'a, C>,
//     separator: Parser<'a, D>,
// ) -> Parser<'a, Vec<A>> 
// where
//     A: 'a + Clone,
//     B: 'a + Clone,
//     C: 'a + Clone,
//     D: 'a + Clone
// {
//     let inner_parser = Parser::new(move |input, context| {
//         let mut current_context = context;
//         let mut new_context;
//         let mut vec = Vec::new();

//         let (next_contet, parse_result) = parser.parse(input, current_context);
//         match parse_result {
//             ParseResult::Success { value } => {
//                 vec.push(value);
//                 new_context = next_contet;
//                 current_context = &mut new_context;
//             }
//             ParseResult::Failure { } => {
//                 return (next_contet, ParseResult::successful(vec));
//             }
//         }


//         loop {
//             let (mut next_context1, parse_result1) = separator.parse(input, &mut current_context.clone());
//             match parse_result1 {
//                 ParseResult::Success { value } => {},
//                 ParseResult::Failure { } => {
//                     return (current_context.clone(), ParseResult::successful(vec));
//                 }
//             }
//             let (next_context2, parse_result2) = parser.parse(input, &mut next_context1);
//             match parse_result2 {
//                 ParseResult::Success { value } => {
//                     vec.push(value);
//                     new_context = next_context2;
//                     current_context = &mut new_context;
//                 }
//                 ParseResult::Failure { } => {
//                     return (current_context.clone(), ParseResult::successful(vec));
//                 }
//             }
//         }
//     });

//     left_bracket.skip_left(inner_parser).skip_right(right_bracket)
// }

// #[test]
// fn test_array() {
//     let parser = array(number_i32().with_skip_space(), char('[').with_skip_space(), char(']').with_skip_space(),char(',').with_skip_space());
//     let (next_context, r) = parser.parse(&mut ParserInput::text("[ 123 , 456 , 789 ]"), &mut ParseContext::new_context(0));
//     println!("{:?}", r);

//     let parser = array(number_i32(), char(','), char(','),char(','));
//     let (next_context, r) = parser.parse(&mut ParserInput::text(",123,456,789,"), &mut ParseContext::new_context(0));
//     println!("{:?}", r);

//     let parser = array(number_i32(), char('['), char(']'),char(','));
//     let (next_context, r) = parser.parse(&mut ParserInput::text("[]"), &mut ParseContext::new_context(0));
//     println!("{:?}", r);
// }

// pub fn break_line<'a>() -> Parser<'a, ()> {
//     Parser::new(move |input, context| {
//         match input.read_by_size(context.location, 1) {
//             Result::Ok(read1) => match read1[0] {
//                 CR_CODE => match input.read_by_size(context.location + 1, 1) {
//                     Result::Ok(read2) => match read2[0] {
//                         LF_CODE => (context.move_location(2),ParseResult::successful(())),
//                         _ => (context.move_location(1),ParseResult::successful(()))
//                     }
//                     _ => (context.move_location(1),ParseResult::successful(()))
//                 },
//                 LF_CODE => (context.move_location(1),ParseResult::successful(())),
//                 _ => (context.new_error("break_line", "read failed"), ParseResult::failure()),
//             },
//             Result::Err(_error) => (context.new_error("break_line", "read failed"), ParseResult::failure())
//         }
//     })
// }

// #[test]
// fn test_break_line() {
//     let break_line_parser = break_line();
//     match break_line_parser.parse(&mut ParserInput::text("abc\ndef"), &mut ParseContext::new_context(3)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(next_context.location, 4);
//         }
//         (next_context, ParseResult::Failure { }) => {
//             assert!(false);
//         }
//     }
// }

pub fn char<'a>(c: char) -> Parser<'a, char> {
    Parser::new(move |context, input, location| {
        if input.len() <= location {
            // length over
            (context, ParseResult::failure(ParseError::new(location, "char".to_string(), "length over".to_string()), location))
        } else if input[location..].starts_with(c) {
            (context, ParseResult::successful(c, location + 1))
        } else {
            // not equal
            (context, ParseResult::failure(ParseError::new(location, "char".to_string(), "mot match".to_string()), location))
        }
    })
}

#[test]
fn char_test() {

    let parser = char('a') + char('b');

    match parser.parse(ParseContext::new_context(), "abcd", 0) {
        (_, ParseResult::Success { value, location}) => {
            assert!(true);
            assert_eq!(('a', 'b'), value);
            assert_eq!(2, location);
        }
        (error_context, _) => {
            println!("{}", error_context);
            assert!(false);
        }
    }
    
}

//     // let parser = char('a').seq1() + end();
//     // match parser.parse(&mut ParserInput::text("aaaa"), &mut ParseContext::new_context(0)) {
//     //     (next_context, ParseResult::Success { value}) => {
//     //         assert!(true);
//     //         assert_eq!((vec!['a', 'a', 'a', 'a'], ()), value);
//     //         assert_eq!(4, next_context.location);
//     //     }
//     //     _ => assert!(false),
//     // }

//     // let parser = (char('🍣') | char('🍺')).seq0() + end();
//     // match parser.parse(&mut ParserInput::text("🍣🍣🍺🍺🍣🍺🍺"), &mut ParseContext::new_context(0)) {
//     //     (next_context, ParseResult::Success { value}) => {
//     //         assert!(true);
//     //         assert_eq!((vec!['🍣', '🍣', '🍺', '🍺', '🍣', '🍺', '🍺'], ()), value);
//     //         assert_eq!(28, next_context.location);
//     //     }
//     //     _ => assert!(false),
//     // }
// }

// pub fn dquote_string<'a>() -> Parser<'a, String> {
//     let dquote_parser = char('"');
//     let dquote_stop_parser = stop_char('"');
    
//     let escape_parser = escape().map(|v| v as u8);

//     dquote_parser.clone().
//         and_right(escape_parser.clone().or(dquote_stop_parser.clone()).seq0()).
//         and_left(dquote_parser.clone()).map(|v| String::from_utf8(v).unwrap())
// }

// #[test]
// pub fn dquote_string_test() {
//     let parser = dquote_string();
//     let mut parse_result = parser.parse(&mut ParserInput::text("\"abddef\""), &mut ParseContext::new_context(0));

//     match parse_result {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(value, "abddef");
//             assert_eq!(next_context.location, 8);
//         }
//         _ => {
//             assert!(false);
//         }
//     }

//     parse_result = parser.parse(&mut ParserInput::text("\"ghi\\\"jkl\""), &mut ParseContext::new_context(0));
//     match parse_result {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(value, "ghi\"jkl");
//             assert_eq!(next_context.location, 10);
//         }
//         _ => {
//             assert!(false);
//         }
//     }


    
// }

// pub fn end<'a>() -> Parser<'a, ()> {
//     Parser::new(move |input, context| {
//         if input.has_more(context.location) {
//             (context.new_error("end", "not end"), ParseResult::failure())

//         } else {
//             (context.move_location(0), ParseResult::successful(()))
//         }
//     })
// }

// pub fn escape<'a>() -> Parser<'a, char> {
//     Parser::new(move |input, context| {
//         let read_result = input.read_by_size(context.location, 2);
//         match input.read_by_size(context.location, 2) {
//             Result::Ok(read) => {
//                 match read {
//                     [b'\\', b't'] => {
//                         (context.move_location(2), ParseResult::successful('\t'))
//                     }
//                     [b'\\', b'r'] => {
//                         (context.move_location(2), ParseResult::successful('\r'))
//                     }
//                     [b'\\', b'n'] => {
//                         (context.move_location(2), ParseResult::successful('\n'))
//                     }
//                     [b'\\', b'\\'] => {
//                         (context.move_location(2), ParseResult::successful('\\'))
//                     }
//                     [b'\\', b'"'] => {
//                         (context.move_location(2), ParseResult::successful('"'))
//                     }
//                     [b'\\', b'\''] => {
//                         (context.move_location(2), ParseResult::successful('\''))
//                     }
//                     _ => {
//                         (context.new_error("escape", "not escape"), ParseResult::failure())
//                     }
//                 }
//             },
//             Result::Err(_) => {
//                 (context.new_error("escape", "read error"), ParseResult::failure())
//             }
//         }
//     })
// }


// #[test]
// fn test_escape() {
//     let parser = char('a').seq0() + escape();
//     let parse_result = parser.parse(&mut ParserInput::text("aaa\\n"), &mut ParseContext::new_context(0));
//     println!("result*{:?}", parse_result);
//     match parse_result {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(next_context.location, 5);
//             assert_eq!(value.0, vec!['a', 'a', 'a']);
//             assert_eq!(value.1, '\n');
//         }
//         _ => {
//             assert!(false);
//         }
//     }
// }


// pub fn head<'a>() -> Parser<'a, ()> {
//     Parser::new(move |input, context| {
//         if context.location == 0 {
//             (context.move_location(0), ParseResult::successful(()))
//         } else if context.location > 1 && input[context.location..context.location].starts_with([LF_CHAR]) {
//             (context.move_location(0), ParseResult::successful(()))
//         } else {
//             (context.new_error("head", "not head"), ParseResult::failure())
//         }
//     })
// }

// // pub fn lazy<'a, A, F>(f: F) -> Parser<'a, A>
// // where
// //     F: Fn() -> Parser<'a, A> + 'a,
// //     A: 'a + Clone,
// // {
// //     unit().flat_map(move |_| f())
// // }

// // #[test]
// // fn lazy_test() {
// //     let a_parser = char('a');
// // }

// pub fn literal<'a>(v: &'a str) -> Parser<'a, &str> {
//     Parser::new(move |input, context| {
//         if input.len() > context.location {
//             // length over
//             (context.new_error("char", "length over"), ParseResult::failure())
//         } else if input[context.location..].starts_with(v) {
//             (context.move_location(1),ParseResult::successful(v))
//         } else {
//             // not equal
//             (context.new_error("char", "not match"), ParseResult::failure())
//         }
//     })
// }

// #[test]
// fn literal_test() {
//     let parser = head() + literal("stuvwxyz.");
//     match parser.parse(&mut ParserInput::text("abc\ndef\nstuvwxyz."), &mut ParseContext::new_context(8)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(((), "stuvwxyz."), value);
//             assert_eq!(17, next_context.location);
//         }
//         _ => assert!(false),
//     }
// }

// pub fn number_i32<'a>() -> Parser<'a, i32> {
//     Parser::new(move |input, context| {
//         let mut sign = 1;
//         let mut number = 0;
//         let mut current_location = context.location;

//         let mut read;
//         match input.read_by_size(current_location, 1) {
//             Result::Ok(r) => {
//                 read = r;
//             },
//             Result::Err(_) => {
//                 return (context.new_error("number", "not number"), ParseResult::failure());
//             }
//         };
//         if read == [MINUS_CODE] {
//             sign = -1;
//             current_location = current_location + 1;
        
//         } else if read == [PLUS_CODE] {
//             sign = 1;
//             current_location = current_location + 1;
//         }

//         match input.read_by_size(current_location, 1) {
//             Result::Ok(r) => {
//                 read = r;
//             },
//             Result::Err(_) => {
//                 return (context.new_error("number", "not number"), ParseResult::failure());
//             }
//         };
//         if read[0] >= ZERO_CODE && read[0] <= NINE_CODE {

//             loop {
//                 if read[0] >= ZERO_CODE && read[0] <= NINE_CODE {
//                     number = number * 10 + (read[0] - ZERO_CODE) as i32;
//                     current_location += 1;
//                     match input.read_by_size(current_location, 1) {
//                         Result::Ok(r) => {
//                             read = r;
//                         },
//                         Result::Err(_) => {
//                             break;
//                         }
//                     }
//                 } else {
//                     break;
//                 }
//             }
//             (context.new_location(current_location), ParseResult::successful(sign * number))
//         } else {
//             (context.new_error("number", "not number"), ParseResult::failure())
//         }
//     })
// }

// #[test]
// fn test_number_i32() {
//     let parser = number_i32();
//     let (next_context, parse_result) = parser.parse(&mut ParserInput::text("1234"), &mut ParseContext::new_context(0));
//     match parse_result {
//         ParseResult::Success { value } => {
//             assert_eq!(value, 1234);
//             assert_eq!(next_context.location, 4);
//         },
//         _ => {
//             assert!(false)
//         }
//     }
// }

// pub fn number_i64<'a>() -> Parser<'a, i64> {
//     Parser::new(move |input, context| {
//         let mut sign = 1;
//         let mut number = 0 as i64;
//         let mut current_location = context.location;

//         let mut read;
//         match input.read_by_size(current_location, 1) {
//             Result::Ok(r) => {
//                 read = r;
//             },
//             Result::Err(_) => {
//                 return (context.new_error("number", "not number"), ParseResult::failure());
//             }
//         };
//         if read == [MINUS_CODE] {
//             sign = -1;
//             current_location = current_location + 1;
        
//         } else if read == [PLUS_CODE] {
//             sign = 1;
//             current_location = current_location + 1;
//         }

//         match input.read_by_size(current_location, 1) {
//             Result::Ok(r) => {
//                 read = r;
//             },
//             Result::Err(_) => {
//                 return (context.new_error("number", "not number"), ParseResult::failure());
//             }
//         };
//         if read[0] >= ZERO_CODE && read[0] <= NINE_CODE {

//             loop {
//                 if read[0] >= ZERO_CODE && read[0] <= NINE_CODE {
//                     number = number * 10 + (read[0] - ZERO_CODE) as i64;
//                     current_location += 1;
//                     match input.read_by_size(current_location, 1) {
//                         Result::Ok(r) => {
//                             read = r;
//                         },
//                         Result::Err(_) => {
//                             break;
//                         }
//                     }
//                 } else {
//                     break;
//                 }
//             }
//             (context.new_location(current_location), ParseResult::successful(sign * number))
//         } else {
//             (context.new_error("number", "not number"), ParseResult::failure())
//         }
//     })
// }

// pub fn number_f64<'a>() -> Parser<'a, f64> {
//     Parser::new(move |input, context| {
//         let mut current_location = context.location;
//         let mut hava_sign = false;
//         let mut hava_float = false;

//         match input.read_by_size(current_location, 1) {
//             Result::Ok(r) => {
//                 if r[0] == MINUS_CODE || r[0] == PLUS_CODE {
//                     current_location = current_location + 1;
//                     hava_sign = true;
                
//                 }
//             },
//             Result::Err(_) => {
//                 return (context.new_error("number", "not number"), ParseResult::failure());
//             }
//         };

//         loop {
//             match input.read_by_size(current_location, 1) {
//                 Result::Ok(r) => {
//                     if r[0] >= ZERO_CODE && r[0] <= NINE_CODE {
//                         current_location = current_location + 1;
                    
//                     } else {
//                         break;
//                     }
//                 },
//                 Result::Err(_) => {
//                     break;
//                 }
//             }
//         }

//         if current_location == context.location || (hava_sign && current_location == context.location + 1) {
//             // 数値なし
//             return (context.new_error("number", "not number"), ParseResult::failure());
//         }

//         match input.read_by_size(current_location, 1) {
//             Result::Ok(r) => {
//                 if r[0] == DOT_CODE {
//                     // 少数あり
//                     current_location = current_location + 1;
//                     hava_float = true;
//                 }
//             },
//             Result::Err(_) => {}
//         };

//         if hava_float {
//             // 少数あり
//             loop {
//                 match input.read_by_size(current_location, 1) {
//                     Result::Ok(r) => {
//                         if r[0] >= ZERO_CODE && r[0] <= NINE_CODE {
//                             current_location = current_location + 1;
                        
//                         } else {
//                             break;
//                         }
//                     },
//                     Result::Err(_) => {
//                         break;
//                     }
//                 }
//             }
            
//         }
//         match input.read_by_size(context.location, current_location - context.location) {
//             Result::Ok(r) => {
//                 return (context.new_location(current_location), ParseResult::successful(std::str::from_utf8(r).unwrap().parse::<f64>().unwrap()));
//             },
//             Result::Err(_) => {
//                 return (context.new_error("number", "read by size error"), ParseResult::failure());
//             }
//         }
//     })
// }

// #[test]
// pub fn number_f64_test() {
//     let parser = number_f64();

//     match parser.parse(&mut ParserInput::text("-1234.567890000001"), &mut ParseContext::new_context(0)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(value, -1234.567890000001);
//             assert_eq!(next_context.location, 18);
//         }, 
//         _ => {
//             assert!(false);
//         }
//     }
    
// }

// pub fn space<'a>() -> Parser<'a, ()> {
//     Parser::new(move |input, context| {
//         match input.read_by_size(context.location, 1) {
//             Result::Ok(read) => match read[0] {
//                 SPACE_CODE | TAB_CODE => {
//                     (context.move_location(1), ParseResult::successful(()))
//                 },
//                 _ => {
//                     (context.new_error("space", "not space"), ParseResult::failure())
//                 },
//             },
//             Result::Err(_error) => {
//                 (context.new_error("space", "read failed"), ParseResult::failure())
//             },
//         }
//     })
// }

// #[test]
// fn space_test() {
//     let break_line_parser = space();
//     match break_line_parser.parse(&mut ParserInput::text("abc def"), &mut ParseContext::new_context(3)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(next_context.location, 4);
//         }, 
//         _ => {
//             assert!(false);
//         }
//     }
//     match break_line_parser.parse(&mut ParserInput::text("abc\tdef"), &mut ParseContext::new_context(3)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(next_context.location, 4);
//         }
//         _ => {
//             assert!(false);
//         }
//     }
// }

// pub fn stop_char<'a>(c: char) -> Parser<'a, u8> {
//     Parser::new(move |input, context| {
//         // 最大4バイトを想定
//         let mut tmp = [0u8; 4];
//         let bytes = c.encode_utf8(&mut tmp).as_bytes();
//         let len = bytes.len();

//         // TODO StopWordがマルチバイトで残りバイト数1の場合の対応
//         match input.read_by_size(context.location, len) {
//             Result::Ok(txt) => {
//                 if txt == bytes {
//                     (context.new_error("stop_char", "match input"), ParseResult::failure())
//                 } else {
//                     (context.move_location(1), ParseResult::successful(txt[0]))
//                 }
//             }
//             Result::Err(_) => (context.new_error("char", "read error"), ParseResult::failure())
//         }
//     })
// }

// #[test]
// pub fn stop_char_test() {
//     let parser = stop_char('"').seq0().map(|v| String::from_utf8(v));
//     let parse_result = parser.parse(&mut ParserInput::text("\"abc def\""), &mut ParseContext::new_context(1));

//     match parse_result {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(value, Result::Ok("abc def".to_owned()));
//             assert_eq!(next_context.location, 8);
//         }
//         _ => {
//             assert!(false);
//         }
//     }

// }

// pub fn space_or_line<'a>() -> Parser<'a, ()> {
//     space().or(break_line())
// }

// pub fn space_or_line_seq<'a>() -> Parser<'a, ()> {
//     space().or(break_line()).seq0().map(move |_| ())
// }

// #[test]
// fn test_space_or_line_seq() {
//     let parser = literal("abc") + space_or_line_seq() + literal("def");
//     match parser.parse(&mut ParserInput::text("abc  \t \r\n\ndef"), &mut ParseContext::new_context(0)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(next_context.location, 13);
//             assert_eq!(value.0 .0, "abc");
//             assert_eq!(value.0 .1, ());
//             assert_eq!(value.1, "def");
//         }
//         _ => {
//             assert!(false);
//         }
//     }
// }

// pub fn unit<'a>() -> Parser<'a, ()>
// where {
//     Parser::new(move |input, context|  {
//         (context.move_location(0), ParseResult::successful(()))
//     })
// }

// #[test]
// fn test_unit() {
//     let parser = unit()
//         .flat_map(move |v| Parser::new(|input, context| (context.move_location(0), ParseResult::successful("aaa"))));
//     match parser.parse(&mut ParserInput::text("abc  \t \r\n\ndef"), &mut ParseContext::new_context(0)) {
//         (next_context, ParseResult::Success { value }) => {
//             assert!(true);
//             assert_eq!(next_context.location, 0);
//             assert_eq!(value, "aaa");
//         }
//         _ => {
//             assert!(false);
//         }
//     }
// }