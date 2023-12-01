use clap::Parser;

/// Advent of Code 2023 solution script
#[derive(Parser, Debug)]
struct Args {
    /// Which day of advent of code to run
    #[arg(short, long)]
    day: usize,
}

fn main() {
    let args = Args::parse();

    let file_path = format!("data/day_{}.dat", args.day);
    let data = std::fs::read_to_string(file_path).expect("Problem reading file to string.");
    if args.day == 1 {
        aoc_2023::day_1::day_1(&data);
    }
}
