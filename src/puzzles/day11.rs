use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub struct Day11;

impl DaySolver<u64> for Day11 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut stones = input.input.split(" ")
            .filter(|s| !s.is_empty() && *s != "\n")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut new_stones = Vec::with_capacity(stones.len() * 2);
        for _ in 1..=25 {
            new_stones.clear();
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
            std::mem::swap(&mut stones, &mut new_stones);
        }
        Ok(stones.len() as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut memoization = input.input.split(" ")
            .filter(|s| !s.is_empty() && *s != "\n")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .fold(FxHashMap::default(), |mut acc, stone| {
                *acc.entry(stone).or_insert(0) += 1;
                acc
            });

        for _ in 0..75 {
            let mut new_memoization = FxHashMap::with_capacity_and_hasher(
                memoization.len() * 2,
                Default::default()
            );
            for (stone, count) in memoization.iter() {
                if *stone == 0 {
                    match new_memoization.entry(1) {
                        Entry::Occupied(mut e) => *e.get_mut() += *count,
                        Entry::Vacant(e) => { e.insert(*count); }
                    }
                } else if (stone.ilog10() + 1) & 1 == 0 {
                    let (first, second) = divide_in_middle(*stone);
                    match new_memoization.entry(first) {
                        Entry::Occupied(mut e) => *e.get_mut() += *count,
                        Entry::Vacant(e) => { e.insert(*count); }
                    }
                    match new_memoization.entry(second) {
                        Entry::Occupied(mut e) => *e.get_mut() += *count,
                        Entry::Vacant(e) => { e.insert(*count); }
                    }
                } else {
                    match new_memoization.entry(*stone * 2024) {
                        Entry::Occupied(mut e) => *e.get_mut() += *count,
                        Entry::Vacant(e) => { e.insert(*count); }
                    }
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