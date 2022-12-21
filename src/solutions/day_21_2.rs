use std::time::Instant;

use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Sub,
    Div,
    Ret,
    Eql,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
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
        if nodes.len() == 0 {
            let operation = Operation::Eql;
        }
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
        name: flat_node.name.clone(),
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
        Operation::Eql => panic!("Eql should not be used here"),
    }
}

fn path_to_humn(node: &Node, current_path: Vec<usize>) -> (bool, Vec<usize>) {
    if node.name == "humn" {
        return (true, current_path);
    }
    if node.children.len() == 0 {
        return (false, vec![]);
    }
    let first_path = path_to_humn(&node.children[0], [&current_path[..], &[0]].concat());
    if first_path.0 {
        return first_path;
    }
    let second_path = path_to_humn(&node.children[1], [&current_path[..], &[1]].concat());
    if second_path.0 {
        return second_path;
    }
    (false, vec![])
}

fn invert_total(node: &Node, side: usize, output: isize) -> isize {
    let other_side = 1 - side;
    let other_input = get_total(&node.children[other_side]);
    if side == 0 {
        match node.operation {
            Operation::Add => output - other_input,
            Operation::Mul => output / other_input,
            Operation::Sub => output + other_input,
            Operation::Div => output * other_input,
            Operation::Ret => panic!("Ret should not be used here"),
            Operation::Eql => panic!("Eql should not be used here"),
        }
    } else {
        match node.operation {
            Operation::Add => output - other_input,
            Operation::Mul => output / other_input,
            Operation::Sub => -(output - other_input),
            Operation::Div => other_input / output,
            Operation::Ret => panic!("Ret should not be used here"),
            Operation::Eql => panic!("Eql should not be used here"),
        }
    }
}

fn get_humn_value(node: &Node, path: Vec<usize>, output: isize) -> isize {
    if path.len() == 0 {
        return output;
    }
    let side = path[0];
    let new_output = invert_total(node, side, output);
    let new_path = path[1..].to_vec();
    get_humn_value(&node.children[side], new_path, new_output)
}

pub fn solve() -> String {
    let now = Instant::now();
    let lines = get_data_as_lines("day_21_monkeys.txt");
    let flat_nodes = get_flat_nodes(lines);
    let nodes = get_node(String::from("root"), &flat_nodes);

    let path_to_human = path_to_humn(&nodes, vec![]).1;
    let target = get_total(&nodes.children[1 - path_to_human[0]]);
    let side = path_to_human[0];
    let new_path = path_to_human[1..].to_vec();
    let humn_value = get_humn_value(&nodes.children[side], new_path, target);
    println!("Runtime: {:.2?}", now.elapsed());
    humn_value.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "3759566892641");
}
