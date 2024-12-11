use std::collections::HashMap;
use std::fmt::Display;
use anyhow::Error;
use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::Matrix;

pub struct Day08;

impl DaySolver<u64> for Day08 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let map_of_chars = matrix.clone().into_iter()
            .fold(HashMap::new(), |mut acc, info| {
                if info.value.value != '.' {
                    acc.entry(info.value.value).or_insert_with(Vec::new).push(info);
                }
                acc
            });

        for value in map_of_chars.values() {
            for i in 0..value.len() {
                for j in i+1..value.len() {
                    let a_node = &value[i];
                    let b_node = &value[j];
                    determine_antinode(&mut matrix, (a_node.row as isize, a_node.col as isize), (b_node.row as isize, b_node.col as isize));
                }
            }
        }
        Ok(matrix.into_iter()
            .filter(|info| info.value.contains_antinode)
            .count() as u64
        )
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let map_of_chars = matrix.clone().into_iter()
            .fold(HashMap::new(), |mut acc, info| {
                if info.value.value != '.' {
                    acc.entry(info.value.value).or_insert_with(Vec::new).push(info);
                }
                acc
            });

        for value in map_of_chars.values() {
            for i in 0..value.len() {
                for j in i+1..value.len() {
                    let a_node = &value[i];
                    let b_node = &value[j];
                    determine_all_antinodes(&mut matrix, (a_node.row as isize, a_node.col as isize), (b_node.row as isize, b_node.col as isize));
                }
            }
        }
        Ok(matrix.into_iter()
            .filter(|info| info.value.contains_antinode)
            .count() as u64
        )
    }
}

fn determine_antinode(matrix: &mut Matrix<Node>, node_a: (isize, isize), node_b: (isize, isize)) {
    let (expected_a_row, expected_a_col) = (node_a.0 + (node_a.0 - node_b.0), node_a.1 + (node_a.1 - node_b.1));
    let (expected_b_row, expected_b_col) = (node_b.0 + (node_b.0 - node_a.0), node_b.1 + (node_b.1 - node_a.1));
    if expected_a_row >= 0 && expected_a_col >= 0 {
        if let Some(antinode_a) = matrix.get(expected_a_row as usize, expected_a_col as usize ) {
            matrix.set(expected_a_row as usize, expected_a_col as usize, Node {value: antinode_a.value, contains_antinode: true});
        }
    }

    if expected_b_row >= 0 && expected_b_col >= 0 {
        if let Some(antinode_b) = matrix.get(expected_b_row as usize, expected_b_col as usize) {
            matrix.set(expected_b_row as usize, expected_b_col as usize, Node {value: antinode_b.value, contains_antinode: true});
        }
    }
}

fn determine_all_antinodes(matrix: &mut Matrix<Node>, node_a: (isize, isize), node_b: (isize, isize)) {
    let (sub_a_row, sub_a_col) = (node_a.0 - node_b.0, node_a.1 - node_b.1);
    let (sub_b_row, sub_b_col) = (node_b.0 - node_a.0, node_b.1 - node_a.1);

    let mut counter = 0;
    loop {
        let expected_a_row = node_a.0 + counter * sub_a_row;
        let expected_a_col = node_a.1 + counter * sub_a_col;
        if expected_a_row >= 0 && expected_a_col >= 0 && expected_a_row < matrix.rows_len as isize && expected_a_col < matrix.cols_len as isize {
            if let Some(antinode_a) = matrix.get(expected_a_row as usize, expected_a_col as usize ) {
                matrix.set(expected_a_row as usize, expected_a_col as usize, Node {value: antinode_a.value, contains_antinode: true});
            }
            counter += 1;
        } else {
            break;
        }
    }

    counter = 0;
    loop {
        let expected_b_row = node_b.0 + counter * sub_b_row;
        let expected_b_col = node_b.1 + counter * sub_b_col;
        if expected_b_row >= 0 && expected_b_col >= 0 && expected_b_row < matrix.rows_len as isize && expected_b_col < matrix.cols_len as isize {
            if let Some(antinode_b) = matrix.get(expected_b_row as usize, expected_b_col as usize) {
                matrix.set(expected_b_row as usize, expected_b_col as usize, Node {value: antinode_b.value, contains_antinode: true});
            }
            counter += 1;
        } else {
            break;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    value: char,
    contains_antinode: bool,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.contains_antinode { '#' } else { self.value })
    }
}

fn parse_input(input: &Input) -> Matrix<Node> {
    let data = input.input.chars().filter(|c| *c != '\n').map(|c| Node {value: c, contains_antinode: false}).collect::<>();
    let rows = input.input.lines().count();
    let cols = input.input.lines().next().unwrap().chars().count();
    Matrix::new(rows, cols, data)
}