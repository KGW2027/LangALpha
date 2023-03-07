use std::fs::{File, read};
use std::io::BufReader;
use crate::file::pushback_reader::PushbackReader;
use crate::machine::exception::Exception;

pub enum LexerResult {
    Failed = 0,
    Success = 1,
    SuccessAndLineFeed = 2,
}

// PushbackReader에서 읽은 벡터를 문자열로 변환
fn vec_to_str(vec: Vec<u8>) -> String {
    vec.into_iter()
        .map(|u8| u8 as char)
        .collect()
}

// start: 에 대해 검사
pub fn lex_start(reader: &mut PushbackReader<BufReader<File>>) -> isize {
    let peek = vec_to_str(reader.peek(6));
    let mut result = LexerResult::Failed;
    if peek == "start:" {
        reader.clean();
        result = LexerResult::SuccessAndLineFeed;
    }
    result as isize
}