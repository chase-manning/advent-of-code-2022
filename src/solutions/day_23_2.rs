use crate::utils::files::get_data_as_lines;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn get_elves(lines: Vec<String>) -> Vec<(isize, isize)> {
    let mut elves = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => elves.push((x as isize, y as isize)),
                '.' => (),
                _ => panic!("Unknown square"),
            }
        }
    }
    elves
}

fn can_propose_direction(elves: &[(isize, isize)], elf: &(isize, isize), direction: &Dir) -> bool {
    let (a, b, c) = match direction {
        Dir::N => (
            (elf.0, elf.1 - 1),
            (elf.0 - 1, elf.1 - 1),
            (elf.0 + 1, elf.1 - 1),
        ),
        Dir::E => (
            (elf.0 + 1, elf.1),
            (elf.0 + 1, elf.1 - 1),
            (elf.0 + 1, elf.1 + 1),
        ),
        Dir::S => (
            (elf.0, elf.1 + 1),
            (elf.0 - 1, elf.1 + 1),
            (elf.0 + 1, elf.1 + 1),
        ),
        Dir::W => (
            (elf.0 - 1, elf.1),
            (elf.0 - 1, elf.1 - 1),
            (elf.0 - 1, elf.1 + 1),
        ),
    };
    !elves.iter().any(|e| e == &a || e == &b || e == &c)
}

fn get_position_after_move(elf: &(isize, isize), direction: &Dir) -> (isize, isize) {
    match direction {
        Dir::N => (elf.0, elf.1 - 1),
        Dir::E => (elf.0 + 1, elf.1),
        Dir::S => (elf.0, elf.1 + 1),
        Dir::W => (elf.0 - 1, elf.1),
    }
}

fn is_alone(elves: &[(isize, isize)], elf: &(isize, isize)) -> bool {
    !elves
        .iter()
        .any(|e| e != elf && (e.0 - elf.0).abs() <= 1 && (e.1 - elf.1).abs() <= 1)
}

fn move_elves(elves: &mut [(isize, isize)]) -> usize {
    let directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];
    let mut iterations = 0;
    'main: loop {
        let mut elf_proposals: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
        let mut proposed_count: HashMap<(isize, isize), usize> = HashMap::new();
        for elf in elves.iter() {
            if is_alone(elves, elf) {
                continue;
            }
            for j in 0..4 {
                let direction = directions[(iterations + j) % 4];
                if can_propose_direction(elves, elf, &direction) {
                    let pos = get_position_after_move(elf, &direction);
                    elf_proposals.insert(*elf, pos);
                    proposed_count.insert(pos, proposed_count.get(&pos).unwrap_or(&0) + 1);
                    break;
                }
            }
        }
        iterations += 1;
        if elf_proposals.is_empty() {
            break 'main;
        }
        for (elf, position) in &elf_proposals {
            if proposed_count.get(position).unwrap() == &1 {
                let index = elves.iter().position(|e| e == elf).unwrap();
                elves[index] = *position;
            }
        }
    }
    iterations
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_23_elves.txt");

    let mut elves = get_elves(lines);
    let solution = move_elves(&mut elves);

    println!("Runtime: {:.2?}", now.elapsed());
    solution.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "908");
}
