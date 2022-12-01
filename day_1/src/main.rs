use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

fn read_file_to_lines(dir: &str, file: &str) -> Vec<String> {
    let path = Path::new(dir).join(file);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn main() {
    let lines = read_file_to_lines("./src/data", "calories.txt");

    let mut first_elf_calories = 0;
    let mut second_elf_calories = 0;
    let mut third_elf_calories = 0;
    let mut elf_calories = 0;
    for line in lines {
        if line == "" {
            if elf_calories > first_elf_calories {
                third_elf_calories = second_elf_calories;
                second_elf_calories = first_elf_calories;
                first_elf_calories = elf_calories;
            } else if elf_calories > second_elf_calories {
                third_elf_calories = second_elf_calories;
                second_elf_calories = elf_calories;
            } else if elf_calories > third_elf_calories {
                third_elf_calories = elf_calories;
            }
            elf_calories = 0;
            continue;
        }
        let calories = line.parse::<i32>().unwrap();
        elf_calories += calories;
    }
    println!(
        "{}",
        first_elf_calories + second_elf_calories + third_elf_calories
    );
}
