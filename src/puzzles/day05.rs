use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;
use std::collections::HashMap;

pub struct Day05;

impl DaySolver<u64> for Day05 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let (rules_map, updates) = parse_input(input);
        let res = updates.iter()
            .filter(|update| check_rule(&rules_map, update))
            .map(|update| {
               let mid = update.len() / 2;
                update[mid]
            })
            .sum::<u32>();
        Ok(res as u64)
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let (rules_map, updates) = parse_input(input);
        let res = updates.iter()
            .filter(|update| !check_rule(&rules_map, update))
            .map(|update| fix_order(&rules_map, update))
            .map(|update| {
                let mid = update.len() / 2;
                update[mid]
            })
            .sum::<u32>();
        Ok(res as u64)
    }
}

fn check_rule(rules_map: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    for (i, n) in update.iter().enumerate().skip(1) {
        let to_test = &update[..i];
        if let Some(rules) = rules_map.get(n) {
            if to_test.iter().any(|test| rules.contains(test)) {
                return false;
            }
        }
    }
    true
}

fn fix_order(rules_map: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    let mut res = Vec::from(update);
    if res.len() <= 1 {
        return res;
    }

    let mut is_not_correct_order = true;
    while is_not_correct_order {
        is_not_correct_order = false;
        for i in 1.. res.len() {
            let n = res[i];
            let to_test = &res[..i];
            if let Some(rules) = rules_map.get(&n) {
                for (j, test) in to_test.iter().enumerate() {
                    if rules.contains(test) {
                        res.swap(j, i);
                        is_not_correct_order = true;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn parse_input(input: &Input) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (rules, updates) = input.input.split_once("\n\n").unwrap();
    let rules_map = rules.lines()
        .flat_map(|line| line.split_once("|"))
        .map(|(key, val)| (key.trim().parse::<u32>().unwrap(), val.trim().parse::<u32>().unwrap()))
        .fold(HashMap::new(), |mut acc, (key, val)| {
            acc.entry(key).or_insert_with(Vec::new).push(val);
            acc
        });
    let updates = updates.lines()
        .map(|line| line.split(",")
            .map(|split| split.parse::<u32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (rules_map, updates)
}