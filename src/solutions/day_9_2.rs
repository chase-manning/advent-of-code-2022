use crate::utils::files::{get_data_as_lines, split2};

fn get_tail(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
    let is_up = head.1 > tail.1 + 1;
    let is_down = head.1 < tail.1 - 1;
    let is_right = head.0 > tail.0 + 1;
    let is_left = head.0 < tail.0 - 1;

    if is_up && is_right {
        return (tail.0 + 1, tail.1 + 1);
    } else if is_up && is_left {
        return (tail.0 - 1, tail.1 + 1);
    } else if is_down && is_right {
        return (tail.0 + 1, tail.1 - 1);
    } else if is_down && is_left {
        return (tail.0 - 1, tail.1 - 1);
    } else if is_up {
        return (head.0, tail.1 + 1);
    } else if is_down {
        return (head.0, tail.1 - 1);
    } else if is_right {
        return (tail.0 + 1, head.1);
    } else if is_left {
        return (tail.0 - 1, head.1);
    }
    tail
}

fn get_tails(head: (i64, i64), tails: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut new_tails = vec![];
    for i in (0..tails.len()).into_iter() {
        if i == 0 {
            new_tails.push(get_tail(head, tails[i]));
        } else {
            new_tails.push(get_tail(new_tails[i - 1], tails[i]));
        }
    }
    new_tails
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_9_rope.txt");

    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 9];
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

            tails = get_tails(head, tails);
            history.push(tails.last().unwrap().clone());
        }
    }

    history.sort();
    history.dedup();
    history.len().to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "2467");
}
