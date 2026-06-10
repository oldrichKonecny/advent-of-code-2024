# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Advent of Code 2024 solution repository written in Rust. It uses a structured approach with a trait-based solver system where each day's puzzle implements the `DaySolver` trait. All 25 days are solved and submitted (50/50 stars).

## Architecture

### Core Components

- **`base/generic_solver.rs`**: Defines the `DaySolver<T>` trait that all daily solutions implement, with `solve_first` and `solve_second` methods
- **`base/day_marker.rs`**: Contains the `DayMarker` enum that maps days to their solvers and handles input loading/downloading
- **`puzzles/`**: Individual day solutions (`day01.rs`, `day02.rs`, etc.), each implementing `DaySolver<u64>`
- **`utils/`**: Shared utilities including `Matrix<T>` for 2D grid operations
- **`main.rs`**: Entry point that runs a specific day's solution with timing

### Input System

- Inputs are stored in `inputs/` directory (gitignored)
- Real inputs: `inputs/input_N.txt`
- Test inputs: `inputs/test_input_N.txt`
- Automatic downloading from adventofcode.com using session cookie from `session_secret` file
- Input selection controlled by boolean parameter in `get_input(test_input: bool)`

### Adding New Days

**Preferred:** use the `aoc-day` skill (`.claude/skills/aoc-day/SKILL.md`) — it fetches the
puzzle from adventofcode.com, scaffolds the day file and registration, builds the test input, and
verifies against the example before running the real input. Trigger it by asking to "solve day N".

Manual steps (what the skill automates):

1. Create `src/puzzles/dayXX.rs` implementing `DaySolver<u64>` (`XX` = zero-padded, e.g. `day06.rs` / `Day06`)
2. Add module declaration in `src/puzzles/mod.rs`
3. Add the import and replace the `todo!()` in `get_solver` with `Box::new(DayXX)` in `base/day_marker.rs`
   (the `DayMarker` enum, `from_day_number`, and `get_input` already cover all 25 days — leave them as-is)
4. Run with `cargo run -- --day N [--test]` (no `main.rs` change needed; the day is a CLI arg)

## Common Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run -- --day N` (day is a CLI arg; defaults to 25 if omitted)
- **Test**: `cargo test`
- **Check**: `cargo check` (faster compilation check without building)

## Command Line Usage

The application now supports command-line arguments for flexible execution:

### Basic Usage
- `cargo run -- --day 11` - Run day 11 with real input
- `cargo run -- --day 11 --test` - Run day 11 with test input
- `cargo run -- --help` - Show all available options

### Command Line Parameters
- `--day, -d`: Day to solve (1-25) [default: 25]
- `--test, -t`: Use test input instead of real input

## Development Workflow

- Use command-line arguments to specify day and input type (no need to modify main.rs)
- Solutions return `Result<u64, anyhow::Error>` for both parts
- Timing is automatically measured and logged for each part