use std::collections::HashMap;
use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    paths: Vec<String>,
}

fn get_shortest_path(
    valves: &Vec<Valve>,
    start: &String,
    end: &String,
    steps: u32,
    visited: Vec<String>,
    shortest_distances: &HashMap<(String, String), u32>,
) -> (u32, Vec<String>) {
    if start == end {
        return (steps, visited);
    }
    if let Some(distance) = shortest_distances.get(&(start.clone(), end.clone())) {
        return (steps + *distance, visited);
    }
    let mut shortest_distance = std::u32::MAX;
    let mut shortest_path = Vec::new();
    let valve = valves.iter().find(|v| v.name == *start).unwrap();
    for path in &valve.paths {
        if visited.contains(path) {
            continue;
        }
        let mut visited = visited.clone();
        visited.push(path.clone());
        let (distance, visited_) = get_shortest_path(
            valves,
            path,
            end,
            steps.clone() + 1,
            visited,
            shortest_distances,
        );
        if distance < shortest_distance {
            shortest_distance = distance;
            shortest_path = visited_;
        }
    }
    (shortest_distance, shortest_path)
}

fn get_valves(lines: Vec<String>) -> Vec<Valve> {
    let mut valves = Vec::new();

    for line in lines {
        let mut parts = line.split(" ");
        let name = parts.nth(1).unwrap().to_string();
        let flow_rate = parts
            .nth(2)
            .unwrap()
            .trim_end_matches(";")
            .split("=")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let _ = parts.nth(3).unwrap();
        let paths = parts
            .into_iter()
            .map(|s| s.trim_end_matches(",").to_string())
            .collect::<Vec<String>>();
        valves.push(Valve {
            name,
            flow_rate,
            paths,
        });
    }

    valves
}

fn get_shortest_distances(valves: &Vec<Valve>) -> HashMap<(String, String), u32> {
    let mut shortest_distances = HashMap::new();
    let valve_names = valves
        .iter()
        .map(|v| v.name.clone())
        .collect::<Vec<String>>();

    for from_name in &valve_names {
        for to_name in &valve_names {
            let (distance, path) = get_shortest_path(
                &valves,
                from_name,
                to_name,
                0,
                vec![from_name.clone()],
                &shortest_distances,
            );
            shortest_distances.insert((from_name.clone(), to_name.clone()), distance);
            shortest_distances.insert((to_name.clone(), from_name.clone()), distance);

            for i in 0..path.len() {
                for j in 0..path.len() {
                    let from = path[i].clone();
                    let to = path[j].clone();
                    let distance = (i as i32 - j as i32).abs() as u32;
                    shortest_distances.insert((from, to), distance);
                }
            }
        }
    }

    shortest_distances
}

struct Path {
    valves: Vec<String>,
    pressure: u32,
}

fn get_nodes(
    valves: &Vec<Valve>,
    unopen_valves: Vec<String>,
    current_valve_name: String,
    seconds_remaining: u32,
    current_pressure: usize,
    shortest_distances: &HashMap<(String, String), u32>,
    nodes: &mut Vec<Path>,
) -> usize {
    let mut max_pressure = current_pressure;
    for valve_name in unopen_valves.clone() {
        let valve = valves.iter().find(|v| v.name == valve_name).unwrap();
        let seconds_to_valve =
            shortest_distances[&(current_valve_name.clone(), valve_name.clone())] + 1;
        if seconds_to_valve >= seconds_remaining {
            continue;
        }
        let seconds_remaining = seconds_remaining - seconds_to_valve;
        let mut unopen_valves = unopen_valves.clone();
        unopen_valves.retain(|v| v != &valve_name);
        let pressure = get_nodes(
            valves,
            unopen_valves,
            valve_name.clone(),
            seconds_remaining,
            current_pressure + (valve.flow_rate as usize * seconds_remaining as usize),
            shortest_distances,
            nodes,
        );
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }
    max_pressure
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_16_valves.txt");
    let valves = get_valves(lines);
    let shortest_distances = get_shortest_distances(&valves);
    let pruned_valves: Vec<Valve> = valves
        .iter()
        .filter(|v| v.flow_rate > 0 || v.name == "AA")
        .cloned()
        .collect();

    let mut nodes: Vec<Path> = Vec::new();
    let max_pressure = get_nodes(
        &pruned_valves,
        pruned_valves.iter().map(|v| v.name.clone()).collect(),
        String::from("AA"),
        30,
        0,
        &shortest_distances,
        &mut nodes,
    );
    println!("Runtime: {:.2?}", now.elapsed());
    max_pressure.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1915");
}
