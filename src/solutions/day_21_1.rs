use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Sub,
    Div,
    Ret,
}

#[derive(Debug, Clone)]
struct Node {
    value: isize,
    children: Vec<Node>,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct FlatNode {
    name: String,
    value: isize,
    children: Vec<String>,
    operation: Operation,
}

fn get_flat_nodes(lines: Vec<String>) -> Vec<FlatNode> {
    let mut nodes = Vec::new();
    for line in lines {
        let mut parts = line.split(' ');
        let name = parts.next().unwrap().to_string().replace(':', "");
        let next = parts.next().unwrap();
        let operation_string = parts.next();
        if operation_string.is_none() {
            nodes.push(FlatNode {
                name,
                value: next.parse::<isize>().unwrap(),
                children: vec![],
                operation: Operation::Ret,
            });
            continue;
        }
        let operation = match operation_string.unwrap() {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            "-" => Operation::Sub,
            "/" => Operation::Div,
            _ => panic!("Unknown operation"),
        };
        let second_child = parts.next().unwrap().to_string();
        nodes.push(FlatNode {
            name,
            value: 0,
            children: vec![next.to_string(), second_child],
            operation,
        });
    }
    nodes
}

fn get_node(node: String, flat_nodes: &Vec<FlatNode>) -> Node {
    let flat_node = flat_nodes
        .iter()
        .find(|flat_node| flat_node.name == node)
        .unwrap();
    Node {
        value: flat_node.value,
        children: flat_node
            .children
            .iter()
            .map(|child| get_node(child.to_string(), flat_nodes))
            .collect(),
        operation: flat_node.operation,
    }
}

fn get_total(nodes: &Node) -> isize {
    match nodes.operation {
        Operation::Add => get_total(&nodes.children[0]) + get_total(&nodes.children[1]),
        Operation::Mul => get_total(&nodes.children[0]) * get_total(&nodes.children[1]),
        Operation::Sub => get_total(&nodes.children[0]) - get_total(&nodes.children[1]),
        Operation::Div => get_total(&nodes.children[0]) / get_total(&nodes.children[1]),
        Operation::Ret => nodes.value,
    }
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_21_monkeys.txt");
    let flat_nodes = get_flat_nodes(lines);
    let nodes = get_node(String::from("root"), &flat_nodes);
    let total = get_total(&nodes);
    println!("Runtime: {:.2?}", now.elapsed());
    total.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "286698846151845");
}
