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
    let answer = if args.day == 1 {
        aoc_2023::day_1::day_1(&data)
    } else if args.day == 2 {
        aoc_2023::day_2::day_2(&data)
    } else {
        ("".into(), "".into())
    };

    println!("Part 1: {}", answer.0);
    println!("Part 2: {}", answer.1);
}
