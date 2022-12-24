use crate::utils::files::get_data_as_lines;
use std::time::Instant;

const MAX_X: usize = 100;
const MAX_Y: usize = 35;
const ENTRANCE: (isize, isize) = (0, -1);
const EXIT: (isize, isize) = (MAX_X as isize - 1, MAX_Y as isize);

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

impl Blizzard {
    pub fn new(x: usize, y: usize, dir: Dir) -> Blizzard {
        Blizzard { x, y, dir }
    }
}

fn get_blizzards(lines: Vec<String>) -> Vec<Blizzard> {
    let mut blizzards = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' || c == '#' {
                continue;
            }
            blizzards.push(Blizzard::new(
                x - 1,
                y - 1,
                match c {
                    '^' => Dir::U,
                    'v' => Dir::D,
                    '<' => Dir::L,
                    '>' => Dir::R,
                    _ => panic!("Unknown square"),
                },
            ));
        }
    }
    blizzards
}

fn move_blizzards(blizzards: &mut Vec<Blizzard>) {
    let mut new_blizzards = Vec::new();
    for b in &mut blizzards.iter() {
        let (new_x, new_y) = match b.dir {
            Dir::D => (b.x as isize, b.y as isize + 1),
            Dir::U => (b.x as isize, b.y as isize - 1),
            Dir::L => (b.x as isize - 1, b.y as isize),
            Dir::R => (b.x as isize + 1, b.y as isize),
        };
        if new_x < 0 {
            new_blizzards.push(Blizzard::new(MAX_X - 1, new_y.try_into().unwrap(), b.dir));
        } else if new_x >= MAX_X as isize {
            new_blizzards.push(Blizzard::new(0, new_y.try_into().unwrap(), b.dir));
        } else if new_y < 0 {
            new_blizzards.push(Blizzard::new(new_x.try_into().unwrap(), MAX_Y - 1, b.dir));
        } else if new_y >= MAX_Y as isize {
            new_blizzards.push(Blizzard::new(new_x.try_into().unwrap(), 0, b.dir));
        } else {
            new_blizzards.push(Blizzard::new(
                new_x.try_into().unwrap(),
                new_y.try_into().unwrap(),
                b.dir,
            ));
        }
    }
    *blizzards = new_blizzards;
}

fn has_blizzard(blizzards: &[Blizzard], x: isize, y: isize) -> bool {
    blizzards
        .iter()
        .any(|b| b.x as isize == x && b.y as isize == y)
}

fn get_movement_options(
    expedition: &(isize, isize),
    blizzards: &[Blizzard],
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

impl Path {
    pub fn new(steps: usize, position: (isize, isize)) -> Path {
        Path { steps, position }
    }
}

fn get_steps_needed_for_quickest_path_to_exit(
    blizzards: &mut Vec<Blizzard>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> usize {
    let mut paths = Vec::new();
    paths.push(Path::new(0, *start));
    loop {
        move_blizzards(blizzards);
        let mut new_paths: Vec<Path> = Vec::new();
        for path in paths {
            let new_positions = get_movement_options(&path.position, blizzards, start, end);
            for p in new_positions {
                if !new_paths
                    .iter()
                    .any(|p2| p2.position == p && p2.steps <= path.steps + 1)
                {
                    new_paths.push(Path::new(path.steps + 1, p))
                }
                if p.0 == end.0 as isize && p.1 == end.1 as isize {
                    return path.steps + 1;
                }
            }
        }
        paths = new_paths;
    }
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_24_blizzard.txt");

    let mut blizzards = get_blizzards(lines);
    let mut steps = get_steps_needed_for_quickest_path_to_exit(&mut blizzards, &ENTRANCE, &EXIT);
    steps += get_steps_needed_for_quickest_path_to_exit(&mut blizzards, &EXIT, &ENTRANCE);
    steps += get_steps_needed_for_quickest_path_to_exit(&mut blizzards, &ENTRANCE, &EXIT);

    println!("Runtime: {:.2?}", now.elapsed());
    steps.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "739");
}
