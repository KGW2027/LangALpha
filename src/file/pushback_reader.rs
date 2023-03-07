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

    pub fn peek(&mut self, mut len: u8) -> Result<Vec<u8>, Exception> {

        let mut peeked: Vec<u8> = Vec::new();
        let mut buffer = [0u8; 1];
        let mut exception: Option<Exception> = None;

        while len > 0 {
            len = len - 1;

            match self.read(&mut buffer) {
                Err(_) => {
                    exception = Some(Exception::new(ExceptionType::AssertionFailed, "PushbackReader try to read unknown pointer.".to_string()));
                    break;
                },
                Ok(read_length) => {
                    if read_length == 0 {
                        exception = Some(Exception::new(ExceptionType::EOFException, "PushbackReader peeked EOF.".to_string()));
                        break;
                    }
                }
            }

            peeked.push(buffer[0]);
        }

        let return_value = peeked.clone();
        while !peeked.is_empty() {
            self.pushed.push(peeked.remove(peeked.len() - 1));
        }

        exception.map(Err).unwrap_or(Ok(return_value))
    }
}

impl<R: Read> Read for PushbackReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(byte) = self.pushed.pop() {
            buf[0] = byte;
            self.inner.read(&mut buf[1..]).map(|n| n + 1)
        } else {
            self.inner.read(buf)
        }
    }
}