use crate::base::generic_solver::{DaySolver, Input};
use anyhow::{anyhow, Error};
use log::info;

pub struct Day17;

struct Machine {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
}

fn parse(input: &str) -> Result<Machine, Error> {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut program = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Register A:") {
            a = rest.trim().parse()?;
        } else if let Some(rest) = line.strip_prefix("Register B:") {
            b = rest.trim().parse()?;
        } else if let Some(rest) = line.strip_prefix("Register C:") {
            c = rest.trim().parse()?;
        } else if let Some(rest) = line.strip_prefix("Program:") {
            program = rest
                .trim()
                .split(',')
                .map(|n| n.trim().parse::<u8>())
                .collect::<Result<Vec<_>, _>>()?;
        }
    }

    if program.is_empty() {
        return Err(anyhow!("No program found in input"));
    }

    Ok(Machine { a, b, c, program })
}

/// Run the program with the given starting value of register A and return the output stream.
fn run(program: &[u8], mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut ip = 0usize;

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        // Resolve combo operand value.
        let combo = || -> u64 {
            match operand {
                0..=3 => operand as u64,
                4 => a,
                5 => b,
                6 => c,
                _ => unreachable!("invalid combo operand 7"),
            }
        };

        match opcode {
            0 => a >>= combo(),       // adv
            1 => b ^= operand as u64, // bxl
            2 => b = combo() % 8,     // bst
            3 => {
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            } // jnz
            4 => b ^= c,              // bxc (operand ignored)
            5 => out.push((combo() % 8) as u8), // out
            6 => b = a >> combo(),    // bdv
            7 => c = a >> combo(),    // cdv
            _ => unreachable!("invalid opcode"),
        }
        ip += 2;
    }

    out
}

/// Reconstruct the smallest A that makes the program output a copy of itself.
///
/// `matched` is how many trailing program values are already reproduced by `a`.
/// Each recursion appends one base-8 digit (the next-higher 3 bits of A) and keeps
/// candidates whose run reproduces the required suffix `program[len - matched - 1..]`.
fn search(program: &[u8], b0: u64, c0: u64, a: u64, matched: usize) -> Option<u64> {
    if matched == program.len() {
        return Some(a);
    }
    let want = &program[program.len() - matched - 1..];
    for d in 0..8u64 {
        let na = (a << 3) | d;
        if run(program, na, b0, c0) == want {
            if let Some(res) = search(program, b0, c0, na, matched + 1) {
                return Some(res);
            }
        }
    }
    None
}

impl DaySolver<u64> for Day17 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let machine = parse(&input.input)?;
        let out = run(&machine.program, machine.a, machine.b, machine.c);

        let joined = out
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        info!("Day 17 part 1 output: {}", joined);

        // The framework expects a u64. All outputs are single digits (0-7),
        // so concatenating them yields a faithful numeric representation.
        let concatenated = out
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse::<u64>()?;
        Ok(concatenated)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let machine = parse(&input.input)?;
        search(&machine.program, machine.b, machine.c, 0, 0)
            .ok_or_else(|| anyhow!("No value of A reproduces the program"))
    }
}
