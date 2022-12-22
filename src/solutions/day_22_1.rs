use crate::utils::files::get_data_as_lines;
use std::time::Instant;

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

fn get_movements(line: &String) -> Vec<Movement> {
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
    let sep = lines.iter().position(|l| l.len() == 0).unwrap();
    (
        get_map(&lines[0..sep].to_vec()),
        get_movements(&lines[sep + 1].to_string()),
    )
}

fn get_starting_position(map: &Vec<Vec<Square>>) -> Position {
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

fn invert_direction(direction: &char) -> char {
    match direction {
        'U' => 'D',
        'D' => 'U',
        'L' => 'R',
        'R' => 'L',
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

fn teleport(map: &Vec<Vec<Square>>, position: &mut Position, vector: &(isize, isize)) {
    loop {
        position.x = position.x - vector.0;
        position.y = position.y - vector.1;
        if requires_teleport(map, position) {
            position.x = position.x + vector.0;
            position.y = position.y + vector.1;
            break;
        }
    }
}

fn move_one_step(map: &Vec<Vec<Square>>, direction: &char, position: &mut Position) {
    let vector = get_direction_vector(direction);
    position.x = position.x + vector.0;
    position.y = position.y + vector.1;
    if requires_teleport(map, position) {
        teleport(map, position, &vector);
    }
}

fn is_colliding(map: &Vec<Vec<Square>>, position: &Position) -> bool {
    match map[position.y as usize][position.x as usize] {
        Square::Wall => true,
        _ => false,
    }
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
    for movement in movements {
        'stepping: for _ in 0..movement.steps {
            move_one_step(map, &position.direction.clone(), position);
            if is_colliding(map, position) {
                move_one_step(map, &invert_direction(&position.direction), position);
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
    assert_eq!(solve(), "3759566892641");
}
