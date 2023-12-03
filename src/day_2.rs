use crate::solution::Solution;
use std::cmp::max;

pub struct Day2;
impl Solution for Day2 {
    fn solution(raw_data: &str) -> (u32, u32) {
        let mut match_idx = 0;
        let mut sol_1 = 0;
        let mut sol_2 = 0;
        for line in raw_data.lines() {
            let mut game = Game::default();
            match_idx += 1;
            let start = line.find(':').unwrap();
            let matches: Vec<_> = line[(start + 1)..].split(';').collect();
            for m in matches {
                for color_info in m.split(',') {
                    let mut iter = color_info.split_whitespace();
                    let number = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();
                    match color {
                        "red" => game.red = max(game.red, number),
                        "green" => game.green = max(game.green, number),
                        "blue" => game.blue = max(game.blue, number),
                        _ => {}
                    }
                }
            }
            if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
                sol_1 += match_idx;
            }
            sol_2 += game.power()
        }

        (sol_1, sol_2)
    }
}

#[derive(Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}
impl Game {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
