extern crate rand;

use rand::prelude::*;
#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
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

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub row: u16,
    pub col: u16,
    pub content: char,
    pub status: Status,
}

impl Cell {
    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_position(&mut self, row: u16, col: u16) {
        self.row = row;
        self.col = col;
    }

    pub fn get_content(&self) -> char {
        self.content
    }
}
