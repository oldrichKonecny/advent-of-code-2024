use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows_len: usize,
    pub cols_len: usize,
    pub data: Vec<T>,
}

impl <T> Matrix<T> {
    pub fn new(rows_len: usize, cols_len: usize, data: Vec<T>) -> Self {
        Self { rows_len, cols_len, data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows_len && col < self.cols_len {
            Some(&self.data[row * self.cols_len + col])
        } else {
            None
        }
    }

    pub fn get_cloned(&self, row: usize, col: usize) -> Option<T>
    where T: Clone {
        if row < self.rows_len && col < self.cols_len {
            Some(self.data[row * self.cols_len + col].clone())
        } else {
            None
        }
    }

    pub fn get_info(&self, row: usize, col: usize) -> Option<NodeInfo<T>>
    where T: Clone {
        if row < self.rows_len && col < self.cols_len {
            Some(NodeInfo {
                row,
                col,
                value: self.data[row * self.cols_len + col].clone(),
            })
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row < self.rows_len && col < self.cols_len {
            self.data[row * self.cols_len + col] = value;
        }
        else { panic!("Index out of bounds") }
    }

    pub fn print(&self) where T: Display {
        for row in 0..self.rows_len {
            for col in 0..self.cols_len {
                print!("{}", self.get(row, col).unwrap());
            }
            println!();
        }
    }

    pub fn get_section(&self, row: usize, col: usize, direction: &Direction, length: usize) -> Option<Vec<T>>
    where T: Clone {
        if col >= self.cols_len || row >= self.rows_len {
            return None;
        }
        let mut res = Vec::with_capacity(length);
        match direction {
            Direction::Up => {
                if row.checked_sub(length - 1).is_none() {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row - i, col) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::Down => {
                if row + (length - 1) >= self.rows_len {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row + i, col) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::Left => {
                if col.checked_sub(length - 1).is_none() {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row, col - i) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::Right => {
                if col + (length - 1) >= self.cols_len {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row, col + i) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::UpLeft => {
                if col.checked_sub(length - 1).is_none() || row.checked_sub(length - 1).is_none() {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row - i, col - i) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::UpRight => {
                if col + (length - 1) >= self.cols_len || row.checked_sub(length - 1).is_none() {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row - i, col + i) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::DownLeft => {
                if col.checked_sub(length - 1).is_none() || row + (length - 1) >= self.rows_len {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row + i, col - i) {
                        res.push(value.clone());
                    }
                }
            }
            Direction::DownRight => {
                if col + (length - 1) >= self.cols_len || row + (length - 1) >= self.rows_len {
                    return None;
                }
                for i in 0..length {
                    if let Some(value) = self.get(row + i, col + i) {
                        res.push(value.clone());
                    }
                }
            }
        }
        Some(res)
    }

    pub fn get_next(&self, row: usize, col: usize, direction: &Direction) -> Option<&T> {
        if col >= self.cols_len || row >= self.rows_len {
            return None;
        }
        match direction {
            Direction::Up => self.get(row.checked_sub(1)?, col),
            Direction::Down => self.get(row + 1, col),
            Direction::Left => self.get(row, col.checked_sub(1)?),
            Direction::Right => self.get(row, col + 1),
            Direction::UpLeft => self.get(row.checked_sub(1)?, col.checked_sub(1)?),
            Direction::UpRight => self.get(row.checked_sub(1)?, col + 1),
            Direction::DownLeft => self.get(row + 1, col.checked_sub(1)?),
            Direction::DownRight => self.get(row + 1, col + 1),
        }
    }

    pub fn get_next_info(&self, row: usize, col: usize, direction: &Direction) -> Option<NodeInfo<T>>
    where T: Clone {
        if col >= self.cols_len || row >= self.rows_len {
            return None;
        }
        let (next_row, next_col, value) = match direction {
            Direction::Up => {
                if let Some(val) = self.get(row.checked_sub(1)?, col) {
                    (row - 1, col, val.clone())
                } else {
                    return None;
                }
            },
            Direction::Down => {
                if let Some(val) = self.get(row + 1, col) {
                    (row + 1, col, val.clone())
                } else {
                    return None;
                }
            },
            Direction::Left => {
                if let Some(val) = self.get(row, col.checked_sub(1)?) {
                    (row, col - 1, val.clone())
                } else {
                    return None;
                }
            },
            Direction::Right => {
                if let Some(val) = self.get(row, col + 1) {
                    (row, col + 1, val.clone())
                } else {
                    return None;
                }
            },
            Direction::UpLeft => {
                if let Some(val) = self.get(row.checked_sub(1)?, col.checked_sub(1)?) {
                    (row - 1, col - 1, val.clone())
                } else {
                    return None;
                }
            },
            Direction::UpRight => {
                if let Some(val) = self.get(row.checked_sub(1)?, col + 1) {
                    (row - 1, col + 1, val.clone())
                } else {
                    return None;
                }
            },
            Direction::DownLeft => {
                if let Some(val) = self.get(row + 1, col.checked_sub(1)?) {
                    (row + 1, col - 1, val.clone())
                } else {
                    return None;
                }
            },
            Direction::DownRight => {
                if let Some(val) = self.get(row + 1, col + 1) {
                    (row + 1, col + 1, val.clone())
                } else {
                    return None;
                }
            },
        };
        Some(NodeInfo {
            row: next_row,
            col: next_col,
            value,
        })
    }

    pub fn find_all(&self, value: T) -> Vec<(usize, usize)>
    where T: PartialEq {
        let mut res = Vec::new();
        for row in 0..self.rows_len {
            for col in 0..self.cols_len {
                if let Some(v) = self.get(row, col) {
                    if *v == value {
                        res.push((row, col));
                    }
                }
            }
        }
        res
    }

    pub fn count_all(&self, value: T) -> usize
    where T: PartialEq {
        self.data.iter()
            .filter(|v| **v == value)
            .count()
    }

    pub fn get_neighbors(&self, row: usize, col: usize, directions: &[Direction]) -> Vec<NodeInfo<T>>
    where T: Clone {
        let mut res = Vec::new();
        for direction in directions {
            if let Some(info) = self.get_next_info(row, col, direction) {
                res.push(info);
            }
        }
        res
    }

    pub fn get_maybe_neighbors(&self, row: usize, col: usize, directions: &[Direction]) -> Vec<Option<NodeInfo<T>>>
    where T: Clone {
        let mut res = Vec::new();
        for direction in directions {
            res.push(self.get_next_info(row, col, direction));
        }
        res
    }

    pub fn get_all_points(&self) -> Vec<NodeInfo<T>>
    where T: Clone {
        let mut res = Vec::new();
        for row in 0..self.rows_len {
            for col in 0..self.cols_len {
                if let Some(info) = self.get_info(row, col) {
                    res.push(info);
                }
            }
        }
        res

    }
}

impl<T> IntoIterator for Matrix<T>
where T: Clone {
    type Item = NodeInfo<T>;
    type IntoIter = MatrixIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}


pub struct MatrixIterator<T>
where T: Clone {
    matrix: Matrix<T>,
    row: usize,
    col: usize,
}

impl<T> Iterator for MatrixIterator<T>
where T: Clone {
    type Item = NodeInfo<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.matrix.get_info(self.row, self.col).map(|node_info| {
            self.col += 1;
            if self.col == self.matrix.cols_len {
                self.col = 0;
                self.row += 1;
            }
            node_info
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeInfo<T> {
    pub row: usize,
    pub col: usize,
    pub value: T,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}