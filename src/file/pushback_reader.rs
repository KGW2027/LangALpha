use std::io::{self, Read};
use crate::machine::exception::{Exception, ExceptionType};

pub struct PushbackReader<R> {
    inner: R,
    pushed: Vec<u8>
}

impl<R: Read> PushbackReader<R> {
    pub fn new(inner: R) -> PushbackReader<R> {
        PushbackReader {
            inner, pushed: Vec::new()
        }
    }

    pub fn unread(&mut self, byte: u8) {
        self.pushed.push(byte);
    }

    pub fn peek(&mut self, mut len: u8) -> Result<String, Exception> {

        let mut peeked: Vec<u8> = Vec::new();
        let mut str: String = String::new();
        let mut buffer = [0u8; 1];

        let mut exception: Option<Exception> = None;

        while len > 0 {
            len = len - 1;

            match self.read(&mut buffer) {
                Err(Error) => {
                    exception = Some(Exception::new(ExceptionType::AssertionFailed, "PushbackReader try to read unknown pointer.".to_string()));
                },
                Ok(read_length) => {
                    if read_length == 0 {
                        exception = Some(Exception::new(ExceptionType::EOFException, "PushbackReader peeked EOF.".to_string()));
                    }
                }
            }
            if exception.is_some() {
                break;
            }

            let read = buffer[0];
            peeked.push(read);
            str.push(read as char);
        }

        while peeked.len() > 0 {
            self.pushed.push(peeked.pop().unwrap());
        }

        if exception.is_none() {
            return Ok(str);
        }
        Err(exception.unwrap())
    }
}

impl<R: Read> Read for PushbackReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pushed.len() > 0{
            buf[0] = self.pushed.pop().unwrap();
            self.inner.read(&mut buf[1..]).map(|n| n + 1)
        } else {
            match self.inner.read(buf) {
                Ok(0) => Ok(0),
                other => {
                    Ok(other.unwrap())
                },
            }
        }
    }
}