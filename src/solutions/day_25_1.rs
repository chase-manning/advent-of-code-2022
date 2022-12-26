use crate::utils::files::get_data_as_lines;
use std::time::Instant;

// fn next_snafu(chars: Vec<char>, )

fn max_next(index: usize) -> isize {
    let mut total = 0;
    for i in 0..index {
        total += 5isize.pow(i as u32) * 2;
    }
    println!("max_next({}) = {}", index, total);
    total
}

fn num_to_snafu(num: &usize) -> String {
    println!("{}", num);
    let mut chars = Vec::new();
    let mut remaining = *num as isize;

    let mut index = 0;
    while 5usize.pow(index as u32) <= *num {
        index += 1;
    }

    loop {
        if index == 0 {
            chars.push(match remaining {
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                0 => '0',
                _ => panic!("Invalid remaining"),
            });
            while chars[0] == '0' {
                chars.remove(0);
            }
            return chars.iter().collect();
        } else if remaining > 0 {
            let max_next = max_next(index);
            if remaining <= max_next {
                chars.push('0');
            } else if remaining <= 5isize.pow(index as u32) + max_next {
                chars.push('1');
                remaining -= 5isize.pow(index as u32);
            } else {
                chars.push('2');
                remaining -= 5isize.pow(index as u32) * 2;
            }
        } else {
            let max_next = max_next(index);
            if remaining >= -max_next {
                chars.push('0');
            } else if remaining >= -5isize.pow(index as u32) - max_next {
                chars.push('-');
                remaining += 5isize.pow(index as u32);
            } else {
                chars.push('=');
                remaining += 5isize.pow(index as u32) * 2;
            }
        }
        println!("index: {}", index);
        println!(
            "chars: {:?}, index: {}, remaining: {}",
            chars, index, remaining
        );
        index -= 1;
    }
}

fn snafu_to_num(snafu: &String) -> usize {
    let mut chars = snafu.chars().collect::<Vec<char>>();
    chars.reverse();
    let mut num = 0;
    for (i, c) in chars.iter().enumerate() {
        let scale = 5usize.pow(i as u32);
        match c {
            '0' => num += 0 * scale,
            '1' => num += 1 * scale,
            '2' => num += 2 * scale,
            '-' => num -= 1 * scale,
            '=' => num -= 2 * scale,
            _ => panic!("Invalid character"),
        }
    }
    num
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_25_snafu.txt");

    let test_cases: Vec<(usize, String)> = vec![
        (1, String::from("1")),
        (2, String::from("2")),
        (3, String::from("1=")),
        (4, String::from("1-")),
        (5, String::from("10")),
        (6, String::from("11")),
        (7, String::from("12")),
        (8, String::from("2=")),
        (9, String::from("2-")),
        (10, String::from("20")),
        (15, String::from("1=0")),
        (20, String::from("1-0")),
        (2022, String::from("1=11-2")),
        (12345, String::from("1-0---0")),
        (314159265, String::from("1121-1110-1=0")),
        (1747, String::from("1=-0-2")),
        (906, String::from("12111")),
        (198, String::from("2=0=")),
        (11, String::from("21")),
        (201, String::from("2=01")),
        (31, String::from("111")),
        (1257, String::from("20012")),
        (32, String::from("112")),
        (353, String::from("1=-1=")),
        (107, String::from("1-12")),
        (7, String::from("12")),
        (3, String::from("1=")),
        (37, String::from("122")),
        (4890, String::from("2=-1=0")),
        (976, String::from("2=-01")),
    ];

    for (input, expected) in &test_cases {
        assert_eq!(&num_to_snafu(&input), expected);
    }

    let solution = lines.iter().map(|line| snafu_to_num(line)).sum::<usize>();

    println!("Runtime: {:.2?}", now.elapsed());
    // String::from("meow")
    num_to_snafu(&solution)
}

#[test]
fn result() {
    let test_cases: Vec<(usize, String)> = vec![
        (1, String::from("1")),
        (2, String::from("2")),
        (3, String::from("1=")),
        (4, String::from("1-")),
        (5, String::from("10")),
        (6, String::from("11")),
        (7, String::from("12")),
        (8, String::from("2=")),
        (9, String::from("2-")),
        (10, String::from("20")),
        (15, String::from("1=0")),
        (20, String::from("1-0")),
        (2022, String::from("1=11-2")),
        (12345, String::from("1-0---0")),
        (314159265, String::from("1121-1110-1=0")),
        (1747, String::from("1=-0-2")),
        (906, String::from("12111")),
        (198, String::from("2=0=")),
        (11, String::from("21")),
        (201, String::from("2=01")),
        (31, String::from("111")),
        (1257, String::from("20012")),
        (32, String::from("112")),
        (353, String::from("1=-1=")),
        (107, String::from("1-12")),
        (7, String::from("12")),
        (3, String::from("1=")),
        (37, String::from("122")),
        (4890, String::from("2=-1=0")),
        (976, String::from("2=-01")),
    ];

    for (input, expected) in &test_cases {
        assert_eq!(&num_to_snafu(&input), expected);
    }
    for (expected, input) in &test_cases {
        assert_eq!(&snafu_to_num(&input), expected);
    }
    assert_eq!(solve(), "4890");
}
