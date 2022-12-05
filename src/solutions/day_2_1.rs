use crate::utils::files::get_data_as_lines;

pub fn solve() -> String {
    let lines = get_data_as_lines("day_2_strategy.txt");

    let mut total_score = 0;
    for line in lines {
        let mut iter = line.split_whitespace();
        let them = iter.next().unwrap();
        let me = iter.next().unwrap();
        total_score += match me {
            "X" => {
                1 + match { them } {
                    "A" => 3,
                    "B" => 0,
                    "C" => 6,
                    _ => panic!("Invalid input"),
                }
            }
            "Y" => {
                2 + match them {
                    "A" => 6,
                    "B" => 3,
                    "C" => 0,
                    _ => panic!("Invalid input"),
                }
            }
            "Z" => {
                3 + match them {
                    "A" => 0,
                    "B" => 6,
                    "C" => 3,
                    _ => panic!("Invalid input"),
                }
            }
            _ => panic!("Invalid me"),
        };
    }
    total_score.to_string()
}

#[test]
fn result() {
    assert_eq!(solve(), "9759");
}
