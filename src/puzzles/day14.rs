use crate::base::generic_solver::{DaySolver, Input};
use anyhow::Error;

pub struct Day14;

impl DaySolver<u64> for Day14 {
    fn solve_first(&self, input: &Input) -> Result<u64, Error> {
        let mut quadrant_counts = [0; 5];
        input.input.lines()
            .map(Robot::parse_from)
            .for_each(|mut robot| {
                robot.move_robot(100);
                let quadrant = robot.determine_quadrant();
                quadrant_counts[quadrant as usize] += 1;
            });
        Ok(quadrant_counts.iter().skip(1).product())
    }

    fn solve_second(&self, input: &Input) -> Result<u64, Error> {
        let mut robots = input.input.lines()
            .map(Robot::parse_from)
            .collect::<Vec<_>>();
        let mut min_entropy = f64::MAX;
        let mut steps = 1;
        for i in 1..10_000 {
            let mut quadrant_counts = [0; 5];
            for robot in &mut robots {
                robot.move_forward();
                quadrant_counts[robot.determine_quadrant() as usize] += 1;
            }

            let mut entropy = 0f64;
            for n in quadrant_counts.iter().skip(1) {
                let n = *n as f64;
                let all_cells = (101f64 / 2f64) * (103f64 / 2f64);
                let vacant = all_cells - n;
                let p = -(n / all_cells) * (n / all_cells).log2() - (vacant / all_cells) * (vacant / all_cells).log2();
                entropy += p;
            }
            if entropy < min_entropy {
                min_entropy = entropy;
                steps = i;
            }
        }
        Ok(steps as u64)
    }

}

struct Robot {
    position: (i32, i32),
    delta: (i32, i32),
    grid_size: (u32, u32),
}

impl Robot {
    fn parse_from(input: &str) -> Self {
        let (position, delta) = input.split_once(" v=").unwrap();
        let position = position.split_once("p=").unwrap().1;
        let position = position.split_once(",").unwrap();
        let position = (position.0.parse().unwrap(), position.1.parse().unwrap());
        let delta = delta.split_once(",").unwrap();
        let delta = (delta.0.parse().unwrap(), delta.1.parse().unwrap());
        Self { position, delta, grid_size: (101, 103) }
    }

    fn move_robot(&mut self, number_of_moves: usize) {
        for _ in 0..number_of_moves {
            self.move_forward();
        }
    }

    fn move_forward(&mut self) {
        self.position.0 += self.delta.0 + self.grid_size.0 as i32;
        self.position.0 %= self.grid_size.0 as i32;

        self.position.1 += self.delta.1 + self.grid_size.1 as i32;
        self.position.1 %= self.grid_size.1 as i32;
    }

    fn determine_quadrant(&self) -> u32 {
        let (x, y) = self.position;
        let x_border = self.grid_size.0 as i32 / 2;
        let y_border = self.grid_size.1 as i32 / 2;
        if x == x_border || y == y_border {
            0
        } else if x < x_border {
            if y < y_border {
                1
            } else {
                2
            }
        } else {
            if y < y_border {
                3
            } else {
                4
            }
        }
    }
}