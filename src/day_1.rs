use crate::solution::Solution;

use std::cmp::min;

pub struct Day1;
impl Solution for Day1 {
    fn solution(raw_data: &str) -> (u32, u32) {
        let mut sol_1 = 0;
        let mut sol_2 = 0;
        for line in raw_data.lines() {
            sol_1 += Day1::process_line_1(line);
            sol_2 += Day1::process_line_2(line);
        }
        (sol_1, sol_2)
    }
}

impl Day1 {
    fn process_line_1(line: &str) -> u32 {
        let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
        let digit_1 = numbers.first().unwrap().to_digit(10).unwrap();
        let digit_2 = numbers.last().unwrap().to_digit(10).unwrap();
        10 * digit_1 + digit_2
    }

    fn process_line_2(line: &str) -> u32 {
        let mut idx_left = 0;
        let mut calibration_numbers = Vec::<u32>::new();
        'inf: loop {
            if idx_left >= line.len() {
                break;
            }

            let letter = line.chars().nth(idx_left).unwrap();
            if letter.is_numeric() {
                calibration_numbers.push(letter.to_digit(10).unwrap());
                idx_left += 1;
                continue;
            }

            for idx_right in (idx_left + 2)..min(line.len(), idx_left + 5) {
                let word = &line[idx_left..=idx_right];
                let number = Day1::get_number(word);

                // 53hvhgchljnlxqjsgrhxgf1zfoureightmlhvvv
                if let Some(num) = number {
                    calibration_numbers.push(num);
                    idx_left += 1;
                    continue 'inf;
                }
            }
            idx_left += 1;
        }
        10 * calibration_numbers.first().unwrap() + calibration_numbers.last().unwrap()
    }

    fn get_number(word: &str) -> Option<u32> {
        if word == "one" {
            Some(1)
        } else if word == "two" {
            Some(2)
        } else if word == "three" {
            Some(3)
        } else if word == "four" {
            Some(4)
        } else if word == "five" {
            Some(5)
        } else if word == "six" {
            Some(6)
        } else if word == "seven" {
            Some(7)
        } else if word == "eight" {
            Some(8)
        } else if word == "nine" {
            Some(9)
        } else {
            None
        }
    }
}
