use crate::utils::files::{get_data_as_lines, split2};

pub fn solve() -> String {
    let lines = get_data_as_lines("day_9_rope.txt");

    let mut tail = (0, 0);
    let mut head = (0, 0);
    let mut history = vec![(0, 0)];

    for line in lines {
        let (direction, movements) = split2(line, " ");
        for _ in (0..movements).into_iter() {
            head = match direction {
                'U' => (head.0, head.1 + 1),
                'D' => (head.0, head.1 - 1),
                'R' => (head.0 + 1, head.1),
                'L' => (head.0 - 1, head.1),
                _ => panic!("Invalid direction"),
            };

            if head.0 > tail.0 + 1 {
                tail = (tail.0 + 1, head.1);
            } else if head.0 < tail.0 - 1 {
                tail = (tail.0 - 1, head.1);
            } else if head.1 > tail.1 + 1 {
                tail = (head.0, tail.1 + 1);
            } else if head.1 < tail.1 - 1 {
                tail = (head.0, tail.1 - 1);
            }
            history.push(tail);
        }
    }

    history.sort();
    history.dedup();
    history.len().to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "5874");
}
