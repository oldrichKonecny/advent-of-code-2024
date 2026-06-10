use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use std::collections::VecDeque;

pub struct Day18;

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (x, y) = l.trim().split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

/// Grid is (0..=max) on both axes. Example uses 6, real puzzle uses 70.
/// First `prefix` bytes corrupt the grid before part one's path is computed.
fn config(bytes: &[(usize, usize)]) -> (usize, usize) {
    let max = bytes.iter().flat_map(|&(x, y)| [x, y]).max().unwrap();
    if max <= 6 {
        (6, 12)
    } else {
        (70, 1024)
    }
}

/// Shortest path length from (0,0) to (max,max) avoiding corrupted cells,
/// or None if the exit is unreachable.
fn shortest_path(corrupted: &[Vec<bool>], max: usize) -> Option<u64> {
    let size = max + 1;
    let mut visited = vec![vec![false; size]; size];
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 0u64));
    visited[0][0] = true;
    while let Some((x, y, steps)) = queue.pop_front() {
        if x == max && y == max {
            return Some(steps);
        }
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];
        for (nx, ny) in neighbors {
            if nx < size && ny < size && !corrupted[ny][nx] && !visited[ny][nx] {
                visited[ny][nx] = true;
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }
    None
}

fn build_grid(bytes: &[(usize, usize)], count: usize, max: usize) -> Vec<Vec<bool>> {
    let size = max + 1;
    let mut corrupted = vec![vec![false; size]; size];
    for &(x, y) in &bytes[..count] {
        corrupted[y][x] = true;
    }
    corrupted
}

impl DaySolver<u64> for Day18 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let bytes = parse(&input.input);
        let (max, prefix) = config(&bytes);
        let corrupted = build_grid(&bytes, prefix, max);
        shortest_path(&corrupted, max).ok_or_else(|| Error::msg("No path found"))
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let bytes = parse(&input.input);
        let (max, prefix) = config(&bytes);
        // Binary search: smallest count of fallen bytes that blocks the path.
        // Known reachable at `prefix`, so search in (prefix, bytes.len()].
        let mut lo = prefix;
        let mut hi = bytes.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let grid = build_grid(&bytes, mid, max);
            if shortest_path(&grid, max).is_some() {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        // The blocking byte is the last one included, i.e. index lo - 1.
        let (x, y) = bytes[lo - 1];
        println!("First blocking byte: {},{}", x, y);
        Ok(0)
    }
}
