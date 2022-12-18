use std::collections::HashMap;
use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u32,
    paths: Vec<String>,
}

fn get_valves(lines: Vec<String>) -> (Vec<String>, HashMap<String, Valve>) {
    let mut valve_names = Vec::new();
    let mut valves = HashMap::new();

    for line in lines {
        let mut parts = line.split(' ');
        let name = parts.nth(1).unwrap().to_string();
        let flow_rate = parts
            .nth(2)
            .unwrap()
            .trim_end_matches(';')
            .split('=')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let _ = parts.nth(3).unwrap();
        let paths = parts
            .into_iter()
            .map(|s| s.trim_end_matches(',').to_string())
            .collect::<Vec<String>>();
        valve_names.push(name.clone());
        valves.insert(name, Valve { flow_rate, paths });
    }

    (valve_names, valves)
}

fn get_shortest_path(
    valves: &HashMap<String, Valve>,
    start: &String,
    end: &String,
    steps: u32,
    current_path: Vec<String>,
    shortest_distances: &HashMap<(String, String), u32>,
) -> (u32, Vec<String>) {
    if start == end {
        return (steps, current_path);
    }
    if let Some(distance) = shortest_distances.get(&(start.clone(), end.clone())) {
        return (steps + *distance, current_path);
    }
    let mut shortest_distance = std::u32::MAX;
    let mut shortest_path = Vec::new();
    let valve = valves.get(start).unwrap();
    for path in &valve.paths {
        if current_path.contains(path) {
            continue;
        }
        let mut branch_path = current_path.clone();
        branch_path.push(path.clone());
        let (distance, new_path) = get_shortest_path(
            valves,
            path,
            end,
            steps + 1,
            branch_path,
            shortest_distances,
        );
        if distance < shortest_distance {
            shortest_distance = distance;
            shortest_path = new_path;
        }
    }
    (shortest_distance, shortest_path)
}

fn get_shortest_distances(
    valve_names: &Vec<String>,
    valves: &HashMap<String, Valve>,
) -> HashMap<(String, String), u32> {
    let mut shortest_distances = HashMap::new();
    for from_name in valve_names {
        for to_name in valve_names {
            let (distance, path) = get_shortest_path(
                valves,
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
                    let distance = (i as i32 - j as i32).unsigned_abs();
                    shortest_distances.insert((path[i].clone(), path[j].clone()), distance);
                }
            }
        }
    }

    shortest_distances
}

fn get_paths(
    valves: &HashMap<String, Valve>,
    current_closed_valve_names: Vec<String>,
    current_path: Vec<String>,
    current_valve_name: String,
    seconds_remaining: u32,
    current_pressure: usize,
    distances: &HashMap<(String, String), u32>,
    paths: &mut Vec<(Vec<String>, usize)>,
) -> usize {
    let mut max_pressure = current_pressure;
    for (i, valve_name) in current_closed_valve_names.iter().enumerate() {
        let valve = valves.get(valve_name).unwrap();
        let seconds_to_valve = distances[&(current_valve_name.clone(), valve_name.clone())] + 1;
        if seconds_to_valve >= seconds_remaining {
            continue;
        }
        let seconds_remaining = seconds_remaining - seconds_to_valve;
        let mut branch_closed_valve_names = current_closed_valve_names.clone();
        branch_closed_valve_names.remove(i);
        let mut branch_path = current_path.clone();
        branch_path.push(valve_name.clone());
        let pressure = get_paths(
            valves,
            branch_closed_valve_names,
            branch_path,
            valve_name.clone(),
            seconds_remaining,
            current_pressure + (valve.flow_rate as usize * seconds_remaining as usize),
            distances,
            paths,
        );
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }
    let mut sorted_path = current_path;
    sorted_path.sort();
    paths.push((sorted_path, current_pressure));
    max_pressure
}

fn get_pruned_values(
    valve_names: &Vec<String>,
    valves: &HashMap<String, Valve>,
) -> (Vec<String>, HashMap<String, Valve>) {
    let mut pruned_names = Vec::new();
    let mut pruned_valves = HashMap::new();

    for name in valve_names {
        let valve = valves.get(name).unwrap();
        if valve.flow_rate > 0 {
            pruned_names.push(name.clone());
            pruned_valves.insert(name.clone(), valve.clone());
        }
    }

    (pruned_names, pruned_valves)
}

fn get_best_path_data(
    paths: Vec<(Vec<String>, usize)>,
) -> (Vec<Vec<String>>, HashMap<Vec<String>, usize>) {
    let mut unique_paths: Vec<Vec<String>> = Vec::new();
    let mut path_data: HashMap<Vec<String>, usize> = HashMap::new();
    for (path, pressure) in paths {
        if !unique_paths.contains(&path) && !path.is_empty() {
            unique_paths.push(path.clone());
        }
        let current_pressure = path_data.get(&path).unwrap_or(&0);
        if pressure > *current_pressure {
            path_data.insert(path, pressure);
        }
    }

    (unique_paths, path_data)
}

fn get_max_pressure(paths: Vec<Vec<String>>, path_data: HashMap<Vec<String>, usize>) -> usize {
    let mut max_pressure = 0;
    for i in 0..paths.len() {
        for j in i + 1..paths.len() {
            let pressure = path_data[&paths[i]] + path_data[&paths[j]];
            if pressure > max_pressure {
                let found = &paths[i].iter().any(|valve| paths[j].contains(valve));
                if !found {
                    max_pressure = pressure;
                }
            }
        }
    }

    max_pressure
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_16_valves.txt");
    let (valve_names, valves) = get_valves(lines);
    let shortest_distances = get_shortest_distances(&valve_names, &valves);
    let (pruned_names, pruned_valves) = get_pruned_values(&valve_names, &valves);
    let mut paths: Vec<(Vec<String>, usize)> = Vec::new();
    get_paths(
        &pruned_valves,
        pruned_names,
        vec![],
        String::from("AA"),
        26,
        0,
        &shortest_distances,
        &mut paths,
    );
    let (unique_paths, path_data) = get_best_path_data(paths);
    let max_pressure = get_max_pressure(unique_paths, path_data);
    println!("Runtime: {:.2?}", now.elapsed());
    max_pressure.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "2772");
}
