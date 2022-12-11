use crate::utils::files::get_data_as_lines;

fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    (0..x).all(|i| trees[y][i] < trees[y][x])
        || (x + 1..trees[0].len()).all(|i| trees[y][i] < trees[y][x])
        || (0..y).all(|i| trees[i][x] < trees[y][x])
        || (y + 1..trees.len()).all(|i| trees[i][x] < trees[y][x])
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

    (0..trees.len())
        .map(|y| {
            (0..trees[0].len())
                .filter(|&x| is_visible(&trees, x, y))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1776");
}
