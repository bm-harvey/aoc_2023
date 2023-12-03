use crate::solution::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn solution(raw_data: &str) -> (u32, u32) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for l in raw_data.lines() {
            let mut row = Vec::new();
            for c in l.chars() {
                row.push(c);
            }
            matrix.push(row);
        }

        let sol_1 = Day3::solution_1(&matrix);
        let sol_2 = Day3::solution_2(&matrix);

        (sol_1, sol_2)
    }
}

impl Day3 {
    fn solution_1(matrix: &Vec<Vec<char>>) -> u32 {
        let mut total = 0;
        for (row_idx, row) in matrix.iter().enumerate() {
            let mut idx = 0;
            'col: loop {
                if idx >= row.len() {
                    break;
                }

                // find the start of a number
                while idx < row.len() && !row[idx].is_numeric() {
                    idx += 1;

                    if idx == row.len() {
                        break 'col;
                    }
                }
                let start = idx;

                // find the end of the number
                while idx < row.len() && row[idx].is_numeric() {
                    idx += 1;
                }
                let stop = idx - 1;

                let number = &row[start..=stop]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                let tuples_to_check = Day3::border_tuples(matrix, start, stop, row_idx);
                let include_num = tuples_to_check.iter().any(|t| {
                    let c = matrix[t.0][t.1];
                    !c.is_numeric() && c != '.'
                });

                if include_num {
                    total += number
                }
            }
        }
        total
    }
    fn solution_2(matrix: &Vec<Vec<char>>) -> u32 {
        let mut total = 0;

        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate().filter(|(_, &c)| c == '*') {
                let tuples = Day3::border_tuples(matrix, col_idx, col_idx, row_idx);

                let mut part_numbers: Vec<PartNumber> = Vec::new();
            }
        }

        total
    }

    fn border_tuples(
        matrix: &Vec<Vec<char>>,
        start: usize,
        stop: usize,
        row_idx: usize,
    ) -> Vec<(usize, usize)> {
        let mut tuples: Vec<(usize, usize)> = Vec::new();
        let row = &matrix[row_idx];

        let start_2 = if start == 0 {
            0
        } else {
            tuples.push((row_idx, start - 1));
            start - 1
        };
        let stop_2 = if stop == row.len() - 1 {
            row.len() - 1
        } else {
            tuples.push((row_idx, stop + 1));
            stop + 1
        };
        for idx_2 in start_2..=stop_2 {
            if row_idx > 0 {
                tuples.push((row_idx - 1, idx_2));
            }
            if row_idx < matrix.len() - 1 {
                tuples.push((row_idx + 1, idx_2))
            }
        }

        tuples
    }
}

struct PartNumber {
    row_idx: usize,
    col_idx: usize,
    len: usize,
}
