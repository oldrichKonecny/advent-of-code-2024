use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day19;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.trim().split(", ").collect();
    let designs = designs
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    (patterns, designs)
}

fn count_ways(design: &str, patterns: &[&str]) -> u64 {
    let design_bytes = design.as_bytes();
    let mut dp = vec![0u64; design_bytes.len() + 1];
    dp[0] = 1;
    for i in 1..=design_bytes.len() {
        for pattern in patterns {
            let pattern_bytes = pattern.as_bytes();
            if pattern_bytes.len() <= i
                && dp[i - pattern_bytes.len()] > 0
                && &design_bytes[i - pattern_bytes.len()..i] == pattern_bytes
            {
                dp[i] += dp[i - pattern_bytes.len()];
            }
        }
    }
    dp[design_bytes.len()]
}

impl DaySolver<u64> for Day19 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let (patterns, designs) = parse(&input.input);
        Ok(designs
            .iter()
            .filter(|d| count_ways(d, &patterns) > 0)
            .count() as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let (patterns, designs) = parse(&input.input);
        Ok(designs.iter().map(|d| count_ways(d, &patterns)).sum())
    }
}
