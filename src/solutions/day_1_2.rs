use crate::utils::files::get_data_as_lines;

pub fn solve() -> String {
    let lines = get_data_as_lines("day_1_calories.txt");

    let mut first_elf_calories = 0;
    let mut second_elf_calories = 0;
    let mut third_elf_calories = 0;
    let mut elf_calories = 0;
    for line in lines {
        if line.is_empty() {
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
    (first_elf_calories + second_elf_calories + third_elf_calories).to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "204639");
}
