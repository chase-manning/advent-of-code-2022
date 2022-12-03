use crate::utils::files::read_file_to_lines;

pub fn solve() -> i32 {
    let lines = read_file_to_lines("./data", "day_2_strategy.txt");

    let mut total_score = 0;
    for line in lines {
        let mut iter = line.split_whitespace();
        let them = iter.next().unwrap();
        let me = iter.next().unwrap();
        total_score += match me {
            "X" => {
                0 + match { them } {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => panic!("Invalid input"),
                }
            }
            "Y" => {
                3 + match them {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => panic!("Invalid input"),
                }
            }
            "Z" => {
                6 + match them {
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => panic!("Invalid input"),
                }
            }
            _ => panic!("Invalid me"),
        };
    }
    total_score
}

#[test]
fn result() {
    assert_eq!(solve(), 12429);
}
