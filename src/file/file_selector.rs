use std::fs;
use scanf::scanf;

pub fn select_file() -> String {
    // Path에 있는 파일들의 목록을 가져온 뒤, unwrap()을 통해 DirEntry로 변환
    let paths = fs::read_dir("./testcodes/").unwrap();
    // DirEntry의 Iterator를 가져온 뒤, 각각의 Element(Entry)에 ~~.to_string()으로 파일 경로를 얻고, collect로 벡터에 수집
    let mut arr: Vec<String> = paths
        .into_iter()
        .map(|entry| entry.unwrap().path().display().to_string())
        .collect();
    // usize는 array index와 같이 런타임 메모리 작업에 사용되는 아키텍처 정수형 ( 32 비트 컴퓨터 -> 32bit, 64 비트 컴퓨터 -> 64bit )
    let mut select: usize = 0;
    println!("\t===== [ File List ] =====");
    for (index, path) in arr.iter().enumerate() {
        println!("{}. Path : {}", index + 1, path);
    }
    println!("\t===== ===== ===== =====");
    let mut select: i32 = 0;
    print!("Select execute file : ");
    scanf!("{}", select);
    if select > arr.len() as i32 {
        eprintln!("Error: Invalid input, select a number within the range ~{} (input: {}).", arr.len(), select);
        std::process::exit(1);
    }
    let key = (select - 1) as usize;
    return arr[key].clone();
}

fn get_name() -> String {
    return String::from("Yay");
}