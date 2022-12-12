use crate::utils::files::get_data_as_lines;

pub fn solve() -> String {
    let lines = get_data_as_lines("day_10_commands.txt");
    let cycles = [20, 60, 100, 140, 180, 220];

    let mut cycle = 1;
    let mut x = 1;
    let mut strength = 0;
    for line in lines {
        if cycles.contains(&cycle) {
            strength += x * cycle;
        }
        cycle += 1;
        if line != "noop" {
            let increment = line.split(' ').last().unwrap().parse::<i64>().unwrap();
            if cycles.contains(&cycle) {
                strength += x * cycle;
            }
            cycle += 1;
            x += increment;
        }
    }

    strength.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "14860");
}
