use std::fmt::Display;

pub trait DaySolver<T>
where
    T: Display,
{
    fn solve_first(&self, input: &Input) -> Result<T, anyhow::Error>;
    fn solve_second(&self, input: &Input) -> Result<T, anyhow::Error>;
}

pub struct Input {
    pub input: String,
}

impl<'a> Input {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn print(&self) {
        self.input.lines()
            .for_each(|line| println!("{}", line));
    }
}