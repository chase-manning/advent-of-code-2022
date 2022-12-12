use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

pub fn get_data_as_lines(file: &str) -> Vec<String> {
    let path = Path::new("./data").join(file);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn split2<T, U>(s: String, pat: &str) -> (T, U)
where
    T: std::str::FromStr,
    U: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <U as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parts = s.split(&pat);
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    (a, b)
}

pub fn last_number(s: &String) -> usize {
    s.split(' ').last().unwrap().parse::<usize>().unwrap()
}
