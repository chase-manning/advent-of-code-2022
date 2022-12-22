use crate::utils::files::get_data_as_lines;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq)]
enum Square {
    Grid,
    Air,
    Wall,
}

#[derive(Debug)]
struct Movement {
    steps: usize,
    direction: Option<char>,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
    direction: char,
}

#[derive(Debug, Clone, Copy)]
struct Portal {
    start: (isize, isize),
    vector: (isize, isize),
    direction: char,
}

impl Portal {
    fn new(start: (isize, isize), vector: (isize, isize), direction: char) -> Self {
        Portal {
            start,
            vector,
            direction,
        }
    }
}

fn get_map(lines: &Vec<String>) -> Vec<Vec<Square>> {
    let mut map = Vec::new();
    let row_width = lines.iter().map(|l| l.len()).max().unwrap();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.push(Square::Wall),
                '.' => row.push(Square::Grid),
                ' ' => row.push(Square::Air),
                _ => panic!("Unknown square"),
            }
        }
        while row.len() < row_width {
            row.push(Square::Air);
        }
        map.push(row);
    }
    map
}

fn get_movements(line: &str) -> Vec<Movement> {
    let mut movements = Vec::new();
    let chars = line.chars();
    let mut steps = 0;
    for c in chars {
        if c.is_numeric() {
            steps = steps * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            movements.push(Movement {
                steps,
                direction: Some(c),
            });
            steps = 0;
        }
    }
    movements.push(Movement {
        steps,
        direction: None,
    });

    movements
}

fn get_map_and_movements(lines: Vec<String>) -> (Vec<Vec<Square>>, Vec<Movement>) {
    let sep = lines.iter().position(|l| l.is_empty()).unwrap();
    (
        get_map(&lines[0..sep].to_vec()),
        get_movements(&lines[sep + 1].to_string()),
    )
}

fn add_portals(portals: &mut HashMap<(isize, isize), Position>, portal_data: (Portal, Portal)) {
    for i in 0..50 {
        let (x, y) = portal_data.0.start;
        let (dx, dy) = portal_data.0.vector;
        let (x2, y2) = portal_data.1.start;
        let (dx2, dy2) = portal_data.1.vector;
        portals.insert(
            (x + dx * i as isize, y + dy * i as isize),
            Position {
                x: x2 + dx2 * i as isize,
                y: y2 + dy2 * i as isize,
                direction: portal_data.1.direction,
            },
        );
    }
}

fn get_portals() -> HashMap<(isize, isize), Position> {
    let portal_data: Vec<(Portal, Portal)> = vec![
        (
            Portal::new((0, 99), (1, 0), 'X'),
            Portal::new((50, 50), (0, 1), 'R'),
        ),
        (
            Portal::new((49, 50), (0, 1), 'R'),
            Portal::new((0, 100), (1, 0), 'D'),
        ),
        (
            Portal::new((49, 0), (0, 1), 'X'),
            Portal::new((0, 149), (0, -1), 'R'),
        ),
        (
            Portal::new((-1, 149), (0, -1), 'R'),
            Portal::new((50, 0), (0, 1), 'R'),
        ),
        (
            Portal::new((50, -1), (1, 0), 'X'),
            Portal::new((0, 150), (0, 1), 'R'),
        ),
        (
            Portal::new((-1, 150), (0, 1), 'R'),
            Portal::new((50, 0), (1, 0), 'D'),
        ),
        (
            Portal::new((100, -1), (1, 0), 'X'),
            Portal::new((0, 199), (1, 0), 'U'),
        ),
        (
            Portal::new((0, 200), (1, 0), 'U'),
            Portal::new((100, 0), (1, 0), 'D'),
        ),
        (
            Portal::new((150, 0), (0, 1), 'X'),
            Portal::new((99, 149), (0, -1), 'L'),
        ),
        (
            Portal::new((100, 149), (0, -1), 'L'),
            Portal::new((149, 0), (0, 1), 'L'),
        ),
        (
            Portal::new((100, 50), (1, 0), 'X'),
            Portal::new((99, 50), (0, 1), 'L'),
        ),
        (
            Portal::new((100, 50), (0, 1), 'L'),
            Portal::new((100, 49), (1, 0), 'U'),
        ),
        (
            Portal::new((50, 150), (1, 0), 'X'),
            Portal::new((49, 150), (0, 1), 'L'),
        ),
        (
            Portal::new((50, 150), (0, 1), 'L'),
            Portal::new((50, 149), (1, 0), 'U'),
        ),
    ];

    let mut portals = HashMap::new();
    for (portal1, portal2) in portal_data {
        add_portals(&mut portals, (portal1, portal2));
    }
    portals
}

fn get_starting_position(map: &[Vec<Square>]) -> Position {
    for (i, square) in map[0].iter().enumerate() {
        if let Square::Grid = square {
            return Position {
                x: i as isize,
                y: 0,
                direction: 'R',
            };
        }
    }
    panic!("No starting position found");
}

fn get_direction_vector(direction: &char) -> (isize, isize) {
    match direction {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Unknown direction"),
    }
}

fn requires_teleport(map: &Vec<Vec<Square>>, position: &mut Position) -> bool {
    position.x >= map[0].len() as isize
        || position.y >= map.len() as isize
        || position.x < 0
        || position.y < 0
        || Square::Air == map[position.y as usize][position.x as usize]
}

fn teleport(position: &mut Position, portals: &HashMap<(isize, isize), Position>) {
    let (x, y) = (position.x, position.y);
    let new_position = portals.get(&(x, y)).unwrap();
    position.x = new_position.x;
    position.y = new_position.y;
    position.direction = new_position.direction;
}

fn move_one_step(
    map: &Vec<Vec<Square>>,
    direction: &char,
    position: &mut Position,
    portals: &HashMap<(isize, isize), Position>,
) {
    let vector = get_direction_vector(direction);
    position.x += vector.0;
    position.y += vector.1;
    if requires_teleport(map, position) {
        teleport(position, portals);
    }
}

fn is_colliding(map: &[Vec<Square>], position: &Position) -> bool {
    matches!(map[position.y as usize][position.x as usize], Square::Wall)
}

fn update_direction(position: &mut Position, direction: &Option<char>) {
    match position.direction {
        'U' => match direction {
            Some('L') => position.direction = 'L',
            Some('R') => position.direction = 'R',
            None => {}
            _ => panic!("Unknown direction"),
        },
        'D' => match direction {
            Some('L') => position.direction = 'R',
            Some('R') => position.direction = 'L',
            None => {}
            _ => panic!("Unknown direction"),
        },
        'L' => match direction {
            Some('L') => position.direction = 'D',
            Some('R') => position.direction = 'U',
            None => {}
            _ => panic!("Unknown direction"),
        },
        'R' => match direction {
            Some('L') => position.direction = 'U',
            Some('R') => position.direction = 'D',
            None => {}
            _ => panic!("Unknown direction"),
        },
        _ => panic!("Unknown direction"),
    }
}

fn complete_map(map: &Vec<Vec<Square>>, movements: &Vec<Movement>, position: &mut Position) {
    let portals = get_portals();
    for movement in movements {
        'stepping: for _ in 0..movement.steps {
            let pos_before = *position;
            move_one_step(map, &position.direction.clone(), position, &portals);
            if is_colliding(map, position) {
                *position = pos_before;
                break 'stepping;
            }
        }
        update_direction(position, &movement.direction);
    }
}

fn get_password(position: &Position) -> isize {
    let direction_score = match position.direction {
        'U' => 3,
        'D' => 1,
        'L' => 2,
        'R' => 0,
        _ => panic!("Unknown direction"),
    };
    1000 * (position.y + 1) + 4 * (position.x + 1) + direction_score
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_22_password.txt");

    let (map, movements) = get_map_and_movements(lines);
    let mut position = get_starting_position(&map);
    complete_map(&map, &movements, &mut position);
    let password = get_password(&position);

    println!("Runtime: {:.2?}", now.elapsed());
    password.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "134076");
}
