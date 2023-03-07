use std::fs::File;
use std::io::BufReader;
use std::io::{Result};
use crate::file::pushback_reader::PushbackReader;

mod file {
    pub mod file_selector;
    pub mod pushback_reader;
}
mod machine {
    pub mod controller;
    pub mod exception;
}

fn main() {
    // File 경로 선택하기
    let file_path = file::file_selector::select_file();

    // File 객체 가져오기
    let mut open = File::open(file_path).unwrap();

    // 파일에 대한 pushback reader를 만든다.
    let mut pbr = PushbackReader::new(BufReader::new(open));

    // 인터프리터 머신으로 리더를 보낸다.
    machine::controller::init(&mut pbr).expect("Will Make StackTrace");
}
