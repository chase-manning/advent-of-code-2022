use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

pub fn get_data_as_lines(file: &str) -> Vec<String> {
    let path = Path::new("./data").join(file);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
