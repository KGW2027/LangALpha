use std::fs::File;
use std::io::{Read, BufReader, Result};
use crate::file::pushback_reader::PushbackReader;

pub fn init(reader: &mut PushbackReader<BufReader<File>>) -> Result<()> {
    // 1바이트 단위로 읽는다.
    let mut read = [0u8; 1];

    let mut col: u64 = 0;
    let mut len: u64 = 0;

    // 결과값이 0 (end of file)이 나올 때 까지 계속 한바이트씩 읽는다.
    loop {
        let peeked = 0;
        match reader.peek(254) {
            Ok(result) => {
                println!("OK {}", result);
            }
            Err(exception) => {
                println!("Err {}", exception.get_message());
            }
        }
        break;
        // if peeked == 0 {
        //     break;
        // }

        // let peeked = peeked as char;
        // if peeked == 's' {
        //
        // }
    }

    // 종료
    Ok(())
}