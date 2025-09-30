extern crate pretty_env_logger;
use crate::base::day_marker::DayMarker;
use crate::base::generic_solver::{DaySolver, Input};
use clap::Parser;
use log::{error, info};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod base;
mod puzzles;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about = "Advent of Code 2024 Solutions", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 11, help = "Day to solve (1-25)")]
    day: u8,

    #[arg(short, long, help = "Use test input instead of real input")]
    test: bool,
}


fn main() {
    let args = Args::parse();

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let day = DayMarker::from_day_number(args.day)
        .unwrap_or_else(|_| {
            error!("Invalid day number: {}. Please choose a day between 1 and 25.", args.day);
            std::process::exit(1);
        });

    solve_day(day.get_solver(), day.get_input(args.test));
}

fn solve_day<P: Display>(day: Box<dyn DaySolver<P>>, input: Input) {
    let timer = Instant::now();
    let first = day.solve_first(&input);
    let first_elapsed = timer.elapsed();
    match first {
        Ok(result) => info!("First part: {} in {} microseconds", result, first_elapsed.as_micros()),
        Err(e) => error!("Error in first part: {}", e),
    }

    let timer = Instant::now();
    let second = day.solve_second(&input);
    let second_elapsed = timer.elapsed();
    match second {
        Ok(result) => info!("Second part: {} in {} microseconds", result, second_elapsed.as_micros()),
        Err(e) => error!("Error in second part: {}", e),
    }
}





