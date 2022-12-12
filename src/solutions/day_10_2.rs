use crate::utils::files::get_data_as_lines;

fn is_within_pixel(x: i64, cycle: i64) -> bool {
    let pixel = cycle % 40 - 1;
    pixel >= x - 1 && pixel <= x + 1
}

fn print_display(display: &Vec<bool>) {
    for i in 0..display.len() {
        if i % 40 == 0 {
            println!();
        }
        if display[i] {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!();
    println!();
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_10_commands.txt");

    let mut cycle = 1;
    let mut x = 1;
    let mut display: Vec<bool> = Vec::new();
    for line in lines {
        display.push(is_within_pixel(x, cycle));
        cycle += 1;
        if line != "noop" {
            let increment = line.split(' ').last().unwrap().parse::<i64>().unwrap();
            display.push(is_within_pixel(x, cycle));
            cycle += 1;
            x += increment;
        }
    }

    print_display(&display);

    display
        .iter()
        .map(|b| if *b { "#" } else { "." })
        .collect::<Vec<&str>>()
        .join("")
}

#[test]
fn result() {
    assert_eq!(solve(), "###...##..####.####.#..#.#..#.###..#..###..#.#..#....#.#....#..#.#..#.#..#.#.#.##..#.#......#..###..####.#..#.#..#.##...###..#.##..#...#....#..#.#..#.###..#.#.##.#..#..#.#....#....#..#.#..#.#.#..#.#.##..#..###.####.####.#..#..##..#..#.#..#.");
}
