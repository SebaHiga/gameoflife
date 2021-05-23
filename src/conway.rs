use std::io::{stdout, Write};

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
    pub matrix: Vec<Vec<Cell>>,
    pub matrix_prev: Vec<Vec<Cell>>,
}

impl GameOfLife {
    fn get_surrounded_alive(&self, row: usize, col: usize) -> u16 {
        let mut alives = 0;

        for r in 0..=2 {
            for c in 0..=2 {
                if r == 1 && c == 1 {
                    continue;
                } else {
                    if self.matrix_prev[row + r - 1][col + c - 1].get_status() == Status::Alive {
                        alives += 1;
                    }
                }
            }
        }

        alives
    }

    pub fn process(&mut self) {
        self.matrix_prev = self.matrix.clone();

        for index_row in 1..self.matrix.len() - 1 {
            for index_col in 1..self.matrix[index_row].len() - 1 {
                let alive = self.get_surrounded_alive(index_row, index_col);

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
        let (row, col) = terminal_size().unwrap();

        write!(
            stdout,
            "{clear}{goto}",
            clear = termion::clear::All,
            goto = termion::cursor::Goto(1, 1)
        )
        .unwrap();

        for index_row in 0..self.matrix.len() {
            for index_col in 0..self.matrix[index_row].len() {
                let cell = self.matrix[index_row][index_col].clone();

                if cell.get_status() == Status::Alive {
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

                if index_col >= (row - 1) as usize {
                    break;
                }
            }
            if index_row >= (col - 1) as usize {
                break;
            }
        }
    }
}
