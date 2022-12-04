use crate::utils::files::get_data_as_lines;

fn is_overlapping(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.0
        || a.0 <= b.1 && a.1 >= b.1
        || a.0 >= b.0 && a.1 <= b.1
        || a.0 <= b.0 && a.1 >= b.1
}

fn get_range(line: &str) -> (i32, i32) {
    let mut iter = line.split("-");
    let min = iter.next().unwrap().parse::<i32>().unwrap();
    let max = iter.next().unwrap().parse::<i32>().unwrap();
    (min, max)
}

pub fn solve() -> i32 {
    let lines = get_data_as_lines("day_4_assignments.txt");
    let mut overlapping = 0;
    for line in lines {
        let mut split = line.split(',');
        let first_elf_range = get_range(split.next().unwrap());
        let second_elf_range = get_range(split.next().unwrap());
        if is_overlapping(first_elf_range, second_elf_range) {
            overlapping += 1;
        }
    }
    overlapping
}

#[test]
fn result() {
    assert_eq!(solve(), 823);
}
