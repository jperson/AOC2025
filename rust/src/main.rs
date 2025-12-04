use clap::Parser;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

#[derive(Parser)]
#[command(name = "AOC25")]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    if args.day == 1 {
        //part 1
        let input = include_str!("../inputs/day1.txt");
        println!("{}\n{}", day1::day1_1(input), day1::day1_2(input));
    }

    if args.day == 2 {
        let input = include_str!("../inputs/day2.txt");
        let (r1, r2) = day2::day2(input);
        println!("{}\n{}", r1, r2);

        // let input = "1-4294967296";
        // let (r1, r2) = day2::day2(input);
        // println!("{}\n{}", r1, r2);

        // let input: &str = "1-18446744073709551615";
        // let (r1, r2) = day2::day2(input);
        // println!("{}\n{}", r1, r2);
    }

    if args.day == 3 {
        let input = include_str!("../inputs/day3.txt");

        let start = Instant::now();
        let r1 = day3::day3(input, 2);
        let duration = start.elapsed();
        println!("{}", r1);
        println!("Time elapsed: {:?}", duration);

        let start = Instant::now();
        let r2 = day3::day3(input, 12);
        let duration = start.elapsed();
        println!("{}", r2);
        println!("Time elapsed: {:?}", duration);
    }

    if args.day == 4 {
        let input = include_str!("../inputs/day4.txt");

        let start = Instant::now();
        let r1 = day4::part1(input);
        let duration = start.elapsed();
        println!("{}", r1);
        println!("Time elapsed: {:?}", duration);

        let start = Instant::now();
        let r2 = day4::part2(input);
        let duration = start.elapsed();
        println!("{}", r2);
        println!("Time elapsed: {:?}", duration);
    }
}
