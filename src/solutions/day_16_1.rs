use std::collections::HashMap;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    paths: Vec<String>,
    valve_distance: HashMap<String, u32>,
}

fn shortest_path(
    valves: &Vec<Valve>,
    start: &String,
    end: &String,
    steps: u32,
    visited: Vec<String>,
) -> u32 {
    if start == end {
        return steps;
    }
    let mut shortest_distance = std::u32::MAX;
    let valve = valves.iter().find(|v| v.name == *start).unwrap();
    for path in &valve.paths {
        if visited.contains(path) {
            continue;
        }
        let mut visited = visited.clone();
        visited.push(path.clone());
        let distance = shortest_path(valves, path, end, steps.clone() + 1, visited);
        if distance < shortest_distance {
            shortest_distance = distance;
        }
    }
    shortest_distance
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
            valve_distance: HashMap::new(),
        });
    }

    let valves_copy = valves.clone();
    let valve_names = valves_copy
        .iter()
        .map(|v| v.name.clone())
        .collect::<Vec<String>>();

    println!("valve_names: {:?}", valve_names);
    for valve in &mut valves {
        for path in &valve_names {
            let distance = shortest_path(&valves_copy, &valve.name, path, 0, vec![]);
            valve.valve_distance.insert(path.clone(), distance);
        }
        println!("valve: {:?}", valve);
    }

    valves
}

fn get_max_pressure(
    valves: &Vec<Valve>,
    unopen_valves: Vec<String>,
    current_valve_name: String,
    seconds_remaining: u32,
    current_pressure: usize,
) -> usize {
    let mut max_pressure = current_pressure;
    let current_valve = valves
        .iter()
        .find(|v| v.name == current_valve_name)
        .unwrap();
    for valve_name in unopen_valves.clone() {
        let valve = valves.iter().find(|v| v.name == valve_name).unwrap();
        let seconds_to_valve = current_valve.valve_distance[&valve_name] + 1;
        if seconds_to_valve >= seconds_remaining {
            continue;
        }
        let seconds_remaining = seconds_remaining - seconds_to_valve;
        let mut unopen_valves = unopen_valves.clone();
        unopen_valves.retain(|v| v != &valve_name);
        let pressure = get_max_pressure(
            valves,
            unopen_valves,
            valve_name.clone(),
            seconds_remaining,
            current_pressure + (valve.flow_rate as usize * seconds_remaining as usize),
        );
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }
    max_pressure
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_16_valves.txt");
    let valves = get_valves(lines);
    println!("{:?}", valves.len());
    let pruned_valves: Vec<Valve> = valves
        .iter()
        .filter(|v| v.flow_rate > 0 || v.name == "AA")
        .cloned()
        .collect();
    println!("{:?}", pruned_valves.len());
    let max_pressure = get_max_pressure(
        &pruned_valves,
        pruned_valves.iter().map(|v| v.name.clone()).collect(),
        String::from("AA"),
        30,
        0,
    );
    max_pressure.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1915");
}
