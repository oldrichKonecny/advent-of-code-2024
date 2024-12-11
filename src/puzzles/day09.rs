use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day09;

impl DaySolver<u64> for Day09 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut disc = parse_input(input);
        compress(&mut disc);
        Ok(compute_checksum(&disc))
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut disc = parse_input(input);
        compress_v2(&mut disc);
        Ok(compute_checksum(&disc))
    }
}

fn parse_input(input: &Input) -> Vec<Option<u32>> {
    let mut disc = Vec::new();
    let mut index_counter = 0u32;
    input.input.split("")
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse::<u32>())
        .enumerate()
        .for_each(|(i, x)| {
            if i & 1 == 0 {
                for _ in 0..x {
                    disc.push(Some(index_counter));
                }
                index_counter += 1;
            } else {
                for _ in 0..x {
                    disc.push(None);
                }
            }
        });
    disc
}

fn compress(disc: &mut [Option<u32>]) {
    let mut end_index = disc.len() - 1;
    let mut start_index = 0;

    while start_index < end_index {
        if let Some(Some(end_val)) = disc.get(end_index) {
            if let Some(None) = disc.get(start_index) {
                disc[start_index] = Some(*end_val);
                disc[end_index] = None;
                start_index += 1;
                end_index -= 1;
            } else {
                start_index += 1;
            }
        } else {
            end_index -= 1;
        }
    }
}

fn compress_v2(disc: &mut [Option<u32>]) {
    let mut end_index_end = disc.len() - 1;
    let mut end_index_start = disc.len() - 1;

    while end_index_end > 0 {
        if let Some(end_val) = disc[end_index_end] {
            for i in (0..end_index_start).rev() {
                let possible_start = disc.get(i);
                if possible_start.is_none() || possible_start.unwrap().is_none() || possible_start.unwrap().unwrap() != end_val {
                    break;
                }
                end_index_start = i;
            }
            let size = end_index_end - end_index_start + 1;
            let new_index = find_new_index(&disc[0..end_index_start], size);
            if let Some(new_index) = new_index {
                for i in 0..size {
                    disc[new_index + i] = Some(end_val);
                }
                for i in 0..size {
                    disc[end_index_start + i] = None;
                }
            }
            end_index_end = end_index_end.checked_sub(size).unwrap_or_default();
            end_index_start = end_index_end;
        } else {
            end_index_end -= 1;
            end_index_start = end_index_end;
        }
    }
}

fn find_new_index(disc_partition: &[Option<u32>], size: usize) -> Option<usize> {
    let mut index = 0;
    let mut counter = 0;
    while index < disc_partition.len() {
        if let Some(None) = disc_partition.get(index) {
            counter += 1;
            if counter == size {
                return Some(index - size + 1);
            }
        } else {
            counter = 0;
        }
        index += 1;
    }
    None
}

fn compute_checksum(disc: &[Option<u32>]) -> u64 {
    disc.iter().enumerate()
        .filter(|(_, v)| v.is_some())
        .map(|(i, val)| i as u64 * val.unwrap() as u64)
        .sum()
}