mod file {
    pub mod file_selector;
}

fn main() {
    let test = file::file_selector::select_file();
    println!("Selected : {}", test);
}
