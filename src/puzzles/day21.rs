use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use std::collections::HashMap;

pub struct Day21;

impl DaySolver<u64> for Day21 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        Ok(solve(&input.input, 2))
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        Ok(solve(&input.input, 25))
    }
}

type Memo = HashMap<(char, char, usize, bool), u64>;

fn solve(input: &str, dir_robots: usize) -> u64 {
    let mut memo = Memo::new();
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|code| {
            let length: u64 = typing_cost(code, dir_robots, true, &mut memo);
            let numeric_part: u64 = code.trim_end_matches('A').parse().unwrap();
            length * numeric_part
        })
        .sum()
}

fn typing_cost(sequence: &str, depth: usize, numeric: bool, memo: &mut Memo) -> u64 {
    let mut prev = 'A';
    let mut total = 0;
    for c in sequence.chars() {
        total += move_cost(prev, c, depth, numeric, memo);
        prev = c;
    }
    total
}

// Minimal human key presses to move this keypad's arm from `from` to `to` and press it,
// with `depth` directional keypads between this keypad and the human.
fn move_cost(from: char, to: char, depth: usize, numeric: bool, memo: &mut Memo) -> u64 {
    if let Some(&cached) = memo.get(&(from, to, depth, numeric)) {
        return cached;
    }

    let (fr, fc) = key_pos(from, numeric);
    let (tr, tc) = key_pos(to, numeric);
    let gap = if numeric { (3, 0) } else { (0, 0) };

    let vertical: String = if tr > fr {
        "v".repeat(tr - fr)
    } else {
        "^".repeat(fr - tr)
    };
    let horizontal: String = if tc > fc {
        ">".repeat(tc - fc)
    } else {
        "<".repeat(fc - tc)
    };

    // Only two path shapes can be optimal: all-horizontal-then-vertical or the
    // reverse. Either is invalid if its corner lands on the gap.
    let mut candidates = Vec::with_capacity(2);
    if (fr, tc) != gap {
        candidates.push(format!("{horizontal}{vertical}A"));
    }
    if (tr, fc) != gap {
        candidates.push(format!("{vertical}{horizontal}A"));
    }

    let cost = candidates
        .iter()
        .map(|path| {
            if depth == 0 {
                path.len() as u64
            } else {
                typing_cost(path, depth - 1, false, memo)
            }
        })
        .min()
        .unwrap();

    memo.insert((from, to, depth, numeric), cost);
    cost
}

fn key_pos(key: char, numeric: bool) -> (usize, usize) {
    if numeric {
        match key {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => panic!("invalid numeric key: {key}"),
        }
    } else {
        match key {
            '^' => (0, 1),
            'A' => (0, 2),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            _ => panic!("invalid directional key: {key}"),
        }
    }
}
