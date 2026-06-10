use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day25;

impl DaySolver<u64> for Day25 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let (locks, keys) = parse(&input.input);
        let fits = locks
            .iter()
            .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
            .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5))
            .count();
        Ok(fits as u64)
    }

    fn solve_second(&self, _input: &Input) -> Result<u64, Error> {
        // Day 25 has no second puzzle; the final star is granted for the other 49.
        Ok(0)
    }
}

/// Column heights (0-5) per schematic, split into locks (top row solid)
/// and keys (bottom row solid). The solid row itself is not counted.
fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block in input.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }
        let mut heights = [0u8; 5];
        for line in block.lines().map(str::trim) {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    heights[col] += 1;
                }
            }
        }
        heights.iter_mut().for_each(|h| *h -= 1);
        if block.starts_with('#') {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }
    (locks, keys)
}
