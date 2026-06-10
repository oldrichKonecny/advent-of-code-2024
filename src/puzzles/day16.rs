use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::base::generic_solver::{DaySolver, Input};
use crate::utils::matrix::{Direction, Matrix};
use anyhow::Error;

pub struct Day16;

/// A search state: position plus the direction the reindeer is facing.
type State = (usize, usize, Direction);

impl DaySolver<u64> for Day16 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let maze = Maze::parse(input);
        Ok(maze.lowest_score())
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let maze = Maze::parse(input);
        Ok(maze.best_path_tiles())
    }

}

struct Maze {
    grid: Matrix<Cell>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn parse(input: &Input) -> Self {
        let rows = input.input.lines().count();
        let cols = input.input.lines().next().unwrap().len();
        let mut data = Vec::with_capacity(rows * cols);
        let (mut start, mut end) = ((0, 0), (0, 0));
        for (row, line) in input.input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let cell = match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'S' => { start = (row, col); Cell::Empty }
                    'E' => { end = (row, col); Cell::Empty }
                    _ => panic!("Invalid character in input '{}'", c),
                };
                data.push(cell);
            }
        }
        Maze { grid: Matrix::new(rows, cols, data), start, end }
    }

    /// Dijkstra over `(row, col, direction)` states: moving forward costs 1, turning 90° costs 1000.
    /// The reindeer starts facing East.
    fn lowest_score(&self) -> u64 {
        let mut best: HashMap<(usize, usize, Direction), u64> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(u64, usize, usize, Direction)>> = BinaryHeap::new();
        heap.push(Reverse((0, self.start.0, self.start.1, Direction::Right)));

        while let Some(Reverse((score, row, col, dir))) = heap.pop() {
            if (row, col) == self.end {
                return score;
            }
            if best.get(&(row, col, dir)).is_some_and(|&seen| seen < score) {
                continue;
            }

            // Step forward one tile (cost 1).
            if let Some(next) = self.grid.get_next_info(row, col, &dir) {
                if next.value == Cell::Empty {
                    self.relax(&mut best, &mut heap, score + 1, next.row, next.col, dir);
                }
            }
            // Turn left or right in place (cost 1000).
            for turned in [turn_left(dir), turn_right(dir)] {
                self.relax(&mut best, &mut heap, score + 1000, row, col, turned);
            }
        }

        panic!("no path from start to end");
    }

    fn relax(
        &self,
        best: &mut HashMap<(usize, usize, Direction), u64>,
        heap: &mut BinaryHeap<Reverse<(u64, usize, usize, Direction)>>,
        score: u64,
        row: usize,
        col: usize,
        dir: Direction,
    ) {
        let entry = best.entry((row, col, dir)).or_insert(u64::MAX);
        if score < *entry {
            *entry = score;
            heap.push(Reverse((score, row, col, dir)));
        }
    }

    /// Counts every tile that lies on at least one minimum-score path.
    ///
    /// A single forward Dijkstra records, for each state, the predecessor states that reach it at
    /// its optimal cost (an edge `pred -> state` is kept whenever `cost(pred) + edge == best(state)`).
    /// Since every edge strictly increases the cost, that predecessor graph is acyclic, so a simple
    /// backtrack from the optimal end states visits exactly the states on some best path.
    fn best_path_tiles(&self) -> u64 {
        let start: State = (self.start.0, self.start.1, Direction::Right);
        let mut dist: HashMap<State, u64> = HashMap::new();
        let mut preds: HashMap<State, Vec<State>> = HashMap::new();
        let mut settled: HashSet<State> = HashSet::new();
        let mut heap: BinaryHeap<Reverse<(u64, State)>> = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push(Reverse((0, start)));

        while let Some(Reverse((score, state))) = heap.pop() {
            // Process each state only once, at its settled (optimal) cost.
            if !settled.insert(state) {
                continue;
            }
            let (row, col, dir) = state;

            let mut edges: Vec<(u64, State)> = Vec::with_capacity(3);
            if let Some(next) = self.grid.get_next_info(row, col, &dir) {
                if next.value == Cell::Empty {
                    edges.push((score + 1, (next.row, next.col, dir)));
                }
            }
            edges.push((score + 1000, (row, col, turn_left(dir))));
            edges.push((score + 1000, (row, col, turn_right(dir))));

            for (cost, next) in edges {
                let best = dist.entry(next).or_insert(u64::MAX);
                if cost < *best {
                    *best = cost;
                    preds.insert(next, vec![state]);
                    heap.push(Reverse((cost, next)));
                } else if cost == *best {
                    // Another equally-optimal way to reach `next`: keep this predecessor too.
                    preds.get_mut(&next).unwrap().push(state);
                }
            }
        }

        // Optimal end states: the end tile reached from whichever directions cost the least.
        let best_score = [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .iter()
            .filter_map(|&d| dist.get(&(self.end.0, self.end.1, d)).copied())
            .min()
            .expect("no path from start to end");

        let mut stack: Vec<State> = [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .iter()
            .map(|&d| (self.end.0, self.end.1, d))
            .filter(|s| dist.get(s) == Some(&best_score))
            .collect();

        // Backtrack through the predecessor graph, collecting every distinct tile.
        let mut visited: HashSet<State> = stack.iter().copied().collect();
        let mut tiles: HashSet<(usize, usize)> = HashSet::new();
        while let Some((row, col, dir)) = stack.pop() {
            tiles.insert((row, col));
            if let Some(parents) = preds.get(&(row, col, dir)) {
                for &p in parents {
                    if visited.insert(p) {
                        stack.push(p);
                    }
                }
            }
        }

        tiles.len() as u64
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        _ => panic!("unexpected diagonal direction"),
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => panic!("unexpected diagonal direction"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}
