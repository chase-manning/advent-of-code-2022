use crate::utils::files::get_data_as_lines;

fn character_priority(character: &str) -> i32 {
    let priority_indicator = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (priority_indicator
        .find(character)
        .unwrap_or_else(|| panic!("Invalid character: {}", character))
        + 1)
    .try_into()
    .unwrap()
}

fn find_matching_character(a: &str, b: &str) -> Option<char> {
    for a_char in a.chars() {
        if b.contains(a_char) {
            return Some(a_char);
        }
    }
    panic!("No matching character found");
}

pub fn solve() -> String {
    let mut total_priority = 0;
    for line in get_data_as_lines("day_3_rucksacks.txt") {
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];
        let matching_character = find_matching_character(first_half, second_half);
        total_priority += character_priority(&matching_character.unwrap().to_string());
    }
    total_priority.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "8401");
}
