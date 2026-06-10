use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use std::collections::HashMap;

pub struct Day24;

impl DaySolver<u64> for Day24 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let circuit = Circuit::parse(&input.input);
        Ok(circuit.z_output())
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let circuit = Circuit::parse(&input.input);
        let wires = circuit.swapped_wires();
        log::info!("Swapped wires: {}", wires);
        Ok(wires.split(',').count() as u64)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    op: Op,
}

struct Circuit<'a> {
    initial: HashMap<&'a str, u8>,
    gates: HashMap<&'a str, Gate<'a>>,
}

impl<'a> Circuit<'a> {
    fn parse(input: &'a str) -> Self {
        let mut initial = HashMap::new();
        let mut gates = HashMap::new();
        for line in input.lines().map(str::trim) {
            if let Some((name, value)) = line.split_once(": ") {
                initial.insert(name, value.parse::<u8>().unwrap());
            } else if let Some((expr, out)) = line.split_once(" -> ") {
                let mut parts = expr.split_whitespace();
                let left = parts.next().unwrap();
                let op = match parts.next().unwrap() {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    other => panic!("Unknown gate {}", other),
                };
                let right = parts.next().unwrap();
                gates.insert(out, Gate { left, right, op });
            }
        }
        Circuit { initial, gates }
    }

    fn z_output(&self) -> u64 {
        let mut values: HashMap<&str, u8> = self.initial.clone();
        let mut result = 0;
        for (&wire, _) in self.gates.iter().filter(|(w, _)| w.starts_with('z')) {
            let bit: u32 = wire[1..].parse().unwrap();
            result |= (self.eval(wire, &mut values) as u64) << bit;
        }
        result
    }

    /// The circuit should be a ripple-carry adder. Gates whose output wire was
    /// swapped violate one of its structural rules:
    /// - every z wire (except the topmost, the final carry) is driven by a XOR
    /// - a XOR not fed by x/y inputs must drive a z wire
    /// - an `x XOR y` gate (except bit 00) must feed another XOR
    /// - an `x AND y` gate (except bit 00) must feed only an OR
    fn swapped_wires(&self) -> String {
        let is_input = |w: &str| w.starts_with('x') || w.starts_with('y');
        let last_z = self
            .gates
            .keys()
            .filter(|w| w.starts_with('z'))
            .max()
            .unwrap();

        let mut consumers: HashMap<&str, Vec<Op>> = HashMap::new();
        for gate in self.gates.values() {
            consumers.entry(gate.left).or_default().push(gate.op);
            consumers.entry(gate.right).or_default().push(gate.op);
        }

        let mut bad: Vec<&str> = Vec::new();
        for (&out, gate) in &self.gates {
            let input_fed = is_input(gate.left) && is_input(gate.right);
            let bit_zero = gate.left.ends_with("00") && gate.right.ends_with("00");
            let feeds = consumers.get(out).map(Vec::as_slice).unwrap_or(&[]);
            let wrong = match gate.op {
                _ if out.starts_with('z') => out != *last_z && gate.op != Op::Xor,
                Op::Xor if !input_fed => true,
                Op::Xor if !bit_zero => !feeds.contains(&Op::Xor),
                Op::And if !bit_zero => feeds.iter().any(|&op| op != Op::Or),
                _ => false,
            };
            if wrong {
                bad.push(out);
            }
        }
        bad.sort_unstable();
        bad.join(",")
    }

    fn eval(&self, wire: &'a str, values: &mut HashMap<&'a str, u8>) -> u8 {
        if let Some(&v) = values.get(wire) {
            return v;
        }
        let gate = &self.gates[wire];
        let (l, r) = (self.eval(gate.left, values), self.eval(gate.right, values));
        let v = match gate.op {
            Op::And => l & r,
            Op::Or => l | r,
            Op::Xor => l ^ r,
        };
        values.insert(wire, v);
        v
    }
}
