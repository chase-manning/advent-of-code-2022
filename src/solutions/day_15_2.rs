use crate::utils::files::get_data_as_lines;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
}

fn get_first_number(line: &String) -> (isize, String) {
    let equals_pos = line.chars().position(|c| c == '=').unwrap();
    let end_pos = line.len();
    let sep_pos = line
        .chars()
        .position(|c| c == ':' || c == ',')
        .unwrap_or(std::usize::MAX);
    let end = std::cmp::min(sep_pos, end_pos);
    let number = line[equals_pos + 1..end].parse::<isize>().unwrap();
    if end == end_pos {
        return (number, "".to_string());
    }
    (number, line[(end + 1)..].to_string())
}

fn get_sensor(line: &str) -> Sensor {
    let (sensor_x, sensor_x_rem) = get_first_number(&line.to_string());
    let (sensor_y, sensor_y_rem) = get_first_number(&sensor_x_rem);
    let (beacon_x, beacon_x_rem) = get_first_number(&sensor_y_rem);
    let (beacon_y, _) = get_first_number(&beacon_x_rem);
    Sensor {
        position: Point {
            x: sensor_x,
            y: sensor_y,
        },
        beacon: Point {
            x: beacon_x,
            y: beacon_y,
        },
    }
}

fn distance(p1: Point, p2: Point) -> isize {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn can_have_beacon(p: Point, sensors: &Vec<Sensor>) -> bool {
    for sensor in sensors {
        let d = distance(sensor.position, p);
        if d <= distance(sensor.position, sensor.beacon) {
            return false;
        }
    }
    true
}

fn find_beacon(sensors: &Vec<Sensor>) -> Point {
    'outer: for sensor in sensors {
        let beacon_distance = distance(sensor.position, sensor.beacon);
        let mut x = sensor.position.x - beacon_distance - 1;
        let mut y = sensor.position.y;
        let mut x_dir = 1;
        let mut y_dir = 1;
        'inner: loop {
            x += x_dir;
            y += y_dir;
            if x >= 0 && y >= 0 && x <= 4_000_000 && y <= 4_000_000 {
                let p = Point { x, y };
                if can_have_beacon(p, sensors) {
                    return p;
                }
            }
            if y_dir == 1 && y == sensor.position.y + beacon_distance + 1 {
                y_dir = -1;
                continue 'inner;
            }
            if x_dir == 1 && x == sensor.position.x + beacon_distance + 1 {
                x_dir = -1;
                continue 'inner;
            }
            if y_dir == -1 && y == sensor.position.y - beacon_distance - 1 {
                y_dir = 1;
                continue 'inner;
            }
            if x_dir == -1 && x == sensor.position.x - beacon_distance - 1 {
                continue 'outer;
            }
        }
    }
    panic!("No beacon found");
}

pub fn solve() -> String {
    let lines = get_data_as_lines("day_15_beacons.txt");
    let sensors: Vec<Sensor> = lines.iter().map(|l| get_sensor(l)).collect();
    let beacon = find_beacon(&sensors);
    (beacon.x * 4_000_000 + beacon.y).to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "10908230916597");
}
