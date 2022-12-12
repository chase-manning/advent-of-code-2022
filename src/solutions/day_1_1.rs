use crate::utils::files::get_data_as_lines;

pub fn solve() -> String {
    let lines = get_data_as_lines("day_1_calories.txt");

    let mut highest = 0;
    let mut elf_calories = 0;
    for line in lines {
        if line.is_empty() {
            if elf_calories > highest {
                highest = elf_calories;
            }
            elf_calories = 0;
            continue;
        }
        let calories = line.parse::<i32>().unwrap();
        elf_calories += calories;
    }
    highest.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "71124");
}
