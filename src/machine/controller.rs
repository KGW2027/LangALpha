use std::fs::File;
use std::io::{Read, BufReader, Result};

pub fn init(file: &mut File) -> Result<()> {
    // BufReader로 파일을 연다.
    let mut reader = BufReader::new(file);

    // 1바이트 단위로 읽는다.
    let mut read = [0u8; 1];

    // 결과값이 0 (end of file)이 나올 때 까지 계속 한바이트씩 읽는다.
    loop {
        match reader.read(&mut read) {
            Ok(0) => break,
            _ => {
                print!("{}", read[0] as char);
            }
        }
    }

    // 종료
    Ok(())
}