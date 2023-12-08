use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    reader
}
pub fn get_file_v_str(file_path: &str) -> Vec<String> {
    let input = read_file(file_path);
    input.lines()
                            .map(|x| String::from(x.unwrap()))
                            .collect()
}
