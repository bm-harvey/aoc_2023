use std::cmp::max;

pub fn day_2(raw_data: &str) -> (String, String) {
    let mut match_idx = 0;
    let mut total = 0;
    let mut power = 0;
    raw_data.lines().for_each(|line| {
        let mut game = Game::default();
        match_idx += 1;

        let start = line.find(':').unwrap();
        let matches: Vec<_> = line[(start + 1)..].split(';').collect();

        for m in matches {
            for color in m.split(',') {
                let mut iter = color.split_whitespace();
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
            total += match_idx;
        }

        power += game.power()
    });

    (format!("{total}"), format!("{power}"))
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
