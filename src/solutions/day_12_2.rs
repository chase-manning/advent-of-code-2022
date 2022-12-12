use crate::utils::files::get_data_as_lines;

fn char_to_number(c: &char) -> u8 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .position(|x| x == *c)
            .unwrap()
            .try_into()
            .unwrap(),
    }
}

fn fastest_path(
    hills: &Vec<Vec<u8>>,
    position: (usize, usize),
    visited: &mut Vec<Vec<usize>>,
    steps: usize,
) -> usize {
    if position.0 == 20 && position.1 == 112 {
        return steps;
    }
    visited[position.0][position.1] = steps;
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(x, y)| (position.0 as i32 + x, position.1 as i32 + y))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|(x, y)| *x < hills.len() && *y < hills[0].len())
        .filter(|(x, y)| visited[*x][*y] > steps + 1)
        .filter(|(x, y)| hills[*x][*y] <= hills[position.0][position.1] + 1)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(x, y)| fastest_path(hills, (x, y), visited, steps + 1))
        .min()
        .unwrap_or(std::usize::MAX)
}

fn get_hills() -> Vec<Vec<u8>> {
    get_data_as_lines("day_12_hills.txt")
        .iter()
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(char_to_number)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

fn get_starting_positions(hills: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut starting_positions = Vec::new();
    for i in 0..hills.len() {
        for j in 0..hills[0].len() {
            if hills[i][j] == 0 {
                starting_positions.push((i, j));
            }
        }
    }
    starting_positions
}

pub fn solve() -> String {
    let hills = get_hills();
    let mut visited = vec![vec![std::usize::MAX; hills[0].len()]; hills.len()];
    get_starting_positions(&hills)
        .into_iter()
        .map(|starting_position| fastest_path(&hills, starting_position, &mut visited, 0))
        .min()
        .unwrap()
        .to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "454");
}
