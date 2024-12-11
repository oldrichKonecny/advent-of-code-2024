use std::cmp::Ordering;
use std::fmt::Display;
use anyhow::Error;
use log::debug;
use crate::base::generic_solver::{DaySolver, Input};

pub struct Day02;

impl DaySolver<u64> for Day02 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let res = input.input.lines()
            .map(|line| line.split(" ")
                .filter(|split| !split.is_empty())
                .map(|split| split.parse::<i32>().unwrap())
                .collect::<Vec<_>>())
            .filter(|numbers| check_conditions(&numbers))
            .count();
        Ok(res as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let res = input.input.lines()
            .map(|line| line.split(" ")
                .filter(|split| !split.is_empty())
                .map(|split| split.parse::<i32>().unwrap())
                .collect::<Vec<_>>())
            .filter(|numbers| check_conditions2(&numbers))
            .count();
        Ok(res as u64)
    }
}

fn check_conditions(numbers: &[i32]) -> bool {
    let convergence = numbers[0].cmp(&numbers[1]);
    if convergence == Ordering::Equal {
        return false;
    }

    numbers.windows(2)
        .all(|pair| {
            let a = pair[0];
            let b = pair[1];
            a.cmp(&b) == convergence && a.abs_diff(b) > 0 && a.abs_diff(b) <= 3
    })
}

fn check_conditions2(numbers: &[i32]) -> bool {
    if check_conditions(numbers) {
        return true;
    }
    for i in 0..numbers.len() {
        let mut nums = numbers.to_vec();
        nums.remove(i);
        if check_conditions(&nums) {
            return true;
        }
    }
    false
}