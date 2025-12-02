use clap::Parser;

pub mod day1;

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
        let input = include_str!("../inputs/day1_1.txt");
        let result = day1::day1_1(&input);
        println!("day1 part 1: {result:?}");

        let input = include_str!("../inputs/day1_1.txt");
        let result = day1::day1_2(&input);
        println!("day1 part 2: {result:?}");
    }
}
