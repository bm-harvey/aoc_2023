use crate::solution::Solution;

pub struct Day4;
impl Solution for Day4 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let lines = raw_data.lines();

        let sol_1: u32 = raw_data
            .lines()
            .map(|line| {
                let (winning_nums, gotten_nums) = Day4::unpack_line(line);
                Day4::score(&winning_nums, &gotten_nums)
            })
            .sum();

        // second pass - solve part 2
        let mut weights = vec![1; lines.count()];
        let mut idx = 0;
        raw_data.lines().for_each(|line| {
            let (winning_nums, gotten_nums) = Day4::unpack_line(line);
            Day4::adjust_scores(&winning_nums, &gotten_nums, idx, &mut weights);
            idx += 1;
        });

        let sol_2 = weights.iter().sum::<u32>();
        (Box::new(sol_1), Box::new(sol_2))
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
    fn adjust_scores(
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
        for w in &mut weights[idx + 1..=idx + num_matches] {
            *w += increment;
        }
    }

    fn unpack_line(line: &str) -> (Vec<u32>, Vec<u32>) {
        let start = line.find(':').unwrap();
        let sets = line[(start + 1)..].split('|').collect::<Vec<_>>();
        let winning_nums = sets[0]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let gotten_nums = sets[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        (winning_nums, gotten_nums)
    }
}
