use crate::utils::files::get_data_as_lines;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: u64,
    children: Vec<Directory>,
    files: Vec<u64>,
}

fn add_child(root: &mut Directory, path: &Vec<String>, child_name: String) {
    if path.is_empty() {
        root.children.push(Directory {
            name: child_name,
            size: 0,
            children: Vec::new(),
            files: Vec::new(),
        });
    } else {
        let child = root.children.iter_mut().find(|child| child.name == path[0]);
        add_child(child.unwrap(), &path[1..].to_vec(), child_name);
    }
}

fn add_file(root: &mut Directory, path: &Vec<String>, file: u64) {
    if path.is_empty() {
        root.files.push(file);
    } else {
        let child = root.children.iter_mut().find(|child| child.name == path[0]);
        add_file(child.unwrap(), &path[1..].to_vec(), file);
    }
}

fn populate_sizes(root: &mut Directory) {
    for child in root.children.iter_mut() {
        populate_sizes(child);
        root.size += child.size;
    }
    for file in root.files.iter() {
        root.size += file;
    }
}

fn get_directory(lines: Vec<String>) -> Directory {
    let mut root = Directory {
        name: String::from("root"),
        size: 0,
        children: Vec::new(),
        files: Vec::new(),
    };

    let mut path: Vec<String> = Vec::new();
    for line in lines.iter() {
        if line == "$ ls" {
            continue;
        }
        if line == "$ cd /" {
            path = Vec::new();
            continue;
        }
        if line == "$ cd .." {
            path.pop();
            continue;
        }
        if line.contains("$ cd") {
            path.push(line.split(' ').last().unwrap().to_string());
            continue;
        }
        if line.contains("dir ") {
            let child_name = line.split(' ').last().unwrap().to_string();
            add_child(&mut root, &path, child_name);
            continue;
        }
        let data = line.split(' ').collect::<Vec<&str>>();
        add_file(&mut root, &path, data[0].parse::<u64>().unwrap());
    }

    populate_sizes(&mut root);
    root
}

fn get_sum_of_sizes_below_100000(root: &Directory) -> u64 {
    let mut sum = 0;
    for child in root.children.iter() {
        if child.size < 100000 {
            sum += child.size;
        }
        sum += get_sum_of_sizes_below_100000(child);
    }
    sum
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_7_commands.txt");
    let directory = get_directory(lines);
    get_sum_of_sizes_below_100000(&directory).to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "1297159");
}
