use std::cmp;

pub trait ParserInput<'a> {
    fn read(&mut self, size: usize) -> &'a str;
    fn read_line(&mut self) -> &'a str;
    fn has_more(&self) -> bool;
}

pub struct StringInput<'a> {
    str: &'a str,
    offset: usize
}

impl <'a>StringInput<'a> {
    
    fn new(str: &'a str) -> StringInput {
        StringInput {
            str,
            offset: 0
        }
    }
}

impl <'a>ParserInput<'a> for StringInput<'a> {
    fn read(&mut self, size: usize) -> &'a str {
        let read_size = cmp::min(size, self.str.len() - self.offset);
        let result = &self.str[self.offset..self.offset+read_size];
        self.offset += read_size;
        result
        
    }
    fn read_line(&mut self) -> &'a str {
        println!("call readline");
        let mut read_size = 0;
        let mut break_size = 0;

        let mut chars = self.str[self.offset..].chars();
        loop {
            match chars.nth(0) {
                Some(c) if c == '\r' => {
                    break_size += 1;
                    match chars.nth(0) {
                        Some(c) if c == '\n' => {
                            break_size += 1;
                            break;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Some(c) if c == '\n' => {
                    break_size += 1;
                    break;
                },
                Some(_) => {
                    read_size += 1;
                },
                None => {
                    break;
                }
            }
        }
        let result = &self.str[self.offset..self.offset+read_size];
        println!("read offset[{}] size[{}] str[{}]", self.offset, read_size, result);
        self.offset += read_size + break_size;
        result
    }
    fn has_more(&self) -> bool {
        self.str.len() > self.offset
    }
}

#[test]
fn test_string_input() {
    let mut input = StringInput::new("abcdefg\r\nhijklmn\n");
    println!("{}", input.read_line());
    println!("{}", input.read_line());
}