use std::fs::File;
use std::io::BufReader;

pub fn read_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    reader
}

