use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use log::info;
use crate::utils::matrix::{Direction, Matrix};

pub struct Day15;

impl DaySolver<u64> for Day15 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let (mut matrix, directions) = parse_input(input);

        let mut robot_pos = matrix.find_first(Cell::Robot).unwrap();
        for direction in directions {
            let neighbour = matrix.get_next_info(robot_pos.0, robot_pos.1, &direction)
                .expect("Invalid, robot should not be at the edge of the matrix");
            match neighbour.value {
                Cell::Empty => {
                    matrix.set(robot_pos.0, robot_pos.1, Cell::Empty);
                    robot_pos = (neighbour.row, neighbour.col);
                    matrix.set(robot_pos.0, robot_pos.1, Cell::Robot);
                }
                Cell::Box => {
                    let mut next_neighbour = matrix.get_next_info(neighbour.row, neighbour.col, &direction)
                        .expect("Invalid, box should not be at the edge of the matrix");
                    while next_neighbour.value == Cell::Box {
                        next_neighbour = matrix.get_next_info(next_neighbour.row, next_neighbour.col, &direction)
                            .expect("Invalid, box should not be at the edge of the matrix");
                    }
                    match next_neighbour.value {
                        Cell::Empty => {
                            matrix.set(robot_pos.0, robot_pos.1, Cell::Empty);
                            robot_pos = (neighbour.row, neighbour.col);
                            matrix.set(robot_pos.0, robot_pos.1, Cell::Robot);
                            matrix.set(next_neighbour.row, next_neighbour.col, Cell::Box);
                        }
                        Cell::Wall => continue,
                        _ => panic!("can be only empty or wall"),
                    }
                }
                Cell::Robot => panic!("wtf? two robots?"),
                Cell::Wall => continue,
            }
        }
        Ok(compute_boxes_gps(&matrix))
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let (mut matrix, directions) = parse_input_wide(input);

        let mut robot_pos = matrix.find_first(WideCell::Robot).unwrap();
        for direction in directions {
            robot_pos = match direction {
                Direction::Left | Direction::Right => move_horizontal(&mut matrix, robot_pos, &direction),
                Direction::Up | Direction::Down => move_vertical(&mut matrix, robot_pos, &direction),
                _ => panic!("unexpected diagonal direction"),
            };
        }

        Ok(compute_wide_boxes_gps(&matrix))
    }

}

/// Pushes the robot horizontally, shifting any contiguous run of boxes. Returns the new robot
/// position (unchanged if blocked by a wall).
fn move_horizontal(matrix: &mut Matrix<WideCell>, robot_pos: (usize, usize), direction: &Direction) -> (usize, usize) {
    // Walk in the push direction until we find empty space (push succeeds) or a wall (blocked).
    let mut scan = robot_pos;
    let empty = loop {
        let next = matrix.get_next_info(scan.0, scan.1, direction)
            .expect("robot/box should never reach the matrix edge");
        match next.value {
            WideCell::Wall => return robot_pos,
            WideCell::Empty => break (next.row, next.col),
            WideCell::BoxLeft | WideCell::BoxRight => scan = (next.row, next.col),
            WideCell::Robot => panic!("wtf? two robots?"),
        }
    };

    // Shift the whole run one cell towards the empty slot by copying from the neighbour on the
    // robot side (the opposite of the push direction).
    let back = opposite(direction);
    let mut cur = empty;
    while cur != robot_pos {
        let prev = step(cur, &back);
        matrix.set(cur.0, cur.1, matrix.get_cloned(prev.0, prev.1).unwrap());
        cur = prev;
    }
    matrix.set(robot_pos.0, robot_pos.1, WideCell::Empty);
    step(robot_pos, direction)
}

/// Pushes the robot vertically. Vertically a single push can cascade into a tree of boxes; the move
/// only happens if every box in that tree has free space to move into. Returns the new robot
/// position (unchanged if blocked by a wall).
fn move_vertical(matrix: &mut Matrix<WideCell>, robot_pos: (usize, usize), direction: &Direction) -> (usize, usize) {
    // Collect the left-anchor of every box that must move, breadth-first from the robot outward.
    let mut boxes: Vec<(usize, usize)> = Vec::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(robot_pos);

    while let Some(pushing) = queue.pop_front() {
        let next = matrix.get_next_info(pushing.0, pushing.1, direction)
            .expect("robot/box should never reach the matrix edge");
        let anchor = match next.value {
            WideCell::Wall => return robot_pos,
            WideCell::Empty => continue,
            WideCell::BoxLeft => (next.row, next.col),
            WideCell::BoxRight => (next.row, next.col - 1),
            WideCell::Robot => panic!("wtf? two robots?"),
        };
        if !boxes.contains(&anchor) {
            boxes.push(anchor);
            // Both halves of the box now push into the cells ahead of them.
            queue.push_back(anchor);
            queue.push_back((anchor.0, anchor.1 + 1));
        }
    }

    // The whole tree can move. Clear every box, then redraw it shifted one row; doing it in two
    // passes avoids any ordering hazard between boxes overwriting each other.
    for &(r, c) in &boxes {
        matrix.set(r, c, WideCell::Empty);
        matrix.set(r, c + 1, WideCell::Empty);
    }
    for &(r, c) in &boxes {
        let (nr, nc) = step((r, c), direction);
        matrix.set(nr, nc, WideCell::BoxLeft);
        matrix.set(nr, nc + 1, WideCell::BoxRight);
    }

    matrix.set(robot_pos.0, robot_pos.1, WideCell::Empty);
    step(robot_pos, direction)
}

fn opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        _ => panic!("unexpected diagonal direction"),
    }
}

fn step((row, col): (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
        _ => panic!("unexpected diagonal direction"),
    }
}

fn compute_wide_boxes_gps(matrix: &Matrix<WideCell>) -> u64 {
    let boxes = matrix.find_all(WideCell::BoxLeft);
    info!("num boxes: {}", boxes.len());
    boxes.iter()
        .map(|(row, col)| (*row * 100 + *col) as u64)
        .sum()
}

fn parse_input_wide(input: &Input) -> (Matrix<WideCell>, Vec<Direction>) {
    let (map, directions) = input.input.split_once("\n\n").unwrap();

    let mut data = Vec::new();
    for line in map.lines() {
        for c in line.chars() {
            match c {
                '.' => data.extend([WideCell::Empty, WideCell::Empty]),
                '#' => data.extend([WideCell::Wall, WideCell::Wall]),
                'O' => data.extend([WideCell::BoxLeft, WideCell::BoxRight]),
                '@' => data.extend([WideCell::Robot, WideCell::Empty]),
                _ => panic!("Invalid character in input"),
            }
        }
    }
    let rows = map.lines().count();
    let cols = map.lines().next().unwrap().len() * 2;

    let directions = directions.chars().filter(|c| *c != '\n').map(|c| match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Invalid character in input '{}'", c),
    }).collect();

    (Matrix::new(rows, cols, data), directions)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideCell {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl Display for WideCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            WideCell::Empty => '.',
            WideCell::Wall => '#',
            WideCell::BoxLeft => '[',
            WideCell::BoxRight => ']',
            WideCell::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

fn compute_boxes_gps(matrix: &Matrix<Cell>) -> u64 {
    info!("num boxes: {}", matrix.find_all(Cell::Box).len());
    matrix.find_all(Cell::Box).iter()
        .map(|(row, col)| *row * 100 + *col)
        .map(|x| x as u64)
        .sum()
}

fn parse_input(input: &Input) -> (Matrix<Cell>, Vec<Direction>) {
    let (matrix, directions) = input.input.split_once("\n\n").unwrap();
    let data = matrix.chars().filter(|c| *c != '\n').map(|c| match c {
        '.' => Cell::Empty,
        '#' => Cell::Wall,
        'O' => Cell::Box,
        '@' => Cell::Robot,
        _ => panic!("Invalid character in input"),
    }).collect();
    let row = matrix.lines().count();
    let col = matrix.lines().next().unwrap().len();

    let directions = directions.chars().filter(|c| *c != '\n').map(|c| match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Invalid character in input '{}'", c),
    }).collect();

    (Matrix::new(row, col, data), directions)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Box => 'O',
            Cell::Robot => '@',
        };
        write!(f, "{}", c)
    }
}