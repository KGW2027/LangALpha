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
    let file_path = file::file_selector::select_file();

    let mut open = File::open(file_path).unwrap();
    machine::controller::init(&mut open);
}
