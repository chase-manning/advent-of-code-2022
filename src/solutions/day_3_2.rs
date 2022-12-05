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

fn find_badge(a: &str, b: &str, c: &str) -> Option<char> {
    for a_char in a.chars() {
        if b.contains(a_char) && c.contains(a_char) {
            return Some(a_char);
        }
    }
    panic!("No matching character found");
}

pub fn solve() -> String {
    let mut total_priority = 0;
    let lines = get_data_as_lines("day_3_rucksacks.txt");
    for i in 0..(lines.len() / 3) {
        let base = i * 3;
        let badge = find_badge(&lines[base], &lines[base + 1], &lines[base + 2]);
        total_priority += character_priority(&badge.unwrap().to_string());
    }
    total_priority.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "2641");
}
