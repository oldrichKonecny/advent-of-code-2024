use anyhow::Error;
use crate::base::generic_solver::{DaySolver, Input};

pub struct Day13;

impl DaySolver<u64> for Day13 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let res = input.input.split("\n\n")
            .map(|section| Equation::parse(section, 0))
            .flat_map(|equation| equation.solve())
            .map(|(a, b)| 3 * a + b)
            .sum::<i128>();

        Ok(res as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let res = input.input.split("\n\n")
            .map(|section| Equation::parse(section, 10000000000000))
            .flat_map(|equation| equation.solve())
            .map(|(a, b)| 3 * a + b)
            .sum::<i128>();
        Ok(res as u64)
    }

}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Equation {
    a: (i128, i128),
    b: (i128, i128),
    prize: (i128, i128),
}

impl Equation {
    fn new(a: (i128, i128), b: (i128, i128), prize: (i128, i128)) -> Self {
        Self { a, b, prize }
    }

    fn parse(input: &str, prize_constant: i128) -> Self {
        let mut lines = input.lines();
        let a = lines.next().unwrap().split_once(", ").unwrap();
        let a = (parse_number_after(a.0, "+"), parse_number_after(a.1, "+"));
        let b = lines.next().unwrap().split_once(", ").unwrap();
        let b = (parse_number_after(b.0, "+"), parse_number_after(b.1, "+"));
        let prize = lines.next().unwrap().split_once(", ").unwrap();
        let prize = (parse_number_after(prize.0, "="), parse_number_after(prize.1, "="));
        Self::new(a, b, (prize.0 + prize_constant, prize.1 + prize_constant))
    }

    fn solve(&self) -> Option<(i128, i128)> {
        let b_part_1 = self.a.0 * self.prize.1 - self.a.1 * self.prize.0;
        let b_part_2 = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        if b_part_1 % b_part_2 != 0 {
            return None;
        }
        let b =  b_part_1 / b_part_2;
        let a_part = self.prize.0 - self.b.0 * b;
        if a_part % self.a.0 != 0 {
            return None;
        }
        let a = a_part / self.a.0;
        Some((a, b))
    }
}

fn parse_number_after(input: &str, after: &str) -> i128 {
    input.trim().split(after).nth(1).unwrap().parse().unwrap()
}

