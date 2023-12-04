use crate::solution::Solution;

pub struct Day4;
impl Solution for Day4 {
    fn solution(raw_data: &str) -> (u32, u32) {
        let mut num_lines = 0;
        let sol_1: u32 = raw_data
            .lines()
            .map(|line| {
                let start = line.find(':').unwrap();
                let sets = line[(start + 1)..].split('|').collect::<Vec<_>>();

                let winning_numbers = sets[0]
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let gotten_numbers = sets[1]
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                num_lines += 1;
                Day4::score(&winning_numbers, &gotten_numbers)
            })
            .sum();

        let mut weights = vec![1; num_lines];

        let mut idx = 0;
        raw_data.lines().for_each(|line| {
            let start = line.find(':').unwrap();
            let sets = line[(start + 1)..].split('|').collect::<Vec<_>>();

            let winning_numbers = sets[0]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let gotten_numbers = sets[1]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            Day4::weighted_score(&winning_numbers, &gotten_numbers, idx, &mut weights);
            idx += 1;
        });

        for w in weights.iter() {
            dbg!(w);
        }
        let sol_2 = weights.iter().sum::<u32>();
        (sol_1, sol_2)
    }
}

impl Day4 {
    fn score(winning_numbers: &[u32], gotten_numbers: &[u32]) -> u32 {
        let num_matches: u32 = gotten_numbers
            .iter()
            .filter(|&x| winning_numbers.iter().any(|w| w == x))
            .count() as u32;

        if num_matches > 0 {
            2_u32.pow(num_matches - 1)
        } else {
            0_u32
        }
    }
    fn weighted_score(
        winning_numbers: &[u32],
        gotten_numbers: &[u32],
        idx: usize,
        weights: &mut [u32],
    ) {
        let num_matches = gotten_numbers
            .iter()
            .filter(|&x| winning_numbers.iter().any(|w| w == x))
            .count();

        let increment = weights[idx];
        for w in &mut weights[(idx + 1)..=(idx + num_matches)] {
            *w += increment;
        }
    }
}
