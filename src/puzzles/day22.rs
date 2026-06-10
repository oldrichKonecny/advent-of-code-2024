use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day22;

impl DaySolver<u64> for Day22 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        Ok(parse(&input.input)
            .map(|secret| (0..2000).fold(secret, |s, _| next_secret(s)))
            .sum())
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        Ok(most_bananas(&input.input))
    }
}

const SEQ_COUNT: usize = 19 * 19 * 19 * 19;

fn most_bananas(input: &str) -> u64 {
    let mut totals = vec![0u64; SEQ_COUNT];
    let mut seen_by = vec![u32::MAX; SEQ_COUNT];

    for (buyer, secret) in parse(input).enumerate() {
        let mut secret = secret;
        let mut prev_price = secret % 10;
        let mut seq = 0usize;
        for i in 0..2000 {
            secret = next_secret(secret);
            let price = secret % 10;
            let change = (9 + price - prev_price) as usize;
            prev_price = price;
            seq = (seq * 19 + change) % SEQ_COUNT;
            if i >= 3 && seen_by[seq] != buyer as u32 {
                seen_by[seq] = buyer as u32;
                totals[seq] += price;
            }
        }
    }

    totals.into_iter().max().unwrap_or(0)
}

fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.lines().filter_map(|line| line.trim().parse().ok())
}

fn next_secret(mut secret: u64) -> u64 {
    secret = (secret ^ (secret << 6)) & 0xFF_FFFF;
    secret = (secret ^ (secret >> 5)) & 0xFF_FFFF;
    (secret ^ (secret << 11)) & 0xFF_FFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_secret_matches_example() {
        assert_eq!(next_secret(123), 15887950);
        assert_eq!(next_secret(15887950), 16495136);
    }

    #[test]
    fn most_bananas_matches_example() {
        assert_eq!(most_bananas("1\n2\n3\n2024"), 23);
    }
}
