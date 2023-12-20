use crate::solution::Solution;
use std::rc::Rc;

pub struct Day10;
impl Solution for Day10 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let mut grid = Grid::new(raw_data);

        let sol_1 = grid.pipe_loop().len() / 2;
        let _sol_2 = 0; // ClusterNode::cluster(&grid);
        cluster(&mut grid);
        (Box::new(sol_1), Box::new(0))
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
    Start,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    grid: Vec<Vec<Pipe>>,
    topo_info: Vec<Vec<PipeSide>>,
}

impl Grid {
    fn new(raw_data: &str) -> Self {
        let mut inner_grid = raw_data
            .lines()
            .map(|line| {
                let mut pipe_line = vec![Pipe::None];
                let mut pipes = line
                    .chars()
                    .map(|c| match c {
                        '|' => Pipe::Vertical,
                        '-' => Pipe::Horizontal,
                        'F' => Pipe::UpperLeft,
                        '7' => Pipe::UpperRight,
                        'L' => Pipe::LowerLeft,
                        'J' => Pipe::LowerRight,
                        'S' => Pipe::Start,
                        '.' => Pipe::None,
                        _ => panic!("oops"),
                    })
                    .collect::<Vec<_>>();

                pipe_line.append(&mut pipes);
                pipe_line.push(Pipe::None);
                pipe_line
            })
            .collect::<Vec<_>>();

        let row_borders = vec![Pipe::None; inner_grid.first().unwrap().len()];
        let topos = vec![PipeSide::Unknown; row_borders.len()];
        let mut grid = vec![row_borders.clone()];
        grid.append(&mut inner_grid);
        grid.push(row_borders);
        let topo_info = vec![topos; grid.len()];

        Self { grid, topo_info }
    }

    fn at(&self, idx_x: usize, idx_y: usize) -> Pipe {
        self.grid[idx_y][idx_x]
    }
    fn topo_at(&self, idx_x: usize, idx_y: usize) -> PipeSide {
        self.topo_info[idx_y][idx_x]
    }
    fn set_topo(&mut self, idx_x: usize, idx_y: usize, side: PipeSide) {
        self.topo_info[idx_y][idx_x] = side
    }

    fn print_topo(&self) {
        self.topo_info.iter().for_each(|line| {
            line.iter().for_each(|ps| match ps {
                PipeSide::Right => print!("R"),
                PipeSide::Left => print!("L"),
                PipeSide::On => print!("*"),
                PipeSide::Unknown => print!(" "),
            });
            println!();
        });
    }
    fn print_pipe(&self) {
        self.grid.iter().for_each(|line| {
            line.iter().for_each(|p| {
                let symbol = match p {
                    Pipe::None => ' ',
                    Pipe::Start => 'S',
                    Pipe::Vertical => '|',
                    Pipe::Horizontal => '-',
                    Pipe::UpperLeft => 'F',
                    Pipe::UpperRight => '7',
                    Pipe::LowerLeft => 'L',
                    Pipe::LowerRight => 'J',
                };
                print!("{}", symbol);
            });
            println!();
        });
    }

    fn assign_topo(&mut self, idx_x: usize, idx_y: usize, side: PipeSide) {
        //dbg!(idx_x);
        //dbg!(idx_y);
        //if idx_y >= self.topo_info.len() || idx_x >= self.topo_info[0].len() {
        //return;
        //}
        if self.topo_at(idx_x, idx_y) == PipeSide::Unknown {
            self.topo_info[idx_y][idx_x] = side
        }
    }
    fn x_dim(&self) -> usize {
        self.grid.len()
    }
    fn y_dim(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let x_dim = self.x_dim();
        let y_dim = self.y_dim();

        (0..x_dim).flat_map(move |x_idx| (0..y_dim).map(move |y_idx| (x_idx, y_idx)))
    }

    fn start(&self) -> (usize, usize) {
        self.iter()
            .find(|pair| self.at(pair.0, pair.1) == Pipe::Start)
            .unwrap()
    }

    fn pipe_loop(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        use Direction::*;
        let start = self.start();
        let mut spot = start;
        let mut direction = Up;

        for dir in [Up, Down, Left, Right] {
            let s = match dir {
                Up => (spot.0, spot.1 - 1),
                Down => (spot.0, spot.1 + 1),
                Left => (spot.0 - 1, spot.1),
                Right => (spot.0 + 1, spot.1),
            };
            if valid_initial_connection(dir, self.at(s.0, s.1)) {
                spot = s;
                direction = dir;
                result.push(s);
                break;
            }
        }

        while spot != start {
            direction = next_direction(direction, self.at(spot.0, spot.1));
            spot = match direction {
                Up => (spot.0, spot.1 - 1),
                Down => (spot.0, spot.1 + 1),
                Left => (spot.0 - 1, spot.1),
                Right => (spot.0 + 1, spot.1),
            };

            result.push(spot);
        }
        result
    }
}

fn next_direction(previous_direction: Direction, pipe: Pipe) -> Direction {
    use Direction::*;
    use Pipe::*;
    match (previous_direction, pipe) {
        (Up, UpperRight) => Left,
        (Up, UpperLeft) => Right,
        (Up, Vertical) => Up,
        (Down, LowerLeft) => Right,
        (Down, LowerRight) => Left,
        (Down, Vertical) => Down,
        (Right, UpperRight) => Down,
        (Right, LowerRight) => Up,
        (Right, Horizontal) => Right,
        (Left, UpperLeft) => Down,
        (Left, LowerLeft) => Up,
        (Left, Horizontal) => Left,
        (_, _) => panic!(),
    }
}

fn valid_initial_connection(direction: Direction, pipe_to: Pipe) -> bool {
    use Direction::*;
    use Pipe::*;

    matches!(
        (direction, pipe_to),
        (Up, Vertical | UpperLeft | UpperRight)
            | (Down, Vertical | LowerLeft | LowerRight)
            | (Right, Horizontal | UpperRight | LowerRight)
            | (Left, Horizontal | UpperLeft | LowerLeft)
    )
}

#[derive(Clone, Copy, PartialEq)]
enum PipeSide {
    Left,
    Right,
    On,
    Unknown,
}

struct ClusterNode {
    _parent: Option<Rc<ClusterNode>>,
    _x: usize,
    _y: usize,
}

impl ClusterNode {}

fn cluster(grid: &mut Grid) -> (Vec<(usize, usize)>, Vec<Rc<ClusterNode>>) {
    use Direction::*;
    use Pipe::*;
    for _ in 0..grid.grid.len() {
        let cols = grid.grid.first().unwrap().len();
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(PipeSide::Unknown);
        }
    }

    let pipes = grid.pipe_loop();

    for idx in 0..pipes.len() {
        let direction = direction(&pipes[idx], &pipes[(idx + 1) % pipes.len()]);

        let location = pipes[idx];
        grid.set_topo(location.0, location.1, PipeSide::On);

        match (direction, grid.at(location.0, location.1)) {
            (Up, Vertical) => {
                grid.assign_topo(location.0 - 1, location.1, PipeSide::Left);
                grid.assign_topo(location.0 + 1, location.1, PipeSide::Right);
            }
            (Down, Vertical) => {
                grid.assign_topo(location.0 + 1, location.1, PipeSide::Left);
                grid.assign_topo(location.0 - 1, location.1, PipeSide::Right);
            }
            (Left, Horizontal) => {
                grid.assign_topo(location.0, location.1 + 1, PipeSide::Left);
                grid.assign_topo(location.0, location.1 - 1, PipeSide::Right);
            }
            (Right, Horizontal) => {
                grid.assign_topo(location.0, location.1 - 1, PipeSide::Left);
                grid.assign_topo(location.0, location.1 + 1, PipeSide::Right);
            }
            (Left, UpperRight) => {
                grid.assign_topo(location.0 + 1, location.1, PipeSide::Left);
                grid.assign_topo(location.0, location.1 - 1, PipeSide::Left);
            }
            (Down, UpperRight) => {
                grid.assign_topo(location.0 + 1, location.1, PipeSide::Right);
                grid.assign_topo(location.0, location.1 - 1, PipeSide::Right);
            }
            (Right, UpperLeft) => {
                grid.assign_topo(location.0 + 1, location.1, PipeSide::Right);
                grid.assign_topo(location.0, location.1 - 1, PipeSide::Right);
            }
            _ => {}
        }
    }
    grid.print_topo();
    grid.print_pipe();

    (vec![], vec![])
}

fn direction(spot_current: &(usize, usize), spot_next: &(usize, usize)) -> Direction {
    let dx = spot_next.0 as isize - spot_current.0 as isize;
    let dy = spot_next.1 as isize - spot_current.1 as isize;

    match (dx, dy) {
        (1, 0) => Direction::Left,
        (-1, 0) => Direction::Right,
        (0, 1) => Direction::Up,
        (0, -1) => Direction::Down,
        _ => panic!(),
    }
}
