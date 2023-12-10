use crate::solution::Solution;
pub struct Day6;

impl Solution for Day6 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        // part 1
        let lines = raw_data.lines().collect::<Vec<_>>();
        let times = values(lines[0]);
        let distances = values(lines[1]);
        let sol_1 = times
            .iter()
            .copied()
            .zip(distances)
            .map(|(t, d)| num_win_times(t, d))
            .product::<u64>();

        // part 2
        let lines = raw_data.lines().collect::<Vec<_>>();
        let time = single_value(lines[0]);
        let distance = single_value(lines[1]);
        let sol_2 = num_win_times(time, distance);

        (Box::new(sol_1), Box::new(sol_2))
    }
}

// value calculations
fn num_win_times(race_time: u64, record_distance: u64) -> u64 {
    let (t_1, t_2) = barely_win_times(race_time, record_distance);
    t_2 - t_1 + 1
}

fn barely_win_times(race_time: u64, record_distance: u64) -> (u64, u64) {
    let a = -1.;
    let b = race_time as f64;
    let c = -((record_distance + 1) as f64);

    let (t_1, t_2) = pol_2_roots(c, b, a);

    let t_1 = t_1.ceil() as u64;
    let t_2 = std::cmp::min(t_2.floor() as u64, race_time);

    (t_1, t_2)
}

fn pol_2_roots(p_0: f64, p_1: f64, p_2: f64) -> (f64, f64) {
    let mut t_1 = (-p_1 - (p_1.powi(2) - 4. * p_2 * p_0).sqrt()) / 2. / p_2;
    let mut t_2 = (-p_1 + (p_1.powi(2) - 4. * p_2 * p_0).sqrt()) / 2. / p_2;
    if t_1 > t_2 {
        std::mem::swap(&mut t_1, &mut t_2);
    }
    (t_1, t_2)
}

// line parsing
fn values(line: &str) -> Vec<u64> {
    line.chars()
        .skip_while(|&c| c != ' ')
        .collect::<String>()
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}
fn single_value(line: &str) -> u64 {
    line.chars()
        .skip_while(|&c| c != ' ')
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse()
        .unwrap()
}
