use crate::utils::files::get_data_as_lines;
use std::{char::MAX, collections::HashMap, time::Instant};

const MAX_X: usize = 100;
const MAX_Y: usize = 35;
const ENTRANCE: (isize, isize) = (0, -1);
const EXIT: (isize, isize) = (MAX_X as isize - 1, MAX_Y as isize);
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

fn move_blizzards(blizzards: &mut Vec<Blizzard>) {
    let mut new_blizzards = Vec::new();
    for b in &mut blizzards.iter() {
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
    *blizzards = new_blizzards;
}

fn has_blizzard(blizzards: &Vec<Blizzard>, x: isize, y: isize) -> bool {
    blizzards
        .iter()
        .any(|b| b.x as isize == x && b.y as isize == y)
}

fn get_expedition_movement_options(
    expedition: &(isize, isize),
    blizzards: &Vec<Blizzard>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> Vec<(isize, isize)> {
    vec![(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .map(|(x, y)| (expedition.0 + x, expedition.1 + y))
        .filter(|(x, y)| {
            (x == &end.0 && y == &end.1)
                || (x == &start.0 && y == &start.1)
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

fn get_steps_needed_for_quickest_path_to_exit(
    blizzards: &mut Vec<Blizzard>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> usize {
    let mut paths = Vec::new();
    paths.push(Path {
        steps: 0,
        position: *start,
    });
    loop {
        move_blizzards(blizzards);
        let mut new_paths: Vec<Path> = Vec::new();
        for path in paths {
            let new_positions =
                get_expedition_movement_options(&path.position, &blizzards, start, end);
            for p in new_positions {
                if p.0 == end.0 as isize && p.1 == end.1 as isize {
                    move_blizzards(blizzards);
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

    let mut blizzards = get_blizzards(lines);
    let steps = get_steps_needed_for_quickest_path_to_exit(&mut blizzards, &ENTRANCE, &EXIT);

    println!("Runtime: {:.2?}", now.elapsed());
    steps.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "221");
}
