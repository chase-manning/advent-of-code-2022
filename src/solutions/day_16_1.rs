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
    valve_names: &Vec<String>,
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
            valve_names,
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
                valve_names,
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

fn get_max_pressure(
    valves: &HashMap<String, Valve>,
    current_closed_valve_names: Vec<String>,
    current_valve_name: String,
    seconds_remaining: u32,
    current_pressure: usize,
    shortest_distances: &HashMap<(String, String), u32>,
) -> usize {
    let mut max_pressure = current_pressure;
    for valve_name in &current_closed_valve_names {
        let valve = valves.get(valve_name).unwrap();
        let seconds_to_valve =
            shortest_distances[&(current_valve_name.clone(), valve_name.clone())] + 1;
        if seconds_to_valve >= seconds_remaining {
            continue;
        }
        let seconds_remaining = seconds_remaining - seconds_to_valve;
        let mut unopen_valves = current_closed_valve_names.clone();
        unopen_valves.retain(|v| v != valve_name);
        let pressure = get_max_pressure(
            valves,
            unopen_valves,
            valve_name.clone(),
            seconds_remaining,
            current_pressure + (valve.flow_rate as usize * seconds_remaining as usize),
            shortest_distances,
        );
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }
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

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_16_valves.txt");
    let (valve_names, valves) = get_valves(lines);
    let shortest_distances = get_shortest_distances(&valve_names, &valves);
    let (pruned_names, pruned_valves) = get_pruned_values(&valve_names, &valves);
    let max_pressure = get_max_pressure(
        &pruned_valves,
        pruned_names,
        String::from("AA"),
        30,
        0,
        &shortest_distances,
    );
    println!("Runtime: {:.2?}", now.elapsed());
    max_pressure.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1915");
}
