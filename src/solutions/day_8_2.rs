use crate::utils::files::get_data_as_lines;

fn visible_trees_bottom(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut visible_trees = 1;
    for i in y + 1..trees.len() - 1 {
        if trees[i][x] < trees[y][x] {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_top(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut visible_trees = 1;
    for i in (1..y).rev().into_iter() {
        if trees[i][x] < trees[y][x] {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_right(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut visible_trees = 1;
    for i in x + 1..trees[0].len() - 1 {
        if trees[y][i] < trees[y][x] {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_left(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut visible_trees = 1;
    for i in (1..x).rev().into_iter() {
        if trees[y][i] < trees[y][x] {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    visible_trees_bottom(trees, x, y)
        * visible_trees_top(trees, x, y)
        * visible_trees_right(trees, x, y)
        * visible_trees_left(trees, x, y)
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_8_trees.txt");
    let trees: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    trees
        .iter()
        .enumerate()
        .filter(|&(y, _)| y != 0 && y != trees.len() - 1)
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, _)| x != 0 && x != row.len() - 1 && y != 0 && y != trees.len() - 1)
                .map(|(x, _)| visible_trees_score(&trees, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "234416");
}
