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
use crate::puzzles::day13::Day13;
use crate::puzzles::day14::Day14;
use crate::puzzles::day15::Day15;
use crate::puzzles::day16::Day16;
use crate::puzzles::day17::Day17;
use crate::puzzles::day18::Day18;
use crate::puzzles::day19::Day19;
use crate::puzzles::day20::Day20;
use crate::puzzles::day21::Day21;
use crate::puzzles::day22::Day22;
use crate::puzzles::day23::Day23;
use crate::puzzles::day24::Day24;
use crate::puzzles::day25::Day25;
use anyhow::Error;
use std::fs::File;
use std::io::{Read, Write};

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
    pub fn from_day_number(day: u8) -> Result<DayMarker, anyhow::Error> {
        match day {
            1 => Ok(DayMarker::Day01),
            2 => Ok(DayMarker::Day02),
            3 => Ok(DayMarker::Day03),
            4 => Ok(DayMarker::Day04),
            5 => Ok(DayMarker::Day05),
            6 => Ok(DayMarker::Day06),
            7 => Ok(DayMarker::Day07),
            8 => Ok(DayMarker::Day08),
            9 => Ok(DayMarker::Day09),
            10 => Ok(DayMarker::Day10),
            11 => Ok(DayMarker::Day11),
            12 => Ok(DayMarker::Day12),
            13 => Ok(DayMarker::Day13),
            14 => Ok(DayMarker::Day14),
            15 => Ok(DayMarker::Day15),
            16 => Ok(DayMarker::Day16),
            17 => Ok(DayMarker::Day17),
            18 => Ok(DayMarker::Day18),
            19 => Ok(DayMarker::Day19),
            20 => Ok(DayMarker::Day20),
            21 => Ok(DayMarker::Day21),
            22 => Ok(DayMarker::Day22),
            23 => Ok(DayMarker::Day23),
            24 => Ok(DayMarker::Day24),
            25 => Ok(DayMarker::Day25),
            _ => Err(anyhow::Error::msg(format!("Day {} is not valid (1-25)", day))),
        }
    }

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
            DayMarker::Day13 => Box::new(Day13),
            DayMarker::Day14 => Box::new(Day14),
            DayMarker::Day15 => Box::new(Day15),
            DayMarker::Day16 => Box::new(Day16),
            DayMarker::Day17 => Box::new(Day17),
            DayMarker::Day18 => Box::new(Day18),
            DayMarker::Day19 => Box::new(Day19),
            DayMarker::Day20 => Box::new(Day20),
            DayMarker::Day21 => Box::new(Day21),
            DayMarker::Day22 => Box::new(Day22),
            DayMarker::Day23 => Box::new(Day23),
            DayMarker::Day24 => Box::new(Day24),
            DayMarker::Day25 => Box::new(Day25),
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
        .header("Cookie", format!("session={}", session.trim()))
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