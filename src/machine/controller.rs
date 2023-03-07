use std::fs::File;
use std::io::{Read, BufReader, Result};
use crate::file::pushback_reader::PushbackReader;

pub fn init(reader: &mut PushbackReader<BufReader<File>>) -> Result<()> {
    loop {
        let mut peeked: String;
        match reader.peek(1) {
            Ok(result) => {
                peeked = result.into_iter()
                        .map(|byte| byte as char)
                        .collect();
                println!("OK\n{}", peeked);
            }
            Err(exception) => {
                println!("Err {}", exception.get_message());
            }
        }
        break;
    }

    // 종료
    Ok(())
}