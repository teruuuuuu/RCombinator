use rcomb_parser::prelude::parser::*;
use rcomb_parser::prelude::common_parsers;
use rcomb_parser::prelude::parser_input::ParserInput;
use rcomb_parser::prelude::parse_context::ParseContext;
use rcomb_parser::prelude::parse_result::*;
use rcomb_parser::prelude::parser_methods::ParserMethods;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JValue {
    JObject(HashMap<String, JValue>),
    JArray(Vec<JValue>),
    JString(String),
    JNumber(i64),
    JBool(bool),
    JNull
}

fn jstr_parser<'a>() -> Parser<'a, JValue> {
    common_parsers::dquote_string().map(|v| JValue::JString(v))
}

fn jnumber_parser<'a>() -> Parser<'a, JValue> {
    common_parsers::number_i64().map(|v| JValue::JNumber(v))
}

fn jnull_parser<'a>() -> Parser<'a, JValue> {
    common_parsers::literal("null").map(|v| JValue::JNull)
}

fn jbool_parser<'a>() -> Parser<'a, JValue> {
    common_parsers::literal("true").map(|_| JValue::JBool(true)).
        or(common_parsers::literal("false").map(|_| JValue::JBool(false)))
}

fn rc_parsers<'a>(parsers: Vec<Rc<RefCell<Parser<'a, JValue>>>> ) -> Parser<'a, JValue> {
    Parser::new(move |input, context| {

        let mut current_context = context.clone();
        let mut error_contexts = Vec::new();

        for parser in parsers.iter() {
            match parser.borrow_mut().parse(input, context) {
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

pub fn rc_array<'a, A, B, C, D>(
    parser: Rc<RefCell<Parser<'a, A>>>,
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

        let (next_contet, parse_result) = parser.borrow_mut().parse(input, current_context);
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
            let (next_context2, parse_result2) = parser.borrow_mut().parse(input, &mut next_context1);
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


fn json_parser<'a>() -> Parser<'a, JValue> {
    let mut jstr = Rc::new(RefCell::new(
        common_parsers::dquote_string().map(|v| JValue::JString(v)).with_skip_space()
    ));
    let mut jnumber = Rc::new(RefCell::new(
        common_parsers::number_i64().map(|v| JValue::JNumber(v)).with_skip_space()
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

    
    *jarray.borrow_mut() = rc_array(
        jvalue, 
        common_parsers::char('[').with_skip_space(), 
        common_parsers::char(']').with_skip_space(), 
        common_parsers::char(',').with_skip_space()).map(|v| JValue::JArray(v)
    );

    return rc_parsers(vec![jarray.clone(), jobject.clone()]);
}


#[test]
fn test() {

    let jsr_parser = json_parser().with_skip_space();
    let parse_result = jsr_parser.parse(&mut ParserInput::text("[ \"abcd\" ]"), &mut ParseContext::new_context(0));

    match parse_result {
        (next_context, ParseResult::Success { value}) => {
            println!("{}", next_context);
            assert!(true);
            assert_eq!(value, JValue::JArray(vec![
                JValue::JString("abcd".to_owned())
            ]));
        }
        _ => assert!(false),
    }
}