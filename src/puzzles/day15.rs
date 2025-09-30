use std::collections::HashMap;
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
        let binding = input.input.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        let word_count = binding.split_whitespace()
            .fold(HashMap::new(), |mut acc, word| {
                let entry = acc.entry(word).or_insert(0);
                *entry += 1;
                acc
            });

        info!("word count: {:?}", word_count);
        Ok(0)
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