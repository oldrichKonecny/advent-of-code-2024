use anyhow::Error;
use log::debug;
use crate::base::generic_solver::{DaySolver, Input};

pub struct Day07;

impl DaySolver<u64> for Day07 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let res = input.input.lines()
            .map(|line| Equation::parse(line))
            .filter(|equation| equation.is_solvable(vec![|a, b| a + b, |a, b| a * b]))
            .map(|equation| equation.target)
            .sum::<u64>();
        Ok(res)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let res = input.input.lines()
            .map(|line| Equation::parse(line))
            .filter(|equation| equation.is_solvable(vec![
                |a, b| a + b,
                |a, b| a * b,
                |a, b: u64| a * 10u64.pow(b.ilog10() as u32 + 1) + b,
            ]))
            .map(|equation| equation.target)
            .sum::<u64>();
        Ok(res)
    }

}

#[derive(Debug)]
struct Equation {
    target: u64,
    factors: Vec<u64>,
}

impl Equation {
    fn parse(line: &str) -> Self {
        let (target, factors) = line.split_once(": ").unwrap();
        Equation {
            target: target.parse().unwrap(),
            factors: factors.trim().split(" ").map(|factor| factor.trim().parse().unwrap()).collect(),
        }
    }

    fn is_solvable<F>(&self, operations: Vec<F>) -> bool
        where F: Fn(u64, u64) -> u64 {
        if self.factors.is_empty() {
            false
        } else if self.factors.len() == 1 {
            self.target == self.factors[0]
        } else {
            solve_rec(self.target, &operations, self.factors[0], &self.factors[1..])
        }
    }
}

fn solve_rec<F>(target: u64, operations: &Vec<F>, tmp_res: u64, factors: &[u64]) -> bool
    where F: Fn(u64, u64) -> u64 {
    if factors.is_empty() {
        return tmp_res == target;
    }
    for op in operations.iter() {
        if solve_rec(target, &operations, op(tmp_res, factors[0]), &factors[1..]) {
            return true;
        }
    }
    false
}