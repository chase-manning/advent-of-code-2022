use crate::utils::files::get_data_as_lines;

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

pub fn solve() -> String {
    let mut lines = get_data_as_lines("day_5_crates.txt");

    // Populating data
    let mut crates: Vec<Vec<char>> = (0..9).map(|_| Vec::new()).collect();
    let mut instructions: Vec<Instruction> = Vec::new();
    lines.reverse();
    lines.iter().for_each(|line| {
        // Handling crates
        if line.contains("[") {
            let chars: Vec<char> = line.chars().collect();
            (0..9).for_each(|i| {
                let char = chars[i * 4 + 1];
                if char != ' ' {
                    crates[i].push(char);
                }
            });
        }

        // Handling instructions
        if line.contains("move") {
            let mut split = line.split(' ');
            instructions.push(Instruction {
                amount: split.nth(1).unwrap().parse::<usize>().unwrap(),
                from: split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                to: split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            });
        }
    });

    // Moving crates
    instructions.reverse();
    instructions.iter().for_each(|instruction| {
        let mut meow: Vec<char> = (0..instruction.amount)
            .map(|_| crates[instruction.from].pop().unwrap())
            .collect();
        meow.reverse();
        crates[instruction.to].append(&mut meow);
    });

    // Returning result
    crates
        .iter()
        .map(|c| c.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn result() {
    assert_eq!(solve(), "LLWJRBHVZ");
}
