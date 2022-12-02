use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

fn read_file_to_lines(dir: &str, file: &str) -> Vec<String> {
    let path = Path::new(dir).join(file);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn main() {
    let lines = read_file_to_lines("./src/data", "strategy.txt");

    let mut total_score = 0;
    for line in lines {
        let mut iter = line.split_whitespace();
        let them = iter.next().unwrap();
        let me = iter.next().unwrap();
        match me {
            "X" => {
                total_score += 0 + match { them } {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => panic!("Invalid input"),
                };
            }
            "Y" => {
                total_score += 3 + match them {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => panic!("Invalid input"),
                };
            }
            "Z" => {
                total_score += 6 + match them {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => panic!("Invalid input"),
                };
            }
            _ => panic!("Invalid me"),
        }
    }
    println!("{}", total_score);
}
