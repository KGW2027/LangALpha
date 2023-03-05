use std::fs::File;
use std::io::BufReader;
use std::io::{Result};

mod file {
    pub mod file_selector;
}
mod machine {
    pub mod controller;
}

fn main() {
    // 1. File 경로 선택하기
    let file_path = file::file_selector::select_file();

    // 2. File 객체 가져오기
    let mut open = File::open(file_path).unwrap();

    // 3. 머신에서 컴파일
    machine::controller::init(&mut open).expect("Will Make StackTrace");
}
