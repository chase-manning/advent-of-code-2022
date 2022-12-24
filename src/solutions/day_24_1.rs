use crate::utils::files::get_data_as_lines;
use std::{char::MAX, collections::HashMap, time::Instant};

const MAX_X: usize = 100;
const MAX_Y: usize = 35;
const ENTRANCE: (isize, isize) = (0, -1);
const EXIT: (usize, usize) = (MAX_X - 1, MAX_Y);
const MAX_PATHS: usize = 20000;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    D,
    U,
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blizzard {
    x: usize,
    y: usize,
    dir: Dir,
}

fn get_blizzards(lines: Vec<String>) -> Vec<Blizzard> {
    let mut Blizzards = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => Blizzards.push(Blizzard {
                    x: x - 1,
                    y: y - 1,
                    dir: Dir::U,
                }),
                'v' => Blizzards.push(Blizzard {
                    x: x - 1,
                    y: y - 1,
                    dir: Dir::D,
                }),
                '<' => Blizzards.push(Blizzard {
                    x: x - 1,
                    y: y - 1,
                    dir: Dir::L,
                }),
                '>' => Blizzards.push(Blizzard {
                    x: x - 1,
                    y: y - 1,
                    dir: Dir::R,
                }),
                '.' => (),
                '#' => (),
                _ => panic!("Unknown square"),
            };
        }
    }
    Blizzards
}

fn print_blizzards(blizzards: &Vec<Blizzard>, expedition: &(isize, isize)) {
    (0..(MAX_X + 2)).for_each(|i| {
        if expedition.1 == -1 && expedition.0 == i as isize - 1 {
            print!("E");
            return;
        }
        if i == 1 {
            print!(".");
        } else {
            print!("#");
        }
    });
    println!();
    for y in 0..MAX_Y {
        print!("#");
        for x in 0..MAX_X {
            if x as isize == expedition.0 && y as isize == expedition.1 {
                print!("E");
                continue;
            }
            let square = blizzards.iter().filter(|b| b.x == x && b.y == y);
            let pos_count = square.clone().count();
            if pos_count == 0 {
                print!(".");
                continue;
            }
            if pos_count > 1 {
                print!("{}", pos_count);
                continue;
            }
            match square.clone().next().unwrap().dir {
                Dir::D => print!("v"),
                Dir::U => print!("^"),
                Dir::L => print!("<"),
                Dir::R => print!(">"),
            }
        }
        println!("#");
    }
    (0..(MAX_X + 2)).for_each(|i| {
        if expedition.1 == (MAX_Y + 1) as isize && expedition.0 == i as isize - 1 {
            print!("E");
            return;
        }
        if i == MAX_X {
            print!(".");
        } else {
            print!("#");
        }
    });
    println!();
    println!();
}

fn move_blizzards(blizzards: Vec<Blizzard>) -> Vec<Blizzard> {
    let mut new_blizzards = Vec::new();
    for b in blizzards {
        match b.dir {
            Dir::D => {
                if b.y == MAX_Y - 1 {
                    new_blizzards.push(Blizzard {
                        x: b.x,
                        y: 0,
                        dir: Dir::D,
                    });
                    continue;
                }
            }
            Dir::U => {
                if b.y == 0 {
                    new_blizzards.push(Blizzard {
                        x: b.x,
                        y: MAX_Y - 1,
                        dir: Dir::U,
                    });
                    continue;
                }
            }
            Dir::L => {
                if b.x == 0 {
                    new_blizzards.push(Blizzard {
                        x: MAX_X - 1,
                        y: b.y,
                        dir: Dir::L,
                    });
                    continue;
                }
            }
            Dir::R => {
                if b.x == MAX_X - 1 {
                    new_blizzards.push(Blizzard {
                        x: 0,
                        y: b.y,
                        dir: Dir::R,
                    });
                    continue;
                }
            }
        }

        let (new_x, new_y) = match b.dir {
            Dir::D => (b.x, b.y + 1),
            Dir::U => (b.x, b.y - 1),
            Dir::L => (b.x - 1, b.y),
            Dir::R => (b.x + 1, b.y),
        };
        new_blizzards.push(Blizzard {
            x: new_x,
            y: new_y,
            dir: b.dir,
        });
    }
    new_blizzards
}

fn has_blizzard(blizzards: &Vec<Blizzard>, x: isize, y: isize) -> bool {
    blizzards
        .iter()
        .any(|b| b.x as isize == x && b.y as isize == y)
}

fn get_expedition_movement_options(
    expedition: &(isize, isize),
    blizzards: &Vec<Blizzard>,
) -> Vec<(isize, isize)> {
    vec![(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .map(|(x, y)| (expedition.0 + x, expedition.1 + y))
        .filter(|(x, y)| {
            (x == &(EXIT.0 as isize) && y == &(EXIT.1 as isize))
                || (x == &(ENTRANCE.0 as isize) && y == &(ENTRANCE.1 as isize))
                || (*x >= 0
                    && *x < MAX_X as isize
                    && *y >= 0
                    && *y < MAX_Y as isize
                    && !has_blizzard(blizzards, *x, *y))
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Path {
    steps: usize,
    position: (isize, isize),
}

fn distance_from_exit(position: &(isize, isize)) -> usize {
    ((position.0 - EXIT.0 as isize).abs() + (position.1 - EXIT.1 as isize).abs()) as usize
}

// Remove paths at the same position with a higher step count
fn prune_paths(paths: Vec<Path>) -> Vec<Path> {
    let mut new_paths: Vec<Path> = Vec::new();
    for path in paths {
        if new_paths
            .iter()
            .any(|p| p.position == path.position && p.steps < path.steps)
        {
            continue;
        }
        new_paths.push(path);
    }

    // if new_paths.len() > MAX_PATHS {
    // sort from lowest distance from exit to highest
    new_paths.sort_by(|a, b| distance_from_exit(&a.position).cmp(&distance_from_exit(&b.position)));
    if new_paths.len() > 2 {
        assert!(
            distance_from_exit(&new_paths.first().unwrap().position)
                < distance_from_exit(&new_paths.last().unwrap().position)
        );
        println!(
            "Closest position {}",
            distance_from_exit(&new_paths.first().unwrap().position)
        );
    }
    new_paths.truncate(MAX_PATHS);
    // }

    new_paths
}

fn get_steps_needed_for_quickest_path_to_exit(lines: Vec<String>) -> usize {
    let mut blizzards = get_blizzards(lines);
    let mut paths = Vec::new();
    paths.push(Path {
        steps: 0,
        position: ENTRANCE,
    });
    loop {
        blizzards = move_blizzards(blizzards);
        let mut new_paths: Vec<Path> = Vec::new();
        for path in paths {
            let new_positions = get_expedition_movement_options(&path.position, &blizzards);
            for p in new_positions {
                if p.0 == EXIT.0 as isize && p.1 == EXIT.1 as isize {
                    return path.steps + 1;
                }
                if new_paths
                    .iter()
                    .any(|p2| p2.position == p && p2.steps <= path.steps + 1)
                {
                    continue;
                }
                new_paths.push(Path {
                    steps: path.steps + 1,
                    position: p,
                })
            }
        }
        paths = new_paths;
    }
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_24_blizzard.txt");

    let steps = get_steps_needed_for_quickest_path_to_exit(lines);

    println!("Runtime: {:.2?}", now.elapsed());
    steps.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "221");
}
