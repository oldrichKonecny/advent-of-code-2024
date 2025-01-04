use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use std::collections::HashMap;

pub struct Day01;

impl DaySolver<u64> for Day01 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let (mut first_vec, mut second_vec) : (Vec<_>, Vec<_>) = input.input.lines()
            .flat_map(|line| line.split_once("   "))
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unzip();

        first_vec.sort();
        second_vec.sort();

        Ok(first_vec.iter().zip(second_vec.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u32>() as u64)
    }
    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let (first_vec, second_vec) : (Vec<_>, Vec<_>) = input.input.lines()
            .flat_map(|line| line.split_once("   "))
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unzip();

        let occurrences_map = second_vec.into_iter()
            .fold(HashMap::new(), |mut acc, value| {
                acc.entry(value).and_modify(|e| *e += 1).or_insert(1u32);
                acc
            });

        Ok(first_vec.iter()
            .fold(0u64, |acc: u64, val| {
                if let Some(occurrence) = occurrences_map.get(val) {
                    acc + (val * occurrence) as u64
                } else {
                    acc
                }
            }))
    }
}

