use std::io::{self, Read};
use crate::machine::exception;
use crate::machine::exception::{Exception, ExceptionType};

pub struct PushbackReader<R> {
    inner: R,
    pushed: Vec<u8>,
    ptr: u64
}

impl<R: Read> PushbackReader<R> {
    pub fn new(inner: R) -> PushbackReader<R> {
        PushbackReader {
            inner, pushed: Vec::new(), ptr: 0
        }
    }

    // len 길이만큼 확인한다.
    pub fn peek(&mut self, mut len: u8) -> Vec<u8> {

        let mut peeked: Vec<u8> = Vec::new();
        let mut buffer = [0u8; 1];

        while len > 0 {
            len = len - 1;

            match self.read(&mut buffer) {
                Err(_) => {
                    exception::throw_exception(Exception::new(ExceptionType::AssertionFailed, "PushbackReader try to read unknown pointer.".to_string()), self.ptr);
                    break;
                },
                Ok(read_length) => {
                    if read_length == 0 {
                        exception::throw_exception(Exception::new(ExceptionType::EOFException, "PushbackReader peeked EOF.".to_string()), self.ptr);
                        break;
                    }
                }
            }

            // println!("byte read : {} -> {}", buffer[0], buffer[0] as char);
            peeked.push(buffer[0]);
        }

        let return_value = peeked.clone();
        while !peeked.is_empty() {
            self.pushed.push(peeked.remove(peeked.len() - 1));
        }

        return_value
    }

    // 포인터를 증가시키고, 캐시를 지운다.
    pub fn clean(&mut self) {
        self.ptr = self.ptr + self.pushed.len() as u64;
        self.pushed.clear();
    }

    // 줄넘김 밑 공백 스킵
    pub fn skip_line(&mut self) {
        self.clean();
        let mut peek = self.peek(1)[0];
        while peek <= 32 {
            self.clean();
            peek = self.peek(1)[0];
        }
    }

    // 포인터값 반환
    pub fn get_ptr(&mut self) -> u64 {
        self.ptr
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