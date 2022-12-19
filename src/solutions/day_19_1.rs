use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
    None,
}

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

fn buildable_robots(blueprint: &Blueprint, state: &State) -> Vec<RobotType> {
    let mut buildable_robots = vec![RobotType::None];
    if can_build(&blueprint.ore, &state.inventory) {
        buildable_robots.push(RobotType::Ore);
    }
    if can_build(&blueprint.clay, &state.inventory) {
        buildable_robots.push(RobotType::Clay);
    }
    if can_build(&blueprint.obsidian, &state.inventory) {
        buildable_robots.push(RobotType::Obsidian);
    }
    if can_build(&blueprint.geode, &state.inventory) {
        buildable_robots.push(RobotType::Geode);
    }
    buildable_robots
}

fn build_robot(robot_type: &RobotType, state: &mut State, blueprint: &Blueprint) {
    match robot_type {
        RobotType::Ore => {
            state.inventory.ore -= blueprint.ore.ore;
            state.inventory.clay -= blueprint.ore.clay;
            state.inventory.obsidian -= blueprint.ore.obsidian;
            state.robots.ore += 1;
        }
        RobotType::Clay => {
            state.inventory.ore -= blueprint.clay.ore;
            state.inventory.clay -= blueprint.clay.clay;
            state.inventory.obsidian -= blueprint.clay.obsidian;
            state.robots.clay += 1;
        }
        RobotType::Obsidian => {
            state.inventory.ore -= blueprint.obsidian.ore;
            state.inventory.clay -= blueprint.obsidian.clay;
            state.inventory.obsidian -= blueprint.obsidian.obsidian;
            state.robots.obsidian += 1;
        }
        RobotType::Geode => {
            state.inventory.ore -= blueprint.geode.ore;
            state.inventory.clay -= blueprint.geode.clay;
            state.inventory.obsidian -= blueprint.geode.obsidian;
            state.robots.geode += 1;
        }
        RobotType::None => {}
    }
}

fn get_quality_level(blueprint: &Blueprint, state: &mut State) -> u32 {
    // If it's the last minute just harvest and return
    state.minutes -= 1;
    if state.minutes == 0 {
        harvest_resources(state);
        return state.inventory.geode * blueprint.id;
    }

    // Otherwise build some robots
    let buildable_robots = buildable_robots(blueprint, state);
    harvest_resources(state);
    buildable_robots
        .iter()
        .map(|robot_type| {
            let mut path_state = state.clone();
            build_robot(robot_type, &mut path_state, blueprint);
            get_quality_level(blueprint, &mut path_state)
        })
        .max()
        .unwrap()
}

fn get_total_quality_level(blueprints: Vec<Blueprint>) -> u32 {
    let mut total_quality_level = 0;
    for blueprint in blueprints {
        total_quality_level += get_quality_level(&blueprint, &mut State::new());
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
    assert_eq!(solve(), "2610");
}
