use std::time::Instant;

use crate::utils::files::get_data_as_lines;

struct Number {
    value: isize,
    index: usize,
}

fn get_numbers(lines: Vec<String>) -> Vec<Number> {
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| Number {
            value: line.parse::<isize>().unwrap() * 811589153,
            index,
        })
        .collect()
}

fn move_numbers(numbers: &mut Vec<Number>) {
    let mut i = 0;
    let len = numbers.len();
    while i < len {
        let pos = numbers.iter().position(|number| number.index == i).unwrap();
        if numbers[pos].value == 0 {
            i += 1;
            continue;
        }
        let number = numbers.remove(pos);
        let mut new_pos = pos as isize + number.value;
        if new_pos <= 0 {
            let mul = (new_pos.abs() / (len as isize - 1)) + 1;
            new_pos += mul * (len as isize - 1);
        }
        if new_pos >= len as isize {
            let mul = new_pos / (len as isize - 1);
            new_pos -= mul * (len as isize - 1);
        }
        numbers.insert(new_pos as usize, number);
        i += 1;
    }
}

fn get_answer(numbers: Vec<Number>) -> isize {
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
    for _ in 0..10 {
        move_numbers(&mut numbers);
    }
    let answer = get_answer(numbers);
    println!("Runtime: {:.2?}", now.elapsed());
    answer.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "11616");
}
