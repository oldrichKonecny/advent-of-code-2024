use std::collections::HashMap;
use anyhow::Error;
use crate::base::generic_solver::{DaySolver, Input};

pub struct Day11;

impl DaySolver<u64> for Day11 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut stones = input.input.split(" ")
            .filter(|s| !s.is_empty() && *s != "\n")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut new_stones;
        for _ in 1..=25 {
            new_stones = Vec::new();
            for stone in stones.iter() {
                if *stone == 0 {
                    new_stones.push(1);
                } else if (stone.ilog10() + 1) & 1 == 0 {
                    let (first, second) = divide_in_middle(*stone);
                    new_stones.push(first);
                    new_stones.push(second);
                } else {
                    new_stones.push(*stone * 2024);
                }
            }
            stones = new_stones;
        }
        Ok(stones.len() as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut memoization = input.input.split(" ")
            .filter(|s| !s.is_empty() && *s != "\n")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(stone).or_insert(0) += 1;
                acc
            });

        for _ in 0..75 {
            let mut new_memoization = HashMap::new();
            for (stone, count) in memoization.iter() {
                if *stone == 0 {
                    *new_memoization.entry(1).or_insert(0) += *count;
                } else if (stone.ilog10() + 1) & 1 == 0 {
                    let (first, second) = divide_in_middle(*stone);
                    *new_memoization.entry(first).or_insert(0) += *count;
                    *new_memoization.entry(second).or_insert(0) += *count;
                } else {
                    *new_memoization.entry(*stone * 2024).or_insert(0) += *count;
                }
            }
            memoization = new_memoization;
        }

        let num_of_stones = memoization.values().sum();
        Ok(num_of_stones)
    }
}

fn divide_in_middle(number: u64) -> (u64, u64) {
    let digits = number.ilog10() + 1;
    let half = digits / 2;
    let first = number / 10u64.pow(half);
    let second = number % 10u64.pow(half);
    (first, second)
}

fn compute_stones_with_memoization(stone: u64, steps: usize, memoization: &mut HashMap<u64, u64>) {

}