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
            minutes: 32,
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
    max_ore: u32,
    max_clay: u32,
    max_obsidian: u32,
}

fn get_blueprints(lines: Vec<String>) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    for (i, line) in lines.iter().enumerate() {
        let mut parts = line.split(' ');
        let ore = Cost {
            ore: parts.nth(6).unwrap().parse::<u32>().unwrap(),
            clay: 0,
            obsidian: 0,
        };
        let clay = Cost {
            ore: parts.nth(5).unwrap().parse::<u32>().unwrap(),
            clay: 0,
            obsidian: 0,
        };
        let obsidian = Cost {
            ore: parts.nth(5).unwrap().parse::<u32>().unwrap(),
            clay: parts.nth(2).unwrap().parse::<u32>().unwrap(),
            obsidian: 0,
        };
        let geode = Cost {
            ore: parts.nth(5).unwrap().parse::<u32>().unwrap(),
            clay: 0,
            obsidian: parts.nth(2).unwrap().parse::<u32>().unwrap(),
        };
        let max_clay = obsidian.clay;
        let max_ore = std::cmp::max(
            geode.ore,
            std::cmp::max(ore.ore, std::cmp::max(clay.ore, obsidian.ore)),
        );
        let max_obsidian = geode.obsidian;

        blueprints.push(Blueprint {
            id: i as u32 + 1,
            ore,
            clay,
            obsidian,
            geode,
            max_clay,
            max_ore,
            max_obsidian,
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
    let robot = match state.next_robot {
        Some(RobotType::Ore) => &blueprint.ore,
        Some(RobotType::Clay) => &blueprint.clay,
        Some(RobotType::Obsidian) => &blueprint.obsidian,
        Some(RobotType::Geode) => &blueprint.geode,
        _ => return false,
    };
    can_build(robot, &state.inventory)
}

fn build(state: &mut State, cost: &Cost) {
    state.inventory.ore -= cost.ore;
    state.inventory.clay -= cost.clay;
    state.inventory.obsidian -= cost.obsidian;
}

fn build_robot(state: &mut State, blueprint: &Blueprint) {
    match state.next_robot {
        Some(RobotType::Ore) => {
            build(state, &blueprint.ore);
            state.robots.ore += 1;
        }
        Some(RobotType::Clay) => {
            build(state, &blueprint.clay);
            state.robots.clay += 1;
        }
        Some(RobotType::Obsidian) => {
            build(state, &blueprint.obsidian);
            state.robots.obsidian += 1;
        }
        Some(RobotType::Geode) => {
            build(state, &blueprint.geode);
            state.robots.geode += 1;
        }
        _ => {
            panic!("No robot to build");
        }
    }
}

fn get_robot_type_options(state: &State, blueprint: &Blueprint) -> Vec<RobotType> {
    ROBOT_TYPES
        .iter()
        .filter(|robot_type| match **robot_type {
            RobotType::Obsidian => {
                state.robots.clay > 0 && state.inventory.obsidian <= blueprint.max_obsidian
            }
            RobotType::Geode => state.robots.obsidian > 0,
            RobotType::Ore => state.inventory.ore <= blueprint.max_ore,
            RobotType::Clay => true,
        })
        .map(|robot_type| *robot_type)
        .collect()
}

fn get_quality_level(blueprint: &Blueprint, state: &mut State) -> u32 {
    while state.minutes > 0 {
        if state.next_robot.is_none() {
            let options = get_robot_type_options(state, blueprint);
            return *options
                .iter()
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
    state.inventory.geode
}

fn get_total_quality_level(blueprints: Vec<Blueprint>) -> u32 {
    let mut total_quality_level = 1;
    for blueprint in blueprints[..3].iter() {
        println!("Blueprint: {}", blueprint.id);
        total_quality_level *= get_quality_level(&blueprint, &mut State::new());
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
    assert_eq!(solve(), "88160");
}
