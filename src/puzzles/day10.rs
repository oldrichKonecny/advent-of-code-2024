use std::fmt::{Display, Formatter};
use anyhow::Error;
use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::{Direction, Matrix};

pub struct Day10;

impl DaySolver<u64> for Day10 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let starts = matrix.find_all((0, false));
        let trailhead_sum = starts.iter()
            .map(|(start_row, start_col)| compute_trailhead(matrix.clone(), *start_row, *start_col))
            .sum();
        Ok(trailhead_sum)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let starts = matrix.find_all((0, false));
        let trailhead_sum = starts.iter()
            .map(|(start_row, start_col)| compute_trailhead_v2(matrix.clone(), *start_row, *start_col))
            .sum();
        Ok(trailhead_sum)
    }

}

fn compute_trailhead(mut matrix: Matrix<(u8, bool)>, start_row: usize, start_col: usize) -> u64 {
    let mut trailhead = 0;
    let mut next_node = Vec::new();
    next_node.push((start_row, start_col, 0));
    while !next_node.is_empty() {
        let (row, col, current_step) = next_node.pop().unwrap();
        matrix.set(row, col, (current_step, true));

        if current_step == 9 {
            trailhead += 1;
            continue;
        }
        matrix.get_neighbors(row, col, &[Direction::Up, Direction::Down, Direction::Left, Direction::Right]).iter()
            .filter(|node_info| node_info.value.0 == current_step + 1 && !node_info.value.1)
            .for_each(|node_info| {
                next_node.push((node_info.row, node_info.col, node_info.value.0));
            });
    }
    trailhead
}

fn compute_trailhead_v2(mut matrix: Matrix<(u8, bool)>, start_row: usize, start_col: usize) -> u64 {
    let mut trailhead = 0;
    let mut next_node = Vec::new();
    next_node.push((start_row, start_col, 0));
    while !next_node.is_empty() {
        let (row, col, current_step) = next_node.pop().unwrap();
        if current_step == 9 {
            trailhead += 1;
            continue;
        }
        matrix.get_neighbors(row, col, &[Direction::Up, Direction::Down, Direction::Left, Direction::Right]).iter()
            .filter(|node_info| node_info.value.0 == current_step + 1)
            .for_each(|node_info| {
                next_node.push((node_info.row, node_info.col, node_info.value.0));
            });
    }
    trailhead
}

fn parse_input(input: &Input) -> Matrix<(u8, bool)> {
    let data = input.input.chars()
        .filter(|c| *c != '\n')
        .flat_map(|c| c.to_digit(10))
        .map(|d| (d as u8, false))
        .collect();
    let rows = input.input.lines().count();
    let cols = input.input.lines().next().unwrap().len();
    Matrix::new(rows, cols, data)
}

