use std::{collections::HashMap, time::Instant};

use crate::utils::files::get_data_as_lines;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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

fn get_neighbours(cube: &Cube) -> Vec<Cube> {
    let mut neighbours = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if (x as i32).abs() + (y as i32).abs() + (z as i32).abs() == 1 {
                    neighbours.push(Cube {
                        x: cube.x + x,
                        y: cube.y + y,
                        z: cube.z + z,
                    });
                }
            }
        }
    }
    neighbours
}

fn get_neighbour_cubes(cube: &Cube, cubes: &Vec<Cube>) -> Vec<Cube> {
    let mut neighbours = vec![];
    for n in cubes {
        let distance = (cube.x - n.x).abs() + (cube.y - n.y).abs() + (cube.z - n.z).abs();
        if distance == 1 {
            neighbours.push(*n);
        }
    }
    neighbours
}

fn is_air(
    cube: &Cube,
    cubes: &Vec<Cube>,
    min_x: &i32,
    max_x: &i32,
    min_y: &i32,
    max_y: &i32,
    min_z: &i32,
    max_z: &i32,
    visited: &mut Vec<Cube>,
    is_air_cache: &mut HashMap<Cube, bool>,
) -> bool {
    visited.push(*cube);

    // Not air if it's on the edge
    if cube.x == *min_x
        || cube.x == *max_x
        || cube.y == *min_y
        || cube.y == *max_y
        || cube.z == *min_z
        || cube.z == *max_z
    {
        return false;
    }

    // Not air if it has a neighbour that is not air
    let neighbours = get_neighbours(cube);
    let empty_neighbours = neighbours
        .iter()
        .filter(|n| {
            !cubes.contains(n) && !is_air_cache.get(n).unwrap_or(&false) && !visited.contains(*n)
        })
        .copied()
        .collect::<Vec<Cube>>();
    for n in &empty_neighbours {
        if !is_air(
            n,
            cubes,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
            visited,
            is_air_cache,
        ) {
            return false;
        }
    }

    is_air_cache.insert(*cube, true);
    true
}

fn get_air(cubes: &Vec<Cube>) -> Vec<Cube> {
    let min_x = cubes.iter().map(|c| c.x).min().unwrap();
    let max_x = cubes.iter().map(|c| c.x).max().unwrap();
    let min_y = cubes.iter().map(|c| c.y).min().unwrap();
    let max_y = cubes.iter().map(|c| c.y).max().unwrap();
    let min_z = cubes.iter().map(|c| c.z).min().unwrap();
    let max_z = cubes.iter().map(|c| c.z).max().unwrap();

    let mut is_air_cache: HashMap<Cube, bool> = HashMap::new();

    let mut air = vec![];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let cube = Cube { x, y, z };
                if cubes.contains(&cube) {
                    continue;
                }

                if is_air(
                    &cube,
                    cubes,
                    &min_x,
                    &max_x,
                    &min_y,
                    &max_y,
                    &min_z,
                    &max_z,
                    &mut vec![],
                    &mut is_air_cache,
                ) {
                    air.push(cube);
                }
            }
        }
    }

    air
}

fn get_surface(cubes: Vec<Cube>, inside: Vec<Cube>) -> usize {
    let mut surface = 0;
    for cube in &cubes {
        surface += 6 - get_neighbour_cubes(cube, &inside).len();
    }
    surface
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_18_cubes.txt");
    let cubes = get_cubes(lines);
    let mut air = get_air(&cubes);
    air.append(&mut cubes.clone());
    let surface = get_surface(cubes, air);
    println!("Runtime: {:.2?}", now.elapsed());
    surface.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "2610");
}
