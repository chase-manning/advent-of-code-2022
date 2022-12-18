use std::{collections::HashMap, time::Instant};

use crate::utils::files::get_data_as_lines;

struct Piece {
    width: usize,
    height: usize,
    shape: Vec<Vec<bool>>,
}

fn pieces() -> Vec<Piece> {
    vec![
        Piece {
            width: 4,
            height: 1,
            shape: vec![vec![true, true, true, true]],
        },
        Piece {
            width: 3,
            height: 3,
            shape: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
        },
        Piece {
            width: 3,
            height: 3,
            shape: vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
        },
        Piece {
            width: 1,
            height: 4,
            shape: vec![vec![true], vec![true], vec![true], vec![true]],
        },
        Piece {
            width: 2,
            height: 2,
            shape: vec![vec![true, true], vec![true, true]],
        },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Left,
    Right,
}

fn get_movements(lines: Vec<String>) -> Vec<Move> {
    assert_eq!(lines.len(), 1);
    lines
        .first()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Unknown move: {}", c),
        })
        .collect::<Vec<Move>>()
}

fn is_colliding(
    grid: &HashMap<(usize, usize), bool>,
    piece: &Piece,
    piece_x: &usize,
    piece_y: &usize,
) -> bool {
    for y in 0..piece.height {
        for x in 0..piece.width {
            if piece.shape[y as usize][x as usize] {
                if let Some(true) = grid.get(&(piece_x + x as usize, piece_y + y as usize)) {
                    return true;
                }
            }
        }
    }
    false
}

fn grid_cache(grid: &HashMap<(usize, usize), bool>, grid_top: &usize, piece: &Piece) -> String {
    let cache_depth = 100;
    let mut s = String::new();
    let y_start = if *grid_top > cache_depth {
        *grid_top - cache_depth
    } else {
        0
    };
    for y in y_start..*grid_top {
        for x in 0..7 {
            if let Some(true) = grid.get(&(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
    }

    for y in 0..piece.height {
        for x in 0..piece.width {
            if piece.shape[y as usize][x as usize] {
                s.push('@');
            } else {
                s.push('_');
            }
        }
    }

    s
}

fn play_tetris(moves: Vec<Move>) -> usize {
    let cycle_start = 153;
    let cycle_length = 1695;
    let increase_per_cycle = 2671;
    let target: usize = 1_000_000_000_000;
    let remaining = target % cycle_length;
    let cycles = target / cycle_length;
    let pieces = pieces();
    let grid_width = 7;
    let mut grid: HashMap<(usize, usize), bool> = HashMap::new();
    let mut grid_top = 0;
    let mut rocks_stopped = 0;
    let mut moves_made = 0;
    let mut cycle_start_top = 0;
    let mut cycle_end_top = 0;
    while rocks_stopped < remaining {
        let piece = &pieces[rocks_stopped % 5];
        let mut piece_y = grid_top + 3;
        let mut piece_x = 2;
        loop {
            let m = &moves[moves_made % moves.len()];
            if m == &Move::Left {
                if piece_x != 0 && !is_colliding(&grid, piece, &(piece_x - 1), &piece_y) {
                    piece_x -= 1;
                }
            } else if piece_x != grid_width - piece.width
                && !is_colliding(&grid, piece, &(piece_x + 1), &piece_y)
            {
                piece_x += 1;
            }
            moves_made += 1;
            if piece_y == 0 {
                break;
            }
            if is_colliding(&grid, piece, &piece_x, &(piece_y - 1)) {
                break;
            }
            piece_y -= 1;
        }
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.shape[y as usize][x as usize] {
                    grid.insert((piece_x + x as usize, piece_y + y as usize), true);
                }
            }
        }

        grid_top = if piece_y + piece.height > grid_top {
            piece_y + piece.height
        } else {
            grid_top
        };
        rocks_stopped += 1;
    }
    grid_top + cycles * increase_per_cycle
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_17_tetris.txt");
    let moves = get_movements(lines);
    let result = play_tetris(moves);
    println!("Runtime: {:.2?}", now.elapsed());
    result.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1575811209487");
}
