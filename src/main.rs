#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

pub mod solutions;
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
                2 => solutions::day_7_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            8 => match args.challenge {
                1 => solutions::day_8_1::solve(),
                2 => solutions::day_8_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            9 => match args.challenge {
                1 => solutions::day_9_1::solve(),
                2 => solutions::day_9_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            10 => match args.challenge {
                1 => solutions::day_10_1::solve(),
                2 => solutions::day_10_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            11 => match args.challenge {
                1 => solutions::day_11_1::solve(),
                2 => solutions::day_11_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            12 => match args.challenge {
                1 => solutions::day_12_1::solve(),
                2 => solutions::day_12_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            13 => match args.challenge {
                1 => solutions::day_13_1::solve(),
                2 => solutions::day_13_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            14 => match args.challenge {
                1 => solutions::day_14_1::solve(),
                2 => solutions::day_14_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            15 => match args.challenge {
                1 => solutions::day_15_1::solve(),
                2 => solutions::day_15_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            16 => match args.challenge {
                1 => solutions::day_16_1::solve(),
                2 => solutions::day_16_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            17 => match args.challenge {
                1 => solutions::day_17_1::solve(),
                2 => solutions::day_17_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            18 => match args.challenge {
                1 => solutions::day_18_1::solve(),
                2 => solutions::day_18_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            19 => match args.challenge {
                1 => solutions::day_19_1::solve(),
                2 => solutions::day_19_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            20 => match args.challenge {
                1 => solutions::day_20_1::solve(),
                2 => solutions::day_20_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            21 => match args.challenge {
                1 => solutions::day_21_1::solve(),
                2 => solutions::day_21_2::solve(),
                _ => panic!("Invalid challenge"),
            },
            _ => panic!("Invalid day"),
        }
    );
}
