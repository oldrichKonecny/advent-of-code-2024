use std::fmt::Display;
use std::time::Instant;
use log::{error, info};
use crate::base::day_marker::DayMarker;
use crate::base::generic_solver::{DaySolver, Input};
extern crate pretty_env_logger;

mod base;
mod puzzles;
mod utils;


fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let day = DayMarker::Day13;
    solve_day(day.get_solver(), day.get_input(false));
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




