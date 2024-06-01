use rcomb_parser::prelude::common_parsers::dquote_string;
use rcomb_parser::prelude::parser::*;
use rcomb_parser::prelude::common_parsers;
use rcomb_parser::prelude::parser_input::ParserInput;
use rcomb_parser::prelude::parse_context::ParseContext;
use rcomb_parser::prelude::parse_result::*;
use rcomb_parser::prelude::parser_methods::ParserMethods;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;



#[derive(Debug, Clone)]
pub struct MyNumber(f64);

// f64がeqを実装していないので簡易対応
impl PartialEq for MyNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MyNumber {}

#[test]
fn test11() {
    println!("{}", MyNumber(0.1) == MyNumber(0.2));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JValue {
    JObject(HashMap<String, JValue>),
    JArray(Vec<JValue>),
    JString(String),
    JNumber(MyNumber),
    JBool(bool),
    JNull
}

fn rc_unwrap<'a>(parser: Rc<RefCell<Parser<'a, JValue>>>) -> Parser<'a, JValue> {
    Parser::new(move |input, context| {
        parser.borrow().parse(input, context)
    })
}

fn rc_parsers<'a>(parsers: Vec<Rc<RefCell<Parser<'a, JValue>>>> ) -> Parser<'a, JValue> {
    Parser::new(move |input, context| {
        let mut error_contexts = Vec::new();

        for parser in parsers.iter() {
            match parser.borrow().parse(input, context) {
                (next_context, ParseResult::Success { value }) => {
                    return (next_context, ParseResult::successful(value));
                },
                (error_context, ParseResult::Failure {}) => {
                    error_contexts.push(error_context);
                }
            }
        }
        return (
            error_contexts.into_iter().fold(context.new_error("json", "jvalue parse error"), |mut acc, cur| acc.add_error(cur)), 
            ParseResult::failure()
        );
    })
}

fn json_parser<'a>() -> Parser<'a, JValue> {
    let mut jstr = Rc::new(RefCell::new(
        common_parsers::dquote_string().map(|v| JValue::JString(v)).with_skip_space()
    ));
    let mut jnumber = Rc::new(RefCell::new(
        common_parsers::number_f64().map(|v| JValue::JNumber(MyNumber(v))).with_skip_space()
    ));
    let mut jnull = Rc::new(RefCell::new(
        common_parsers::literal("null").map(|_v| JValue::JNull).with_skip_space()
    ));
    let mut jbool = Rc::new(RefCell::new(
        common_parsers::literal("true").map(|_| JValue::JBool(true)).
            or(common_parsers::literal("false").map(|_| JValue::JBool(false))).with_skip_space()
        )
    );
    let mut jarray = Rc::new(RefCell::new(Parser::new(move |input, context| {
        (context.clone(), ParseResult::<JValue>::failure())
    })));
    let mut jobject = Rc::new(RefCell::new(Parser::new(move |input, context| {
        (context.clone(), ParseResult::<JValue>::failure())
    })));
    let mut jvalue = Rc::new(RefCell::new(rc_parsers(vec![jstr.clone(), jnumber.clone(), jnull.clone(), jbool.clone(), jarray.clone(), jobject.clone()])));

    
    *jarray.borrow_mut() = common_parsers::array(
        rc_unwrap(jvalue.clone()), 
        common_parsers::char('[').with_skip_space(), 
        common_parsers::char(']').with_skip_space(), 
        common_parsers::char(',').with_skip_space()
    ).map(|v| JValue::JArray(v));


    *jobject.borrow_mut() = common_parsers::array(
        dquote_string().with_skip_space().
            and_left(common_parsers::char(':').with_skip_space()).
            and(rc_unwrap(jvalue.clone())), 
        common_parsers::char('{').with_skip_space(), 
        common_parsers::char('}').with_skip_space(), 
        common_parsers::char(',').with_skip_space()
    ).map(|v| JValue::JObject(v.into_iter().collect::<HashMap<String, JValue>>()));    

    return rc_parsers(vec![jarray.clone(), jobject.clone()]);
}


#[test]
fn test1() {
    let parser = json_parser().with_skip_space();
    match parser.parse(&mut ParserInput::text("[ \"abcd\" ]"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            println!("{}", next_context);
            assert!(true);
            assert_eq!(value, JValue::JArray(vec![
                JValue::JString("abcd".to_owned())
            ]));
        }
        _ => assert!(false),
    }


    match parser.parse(&mut ParserInput::text("{ }"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            println!("{}", next_context);
            assert!(true);
            assert_eq!(value, JValue::JObject(HashMap::new()));
        }
        (next_context, ParseResult::Failure {}) => {
            println!("{}", next_context);
            assert!(false)
        },
    }

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key.to_owned(), $val); )*
             map
        }}
    }

    match parser.parse(&mut ParserInput::text("{ \"array\": [1,2,3]}"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            println!("{}", next_context);
            assert!(true);
            println!("{:?}", value);
            assert_eq!(value, JValue::JObject(
                hashmap!["array" => JValue::JArray(vec![JValue::JNumber(MyNumber(1.0)), JValue::JNumber(MyNumber(2.0)), JValue::JNumber(MyNumber(3.0))])]
            ));
        }
        (next_context, ParseResult::Failure {}) => {
            println!("{}", next_context);
            assert!(false)
        },
    }

    match parser.parse(&mut ParserInput::text("  [ 123]"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            println!("{}", next_context);
            assert!(true);
            println!("{:?}", value);
            // assert_eq!(value, JValue::JObject(
            //     hashmap!["array" => JValue::JArray(vec![JValue::JNumber(1), JValue::JNumber(2), JValue::JNumber(3)])]
            // ));
        }
        (next_context, ParseResult::Failure {}) => {
            println!("{}", next_context);
            assert!(false)
        },
    }
}

#[test]
fn test2() {
    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key.to_owned(), $val); )*
             map
        }}
    }
    
    let parser = json_parser().with_skip_space();
    match parser.parse(&mut ParserInput::text("  [ 123, true, [456,  { \"string\": \"aaaaa\", \"numberInt\": 123, \"numberDouble\": -123.456, \"bool\": true, \"null\": null,\"array\":[ [1]] } ] ]"), &mut ParseContext::new_context(0)) {
        (next_context, ParseResult::Success { value}) => {
            println!("{}", next_context);
            assert!(true);
            println!("{:?}", value);
            assert_eq!(value, JValue::JArray(
                vec![
                    JValue::JNumber(MyNumber(123.0)),
                    JValue::JBool(true),
                    JValue::JArray(vec![
                        JValue::JNumber(MyNumber(456.0)),
                        JValue::JObject(hashmap![
                            "string" => JValue::JString("aaaaa".to_owned()),
                            "numberInt" => JValue::JNumber(MyNumber(123.0)),
                            "numberDouble" => JValue::JNumber(MyNumber(-123.456)),
                            "bool" => JValue::JBool(true),
                            "null" => JValue::JNull,
                            "array" => JValue::JArray(vec![JValue::JArray(vec![JValue::JNumber(MyNumber(1.0))])])
                        ])
                    ])
                ]
            ));
        }
        (next_context, ParseResult::Failure {}) => {
            println!("{}", next_context);
            assert!(false)
        },
    }
}

#[test]
fn test3() {
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    let parser = json_parser().with_skip_space();
    let mut result;

    let start = Instant::now();
    for i in 0..100000 {
        result =  parser.parse(&mut ParserInput::text("  [ 123, true, [456,  { \"string\": \"aaaaa\", \"numberInt\": 123, \"numberDouble\": -123.456, \"bool\": true, \"null\": null,\"array\":[ [1]] } ] ]"), &mut ParseContext::new_context(0));
    }
    let end = start.elapsed();
    println!("time: {}.{:03}", end.as_secs(), end.subsec_nanos() / 1_000_000);

}