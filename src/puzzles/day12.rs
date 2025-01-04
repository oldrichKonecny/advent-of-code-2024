use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::{Direction, Matrix, NodeInfo};
use anyhow::Error;

pub struct Day12;

impl DaySolver<u64> for Day12 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let all_nodes = matrix.get_all_points();
        let mut sum = 0;
        for node in all_nodes.iter() {
            if matrix.get_cloned(node.row, node.col).unwrap().1 {
                continue;
            }
            sum += compute_group(&mut matrix, node);
        }
        Ok(sum)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut matrix = parse_input(input);
        let all_nodes = matrix.get_all_points();
        let mut sum = 0;
        for node in all_nodes.iter() {
            if matrix.get_cloned(node.row, node.col).unwrap().1 {
                continue;
            }
            sum += compute_group_v2(&mut matrix, node);
        }
        Ok(sum)
    }
}

fn compute_group(matrix: &mut Matrix<(char, bool)>, node: &NodeInfo<(char, bool)>) -> u64 {
    let look_directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    matrix.set(node.row, node.col, (node.value.0, true));
    let mut to_visit = vec![node.clone()];
    let mut area = 0;
    let mut perimeter = 0;

    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        area += 1;

        let neighbors = matrix.get_maybe_neighbors(current.row, current.col, &look_directions);
        for neighbor in neighbors {
            if neighbor.is_none() {
                perimeter += 1;
                continue;
            }
            let neighbor = neighbor.unwrap();
            if neighbor.value.0 != current.value.0 {
                perimeter += 1;
                continue;
            }
            if neighbor.value.1 {
                continue;
            }
            matrix.set(neighbor.row, neighbor.col, (neighbor.value.0, true));
            to_visit.push(neighbor);
        }
    }
    area * perimeter
}

fn compute_group_v2(matrix: &mut Matrix<(char, bool)>, node: &NodeInfo<(char, bool)>) -> u64 {
    matrix.set(node.row, node.col, (node.value.0, true));
    let mut to_visit = vec![node.clone()];
    let mut area = 0;
    let mut sides = 0;

    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        area += 1;


        let up_neighbor = matrix.get_next_info(current.row, current.col, &Direction::Up);
        let down_neighbor = matrix.get_next_info(current.row, current.col, &Direction::Down);
        let left_neighbor = matrix.get_next_info(current.row, current.col, &Direction::Left);
        let right_neighbor = matrix.get_next_info(current.row, current.col, &Direction::Right);
        let up_left_neighbor = matrix.get_next_info(current.row, current.col, &Direction::UpLeft);
        let up_right_neighbor = matrix.get_next_info(current.row, current.col, &Direction::UpRight);
        let down_left_neighbor = matrix.get_next_info(current.row, current.col, &Direction::DownLeft);
        let down_right_neighbor = matrix.get_next_info(current.row, current.col, &Direction::DownRight);

        for neighbor in &[&up_neighbor, &down_neighbor, &left_neighbor, &right_neighbor] {
            if neighbor.is_some() && neighbor.as_ref().unwrap().value.0 == current.value.0 && !neighbor.as_ref().unwrap().value.1 {
                matrix.set(neighbor.as_ref().unwrap().row, neighbor.as_ref().unwrap().col, (neighbor.as_ref().unwrap().value.0, true));
                to_visit.push(neighbor.as_ref().unwrap().clone());
            }
        }

        let no_up = up_neighbor.is_none() || up_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_down = down_neighbor.is_none() || down_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_left = left_neighbor.is_none() || left_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_right = right_neighbor.is_none() || right_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_up_left = up_left_neighbor.is_none() || up_left_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_up_right = up_right_neighbor.is_none() || up_right_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_down_left = down_left_neighbor.is_none() || down_left_neighbor.as_ref().unwrap().value.0 != current.value.0;
        let no_down_right = down_right_neighbor.is_none() || down_right_neighbor.as_ref().unwrap().value.0 != current.value.0;
        if (no_up && no_left) || (!no_up && !no_left && no_up_left)  {
            sides += 1;
        }
        if (no_up && no_right) || (!no_up && !no_right && no_up_right)  {
            sides += 1;
        }
        if (no_down && no_left) || (!no_down && !no_left && no_down_left)  {
            sides += 1;
        }
        if (no_down && no_right) || (!no_down && !no_right && no_down_right)  {
            sides += 1;
        }

    }
    area * sides
}



fn parse_input(input: &Input) -> Matrix<(char, bool)> {
    let data = input.input.chars()
        .filter(|c| *c != '\n')
        .map(|c| (c, false))
        .collect::<Vec<_>>();
    let row = input.input.lines().count();
    let col = input.input.lines().next().unwrap().chars().count();
    Matrix::new(row, col, data)
}