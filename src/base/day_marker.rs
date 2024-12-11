use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use anyhow::{format_err, Error};
use crate::base::generic_solver::{DaySolver, Input};
use crate::puzzles::day01::Day01;
use crate::puzzles::day02::Day02;
use crate::puzzles::day03::Day03;
use crate::puzzles::day04::Day04;
use crate::puzzles::day05::Day05;
use crate::puzzles::day06::Day06;
use crate::puzzles::day07::Day07;
use crate::puzzles::day08::Day08;
use crate::puzzles::day09::Day09;
use crate::puzzles::day10::Day10;
use crate::puzzles::day11::Day11;
use crate::puzzles::day12::Day12;

pub enum DayMarker {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl DayMarker{
    pub fn get_solver(&self) -> Box<dyn DaySolver<u64>> {
        match self {
            DayMarker::Day01 => Box::new(Day01),
            DayMarker::Day02 => Box::new(Day02),
            DayMarker::Day03 => Box::new(Day03),
            DayMarker::Day04 => Box::new(Day04),
            DayMarker::Day05 => Box::new(Day05),
            DayMarker::Day06 => Box::new(Day06),
            DayMarker::Day07 => Box::new(Day07),
            DayMarker::Day08 => Box::new(Day08),
            DayMarker::Day09 => Box::new(Day09),
            DayMarker::Day10 => Box::new(Day10),
            DayMarker::Day11 => Box::new(Day11),
            DayMarker::Day12 => Box::new(Day12),
            DayMarker::Day13 => todo!("Not implemented yet!"),
            DayMarker::Day14 => todo!("Not implemented yet!"),
            DayMarker::Day15 => todo!("Not implemented yet!"),
            DayMarker::Day16 => todo!("Not implemented yet!"),
            DayMarker::Day17 => todo!("Not implemented yet!"),
            DayMarker::Day18 => todo!("Not implemented yet!"),
            DayMarker::Day19 => todo!("Not implemented yet!"),
            DayMarker::Day20 => todo!("Not implemented yet!"),
            DayMarker::Day21 => todo!("Not implemented yet!"),
            DayMarker::Day22 => todo!("Not implemented yet!"),
            DayMarker::Day23 => todo!("Not implemented yet!"),
            DayMarker::Day24 => todo!("Not implemented yet!"),
            DayMarker::Day25 => todo!("Not implemented yet!"),
        }
    }

    pub fn get_input(&self, test_input: bool) -> Input {
        let day_number = match self {
            DayMarker::Day01 => 01,
            DayMarker::Day02 => 02,
            DayMarker::Day03 => 03,
            DayMarker::Day04 => 04,
            DayMarker::Day05 => 05,
            DayMarker::Day06 => 06,
            DayMarker::Day07 => 07,
            DayMarker::Day08 => 08,
            DayMarker::Day09 => 09,
            DayMarker::Day10 => 10,
            DayMarker::Day11 => 11,
            DayMarker::Day12 => 12,
            DayMarker::Day13 => 13,
            DayMarker::Day14 => 14,
            DayMarker::Day15 => 15,
            DayMarker::Day16 => 16,
            DayMarker::Day17 => 17,
            DayMarker::Day18 => 18,
            DayMarker::Day19 => 19,
            DayMarker::Day20 => 20,
            DayMarker::Day21 => 21,
            DayMarker::Day22 => 22,
            DayMarker::Day23 => 23,
            DayMarker::Day24 => 24,
            DayMarker::Day25 => 25,
        };

        let mut file = input_file(day_number, test_input)
            .unwrap_or_else(|_| {
                if test_input {
                    panic!("Test input for day {} not found", day_number);
                } else {
                    download_and_save_input(day_number).expect("Could not download input");
                    input_file(day_number, false).expect("Could not open downloaded file")
                }

            });
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Could not read file");
        Input::new(content)
    }
}

fn input_file(day_number: u32, test_input: bool) -> Result<File, Error> {
    let file_path = if test_input {
        format!("inputs/test_input_{}.txt", day_number)
    } else {
        format!("inputs/input_{}.txt", day_number)
    };
    File::open(file_path).map_err(Error::from)
}

fn download_and_save_input(day_number: u32) -> Result<(), Error> {
    let input_url = format!("https://adventofcode.com/2024/day/{}/input", day_number);
    let client = reqwest::blocking::Client::new();
    let session = std::fs::read_to_string("session_secret")?;
    let response = client.get(&input_url)
        .header("Cookie", format!("session={}", session))
        .send()?;

    response.text()
        .map(|text| {
            let file_path = format!("inputs/input_{}.txt", day_number);
            let mut file = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&file_path)
                .expect(&format!("Could not create file {}", &file_path));
            file.write_all(text.as_bytes()).expect(&format!("Could not write to file {}", &file_path));
            file.flush().expect("Could not flush file");
            file
        })
        .map(|_| ())
        .map_err(Error::from)
}