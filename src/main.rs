mod solutions;
pub mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
    challenge: u8,
}

fn main() {
    let args = Args::parse();

    println!(
        "{}",
        match args.day {
            1 => match args.challenge {
                1 => solutions::day_1_1::solve(),
                2 => solutions::day_1_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            2 => match args.challenge {
                1 => solutions::day_2_1::solve(),
                2 => solutions::day_2_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            3 => match args.challenge {
                1 => solutions::day_3_1::solve(),
                2 => solutions::day_3_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            4 => match args.challenge {
                1 => solutions::day_4_1::solve(),
                2 => solutions::day_4_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            5 => match args.challenge {
                1 => solutions::day_5_1::solve(),
                2 => solutions::day_5_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            6 => match args.challenge {
                1 => solutions::day_6_1::solve(),
                2 => solutions::day_6_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            7 => match args.challenge {
                1 => solutions::day_7_1::solve(),
                2 => solutions::day_6_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            _ => panic!("Invalid day"),
        }
    );
}
