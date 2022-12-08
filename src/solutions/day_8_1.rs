use crate::utils::files::get_data_as_lines;

fn is_visible_bottom(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let current_tree = trees[y][x];
    for i in y + 1..trees.len() {
        if trees[i][x] >= current_tree {
            return false;
        }
    }
    return true;
}

fn is_visible_top(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let current_tree = trees[y][x];
    for i in 0..y {
        if trees[i][x] >= current_tree {
            return false;
        }
    }
    return true;
}

fn is_visible_right(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let current_tree = trees[y][x];
    for i in x + 1..trees[0].len() {
        if trees[y][i] >= current_tree {
            return false;
        }
    }
    return true;
}

fn is_visible_left(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let current_tree = trees[y][x];
    for i in 0..x {
        if trees[y][i] >= current_tree {
            return false;
        }
    }
    return true;
}

fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    is_visible_bottom(trees, x, y)
        || is_visible_top(trees, x, y)
        || is_visible_right(trees, x, y)
        || is_visible_left(trees, x, y)
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

    let mut visible_trees = 0;
    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            if is_visible(&trees, x, y) {
                visible_trees += 1;
            }
        }
    }
    visible_trees.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "3866390");
}
