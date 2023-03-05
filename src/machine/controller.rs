use std::fs::File;
use std::io::{Read, BufReader, Result};

pub fn init(file: &mut File) -> Result<()> {
    let mut reader = BufReader::new(file);
    let mut read = [0u8; 1];
    loop {
        match reader.read(&mut read) {
            Ok(0) => break,
            _ => {
                print!("{}", read[0] as char);
            }
        }
    }
    Ok(())
}