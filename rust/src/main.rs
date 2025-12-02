use clap::Parser;

pub mod day1;
pub mod day2;

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

        let result = day1::day1_1(&input);
        println!("part 1: {result:?}");

        let result = day1::day1_2(&input);
        println!("part 2: {result:?}");
    }

    if args.day == 2 {
        let input = include_str!("../inputs/day2.txt");

        let result = day2::part1(&input);
        println!("part 1: {result:?}");

        let result = day2::part2(&input);
        println!("part 2: {result:?}");
    }
}
