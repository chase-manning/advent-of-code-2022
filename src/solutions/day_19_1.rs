use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone, Copy)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const ROBOT_TYPES: [RobotType; 4] = [
    RobotType::Ore,
    RobotType::Clay,
    RobotType::Obsidian,
    RobotType::Geode,
];

#[derive(Debug, Clone)]
struct Materials {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Clone)]
struct State {
    minutes: u32,
    inventory: Materials,
    robots: Materials,
    next_robot: Option<RobotType>,
}

impl State {
    fn new() -> State {
        State {
            minutes: 24,
            inventory: Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Materials {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            next_robot: None,
        }
    }
}

#[derive(Debug)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

fn get_blueprints(lines: Vec<String>) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    for (i, line) in lines.iter().enumerate() {
        let mut parts = line.split(' ');
        blueprints.push(Blueprint {
            id: i as u32 + 1,
            ore: Cost {
                ore: parts.nth(6).unwrap().parse::<u32>().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: parts.nth(5).unwrap().parse::<u32>().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: parts.nth(5).unwrap().parse::<u32>().unwrap(),
                clay: parts.nth(2).unwrap().parse::<u32>().unwrap(),
                obsidian: 0,
            },
            geode: Cost {
                ore: parts.nth(5).unwrap().parse::<u32>().unwrap(),
                clay: 0,
                obsidian: parts.nth(2).unwrap().parse::<u32>().unwrap(),
            },
        });
    }
    blueprints
}

fn harvest_resources(state: &mut State) {
    state.inventory.ore += state.robots.ore;
    state.inventory.clay += state.robots.clay;
    state.inventory.obsidian += state.robots.obsidian;
    state.inventory.geode += state.robots.geode;
}

fn can_build(cost: &Cost, inventory: &Materials) -> bool {
    cost.ore <= inventory.ore && cost.clay <= inventory.clay && cost.obsidian <= inventory.obsidian
}

fn can_build_robot(state: &State, blueprint: &Blueprint) -> bool {
    match state.next_robot {
        Some(RobotType::Ore) => {
            return can_build(&blueprint.ore, &state.inventory);
        }
        Some(RobotType::Clay) => {
            return can_build(&blueprint.clay, &state.inventory);
        }
        Some(RobotType::Obsidian) => {
            return can_build(&blueprint.obsidian, &state.inventory);
        }
        Some(RobotType::Geode) => {
            return can_build(&blueprint.geode, &state.inventory);
        }
        _ => {
            return false;
        }
    }
}

fn build_robot(state: &mut State, blueprint: &Blueprint) {
    match state.next_robot {
        Some(RobotType::Ore) => {
            state.inventory.ore -= blueprint.ore.ore;
            state.inventory.clay -= blueprint.ore.clay;
            state.inventory.obsidian -= blueprint.ore.obsidian;
            state.robots.ore += 1;
        }
        Some(RobotType::Clay) => {
            state.inventory.ore -= blueprint.clay.ore;
            state.inventory.clay -= blueprint.clay.clay;
            state.inventory.obsidian -= blueprint.clay.obsidian;
            state.robots.clay += 1;
        }
        Some(RobotType::Obsidian) => {
            state.inventory.ore -= blueprint.obsidian.ore;
            state.inventory.clay -= blueprint.obsidian.clay;
            state.inventory.obsidian -= blueprint.obsidian.obsidian;
            state.robots.obsidian += 1;
        }
        Some(RobotType::Geode) => {
            state.inventory.ore -= blueprint.geode.ore;
            state.inventory.clay -= blueprint.geode.clay;
            state.inventory.obsidian -= blueprint.geode.obsidian;
            state.robots.geode += 1;
        }
        _ => {
            panic!("No robot to build");
        }
    }
}

fn get_quality_level(blueprint: &Blueprint, state: &mut State) -> u32 {
    while state.minutes > 0 {
        if !state.next_robot.is_some() {
            return *ROBOT_TYPES
                .iter()
                .filter(|robot_type| match **robot_type {
                    RobotType::Obsidian => {
                        return state.robots.clay > 0;
                    }
                    RobotType::Geode => {
                        return state.robots.obsidian > 0;
                    }
                    _ => return true,
                })
                .map(|robot_type| {
                    let mut path_state = state.clone();
                    path_state.next_robot = Some(*robot_type);
                    get_quality_level(blueprint, &mut path_state)
                })
                .collect::<Vec<u32>>()
                .iter()
                .max()
                .unwrap();
        }
        let can_build = can_build_robot(state, blueprint);
        harvest_resources(state);
        if can_build {
            build_robot(state, blueprint);
            state.next_robot = None;
        }
        state.minutes -= 1;
    }
    // if state.inventory.geode == 22 {
    //     println!("Found it: {:#?}", state);
    // }
    state.inventory.geode * blueprint.id
}

fn get_total_quality_level(blueprints: Vec<Blueprint>) -> u32 {
    let mut total_quality_level = 0;
    for blueprint in blueprints {
        let meow = get_quality_level(&blueprint, &mut State::new());
        println!("{}: {}", blueprint.id, meow);
        total_quality_level += meow;
    }
    total_quality_level
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_19_robots.txt");
    let blueprints = get_blueprints(lines);
    let total_quality_level = get_total_quality_level(blueprints);
    println!("Runtime: {:.2?}", now.elapsed());
    total_quality_level.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1962");
}
