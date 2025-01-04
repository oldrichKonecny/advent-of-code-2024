use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::{Direction, Matrix};
use anyhow::Error;

pub struct Day04;

impl DaySolver<u64> for Day04 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let matrix = parse_input(input);
        let all_x = matrix.find_all('X');
        let mut count = 0;
        all_x.iter().for_each(|(row, col)| {
            if let Some(up_section) = matrix.get_section(*row, *col, &Direction::Up, 4) {
                if check_xmas(&up_section) {
                    count += 1;
                }
            }
            if let Some(down_section) = matrix.get_section(*row, *col, &Direction::Down, 4) {
                if check_xmas(&down_section) {
                    count += 1;
                }
            }
            if let Some(left_section) = matrix.get_section(*row, *col, &Direction::Left, 4) {
                if check_xmas(&left_section) {
                    count += 1;
                }
            }
            if let Some(right_section) = matrix.get_section(*row, *col, &Direction::Right, 4) {
                if check_xmas(&right_section) {
                    count += 1;
                }
            }
            if let Some(up_right_section) = matrix.get_section(*row, *col, &Direction::UpRight, 4) {
                if check_xmas(&up_right_section) {
                    count += 1;
                }
            }
            if let Some(up_left_section) = matrix.get_section(*row, *col, &Direction::UpLeft, 4) {
                if check_xmas(&up_left_section) {
                    count += 1;
                }
            }
            if let Some(down_right_section) = matrix.get_section(*row, *col, &Direction::DownRight, 4) {
            if check_xmas(&down_right_section) {
                count += 1;
            }            }
            if let Some(down_left_section) = matrix.get_section(*row, *col, &Direction::DownLeft, 4) {
                if check_xmas(&down_left_section) {
                    count += 1;
                }
            }
        });
        Ok(count)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let matrix = parse_input(input);
        let all_a = matrix.find_all('A');
        let mut count = 0;
        all_a.iter().for_each(|(row, col)| {
            let maybe_up_left = matrix.get_next(*row, *col, &Direction::UpLeft);
            let maybe_up_right = matrix.get_next(*row, *col, &Direction::UpRight);
            let maybe_down_left = matrix.get_next(*row, *col, &Direction::DownLeft);
            let maybe_down_right = matrix.get_next(*row, *col, &Direction::DownRight);
            if maybe_up_left.is_some() && maybe_up_right.is_some() && maybe_down_left.is_some() && maybe_down_right.is_some() {
                let up_left = maybe_up_left.unwrap();
                let up_right = maybe_up_right.unwrap();
                let down_left = maybe_down_left.unwrap();
                let down_right = maybe_down_right.unwrap();
                if ((*up_left == 'M' && *down_right == 'S') || (*up_left == 'S' && *down_right == 'M')) &&
                    ((*up_right == 'M' && *down_left == 'S') || (*up_right == 'S' && *down_left == 'M')) {
                    count += 1;
                }
            }
        });


        Ok(count)
    }
}

fn parse_input(input: &Input) -> Matrix<char> {
    let data = input.input.chars().filter(|c| c.is_alphanumeric()).collect::<Vec<char>>();
    let rows = input.input.lines().filter(|line| !line.is_empty()).count();
    let cols = input.input.lines().next().unwrap().chars().count();
    Matrix::new(rows, cols, data)
}

fn check_xmas(section: &[char]) -> bool {
    section.eq(&['X', 'M', 'A', 'S'])
}