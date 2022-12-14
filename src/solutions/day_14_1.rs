use std::collections::HashMap;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Point {
    Sand,
    Rock,
}

fn get_points(line: &String) -> Vec<Vec<usize>> {
    line.split(" -> ")
        .into_iter()
        .map(|x| x.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
        .collect()
}

fn get_grid(points: &Vec<Vec<Vec<usize>>>) -> HashMap<(usize, usize), Point> {
    let mut grid = HashMap::new();
    for point in points {
        for i in 1..point.len() {
            let from_x = point[i - 1][0];
            let from_y = point[i - 1][1];
            let to_x = point[i][0];
            let to_y = point[i][1];

            let min_x = from_x.min(to_x);
            let max_x = from_x.max(to_x);
            let min_y = from_y.min(to_y);
            let max_y = from_y.max(to_y);

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid.entry((x, y)).or_insert(Point::Rock);
                }
            }
        }
    }
    grid
}

fn rock_floor(grid: &HashMap<(usize, usize), Point>) -> usize {
    *grid.keys().map(|(_, y)| y).max().unwrap()
}

fn move_sand(
    grid: &HashMap<(usize, usize), Point>,
    sand: (usize, usize),
) -> Option<(usize, usize)> {
    let one_below = (sand.0, sand.1 + 1);
    if !grid.contains_key(&one_below) {
        return Some(one_below);
    }

    let one_diagonal_left = (sand.0 - 1, sand.1 + 1);
    if !grid.contains_key(&one_diagonal_left) {
        return Some(one_diagonal_left);
    }

    let one_diagonal_right = (sand.0 + 1, sand.1 + 1);
    if !grid.contains_key(&one_diagonal_right) {
        return Some(one_diagonal_right);
    }

    None
}

fn add_sand(grid: &mut HashMap<(usize, usize), Point>, bottom: usize) -> bool {
    let mut sand = (500, 0);
    while sand.1 <= bottom {
        if let Some(new_sand) = move_sand(&grid, sand) {
            sand = new_sand;
        } else {
            grid.insert(sand, Point::Sand);
            return true;
        }
    }
    false
}

fn add_all_sand(grid: &mut HashMap<(usize, usize), Point>) {
    let bottom = rock_floor(&grid);
    loop {
        if !add_sand(grid, bottom) {
            break;
        }
    }
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_14_sand.txt");

    let points: Vec<Vec<Vec<usize>>> = lines.iter().map(|l| get_points(l)).collect();
    let mut grid = get_grid(&points);

    add_all_sand(&mut grid);

    grid.values()
        .filter(|p| **p == Point::Sand)
        .count()
        .to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "665");
}
