use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

pub fn read_file_to_lines(dir: &str, file: &str) -> Vec<String> {
    let path = Path::new(dir).join(file);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
