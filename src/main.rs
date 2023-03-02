mod file {
    pub mod file_reader;
}

fn main() {
    let test = file::file_reader::get_name();
    println!("{} and {}", test, test);
}
