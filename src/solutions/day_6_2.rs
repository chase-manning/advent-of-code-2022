use crate::utils::files::get_data_as_lines;

fn all_chars_are_different(chars: &Vec<char>) -> bool {
    let mut chars = chars.clone();
    chars.sort();
    chars.dedup();
    chars.len() == chars.capacity()
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_6_packet.txt");
    let chars = &lines[0].chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        if all_chars_are_different(&chars[i..i + 14].to_vec()) {
            return (i + 14).to_string();
        }
    }
    panic!("No solution found");
}

#[test]
fn result() {
    assert_eq!(solve(), "3697");
}
