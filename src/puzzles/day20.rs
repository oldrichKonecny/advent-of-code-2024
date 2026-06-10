use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::Matrix;
use anyhow::Error;

pub struct Day20;

impl DaySolver<u64> for Day20 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        Ok(count_cheats(&input.input, 2, 100))
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        Ok(count_cheats(&input.input, 20, 100))
    }
}

fn count_cheats(input: &str, max_cheat_len: i64, min_saving: i64) -> u64 {
    let grid = parse_grid(input);
    let path = trace_path(&grid);

    let mut dist = vec![-1i64; grid.rows_len * grid.cols_len];
    for (i, &(row, col)) in path.iter().enumerate() {
        dist[row * grid.cols_len + col] = i as i64;
    }

    let mut count = 0;
    for &(row, col) in &path {
        let from_dist = dist[row * grid.cols_len + col];
        for dr in -max_cheat_len..=max_cheat_len {
            let remaining = max_cheat_len - dr.abs();
            for dc in -remaining..=remaining {
                let cheat_len = dr.abs() + dc.abs();
                if cheat_len < 2 {
                    continue;
                }
                let (to_row, to_col) = (row as i64 + dr, col as i64 + dc);
                if to_row < 0 || to_row >= grid.rows_len as i64
                    || to_col < 0 || to_col >= grid.cols_len as i64 {
                    continue;
                }
                let to_dist = dist[to_row as usize * grid.cols_len + to_col as usize];
                if to_dist >= 0 && to_dist - from_dist - cheat_len >= min_saving {
                    count += 1;
                }
            }
        }
    }
    count
}

fn parse_grid(input: &str) -> Matrix<char> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let data = input.lines().flat_map(|line| line.chars()).collect();
    Matrix::new(rows, cols, data)
}

fn trace_path(grid: &Matrix<char>) -> Vec<(usize, usize)> {
    let start = grid.find_first('S').expect("no start position");
    let end = grid.find_first('E').expect("no end position");

    let mut path = vec![start];
    let mut prev = start;
    let mut current = start;
    while current != end {
        let (row, col) = current;
        let next = [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)].iter()
            .map(|&(dr, dc)| ((row as i64 + dr) as usize, (col as i64 + dc) as usize))
            .find(|&(r, c)| (r, c) != prev && grid.get(r, c).is_some_and(|&ch| ch != '#'))
            .expect("dead end on track");
        prev = current;
        current = next;
        path.push(current);
    }
    path
}
