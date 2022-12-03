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
            _ => panic!("Invalid day"),
        }
    );
}
