use std::fs::File;
use std::io::{Read, BufReader, Result};
use crate::file::pushback_reader::PushbackReader;
use crate::machine::lexer;

pub fn init(reader: &mut PushbackReader<BufReader<File>>) -> Result<()> {

    let mut status: isize = -1;
    let mut in_func = false;

    loop {
        if status == 2 {
            reader.skip_line();
        }

        let mut peeked: String = reader.peek(1)
            .into_iter()
            .map(|byte| byte as char)
            .collect();

        // start:
        if peeked == "s" {
            status = lexer::lex_start(reader);
            if status > 0 {
                in_func = true;
                continue;
            }
        }

        // start: 를 통과하기 전에는 다른 함수는 모두 bypass 된다.
        if !in_func {
            continue;
        }

        if peeked == "p" {
            // print(<expr>);
        }

        break;
    }

    // 종료
    Ok(())
}