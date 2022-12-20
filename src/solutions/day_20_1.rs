use std::time::Instant;

use crate::utils::files::get_data_as_lines;

struct Number {
    value: i32,
    index: usize,
}

fn get_numbers(lines: Vec<String>) -> Vec<Number> {
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| Number {
            value: line.parse::<i32>().unwrap(),
            index,
        })
        .collect()
}

fn move_numbers(numbers: &mut Vec<Number>) {
    let mut i = 0;
    let len = numbers.len();
    while i < len {
        let pos = numbers.iter().position(|number| number.index == i).unwrap();
        let number = numbers.remove(pos);
        let mut new_pos = pos as i32 + number.value;
        while new_pos <= 0 {
            new_pos += len as i32 - 1;
        }
        while new_pos >= len as i32 {
            new_pos -= len as i32 - 1;
        }
        numbers.insert(new_pos as usize % len, number);
        i += 1;
    }
}

fn get_answer(numbers: Vec<Number>) -> i32 {
    let zero_pos = numbers.iter().position(|number| number.value == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|pos| numbers[(zero_pos + pos) % numbers.len()].value)
        .sum()
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_20_encoded.txt");
    let mut numbers = get_numbers(lines);
    move_numbers(&mut numbers);
    let answer = get_answer(numbers);
    println!("Runtime: {:.2?}", now.elapsed());
    answer.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "11616");
}
