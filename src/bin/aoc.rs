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

    let start = std::time::Instant::now();
    if args.day == 1 {
        aoc_2023::day_1::Day1::print(&data)
    } else if args.day == 2 {
        aoc_2023::day_2::Day2::print(&data)
    } else if args.day == 3 {
        aoc_2023::day_3::Day3::print(&data)
    } else if args.day == 4 {
        aoc_2023::day_4::Day4::print(&data)
    } else if args.day == 5 {
        aoc_2023::day_5::Day5::print(&data)
    } else if args.day == 6 {
        aoc_2023::day_6::Day6::print(&data)
    } else if args.day == 7 {
        aoc_2023::day_7::Day7::print(&data)
    } else if args.day == 8 {
        aoc_2023::day_8::Day8::print(&data)
    }


    println!("Solution took {}", start.elapsed().as_secs_f64());
}
