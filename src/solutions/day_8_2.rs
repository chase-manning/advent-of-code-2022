use crate::utils::files::get_data_as_lines;

fn visible_trees_bottom(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let current_tree = trees[y][x];
    let mut visible_trees = 1;
    for i in y + 1..trees.len() - 1 {
        if trees[i][x] < current_tree {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_top(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let current_tree = trees[y][x];
    let mut visible_trees = 1;
    for i in (1..y).rev().into_iter() {
        if trees[i][x] < current_tree {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_right(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let current_tree = trees[y][x];
    let mut visible_trees = 1;
    for i in x + 1..trees[0].len() - 1 {
        if trees[y][i] < current_tree {
            visible_trees += 1;
        } else {
            return visible_trees;
        }
    }
    visible_trees
}

fn visible_trees_left(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let current_tree = trees[y][x];
    let mut visible_trees = 1;
    for i in (1..x).rev().into_iter() {
        if trees[y][i] < current_tree {
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

    // Return max visible score
    let mut max_score = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for y in 1..trees.len() - 1 {
        for x in 1..trees[0].len() - 1 {
            let score = visible_trees_score(&trees, x, y);
            if score > max_score {
                max_score = score;
                max_x = x;
                max_y = y;
            }
        }
    }
    max_score.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "234416");
}
