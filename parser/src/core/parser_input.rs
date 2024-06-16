use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::sync::Mutex;

use crate::core::error::MyError;

const BUF_SIZE: usize = 4096;
const NULL_CODE: u8 = 0;
const CR_CODE: u8 = 13;
const LF_CODE: u8 = 10;

pub enum ParserInput<'a> {
    Text(&'a str),
    File {
        reader: Mutex<BufReader<File>>,
        buffer: Mutex<[u8; BUF_SIZE]>,
        buffer_seek: Mutex<u64>,
        buffer_read: Mutex<usize>,
    },
}

impl<'a> ParserInput<'a> {
    pub fn text<'b>(str: &'b str) -> ParserInput<'b> {
        ParserInput::Text(&str)
    }

    pub fn file<'b>(file: File) -> ParserInput<'b> {
        let mut reader = Mutex::new(BufReader::new(file));
        let mut buffer = Mutex::new([0; BUF_SIZE]);
        let mut buffer_seek = Mutex::new(0);
        let mut buffer_read = Mutex::new(0);

        let mut_reader = reader.get_mut().unwrap();
        let mut_buffer = buffer.get_mut().unwrap();
        let mut_buffer_seek = buffer_seek.get_mut().unwrap();
        let mut_buffer_read = buffer_read.get_mut().unwrap();
        ParserInput::load_buf(mut_reader, mut_buffer, mut_buffer_seek, mut_buffer_read, 0);
        ParserInput::File {
            reader,
            buffer,
            buffer_seek,
            buffer_read,
        }
    }

    pub fn has_more(&mut self, offset: usize) -> bool {
        match self {
            ParserInput::Text(text) => {
                let bytes = text.as_bytes();
                bytes.len() > offset
            }
            ParserInput::File {
                reader,
                buffer,
                buffer_seek,
                buffer_read,
            } => {
                let mut_reader = reader.get_mut().unwrap();
                let mut_buffer = buffer.get_mut().unwrap();
                let mut_buffer_seek = buffer_seek.get_mut().unwrap();
                let mut_buffer_read = buffer_read.get_mut().unwrap();

                if !(*mut_buffer_seek <= offset as u64
                    && (*mut_buffer_seek + *mut_buffer_read as u64) >= offset as u64)
                {
                    ParserInput::load_buf(
                        mut_reader,
                        mut_buffer,
                        mut_buffer_seek,
                        mut_buffer_read,
                        offset as u64,
                    );
                }

                if *mut_buffer_read < BUF_SIZE {
                    *mut_buffer_seek as usize + *mut_buffer_read > offset
                } else {
                    ParserInput::load_buf(
                        mut_reader,
                        mut_buffer,
                        mut_buffer_seek,
                        mut_buffer_read,
                        offset as u64,
                    );
                    *mut_buffer_seek as usize + *mut_buffer_read > offset
                }
            }
        }
    }

    pub fn read_line(&mut self, offset: usize) -> Vec<u8> {
        match self {
            ParserInput::Text(text) => {
                let bytes = text.as_bytes();
                let text_length = bytes.len();

                let mut curr_index = offset;
                let mut cr_flg = false;
                let mut vec = Vec::new();
                loop {
                    if curr_index >= text_length {
                        break;
                    }
                    if bytes[curr_index] == NULL_CODE {
                        break;
                    } else if bytes[curr_index] == LF_CODE {
                        vec.push(LF_CODE);
                        break;
                    } else if bytes[curr_index] == CR_CODE {
                        cr_flg = true;
                        vec.push(CR_CODE);
                        curr_index = curr_index + 1;
                    } else if cr_flg {
                        break;
                    } else {
                        vec.push(bytes[curr_index]);
                        curr_index = curr_index + 1;
                    }
                }
                vec
            }
            ParserInput::File {
                reader,
                buffer,
                buffer_seek,
                buffer_read,
            } => {
                let mut_reader = reader.get_mut().unwrap();
                let mut_buffer = buffer.get_mut().unwrap();
                let mut_buffer_seek = buffer_seek.get_mut().unwrap();
                let mut_buffer_read = buffer_read.get_mut().unwrap();

                let mut vec = Vec::new();
                let mut cr_flg = false;

                let mut curr_buffer_offset = offset as u64;
                let mut curr_buffer_index = 0;
                ParserInput::load_buf(
                    mut_reader,
                    mut_buffer,
                    mut_buffer_seek,
                    mut_buffer_read,
                    curr_buffer_offset,
                );
                loop {
                    if *mut_buffer_read <= 0 {
                        break;
                    } else if curr_buffer_index >= *mut_buffer_read {
                        curr_buffer_offset = curr_buffer_offset + curr_buffer_index as u64;
                        curr_buffer_index = 0;
                        ParserInput::load_buf(
                            mut_reader,
                            mut_buffer,
                            mut_buffer_seek,
                            mut_buffer_read,
                            curr_buffer_offset,
                        );
                    }

                    if mut_buffer[curr_buffer_index] == NULL_CODE {
                        break;
                    } else if mut_buffer[curr_buffer_index] == LF_CODE {
                        vec.push(LF_CODE);
                        break;
                    } else if mut_buffer[curr_buffer_index] == CR_CODE {
                        cr_flg = true;
                        vec.push(CR_CODE);
                        curr_buffer_index = curr_buffer_index + 1;
                    } else if cr_flg {
                        break;
                    } else {
                        vec.push(mut_buffer[curr_buffer_index]);
                        curr_buffer_index = curr_buffer_index + 1;
                    }
                }
                vec
            }
        }
    }

    pub fn read_by_size(&mut self, offset: usize, size: usize) -> Result<&[u8], MyError> {
        match self {
            ParserInput::Text(text) => {
                let bytes = text.as_bytes();
                if (offset + size) <= bytes.len() {
                    Result::Ok(&bytes[(offset as usize)..(offset as usize + size as usize)])
                } else {
                    Result::Err(MyError::Common("offset over".to_owned()))
                }
            }
            ParserInput::File {
                reader,
                buffer,
                buffer_seek,
                buffer_read,
            } => {
                let mut_reader = reader.get_mut().unwrap();
                let mut_buffer = buffer.get_mut().unwrap();
                let mut_buffer_seek = buffer_seek.get_mut().unwrap();
                let mut_buffer_read = buffer_read.get_mut().unwrap();
                if size > BUF_SIZE {
                    Result::Err(MyError::Common("buffer over".to_owned()))
                } else {
                    if !(*mut_buffer_seek <= offset as u64
                        && (*mut_buffer_seek + *mut_buffer_read as u64)
                            >= (offset as u64 + size as u64))
                    {
                        ParserInput::load_buf(
                            mut_reader,
                            mut_buffer,
                            mut_buffer_seek,
                            mut_buffer_read,
                            offset as u64,
                        );
                    }
                    if *mut_buffer_seek <= offset as u64
                        && (*mut_buffer_seek + *mut_buffer_read as u64)
                            >= (offset as u64 + size as u64)
                    {
                        let buf_from = offset - *mut_buffer_seek as usize;
                        let buf_to = buf_from + size;
                        Result::Ok(&mut_buffer[buf_from..buf_to])
                    } else {
                        Result::Err(MyError::Common("offset over".to_owned()))
                    }
                }
            }
        }
    }

    fn load_buf(
        buf_reader: &mut BufReader<File>,
        buffer: &mut [u8; BUF_SIZE],
        buffer_seek: &mut u64,
        buffer_read: &mut usize,
        offset: u64,
    ) {
        buffer.iter_mut().for_each(|m| *m = 0);
        *buffer_seek = buf_reader.seek(SeekFrom::Start(offset)).unwrap();
        *buffer_read = buf_reader.read(buffer).unwrap();
    }
}

#[test]
fn test_txt() {
    let mut curr_index = 0;
    let mut a = ParserInput::text("abc\ndef\nghi\n");

    while a.has_more(curr_index + 1) {
        match a.read_by_size(curr_index, 2) {
            Result::Ok(read) => {
                println!("curr_index[{}]", curr_index);
                println!("{:?}", read);
                println!("");
            }
            Result::Err(_) => {
                assert!(false);
            }
        };
        curr_index = curr_index + 1;
    }
}

#[test]
fn test_file() {
    let mut curr_index = 0;
    let file = File::open("./test/test_file.txt").unwrap();
    println!("{:?}", file);
    let mut a = ParserInput::file(file);

    while a.has_more(curr_index + 1) {
        match a.read_by_size(curr_index, 2) {
            Result::Ok(read) => {
                println!("curr_index[{}]", curr_index);
                println!("{:?}", read);
                println!("");
            }
            Result::Err(_) => {
                assert!(false);
            }
        };
        curr_index = curr_index + 1;
    }
}

// #[test]
// fn test3() {
//     let mut a = ParserInput::text("abc\rdef\nghijkl\r\n");
//     let mut curr_index = 0;
//     while a.has_more(curr_index) {
//         let read = a.read_by_size(curr_index, 2);
//         println!("curr_index[{}]", curr_index);
//         println!("{:?}", read);
//         println!("");
//         curr_index = curr_index + 1;
//     }

//     curr_index = 0;
//     let file = File::open("../test.txt").unwrap();
//     let mut a = ParserInput::file(file);
//     while a.has_more(curr_index) {
//         let read = a.read_by_size(curr_index, 2);
//         println!("curr_index[{}]", curr_index);
//         println!("{:?}", read);
//         println!("");
//         curr_index = curr_index + 1;
//     }
// }
