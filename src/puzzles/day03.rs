use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day03;

impl DaySolver<u64> for Day03 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let sum = regex.captures_iter(&input.input)
            .map(|cap| {
                let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                a * b
            })
            .sum::<i32>();
        Ok(sum as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let regex = regex::Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)")?;
        let mut count_mul = true;
        let sum = regex.captures_iter(&input.input)
            .map(|cap| {
                if cap.get(1).is_some() {
                    count_mul = true;
                    0
                } else if cap.get(2).is_some() {
                    count_mul = false;
                    0
                } else {
                    if count_mul {
                        let a = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
                        let b = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
                        a * b
                    } else {
                        0
                    }
                }
            })
            .sum::<i32>();
        Ok(sum as u64)
    }
}