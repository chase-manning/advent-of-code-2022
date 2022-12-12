use crate::utils::files::{get_data_as_lines, last_number};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: char,
    value: usize,
    check: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

fn get_monkeys(lines: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for i in 0..8 {
        let monkey_pos = i * 7;
        let mut value_string = lines[monkey_pos + 2][25..].split(' ').next().unwrap();
        if value_string == "old" {
            value_string = "0";
        }
        monkeys.push(Monkey {
            items: lines[monkey_pos + 1][18..]
                .split(", ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            operation: lines[monkey_pos + 2][23..=24].chars().next().unwrap(),
            value: value_string.parse::<usize>().unwrap(),
            check: last_number(&lines[monkey_pos + 3]),
            true_monkey: last_number(&lines[monkey_pos + 4]),
            false_monkey: last_number(&lines[monkey_pos + 5]),
            inspections: 0,
        });
    }
    monkeys
}

fn lowest_common_multiple(items: Vec<usize>) -> usize {
    let mut lcm = items[0];
    for i in 1..items.len() {
        let mut a = lcm;
        let mut b = items[i];
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        lcm = (lcm * items[i]) / a;
    }
    lcm
}

pub fn solve() -> String {
    let mut monkeys = get_monkeys(get_data_as_lines("day_11_monkey.txt"));

    let lcm = lowest_common_multiple(monkeys.iter().map(|m| m.check).collect());

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items {
                let mut new_item = item;
                let mut value = monkey.value;
                if value == 0 {
                    value = item;
                }
                match monkey.operation {
                    '+' => new_item += value,
                    '*' => new_item *= value,
                    _ => panic!("Unknown operation"),
                }
                new_item %= lcm;
                if new_item % monkey.check == 0 {
                    monkeys[monkey.true_monkey].items.push(new_item);
                } else {
                    monkeys[monkey.false_monkey].items.push(new_item);
                }
            }
            monkeys[i].inspections += monkeys[i].items.len();
            monkeys[i].items = Vec::new();
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    (inspections[0] * inspections[1]).to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "11309046332");
}
