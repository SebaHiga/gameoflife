use std::{
    io::{stdout, Write},
    usize,
};

use rand::prelude::*;
use termion::{raw::IntoRawMode, terminal_size};
#[derive(Clone, Copy, std::cmp::PartialEq)]
pub enum Status {
    Alive,
    Dead,
}

impl Status {
    pub fn get_rand() -> Status {
        let mut rng = thread_rng();
        if rng.gen_range(0, 10) > 8 {
            return Status::Alive;
        }

        Status::Dead
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub content: char,
    pub status: Status,
}

impl Cell {
    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_content(&self) -> char {
        self.content
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

pub struct GameOfLife {
    matrix: Vec<Vec<Cell>>,
    offset_row: usize,
    offset_col: usize,
    max_row: usize,
    max_col: usize,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        let max_col = 512;
        let max_row = 128;
        let mut matrix = Vec::new();

        for _ in 0..max_row {
            let mut cells = Vec::new();

            for _ in 0..max_col {
                cells.push(Cell {
                    content: '█',
                    status: Status::get_rand(),
                });
            }

            matrix.push(cells);
        }

        GameOfLife {
            matrix: matrix.clone(),
            offset_row: 64,
            offset_col: 64,
            max_row: max_row,
            max_col: max_col,
        }
    }

    fn get_surrounded_alive(&self, matrix_prev: &Vec<Vec<Cell>>, row: usize, col: usize) -> u16 {
        let mut alives = 0;

        for r in 0..=2 {
            for c in 0..=2 {
                if r == 1 && c == 1 {
                    continue;
                } else {
                    if matrix_prev[row + r - 1][col + c - 1].get_status() == Status::Alive {
                        alives += 1;
                    }
                }
            }
        }

        alives
    }

    pub fn process(&mut self) {
        let matrix_prev = self.matrix.clone();

        for index_row in 1..self.matrix.len() - 1 {
            for index_col in 1..self.matrix[index_row].len() - 1 {
                let alive = self.get_surrounded_alive(&matrix_prev, index_row, index_col);

                match self.matrix[index_row][index_col].get_status() {
                    Status::Alive => {
                        if alive == 2 || alive == 3 {
                            continue;
                        } else {
                            self.matrix[index_row][index_col].set_status(Status::Dead);
                        }
                    }

                    Status::Dead => {
                        if alive == 3 {
                            self.matrix[index_row][index_col].set_status(Status::Alive);
                        }
                    }
                }
            }
        }
    }

    pub fn render(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let (col, row) = terminal_size().unwrap();

        for index_row in self.offset_row..self.matrix.len() {
            for index_col in self.offset_col..self.matrix[index_row].len() {
                if self.matrix[index_row][index_col].get_status() == Status::Alive {
                    write!(
                        stdout,
                        "{}",
                        self.matrix[index_row][index_col].get_content()
                    )
                    .unwrap();
                } else {
                    write!(stdout, "{}", ' ').unwrap();
                }

                stdout.flush().unwrap();

                if index_col - self.offset_col >= (col - 1) as usize {
                    break;
                } else if index_col == self.matrix[index_row].len() - 1 {
                    for _ in index_col - self.offset_col..(col - 1) as usize {
                        write!(stdout, " ").unwrap();
                    }
                    write!(stdout, "\n\r").unwrap();
                }
            }

            if index_row - self.offset_row >= (row - 2) as usize {
                break;
            } else if index_row == self.matrix.len() - 1 {
                for _ in index_row - self.offset_row..(row - 2) as usize {
                    write!(stdout, "\n\r").unwrap();
                    write!(stdout, "\n\r").unwrap();
                }
            }
        }
    }

    pub fn shift_right(&mut self, col: usize) {
        if self.offset_col + col < self.max_col {
            self.offset_col += col;
        }
    }
    pub fn shift_left(&mut self, col: usize) {
        if self.offset_col > 1 && (self.offset_col as i16 - col as i16) > 0 {
            self.offset_col -= col;
        }
    }
    pub fn shift_top(&mut self, row: usize) {
        if self.offset_row > 1 {
            self.offset_row -= row;
        }
    }
    pub fn shift_bottom(&mut self, row: usize) {
        if self.offset_row + row < self.max_row {
            self.offset_row += row;
        }
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        if self.offset_row + row > self.max_row || self.offset_col + col > self.max_col {
            return;
        }

        match self.matrix[self.offset_row + row][self.offset_col + col].get_status() {
            Status::Alive => {
                self.matrix[self.offset_row + row][self.offset_col + col].set_status(Status::Dead)
            }
            Status::Dead => {
                self.matrix[self.offset_row + row][self.offset_col + col].set_status(Status::Alive)
            }
        }
    }

    pub fn shift_vertical(&mut self, row: i16) {
        if row > 0 {
            self.shift_top(row.abs() as usize);
        } else {
            self.shift_bottom(row.abs() as usize);
        }
    }

    pub fn shift_horizontal(&mut self, col: i16) {
        if col > 0 {
            self.shift_left(col.abs() as usize);
        } else {
            self.shift_right(col.abs() as usize);
        }
    }
}
