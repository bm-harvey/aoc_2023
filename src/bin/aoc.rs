use aoc_2023::solution::Solution;
use clap::Parser;

/// Advent of Code 2023 solution script
#[derive(Parser, Debug)]
struct Args {
    /// Which day of advent of code to run
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let file_path = if !args.test {
        format!("data/day_{}.dat", args.day)
    } else {
        format!("test_data/day_{}.dat", args.day)
    };
    let data = std::fs::read_to_string(file_path).expect("Problem reading file to string.");
    if args.day == 1 {
        aoc_2023::day_1::Day1::print(&data)
    } else if args.day == 2 {
        aoc_2023::day_2::Day2::print(&data)
    } else if args.day == 3 {
        aoc_2023::day_3::Day3::print(&data)
    }
}
