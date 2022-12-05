use crate::utils::files::get_data_as_lines;

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

pub fn solve() -> String {
    let mut lines = get_data_as_lines("day_5_crates.txt");
    lines.reverse();

    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut processing_crates = false;
    for line in lines {
        if processing_crates {
            let chars: Vec<char> = line.chars().collect();
            if chars[1] == '1' {
                continue;
            }
            for i in 0..9 {
                let index = i * 4 + 1;
                if chars[index] != ' ' {
                    if crates.len() <= i {
                        crates.push(Vec::new());
                    }
                    crates[i].push(chars[index]);
                }
            }
        } else {
            if line == "" {
                processing_crates = true;
                continue;
            }
            let mut split = line.split(' ');
            let amount = split.nth(1).unwrap().parse::<usize>().unwrap();
            let from = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let to = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            instructions.push(Instruction { from, to, amount });
        }
    }

    instructions.reverse();
    for instruction in instructions {
        let from = instruction.from;
        let to = instruction.to;
        let mut amount = instruction.amount;
        while amount > 0 {
            let crate_from = crates[from].pop().unwrap();
            crates[to].push(crate_from);
            amount -= 1;
        }
    }

    crates
        .iter()
        .map(|c| c[c.len() - 1].to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn result() {
    assert_eq!(solve(), "MQSHJMWNH");
}
