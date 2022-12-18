use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

fn get_cubes(lines: Vec<String>) -> Vec<Cube> {
    let mut cubes = vec![];
    for line in lines {
        let mut parts = line.split(',');
        cubes.push(Cube {
            x: parts.next().unwrap().parse::<i32>().unwrap(),
            y: parts.next().unwrap().parse::<i32>().unwrap(),
            z: parts.next().unwrap().parse::<i32>().unwrap(),
        });
    }
    cubes
}

fn get_surface(cubes: Vec<Cube>) -> usize {
    let mut surface = 0;
    for cube in &cubes {
        let mut neighbours = 0;
        for n in &cubes {
            let distance = (cube.x - n.x).abs() + (cube.y - n.y).abs() + (cube.z - n.z).abs();
            if distance == 1 {
                neighbours += 1;
            }
        }
        surface += 6 - neighbours;
    }
    surface
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_18_cubes.txt");
    let cubes = get_cubes(lines);
    let surface = get_surface(cubes);
    println!("Runtime: {:.2?}", now.elapsed());
    surface.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "4580");
}
