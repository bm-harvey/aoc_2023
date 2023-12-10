use crate::solution::Solution;

pub struct Day9;
impl Solution for Day9 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let sol_1 = raw_data
            .lines()
            .map(extrapolate_hist_line_right)
            .sum::<isize>();

        let sol_2 = raw_data
            .lines()
            .map(extrapolate_hist_line_left)
            .sum::<isize>();

        (Box::new(sol_1), Box::new(sol_2))
    }
}

fn extrapolate_hist_line_right(line: &str) -> isize {
    let starting_line = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut difference_lines = vec![starting_line];

    while !last_line_zeroes(&difference_lines) {
        let last_line = difference_lines.last().unwrap();

        let new_line = (1..last_line.len())
            .map(|idx| last_line[idx] - last_line[idx - 1])
            .collect::<Vec<_>>();
        difference_lines.push(new_line);
    }

    difference_lines.last_mut().unwrap().push(0);
    for line_idx in (0..difference_lines.len() - 1).rev() {
        let extrapolated_value = difference_lines[line_idx].last().unwrap()
            + difference_lines[line_idx + 1].last().unwrap();

        difference_lines[line_idx].push(extrapolated_value);
    }

    *difference_lines.first().unwrap().last().unwrap()
}

fn extrapolate_hist_line_left(line: &str) -> isize {
    let starting_line = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .rev()
        .collect::<Vec<_>>();

    let mut difference_lines = vec![starting_line];

    while !last_line_zeroes(&difference_lines) {
        let last_line = difference_lines.last().unwrap();

        let new_line = (1..last_line.len())
            .map(|idx| last_line[idx] - last_line[idx - 1])
            .collect::<Vec<_>>();
        difference_lines.push(new_line);
    }

    difference_lines.last_mut().unwrap().push(0);
    for line_idx in (0..difference_lines.len() - 1).rev() {
        let extrapolated_value = difference_lines[line_idx].last().unwrap()
            + difference_lines[line_idx + 1].last().unwrap();

        difference_lines[line_idx].push(extrapolated_value);
    }

    *difference_lines.first().unwrap().last().unwrap()
}

fn last_line_zeroes(diff_lines: &[Vec<isize>]) -> bool {
    let last_line = diff_lines.last();
    match last_line {
        Some(line) => line.iter().all(|&num| num == 0),
        None => false,
    }
}
