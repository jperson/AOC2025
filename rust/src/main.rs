use clap::Parser;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Parser)]
#[command(name = "AOC25")]
struct Args {
    #[arg(short, long)]
    day: u8,
}

#[macro_export]
macro_rules! aoc {
    ($args:expr, $day:ident) => {
        if $args.day
            == stringify!($day)
                .strip_prefix("day")
                .unwrap()
                .parse::<u8>()
                .unwrap()
        {
            let input = include_str!(concat!("../inputs/", stringify!($day), ".txt"));

            let start = Instant::now();
            let r1 = $day::part1(input);
            let duration = start.elapsed();
            println!("{}", r1);
            println!("Time elapsed: {:?}", duration);

            let start = Instant::now();
            let r2 = $day::part2(input);
            let duration = start.elapsed();
            println!("{}", r2);
            println!("Time elapsed: {:?}", duration);
        }
    };
}

fn main() {
    let args = Args::parse();

    aoc!(args, day1);
    aoc!(args, day2);
    aoc!(args, day3);
    aoc!(args, day4);
    aoc!(args, day5);
    aoc!(args, day6);
    aoc!(args, day7);
    aoc!(args, day8);
    aoc!(args, day9);
}
