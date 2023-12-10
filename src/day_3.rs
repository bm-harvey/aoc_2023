use crate::solution::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
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

        (Box::new(sol_1), Box::new(sol_2))
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
            for (col_idx, _) in row.iter().enumerate().filter(|(_, &c)| c == '*') {
                let tuples = Day3::border_tuples(matrix, col_idx, col_idx, row_idx);

                let mut part_numbers: Vec<PartNumber> = Vec::new();
                for t in tuples {
                    Day3::add_number(matrix, t.0, t.1, &mut part_numbers);
                }

                if part_numbers.len() == 2 {
                    let pn_1 = &part_numbers[0];
                    let num_1 = &matrix[pn_1.row_idx][pn_1.col_idx..=(pn_1.col_idx + pn_1.len)];
                    let num_1 = num_1.iter().collect::<String>().parse::<u32>().unwrap();

                    let pn_2 = &part_numbers[1];
                    let num_2 = &matrix[pn_2.row_idx][pn_2.col_idx..=(pn_2.col_idx + pn_2.len)];
                    let num_2 = num_2.iter().collect::<String>().parse::<u32>().unwrap();

                    total += num_1 * num_2;
                }
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

    fn add_number(
        matrix: &[Vec<char>],
        row_idx: usize,
        col_idx: usize,
        current_list: &mut Vec<PartNumber>,
    ) {
        let mut idx = col_idx;
        let row = &matrix[row_idx];

        let mut c = row[idx];
        if !c.is_numeric() {
            // not on a number so early return
            return;
        }

        // on a number
        while c.is_numeric() && idx != 0 {
            idx -= 1;
            c = row[idx];
        }

        let start = if c.is_numeric() { idx } else { idx + 1 };

        if current_list
            .iter()
            .any(|pn| pn.row_idx == row_idx && pn.col_idx == start)
        {
            return;
        }

        c = row[start]; 
        while c.is_numeric() && idx < row.len() - 1 {
            idx += 1;
            c = row[idx];
        }
        let stop = if c.is_numeric() { idx } else { idx - 1 };

        let pn = PartNumber {
            row_idx,
            col_idx: start,
            len: stop - start,
        };

        current_list.push(pn);
    }
}

struct PartNumber {
    row_idx: usize,
    col_idx: usize,
    len: usize,
}
