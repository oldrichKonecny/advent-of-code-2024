use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::{Direction, Matrix, NodeInfo};
use anyhow::Error;

pub struct Day06;

impl DaySolver<u64> for Day06 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let (mut guard_row, mut guard_col) = matrix.find_all('^').get(0).unwrap().clone();
        let mut guard_direction = Direction::Up;
        loop {
            if let Some(node_info) = matrix.get_next_info(guard_row, guard_col, &guard_direction) {
                match node_info {
                    NodeInfo {
                        row,
                        col,
                        value: '#'
                    } => {
                        guard_direction = match guard_direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            _ => panic!("Invalid direction for guard {:?}", guard_direction)
                        }
                    }
                    NodeInfo {
                        row,
                        col,
                        value
                    } => {
                        matrix.set(guard_row, guard_col, 'X');
                        guard_row = row;
                        guard_col = col;
                    }
                }
            } else {
                matrix.set(guard_row, guard_col, 'X');
                break;
            }
        }
        Ok(matrix.count_all('X') as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let matrix = parse_input(input);
        let (starting_guard_row, starting_guard_col) = matrix.find_all('^').get(0).unwrap().clone();
        let starting_guard_direction = Direction::Up;

        let mut guard_stucked_count = 0;
        for obstacle_info in matrix.clone().into_iter() {
            if obstacle_info.value != '.' {
                continue;
            }
            let mut matrix = matrix.clone();
            matrix.set(obstacle_info.row, obstacle_info.col, '#');
            let mut guard_row = starting_guard_row;
            let mut guard_col = starting_guard_col;
            let mut guard_direction = starting_guard_direction.clone();

            loop {
                if let Some(node_info) = matrix.get_next_info(guard_row, guard_col, &guard_direction) {
                    match node_info {
                        NodeInfo { row: _, col: _, value: '#' } => {
                            guard_direction = match guard_direction {
                                Direction::Up => {
                                    if *matrix.get(guard_row, guard_col).unwrap() == '>' {
                                        guard_stucked_count += 1;
                                        break;
                                    }
                                    Direction::Right
                                },
                                Direction::Down => {
                                    if *matrix.get(guard_row, guard_col).unwrap() == '<' {
                                        guard_stucked_count += 1;
                                        break;
                                    }
                                    Direction::Left
                                },
                                Direction::Left => {
                                    if *matrix.get(guard_row, guard_col).unwrap() == '^' {
                                        guard_stucked_count += 1;
                                        break;
                                    }
                                    Direction::Up
                                },
                                Direction::Right => {
                                    if *matrix.get(guard_row, guard_col).unwrap() == 'v' {
                                        guard_stucked_count += 1;
                                        break;
                                    }
                                    Direction::Down
                                },
                                _ => panic!("Invalid direction for guard {:?}", guard_direction)
                            }
                        }
                        NodeInfo { row, col, value: val } => {
                            match (&guard_direction, val) {
                                (Direction::Up, '^') => {
                                    guard_stucked_count += 1;
                                    break;
                                },
                                (Direction::Down, 'v') => {
                                    guard_stucked_count += 1;
                                    break;
                                },
                                (Direction::Left, '<') => {
                                    guard_stucked_count += 1;
                                    break;
                                },
                                (Direction::Right, '>') => {
                                    guard_stucked_count += 1;
                                    break;
                                },
                                (Direction::Up, _) => matrix.set(guard_row, guard_col, '^'),
                                (Direction::Down, _) => matrix.set(guard_row, guard_col, 'v'),
                                (Direction::Left, _) => matrix.set(guard_row, guard_col, '<'),
                                (Direction::Right, _) => matrix.set(guard_row, guard_col, '>'),
                                _ => panic!("Invalid direction for guard {:?}", guard_direction)
                            };
                            guard_row = row;
                            guard_col = col;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        Ok(guard_stucked_count)
    }
}

fn parse_input(input: &Input) -> Matrix<char> {
    let data = input.input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let rows = input.input.lines().count();
    let cols = input.input.lines().next().unwrap().chars().count();
    Matrix::new(rows, cols, data)
}