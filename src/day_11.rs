use crate::solution::Solution;

pub struct Day11;
impl Solution for Day11 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let universe = Universe::new(raw_data);
        let sol_1 = universe.pair_wise_galactic_distances(2);
        let sol_2 = universe.pair_wise_galactic_distances(1_000_000);
        (Box::new(sol_1), Box::new(sol_2))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Space {
    Empty,
    Galaxy,
}
#[derive(Debug)]
struct Universe {
    data: Vec<Vec<Space>>,
    empty_cols: Vec<usize>,
    empty_rows: Vec<usize>,
}

type Coordinate = (usize, usize);

impl Universe {
    fn new(raw_data: &str) -> Self {
        let data: Vec<Vec<Space>> = raw_data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Space::Empty,
                        '#' => Space::Galaxy,
                        _ => panic!("{c}"),
                    })
                    .collect()
            })
            .collect();

        let empty_rows = (0..data.len())
            .filter(|&row_idx| {
                (0..data[0].len()).all(|col_idx| data[row_idx][col_idx] == Space::Empty)
            })
            .collect();
        let empty_cols = (0..data[0].len())
            .filter(|&col_idx| {
                (0..data.len()).all(|row_idx| data[row_idx][col_idx] == Space::Empty)
            })
            .collect();

        Self {
            data,
            empty_rows,
            empty_cols,
        }
    }

    fn num_rows(&self) -> usize {
        self.data.len()
    }
    fn num_cols(&self) -> usize {
        self.data[0].len()
    }

    fn iter(&self) -> impl Iterator<Item = Coordinate> {
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        (0..num_rows).flat_map(move |row_idx| (0..num_cols).map(move |col_idx| (row_idx, col_idx)))
    }

    fn galaxies_locations(&self) -> Vec<Coordinate> {
        self.iter()
            .filter(|(r, c)| self.data[*r][*c] == Space::Galaxy)
            .collect()
    }

    fn distance(&self, coord_1: &Coordinate, coord_2: &Coordinate, scaling: usize) -> usize {
        let unexpanded_distance = (coord_2.0 as isize - coord_1.0 as isize).abs()
            + (coord_2.1 as isize - coord_1.1 as isize).abs();
        let unexpanded_distance = unexpanded_distance as usize;

        let min_row = std::cmp::min(coord_1.0, coord_2.0);
        let max_row = std::cmp::max(coord_1.0, coord_2.0);
        let extra_rows = self
            .empty_rows
            .iter()
            .filter(|&&row| min_row < row && row < max_row)
            .count()
            * (scaling - 1);

        let min_col = std::cmp::min(coord_1.1, coord_2.1);
        let max_col = std::cmp::max(coord_1.1, coord_2.1);
        let extra_cols = self
            .empty_cols
            .iter()
            .filter(|&&col| min_col < col && col < max_col)
            .count()
            * (scaling - 1);

        unexpanded_distance + extra_rows + extra_cols
    }

    fn pair_wise_galactic_distances(&self, scaling: usize) -> usize {
        let galaxies = self.galaxies_locations();

        println!("num_galaxies{}", galaxies.len());

        let mut result = 0;
        for (g1_idx, g1) in galaxies.iter().enumerate() {
            for g2 in galaxies.iter().skip(g1_idx + 1) {
                result += self.distance(g1, g2, scaling);
            }
        }

        result
    }
}
